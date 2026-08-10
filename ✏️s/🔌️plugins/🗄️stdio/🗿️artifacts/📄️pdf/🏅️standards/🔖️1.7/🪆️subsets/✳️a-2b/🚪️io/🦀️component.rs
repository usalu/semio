//! 🚪️ IO stdio.pdf (1.7/✳️a-2b) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️any/🚪️io` already established for this artifact.
