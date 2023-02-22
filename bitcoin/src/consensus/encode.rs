// Lubanso Bitcoin Library

//! Bitcoin consensus-encodable types.
//!
//! This is basically a replacement of the `Encodable` trait which does
//! normalization of endianness etc., to ensure that the encoding matches
//! the network consensus encoding.
//!
//! Essentially, anything that must go on the _disk_ or _network_ must be
//! encoded using the `Encodable` trait, since this data must be the same for
//! all systems. Any data going to the _user_ e.g., over JSONRPC, should use the
//! ordinary `Encodable` trait. (This should also be the same across systems, of
//! course, but has some critical differences from the network format e.g.,
//! scripts come with an opcode decode, hashes are big-endian, numbers are
//! typically big-endian decimals, etc.)
//!

use crate::prelude::*;
use core::{fmt, mem, u32, convert::From};

use lubanso_internals::write_err;
use crate::hashes::{sha256d, Hash, sha256};
use crate::hash_types::{BlockHash, FilterHash, TxMerkleNode, FilterHeader};
use crate::io::{self, Cursor, Read};




