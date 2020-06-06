//! Rep is a small tool for checking representation/class invariants

pub use rep_derive::*;
pub use rep_derive::check_rep;
pub use log::Level::Error;
pub use log::{error, log_enabled};

/// A trait for representation checking
pub trait CheckRep {
	/// Returns true if representation is correct, false otherwise
    fn is_correct(&self) -> bool {
    	self.correctness().is_ok()
    }

    /// Returns Ok if representation is correct, vector of errors otherwise
    fn correctness(&self) -> Result<(), Vec<String>> {
    	if self.is_correct() {
    		Ok(())
    	} else {
    		Err(vec![])
    	}
    }

    /// Asserts that self is correct
	fn check_rep(&self) {
		match self.correctness() {
			Ok(_) => {}
			Err(errors) => {
				if log_enabled!(Error) {
					for error in errors {
						error!("representation invariant violated: {:?}", error);
					}
				} else {
					if errors.len() > 0 {
						panic!("representation invariant violated: {:?}", errors);	
					} else {
						panic!("representation invariant violated");	
					}
				}
			}
		}
	}
}

/// A trait for adding extra rep-checking functionality to a data structure with `CheckRep` implemented
pub trait CustomCheckRep {
	/// Returns true if representation is correct, false otherwise
    fn c_is_correct(&self) -> bool {
    	self.c_correctness().is_ok()
    }

    /// Returns Ok if representation is correct, vector of errors otherwise
    fn c_correctness(&self) -> Result<(), Vec<String>> {
    	if self.c_is_correct() {
    		Ok(())
    	} else {
    		Err(vec![])
    	}
    }
}