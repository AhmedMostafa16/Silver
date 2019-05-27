use http::Request;
use hyperx::header::Header;
use std::ops::Deref;
use std::sync::Arc;

use app::AppState;
use error::Error;
use input::RequestBody;
use router::{Route, RouterState};

#[cfg(feature = "session")]
use session::CookieManager;

#[cfg(feature = "session")]
pub use session::Cookies;

scoped_thread_local!(static CONTEXT: Context);

#[derive(Debug)]
pub struct Context {
    pub(crate) request: Request<RequestBody>,
    route: RouterState,
    pub(crate) state: Arc<AppState>,
    #[cfg(feature = "session")]
    pub(crate) cookies: CookieManager,
}

impl Context {
    pub(crate) fn new(request: Request<RequestBody>, state: Arc<AppState>) -> Context {
        Context {
            request: request,
            route: RouterState::Uninitialized,
            state: state,
            #[cfg(feature = "session")]
            cookies: Default::default(),
        }
    }

    pub(crate) fn set<R>(&self, f: impl FnOnce() -> R) -> R {
        CONTEXT.set(self, f)
    }

    pub fn with<R>(f: impl FnOnce(&Context) -> R) -> R {
        CONTEXT.with(f)
    }

    pub fn request(&self) -> &Request<RequestBody> {
        &self.request
    }

    // FIXME: cache parsed value
    pub fn header<H>(&self) -> Result<Option<H>, Error>
    where
        H: Header,
    {
        self.request.headers().get(H::header_name()).map_or_else(
            || Ok(None),
            |h| {
                H::parse_header(&h.as_bytes().into()).map(Some).map_err(
                    Error::bad_request,
                )
            },
        )
    }

    pub fn route(&self) -> Option<&Route> {
        match self.route {
            RouterState::Matched(i, ..) => self.state.router().get_route(i),
            _ => None,
        }
    }

    pub(crate) fn set_route(&mut self, state: RouterState) {
        self.route = state;
    }

    pub fn params(&self) -> Option<Params> {
        match self.route {
            RouterState::Matched(_, ref params) => Some(Params {
                path: self.request().uri().path(),
                params: &params[..],
            }),
            _ => None,
        }
    }

    #[cfg(feature = "session")]
    pub fn cookies(&self) -> Result<Cookies, Error> {
        if self.cookies.is_init() {
            self.cookies.init(self.request.headers()).map_err(
                Error::internal_server_error,
            )?;
        }
        Ok(self.cookies.cookies(self.state.secret_key()))
    }
}

impl Deref for Context {
    type Target = Request<RequestBody>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

#[derive(Debug)]
pub struct Params<'a> {
    path: &'a str,
    params: &'a [(usize, usize)],
}

impl<'a> Params<'a> {
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    pub fn get(&self, i: usize) -> Option<&str> {
        self.params.get(i).and_then(|&(s, e)| self.path.get(s..e))
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a str> + 'a {
        let path = self.path;
        self.params.into_iter().map(move |&(s, e)| &path[s..e])
    }
}
