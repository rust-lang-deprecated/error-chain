#[doc(hidden)]
#[cfg(feature = "sync")]
pub trait StdError: std::error::Error + Send + Sync { }
#[doc(hidden)]
#[cfg(not(feature = "sync"))]
pub trait StdError: std::error::Error + Send { }
#[cfg(feature = "sync")]
impl<E> StdError for E where E: std::error::Error + Send + Sync { }
#[cfg(not(feature = "sync"))]
impl<E> StdError for E where E: std::error::Error + Send { }
#[doc(hidden)]
#[cfg(feature = "sync")]
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
#[doc(hidden)]
#[cfg(not(feature = "sync"))]
pub type BoxError = Box<dyn std::error::Error + Send>;

#[doc(hidden)]
#[cfg(feature = "sync")]
pub trait TraitCheck: Send + Sync { }
#[doc(hidden)]
#[cfg(not(feature = "sync"))]
pub trait TraitCheck: Send { }