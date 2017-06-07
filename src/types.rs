//! Re-exports of types for glossing over no_std/std distinctions

/// boxed
#[cfg(not(feature = "std"))]
pub use collections::boxed;
#[cfg(feature = "std")]
pub use std::boxed;

/// fmt
#[cfg(not(feature = "std"))]
pub use core::fmt;
#[cfg(feature = "std")]
pub use std::fmt;

/// ops
#[cfg(not(feature = "std"))]
pub use core::ops;
#[cfg(feature = "std")]
pub use std::ops;

/// error
#[cfg(not(feature = "std"))]
pub use error;
#[cfg(feature = "std")]
pub use std::error;

/// result
#[cfg(not(feature = "std"))]
pub use core::result;
#[cfg(feature = "std")]
pub use std::result;

/// string
#[cfg(not(feature = "std"))]
pub use collections::string;
#[cfg(feature = "std")]
pub use std::string;
