use crate::config::Config;
use crate::errors::Error;
use matrix_sdk::events::{room::message::MessageEventContent, AnyMessageEventContent};
use mrsbfh::commands::command;
use matrix_sdk::Client;
use rand::{seq::IteratorRandom, thread_rng};
use std::borrow::Cow;
use tracing::*;

#[command(help = "`!pep` - Gives you a randomized pep talk when you need it!")]
pub async fn pep<'a>(
    _client: Client,
    tx: mrsbfh::Sender,
    config: Config<'a>,
    _sender: String,
    mut _args: Vec<&str>,
) -> Result<(), Error>
where
    Config<'a>: mrsbfh::config::Loader + Clone,
{
    let pep = create_pep(&config).await?;

    let content =
        AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(pep));

    tx.send(content).await?;
    Ok(())
}

fn choose_from_vec<'a>(config_name: &str, choices: &[Cow<'a, str>]) -> Result<String, Error> {
    debug!("Getting pep fragrment from {}", config_name);
    let mut rng = thread_rng();
    if !choices.is_empty() {
        debug!("List of options isn't empty picking one at random...");
        Ok(choices.iter().choose(&mut rng).unwrap().to_string())
    } else {
        error!("List of options is empty? Please fix that...");
        Err(Error::PepChoiceError(String::from(config_name)))
    }
}

async fn create_pep<'a>(config: &Config<'a>) -> Result<String, Error>
    where
        Config<'a>: mrsbfh::config::Loader + Clone,
{
    info!("Generating Pep...");
    let leadin = choose_from_vec("lead-ins", &config.pep_config.lead_ins)?;
    let about_you = choose_from_vec("about_yous", &config.pep_config.about_yous)?;
    let complement = choose_from_vec("compliments", &config.pep_config.complements)?;
    let ending = choose_from_vec("endings", &config.pep_config.endings)?;

    info!("Successfully got all options!");
    let final_pep = format!("{} {} {} {}", leadin, about_you, complement, ending);
    debug!("Final pep: {}", &final_pep);
    Ok(final_pep)
}
