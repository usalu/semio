//! 🚪️ IO stdio.step (ap214/✳️cc6) — reuses the ✳️any subset's import/export DAG leaves (same
//! `StepSnapshot` type, same catalog DAG edges) rather than duplicating them. Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` established.
