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
SLACK_TOKEN='xoxb-......'
SLACK_SIGNING_SECRET='...'
SHEETS_ID='123123123'
ACCOUNT_EMAIL='some-account@project.iam.gserviceaccount.com'
CREDENTIALS='...'
```

`SLACK_TOKEN` Is the bot token of your slack application, denotet by starting with `xoxb`

`SLACK_SIGNING_SECRET` is the secret provided by slack to sign their own requests, we need this since the endpoint needs to be publicly exposed

`SHEETS_ID` is the Id of you Google Sheets file, our "database"

`ACCOUNT_EMAIL` is the email for your service account on GCP to use to authenticate to the Google Sheets API

`CREDENTIALS` is the credentials file for said account

Provide the application with those as enviroment variables and you are good to go!

## Building

No secret here

```bash
cargo build --release
```

## Contributing

Don't hesitate to ask for features and open PR's.
