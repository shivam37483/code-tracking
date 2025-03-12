use teloxide::{Bot, types::{Message, ChatId}};
use teloxide::errors::RequestError;
use teloxide::prelude::Requester;


pub async fn is_admin(bot: &Bot, msg: &Message) -> Result<bool, RequestError> {
    let admins = bot.get_chat_administrators(msg.chat.id).await?;
    let user_id = msg.from().unwrap().id;
    Ok(admins.iter().any(|admin| admin.user.id == user_id))
}

pub async fn notify_admins(bot: &Bot, chat_id: ChatId, text: &str) -> Result<(), RequestError> {
    let admins = bot.get_chat_administrators(chat_id).await?;
    for admin in admins {
        bot.send_message(admin.user.id, format!("Spam detected: {}", text)).await?;
    }
    Ok(())
}