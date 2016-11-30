// This macro uses recursion a lot.
#![recursion_limit = "1024"]

// Import of the macro. Don't forget to add `error-chain = "*"` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// Generations of the `Error` type, the `ErrorKind` enum and the `Result`
// wrapper. See the documentation for more details.
//
// You can also generate the types in a dedicated modules, like `errors.rs`.
error_chain! {
    foreign_links {
        // An IO error can occur.
        Io(::std::io::Error);
    }
}

// With the `Result` wrapper, we don't have to specify `Error` each time.
fn do_some_work() -> Result<()> {
    use std::fs::File;
    // Dummy file, this operation should fail on your system. The result of
    // `File::open` is automatically converted to our `Error` type.
    File::open("tretrete")?;
    Ok(())
}

fn main() {
    match do_some_work() {
        Ok(()) => println!("No errors"),
        Err(e) => {
            println!("An error occured: {}", e);
            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = e.backtrace() {
                println!("Backtrace: {:?}", backtrace);
            }
        }
    }
}
