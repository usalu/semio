# 🏠️ Workspace

Repository-root discovery (`.🧬semio`), the layout vocabulary every other module addresses paths
with, the `📋️config.toml` reader, and the owned glob and gitignore matchers.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-workspace`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/workspace`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; `micromatch` and `ignore` are registered as third-party references in
`🔮️oracles/🔣️.json`, and the two deliberate divergences from git are recorded there as no-oracle
decisions.

`🧭️root-discovery`, `🃏️glob-matching`, `🙈️ignore-precedence`, `🚫️posix-negation-class`,
`🐙️gitignore-divergence`.
