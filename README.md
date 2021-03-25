[![Main](https://github.com/sousandrei/fika_bot/actions/workflows/main.yaml/badge.svg)](https://github.com/sousandrei/fika_bot/actions/workflows/main.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Fika Bot

FikaBot is a simple Slack bot to match two random registered people for them to have some [Fika](https://sweden.se/culture-traditions/fika)!

<br>

## Usage

First you are going to need a Slack application. FikaBot needs only the permission to send DM's to users.

The application itself only needs 2 environment variables to function!

```sh
SLACK_TOKEN='xoxb-......'
MONGO_URL='mongodb://127.0.0.1'
```

`SLACK_TOKEN` Is the bot token of your slack application, denotet by starting with `xoxb`

`MONGO_URL` is the mongo connection string for the bot to store it's data

Provide the application with those as enviroment variables and you are good to go!

## Building

No secret here

```bash
cargo build --release
```

<br>

## Contributing

Don't hesitate to ask for features and open PR's.

<br>

#### Pending Features

- Randomized messages when interacting with the user
- Store past fikas to try to avoid repetition
- Conversation topic generator
