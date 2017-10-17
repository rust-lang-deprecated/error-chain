//! These modules show an example of code generated by the macro. **IT MUST NOT BE
//! USED OUTSIDE THIS CRATE**.
//!
//! This is the basic error structure. You can see that `ErrorKind`
//! has been populated in a variety of ways. All `ErrorKind`s get a
//! `Msg` variant for basic errors. When strings are converted to
//! `ErrorKind`s they become `ErrorKind::Msg`. The "links" defined in
//! the macro are expanded to the `Inner` variant, and the
//! "foreign links" to the `Io` variant.
//!
//! Both types come with a variety of `From` conversions as well:
//! `Error` can be created from `ErrorKind`, `&str` and `String`,
//! and the `links` and `foreign_links` error types. `ErrorKind`
//! can be created from the corresponding `ErrorKind`s of the link
//! types, as well as from `&str` and `String`.
//!
//! `into()` and `From::from` are used heavily to massage types into
//! the right shape. Which one to use in any specific case depends on
//! the influence of type inference, but there are some patterns that
//! arise frequently.

/// Another code generated by the macro.

pub mod inner {
    error_chain! {
        derive {
            //PartialEq, PartialEq<Error>
        }
    }
}

error_chain! {
    links {
        Inner(inner::Error, inner::ErrorKind, inner::Trait) #[doc = "Link to another `ErrorChain`."];
    }
    foreign_links {
        //Io(::std::io::Error) #[doc = "Link to a `std::error::Error` type."];
    }
    errors {
        #[doc = "A custom error kind."]
        Custom
    }
    derive {
        //PartialEq, PartialEq<Error>
    }
}

//fn foo<T: PartialEq>() {}
//fn bar() {
//foo::<Error>();
//}
