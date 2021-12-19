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
        _setup: Arc<CommonSetup>,
    ) -> HandlerResult<Response<Body>> {
        // Routing.
        StaticFilesHandler::handle(request, "../web_ui/public/").await
    }
}
