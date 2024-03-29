[![Main](https://github.com/sousandrei/fika_bot/actions/workflows/main.yaml/badge.svg)](https://github.com/sousandrei/fika_bot/actions/workflows/main.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Fika Bot

FikaBot is a simple Slack bot to match two random people in a channel so they can have some [Fika](https://sweden.se/culture-traditions/fika)!

## Usage

First you are going to need a Slack application. FikaBot needs the following permissions

```
- commands
- im:write
- users:write
- channels:read
```

On top of that, you'll need to create a couple of slash commands, all pointing to a `/commands` url

```
/fika_start
/fika_stop
```

The application needs a couple of environment variables to function

```sh
# setup logging, non dev is json formatted
export RUST_LOG="info"
export ENV="dev"

# token for calling the start apis
export WEBHOOK_TOKEN="webhook_token"

export SLACK_TOKEN="..."
export SLACK_SIGNING_SECRET="..."

export PORT="8080"

# Make sure the database exists
export DB_USERNAME=root
export DB_PASSWORD=1234
export DB_HOST=localhost
export DB_PORT=3306
export DB_DATABASE=fikabot

# For operating SeaORM migration CLI
export DATABASE_URL="mysql://$DB_USERNAME:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_DATABASE"

```

Provide the application with those as enviroment variables and you are good to go!

## Running

To run the bot, it is recommended to setup a cron job/cloud scheduler to call the `/start_fika` endpoint.
This allow to run this bot at extreme low costs in something like Google Cloud Run with Google Cloud Scheduler calling it.

The image is build in a distroless manner, further minimizing size and complexity and can be easily extended.

## Contributing

Don't hesitate to ask for features and open PR's.
