use crate::command::process_task::{process, Command};
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::MethodResponse;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum ApiError {
    #[error(transparent)]
    FrankensteinError(#[from] frankenstein::Error),
}

pub struct TelegramBot {
    telegram_api: Api,
}

impl TelegramBot {
    pub fn new(token: String) -> Self {
        let api = Api::new(token.as_str());
        Self { telegram_api: api }
    }

    pub fn run(&self) {
        let update_params_builder = GetUpdatesParams::builder();
        let mut update_params = update_params_builder.clone().build();

        loop {
            match self.telegram_api.get_updates(&update_params) {
                Ok(response) => {
                    for update in response.result {
                        let update_id = update.update_id;
                        self.handle_update(update);
                        update_params = update_params_builder
                            .clone()
                            .offset(update_id + 1)
                            .build();
                    }
                }
                Err(error) => {
                    println!("Failed to get updates: {error:?}");
                }
            }
        }
    }

    fn handle_update(&self, update: frankenstein::Update) {
        if let UpdateContent::Message(message) = update.content {
            let text = message.text.clone().unwrap();

            match text.split_once('\n') {
                Some((command_candidate, par)) => {
                    let command =
                        Command::from_str(command_candidate).unwrap();
                    let msg_to_send = process(command, par);

                    if let Err(err) = self.send_message_with_reply(
                        message.chat.id,
                        message.message_id,
                        msg_to_send.as_str(),
                    ) {
                        println!("Failed to send message: {err:?}");
                    }
                }

                None => {
                    println!("No command received.");

                    if let Err(err) = self.send_message_with_reply(
                        message.chat.id,
                        message.message_id,
                        "No command received.",
                    ) {
                        println!("Failed to send message: {err:?}");
                    }
                }
            }
            println!("message: {:?}", text);
        }
    }

    fn send_message_with_reply(
        &self,
        chat_id: i64,
        message_id: i32,
        text: &str,
    ) -> Result<MethodResponse<Message>, ApiError> {
        let send_message_params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(text)
            .reply_to_message_id(message_id)
            .build();

        Ok(self.telegram_api.send_message(&send_message_params)?)
    }
}
