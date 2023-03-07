use crate::command::process_task::{process, Command};
use dotenv::dotenv;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::MethodResponse;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use std::str::FromStr;
use thiserror::Error;

pub mod command;

#[derive(Debug, Error)]
enum ApiError {
    #[error(transparent)]
    FrankensteinError(#[from] frankenstein::Error),
}

fn main() {
    dotenv().ok();

    let token =
        std::env::var("BOT_API_TOKEN").expect("BOT_API_TOKEN must be set.");
    let api = Api::new(token.as_str());

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        match api.get_updates(&update_params) {
            Ok(response) => {
                for update in response.result {
                    let update_id = update.update_id;
                    handle_update(update, &api);
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

fn handle_update(update: frankenstein::Update, api: &Api) {
    if let UpdateContent::Message(message) = update.content {
        let text = message.text.clone().unwrap();

        match text.split_once('\n') {
            Some((command_candidate, par)) => {
                let command = Command::from_str(command_candidate).unwrap();
                let msg_to_send = process(command, par);

                if let Err(err) = send_message_with_reply(
                    api,
                    message.chat.id,
                    message.message_id,
                    msg_to_send.as_str(),
                ) {
                    println!("Failed to send message: {err:?}");
                }
            }

            None => {
                println!("No command received.");

                if let Err(err) = send_message_with_reply(
                    api,
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
    api: &Api,
    chat_id: i64,
    message_id: i32,
    text: &str,
) -> Result<MethodResponse<Message>, ApiError> {
    let send_message_params = SendMessageParams::builder()
        .chat_id(chat_id)
        .text(text)
        .reply_to_message_id(message_id)
        .build();

    Ok(api.send_message(&send_message_params)?)
}
