//! Experimental extensions to the `std::time` module.
#![warn(missing_docs)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

pub use duration::DurationExt;
#[cfg(feature = "beta")]
pub use system_time::SystemTimeExt;

mod duration;
#[cfg(feature = "beta")]
mod system_time;
