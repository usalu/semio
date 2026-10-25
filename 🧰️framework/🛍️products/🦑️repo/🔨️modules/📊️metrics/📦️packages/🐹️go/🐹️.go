// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📊️ Package metrics is the repository's line-of-code and benchmark domain: the pure
// `git log --numstat` parser, the unified-LOC counters, the LocReport aggregation, UTC time
// bucketing and the benchmark timing model. Behaviour twin of the Rust crate
// `semio-framework-repo-metrics`.
//
// Everything here is a pure function over data; the only door to the outside world is
// [GitLogSource], which the CLI satisfies with [SystemGit] and the tests satisfy with a recorded
// [GitTranscript].
package metrics


// #endregion 🧲️Header
