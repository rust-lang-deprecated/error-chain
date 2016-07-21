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
