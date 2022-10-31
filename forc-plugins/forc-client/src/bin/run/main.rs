use forc_client::ops::run::{cmd::RunCommand, op::run};
use forc_tracing::init_tracing_subscriber;
use std::process;

use clap::Parser;

#[tokio::main]
async fn main() {
    init_tracing_subscriber(Default::default());
    let command = RunCommand::parse();
    if let Err(err) = run(command).await {
        tracing::error!("Error: {:?}", err);
        process::exit(1);
    }
}
