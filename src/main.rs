use crate::telegram::telegram_bot::TelegramBot;
use dotenv::dotenv;

pub mod command;
pub mod telegram;

fn main() {
    dotenv().ok();

    let token =
        std::env::var("BOT_API_TOKEN").expect("BOT_API_TOKEN must be set.");

    let telegram_bot = TelegramBot::new(token);
    telegram_bot.run();
}
