<p align="center" >
    <img src="assets/rust-logo.svg" alt="logo" width="250"/>
<h3 align="center">ipfinder</h3>
<p align="center">Built your own IPV4 geolocation database</p>
<p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Badges](#badges)
- [Introduction](#introduction)
- [How works](#how-works)
- [Using the API](#using-the-api)
  - [Config.toml](#config.toml)
  - [Launch the `docker-compose`](#launch-the-`docker-compose`)
  - [Run your first query](#run-your-first-query)
  - [Api alive?](#api-alive?)
  - [Visualize data](#visualize-data)
- [Import from local mongodb to mongodb atlas](#import-from-local-mongodb-to-mongodb-atlas)
  - [Install mongodb tools](#install-mongodb-tools)
  - [Dump local database](#dump-local-database)
  - [Import local database](#import-local-database)
- [TO DO](#to-do)
- [Useful Links](#useful-links)
- [LICENSE](#license)
<!-- END OF TOC -->

# Badges
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License](https://img.shields.io/github/license/containerscrew/iproxy)](/LICENSE)
![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/mtoc)

# Introduction

So imagine that you want to start indexing public IPv4 geolocation data. There are paid services, others free but requiring registration, that offer this type of content.
By consulting one of these services you will obtain the data you need.

I do not know exactly all the public services that offer this data, in my case I have used the public database of **[ip-api.com](http://ip-api.com)**

> Without this external service this tool does not work, it would be necessary to refactor and use another API and map the responses.

Other possible solutions (not implemented):
* https://www.maxmind.com/en/home
* https://ipstack.com/
* https://ip.guide


# How works

You will make a request to your API endpoint, for example, `curl http://127.0.0.1/api/v1/1.1.1.1`, and the API will first check if the data exists in the database. If it does not exist, it will retrieve the information from the external website mentioned above. The next time you query the same IP, the data will be retrieved from MongoDB, avoiding the external query.

# Using the API

The setup is configured to work with docker-compose locally.

## Config.toml

With [this file](./config.toml) located in the root of this repository, you will be able to change some parameters.

## Launch the `docker-compose`

| :warning: WARNING           |
|:----------------------------|
| Before start the docker-compose, change the directory where you want to save the mongodb data     |

Example, from [compose.yml](./compose.yml):

```yaml
  mongodb:
    ....other config
    volumes:
      - ~/Documents/DockerData/mongoIproxy:/data/db # this line!!
```

Now, launch all the stack:

```bash
make compose-up-build
```

This will starts the `iproxy` container and `mongodb`.

## Run your first query

```bash
curl http://127.0.0.1:8000/api/v1/1.1.1.1
```

> http://ip:port/api/v1/ip-to-query

## Api alive?

```bash
curl http://127.0.0.1:8000/api/v1/health
```

## Stop the stack

```bash
make compose-down
```

## Visualize data

Using [mongodb compass](https://www.mongodb.com/products/tools/compass) you can visualize your data from the collection `ips`

![data](assets/mongo_data.png)

# Import from local mongodb to mongodb atlas

## Install mongodb tools

```shell
brew install mongodb/brew/mongodb-database-tools
```

If you are not using OSX, please visit the official documentation to install `mongodump` and `mongorestore`

## Dump local database

```shell
mongodump --uri="mongodb://admin:admin@localhost:27017/?maxPoolSize=20&w=majority"
```

This command will create a new `dump/` directory with the backup

## Import local database

```shell
mongorestore --uri="mongodb+srv://USERNAME:PASSWORD@XXXXX.XXXX.mongodb.net/?retryWrites=true&w=majority" --db="ipfinder" --collection="ips" dump/ipfinder/ips.bson
```

# TO DO

* Testing and error handling with custom errors ([MyErrors](./src/error.rs))

# Useful Links

* [Github](https://github.com/tokio-rs/axum)
* [API example](https://github.com/wpcodevo/simple-api-rust-axum)
* [API example mongodb](https://github.com/wpcodevo/rust-axum-mongodb)

# LICENSE

[LICENSE](./LICENSE)
