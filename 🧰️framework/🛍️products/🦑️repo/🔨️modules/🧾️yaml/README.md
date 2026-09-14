# 🧾️ YAML

The hand-rolled YAML codec: a decoder for the subset the repository reads, and an encoder that emits
one deterministic, key-sorted rendering chosen for diffability.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-yaml`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/yaml`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the `yaml` package is registered as the third-party reference for decoding in
`🔮️oracle/🔣️.json`, and the owned encoder is held by a metamorphic law recorded there.

`🔁️codec-roundtrip`, `🕳️empty-container-encoding`.
