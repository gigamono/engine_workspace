// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::sync::Arc;

use utilities::{
    http::{Body, Request},
    result::HandlerResult,
    setup::CommonSetup,
};

pub struct WorkspacesHandler;

impl WorkspacesHandler {
    pub async fn handle(request: Request<Body>, setup: Arc<CommonSetup>) -> HandlerResult<()> {
        Ok(())
    }
}
