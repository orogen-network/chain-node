//! Local in-tree stub of [`core2`] 0.4.0.
//!
//! Upstream `core2` was yanked from crates.io but is referenced
//! transitively by Substrate's `sc-network → litep2p 0.13.3 → multihash
//! 0.17`. This stub re-exports the bits of `std::io` that `multihash`
//! actually uses (`Read`, `Write`, `Error`, `ErrorKind`, `Result`) so the
//! workspace links.
//!
//! Remove once Parity publishes a refreshed `sc-network` (post-litep2p
//! 0.14 → multihash 0.19, which has no `core2` dep).

#![allow(missing_docs)]

pub mod io {
    pub use std::io::{Error, ErrorKind, Read, Result, Write};

    // `core2`'s real BufRead is identical to std's.
    pub use std::io::BufRead;
}

pub use io::Error;
