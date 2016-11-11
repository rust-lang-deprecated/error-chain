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
//! * The macro additionally defines a trait, by convention called
//!   `ChainErr`, that defines a `chain_err` method. This method
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
//!   `From` and `ChainErr` conversions.
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
//!         Error, ErrorKind, ChainErr, Result;
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
//!         ::rustup_dist::Error, rustup_dist::ErrorKind, Dist;
//!         ::rustup_utils::Error, rustup_utils::ErrorKind, Utils, #[cfg(unix)];
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
//! use errors::ChainErr;
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

#[cfg(feature = "backtrace")]
pub use backtrace::Backtrace;
#[cfg(not(feature = "backtrace"))]
#[derive(Debug)]
pub enum Backtrace {}

mod quick_error;

#[macro_export]
macro_rules! error_chain {
    (
        types {
            $error_name:ident, $error_kind_name:ident,
            $chain_error_name:ident, $result_name:ident;
        }

        links {
            $( $link_error_path:path, $link_kind_path:path, $link_variant:ident $(, #[$meta_links:meta])*; ) *
        }

        foreign_links {
            $( $foreign_link_error_path:path, $foreign_link_variant:ident $(, #[$meta_foreign_links:meta])*; )*
        }

        errors {
            $( $error_chunks:tt ) *
        }

    ) => {


        // The Error type
        // --------------

        // This has a simple structure to support pattern matching
        // during error handling. The second field is internal state
        // that is mostly irrelevant for error handling purposes.
        #[derive(Debug)]
        pub struct $error_name(pub $error_kind_name,
                               pub (Option<Box<::std::error::Error + Send>>,
                                    Option<::std::sync::Arc<$crate::Backtrace>>));

        #[allow(unused)]
        impl $error_name {
            pub fn kind(&self) -> &$error_kind_name {
                &self.0
            }

            pub fn iter(&self) -> $crate::ErrorChainIter {
                $crate::ErrorChainIter(Some(self))
            }

            pub fn backtrace(&self) -> Option<&$crate::Backtrace> {
                (self.1).1.as_ref().map(|v| &**v)
            }
        }

        impl ::std::error::Error for $error_name {
            fn description(&self) -> &str { self.0.description() }
            fn cause(&self) -> Option<&::std::error::Error> {
                match (self.1).0 {
                    Some(ref c) => Some(&**c),
                    None => {
                        match self.0 {
                            $(
                                $(#[$meta_foreign_links])*
                                $error_kind_name::$foreign_link_variant(ref foreign_err) => {
                                    foreign_err.cause()
                                }
                            ) *
                            _ => None
                        }
                    }
                }
            }
        }

        impl ::std::fmt::Display for $error_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        $(
            $(#[$meta_links])*
            impl From<$link_error_path> for $error_name {
                fn from(e: $link_error_path) -> Self {
                    $error_name($error_kind_name::$link_variant(e.0), e.1)
                }
            }
        ) *

        $(
            $(#[$meta_foreign_links])*
            impl From<$foreign_link_error_path> for $error_name {
                fn from(e: $foreign_link_error_path) -> Self {
                    $error_name(
                        $error_kind_name::$foreign_link_variant(e),
                        (None, $crate::make_backtrace()))
                }
            }
        ) *

        impl From<$error_kind_name> for $error_name {
            fn from(e: $error_kind_name) -> Self {
                $error_name(e,
                            (None, $crate::make_backtrace()))
            }
        }

        impl<S: Into<String>> From<S> for $error_name {
            fn from(s: S) -> Self {
                $error_name(s.into().into(),
                            (None, $crate::make_backtrace()))
            }
        }

        impl ::std::ops::Deref for $error_name {
            type Target = $error_kind_name;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }


        // The ErrorKind type
        // --------------

        quick_error! {
            #[derive(Debug)]
            pub enum $error_kind_name {

                Msg(s: String) {
                    description(&s)
                    display("{}", s)
                }

                $(
                    $(#[$meta_links])*
                    $link_variant(e: $link_kind_path) {
                        description(e.description())
                        display("{}", e)
                    }
                ) *

                $(
                    $(#[$meta_foreign_links])*
                    $foreign_link_variant(err: $foreign_link_error_path) {
                        description(::std::error::Error::description(err))
                        display("{}", err)
                    }
                ) *

                $($error_chunks)*
            }
        }

        $(
            $(#[$meta_links])*
            impl From<$link_kind_path> for $error_kind_name {
                fn from(e: $link_kind_path) -> Self {
                    $error_kind_name::$link_variant(e)
                }
            }
        ) *

        impl<S: Into<String>> From<S> for $error_kind_name {
            fn from(s: S) -> Self {
                $error_kind_name::Msg(s.into())
            }
        }

        impl From<$error_name> for $error_kind_name {
            fn from(e: $error_name) -> Self {
                e.0
            }
        }


        // The ChainErr trait
        // ------------------

        pub trait $chain_error_name<T> {
            fn chain_err<F, EK>(self, callback: F) -> ::std::result::Result<T, $error_name>
                where F: FnOnce() -> EK,
                      EK: Into<$error_kind_name>;
        }

        impl<T, E> $chain_error_name<T> for ::std::result::Result<T, E>
            where E: ::std::error::Error + Send + 'static
        {
            fn chain_err<F, EK>(self, callback: F) -> ::std::result::Result<T, $error_name>
                where F: FnOnce() -> EK,
                      EK: Into<$error_kind_name>
            {
                self.map_err(move |e| {
                    let e = Box::new(e) as Box<::std::error::Error + Send + 'static>;
                    let (e, backtrace) = backtrace_from_box(e);
                    let backtrace = backtrace.unwrap_or_else($crate::make_backtrace);

                    $error_name(callback().into(), (Some(e), backtrace))
                })
            }
        }

        // Use downcasts to extract the backtrace from types we know,
        // to avoid generating a new one. It would be better to not
        // define this in the macro, but types need some additional
        // machinery to make it work.
        fn backtrace_from_box(mut e: Box<::std::error::Error + Send + 'static>)
                              -> (Box<::std::error::Error + Send + 'static>,
                                  Option<Option<::std::sync::Arc<$crate::Backtrace>>>) {
            let mut backtrace = None;

            e = match e.downcast::<$error_name>() {
                Err(e) => e,
                Ok(e) => {
                    backtrace = Some((e.1).1.clone());
                    e as Box<::std::error::Error + Send + 'static>
                }
            };

            $(

                e = match e.downcast::<$link_error_path>() {
                    Err(e) => e,
                    Ok(e) => {
                        backtrace = Some((e.1).1.clone());
                        e as Box<::std::error::Error + Send + 'static>
                    }
                };

            ) *

            (e, backtrace)
        }

        // The Result type
        // ---------------

        pub type $result_name<T> = ::std::result::Result<T, $error_name>;
    };

    // Handle missing sections, or missing type names in types { }
    //
    // Macros cannot specify "zero or one repetitions" at the moment, so we allow
    // repeating sections. Only for the `types` section this makes no sense, which
    // is the reason for the three separate cases.
    //
    // Case 1: types fully specified
    (
        types {
            $error_name:ident, $error_kind_name:ident,
            $chain_error_name:ident, $result_name:ident;
        }

        $( links {
            $( $link_chunks:tt ) *
        } ) *

        $( foreign_links {
            $( $foreign_link_chunks:tt ) *
        } ) *

        $( errors {
            $( $error_chunks:tt ) *
        } ) *
    ) => (
        error_chain! {
            types {
                $error_name, $error_kind_name, $chain_error_name, $result_name;
            }

            links {
                $( $( $link_chunks ) * ) *
            }

            foreign_links {
                $( $( $foreign_link_chunks ) * ) *
            }

            errors {
                $( $( $error_chunks ) * ) *
            }
        }
    );
    // Case 2: types section present, but empty
    (
        types { }

        $( links {
            $( $link_chunks:tt ) *
        } ) *

        $( foreign_links {
            $( $foreign_link_chunks:tt ) *
        } ) *

        $( errors {
            $( $error_chunks:tt ) *
        } ) *
    ) => (
        error_chain! {
            types {
                Error, ErrorKind, ChainErr, Result;
            }

            links {
                $( $( $link_chunks ) * ) *
            }

            foreign_links {
                $( $( $foreign_link_chunks ) * ) *
            }

            errors {
                $( $( $error_chunks ) * ) *
            }
        }
    );
    // Case 3: types section not present
    (
        $( links {
            $( $link_chunks:tt ) *
        } ) *

        $( foreign_links {
            $( $foreign_link_chunks:tt ) *
        } ) *

        $( errors {
            $( $error_chunks:tt ) *
        } ) *
    ) => (
        error_chain! {
            types { }

            links {
                $( $( $link_chunks ) * ) *
            }

            foreign_links {
                $( $( $foreign_link_chunks ) * ) *
            }

            errors {
                $( $( $error_chunks ) * ) *
            }
        }
    );
}


use std::error::Error as StdError;
use std::iter::Iterator;
use std::sync::Arc;

pub struct ErrorChainIter<'a>(pub Option<&'a StdError>);

impl<'a> Iterator for ErrorChainIter<'a> {
    type Item = &'a StdError;

    fn next<'b>(&'b mut self) -> Option<&'a StdError> {
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
pub fn make_backtrace() -> Option<Arc<Backtrace>> {
    match std::env::var_os("RUST_BACKTRACE") {
        Some(ref val) if val != "0" => Some(Arc::new(Backtrace::new())),
        _ => None
    }
}

#[cfg(not(feature = "backtrace"))]
pub fn make_backtrace() -> Option<Arc<Backtrace>> {
    None
}
