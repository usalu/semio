//! 🚪️ IO stdio.ifc.2x3 (2x3/✳️cv20) — reuses the ✳️any subset's `binary`/`txt` raw-codec DAG
//! leaves rather than duplicating them (same `Ifc2x3Snapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()`.
