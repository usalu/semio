# 🗣️ Languages

One `LanguagePlugin` per language — TypeScript, Go, C#, JSON, Markdown, Rust, Ruby, Shell, TOML, YAML,
SQL, GraphQL — with the region markers, comment formats, headers, section and definition parsing each
one implies. The language table is authored once in `🧬️schema/🔣️.json` and read by both
implementations, so neither owns a private copy.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-languages`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/languages`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the third-party references are registered in `🔮️oracles/🔣️.json`.

`📑️section-parsing`, `📖️definition-parsing`, `🧾️header-roundtrip`, `🏷️scope-ids`,
`💥️malformed-regions`.
