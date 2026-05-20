# chain-node

Substrate node binary for the Orogen chain. Wraps the
`orogen-runtime` from `pallet-suite/runtime` into a runnable Substrate
node with Aura block authoring and GRANDPA finality.

## Status

**Bootable.** `--dev --tmp` produces blocks immediately and finalizes them
via GRANDPA. The composed runtime exposes the full standard runtime API
set (`Core`, `BlockBuilder`, `TaggedTransactionQueue`, `OffchainWorkerApi`,
`AuraApi`, `GrandpaApi`, `SessionKeys`, `AccountNonceApi`,
`TransactionPaymentApi`, `Metadata`, `GenesisBuilder`).

## Layout

```
chain-node/
├── Cargo.toml          # client + RPC + consensus deps (47.x family)
├── build.rs            # substrate-build-script-utils
├── scripts/
│   └── validate-core2-workaround.sh
├── src/
│   ├── main.rs         # entrypoint
│   ├── cli.rs          # clap subcommand definitions (sc-cli types)
│   ├── command.rs      # subcommand dispatch + SubstrateCli impl
│   ├── chain_spec.rs   # dev / local builders with WASM genesis
│   ├── service.rs      # Aura + GRANDPA full service
│   └── rpc.rs          # system + payment RPC extensions
└── vendor/
    └── core2/          # local stub of yanked core2 0.4.0 (see below)
```

## Build & run

```bash
cd chain-node

# Build (~4 minutes cold, ~30s warm).
cargo build --release

# Boot a single-node dev chain. Alice authors with Aura; GRANDPA finalizes.
./target/release/chain-node --dev --tmp --port=43330 --rpc-port=43331

# Or just:
./target/release/chain-node --dev --tmp
```

You should see lines like:

```
🔨 Initializing Genesis block/state (state: 0x…, header-hash: 0x…)
🙌 Starting consensus session on top of parent 0x… (#0)
🎁 Prepared block for proposing at 1 (… ms) …
🔖 Pre-sealed block for proposal at 1. Hash now 0x…
🏆 Imported #1 (0x… → 0x…)
💤 Idle (0 peers), best: #1 (0x…), finalized #0 (0x…), ⬇ 0 ⬆ 0
```

Block time is 6 seconds (matches `pallet-nonce-vault::REPLAY_WINDOW_BLOCKS`'s
24h assumption). On a single-node `--dev` setup, GRANDPA finalizes blocks
one or two slots behind the head.

### Other useful invocations

```bash
./target/release/chain-node --help
./target/release/chain-node --version
./target/release/chain-node build-spec --chain dev > /tmp/dev-spec.json
./target/release/chain-node key generate --scheme sr25519
./target/release/chain-node purge-chain --dev -y
./target/release/chain-node chain-info --dev --base-path /tmp/llm-chain-data
```

## Dependency stance

Pinned to the May 2026 Substrate stable family. Selected versions:

| Layer | Versions |
|---|---|
| `sc-cli`, `sc-service`, `sc-executor`, `sc-rpc` | 0.59 / 0.58 / 0.49 / 52 |
| `sc-consensus`, `sc-consensus-aura`, `sc-consensus-grandpa` | 0.56 / 0.57 / 0.42 |
| `sc-consensus-manual-seal`, `sc-basic-authorship` | 0.58 / 0.55 |
| `sc-network`, `sc-network-sync`, `sc-transaction-pool` | 0.57 / 0.56 / 46 |
| `sp-runtime`, `sp-core`, `sp-io`, `sp-api` | 47 / 41 / 46 / 42 |
| `frame-support`, `frame-system`, `frame-executive` | 47 / 47 / 47 |
| `substrate-build-script-utils`, `substrate-wasm-builder` | 11 / 33 |

### Patched: `core2` (yanked-upstream workaround)

The crates.io publication of `sc-network 0.57.0` transitively depends on
`litep2p 0.13.3 → multihash 0.17 → core2 0.4.0`. **`core2` was yanked
from crates.io.** We work around this with a
`[patch.crates-io] core2 = { path = "vendor/core2" }` pointing at a local
stub that re-exports the needed `std::io` items
(`Read`, `Write`, `Error`, `ErrorKind`, `Result`, `BufRead`). The stub
ships under `vendor/core2/`. Drop the patch once Parity republishes
`sc-network` on top of `litep2p 0.14` (which uses `multihash 0.19+`, no
`core2` dependency).

The local release gate runs `scripts/validate-core2-workaround.sh`. It checks
that the locked dependency graph still reaches
`sc-network 0.57.0 -> litep2p 0.13.3 -> multihash 0.17.0 -> core2 0.4.0`,
then verifies in a temporary copy that removing the patch fails specifically
because `core2 0.4.0` is yanked. If a future upstream release removes that
edge, the validation fails and the patch can be deleted.

## Changes to `pallet-suite`

The runtime was upgraded to expose the standard runtime API set, switch
from `MockBlock` to `sp_runtime::generic::Block<Header, UncheckedExtrinsic>`,
add Aura/GRANDPA/Balances/Sudo/Timestamp/Transaction-Payment pallets, and
build a WASM blob via `substrate-wasm-builder`. See
`pallet-suite/README.md` for details.

## License

Apache-2.0
