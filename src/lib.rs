#![warn(missing_docs)]

//! A library for consistent and reliable error handling
//!
//! This crate defines an opinionated strategy for error handling in Rust,
//! built on the following principles:
//!
//! * No error should ever be discarded. This library primarily
//!   makes it easy to "chain" errors with the `chain_err` method.
//! * Introducing new errors is trivial. Simple errors can be introduced
//!   at the error site with just a string.
//! * Handling errors is possible with pattern matching.
//! * Conversions between error types are done in an automatic and
//!   consistent way - `From` conversion behavior is never specified
//!   explicitly.
//! * Errors implement Send.
//! * Errors can carry backtraces.
//!
//! Similar to other libraries like [error-type] and [quick-error], this
//! library defines a macro, `error_chain!` that declares the types
//! and implementation boilerplate necessary for fulfilling a
//! particular error-handling strategy. Most importantly it defines
//! a custom error type (called `Error` by convention) and the `From`
//! conversions that let the `try!` macro and `?` operator work.
//!
//! This library differs in a few ways from previous error libs:
//!
//! * Instead of defining the custom `Error` type as an enum, it is a
//!   struct containing an `ErrorKind` (which defines the
//!   `description` and `display` methods for the error), an opaque,
//!   optional, boxed `std::error::Error + Send + 'static` object
//!   (which defines the `cause`, and establishes the links in the
//!   error chain), and a `Backtrace`.
//! * This crate additionally defines the  trait ResultExt
//!   that defines a `chain_err` method. This method
//!   on all `std::error::Error + Send + 'static` types extends
//!   the error chain by boxing the current error into an opaque
//!   object and putting it inside a new concrete error.
//! * It provides automatic `From` conversions between other error types
//!   defined by the `error_chain!` that preserve type information,
//!   and facilitate seamless error composition and matching of composed
//!   errors.
//! * It provides automatic `From` conversions between any other error
//!   type that hides the type of the other error in the `cause` box.
//! * If `RUST_BACKTRACE` is enabled, it collects a single backtrace at
//!   the earliest opportunity and propagates it down the stack through
//!   `From` and `ResultExt` conversions.
//!
//! To accomplish its goals it makes some tradeoffs:
//!
//! * The split between the `Error` and `ErrorKind` types can make it
//!   slightly more cumbersome to instantiate new (unchained) errors
//!   errors, requiring an `Into` or `From` conversion; as well as
//!   slightly more cumbersome to match on errors with another layer
//!   of types to match.
//! * Because the error type contains `std::error::Error + Send + 'static` objects,
//!   it can't implement `PartialEq` for easy comparisons.
//!
//! ## Quick start
//!
//! Add this to Cargo.toml, under `[dependencies]`:
//!
//! ```toml
//! error-chain = "0.5"
//! ```
//!
//! Write this at the top of your crate:
//!
//! ```ignore
//! #![recursion_limit = "1024"]
//! ```
//!
//! Again near the top of your crate, import the `error_chain` crate and its macros:
//!
//! ```ignore
//! #[macro_use]
//! extern crate error_chain;
//! ```
//!
//! Add an `errors` module to your crate:
//!
//! ```ignore
//! mod errors;
//! ```
//!
//! Add a file for that module called `errors.rs` and put this inside:
//!
//! ```ignore
//! error_chain! { }
//! ```
//!
//! That's the setup. Now when writing modules for your crate,
//! import everything from the `errors` module:
//!
//! ```ignore
//! use errors::*;
//! ```
//!
//! Create functions that return `Result`, which is defined by
//! the `error_chain!` macro, and start chaining errors!
//!
//! ```ignore
//! fn do_error_prone_work() -> Result<()> {
//!     let file = try!(File::open("foo").chain_err(|| "couldn't open file"));
//!     try!(file.write_all("important".as_bytes()).chain_err(|| "couldn't write file"));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Declaring error types
//!
//! Generally, you define one family of error types per crate, though
//! it's also perfectly fine to define error types on a finer-grained
//! basis, such as per module.
//!
//! Assuming you are using crate-level error types, typically you will
//! define an `errors` module and inside it call `error_chain!`:
//!
//! ```ignore
//! error_chain! {
//!     // The type defined for this error. These are the conventional
//!     // and recommended names, but they can be arbitrarily chosen.
//!     // It is also possible to leave this block out entirely, or
//!     // leave it empty, and these names will be used automatically.
//!     types {
//!         Error, ErrorKind, Result;
//!     }
//!
//!     // Automatic conversions between this error chain and other
//!     // error chains. In this case, it will e.g. generate an
//!     // `ErrorKind` variant called `Dist` which in turn contains
//!     // the `rustup_dist::ErrorKind`, with conversions from
//!     // `rustup_dist::Error`.
//!     //
//!     // Optionally, some attributes can be added to a variant.
//!     //
//!     // This section can be empty.
//!     links {
//!         ::rustup_dist::Error, Dist;
//!         ::rustup_utils::Error, Utils, #[cfg(unix)];
//!     }
//!
//!     // Automatic conversions between this error chain and other
//!     // error types not defined by the `error_chain!`. These will be
//!     // wrapped in a new error with, in this case, the
//!     // `ErrorKind::Temp` variant. The description and cause will
//!     // forward to the description and cause of the original error.
//!     //
//!     // Optionally, some attributes can be added to a variant.
//!     //
//!     // This section can be empty.
//!     foreign_links {
//!         ::temp::Error, Temp;
//!         io::Error, Io, #[cfg(unix)];
//!     }
//!
//!     // Define additional `ErrorKind` variants. The syntax here is
//!     // the same as `quick_error!`, but the `from()` and `cause()`
//!     // syntax is not supported.
//!     errors {
//!         InvalidToolchainName(t: String) {
//!             description("invalid toolchain name")
//!             display("invalid toolchain name: '{}'", t)
//!         }
//!     }
//! }
//! ```
//!
//! Each section, `types`, `links`, `foreign_links`, and `errors` may
//! be omitted if it is empty.
//!
//! This populates the module with a number of definitions,
//! the most important of which are the `Error` type
//! and the `ErrorKind` type. They look something like the
//! following:
//!
//! ```ignore
//! use std::error::Error as StdError;
//! use std::sync::Arc;
//!
//! #[derive(Debug)]
//! pub struct Error(pub ErrorKind,
//!                  pub Option<Box<StdError + Send>>,
//!                  pub Option<Arc<error_chain::Backtrace>>);
//!
//! impl Error {
//!     pub fn kind(&self) -> &ErrorKind { ... }
//!     pub fn into_kind(self) -> ErrorKind { ... }
//!     pub fn iter(&self) -> error_chain::ErrorChainIter { ... }
//!     pub fn backtrace(&self) -> Option<&error_chain::Backtrace> { ... }
//! }
//!
//! impl StdError for Error { ... }
//! impl Display for Error { ... }
//!
//! #[derive(Debug)]
//! pub enum ErrorKind {
//!     Msg(String),
//!     Dist(rustup_dist::ErrorKind),
//!     Utils(rustup_utils::ErrorKind),
//!     Temp,
//!     InvalidToolchainName(String),
//! }
//! ```
//!
//! This is the basic error structure. You can see that `ErrorKind`
//! has been populated in a variety of ways. All `ErrorKind`s get a
//! `Msg` variant for basic errors. When strings are converted to
//! `ErrorKind`s they become `ErrorKind::Msg`. The "links" defined in
//! the macro are expanded to `Dist` and `Utils` variants, and the
//! "foreign links" to the `Temp` variant.
//!
//! Both types come with a variety of `From` conversions as well:
//! `Error` can be created from `ErrorKind`, `&str` and `String`,
//! and the "link" and "foreign_link" error types. `ErrorKind`
//! can be created from the corresponding `ErrorKind`s of the link
//! types, as well as from `&str` and `String`.
//!
//! `into()` and `From::from` are used heavily to massage types into
//! the right shape. Which one to use in any specific case depends on
//! the influence of type inference, but there are some patterns that
//! arise frequently.
//!
//! ## Returning new errors
//!
//! Introducing new error chains, with a string message:
//!
//! ```ignore
//! fn foo() -> Result<()> {
//!     Err("foo error!".into())
//! }
//! ```
//!
//! Introducing new error chains, with an `ErrorKind`:
//!
//! ```ignore
//! fn foo() -> Result<()> {
//!     Err(ErrorKind::FooError.into())
//! }
//! ```
//!
//! Note that the return type is the typedef `Result`, which is
//! defined by the macro as `pub type Result<T> =
//! ::std::result::Result<T, Error>`. Note that in both cases
//! `.into()` is called to convert a type into the `Error` type; both
//! strings and `ErrorKind` have `From` conversions to turn them into
//! `Error`.
//!
//! When the error is emitted inside a `try!` macro or behind the
//! `?` operator, the explicit conversion isn't needed; `try!` will
//! automatically convert `Err(ErrorKind)` to `Err(Error)`. So the
//! below is equivalent to the previous:
//!
//! ```ignore
//! fn foo() -> Result<()> {
//!     Ok(try!(Err(ErrorKind::FooError)))
//! }
//!
//! fn bar() -> Result<()> {
//!     Ok(try!(Err("bogus!")))
//! }
//! ```
//!
//! ## Chaining errors
//!
//! To extend the error chain:
//!
//! ```ignore
//! use error_chain::ResultExt;
//! try!(do_something().chain_err(|| "something went wrong"));
//! ```
//!
//! `chain_err` can be called on any `Result` type where the contained
//! error type implements `std::error::Error + Send + 'static`.  If
//! the `Result` is an `Err` then `chain_err` evaluates the closure,
//! which returns *some type that can be converted to `ErrorKind`*,
//! boxes the original error to store as the cause, then returns a new
//! error containing the original error.
//!
//! ## Foreign links
//!
//! Errors that do not conform to the same conventions as this library
//! can still be included in the error chain. They are considered "foreign
//! errors", and are declared using the `foreign_links` block of the
//! `error_chain!` macro. `Error`s are automatically created from
//! foreign errors by the `try!` macro.
//!
//! Foreign links and regular links have one crucial difference:
//! `From` conversions for regular links *do not introduce a new error
//! into the error chain*, while conversions for foreign links *always
//! introduce a new error into the error chain*. So for the example
//! above all errors deriving from the `temp::Error` type will be
//! presented to the user as a new `ErrorKind::Temp` variant, and the
//! cause will be the original `temp::Error` error. In contrast, when
//! `rustup_utils::Error` is converted to `Error` the two `ErrorKinds`
//! are converted between each other to create a new `Error` but the
//! old error is discarded; there is no "cause" created from the
//! original error.
//!
//! ## Backtraces
//!
//! If the `RUST_BACKTRACE` environment variable is set to anything
//! but ``0``, the earliest non-foreign error to be generated creates
//! a single backtrace, which is passed through all `From` conversions
//! and `chain_err` invocations of compatible types. To read the
//! backtrace just call the `backtrace()` method.
//!
//! Backtrace generation can be disabled by turning off the `backtrace` feature.
//!
//! ## Iteration
//!
//! The `iter` method returns an iterator over the chain of error boxes.
//!
//! [error-type]: https://github.com/DanielKeep/rust-error-type
//! [quick-error]: https://github.com/tailhook/quick-error

