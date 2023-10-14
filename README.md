<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [DEPENDENCIES](#dependencies)
- [TO DO](#to-do)
- [Running in local](#running-in-local)
- [Async trait](#async-trait)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# DEPENDENCIES

* Using external database: `http://ip-api.com/json/{ip}`
Other not used, need registration for token:
* https://www.maxmind.com/en/home
* https://ipstack.com/

# TO DO

* improve error handling and logging with env_logger (custom error handling)
* implement update function in CRUD
* middleware with API authentication?
* testing like here: https://github.com/actix/examples/blob/master/databases/mongodb/src/test.rs

# Running in local

```bash
docker-compose -f compose.yml up -d # start mongodb local server
cargo run --
```

# Async trait

Officially not implemented, but exists a crate: https://github.com/dtolnay/async-trait

[Documentation](https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html)
