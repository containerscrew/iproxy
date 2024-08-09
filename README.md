<p align="center" >
    <img src="assets/rust-logo.svg" alt="logo" width="250"/>
<h3 align="center">ipfinder</h3>
<p align="center">Built your own IPV4 geolocation database</p>
<p align="center">Build with ❤ in Rust</p>
<p align="center">Don't judge my code, I've only been programming in RUST for 3 weeks without being a developer :)</p>
</p>

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Badges](#badges)
- [Introduction](#introduction)
- [How works](#how-works)
- [Local development](#local-development)
  - [Requirements](#requirements)
  - [Clone the repository](#clone-the-repository)
  - [Set your .env file with the necessary credentials](#set-your-env-file-with-the-necessary-credentials)
  - [Start your local mongodb using a container](#start-your-local-mongodb-using-a-container)
- [Running in local](#running-in-local)
  - [Cargo run with autoload](#cargo-run-with-autoload)
  - [Cargo in local dev](#cargo-in-local-dev)
  - [Build](#build)
  - [Running the API](#running-the-api)
  - [Going to production](#going-to-production)
- [Examples](#examples)
  - [Inserting IP](#inserting-ip)
  - [Getting IP info](#getting-ip-info)
  - [Updating IP info](#updating-ip-info)
  - [Delete IP data](#delete-ip-data)
  - [API alive?](#api-alive)
  - [Visualize](#visualize)
- [Import from local mongodb to mongodb atlas](#import-from-local-mongodb-to-mongodb-atlas)
  - [Install mongodb tools](#install-mongodb-tools)
  - [Dump local database](#dump-local-database)
  - [Import local database](#import-local-database)
- [TO DO](#to-do)
- [Contribution](#contribution)
- [LICENSE](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# Badges
![Test Status](https://github.com/containerscrew/ipfinder/actions/workflows/test.yml/badge.svg)
[![License](https://img.shields.io/github/license/containerscrew/ipfinder)](/LICENSE)

# Introduction
So imagine that you want to start indexing public IPv4 geolocation data. There are paid services, others free but requiring registration, that offer this type of content.
By consulting one of these services you will obtain the data you need.

I do not know exactly all the public services that offer this data, in my case I have used the public database of **[ip-api.com](http://ip-api.com)**

> Without this service this tool does not work, it would be necessary to refactor and use another API.

Other possible solutions (not implemented):
* https://www.maxmind.com/en/home
* https://ipstack.com/


# How works

When you start the API, it accepts all 4 CRUD methods of a simple API. Insert, get, delete and update. All data is stored in mongodb (the only database implemented to date). We will see more examples of the commands in this README.


# Local development

## Requirements

* Rust
* Cargo
* MongoDB

Take a look to the [official documentation](https://www.rust-lang.org/tools/install)

## Clone the repository

```shell
git clone https://github.com/containerscrew/ipfinder.git
cd ipfinder
```

## Set your .env file with the necessary credentials

```dotenv
DB_ENDPOINT="mongodb://admin:admin@localhost:27017/?maxPoolSize=20&w=majority"
DB_NAME="ipfinder"
COLLECTION_NAME="ips"
RUST_LOG="actix_web=debug"
```

If you are using mongodb atlas, just set the endpoint that you get from the mongodb atlas console:

```dotenv
DB_ENDPOINT="mongodb+srv://XXXX:XXXXX@XXXX.XXXXX.mongodb.net/?retryWrites=true&w=majority"
DB_NAME="ipfinder"
COLLECTION_NAME="ips"
RUST_LOG="actix_web=debug"
```

If not, run the container locally

## Start your local mongodb using a container

```bash
docker-compose -f compose.yml up -d
```

# Running in local

## Cargo run with autoload

```shell
cargo binstall cargo-watch
cargo watch -x run
```

## Cargo in local dev

```shell
cargo run --
```

## Build
```shell
cargo build --release # --release flag for production environment, without --release flag for testing
```

## Running the API

If the previous build was success, then:

```shell
./target/release/ipfinder
```

> Remember to have the database created locally or in mongo atlas, otherwise the API will panic

## Going to production

* containerfile
* k8s....

TO DO...

[See to do section](#to-do)

# Examples

## Inserting IP

```bash
curl -XPOST http://127.0.0.1:8081/api/v1/ipfinder/insert -d '{"ip":"8.8.8.8"}' -H "Content-Type: application/json"
```

## Getting IP info

```bash
curl -XGET http://127.0.0.1:8081/api/v1/ipfinder/get/8.8.8.8
```

## Updating IP info

> THIS METHOD IS ACTUALLY FAILING, NEED TO BE FIXED

```bash
curl -XPUT http://127.0.0.1:8081/api/v1/ipfinder/update/8.8.8.8
```

> **The update method is not a CRUD update as such. By relying on data from an external API, launching the update command basically re-queries the data from the external *ip-api* database and refreshes it again.**

## Delete IP data

```bash
curl -XDELETE http://127.0.0.1:8081/api/v1/ipfinder/delete/8.8.8.8
```

## API alive?

```shell
curl -XGET http://127.0.0.1:8081/api/v1/ipfinder/health
```

## Visualize

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

* improve error handling and logging with env_logger (custom error handling)
* implement update function in CRUD
* middleware with API authentication?
* testing like: https://github.com/actix/examples/blob/master/databases/mongodb/src/test.rs
* testcontainers for pipeline testing: https://docs.rs/testcontainers/latest/testcontainers/
* containerize this application to allow launching inside a simple container or pod in k8s (create also a small helm chart)
* bind address and port should be defined by the user

# Axum

* [Github](https://github.com/tokio-rs/axum)
* [API example](https://github.com/wpcodevo/simple-api-rust-axum)

# Contribution

Pull requests are welcome! Any code refactoring, improvement, implementation. I just want to learn Rust! I'm a rookie

# LICENSE

[LICENSE](./LICENSE)