#[cfg(feature = "backtrace")]
extern crate backtrace;

use std::error;
use std::iter::Iterator;
#[cfg(feature = "backtrace")]
use std::sync::Arc;

#[cfg(feature = "backtrace")]
pub use backtrace::Backtrace;

mod quick_error;
mod error_chain;

/// Iterator over the error chain.
pub struct ErrorChainIter<'a>(pub Option<&'a error::Error>);

impl<'a> Iterator for ErrorChainIter<'a> {
    type Item = &'a error::Error;

    fn next<'b>(&'b mut self) -> Option<&'a error::Error> {
        match self.0.take() {
            Some(e) => {
                self.0 = e.cause();
                Some(e)
            }
            None => None,
        }
    }
}

/// Returns a backtrace of the current call stack if `RUST_BACKTRACE`
/// is set to anything but ``0``, and `None` otherwise.  This is used
/// in the generated error implementations.
#[cfg(feature = "backtrace")]
#[doc(hidden)]
pub fn make_backtrace() -> Option<Arc<Backtrace>> {
    match std::env::var_os("RUST_BACKTRACE") {
        Some(ref val) if val != "0" => Some(Arc::new(Backtrace::new())),
        _ => None
    }
}

/// This trait is implemented on all the errors generated by the `error_chain`
/// macro.
pub trait ChainedError: error::Error + Send + 'static {
    /// Associated kind type.
    type ErrorKind;
    /// Creates an error from it's parts.
    fn new(kind: Self::ErrorKind, state: State) -> Self;
    /// Returns the first known backtrace, either from it's State or from one
    /// of the errors from `foreign_links`.
    #[cfg(feature = "backtrace")]
    fn extract_backtrace(e: &(error::Error + Send + 'static))
        -> Option<Option<Arc<Backtrace>>>;
}

/// Additionnal methods for `Result`, for easy interaction with this crate.
pub trait ResultExt<T, E: ChainedError> {
    /// If the `Result` is an `Err` then `chain_err` evaluates the closure,
    /// which returns *some type that can be converted to `ErrorKind`*, boxes
    /// the original error to store as the cause, then returns a new error
    /// containing the original error.
    fn chain_err<F, EK>(self, callback: F) -> Result<T, E>
        where F: FnOnce() -> EK,
        EK: Into<E::ErrorKind>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> where E: ChainedError {
    fn chain_err<F, EK>(self, callback: F) -> Result<T, E>
        where F: FnOnce() -> EK,
        EK: Into<E::ErrorKind> {
        self.map_err(move |e| {
            #[cfg(feature = "backtrace")]
            let error = {
                let backtrace = E::extract_backtrace(&e)
                                  .unwrap_or_else(make_backtrace);
                E::new(callback().into(), State {
                    next_error: Some(Box::new(e)),
                    backtrace: backtrace,
                })
            };
            #[cfg(not(feature = "backtrace"))]
            let error = E::new(callback().into(), State {
                next_error: Some(Box::new(e)),
            });
            error
        })
    }
}


/// Common state between errors.
#[derive(Debug)]
pub struct State {
    /// Next error in the error chain.
    pub next_error: Option<Box<error::Error + Send>>,
    /// Backtrace for the current error.
    #[cfg(feature = "backtrace")]
    pub backtrace: Option<Arc<Backtrace>>,
}

impl Default for State {
    fn default() -> State {
        #[cfg(feature = "backtrace")]
        let state = State {
            next_error: None,
            backtrace: make_backtrace(),
        };
        #[cfg(not(feature = "backtrace"))]
        let state = State {
            next_error: None,
        };
        state
    }
}
