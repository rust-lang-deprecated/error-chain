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

See https://github.com/brson/error-chain/blob/master/examples/quickstart.rs.

## License

MIT/Apache-2.0
