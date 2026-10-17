//! 🧱️ Directory native binary artifact admission: explicit hash text, descriptor digest and the canonical checkpoint pair (value and framing codec).

#[path = "🪢️checkpoint-pair/🦀️.rs"]
pub mod checkpoint_pair;

#[path = "🔐️descriptor-digest/🦀️.rs"]
pub mod descriptor_digest;

#[path = "🪪️artifact-hash/🦀️.rs"]
pub mod artifact_hash;

#[path = "../../🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs"]
pub mod canonical_checkpoint_pair;
pub use canonical_checkpoint_pair::{AdmittedCheckpointSelectionV1, CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1};
