// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

extern crate engine_workspace;
extern crate utilities;

use std::sync::Arc;
use engine_workspace::WorkspaceServer;
use utilities::result::Result;
use utilities::setup::CommonSetup;

#[tokio::main]
async fn main() -> Result<()> {
    let setup = Arc::new(CommonSetup::new().await?);
    let server = WorkspaceServer::new(setup);
    server.listen().await
}
