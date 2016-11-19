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
            inner::Error, Test, #[doc = "Doc"];
        }
        foreign_links {
            ::std::io::Error, Io, #[doc = "Io"];
        }
        errors {
            /// Doc
            Test2 {

            }
        }
    }
}
