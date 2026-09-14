# 🧩️ Providers

Everything outside the repository the repo talks to, each behind an interface: management (GitHub
through `gh`), version control (git), sandboxing (devcontainer), and the editors — Copilot, Cursor,
Windsurf, Claude, Droid, Codex, Antigravity, Kiro. Process execution never happens in a caller.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-providers`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/providers`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; `git` itself is registered as the third-party reference for version control in
`🔮️oracle/🔣️.json`. The `🌿️git-version-control` scenarios are `@level-long` because they spawn a
real version control system, so they run under `parity long`, never `fundamental` or `quick`.

`🪪️mcp-client-kind-parse`, `🪝️editor-hook-output-format`, `🐙️github-management-transcripts`,
`🌿️git-version-control`.
