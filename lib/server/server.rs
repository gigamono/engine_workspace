// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use crate::Router;
use futures_util::FutureExt;
use log::{error, info};
use std::convert::Infallible;
use std::future::Future;
use std::{panic::AssertUnwindSafe, sync::Arc};
use utilities::ip;
use utilities::{
    http,
    hyper::{
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

        // Get socket address.
        let addr = ip::parse_socket_address(&self.setup.config.engines.workspace.socket_address)?;

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
