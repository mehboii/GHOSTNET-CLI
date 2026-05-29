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
 Welcome, User !
```

## Install (npm)

```bash
npm install -g @n11x/ghostnet-cli
```

This downloads the prebuilt binary for your platform (Windows / macOS / Linux,
x64 + arm64) from GitHub Releases and exposes the `ghostnet` command. Node.js 18+
must be on your `PATH` at runtime (the CLI drives the SDK through it).

## Requirements (building from source)

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
#    (prefer GHOSTNET_SEED over --seed to keep the phrase out of shell history)
GHOSTNET_SEED="your twelve words ..." ghostnet send 0x<peer-node-id> "hello from the mesh!"

# 5. Listen for incoming messages
GHOSTNET_SEED="your twelve words ..." ghostnet listen
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

| Variable         | Purpose                                                        |
| ---------------- | -------------------------------------------------------------- |
| `GHOSTNET_SEED`  | Seed phrase, kept out of argv/shell history (preferred)        |
| `GHOSTNET_NODE`  | Path to the `node` executable                                  |
| `GHOSTNET_NPM`   | Path to the `npm` executable                                   |

## Security

- **Verified binaries.** The npm postinstall downloads the platform binary over
  HTTPS (GitHub hosts only) and verifies its **SHA-256 against `checksums.json`
  shipped inside the npm package** before installing. A tampered release, CDN, or
  man-in-the-middle is rejected — the binary is never executed. Downloads are
  atomic (temp file + rename), size-capped, and follow a bounded number of
  redirects.
- **Secrets stay off the command line.** Seed phrases are passed to the Node
  bridge via the environment, never as process arguments, so they don't leak
  through `ps` / `/proc/<pid>/cmdline` / Task Manager. Use `GHOSTNET_SEED` to also
  keep them out of shell history.
- **Pinned CI.** The release workflow pins every GitHub Action to a commit SHA,
  runs least-privilege (`contents: read`, write only on the publish job), and
  does not persist credentials in the checkout.

## License

MIT — N11X Collective
