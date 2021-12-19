// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use hyper_staticfile::Static;
use utilities::{
    errors::{self, HandlerError, HandlerErrorMessage},
    hyper::{Body, Request, Response},
    result::HandlerResult,
};

pub struct StaticFilesHandler;

impl StaticFilesHandler {
    pub async fn handle(request: Request<Body>, path: &str) -> HandlerResult<Response<Body>> {
        Static::new(path)
            .serve(request)
            .await
            .map_err(|err| HandlerError::Internal {
                ctx: HandlerErrorMessage::InternalError,
                src: errors::wrap_error("getting static file", err),
            })
    }
}
