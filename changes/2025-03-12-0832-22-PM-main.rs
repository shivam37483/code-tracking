use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;
use std::sync::Arc;
use spam_bot_mvp::rules::RuleManager;
use spam_bot_mvp::utils::{is_admin, notify_admins};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Bot commands")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Report a message as spam")]
    Report,
    #[command(description = "Add a custom spam rule (admin only, format: /add_rule <keyword> <score>)")]
    AddRule(String),
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    rule_manager: Arc<RuleManager>,
) -> Result<(), teloxide::RequestError> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Hello! I'm a spam filter bot.").await?;
        }
        Command::Report => {
            if let Some(reply) = msg.reply_to_message() {
                let text = reply.text().unwrap_or("(non-text message)");
                let is_spam = rule_manager.check_custom_rules(text) >= 5.0;
                bot.send_message(msg.chat.id, format!("Reported: {}\nSpam: {}", text, is_spam)).await?;
                if is_spam {
                    let user_id = reply.from().unwrap().id.to_string();
                    if let Err(e) = rule_manager.increment_sender_score(&user_id, true) {
                        log::error!("Failed to update sender score: {}", e);
                    }
                    notify_admins(&bot, msg.chat.id, text, &rule_manager, &user_id).await?;
                }
            } else {
                bot.send_message(msg.chat.id, "Please reply to a message to report it.").await?;
            }
        }
        Command::AddRule(args) => {
            if is_admin(&bot, &msg).await.unwrap_or(false) {
                let parts: Vec<&str> = args.split_whitespace().collect();
                if parts.len() == 2 {
                    let keyword = parts[0].to_string();
                    if let Ok(score) = parts[1].parse::<f32>() {
                        if let Err(e) = rule_manager.add_rule(keyword.clone(), score) {
... (truncated for brevity)