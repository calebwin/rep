//! Rep is a small tool for checking representation/class invariants

pub use rep_derive::*;
pub use rep_derive::check_rep;

/// A trait for representation checking
pub trait CheckRep {
	/// Returns true if representation is correct, false otherwise
    fn is_correct(&self) -> bool;

    /// Asserts that self is correct
	fn check_rep(&self) {
		assert!(self.is_correct());
	}    
}
