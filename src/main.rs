//! `chain-node` — Substrate node binary for the Orogen.
//!
//! Wraps `orogen-runtime` (the pallet-suite composed runtime) into a
//! runnable Substrate node with Aura block production and GRANDPA
//! finality. `--dev --tmp` boots a single-node Alice authority and
//! produces blocks immediately.
#![warn(missing_docs)]

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
    command::run()
}
