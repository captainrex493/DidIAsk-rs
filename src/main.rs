//hi :)
//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
use std::borrow::Borrow;
use std::{collections::HashSet, env, sync::Arc};

use log::{debug, error, info};
use rand::Rng;
use serenity::model::channel::{Message, GuildChannel};
use serenity::model::gateway::Activity;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use commands::{dia::*, math::*, meta::*, owner::*, message_generation::*, emojis::*};
use serenity::model::guild::{Guild, Member};
use serenity::model::user::User;

mod commands;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

const CHANNEL_LIST: [&str; 4] = [
    "530491424529973260",
    "535663746555707394",
    "675993960178647040",
    "569586147160883241",
];

pub struct MessageInfo {
    channel: Arc<RwLock<GuildChannel>>,
    guild: Arc<RwLock<Guild>>,
    member: Member,
    user: Arc<RwLock<User>>,
}

pub fn get_message_info(_ctx: Context,msg: Message) -> Result<MessageInfo, bool> {
    let _channel = match _ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel,
        None => return Err(true),
    };
    let _guild = match msg.guild(&_ctx.cache) {
        Some(guild) => guild,
        None => return Err(true),
    };
    let _member = match _ctx.cache.read().member(_guild.read().id, msg.author.id) {
        None => return Err(true),
        Some(member) => member,
    };
    let _user = match _ctx.cache.read().user(msg.author.id) {
        Some(user) => user,
        None => return Err(true),
    };
    Ok(MessageInfo {
        channel: _channel,
        guild: _guild,
        member: _member,
        user: _user
    })
}

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let _message_info = match get_message_info(_ctx.clone(), msg.clone()) {
            Ok(message_info) => message_info,
            Err(_) => {
                error!("Failed to parse message information");
                return;
            },
        };

        info!(
            "[{}][{}] {}: {}",
            _message_info.guild.read().name,
            _message_info.channel.read().name,
            _message_info.user.read().name,
            msg.content
        );

        let mut allowed_channel = false;
        for str in &CHANNEL_LIST {
            if msg.channel_id.to_string().as_str() == str.to_string() {
                allowed_channel = true;
            }
        }

        if !allowed_channel {
            return;
        }
        let should_ask = rand::thread_rng().gen_range(1, 101);
        if msg.author.to_string() == "<@207686242874294272>" {
            info!("Should I ask is {}", should_ask);
            if should_ask < 10 {
                let _ = msg.channel_id.say(&_ctx.http, get_dia_string());
                info!("Did I asked {}", msg.author)
            }
        } else {
            if should_ask < 5 {
                let _ = msg.channel_id.say(&_ctx.http, get_dia_string());
                info!("Did I asked {}", msg.author);
            }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        ctx.set_activity(Activity::listening("prisoners' cries"))
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(multiply, ping, quit, dia, gulag)]
struct General;

#[group]
#[commands(owo, uwu, smile, hug, flex, animal, surprise, dance, shrug, flip, unflip, sus, cri, yike, bear, fight)]
struct Emoji;

fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.owners(owners)
                    .prefix(env::var("PREFIX").expect("Expected Prefix").borrow())
            })
            .group(&GENERAL_GROUP)
            .group(&EMOJI_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
