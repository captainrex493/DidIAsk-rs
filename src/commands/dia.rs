use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn dia(ctx: &mut Context, msg: &Message) -> CommandResult {
    if msg.author.to_string() == "<@207686242874294272>" {
        return Ok(());
    };
    let _ = msg.channel_id.say(&ctx.http, "Did I Ask?");

    Ok(())
}
