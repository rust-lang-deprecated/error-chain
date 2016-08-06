#![allow(dead_code)]

#[macro_use]
extern crate error_chain;

#[test]
fn smoke_test_1() {
    error_chain! {
        types {
            Error, ErrorKind, ChainErr, Result;
        }

        links { }

        foreign_links { }

        errors { }
    }
}

#[test]
fn smoke_test_2() {
    error_chain! {
        types { }

        links { }

        foreign_links { }

        errors { }
    }
}

#[test]
fn smoke_test_3() {
    error_chain! {
        links { }

        foreign_links { }

        errors { }
    }
}

#[test]
fn smoke_test_4() {
    error_chain! {
        links { }

        foreign_links { }

        errors {
            HttpStatus(e: u32) {
                description("http request returned an unsuccessful status code")
                display("http request returned an unsuccessful status code: {}", e)
            }
        }
    }
}

#[test]
fn smoke_test_5() {
    error_chain! {
        types { }

        links { }

        foreign_links { }

        errors {
            HttpStatus(e: u32) {
                description("http request returned an unsuccessful status code")
                display("http request returned an unsuccessful status code: {}", e)
            }
        }
    }
}

#[test]
fn smoke_test_6() {
    error_chain! {
        errors {
            HttpStatus(e: u32) {
                description("http request returned an unsuccessful status code")
                display("http request returned an unsuccessful status code: {}", e)
            }
        }
    }
}

#[test]
fn smoke_test_7() {
    error_chain! {
        types { }

        foreign_links { }

        errors {
            HttpStatus(e: u32) {
                description("http request returned an unsuccessful status code")
                display("http request returned an unsuccessful status code: {}", e)
            }
        }
    }
}

#[test]
fn empty() {
    error_chain! { }
}

#[cfg(test)]
mod foreign_link_test {

    use std::error::Error as StdError;
    use std::fmt;

    // Note: foreign errors must be `pub` because they appear in the
    // signature of the public foreign_link_error_path
    #[derive(Debug)]
    pub struct ForeignError {
        cause: ForeignErrorCause
    }

    impl StdError for ForeignError {
        fn description(&self) -> &'static str {
            "Foreign error description"
        }

        fn cause(&self) -> Option<&StdError> { Some(&self.cause) }
    }

    impl fmt::Display for ForeignError {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Foreign error display")
        }
    }

    #[derive(Debug)]
    pub struct ForeignErrorCause {}

    impl StdError for ForeignErrorCause {
        fn description(&self) -> &'static str {
            "Foreign error cause description"
        }

        fn cause(&self) -> Option<&StdError> { None }
    }

    impl fmt::Display for ForeignErrorCause {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Foreign error cause display")
        }
    }

    error_chain! {
        types{
            Error, ErrorKind, ChainErr, Result;
        }
        links {}
        foreign_links {
            ForeignError, Foreign;
        }
        errors {}
    }

    #[test]
    fn display_underlying_error() {
        let chained_error = try_foreign_error().err().unwrap();
        assert_eq!(
            format!("{}", ForeignError{ cause: ForeignErrorCause{} }),
            format!("{}", chained_error)
        );
    }

    #[test]
    fn finds_cause() {
        let chained_error = try_foreign_error().err().unwrap();
        assert_eq!(
            format!("{}", ForeignErrorCause{}),
            format!("{}", chained_error.cause().unwrap())
        );
    }

    #[test]
    fn iterates() {
        let chained_error = try_foreign_error().err().unwrap();
        let mut error_iter = chained_error.iter();
        assert_eq!(
            format!("{}", ForeignError{ cause: ForeignErrorCause{} }),
            format!("{}", error_iter.next().unwrap())
        );
        assert_eq!(
            format!("{}", ForeignErrorCause{}),
            format!("{}", error_iter.next().unwrap())
        );
        assert_eq!(
            format!("{:?}", None as Option<&StdError>),
            format!("{:?}", error_iter.next())
        );
    }

    fn try_foreign_error() -> Result<()> {
        try!(Err(ForeignError{
            cause: ForeignErrorCause{}
        }));
        Ok(())
    }
}
