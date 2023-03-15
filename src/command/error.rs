use frankenstein::Message;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("Found a message, but no command.")]
    MessageError(Message),
    #[error("No message found.")]
    NoMessageError(()),
}
