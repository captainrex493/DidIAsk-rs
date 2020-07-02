# DidIAsk-rs
Did I ask, rewritten in Rust using serenity-rs.

This bot randomly comments "Did I ask?" on specified channels (along with other goodies).

## Building and Configuring:
1. Clone this repository
1. Install Rust and Cargo. [Don't Know How?](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. Sign into Discord and get the bot registered. Note down the bot token and take care of it. Also set the permissions to send messages and read message history. [Don't Know How?](https://www.writebots.com/discord-bot-token/)
1. Using Discord, find out the channel ID you want the bot to run on. [Don't know how?](https://www.swipetips.com/how-to-get-channel-id-in-discord/)
1. Edit the file `src/identifications.rs` and replace the array `CHANNEL_LIST` with the channel IDs you want the bot to randomly comment "Did I ask?" in. Take care to set the size of the array to the correct value as well. Example
```rust
const CHANNEL_LIST: [&str; 2] = [
    "519875138979834798",
    "489023098423098423",
];
```
1. Edit the file `src/commands/identifications.rs` and replace the `GULAG_ROLE` with the administrator role in Discord. Getting the role ID is very similar to getting the channel ID. Users in this role must be shamed.
1. (Optional) Replace the ID list in the file `src/identifications.rs` with the User IDs of who you want to permanently disallow from using the bot command. Replace the strings in the list with the proper ID in the format `<@ID_GOES_HERE>`. Similar to the `CHANNEL_LIST` format.
1. From the source directory, run `cargo build --release` to generate the binary. (If you want debug features, get rid of the `--release` flag)
1. Copy the generated binary (should be in the directory `target/release/DidIAsk` to a different (preferably empty) directory.
1. In the new directory, create a file called `.env` and place the following code inside of it, replacing the values needed:
```sh
DISCORD_TOKEN=YOUR_DISCORD_TOKEN # You got this earlier
PREFIX=! # What all commands start with
RUST_LOG=error # The options are info, error, or debug
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
