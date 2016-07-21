[![Build Status](https://api.travis-ci.org/brson/error-chain.svg?branch=master)](https://travis-ci.org/brson/error-chain)
[![Latest Version](https://img.shields.io/crates/v/error-chain.svg)](https://crates.io/crates/error-chain)

# error-chain - Consistent error handling for Rust

The [error-chain crate](https://crates.io/crates/error-chain)
([docs](http://brson.github.io/error-chain/index.html)) is a new crate
for dealing with Rust error boilerplate. It provides a few unique
features:

* No error is ever discarded. This library primarily makes it easy to
  "chain" errors with the `chain_err` method.
* Introducing new errors is trivial. Simple errors can be introduced
  at the error site with just a string.
* Errors create and propagate backtraces.

I think the lack of the above are widespread problems with Rust error
handling, so I'm interested to hear what people think about this
solution.  It is inspired by
[quick-error](https://github.com/tailhook/quick-error) (and in fact
includes a hacked up version of it for internal use) as well as
Cargo's internal error handling techniques. This library is used by
[rustup](https://github.com/rust-lang-nursery/rustup.rs) for error
handling.

One note about usage that isn't included in the docs: the
`chain_error!` macro recurses deeply, so you'll probably need to use
the little-known `#![recursion_limit = "1024"]` macro on crates that
import it.

For detailed usage information read [the
docs](http://brson.github.io/error-chain/index.html),
which are reproduced in part below.

## Quick start

Add this to Cargo.toml, under `[dependencies]`:

```toml
error-chain = "0.2"
```

Write this at the top of your crate:

```rust
#![recursion_limit = "1024"];
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

Add a module called errors to your crate and put this inside:

```rust
error_chain! {
    links { }

    foreign_links { }

    errors { }
}
```

That's the setup. Now when writing modules for your crate,
import everything from the `errors` module:

```rust
use errors::*;
```

Create functions that return `Result`, which is defined my
the `error_chain!` macro, and start chaining errors!

```rust
fn do_error_prone_work() -> Result<()> {
    let file = try!(File::open("foo").chain_err(|| "couldn't open file"));
    try!(file.write_str("important").chain_err(|| "couldn't write file"));

    Ok(())
}
```

## Declaring error types

Generally, you define one family of error types per crate, though it's
also perfectly fine to define error types on a finer-grained basis,
such as per module.

Assuming you are using crate-level error types, typically you will
define an `errors` module and inside it call `error_chain!`:

```rust
// Define the error types and conversions for this crate
error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    // It is also possible to leave this block out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    // Automatic conversions between this error chain and other
    // error chains. In this case, it will e.g. generate an
    // `ErrorKind` variant called `Dist` which in turn contains
    // the `rustup_dist::ErrorKind`, with conversions from
    // `rustup_dist::Error`.
    //
    // This section can be empty.
    links {
        rustup_dist::Error, rustup_dist::ErrorKind, Dist;
        rustup_utils::Error, rustup_utils::ErrorKind, Utils;
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // boxed as the error cause and wrapped in a new error with,
    // in this case, the `ErrorKind::Temp` variant.
    //
    // This section can be empty.
    foreign_links {
        temp::Error, Temp,
        "temporary file error";
    }

    // Define additional `ErrorKind` variants. The syntax here is
    // the same as `quick_error!`, but the `from()` and `cause()`
    // syntax is not supported.
    errors {
        InvalidToolchainName(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }
    }
}
```

This populates the the module with a number of definitions, the most
important of which are the `Error` type and the `ErrorKind` type. They
look something like the following:

```rust
use std::error::Error as StdError;
use std::sync::Arc;

#[derive(Debug)]
pub struct Error(pub ErrorKind,
                 pub Option<Box<StdError + Send>>,
                 pub Arc<error_chain::Backtrace>);

impl Error {
    pub fn kind(&self) -> &ErrorKind { ... }
    pub fn into_kind(self) -> ErrorKind { ... }
    pub fn iter(&self) -> error_chain::ErrorChainIter { ... }
    pub fn backtrace(&self) -> &error_chain::Backtrace { ... }
}

impl StdError for Error { ... }
impl Display for Error { ... }

#[derive(Debug)]
pub enum ErrorKind {
    Msg(String),
    Dist(rustup_dist::ErrorKind),
    Utils(rustup_utils::ErrorKind),
    Temp,
    InvalidToolchainName(String),
}
```

This is the basic error structure. You can see that `ErrorKind` has
been populated in a variety of ways. All `ErrorKind`s get a `Msg`
variant for basic errors. When strings are converted to `ErrorKind`s
they become `ErrorKind::Msg`. The "links" defined in the macro are
expanded to `Dist` and `Utils` variants, and the "foreign links" to
the `Temp` variant.

Both types come with a variety of `From` conversions as well: `Error`
can be created from `ErrorKind`, from `&str` and `String`, and from
the "link" and "foreign_link" error types. `ErrorKind` can be created
from the corresponding `ErrorKind`s of the link types, as wall as from
`&str` and `String`.

`into()` and `From::from` are used heavily to massage types into the
right shape. Which one to use in any specific case depends on the
influence of type inference, but there are some patterns that arise
frequently.

## Chaining errors

This is the focus of the crate's design. To extend the error chain:

```
use errors::ChainErr;
try!(do_something().chain_err(|| "something went wrong"));
```

`chain_err` can be called on any `Result` type where the contained
error type implements `std::error::Error + Send + 'static`.  If the
`Result` is an `Err` then `chain_err` evaluates the closure, which
returns *some type that can be converted to `ErrorKind`*, boxes the
original error to store as the cause, then returns a new error
containing the original error.

The above example turns a string into an error, but you could also write e.g.

```
try!(do_something().chain_err(|| ErrorKind::Foo));
```

## Returning new errors

Introducing new error chains, with a string message:

```rust
fn foo() -> Result<()> {
    Err("foo error!".into())
}
```

Introducing new error chains, with an `ErrorKind`:

```rust
fn foo() -> Result<()> {
    Err(ErrorKind::FooError.into())
}
```

Note that the return type is is the typedef `Result`, which is defined
by the macro as `pub type Result<T> = ::std::result::Result<T,
Error>`. Note that in both cases `.into()` is called to convert a type
into the `Error` type: both strings and `ErrorKind` have `From`
conversions to turn them into `Error`.

When the error is emitted inside a `try!` macro or behind the `?`
operator, then the explicit conversion isn't needed, since the
behavior of `try!` will automatically convert `Err(ErrorKind)` to
`Err(Error)`. So the below is equivalent to the previous:

```rust
fn foo() -> Result<()> {
    Ok(try!(Err(ErrorKind::FooError)))
}

fn bar() -> Result<()> {
    Ok(try!(Err("bogus!")))
}
```

## Foreign links

Errors that do not conform to the same conventions as this library can
still be included in the error chain. They are considered "foreign
errors", and are declared using the `foreign_links` block of the
`error_chain!` macro. `Error`s are automatically created from foreign
errors by the `try!` macro.

Foreign links and regular links have one crucial difference: `From`
conversions for regular links *do not introduce a new error into the
error chain*, while conversions for foreign links *always introduce a
new error into the error chain*. So for the example above all errors
deriving from the `temp::Error` type will be presented to the user as
a new `ErrorKind::Temp` variant, and the cause will be the original
`temp::Error` error. In contrast, when `rustup_utils::Error` is
converted to `Error` the two `ErrorKinds` are converted between each
other to create a new `Error` but the old error is discarded; there is
no "cause" created from the original error.

## Backtraces

The earliest non-foreign error to be generated creates a single
backtrace, which is passed through all `From` conversions and
`chain_err` invocations of compatible types. To read the backtrace
just call the `backtrace()` method.

## Iteration

The `iter` method returns an iterator over the chain of error
boxes. [See how rustup uses this during error
reporting](https://github.com/rust-lang-nursery/rustup.rs/blob/master/src/rustup-cli/common.rs#L344).

## License

MIT/Apache-2.0
