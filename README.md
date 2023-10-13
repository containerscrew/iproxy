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

# TO DO

* resync existing database ip information with external database
* when querying for the geolocation of an ip, check first if exist in database

# Running in local

```bash
cargo run --
```

# Async trait

Officially not implemented, but exists a crate: https://github.com/dtolnay/async-trait

[Documentation](https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html)
