

pub extern crate lubanso_hashes as hashes;

// May depend on crate features and we don't want to bother with it
#[allow(unused)]
#[cfg(feature = "std")]
use std::error::Error as StdError;
#[cfg(feature = "std")]
use std::io;

#[allow(unused)]
#[cfg(not(feature = "std"))]
use core2::error::Error as StdError;
#[cfg(not(feature = "std"))]
use core2::io;