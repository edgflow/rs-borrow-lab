//! Helpers shared by the examples.

/// A value with a name, that tells you when it is dropped.
pub struct Noisy(pub &'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}
