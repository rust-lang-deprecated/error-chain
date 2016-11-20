# error-chain - Consistent error handling for Rust

[![Build Status](https://api.travis-ci.org/brson/error-chain.svg?branch=master)](https://travis-ci.org/brson/error-chain)
[![Latest Version](https://img.shields.io/crates/v/error-chain.svg)](https://crates.io/crates/error-chain)
[![License](https://img.shields.io/github/license/brson/error-chain.svg)](https://github.com/brson/error-chain)

`error-chain` is a crate for dealing with Rust error boilerplate. It
provides a few unique features:

* No error is ever discarded. This library primarily makes it easy to
  "chain" errors with the `chain_err` method.
* Introducing new errors is trivial. Simple errors can be introduced
  at the error site with just a string.
* Errors can create and propagate backtraces.

[Documentation (crates.io)](https://docs.rs/error-chain).

[Documentation (master)](https://brson.github.io/error-chain).

## Quick start

Add this to Cargo.toml, under `[dependencies]`:

```toml
error-chain = "0.6"
```

Write this at the top of your crate:

```rust
#![recursion_limit = "1024"]
```

Again near the top of your crate, import the `error_chain` crate and its macros:

```rust
#[macro_use]
extern crate error_chain;
```

Add an `errors` module to your crate:

```rust
mod errors;
```

Add a file for that module called `errors.rs` and put this inside:

```rust
error_chain! { }
```

That's the setup. Now when writing modules for your crate,
import everything from the `errors` module:

```rust
use errors::*;
```

Create functions that return `Result`, which is defined by
the `error_chain!` macro, and start chaining errors!

```rust
fn do_error_prone_work() -> Result<()> {
    let file = try!(File::open("foo").chain_err(|| "couldn't open file"));
    try!(file.write_all("important".as_bytes()).chain_err(|| "couldn't write file"));

    Ok(())
}
```

## License

MIT/Apache-2.0
