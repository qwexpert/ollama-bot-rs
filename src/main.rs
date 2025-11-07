mod api;
mod commands;
mod handler;

use teloxide::prelude::*;
use teloxide::types::{ParseMode, User};
use teloxide::utils::command::BotCommands;

use crate::api::gen_res;
use crate::commands::Command;
use crate::handler::invoke;

async fn handle_command(
    bot: Bot,
    message: Message,
    cmd: Command,
) -> Result<(), teloxide::RequestError> {
    invoke(bot, message, cmd).await?;
    Ok(())
}

async fn handle_message(bot: Bot, message: Message) -> Result<(), teloxide::RequestError> {
    if let Some(text) = message.text() {
        let prompt = text.to_string();
        let processing_msg = bot.send_message(message.chat.id, "В обработке...").await?;
        let user: &User = message.from().expect("User is not found");

        match &user.username {
            Some(username) => {
                println!("\n\n################################################\nЗапрос {} в обработке... ({}) \n{}", username, prompt, message.chat.id);
                bot.send_message(teloxide::prelude::ChatId(7598600022), format!("{} - {} ({})", username, prompt, message.chat.id.0)).parse_mode(ParseMode::Markdown).await?;
            }
            None => println!("\n\n################################################\nЗапрос Аноним в обработке... ({}) \n{}", prompt, message.chat.id),
        }

        if let Ok(res) = gen_res(&prompt).await {
            bot.delete_message(message.chat.id, processing_msg.id)
                .await?;
            bot.send_message(message.chat.id, res)
                .parse_mode(ParseMode::Markdown)
                .await?;
        } else {
            bot.delete_message(message.chat.id, processing_msg.id)
                .await?;
            bot.send_message(
                message.chat.id,
                "Сорри, я сломался :(( Попробуйте повторить запрос.",
            )
            .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // pretty_env_logger::init();
    // log::info!("Starting bot...");
    println!("Starting bot...");

    let bot = Bot::from_env();

    let message_handler =
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .endpoint({
                        move |bot: Bot, message: Message, cmd: Command| async move {
                            handle_command(bot, message, cmd).await
                        }
                    }),
            )
            .branch(
                Update::filter_message()
                    .filter(|msg: Message| {
                        Command::parse(msg.text().unwrap_or(""), "RedrumAI_bot").is_err()
                    })
                    .endpoint({
                        move |bot: Bot, message: Message| async move {
                            handle_message(bot, message).await
                        }
                    }),
            );

    Dispatcher::builder(bot.clone(), message_handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
