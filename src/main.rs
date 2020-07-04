use std::borrow::Borrow;
use std::{collections::{HashMap,HashSet}, env, sync::Arc};

use log::{debug, error, info};
use rand::Rng;
use serenity::model::channel::{Message, GuildChannel};
use serenity::model::gateway::Activity;
use serenity::model::gateway::ActivityType;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use global::{GlobalInformation, GlobalKeys};

use commands::{dia::*, math::*, meta::*, owner::*, message_generation::*, emojis::*};
use serenity::model::guild::{Guild, Member};
use serenity::model::user::User;

mod commands;
mod global;


struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}



struct Handler;

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
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        
        let _message_info = match get_message_info(ctx.clone(), msg.clone()) {
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

        let data = ctx.data.read();
        let global = data.get::<GlobalInformation>().expect("Expected GlobalInformation in ShareMap");

        let channel_whitelist = {
            
            match global.get(&GlobalKeys::ChannelWhitelist) {
                Some(value) => value,
                None => {
                    error!("No whitelist");
                    return;
                },
    
            }
        };

        let mut allowed_channel = false;
        for str in channel_whitelist {
            if msg.channel_id.to_string().as_str() == str.as_str() {
                allowed_channel = true;
            }
        }

        if !allowed_channel {
            return;
        }
        let number = rand::thread_rng().gen_range(1, 101);
        info!("Should I ask number is {}", number);

        if number < 5 {
            let _ = msg.channel_id.say(&ctx.http, get_dia_string());
                info!("Did I asked {}", msg.author);
        }

    }

    fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        let data = ctx.data.read();
        let global = data.get::<GlobalInformation>().expect("Expected GlobalInformation in ShareMap");

        let status_prefix = {
            
            let prefix = match global.get(&GlobalKeys::StatusPrefix) {
                Some(value) => value.get(0).unwrap().to_owned(),
                None => {
                    info!("No status prefix found, using default");
                    "".to_string()
                },
    
            };

            match prefix.as_str() {
                "playing" => ActivityType::Playing,
                "streaming" => ActivityType::Streaming,
                "listening" => ActivityType::Listening,
                _ => ActivityType::Custom,
            }
        };

            
        match global.get(&GlobalKeys::StatusPhrase) {
            Some(value) => {
                let phrase = value.get(0).unwrap();
                match (status_prefix, phrase) {
                    (ActivityType::Playing, _) => {
                        ctx.set_activity(Activity::playing(phrase));
                    },
                    (ActivityType::Listening, _) => {
                        ctx.set_activity(Activity::listening(phrase))
                    },
                    (_,_) => {
                        info!("Using default status")
                    }
                };
            },
            None => {
                info!("No phrase found, using default");               
            },
    
        };

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
        data.insert::<GlobalInformation>(HashMap::default());

        let global = data.get_mut::<GlobalInformation>().expect("Expected GlobalInformation Hashmap");

        let channel_whitelist = 
        match env::var("CHANNEL_WHITELIST") {
            Ok(val) => Some(val),
            Err(e) => {
                error!("No whitelist found: {}", e);
                None
            }
        };
        match channel_whitelist {
            Some(val) => {
                let channel_vec = val
                .split(',')
                .into_iter()
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
                global.insert(GlobalKeys::ChannelWhitelist, channel_vec);
                info!("Added channel whitelist");

            },
            None => {
                info!("No channel whitelist")
            },
        };

        match env::var( "GULAG_ROLE") {
            Ok(val) => {
                info!("Added gulag role id");
                global.insert(GlobalKeys::GulagRole, vec![val])
            },
            Err(e) => {
                info!("No gulag role id found: {}", e);
                None
            }

        };

        match env::var("STATUS_PREFIX") {
            Ok(val) => {
                info!("added status prefix");
                global.insert(GlobalKeys::StatusPrefix, vec![val]);
            },
            Err(e) => {
                info!("No status prefix found: {}", e);
            },
        };

        match env::var("STATUS_PHRASE") {
            Ok(val) => {
                info!("added status prefix");
                global.insert(GlobalKeys::StatusPhrase, vec![val]);
            },
            Err(e) => {
                info!("No status phrase found: {}", e);
            },
        };


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
