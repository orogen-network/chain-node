//! Subcommand dispatch for `chain-node`.
//!
//! Parses the `clap`-driven `Cli`, builds the requested chain spec, and
//! either runs a subcommand against the partial-components or starts the
//! full Aura + GRANDPA service.

use orogen_runtime::opaque::Block;
use sc_cli::SubstrateCli;
use sc_service::PartialComponents;

use crate::chain_spec;
use crate::cli::{Cli, Subcommand};
use crate::service;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Orogen Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/orogen-network/chain-node/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2026
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        // Production builds (no `dev-runtime` feature) refuse to start
        // without an explicit `--chain /path/to/spec.json` path. An empty
        // or unknown id is rejected; the operator must consciously choose
        // a spec.
        #[cfg(feature = "dev-runtime")]
        {
            return Ok(match id {
                "" | "dev" => Box::new(chain_spec::dev()?),
                "local" => Box::new(chain_spec::local()?),
                "forge" => Box::new(chain_spec::forge()?),
                path => Box::new(chain_spec::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?),
            });
        }
        #[cfg(not(feature = "dev-runtime"))]
        {
            if id.is_empty() || id == "dev" || id == "local" {
                return Err(format!(
                    "Refusing to start: production builds require an explicit \
                     `--chain /path/to/spec.json`. The id `{}` is only available \
                     in builds compiled with `--features dev-runtime`.",
                    id
                ));
            }
            Ok(Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(id),
            )?))
        }
    }
}

/// Entry point invoked from `main.rs`.
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                service::new_full::<
                    sc_network::NetworkWorker<
                        orogen_runtime::opaque::Block,
                        <orogen_runtime::opaque::Block as sp_runtime::traits::Block>::Hash,
                    >,
                >(config)
                .map_err(Into::into)
            })
        },
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        },
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        },
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, .. } =
                    service::new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        },
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, .. } =
                    service::new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        },
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        },
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        },
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, backend, .. } =
                    service::new_partial(&config)?;
                let aux_revert = Box::new(|client, _, blocks| {
                    sc_consensus_grandpa::revert(client, blocks)?;
                    Ok(())
                });
                Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
            })
        },
        Some(Subcommand::ChainInfo(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run::<Block>(&config))
        },
    }
}
