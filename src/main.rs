extern crate clap;
extern crate lazy_static;
use crate::config::Config;
use clap::{crate_version, App, Arg};
use mrsbfh::config::Loader;
use std::error::Error;
use tracing::*;

mod commands;
mod config;
mod errors;
mod matrix;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
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
                .env("CONFIG_PATH")
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
