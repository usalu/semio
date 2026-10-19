# 🏠️ Workspace

Repository-root discovery (`.🧬semio`), the layout vocabulary every other module addresses paths
with, the `📋️config.toml` reader, and the owned glob and gitignore matchers.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-workspace`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/workspace`
- `📦️packages/🐹️go/🏖️sandbox` — `github.com/usalu/semio/repo/workspace/sandbox`, a nested Go module: the hermetic harness (sandboxed test runs, root guard, fixture repositories) every Go module's tests run under; it is imported by tests only, so no test-only code ships in the production package

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; `micromatch` and `ignore` are registered as third-party references in
`🔮️oracles/🔣️.json`, and the two deliberate divergences from git are recorded there as no-oracle
decisions.

`🧭️root-discovery`, `🃏️glob-matching`, `🙈️ignore-precedence`, `🚫️posix-negation-class`,
`🐙️gitignore-divergence`, `🛡️hermetic-root-guard` and `🏖️sandbox` (Go-only proofs of the unbound workspace and of the sandbox).
