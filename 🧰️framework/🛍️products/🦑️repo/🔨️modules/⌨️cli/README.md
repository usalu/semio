# ⌨️ Command line

The repo command surface: the command framework and its verb tree, the engine and its event stream,
the NDJSON / human / markdown renderers, the usage text, and the `mcp` verb that serves the protocol.
The Rust crate produces the `semio` binary every dev entry point resolves by default; the Go module
produces the `semio-repo` twin selected by `SEMIO_REPO_IMPLEMENTATION=go`.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-cli` (binaries `semio`, `repo`)
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/cli`, entry point in `📦️packages/🐹️go/🚀️bin`
- `📦️packages/🟦️typescript` — the binary-resolution and invocation helpers

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the third-party references are registered in `🔮️oracles/🔣️.json`.

`🧭️command-parsing`, `🖨️render-formats`, `📖️usage-text`, `🔁️graphql-verb-roundtrip`,
`🔌️mcp-verb-handshake`, `🧪️test-verb-planning`, `📤️export-verb-records`, `🪝️hook-verb-dispatch`.
