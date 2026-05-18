//! Chain specifications for `chain-node`.
//!
//! Two predefined spec IDs (`dev`, `local`) are gated behind the
//! `dev-runtime` Cargo feature so production builds cannot accidentally
//! launch with the well-known Alice/Bob seeds. In production builds, the
//! only way to start the node is by passing an explicit `--chain
//! /path/to/spec.json`.

use orogen_runtime::{AccountId, WASM_BINARY};
#[cfg(feature = "dev-runtime")]
use sc_chain_spec::ChainType;
use sc_chain_spec::Properties;
#[cfg(feature = "dev-runtime")]
use sp_core::{sr25519, Pair};
#[cfg(feature = "dev-runtime")]
use sp_runtime::traits::{IdentifyAccount, Verify};
#[cfg(feature = "dev-runtime")]
use sp_core::crypto::Ss58Codec;

/// Concrete chain-spec type for this node. No extensions, no host functions.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Token / chain properties published in chain specs.
pub struct ChainProperties {
    pub token_symbol: &'static str,
    pub token_decimals: u32,
    pub ss58_format: u16,
}

pub const DEFAULT_PROPERTIES: ChainProperties = ChainProperties {
    token_symbol: "CUC",
    token_decimals: 12,
    ss58_format: 42,
};

#[allow(dead_code)]
fn props() -> Properties {
    let mut p = Properties::new();
    p.insert("tokenSymbol".into(), DEFAULT_PROPERTIES.token_symbol.into());
    p.insert("tokenDecimals".into(), DEFAULT_PROPERTIES.token_decimals.into());
    p.insert("ss58Format".into(), DEFAULT_PROPERTIES.ss58_format.into());
    p
}

/// Generate an account id from a seed (sr25519). Dev-only.
#[cfg(feature = "dev-runtime")]
pub fn account_from_seed(seed: &str) -> AccountId {
    let pair = sr25519::Pair::from_string(&format!("//{seed}"), None)
        .expect("static seed always valid");
    <<sp_runtime::MultiSignature as Verify>::Signer as IdentifyAccount>::into_account(
        pair.public().into(),
    )
}

/// Aura authority from a seed. Dev-only.
#[cfg(feature = "dev-runtime")]
pub fn aura_authority_from_seed(seed: &str) -> sp_consensus_aura::sr25519::AuthorityId {
    sp_core::sr25519::Pair::from_string(&format!("//{seed}"), None)
        .expect("static seed always valid")
        .public()
        .into()
}

/// GRANDPA authority from a seed. Dev-only.
#[cfg(feature = "dev-runtime")]
pub fn grandpa_authority_from_seed(seed: &str) -> sp_consensus_grandpa::AuthorityId {
    sp_core::ed25519::Pair::from_string(&format!("//{seed}"), None)
        .expect("static seed always valid")
        .public()
        .into()
}

/// Build a JSON genesis patch for the runtime's `RuntimeGenesisConfig`.
#[cfg(feature = "dev-runtime")]
fn genesis_patch(
    sudo: AccountId,
    endowed: Vec<AccountId>,
    aura_authorities: Vec<sp_consensus_aura::sr25519::AuthorityId>,
    grandpa_authorities: Vec<sp_consensus_grandpa::AuthorityId>,
) -> serde_json::Value {
    let balances: Vec<(AccountId, u128)> =
        endowed.into_iter().map(|a| (a, 1u128 << 60)).collect();
    serde_json::json!({
        "balances": { "balances": balances },
        "aura": { "authorities": aura_authorities },
        "grandpa": {
            "authorities": grandpa_authorities.into_iter().map(|a| (a, 1u64)).collect::<Vec<_>>(),
        },
        "sudo": { "key": Some(sudo) },
    })
}

/// Dev chain spec — single Alice authority, instant sealing optional.
#[cfg(feature = "dev-runtime")]
pub fn dev() -> Result<ChainSpec, String> {
    let wasm = WASM_BINARY.ok_or("WASM binary not available; build with `cargo build`.")?;
    let alice = account_from_seed("Alice");
    let bob = account_from_seed("Bob");
    log::warn!(
        "Building DEV chain-spec with the well-known //Alice seed as sudo. \
         NEVER use this spec for a production network."
    );
    Ok(ChainSpec::builder(wasm, Default::default())
        .with_name("Orogen Dev")
        .with_id("llm_mining_dev")
        .with_chain_type(ChainType::Development)
        .with_properties(props())
        .with_genesis_config_patch(genesis_patch(
            alice.clone(),
            vec![alice.clone(), bob.clone()],
            vec![aura_authority_from_seed("Alice")],
            vec![grandpa_authority_from_seed("Alice")],
        ))
        .build())
}

