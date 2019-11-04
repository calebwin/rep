//! Rep is a small tool for checking representation/class invariants

pub use rep_derive::*;
pub use rep_derive::check_rep;

/// A trait for representation checking
pub trait CheckRep {
	/// Returns true if representation is valid, false otherwise
    fn check_rep(&self) -> bool;
}
