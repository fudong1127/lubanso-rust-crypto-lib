//! Bitcoin consensus.
//!
//! This module defines structures, functions, and traits that are needed to
//! conform to Bitcoin consensus.
//!

pub mod encode;

pub use self::encode::{Encodable, Decodable, WriteExt, ReadExt};
pub use self::encode::{serialize, deserialize, deserialize_partial};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;