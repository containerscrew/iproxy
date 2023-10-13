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

