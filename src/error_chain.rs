#[macro_export]
macro_rules! error_chain {
    (
        $( @processed )*
        types {
            $error_name:ident, $error_kind_name:ident, $result_name:ident;
        }

        links {
            $( $link_error_path:path, $link_variant:ident $(, #[$meta_links:meta])*; ) *
        }

        foreign_links {
            $( $foreign_link_error_path:path, $foreign_link_variant:ident $(, #[$meta_foreign_links:meta])*; )*
        }

        errors {
            $( $error_chunks:tt ) *
        }

    ) => {
        /// The Error type
        ///
        /// This has a simple structure to support pattern matching
        /// during error handling. The second field is internal state
        /// that is mostly irrelevant for error handling purposes.
        #[derive(Debug)]
        pub struct $error_name {
            /// The kind of the error.
            pub kind: $error_kind_name,
            /// Contains the error chain and the backtrace.
            pub state: $crate::State,
        }

        impl_error!($error_name $error_kind_name $($link_error_path)*);

        impl $error_name {
            /// Constructs an error from a kind.
            pub fn from_kind(kind: $error_kind_name) -> $error_name {
                $error_name {
                    kind: kind,
                    state: $crate::State::default(),
                }
            }

            /// Returns the kind of the error.
            pub fn kind(&self) -> &$error_kind_name {
                &self.kind
            }

            /// Iterates over the error chain.
            pub fn iter(&self) -> $crate::ErrorChainIter {
                $crate::ErrorChainIter(Some(self))
            }
        }

        impl ::std::error::Error for $error_name {
            fn description(&self) -> &str {
                self.kind.description()
            }

            fn cause(&self) -> Option<&::std::error::Error> {
                match self.state.next_error {
                    Some(ref c) => Some(&**c),
                    None => {
                        match self.kind {
                            $(
                                $(#[$meta_foreign_links])*
                                $error_kind_name::$foreign_link_variant(ref foreign_err) => {
                                    foreign_err.cause()
                                }
                            ) *
                            _ => None
                        }
                    }
                }
            }
        }

        impl ::std::fmt::Display for $error_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.kind, f)
            }
        }

        $(
            $(#[$meta_links])*
            impl From<$link_error_path> for $error_name {
                fn from(e: $link_error_path) -> Self {
                    $error_name {
                        kind: $error_kind_name::$link_variant(e.kind),
                        state: e.state,
                    }
                }
            }
        ) *

        $(
            $(#[$meta_foreign_links])*
            impl From<$foreign_link_error_path> for $error_name {
                fn from(e: $foreign_link_error_path) -> Self {
                    $error_name::from_kind(
                        $error_kind_name::$foreign_link_variant(e)
                    )
                }
            }
        ) *

        impl From<$error_kind_name> for $error_name {
            fn from(e: $error_kind_name) -> Self {
                $error_name::from_kind(e)
            }
        }

        impl<'a> From<&'a str> for $error_name {
            fn from(s: &'a str) -> Self {
                $error_name::from_kind(s.into())
            }
        }

        impl From<String> for $error_name {
            fn from(s: String) -> Self {
                $error_name::from_kind(s.into())
            }
        }

        impl ::std::ops::Deref for $error_name {
            type Target = $error_kind_name;

            fn deref(&self) -> &Self::Target {
                &self.kind
            }
        }


        // The ErrorKind type
        // --------------

        quick_error! {
            /// The kind of an error
            #[derive(Debug)]
            pub enum $error_kind_name {

                Msg(s: String) {
                    description(&s)
                    display("{}", s)
                }

                $(
                    $(#[$meta_links])*
                    $link_variant(e: <$link_error_path as $crate::ChainedError>::ErrorKind) {
                        description(e.description())
                        display("{}", e)
                    }
                ) *

                $(
                    $(#[$meta_foreign_links])*
                    $foreign_link_variant(err: $foreign_link_error_path) {
                        description(::std::error::Error::description(err))
                        display("{}", err)
                    }
                ) *

                $($error_chunks)*
            }
        }

        $(
            $(#[$meta_links])*
            impl From<<$link_error_path as $crate::ChainedError>::ErrorKind> for $error_kind_name {
                fn from(e: <$link_error_path as $crate::ChainedError>::ErrorKind) -> Self {
                    $error_kind_name::$link_variant(e)
                }
            }
        ) *

        impl<'a> From<&'a str> for $error_kind_name {
            fn from(s: &'a str) -> Self {
                $error_kind_name::Msg(s.to_string())
            }
        }

        impl From<String> for $error_kind_name {
            fn from(s: String) -> Self {
                $error_kind_name::Msg(s)
            }
        }

        impl From<$error_name> for $error_kind_name {
            fn from(e: $error_name) -> Self {
                e.kind
            }
        }

        /// Convenient wrapper around `std::Result`.
        pub type $result_name<T> = ::std::result::Result<T, $error_name>;
    };

    // Handle missing sections, or missing type names in types { }
    //
    // Macros cannot specify "zero or one repetitions" at the moment, so we allow
    // repeating sections. Only for the `types` section this makes no sense, which
    // is the reason for the three separate cases.
    //
    // Case 1: types fully specified
    (
        $( @processed )*
        types {
            $error_name:ident, $error_kind_name:ident, $result_name:ident;
        }

        $( links {
            $( $link_chunks:tt ) *
        } ) *

        $( foreign_links {
            $( $foreign_link_chunks:tt ) *
        } ) *

        $( errors {
            $( $error_chunks:tt ) *
        } ) *
    ) => (
        error_chain! {
            @processed
            types {
                $error_name, $error_kind_name, $result_name;
            }

            links {
                $( $( $link_chunks ) * ) *
            }

            foreign_links {
                $( $( $foreign_link_chunks ) * ) *
            }

            errors {
                $( $( $error_chunks ) * ) *
            }
        }
    );
    // Case 2: types section present, but empty
    (
        $( @processed )*
        types { }

        $( links {
            $( $link_chunks:tt ) *
        } ) *

        $( foreign_links {
            $( $foreign_link_chunks:tt ) *
        } ) *

        $( errors {
            $( $error_chunks:tt ) *
        } ) *
    ) => (
        error_chain! {
            @processed
            types {
                Error, ErrorKind, Result;
            }

            links {
                $( $( $link_chunks ) * ) *
            }

            foreign_links {
                $( $( $foreign_link_chunks ) * ) *
            }

            errors {
                $( $( $error_chunks ) * ) *
            }
        }
    );
    // Case 3: types section not present
    (
        $( @processed )*
        $( links {
            $( $link_chunks:tt ) *
        } ) *

        $( foreign_links {
            $( $foreign_link_chunks:tt ) *
        } ) *

        $( errors {
            $( $error_chunks:tt ) *
        } ) *
    ) => (
        error_chain! {
            @processed
            types { }

            links {
                $( $( $link_chunks ) * ) *
            }

            foreign_links {
                $( $( $foreign_link_chunks ) * ) *
            }

            errors {
                $( $( $error_chunks ) * ) *
            }
        }
    );
    (
        @processing ($a:tt, $b:tt, $c:tt, $d:tt)
        types $content:tt
        $( $tail:tt )*
    ) => {
        error_chain! {
            @processing ($content, $b, $c, $d)
            $($tail)*
        }
    };
    (
        @processing ($a:tt, $b:tt, $c:tt, $d:tt)
        links $content:tt
        $( $tail:tt )*
    ) => {
        error_chain! {
            @processing ($a, $content, $c, $d)
            $($tail)*
        }
    };
    (
        @processing ($a:tt, $b:tt, $c:tt, $d:tt)
        foreign_links $content:tt
        $( $tail:tt )*
    ) => {
        error_chain! {
            @processing ($a, $b, $content, $d)
            $($tail)*
        }
    };
    (
        @processing ($a:tt, $b:tt, $c:tt, $d:tt)
        errors $content:tt
        $( $tail:tt )*
    ) => {
        error_chain! {
            @processing ($a, $b, $c, $content)
            $($tail)*
        }
    };
    (
        @processing ($a:tt, $b:tt, $c:tt, $d:tt)
    ) => {
        error_chain! {
            @processed
            types $a
            links $b
            foreign_links $c
            errors $d
        }
    };
    (
        @processed
        $( $block_name:ident $block_content:tt )*
    ) => {
        !!!!!parse error!!!!!
    };
    (
        $( $block_name:ident $block_content:tt )*
    ) => {
        error_chain! {
            @processing ({}, {}, {}, {})
            $($block_name $block_content)+
        }
    };
}

/// Macro used to manage the `backtrace` feature.
///
/// See
/// https://www.reddit.com/r/rust/comments/57virt/hey_rustaceans_got_an_easy_question_ask_here/da5r4ti/?context=3
/// for more details.
#[macro_export]
#[doc(hidden)]
#[cfg(feature = "backtrace")]
macro_rules! impl_error {
    ($error_name: ident
     $error_kind_name: ident
     $($link_error_path: path)*) => {
        impl $error_name {
            /// Returns the backtrace associated with this error.
            pub fn backtrace(&self) -> Option<&$crate::Backtrace> {
                self.state.backtrace.as_ref().map(|v| &**v)
            }
        }

        impl $crate::ChainedError for $error_name {
            type ErrorKind = $error_kind_name;

            fn new(kind: $error_kind_name, state: $crate::State) -> $error_name {
                $error_name {
                    kind: kind,
                    state: state,
                }
            }

            fn extract_backtrace(e: &(::std::error::Error + Send + 'static))
                -> Option<Option<::std::sync::Arc<$crate::Backtrace>>> {
                if let Some(e) = e.downcast_ref::<$error_name>() {
                    Some(e.state.backtrace.clone())
                }
                $(
                    else if let Some(e) = e.downcast_ref::<$link_error_path>() {
                        Some(e.state.backtrace.clone())
                    }
                ) *
                else {
                    None
                }
            }
        }
    }
}

/// Macro used to manage the `backtrace` feature.
///
/// See
/// https://www.reddit.com/r/rust/comments/57virt/hey_rustaceans_got_an_easy_question_ask_here/da5r4ti/?context=3
/// for more details.
#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "backtrace"))]
macro_rules! impl_error {
    ($error_name: ident
     $error_kind_name: ident
     $($link_error_path: path)*) => {
        impl $crate::ChainedError for $error_name {
            type ErrorKind = $error_kind_name;

            fn new(kind: $error_kind_name, state: $crate::State) -> $error_name {
                $error_name {
                    kind: kind,
                    state: state,
                }
            }
        }
    }
}
