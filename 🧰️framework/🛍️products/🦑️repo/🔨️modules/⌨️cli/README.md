# ⌨️ Command line

The repo command surface: the command framework and its verb tree, the engine and its event stream,
the NDJSON / human / markdown renderers, the usage text, and the `mcp` verb that serves the protocol.
The Rust crate produces the `semio-repo` binary every dev entry point resolves by default and the
`repo` MCP stdio server; the Go module produces the `semio-repo` twin selected by
`SEMIO_REPO_IMPLEMENTATION=go`. Neither binary links a terminal user interface: the `semio`
dashboard binary belongs to `🎛️dashboard`, which may depend on the repo domain crates but is never
a dependency of this crate.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-cli` (binaries `semio-repo`, `repo`): the declared command tree, the process entry and the projections, over the producer crates below
- `📡️event-stream/📦️packages/🦀️rust` — `semio-framework-repo-cli-event-stream`
- `🧭️repo-cli/🎮️command-framework/📦️packages/🦀️rust` — `semio-framework-repo-cli-command-framework`
- `🧭️repo-cli/🗂️tree-source-and-search/📦️packages/🦀️rust` — `semio-framework-repo-cli-tree-source-and-search`
- `🔌️mcp-verb/📦️packages/🦀️rust` — `semio-framework-repo-cli-mcp-verb`
- `🧭️repo-cli/🚦️verbs/📦️packages/🦀️rust` — `semio-framework-repo-cli-verbs`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/cli`, entry point in `📦️packages/🐹️go/🚀️bin`
- `📦️packages/🟦️typescript` — the binary-resolution and invocation helpers

## 🧱️ Rust layout

The lib root mounts `🧭️repo-cli` (the `repo_cli` facade over `🌳️command-tree` and `🚀️run-and-projection`) and re-exports the `mcp_verb` crate. Five producer crates sit below it, each with its lib root at the node and its package under `📦️packages/🦀️rust`: `📡️event-stream` (events, Go-twin formatting and the renderers, a leaf every other crate reads), `🎮️command-framework` (typed flags, the argv parser, the invocation; reads the event stream), `🗂️tree-source-and-search` (the filesystem tree source, model ids and stream options; reads the framework), `🔌️mcp-verb` (the production repository of the `repo` MCP server; reads the event stream) and `🚦️verbs` (one handler per verb; reads all four). Dependencies point downward only, so the `mcp` verb and the MCP repository share no cycle and the declared command tree is the only crate that names every verb.

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the third-party references are registered in `🔮️oracles/🔣️.json`.

`🧭️command-parsing`, `🖨️render-formats`, `📖️usage-text`, `🔁️graphql-verb-roundtrip`,
`🔌️mcp-verb-handshake`, `🧪️test-verb-planning`, `📤️export-verb-records`, `🪝️hook-verb-dispatch`.
