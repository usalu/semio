//! 🚪️ IO stdio.pdf (1.7/✳️e) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `✳️a/🚪️io` and `✳️any/🚪️io` for this artifact. ISO 24517-1:2008 (PDF/E-1).
