//! Application composition and greeting.

pub use model;

/// Print the template greeting.
#[allow(clippy::print_stdout, reason = "The template greeting intentionally writes to stdout.")]
pub fn greet() {
    println!("Hello, world!");
}
