use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::Rng;

#[command]
fn dia(ctx: &mut Context, msg: &Message) -> CommandResult {
    if msg.author.to_string() != "<@483297157580062732>" {
        return Ok(());
    };
    let _ = msg.channel_id.say(&ctx.http, get_dia_string());

    Ok(())
}

fn get_dia_string() -> String {
    let mut s = String::from("");
    let mut i = 0;
    while i < 10 {
        if i == 0 || i == 2 {
            s.push_str(D_LIST[rand::thread_rng().gen_range(0, 4)]);
        }
        if i == 1 || i == 4 {
            s.push_str(I_LIST[rand::thread_rng().gen_range(0, 4)]);
        }
        if i == 3 || i == 5 {
            s.push_str(" ");
        }
        if i == 6 {
            s.push_str(A_LIST[rand::thread_rng().gen_range(0, 4)]);
        }
        if i == 7 {
            s.push_str(S_LIST[rand::thread_rng().gen_range(0, 4)]);
        }
        if i == 8 {
            s.push_str(K_LIST[rand::thread_rng().gen_range(0, 4)]);
        }
        if i == 9 {
            s.push_str(QMARK_LIST[rand::thread_rng().gen_range(0, 4)]);
        }
        i+= 1;
    }
    return s;
}

const D_LIST: [&str; 4] = [
    "d",
    "D",
    "ɖ",
    "ɗ",
];

const I_LIST: [&str; 4] = [
    "i",
    "I",
    "¡",
    "ϊ",
];

const A_LIST: [&str; 4] = [
    "a",
    "A",
    "∆",
    "@",
];

const S_LIST: [&str; 4] = [
    "s",
    "S",
    "$",
    "$",
];

const K_LIST: [&str; 4] = [
    "k",
    "K",
    "ꝅ",
    "ꝅ",
];

const QMARK_LIST: [&str; 4] = [
    "⁉",
    "⁇",
    "‽",
    "‽",
];