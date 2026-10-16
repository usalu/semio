//! 📥️ Trait impls of `♻️retirement` whose trait or type belongs to `os-store-ledger` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/♻️retirement/🦀️.rs`.
use semio_framework_value::artifact_retire_struct;

artifact_retire_struct!(crate::CursorRevisionRecord { ledger_key, id_digest, edit_digest, prefix_digest });

artifact_retire_struct!(crate::EditDigestChains { forwards, inverse, meta, forwards_digest, inverse_digest, meta_digest });

artifact_retire_struct!(crate::CursorRevisionAccumulator { identity_digest, applied, redo, applied_tail_chains, mutation_positions, indexed_edits, unit_flags });
