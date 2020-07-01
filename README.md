# DidIAsk-rs
Did I ask, rewritten in Rust using serenity-rs

## Getting Started:
1. Clone this repository
1. Install Rust and Cargo. [Don't Know How?](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. Sign into Discord and get the bot registered. Note down the bot token and take care of it. Also set the permissions to send messages and read message history. [Don't Know How?](https://www.writebots.com/discord-bot-token/)
1. Using Discord, find out the channel ID you want the bot to run on. [Don't know how?](https://www.swipetips.com/how-to-get-channel-id-in-discord/)
1. Edit the file `src/main.rs` and replace the array `CHANNEL_LIST` with your channel IDs. Take care to set the size of the array to the correct value as well. Example
```rust
const CHANNEL_LIST: [&str; 2] = [
    "519875138979834798",
    "489023098423098423",
];
```
1. Edit the file `src/commands/owner.rs` and replace the `GULAG_ROLE` with your user ID. Getting your user ID is very similar to getting the channel ID. This gives you the permission to disallow users from using the bot. <!-- I think... -->
1. (Optional) Replace the ID in the file `src/commands/dia.rs` with the User ID of who you want to permanently disallow from using the `dia` command.
1. From the source directory, run `cargo build --release` to generate the binary. (If you want debug features, get rid of the `--release` flag)
1. Copy the generated binary (should be in the directory `target/release/DidIAsk` to a different (preferably empty) directory.
1. In the new directory, create a file called `.env` and place the following code inside of it, replacing the values needed:
```sh
DISCORD_TOKEN=YOUR_DISCORD_TOKEN # You got this earlier
PREFIX=! # What all commands start with
```
