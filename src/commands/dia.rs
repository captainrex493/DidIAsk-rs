use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::commands::message_generation;

#[command]
fn dia(ctx: &mut Context, msg: &Message) -> CommandResult {
    
    let _ = msg.channel_id.say(&ctx.http, message_generation::get_dia_string());

    Ok(())
}