use crate::rules::RuleManager;
use teloxide::errors::RequestError;
use teloxide::prelude::Requester;
/// A module providing utility functions for Telegram bot administration and notifications.
///
/// This module contains helper functions to check user admin status and notify administrators
/// about spam detection events. It leverages the `teloxide` library for Telegram interactions
/// and integrates with the `rules` module for spam score management.
use teloxide::{
    types::{ChatId, Message},
    Bot,
};

/// Checks if a user is an administrator in the given chat.
///
/// Determines whether the sender of a message is an admin. In private chats,
/// all users are considered admins by default. In group chats, it queries the
/// Telegram API to fetch the list of administrators and checks if the user's
/// ID is included.
///
/// # Arguments
/// * `bot` - A reference to the Telegram bot instance.
/// * `msg` - A reference to the message containing the user and chat context.
///
/// # Returns
/// * `Result<bool>` - A `Result` containing `true` if the user is an admin,
///   `false` otherwise, or a `RequestError` if the API call fails.
///
/// # Panics
/// * Panics if `msg.from()` is `None` (i.e., no sender information).
pub async fn is_admin(bot: &Bot, msg: &Message) -> Result<bool, RequestError> {
    if msg.chat.is_private() {
        Ok(true)
    } else {
        let admins = bot.get_chat_administrators(msg.chat.id).await?;
        let user_id = msg.from().unwrap().id;
        log::info!(
            "Checking admin status for user {} in chat {}",
            user_id,
            msg.chat.id
        );
        let is_admin = admins.iter().any(|admin| {
            log::info!("Admin found: {}", admin.user.id);
            admin.user.id == user_id
        });
        Ok(is_admin)
    }
}

/// Notifies administrators about a detected spam message.
... (truncated for brevity)