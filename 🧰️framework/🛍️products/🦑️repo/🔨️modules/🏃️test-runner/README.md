# 🏃️ Test runner

Detection and execution of the test runner a scope actually needs — go, bun, uv-pytest, cargo, dotnet,
rspec — the invocation plan it builds, the result parsing, the scope identifier normalisation and the
cancellation every long run owes its caller.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-test-runner`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/testrunner`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the third-party references are registered in `🔮️oracle/🔣️.json`.

`🧭️runner-detection`, `🗺️invocation-planning`, `📊️result-parsing`,
`🧬️scope-identifier-normalisation`, `🛑️cancellation`.
