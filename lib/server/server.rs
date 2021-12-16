// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use crate::Router;
use futures_util::FutureExt;
use hyper_staticfile::Static;
use log::{error, info};
use std::convert::Infallible;
use std::future::Future;
use std::{net::SocketAddr, panic::AssertUnwindSafe, sync::Arc};
use utilities::{
    http::{
        self,
        service::{make_service_fn, service_fn},
        Body, Request, Response, Server,
    },
    result::{HandlerResult, Result},
    setup::CommonSetup,
};

pub struct WorkspaceServer {
    setup: Arc<CommonSetup>,
}

impl WorkspaceServer {
    pub fn new(setup: Arc<CommonSetup>) -> Self {
        Self { setup }
    }

    pub async fn listen(&self) -> Result<()> {
        // Initialize logger.
        env_logger::init();

        // Get port info and create socket address.
        let port = self.setup.config.engines.workspace.port;
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        info!(r#"Socket address = "{}""#, addr);

        let make_svc = make_service_fn(move |_| {
            let setup = Arc::clone(&self.setup);

            async move {
                Ok::<_, Infallible>(service_fn(move |request| {
                    Self::handler_panic_wrap(Router::route, request, Arc::clone(&setup))
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        Ok(server.await?)
    }

    async fn handler_panic_wrap<F, Fut>(
        func: F,
        request: Request<Body>,
        setup: Arc<CommonSetup>,
    ) -> std::result::Result<Response<Body>, Infallible>
    where
        F: FnOnce(Request<Body>, Arc<CommonSetup>) -> Fut,
        Fut: Future<Output = HandlerResult<Response<Body>>>,
    {
        match AssertUnwindSafe(func(request, setup)).catch_unwind().await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(err)) => {
                error!("{:?}", err);
                Ok(err.as_hyper_response())
            }
            Err(err) => http::handle_panic_error_t(err),
        }
    }
}