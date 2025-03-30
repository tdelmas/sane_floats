#![doc = include_str!("../README.md")]

#![warn(clippy::indexing_slicing)]
#![warn(clippy::nursery)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_crate_dependencies)]

mod sqrt;

pub use sqrt::Sqrt;
