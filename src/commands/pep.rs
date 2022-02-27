use crate::config::Config;
use crate::errors::Error;
use lazy_static::lazy_static;
use matrix_sdk::{
    ruma::{
        events::{
            room::message::{MessageEventContent, MessageType, TextMessageEventContent},
            AnyMessageEventContent,
        },
        RoomId,
    },
    Client,
};
use mrsbfh::commands::command;
use rand::{seq::IteratorRandom, thread_rng};
use regex::Regex;
use std::{borrow::Cow, collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tracing::*;

lazy_static! {
    static ref LEADIN_REGEX: Regex = Regex::new(r"^(?:[\w\s,]+)(\.{1,3}|[:?!])$").unwrap();
    static ref DUP_MANAGER: RwLock<HashMap<String, String>> = {
        let map = HashMap::new();
        RwLock::new(map)
    };
}

#[command(help = "`!pep` - Gives you a randomized pep talk when you need it!")]
pub async fn pep<'a>(
    _client: Client,
    tx: mrsbfh::Sender,
    config: Arc<Mutex<Config<'a>>>,
    _sender: String,
    _room_id: RoomId,
    mut _args: Vec<&str>,
) -> Result<(), Error>
where
    Config<'a>: mrsbfh::config::Loader + Clone,
{
    let pep = create_pep(&*config.lock().await).await?;

    let content = AnyMessageEventContent::RoomMessage(MessageEventContent::new(MessageType::Text(
        TextMessageEventContent::markdown(&pep),
    )));

    tx.send(content).await?;
    Ok(())
}

async fn choose_from_vec(config_name: String, choices: &[Cow<'_, str>]) -> Result<String, Error> {
    debug!("Getting pep fragrment from {}", config_name);
    if !choices.is_empty() {
        debug!("List of options isn't empty picking one at random...");
        let mut choice = choices
            .iter()
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        if let Some(already_used) = DUP_MANAGER.read().await.get(&config_name) {
            while choice.eq(already_used) {
                debug!("I already used that in a response earlier, picking new item...");
                choice = choices
                    .iter()
                    .choose(&mut thread_rng())
                    .unwrap()
                    .to_string();
            }
        }
        DUP_MANAGER
            .write()
            .await
            .insert(config_name, choice.clone());
        Ok(choice)
    } else {
        error!("List of options is empty? Please fix that...");
        Err(Error::PepChoice(config_name))
    }
}

fn uppercase_about_you(s: &str) -> Result<String, Error> {
    let mut c = s.chars();
    match c.next() {
        None => Err(Error::AboutYouEmpty),
        Some(f) => Ok(f.to_uppercase().collect::<String>() + c.as_str()),
    }
}

async fn create_pep<'a>(config: &Config<'a>) -> Result<String, Error>
where
    Config<'a>: mrsbfh::config::Loader + Clone,
{
    info!("Generating Pep...");
    let leadin = choose_from_vec("lead-ins".to_owned(), &config.pep_config.lead_ins).await?;
    let mut about_you =
        choose_from_vec("about_yous".to_owned(), &config.pep_config.about_yous).await?;
    let complement =
        choose_from_vec("compliments".to_owned(), &config.pep_config.complements).await?;
    let ending = choose_from_vec("endings".to_owned(), &config.pep_config.endings).await?;

    if LEADIN_REGEX.is_match(leadin.as_str()) {
        about_you = uppercase_about_you(about_you.as_str())?;
    }

    info!("Successfully got all options!");
    let final_pep = format!("{} {} {} {}", leadin, about_you, complement, ending);
    debug!("Final pep: {}", &final_pep);
    Ok(final_pep)
}
