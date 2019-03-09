extern crate version_check;

use version_check::is_min_version;

fn main() {
    // Switch on for versions that have Error::source
    // As introduced by https://github.com/rust-lang/rust/pull/53533
    if is_min_version("1.30").map(|(is_high_enough, _actual_version)| is_high_enough).unwrap_or(false)
    {
        println!("cargo:rustc-cfg=has_error_source");
    }
}
