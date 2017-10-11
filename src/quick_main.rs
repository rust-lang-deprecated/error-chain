/// Convenient wrapper to be able to use `?` and such in the main. You can
/// use it with a separated function:
///
/// ```
/// # #[macro_use] extern crate error_chain;
/// # error_chain! {}
/// # fn main() {
/// quick_main!(run);
/// # }
///
/// fn run() -> Result<()> {
///     Err("error".into())
/// }
/// ```
///
/// or with a closure:
///
/// ```
/// # #[macro_use] extern crate error_chain;
/// # error_chain! {}
/// # fn main() {
/// quick_main!(|| -> Result<()> {
///     Err("error".into())
/// });
/// # }
/// ```
///
/// You can also set the exit value of the process by returning a type that implements [`ExitCode`](trait.ExitCode.html):
///
/// ```
/// # #[macro_use] extern crate error_chain;
/// # error_chain! {}
/// # fn main() {
/// quick_main!(run);
/// # }
///
/// fn run() -> Result<i32> {
///     Err("error".into())
/// }
/// ```
#[macro_export]
macro_rules! quick_main {
    ($main:expr) => {
        fn main() {
            ::std::process::exit(match $main() {
                Ok(ret) => $crate::ExitCode::code(ret),
                Err(ref e) => {
                    { $crate::print_quickmain_error(e); }

                    1
                }
            });
        }
    };
}

/// Represents a value that can be used as the exit status of the process.
/// See [`quick_main!`](macro.quick_main.html).
pub trait ExitCode {
    /// Returns the value to use as the exit status.
    fn code(self) -> i32;
}

impl ExitCode for i32 {
    fn code(self) -> i32 {
        self
    }
}

impl ExitCode for () {
    fn code(self) -> i32 {
        0
    }
}

/// When using `quick_main!`, prints the error if the program doesn't terminate successfully.
pub fn print_quickmain_error<K,E: ::ChainedError<ErrorKind=K>>(e: &E) {
    print_error_helper(e);
}

#[cfg(not(feature = "quickmain_log"))]
fn print_error_helper<K,E: ::ChainedError<ErrorKind=K>>(e: &E) {
    use ::std::io::Write;
    write!(&mut ::std::io::stderr(), "{}", ::ChainedError::display_chain(e))
        .expect("Error writing to stderr");
}

#[cfg(feature = "quickmain_log")]
fn print_error_helper<K,E: ::ChainedError<ErrorKind=K>>(e: &E) {
    { error!("{}", ::ChainedError::display_chain(e)) }
}
