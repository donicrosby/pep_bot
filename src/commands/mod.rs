use crate::config::Config;
use crate::errors::Error;
use mrsbfh::commands::command_generate;

pub mod pep;

#[command_generate(bot_name = "Pep Bot", description = "This bot gives you a pep talk!")]
enum Commands {
    Pep,
}
