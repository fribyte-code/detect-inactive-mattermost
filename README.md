# Detecting Inactive Members in Mattermost

We use Mattermost to coordinate attendance at 'dugnad', our weekly meetings.
The way we do it is having a bot posting announcements that we react to with a :white_check_mark: if we're attending.

<p><img src="https://raw.githubusercontent.com/fribyte-code/detect-inactive-mattermost/refs/heads/main/figures/dugnad-bot-message.png?sanitize=true" width="360"></p>

With many members, some people, especially newer members might stop attending without giving a notice of it.
Though we of course don't like this situation, we need to keep our member list up to date as an adminstrative matter.
Querying the Mattermost API is a method to check for this situation, and is the goal of this project.

## Using

1. Install Rust and Cargo, for example using https://rustup.rs
2. Run `cargo run` in this repo
3. Enter your Mattermost username at the prompt
4. Enter your Mattermost password

A table will be printed. This contains the username of each user, the last date where they reacted with :white_check_mark: to a message from *friBot* in the *dugnad* channel, and the total number of :white_check_mark: reactions they've made to such messages.
In the situation where the user is deleted, it will show 'unknown user \[their id\]'.

## Developing

Pay attention to the [Mattermost API Reference](https://api.mattermost.com).
This program uses the `mattermost_api` crate. This crate has very few built-in functions, and more or less requires you to formulate your requests manually.
This formulation has been done by consulting the api, and generating some structs for deserializing responses using this website: [https://transform.tools/json-to-rust-serde](https://transform.tools/json-to-rust-serde).
This did not turn out perfectly for a couple of reasons, so the structs defined in [src/apitypes.rs](src/apitypes.rs) are not exact matches of what the API will return.

Take note that our chat server URL, friBot's user id, and the dugnad channels' channel id are hardcoded.

## Copyright and License

TODO

