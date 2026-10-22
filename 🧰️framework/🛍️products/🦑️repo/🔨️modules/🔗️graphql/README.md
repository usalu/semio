# 🔗️ GraphQL

The owned GraphQL surface: the document parser and its syntax errors, the variable coercion, the
executor for queries and mutations, the execution error shape, the SDL dump and the `RepoContext` port
the resolvers read the repository through.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-graphql`
- `🏗️repo-schema/📦️packages/🦀️rust` — `semio-framework-repo-graphql-schema`: the schema type vocabulary, the repo schema builders and the SDL rendering, a dependency-free leaf the executor serves
- `🗄️fs-context/📦️packages/🦀️rust` — `semio-framework-repo-graphql-fs`: the production `FsRepoContext`, the provider-backed goal management and the coordinator emitter, the only crate here that links the repo domain crates
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/graphql`

## 🧱️ Rust layout

`semio-framework-repo-graphql` depends on `semio-framework-repo-graphql-schema`, `semio-framework-repo-identity` and `semio-framework-repo-model`. Its lib root mounts `🔤️grammar`, `🎥️context-port`, `🖼️sources`, `⚡️execution` and `🩻️recording-context`, and re-exports `Schema`, `build_schema` and `schema_inventory` from the schema crate. The schema crate depends only on `serde_json` and owns the committed-SDL unit tests; the filesystem context lives in `semio-framework-repo-graphql-fs`, which implements the `RepoContext` port.

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the `graphql` reference implementation is registered in `🔮️oracles/🔣️.json`.

`📃️document-parsing`, `🚫️syntax-errors`, `🙅️unsupported-syntax`, `🔀️variable-coercion`,
`▶️query-execution`, `✏️mutation-execution`, `❌️execution-errors`, `📜️sdl-dump`.


## GraphQL Client Types

The canonical `🔤️types` owner reads the served SDL and repository operation documents, then projects TypeScript client types. `repo run @semio-tech/repo-graphql-rs:generate-types` writes the deterministic output, `check-types` compares it without writing, and `test-types-source` runs the owned parser/projection against graphql-js and the TypeScript syntax oracle. `preview-types` emits the existing read-only generator protocol. These controls have no editor host or extension package dependency.
