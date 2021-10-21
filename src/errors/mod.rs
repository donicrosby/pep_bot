use matrix_sdk::events::AnyMessageEventContent;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    SendError(#[from] tokio::sync::mpsc::error::SendError<AnyMessageEventContent>),
    #[error("Could not get valid value from pep component list {0}")]
    PepChoiceError(String),
    #[error("About You string was empty!")]
    AboutYouEmpty,
}
