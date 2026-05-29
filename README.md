# GhostNet CLI

> encrypted · decentralized · private — by the **N11X Collective**

A native Rust command-line client for the **GhostNet** encrypted mesh network. It
drives the official [`@n11x/ghostnet-sdk`](https://www.npmjs.com/package/@n11x/ghostnet-sdk)
through a small bundled Node bridge, so every cryptographic operation uses the
same audited SDK that powers GhostNet apps.

```
╔═══════════════════════════════════════════════╗
║        N 1 1 X   C O L L E C T I V E           ║
║         · G H O S T N E T   C L I ·            ║
╚═══════════════════════════════════════════════╝
👻 Welcome, <you>!
```

## Requirements

- [Rust](https://rustup.rs) (1.74+) to build the binary
- [Node.js](https://nodejs.org) 18+ on your `PATH` at runtime (the CLI shells out
  to it to drive the SDK)

## Build & install

```bash
# from this directory
cargo build --release
# the binary lands at target/release/ghostnet (ghostnet.exe on Windows)

# optional: install it onto your PATH
cargo install --path .
```

## First run

```bash
# 1. Pull in the GhostNet SDK (runs: npm install @n11x/ghostnet-sdk)
ghostnet setup

# 2. Create an identity (back up the seed phrase!)
ghostnet identity create

# 3. Restore an identity later
ghostnet identity load "word1 word2 ... word12"

# 4. Send an encrypted message to a peer
ghostnet send 0x<peer-node-id> "hello from the mesh!" --seed "your twelve words ..."

# 5. Listen for incoming messages
ghostnet listen --seed "your twelve words ..."
```

`ghostnet info` shows the CLI version, SDK package, and whether the bridge is
installed. Add `--no-color` to any command for plain output.

## How it works

```
 ghostnet (Rust binary)
        │  spawns `node`
        ▼
 ~/.ghostnet-cli/bridge/ghostnet-bridge.mjs
        │  imports
        ▼
 @n11x/ghostnet-sdk   ──►  GhostNet mesh relay (wss://)
```

On `setup`, the CLI writes its embedded bridge (`ghostnet-bridge.mjs` +
`package.json`) into `~/.ghostnet-cli/bridge` and runs `npm install` there. Each
command then spawns Node against that bridge and exchanges newline-delimited JSON.

### Environment overrides

| Variable         | Purpose                                  |
| ---------------- | ---------------------------------------- |
| `GHOSTNET_NODE`  | Path to the `node` executable            |
| `GHOSTNET_NPM`   | Path to the `npm` executable             |

## License

MIT — N11X Collective
