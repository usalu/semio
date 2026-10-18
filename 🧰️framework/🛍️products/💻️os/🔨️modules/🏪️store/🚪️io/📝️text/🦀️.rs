//! 📝️ The document text format: parsing and printing `.dsl`/`.ops` text and `.spr` history into envelopes, edit lines and
//! the mirror files the codecs and the folder backbones write.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, UndoPolicy};
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint, Identified, Patchable};
use semio_framework_os_store_space_schema::{ArtifactCursor, ArtifactEnvelope, ArtifactEnvelopeOwners, HistoryLane};
use semio_framework_os_store_envelope::owner::OwnerRef;
use crate::{ArtifactPackFiles, encode_document_pack_bytes};
use semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost;

use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_command::{OpsAuthor, OpsHeaderLine, OpsPin, OpsSupersededInput, ops_line_provenance};
use semio_framework_os_store_ledger::{ArtifactEditMessageLedger, fold_envelope_history};
use semio_framework_os_store_ledger::{retire_replayed_projection, retire_scratch_operations, stamp_edit_semantics, validate_composition_pins};
use semio_framework_os_store_canonical_json::{ArtifactCanonicalJson, ArtifactCanonicalJsonTree};
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, ReplayEdits, ReplayMode, admit_replacement};
use semio_framework_os_store_replay::{FoldProjection, retire_shared_projection};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_os_store_backbone::local_storage_backbone_key;

#[path = "🗒️text-files-parsed-document-and-ops-log/🦀️.rs"]
mod text_files_parsed_document_and_ops_log;
pub use text_files_parsed_document_and_ops_log::{
    adopt_loaded_replay, common_address, fill_supersession_targets, loaded_history_replay,
    mutation_ids, operation_ids, ops_line_from_transition, parse_clock, print_clock,
    print_document_text, print_edit_lines, print_ops_log, project_envelope_history,
    replay_loaded_history, supersession_address, take_loaded_replay, transition_from_ops_line,
    ArtifactTextFiles, ParsedDocumentText,
};

#[path = "🛂️history-messages-and-persisted-validation/🦀️.rs"]
mod history_messages_and_persisted_validation;
pub use history_messages_and_persisted_validation::{
    canonical_conflict_actors, conflict_from_history_conflict, conflict_messages_for_edits,
    expected_mutation_message_level, history_conflict_from_conflict, history_edit_from_edit,
    history_message_from_mutation_message, history_op_meta_from_operation_meta, history_op_payloads,
    mutation_message_from_history_message, mutation_meta_from_history_op_meta,
    protocol_undo_policy_from_ordinal, protocol_undo_policy_ordinal, stable_mutation_ids_for_edit,
    validate_durable_history, validate_history_lanes, validate_persisted_conflicts,
    validate_persisted_message,
};

#[path = "📦️document-spr-pack-and-history-composition/🦀️.rs"]
mod document_spr_pack_and_history_composition;
pub use document_spr_pack_and_history_composition::{
    append_history_events_to_spr, apply_history_composition, empty_document_spr,
    genesis_member_envelope_pack, history_composition_from_envelope, parse_decoded_document_spr,
    parse_document_spr, print_document_pack, print_document_spr, stamp_document_spr_identity,
};

#[path = "🔁️replay-ops-and-document-parsing/🦀️.rs"]
mod replay_ops_and_document_parsing;
pub use replay_ops_and_document_parsing::{
    discard_parsed_envelope, parse_document_pack, parse_document_text, replay_ops,
    settle_parsed_envelope,
};
