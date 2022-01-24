// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use hyper_staticfile::ResolveResult;
use std::path::Path;
use utilities::{
    errors::{self, HandlerError, HandlerErrorMessage},
    hyper::{Body, Request, Response, StatusCode},
    result::HandlerResult,
};

pub struct StaticFilesHandler;

impl StaticFilesHandler {
    pub async fn handle(request: Request<Body>, path: &str) -> HandlerResult<Response<Body>> {
        // Document root path.
        let root = Path::new(path);

        // Resolve root.
        let result = hyper_staticfile::resolve(&root, &request)
            .await
            .map_err(|err| HandlerError::Internal {
                ctx: HandlerErrorMessage::InternalError,
                src: errors::wrap_error("getting static file", err),
            })?;

        // Handle resolve result.
        match result {
            ResolveResult::MethodNotMatched => Err(HandlerError::Client {
                ctx: HandlerErrorMessage::GetMethodExpected,
                code: StatusCode::BAD_REQUEST,
                src: errors::new_error("wrong method; expected a GET method"),
            }),
            ResolveResult::UriNotMatched => Err(HandlerError::Client {
                ctx: HandlerErrorMessage::InvalidFilePath,
                code: StatusCode::BAD_REQUEST,
                src: errors::new_error(format!(r#"invalid file path "{:?}""#, path)),
            }),
            ResolveResult::NotFound => Err(HandlerError::Client {
                ctx: HandlerErrorMessage::NotFound,
                code: StatusCode::NOT_FOUND,
                src: errors::new_error(format!(r#"resource not found in "{:?}""#, path)),
            }),
            ResolveResult::PermissionDenied => Err(HandlerError::Client {
                ctx: HandlerErrorMessage::BadRequest,
                code: StatusCode::BAD_REQUEST,
                src: errors::new_error(format!(r#"permission denied for "{:?}""#, path)),
            }),
            _ => Ok(hyper_staticfile::ResponseBuilder::new()
                .request(&request)
                .build(result)
                .unwrap()),
        }
    }
}
