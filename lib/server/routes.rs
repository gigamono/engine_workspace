// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::sync::Arc;
use utilities::{
    hyper::{Body, Request, Response},
    result::HandlerResult,
    setup::CommonSetup,
};

use crate::handlers::StaticFilesHandler;

pub struct Router;

impl Router {
    pub async fn route(
        request: Request<Body>,
        setup: Arc<CommonSetup>,
    ) -> HandlerResult<Response<Body>> {
        let path = request.uri().path();
        let config = &setup.config;

        // Routing.
        if path == "/v1/workspaces" || path.starts_with("/v1/workspaces/") {
            // Route for workspace management. e.g., GET https://internal.gigamono.com/v1/workspaces/
            todo!();
        } else {
            // For everything else, use the the static file server.
            StaticFilesHandler::handle(request, &config.web_ui.dir).await
        }
    }
}
