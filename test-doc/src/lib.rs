#[macro_use]
extern crate error_chain;

/// This module is used to check that all generated items are documented.
#[deny(missing_docs)]
pub mod doc {
    /// Inner module.
    pub mod inner {
        error_chain! {
        }
    }

    error_chain! {
        links {
            inner::Error, Test;
        }
        errors {
            Test2 {

            }
        }
    }
}
