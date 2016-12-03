// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, Result, and ChainMain types
    error_chain! { }
}

use errors::*;

fn main() {
    // The backtrace is not always generated. Try to run this example
    // with `RUST_BACKTRACE=1`.
    real_main().chain_main()
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
//
// real_main() can also return Result<i32> to provide a non-zero
// exit code in the "success" case, such as for a grep-like program
// returning 1 if no match.
fn real_main() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("tretrete")
        .chain_err(|| "unable to open tretrete file")?;

    Ok(())
}

