# 🔗️ GraphQL

The owned GraphQL surface: the document parser and its syntax errors, the variable coercion, the
executor for queries and mutations, the execution error shape, the SDL dump and the `RepoContext` port
the resolvers read the repository through.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-graphql`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/graphql`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the `graphql` reference implementation is registered in `🔮️oracle/🔣️.json`.

`📃️document-parsing`, `🚫️syntax-errors`, `🙅️unsupported-syntax`, `🔀️variable-coercion`,
`▶️query-execution`, `✏️mutation-execution`, `❌️execution-errors`, `📜️sdl-dump`.
