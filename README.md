# cobalt

Cobalt aims to be a lightweight implementation of the [ActivityPub](https://www.w3.org/TR/activitypub/) standard, allowing it to federate with other implementations such as [Mastodon](https://github.com/tootsuite/mastodon/).

For a lightweight frontend see [Kobold](https://github.com/sorin-davidoi/kobold).

⚠️ This is nowhere near production ready ⚠️

⚠️ **The passwords are currently stored in plain text** ⚠️

## Features
- [x] Authentication - ⚠️ currently vulnerable to CSFR and possibly others ⚠️
- [x] User endpoint
- [x] Following endpoint
- [x] Followers endpoint
- [ ] Inbox endpoint
- [ ] Outbox endpoint

The roadmap will be expanded as the features will be implemented.

## Getting started

This relies on Rust Nightly, since that is what [Rocket](http://rocket.rs/) - the web framework we use - requires.

You need the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) to set up the database. It can be installed with

```bash
cargo install diesel_cli --no-default-features --features postgres
```

To set up the database run
```bash
DATABASE_URL=postgres://cobalt@localhost/cobalt_development diesel run
```

You should now be able to start the server
```bash
DATABASE_URL=postgres://cobalt@localhost/cobalt_development ROCKET_SECRET_KEY=4Ei10ygh7KiPBkavOD0PV2lyiF6c20rJbECFkNfraw8= cargo run
```
