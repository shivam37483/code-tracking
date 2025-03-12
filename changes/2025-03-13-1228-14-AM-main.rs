use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;
use std::sync::Arc;
use spam_bot_mvp::rules::RuleManager;
use spam_bot_mvp::utils::{is_admin, notify_admins};

/// The main entry point for the Telegram spam detection bot.
///
/// This module sets up a Telegram bot using the `teloxide` framework to detect spam messages
/// in chats, manage custom spam rules, and notify administrators. It integrates with the
/// `rules` module for spam detection logic and the `utils` module for admin-related utilities.
///
/// The bot supports the following features:
/// - Responds to commands (`/start`, `/report`, `/add_rule`) for bot interaction.
/// - Automatically checks incoming messages for spam using custom Lua rules.
/// - Notifies admins when spam is detected, with a fallback to group notifications.
/// - Logs bot activity and errors using the `log` crate and `env_logger`.
///
/// # Dependencies
/// - `teloxide`: For Telegram bot API interactions.
/// - `dotenv`: For loading environment variables (e.g., bot token).
/// - `std::sync::Arc`: For thread-safe sharing of the `RuleManager`.
/// - `spam_bot_mvp::rules`: For spam detection and rule management.
/// - `spam_bot_mvp::utils`: For admin checks and notifications.
///
/// # Environment Variables
/// - `TELOXIDE_TOKEN`: The Telegram bot token, loaded from a `.env` file or environment.
///
/// # Examples
/// To run the bot:
/// 1. Create a `.env` file with `TELOXIDE_TOKEN=your_bot_token`.
/// 2. Ensure a `rules.db` SQLite database exists (created automatically if not).
/// 3. Run the bot with `cargo run`.
///
/// The bot will respond to commands in Telegram chats and detect spam messages.
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Bot commands")]
enum Command {
    /// Starts the bot and sends a welcome message.
    #[command(description = "Start the bot")]
    Start,

    /// Reports a message as spam by replying to it.
    #[command(description = "Report a message as spam")]
    Report,

    /// Adds a custom spam rule (admin only).
    ///
    /// Format: `/add_rule <keyword> <score>`.
    /// Example: `/add_rule spam 10.0` adds a rule to flag "spam" with a score of 10.0.
... (truncated for brevity)