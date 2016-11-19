#![deny(missing_docs)]

//! This module is used to check that all generated items are documented.

#[macro_use]
extern crate error_chain;

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

fn main() {}
