// Lubanso Bitcoin Library

//! Provides type [`LockTime`] that implements the logic around nLockTime/OP_CHECKLOCKTIMEVERIFY.
//!
//! There are two types of lock time: lock-by-blockheight and lock-by-blocktime, distinguished by
//! whether `LockTime < LOCKTIME_THRESHOLD`.
//!

use core::{mem, fmt};
use core::cmp::{PartialOrd, Ordering};
use core::convert::TryFrom;
use core::str::FromStr;

use lubanso_internals::write_err;

#[cfg(all(test, mutate))]
use mutagen::mutate;


