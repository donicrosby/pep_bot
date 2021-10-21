extern crate lazy_static;
extern crate clap;
use crate::config::Config;
use mrsbfh::config::Loader;
use clap::{crate_version, App, Arg};
use std::error::Error;
use tokio;
use tracing::*;

mod matrix;
mod config;
mod commands;
mod errors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .init();

    info!("Booting up....");
    debug!("Creating arguments...");
    let args = App::new("Pep Bot")
        .version(crate_version!())
        .author("Jeansburger <@doni:jeansburger.net>")
        .about("Matrix Bot that Peps you up!")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .default_value("config.yml")
                .takes_value(true)
                .help("Configuration file for logging into a matrix homeserver"),
        )
        .get_matches();
    info!("Loading configs...");
    let config = Config::load(args.value_of("config").unwrap())?;
    info!("Setting up Client...");
    let client = &mut matrix::setup(config.clone()).await?;
    info!("Starting Sync...");
    matrix::start_sync(client, config).await?;
    Ok(())
}
