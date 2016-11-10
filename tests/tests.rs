#![allow(dead_code)]

#[macro_use]
extern crate error_chain;

#[test]
fn smoke_test_1() {
    error_chain! {
        types {
            Error, ErrorKind, Result;
        }

        links { }

        foreign_links { }

        errors { }
    };
}

#[test]
fn smoke_test_2() {
    error_chain! {
        types { }

        links { }

        foreign_links { }

        errors { }
    };
}

#[test]
fn smoke_test_3() {
    error_chain! {
        links { }

        foreign_links { }

        errors { }
    };
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
    };
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
    };
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
    };
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
    };
}

#[test]
fn smoke_test_8() {
    error_chain! {
        types { }

        links { }
        links { }

        foreign_links { }
        foreign_links { }

        errors {
            FileNotFound
        }
        errors {
            AccessDenied
        }
    };
}

#[test]
fn empty() {
    error_chain! { };
}

#[test]
#[cfg(feature = "backtrace")]
fn has_backtrace_depending_on_env() {
    use std::env;

    error_chain! {
        types {}
        links {}
        foreign_links {}
        errors {
            MyError
        }
    }

    // missing RUST_BACKTRACE and RUST_BACKTRACE=0
    env::remove_var("RUST_BACKTRACE");
    let err = Error::from(ErrorKind::MyError);
    assert!(err.backtrace().is_none());
    env::set_var("RUST_BACKTRACE", "0");
    let err = Error::from(ErrorKind::MyError);
    assert!(err.backtrace().is_none());

    // RUST_BACKTRACE set to anything but 0
    env::set_var("RUST_BACKTRACE", "yes");
    let err = Error::from(ErrorKind::MyError);
    assert!(err.backtrace().is_some());
}

#[test]
fn chain_err() {
    use error_chain::ResultExt;

    error_chain! {}

    let _: Result<()> = Err("".into()).chain_err(|| "");
}

#[test]
fn links() {
    mod test {
        error_chain! {}
    }

    error_chain! {
        links {
            test::Error, Test;
        }
    }
}

#[cfg(test)]
mod foreign_link_test {

    use std::fmt;

    // Note: foreign errors must be `pub` because they appear in the
    // signature of the public foreign_link_error_path
    #[derive(Debug)]
    pub struct ForeignError {
        cause: ForeignErrorCause
    }

    impl ::std::error::Error for ForeignError {
        fn description(&self) -> &'static str {
            "Foreign error description"
        }

        fn cause(&self) -> Option<&::std::error::Error> { Some(&self.cause) }
    }

    impl fmt::Display for ForeignError {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Foreign error display")
        }
    }

    #[derive(Debug)]
    pub struct ForeignErrorCause {}

    impl ::std::error::Error for ForeignErrorCause {
        fn description(&self) -> &'static str {
            "Foreign error cause description"
        }

        fn cause(&self) -> Option<&::std::error::Error> { None }
    }

    impl fmt::Display for ForeignErrorCause {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Foreign error cause display")
        }
    }

    error_chain! {
        types{
            Error, ErrorKind, Result;
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
            format!("{}", ::std::error::Error::cause(&chained_error).unwrap())
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
            format!("{:?}", None as Option<&::std::error::Error>),
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

#[cfg(test)]
mod attributes_test {
    #[allow(unused_imports)]
    use std::io;

    mod inner {
        error_chain! {

        }
    }

    error_chain! {
        types {
            Error, ErrorKind, Result;
        }

        links {
            inner::Error, Inner, #[cfg(not(test))];
        }

        foreign_links {
            io::Error, Io, #[cfg(not(test))];
        }

        errors {
            #[cfg(not(test))]
            AnError {

            }
        }
    }
}
