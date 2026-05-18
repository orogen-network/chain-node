//! Clap subcommand definitions for the Orogen node.
//!
//! Mirrors the standard `substrate-node-template` CLI surface.

use clap::Parser;

/// Top-level CLI for `chain-node`.
#[derive(Debug, Parser)]
#[command(
    name = "chain-node",
    about = "Orogen Substrate node",
    version
)]
pub struct Cli {
    /// Subcommand. If absent, the node is started with `RunCmd` semantics.
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Standard Substrate run flags (`--dev`, `--tmp`, `--rpc-port`, etc).
    #[command(flatten)]
    pub run: sc_cli::RunCmd,
}

/// All supported subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management CLI utilities.
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Display some chain information.
    ChainInfo(sc_cli::ChainInfoCmd),
}