/// Local testnet chain spec — Alice + Bob authorities.
#[cfg(feature = "dev-runtime")]
pub fn local() -> Result<ChainSpec, String> {
    let wasm = WASM_BINARY.ok_or("WASM binary not available; build with `cargo build`.")?;
    let alice = account_from_seed("Alice");
    let bob = account_from_seed("Bob");
    let charlie = account_from_seed("Charlie");
    log::warn!(
        "Building LOCAL chain-spec with well-known //Alice/Bob/Charlie seeds. \
         NEVER use this spec for a production network."
    );
    Ok(ChainSpec::builder(wasm, Default::default())
        .with_name("Orogen Local Testnet")
        .with_id("llm_mining_local")
        .with_chain_type(ChainType::Local)
        .with_properties(props())
        .with_genesis_config_patch(genesis_patch(
            alice.clone(),
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![
                aura_authority_from_seed("Alice"),
                aura_authority_from_seed("Bob"),
            ],
            vec![
                grandpa_authority_from_seed("Alice"),
                grandpa_authority_from_seed("Bob"),
            ],
        ))
        .build())
}

/// Orogen Forge testnet chain spec.
///
/// Single-validator forge testnet seeded by the foundation. Authority and
/// sudo accounts are freshly-generated sr25519/ed25519 keys (no well-known
/// `//Alice` seeds) so that the chain id `orogen_forge` cannot be hijacked
/// by anyone running the same dev seeds.
///
/// Public keys are embedded as raw hex below; the corresponding private
/// keys live in the validator's local keystore on the seed node and are
/// not part of the public chain spec.
#[cfg(feature = "dev-runtime")]
pub fn forge() -> Result<ChainSpec, String> {
    use sp_core::crypto::AccountId32;
    let wasm = WASM_BINARY.ok_or("WASM binary not available; build with `cargo build`.")?;

    let sudo: AccountId =
        AccountId32::from_ss58check("5FeYeFTh4xa6n85zq1Aswn2vKbuRW76HvVcXPfi2pdsFBGLR")
            .map_err(|e| format!("invalid forge sudo ss58: {e:?}"))?;

    let validator: AccountId =
        AccountId32::from_ss58check("5Fsykze6UqiXB9VrLr92icHG64AQckwRLuNZWAdXkuU92y6n")
            .map_err(|e| format!("invalid forge validator ss58: {e:?}"))?;

    let aura_authority: sp_consensus_aura::sr25519::AuthorityId =
        sp_core::sr25519::Public::from_ss58check(
            "5Fsykze6UqiXB9VrLr92icHG64AQckwRLuNZWAdXkuU92y6n",
        )
        .map_err(|e| format!("invalid forge aura ss58: {e:?}"))?
        .into();

    let grandpa_authority: sp_consensus_grandpa::AuthorityId =
        sp_core::ed25519::Public::from_ss58check(
            "5DSGzY62YjTmcgspz3wzZWFdG3SpuDc9pgzYgMhp1DeBCVBv",
        )
        .map_err(|e| format!("invalid forge grandpa ss58: {e:?}"))?
        .into();

    let mut p = props();
    p.insert("tokenSymbol".into(), "OROG".into());
    p.insert("tokenDecimals".into(), 12u32.into());

    Ok(ChainSpec::builder(wasm, Default::default())
        .with_name("Orogen Forge Testnet")
        .with_id("orogen_forge")
        .with_chain_type(ChainType::Live)
        .with_protocol_id("orogenforge")
        .with_properties(p)
        .with_genesis_config_patch(genesis_patch(
            sudo,
            vec![validator],
            vec![aura_authority],
            vec![grandpa_authority],
        ))
        .build())
}

// Suppress unused-import warnings of `AccountId` etc. when the feature is off.
#[cfg(not(feature = "dev-runtime"))]
#[allow(dead_code)]
fn _account_id_used() -> Option<AccountId> {
    let _ = WASM_BINARY;
    None
}
