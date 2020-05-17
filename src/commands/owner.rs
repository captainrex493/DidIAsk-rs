use crate::ShardManagerContainer;
use serenity::framework::standard::{macros::command, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::thread;
use std::time::Duration;
use log::error;

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

#[command]
#[owners_only]
fn gulag(ctx: &mut Context, msg: &Message) -> CommandResult {
    let list = msg.content.split_whitespace();
    let time = list.last().unwrap().parse::<u64>().unwrap();
    let _user = &msg.mentions[0];

    let mut _member =match ctx.http.get_member(msg.guild_id.unwrap().0,_user.id.0){
        Ok(_member) => _member,
        Err(_) => {
            error!("Failed to parse message information");
            return Err(CommandError(String::from("Couldn't parse user")));
        }
    };
    let _roles = _member.clone().roles;
    for i in _roles.clone() {
        match _member.remove_role(&ctx.http,i) {
            Ok(()) => (),
            Err(_) => {
                error!("Couldn't remove role {}",i);
                return Err(CommandError(String::from("Couldn't parse stuff")));
            }
        }
    }
    if let Err(_) = _member.add_role(&ctx.http, 691541303707566100) {
        error!("Couldn't give gulag role");
        return Err(CommandError(String::from("Couldn't parse stuff")));
    };

    if let Err(_) = _member.user.read().direct_message(&ctx,|m|
        m.content(format!("You will be released in {} minute(s)",time))) {
        error!("Couldn't give gulag role");
        return Err(CommandError(String::from("Couldn't parse stuff")))
    };

    thread::sleep(Duration::from_secs(time*60));

    for i in _roles.clone() {
        match _member.add_role(&ctx.http,i) {
            Ok(()) => (),
            Err(_) => {
                error!("Couldn't give role {}", i);
                return Err(CommandError(String::from("Couldn't parse stuff")));
            }
        }
    }
    match _member.remove_role(&ctx.http,705895621961187710) {
        Ok(()) => (),
        Err(_) => {
            error!("Could not remove gulag role");
            return Err(CommandError(String::from("Couldn't remove gulag role")))
        }
    }
    Ok(())






}