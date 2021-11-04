// Doc: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#publishing-a-crate-to-cratesio
// Run cargo doc --open to generate and open documentation html
// Documentation comments run as tests (for cargo test)

// Documents the item that contains the comments rather than adding documentation
// to the items following the comments
//! # Publishing a Crate to Crates.io
//!
//! `publishing_crate_cratesio` is a collection of utilities to make performing
//! calculations more convenient.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publishing_crate_cratesio::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Deeply nested modules can be re-eported for the convenience of the crate user
// using `pub use deeply::nested::module without changing the internal structure
// See https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
// (Re-exports get listed in the documentation)

// Run `cargo publish` to publish crate to crates.io
// (needs authentification: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#setting-up-a-cratesio-account)
// Change version value to publish a new version to crates.io (Semantic Versioning: https://semver.org)

// Removing Versions from Cartes.io with `cargo yank`
// Prevent new projects from using a broken crate version
// `cargo yank --vers 1.0.1
// `cargo yank --vers 1.0.1 --undo`
