use crate::commands::Command;
use teloxide::prelude::*;

pub async fn invoke(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(message.chat.id, "Никто тебе не поможет...")
                .await?
        }
        Command::Start => {
            bot.send_message(message.chat.id, "Спрашивай что угодно! Я умнее ЧатГПТ")
                .await?
        }
    };

    Ok(())
}
