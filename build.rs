extern crate version_check;

use version_check::{is_min_date, is_min_version, is_nightly};

fn main() {
    // Switch on for versions that have Error::source
    // As introduced by https://github.com/rust-lang/rust/pull/53533
    if (is_nightly().unwrap_or(false)
            && is_min_date("2018-09-02")
                .unwrap_or((false, "".to_string())).0)
        || is_min_version("1.30").unwrap_or((false, "".to_string())).0
    {
        println!("cargo:rustc-cfg=has_error_source");
    }
}
