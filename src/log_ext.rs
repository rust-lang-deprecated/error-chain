#[cfg(feature = "logging")]
#[macro_use]
mod impl_log_ext {
#[macro_export]
macro_rules! impl_result_log_ext {
    ( $result_log_ext_name:ident, $error_name:ident ) => (
        /// Extend chained errors to be able to log them on the spot using the log crate.
        /// See [`loge`], [`logw`], [`logi`], [`logd`], [`logt`] functions.
        pub trait $result_log_ext_name {
            /// Log error using the log crate `error!` macro.
            fn loge(self) -> Self; 
            /// Log error using the log crate `warn!` macro.
            fn logw(self) -> Self;
            /// Log error using the log crate `info!` macro.
            fn logi(self) -> Self;
            /// Log error using the log crate `debug!` macro.
            fn logd(self) -> Self;
            /// Log error using the log crate `trace!` macro.
            fn logt(self) -> Self;
        }

        impl<T> $result_log_ext_name for ::std::result::Result<T,$error_name> 
        {
             impl_make_log_fn_for_result!( loge, error, Error);
             impl_make_log_fn_for_result!( logw, warn, Warn);
             impl_make_log_fn_for_result!( logi, info, Info);
             impl_make_log_fn_for_result!( logd, debug, Debug);
             impl_make_log_fn_for_result!( logt, trace, Trace);
        }

        impl $result_log_ext_name for $error_name
        {
             impl_make_log_fn_for_chained_error!( loge, error, Error);
             impl_make_log_fn_for_chained_error!( logw, warn, Warn);
             impl_make_log_fn_for_chained_error!( logi, info, Info);
             impl_make_log_fn_for_chained_error!( logd, debug, Debug);
             impl_make_log_fn_for_chained_error!( logt, trace, Trace);
        }

    )
}

/// Internal macro used to implement the logX() functions
/// It logs the causes of the error using the specified log crate level:
/// For example:
///
/// `log_causes!(err,info)`
// #[cfg(feature = "logging")]
#[macro_export]
macro_rules! impl_log_causes {
    ($e:expr, $level:ident) => (
        for c in $e.iter().skip(1) {
            $level!("     caused by: {}", c);
        }

        if let Some(backtrace) = $e.backtrace() {
            $level!("backtrace: {:?}", backtrace);
        }
    )
}

/// Internal macro used to implement the logX() functions
/// It generates a function that logs the error and its causes
/// using the specified log crate level.
/// 1st argument -$name: Function name
/// 2nd argument -$level: log macro to use (error, warn, info, debug, trace)
/// 3nd argument -$lvlchk: Do not execute the code
///                        if logging is not enabled for this level
///                        (Error, Warnm, Indo, Debug, Trace)
///
/// For example:
///
/// `log_error!(err,info,Info)`
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! impl_make_log_fn_for_result {
    ($name:ident, $level:ident, $lvlchk:ident) => (
        fn $name(self) -> Self {
            use log;
            if let Err(ref e) = self {
                if log_enabled!(log::LogLevel::$lvlchk) {
                    $level!("{}", e);
                    impl_log_causes!(e,$level);
                }
            };
            self
        }
    )
}

/// Internal implementation macro for logging the chained error type
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! impl_make_log_fn_for_chained_error {
    ($name:ident, $level:ident, $lvlchk:ident) => (
        fn $name(self) -> Self {
            use log;
            if log_enabled!(log::LogLevel::$lvlchk) {
                $level!("{}", self);
                impl_log_causes!(self,$level);
            };
            self
        }
    )
}
}
