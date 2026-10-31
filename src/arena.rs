//! Arena: strings that live as long as the arena, handed out through &self.

use std::cell::RefCell;

/// Owns every string it hands out, and frees them all when it's dropped.
#[derive(Default)]
pub struct Arena {
    strings: RefCell<Vec<*mut str>>,
}

impl Arena {
    /// A copy of `s` that lives as long as the arena: `&self` in, `&str` out.
    pub fn alloc(&self, s: &str) -> &str {
        let ptr = Box::into_raw(Box::<str>::from(s)); // a heap copy that never moves
        self.strings.borrow_mut().push(ptr);
        unsafe { &*ptr }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        for &ptr in self.strings.get_mut().iter() {
            unsafe { drop(Box::from_raw(ptr)) };
        }
    }
}
