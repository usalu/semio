# 📊️ Metrics

What the repository measures about itself: the tracked-file LOC scan by technology, markup and data
bucket, the `git log --numstat` history walk that turns commits into per-contributor line deltas, the
time bucketing that folds those deltas into a series, and the benchmark summary every ecosystem
reports into.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-metrics`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/metrics`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the real `git` command line is registered as the normative producer of the numstat
stream in `🔮️oracle/🔣️.json`, the TypeScript library's unified LOC counter is registered as a
cross-implementation supplement, and the aggregation, bucketing and benchmark shapes rest on recorded
no-oracle decisions.

`🔢️numstat-parsing`, `🧮️loc-aggregation`, `⏳️time-bucketing`, `📈️benchmark-summary`.
