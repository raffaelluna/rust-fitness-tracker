use crate::command::{error::ProcessorError, process_task::UpdateProcessor};
use crate::telegram::error::ApiError;
use frankenstein::Api;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::MethodResponse;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;

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
        match UpdateProcessor::new(self.telegram_api.clone(), update) {
            Ok(processor) => {
                let msg_to_send = processor.run();
                if let Err(err) = self.send_message_with_reply(
                    processor.message.chat.id,
                    processor.message.message_id,
                    msg_to_send.as_str(),
                ) {
                    println!("Failed to send message: {err:?}");
                }
            }
            Err(err) => match err {
                ProcessorError::MessageError(message) => {
                    println!("No command received.");

                    if let Err(err) = self.send_message_with_reply(
                        message.chat.id,
                        message.message_id,
                        "No command received.",
                    ) {
                        println!("Failed to send message: {err:?}");
                    }
                }
                ProcessorError::NoMessageError(_) => {
                    println!("No message received.")
                }
            },
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
