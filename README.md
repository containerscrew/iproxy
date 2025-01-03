<p align="center" >
    <img src="img/world-map.svg" alt="logo" width="250"/>
<h3 align="center">iproxy</h3>
<p align="center">Built your own IPV4 geolocation database</p>
<p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Badges](#badges)
- [Introduction](#introduction)
- [How works](#how-works)
- [Example](#example)
- [Using the API](#using-the-api)
  - [Config.toml](#config.toml)
  - [Launch the `docker-compose`](#launch-the-`docker-compose`)
  - [Installing the binary](#installing-the-binary)
  - [Run your first query](#run-your-first-query)
  - [Api alive?](#api-alive?)
  - [Stop the stack](#stop-the-stack)
- [Local development](#local-development)
  - [Stop local mongodb](#stop-local-mongodb)
- [Visualize data](#visualize-data)
- [Import from local mongodb to mongodb atlas](#import-from-local-mongodb-to-mongodb-atlas)
  - [Install mongodb tools](#install-mongodb-tools)
  - [Dump local database](#dump-local-database)
  - [Import local database to mongodb atlas](#import-local-database-to-mongodb-atlas)
- [TO DO](#to-do)
- [Pending to fix](#pending-to-fix)
- [Useful Links](#useful-links)
- [LICENSE](#license)
<!-- END OF TOC -->

# Badges
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License](https://img.shields.io/github/license/containerscrew/iproxy)](/LICENSE)
![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/mtoc)
![Crates.io Version](https://img.shields.io/crates/v/iproxy)
![Latest Tag](https://img.shields.io/github/v/tag/containerscrew/iproxy?sort=semver)

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

# Example

![example](./img/example.png)

> API logs can be retrieved executing `docker logs -f iproxy` once the API is running. Plase visit the next step.

# Using the API

The setup is configured to work with docker-compose locally.

## Config.toml

With [this file](./config.toml) located in the root of this repository, you will be able to change some parameters.

## Launch the `docker-compose`

| :warning: WARNING           |
|:----------------------------|
| Before start the docker-compose, change the directory where you want to save the mongodb data     |

Example, from [compose.yml](compose.yml):

```yaml
  mongodb:
    ....other config
    volumes:
      - /mnt/ssd/iproxy:/data/db # this line!!
```

`In mi case, I'm mapping all the data using an external SSD mounted in /mnt/ssd`

Now, launch all the stack:

```bash
cp .env-example .env
# EDIT .env file as you need!!!!!
make compose-up-build
```

This will starts the `iproxy` container and `mongodb`.

## Installing the binary

You can install the binary using cargo:

```bash
cargo install iproxy
# Then run the command iproxy (config.toml is neccesary in the same dir)
```

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

# Local development

```shell
cargo install cargo-watch systemfd
docker network create iproxy
make local-development
```

## Stop local mongodb

```shell
make local-development-down
```

# Visualize data

Using [mongodb compass](https://www.mongodb.com/products/tools/compass) you can visualize your data from the collection `ips`

![data](img/mongo_data.png)

# Import from local mongodb to mongodb atlas

Import data from localhost to mongodb atlas

## Install mongodb tools

Visit the official webpage https://www.mongodb.com/docs/database-tools/installation/installation/.


## Dump local database

```shell
mongodump --uri="mongodb://admin:admin@localhost:27017/?maxPoolSize=20&w=majority"
```

This command will create a new `dump/` directory with the backup

## Import local database to mongodb atlas

> The uri of this command is the remote uri (mongodb atlas)

```shell
mongorestore --uri="mongodb+srv://USERNAME:PASSWORD@XXXXX.XXXX.mongodb.net/?retryWrites=true&w=majority" --db="ipfinder" --collection="ips" dump/iproxy/ips.bson
```

# TO DO

* Testing and error handling with custom errors ([MyErrors](./src/error.rs))
* Generate possible public ipv4 https://www.criminalip.io/ip-ranges
* Prometheus metrics
* JWT token
* Build takes too long due to `openssl` dependency.

# Pending to fix

* When the ip is private, the process crashes.

# Useful Links

* [Github](https://github.com/tokio-rs/axum)
* [API example](https://github.com/wpcodevo/simple-api-rust-axum)
* [API example mongodb](https://github.com/wpcodevo/rust-axum-mongodb)

# LICENSE

`iproxy` is distributed under the terms of the [`GNU AFFERO GENERAL PUBLIC LICENSE`](./LICENSE).
