use matrix_sdk::ruma::events::AnyMessageEventContent;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Send(#[from] tokio::sync::mpsc::error::SendError<AnyMessageEventContent>),
    #[error("Could not get valid value from pep component list {0}")]
    PepChoice(String),
    #[error("About You string was empty!")]
    AboutYouEmpty,
}
