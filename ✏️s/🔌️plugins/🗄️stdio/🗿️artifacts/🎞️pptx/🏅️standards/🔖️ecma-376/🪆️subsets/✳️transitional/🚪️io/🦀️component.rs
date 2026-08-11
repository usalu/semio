//! 🚪️ IO stdio.pptx (ecma-376/✳️transitional) — reuses the ✳️any subset's `zip`/`xml` raw-codec
//! DAG leaves rather than duplicating them (same `PptxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️any/🚪️io` already established for this artifact.
