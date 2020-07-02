use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::commands::message_generation;
use crate::identifications::users;

#[command]
fn dia(ctx: &mut Context, msg: &Message) -> CommandResult {
    let author = msg.author.to_string();
    if users::NEVER_ASKED.iter().any(|v| v == &author) {
        return Ok(());
    };
    let _ = msg.channel_id.say(&ctx.http, message_generation::get_dia_string());

    Ok(())
}
