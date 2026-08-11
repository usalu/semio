//! 🚪️ IO stdio.tiff (6.0/✳️baseline) — reuses the ✳️any subset's `binary` raw-codec DAG leaf
//! rather than duplicating it (same `TiffSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
