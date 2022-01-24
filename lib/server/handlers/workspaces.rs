// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use std::sync::Arc;

use utilities::{
    hyper::{Body, Request},
    result::HandlerResult,
    setup::CommonSetup,
};

pub struct WorkspacesHandler;

impl WorkspacesHandler {
    pub async fn handle(request: Request<Body>, setup: Arc<CommonSetup>) -> HandlerResult<()> {
        Ok(())
    }
}
