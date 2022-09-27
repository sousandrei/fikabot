# Changelog

## [0.1.1](https://github.com/sousandrei/fikabot/compare/v0.1.0...v0.1.1) (2022-09-27)


### Bug Fixes

* **fika:** channel name not escaped ([#110](https://github.com/sousandrei/fikabot/issues/110)) ([b60fbb5](https://github.com/sousandrei/fikabot/commit/b60fbb54c0b60ab6f5c5426642ef51d0c078f6dd))

## 0.1.0 (2022-08-11)


### Features

* Adding channel for messages ([8a0cc09](https://github.com/sousandrei/fikabot/commit/8a0cc09ba8f0808dd5d151c53bf8ca49eabaaa0a))
* adding dependabot ([8a2b9f0](https://github.com/sousandrei/fikabot/commit/8a2b9f0db30f8e86ac474ce6172fad1237f831a2))
* adding envy to parse environment variables ([1462fb2](https://github.com/sousandrei/fikabot/commit/1462fb2ce7e4cc169bad16f04e5147e5c93ead8a))
* Adding general guard ([a75b1f7](https://github.com/sousandrei/fikabot/commit/a75b1f77e8ee00d138de4a745af1a180bc69f37e))
* Adding ping endpoint and google cloud run vars ([26a10f2](https://github.com/sousandrei/fikabot/commit/26a10f25564a42bf4368522abc79662265432706))
* adding seaorm and migrations ([4c5c438](https://github.com/sousandrei/fikabot/commit/4c5c4389f938a2bf069afdae6934adedaafb5f05))
* Adding slack request signing ([6635e6d](https://github.com/sousandrei/fikabot/commit/6635e6d1a0733f168ab1ee4f843d7f1d4d242ed2))
* Adding song command ([fd2b891](https://github.com/sousandrei/fikabot/commit/fd2b8911a9d8a2fb4027b2ac4372cd1c1d58ffcd))
* adding song cron job and storage on db ([22f3b57](https://github.com/sousandrei/fikabot/commit/22f3b57afe25ed788527589b3889d184f921e275))
* Adding tini and reducing image size ([67268ab](https://github.com/sousandrei/fikabot/commit/67268ab534bcec3a6d1c54083d3b5324e44f9c7d))
* command to trigger fika now ([75b6231](https://github.com/sousandrei/fikabot/commit/75b6231dcf6cf515bb66b82c10b0affa0ae1852e))
* Going distroless for docker ([478d2ac](https://github.com/sousandrei/fikabot/commit/478d2aca9229519010cb8301210e09e013864fd2))
* logging on json with env variable ([c490896](https://github.com/sousandrei/fikabot/commit/c4908969e83b4bf4a5cd26d98e5c67222a88b713))
* matchmaking based on users gotten from slack ([1a75705](https://github.com/sousandrei/fikabot/commit/1a7570545e1a7c7ecd8281410b918cf7c470c074))
* migrating to sqlx ([d566628](https://github.com/sousandrei/fikabot/commit/d5666286896510f54f7e48101fbdb82db51d5c34))
* Migrating to tide and async-std from warp and tokio ([ef60afe](https://github.com/sousandrei/fikabot/commit/ef60afe7ba365380a7aa4f89c9436caf7b87f555))
* moving to a channel based logic ([75506da](https://github.com/sousandrei/fikabot/commit/75506da4aef8cdd8169730d587e3381e304eccfb))
* removing cron job in favor of http webhook ([90c967e](https://github.com/sousandrei/fikabot/commit/90c967e12054e37dc4d6f7e5fa06430d8d439033))
* Replacing mongo for google sheets ([ca18f99](https://github.com/sousandrei/fikabot/commit/ca18f99d3635ecb532d4a6c30384b4b09afdc852))
* Validating song urls ([7fbd059](https://github.com/sousandrei/fikabot/commit/7fbd059bfedfd387dabd60811596bd68fde8ceb4))


### Bug Fixes

* Actually setting db on update_one ([646f7be](https://github.com/sousandrei/fikabot/commit/646f7be38e4f80bcc034dcc458b369ea555fb793))
* Change time to GMT on cron jobs ([bd5aeef](https://github.com/sousandrei/fikabot/commit/bd5aeef65c8738501d03398f12943a3cc20e2ca3))
* Dockerfile building and Taskfile releasing ([e47ec7a](https://github.com/sousandrei/fikabot/commit/e47ec7a3e523605a48d0fb6a223c32025b4d3325))
* excluding bot from algos ([c691818](https://github.com/sousandrei/fikabot/commit/c691818b68b1dadc86655c061f1fdca14d7d1e47))
* fix build env ([73482c3](https://github.com/sousandrei/fikabot/commit/73482c3e103bdf303ec2664a39fa2fe4bf50723a))
* fix duplicate adding to db ([3a2608a](https://github.com/sousandrei/fikabot/commit/3a2608a847e0aa2921b7804ddfa6f202b61bc4c4))
* Fixed a bug on cron timing ([288dc89](https://github.com/sousandrei/fikabot/commit/288dc89df053761f05860c669293873013a29365))
* Fixing binary on dockerfile and caching build ([be00e76](https://github.com/sousandrei/fikabot/commit/be00e76afa866d7a271e2523afb8d8604369ab31))
* Fixing bug on trio message routing ([3a2e7f8](https://github.com/sousandrei/fikabot/commit/3a2e7f89a1a7c4f95a3905d5b1f6a3f6eb39fe81))
* Fixing env variable bug when PORT not present ([f1b62b3](https://github.com/sousandrei/fikabot/commit/f1b62b3274f99e0baf1b186e57069896704a54ce))
* Fixing parameter order on fika message ([62417be](https://github.com/sousandrei/fikabot/commit/62417be5a3ac4e29bdce00320b0b832d52260e9d))
* Let's not be our own pairs ([e26d6c9](https://github.com/sousandrei/fikabot/commit/e26d6c9258437da48b759508731bde21a23d2a8e))
* removing blocking client ([f1946e4](https://github.com/sousandrei/fikabot/commit/f1946e4b9338c5291d01c28ec4fab68c8681e4bc))
* **task:** git commit to tag version ([eb0eb5c](https://github.com/sousandrei/fikabot/commit/eb0eb5c36ccb5c0980aff5ab85140cfeb187e488))
* **task:** vars is not a map ([456f702](https://github.com/sousandrei/fikabot/commit/456f7020b306c5645435ee1955709235553b272b))
* updating user ([ec9e133](https://github.com/sousandrei/fikabot/commit/ec9e133f3d461ec5d78a9261dc68449292b61c51))
