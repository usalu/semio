//! 🧾️ Document envelope materialization: genesis-to-envelope construction and the materialized views the codecs and
//! the engine build documents from.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "📜️history/💧️hydration/🦀️.rs"]
pub mod persisted_document_hydration;
#[path = "📥️mounted-pack/🦀️.rs"]
pub mod mounted_pack_session;
use semio_framework_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};
use semio_framework_os_spr::{HybridLogicalTimestamp, MutationId};
use semio_framework_os_spr::{Edit, Mutation, OpBinary};
use std::collections::HashSet;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_vcs::{ArtifactHistoryLedger, ArtifactVcs, Identified};
use semio_framework_os_store_space_schema::{ArtifactBackboneRef, ArtifactCursor, ArtifactCursorOwners, ArtifactEnvelope, ArtifactEnvelopeOwners};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_text_io::ArtifactCodecApplyFuture;
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_ledger::{ArtifactEditMessageLedger, fold_envelope_history};
use semio_framework_os_store_ledger::ReplayProjection;
use semio_framework_os_store_replay::fold_effective_edit;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_os_store_backbone::local_storage_backbone_key;
pub use semio_framework_os_store_space_member::lifted_document_materialize::{edit_ids_for_changes, materialize_document_snapshot};
pub use semio_framework_os_store_engine_core::lifted_document_materialize::{EMPTY_CHECKPOINT_MESSAGE, now_iso, uncommitted_edit_ids};
pub use semio_framework_os_store_initialization_runtime::lifted_document_materialize::{fresh_replica_clock, now_ms};
pub use semio_framework_os_store_engine_live::lifted_document_materialize::{stamp_primary_operation_identity};

// config_spec_* removed — ConfigSpec is UI (framework-core); avoids kernel↔core cycle
pub fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>, producer:&impl ArtifactGenesisProducer<P>) -> ArtifactEnvelope<P, Mutation>
where P: Clone,
{
    create_document_envelope_from_genesis(schema, id, AdmittedArtifactGenesis::born(initial_snapshot,producer).expect("artifact genesis birth encodes once"), backbone)
}

pub fn create_document_envelope_from_genesis<P, Mutation>(schema: &str, id: &str, genesis: AdmittedArtifactGenesis<P>, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where P: Clone,
{
    create_document_envelope_from_genesis_owners(schema.into(), id.into(), genesis, backbone)
}

/// 📥️ Moves exact original metadata into the document constructor while preserving its native catalogs.
pub fn create_document_envelope_from_genesis_owners<P, Mutation>(schema: String, id: String, genesis: AdmittedArtifactGenesis<P>, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where P: Clone,
{
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema,
        id,
        vcs: ArtifactVcs { genesis, edits: ArtifactHistoryLedger::new(), changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone,
        active_alternative_id: None,
        cursor: Some(ArtifactCursor::default()),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::HistoryFoldIndex::new(),
        edit_messages: ArtifactEditMessageLedger::new(),
        conflicts: Vec::new(),
        transitions: Vec::new(),
        history_shape: semio_framework_os_spr::HistoryShape::Document,
        open_transaction: None,
        viewer_checkpoint_id: None,
    })
}

/// 🪶 Moves exact original Genesis and metadata into an allocation-free document catalog.
pub fn empty_document_envelope_from_genesis_owners<P, Mutation>(schema: String, id: String, genesis: AdmittedArtifactGenesis<P>, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation> {
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema, id,
        vcs: ArtifactVcs { genesis, edits: ArtifactHistoryLedger::empty(), changes: ArtifactHistoryLedger::empty(), checkpoints: ArtifactHistoryLedger::empty(), alternatives: ArtifactHistoryLedger::empty() },
        backbone, active_alternative_id: None,
        cursor: Some(ArtifactCursor::from_owners(ArtifactCursorOwners { applied_edit_ids: semio_framework_os_vcs::HistoryPageStack::empty(), redo_edit_ids: semio_framework_os_vcs::HistoryPageStack::empty(), checkpoint_id: None })),
        dialect: None, migrated_from: None, owner: None, lanes: protocol::HistoryFoldIndex::new(),
        edit_messages: ArtifactEditMessageLedger::empty(), conflicts: Vec::new(), transitions: Vec::new(), history_shape: semio_framework_os_spr::HistoryShape::Document, open_transaction: None, viewer_checkpoint_id: None,
    })
}
















