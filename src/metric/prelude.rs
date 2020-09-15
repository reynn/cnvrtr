pub use super::SiPrefix;
pub use crate::metric::length::*;
pub use crate::metric::*;
pub use crate::traits::*;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
