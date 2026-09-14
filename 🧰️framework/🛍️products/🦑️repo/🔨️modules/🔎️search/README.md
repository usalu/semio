# 🔎️ Search

The owned content search: a conjunction over terms, a bounded edit distance per term, ties broken by
identifier, and a total counted before truncation.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-search`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/search`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; no third-party engine implements this ranking function, so `🔮️oracle/🔣️.json`
registers a TypeScript reference implementation alongside the recorded no-oracle decision.

`🔍️ranked-search`.
