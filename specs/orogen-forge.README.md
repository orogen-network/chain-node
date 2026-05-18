# Orogen Forge Testnet — public seed validator metadata

**Status:** running on `edge-01.orogen.network` since 2026-05-18 (cycle 13).
Single-validator forge testnet, foundation-seeded. Pallet logic is still
skeleton (see `pallet-suite/HANDOFF.md`); treat this chain as a connectivity
/ tooling testnet, not an economically meaningful one.

## Public endpoints

| Use | URL |
|---|---|
| HTTPS JSON-RPC (read-only `Safe` methods) | `https://forge-rpc.orogen.network` |
| HTTPS + WSS RPC (Polkadot.js / subxt / explorer-web) | `https://chain.orogen.network` / `wss://chain.orogen.network` |
| libp2p (IPv6 only) | `/ip6/2a01:240:ad00:2502:3:a68c:1ab2:1861/tcp/30333/p2p/12D3KooWQdR4TD9JEDkim5nKsETDhS3guQVR1U5s1S6JMefVMSn8` |

Use the libp2p multiaddr as `--bootnodes` when running a peer that should
sync from the foundation seed.

## Chain identity

| Field | Value |
|---|---|
| `name` | `Orogen Forge Testnet` |
| `id` | `orogen_forge` |
| `chainType` | `Live` |
| `protocolId` | `orogenforge` |
| `tokenSymbol` | `OROG` |
| `tokenDecimals` | `12` |
| `ss58Format` | `42` |
| Genesis hash | `0x78f3de354670b9080a9d1c92cfe0413765a7a42f073bd7dfa51b4d5219cd003d` |

## Authority + sudo public keys (genesis)

| Role | ss58 | Scheme |
|---|---|---|
| Aura authority | `5Fsykze6UqiXB9VrLr92icHG64AQckwRLuNZWAdXkuU92y6n` | sr25519 |
| GRANDPA authority | `5DSGzY62YjTmcgspz3wzZWFdG3SpuDc9pgzYgMhp1DeBCVBv` | ed25519 |
| Sudo | `5FeYeFTh4xa6n85zq1Aswn2vKbuRW76HvVcXPfi2pdsFBGLR` | sr25519 |

The validator account (Aura authority above) is also endowed with `1 << 60`
plancks at genesis for fee headroom.

The corresponding private keys live (a) on the seed node in
`/var/lib/orogen-chain-node/chains/orogen_forge/keystore/` and
`.../network/secret_ed25519`, owned by `orogen:orogen` mode `0600`, and
(b) in foundation custody outside this repo. They are not committed.

## Reproducing the chain spec

Build the node with the `dev-runtime` feature (required to expose the
`forge` chain-spec id), then regenerate the raw JSON spec:

```sh
cd chain-node
cargo build --release --features dev-runtime
./target/release/chain-node build-spec --chain forge --raw > orogen-forge.raw.json
```

The raw JSON is byte-deterministic given the same runtime WASM, so two
operators on the same `pallet-suite` SHA will produce identical genesis
hashes.

## Running a non-validator full node against forge

```sh
./chain-node \
  --chain orogen-forge.raw.json \
  --base-path ~/.local/share/orogen-forge \
  --name "my-node" \
  --bootnodes /ip6/2a01:240:ad00:2502:3:a68c:1ab2:1861/tcp/30333/p2p/12D3KooWQdR4TD9JEDkim5nKsETDhS3guQVR1U5s1S6JMefVMSn8 \
  --rpc-port 9944 \
  --port 30333
```

IPv6 connectivity is required for libp2p — the seed node has no public IPv4.
