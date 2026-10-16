//! 📥️ Trait impls of `🏪️store` whose trait or type belongs to `os-store-read` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🦀️.rs`.
use crate::ArtifactEnvelopeMetadata;

semio_framework_value::artifact_retire_struct!(ArtifactEnvelopeMetadata { schema, id, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, conflicts, transitions, open_transaction });
