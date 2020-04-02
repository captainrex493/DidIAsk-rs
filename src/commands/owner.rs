use crate::ShardManagerContainer;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let _ = msg.channel_id.say(&ctx.http, "Shutting down!");

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }

    Ok(())
}
