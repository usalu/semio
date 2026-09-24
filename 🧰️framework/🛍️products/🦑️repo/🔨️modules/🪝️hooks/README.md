# 🪝️ Hooks

The agent hook surface: the native events each IDE emits, their normalisation into one shape, the
tool-blocking policy, the plan and spec source resolution, the session log and the hook result
formatting every editor reads back.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-hooks`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/hooks`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the third-party references are registered in `🔮️oracles/🔣️.json`.

`🔀️native-event-normalisation`, `🛡️tool-blocking-policy`, `🗺️plan-step-extraction`,
`🖨️hook-result-formatting`, `📓️session-logging`.
