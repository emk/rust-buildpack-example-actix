# Example application: Rust, Actix & Heroku

[![Build Status](https://travis-ci.org/emk/rust-buildpack-example-actix.svg?branch=master)](https://travis-ci.org/emk/rust-buildpack-example-actix)

[Rust][] is a language suitable for high-peformance, reliable web services. [Actix][] is a popular Rust web framework. [Heroku][] deploys, manages and scales web applications.

[Rust]: https://www.rust-lang.org/
[Actix]: https://actix.rs/
[Heroku]: https://www.heroku.com/

To deploy this application to Heroku, use this button:

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

Or, if you'd prefer to use the command line, try running:

``` sh
git clone https://github.com/emk/rust-buildpack-example-actix.git
cd rust-buildpack-example-actix
heroku create --buildpack emk/rust
git push heroku master
```

This should make a local copy of this application and deploy it to Heroku.

For further instructions, see the [page for this buildpack][buildpack].

[buildpack]: https://github.com/emk/heroku-buildpack-rust

## Does this work with the latest version of Rust?

This application works with version 1.31 of Rust, which theoretically means
that it should run on any future 1.x release of Rust. If it doesn't work,
please file a bug.
