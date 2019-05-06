pub(crate) mod service;

use failure::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use router::{self, Route, Router};
use {rt, transport};

use self::service::NewAppService;

pub type Result<T> = ::std::result::Result<T,Error>;

#[derive(Debug)]
pub struct App {
    router: Arc<Router>,
    addr: SocketAddr,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder {
            router: Router::builder(),
        }
    }

    pub fn serve(self) -> Result<()> {
        let incoming = transport::Incoming::new(&self.addr)?;
        rt::serve(self.lift_new_service(), incoming)
    }

    fn lift_new_service(self) -> NewAppService {
        NewAppService { app: self }
    }
}

#[derive(Debug)]
pub struct AppBuilder {
    router: router::Builder,
}

impl AppBuilder {
    pub fn mount<I>(&mut self, routes: I) -> &mut Self
    where
        I: IntoIterator<Item = Route>,
    {
        self.router.mount(routes);
        self
    }

    pub fn finish(&mut self) -> Result<App> {
        Ok(App {
            router: self.router.finish().map(Arc::new)?,
            addr: ([127, 0, 0, 1], 4000).into(),
        })
    }

    pub fn serve(&mut self) -> Result<()> {
        self.finish()?.serve()
    }
}
