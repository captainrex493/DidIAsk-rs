# DidIAsk-rs
Did I ask, rewritten in Rust using serenity-rs.

This bot randomly comments "Did I ask?" on specified channels (along with other goodies).

## Building and Configuring:
1. Clone this repository
1. Install Rust and Cargo. [Don't Know How?](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. Sign into Discord and get the bot registered. Note down the bot token and take care of it. Also set the permissions to send messages and read message history. [Don't Know How?](https://www.writebots.com/discord-bot-token/)
1. Using Discord, find out the channel ID you want the bot to run on. [Don't know how?](https://www.swipetips.com/how-to-get-channel-id-in-discord/)

1. From the source directory, run `cargo build --release` to generate the binary. (If you want debug features, get rid of the `--release` flag)
1. Copy the generated binary (should be in the directory `target/release/DidIAsk` to a different (preferably empty) directory.
1. In the new directory, create a file called `.env` and place the following code inside of it, replacing the values needed or copy the `.env.example` file and rename it and replace the values needed.
```sh
DISCORD_TOKEN=YOUR_DISCORD_TOKEN # You got this earlier
PREFIX=! # What all commands start with
RUST_LOG=error # The options are info, error, or debug

CHANNEL_WHITELIST=519875138979834798,489023098423098423,705895622074433549 # Channel ids that the bot is allowed to randomly did i ask
GULAG_ROLE=705895621961187710 # Id a disciplinary role
STATUS_PREFIX=listening # Prefix of the bot status, either playing or listening
STATUS_PHRASE=-h # Status of the bot
```

## Commands:
1. Core:
    1. `dia`: Bot comments a stylized "Did I ask?"
1. Emojis / Emoticons:
    1. `uwu`: Bot comments a "uwu" kaomoji.
    1. `owo`: Bot comments a "owo" kaomoji.
    1. `smile`: Bot comments a random smiling kaomoji.
    1. `hug`: Bot comments a random hugging kaomoji.
    1. `flex`: Bot comments a random flexing kaomoji.
    1. `animal`: Bot comments a random animal kaomoji.
    1. `surprise`: Bot comments a random suprised kaomoji.
    1. `dance`: Bot comments a random dancing kaomoji.
    1. `shrug`: Bot comments a random shrugging kaomoji. 
    1. `flip`: Bot comments a random "table flip" kaomoji.
    1. `unflip`: Bot comments a random "table unflip" kaomoji.
    1. `sus`: Bot comments a random suspicious kaomoji.
    1. `cri`: Bot comments a random crying kaomoji.
    1. `yike`: Bot comments a random "yikes" kaomoji.
    1. `bear`: Bot comments a random bear kaomoji / emoji.
    1. `fight`: Bot comments a random fighting kaomoji.
1. Math:
    1. `multiply`: Bot multiplies two given numbers.
1. Administration (only owner is allowed to run):
    1. `quit`: Bot quits.
    1. `Gulag`: Bot adds user to Gulag role specified in configuration.
1. Misc
    1. `ping`: Pong!
