//! 🚪️ IO stdio.json (rfc8259/✳️i-json) — reuses the ✳️any subset's `txt` raw-codec DAG leaf
//! rather than duplicating it (same `JsonSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
