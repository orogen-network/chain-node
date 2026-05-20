//! RPC extensions for `chain-node`.
//!
//! Exposes the standard substrate RPC (`system`, `chain`, `state`, `author`,
//! `payment`). Custom Orogen RPCs (e.g. `bme_status`, `jobs_pending`)
//! will land here when their backing runtime APIs are implemented.

use std::sync::Arc;

use jsonrpsee::RpcModule;
use orogen_runtime::{opaque::Block, AccountId, Balance, Nonce};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Dependencies for RPC building.
pub struct FullDeps<C, P> {
    pub client: Arc<C>,
    pub pool: Arc<P>,
}

/// Build the full RPC module for the node.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, sc_service::Error>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let FullDeps { client, pool } = deps;

    module
        .merge(System::new(client.clone(), pool).into_rpc())
        .map_err(|e| sc_service::Error::Other(e.to_string()))?;
    module
        .merge(TransactionPayment::new(client).into_rpc())
        .map_err(|e| sc_service::Error::Other(e.to_string()))?;
    Ok(module)
}
