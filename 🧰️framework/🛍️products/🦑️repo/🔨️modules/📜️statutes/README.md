# 📜️ Statutes

The law this repository holds itself to: the statute catalog, the policy and territory tree, the
breach identifier grammar, the analysis that finds breaches, the autofix that repairs them, the
`compose-ignore-` directives that waive them, and the gzip + SHA-256 breach cache envelope.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-statutes`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/statutes`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; Node `zlib` and `crypto` are registered as the standards reference for the cache
envelope in `🔮️oracles/🔣️.json`, and the owned law itself is a recorded no-oracle decision.

`📚️statute-catalog`, `🔍️analyze-breaches`, `🩹️autofix-roundtrip`, `🙈️ignore-directives`,
`🗜️breach-cache-envelope`.
