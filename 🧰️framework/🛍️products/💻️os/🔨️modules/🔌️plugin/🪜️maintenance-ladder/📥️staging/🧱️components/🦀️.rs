//! 🪷️ The instance components of `instance-ladders`: MaintenanceCursors, CloseCursors (WP6, plan section 7.3).
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::{ArtifactDisposal, ArtifactDocumentStoreDisposer, ArtifactOwnedDisposer};
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, ConfigStore, MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_child::ChildMemberRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use std::mem::ManuallyDrop;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

/// 🪻️ Maintenance cursors: the rotation positions of the live-maintenance ladder.
pub struct MaintenanceCursors {
    pub maintenance_window_retirement_cursor: usize,
    pub maintenance_stage: u8,
    pub maintenance_cancellation_cursor: usize,
    pub maintenance_tool_cursor: usize,
    pub maintenance_media_cursor: usize,
    pub maintenance_segment_cursor: usize,
    pub maintenance_snapshot_cursor: usize,
    pub maintenance_child_root_cursor: usize,
    pub maintenance_peer_presence_cursor: usize,
    pub maintenance_presence_peer_cursor: usize,
    pub maintenance_peer_roster_cursor: usize,
    pub maintenance_envelope_ingress_cursor: usize,
    pub maintenance_envelope_decode_cursor: usize,
    pub maintenance_store_replacement_cursor: usize,
    pub maintenance_document_archive_cursor: usize,
    pub maintenance_envelope_field_decoder_cursor: usize,
    pub maintenance_envelope_completed_record_cursor: usize,
}

impl MaintenanceCursors {
    /// 🪼️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            maintenance_window_retirement_cursor: 0,
            maintenance_stage: 0,
            maintenance_cancellation_cursor: 0,
            maintenance_tool_cursor: 0,
            maintenance_media_cursor: 0,
            maintenance_segment_cursor: 0,
            maintenance_snapshot_cursor: 0,
            maintenance_child_root_cursor: 0,
            maintenance_peer_presence_cursor: 0,
            maintenance_presence_peer_cursor: 0,
            maintenance_peer_roster_cursor: 0,
            maintenance_envelope_ingress_cursor: 0,
            maintenance_envelope_decode_cursor: 0,
            maintenance_store_replacement_cursor: 0,
            maintenance_document_archive_cursor: 0,
            maintenance_envelope_field_decoder_cursor: 0,
            maintenance_envelope_completed_record_cursor: 0,
        }
    }
}

/// 🫎️ Close cursors: the positions and drained flags of the close ladder.
pub struct CloseCursors<A: ArtifactApp, M: SpaceMember + MemberFactory> {
    pub close_window_retirement_cursor: usize,
    pub close_started: bool,
    pub close_cancellation_cursor: usize,
    pub close_typed_operation_cursor: usize,
    pub close_media_cursor: usize,
    pub close_media_cleanup_cursor: usize,
    pub close_segment_cursor: usize,
    pub close_segment_cleanup_cursor: usize,
    pub close_snapshot_cursor: usize,
    pub close_instance_operation_owner_drained: bool,
    pub close_child_root_cursor: usize,
    pub close_child_root_detached: bool,
    pub close_child_member_cursor: usize,
    pub close_child_member: ManuallyDrop<Option<ChildMemberRetirement<M>>>,
    pub close_peer_presence_cursor: usize,
    pub close_peer_presence_detached: bool,
    pub close_presence_peer_cursor: usize,
    pub close_peer_roster_cursor: usize,
    pub close_envelope_ingress_cursor: usize,
    pub close_envelope_ingress_drained: bool,
    pub close_envelope_decode_cursor: usize,
    pub close_envelope_decode_stage: u8,
    pub close_envelope_decode_jobs_drained: bool,
    pub close_store_replacement_cursor: usize,
    pub close_store_replacement_jobs_drained: bool,
    pub close_document_archive_cursor: usize,
    pub close_document_archive_loads_drained: bool,
    pub close_envelope_field_decoder_cursor: usize,
    pub close_envelope_field_decoders_drained: bool,
    pub close_envelope_completed_record_cursor: usize,
    pub close_envelope_completed_records_drained: bool,
    pub close_snapshot_reads_drained: bool,
    pub close_owned_stage: u8,
    pub close_owned_advance_ready: bool,
    pub close_document_disposer: ManuallyDrop<ArtifactDisposal<ArtifactStore<A::Snapshot, A::Mutation>>>,
    pub close_config_disposer: ManuallyDrop<ArtifactDisposal<ConfigStore<A::Config, A::ConfigMutation>>>,
    pub close_draft_disposer: ManuallyDrop<ArtifactDisposal<store::DraftStore<A::Draft, A::DraftMutation>>>,
    pub close_presence_disposer: ManuallyDrop<ArtifactDisposal<store::PresenceStore<A::Presence, A::PresenceMutation>>>,
    pub close_transient_disposer: ManuallyDrop<ArtifactDisposal<store::TransientStore<A::Transient, A::TransientMutation>>>,
    pub close_interaction_disposer: ManuallyDrop<ArtifactDisposal<ConfigStore<protocol::InteractionState, InteractionConfigMutation>>>,
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> CloseCursors<A, M> {
    /// 🫏️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            close_window_retirement_cursor: 0,
            close_started: false,
            close_cancellation_cursor: 0,
            close_typed_operation_cursor: 0,
            close_media_cursor: 0,
            close_media_cleanup_cursor: 0,
            close_segment_cursor: 0,
            close_segment_cleanup_cursor: 0,
            close_snapshot_cursor: 0,
            close_instance_operation_owner_drained: false,
            close_child_root_cursor: 0,
            close_child_root_detached: false,
            close_child_member_cursor: 0,
            close_child_member: ManuallyDrop::new(None),
            close_peer_presence_cursor: 0,
            close_peer_presence_detached: false,
            close_presence_peer_cursor: 0,
            close_peer_roster_cursor: 0,
            close_envelope_ingress_cursor: 0,
            close_envelope_ingress_drained: false,
            close_envelope_decode_cursor: 0,
            close_envelope_decode_stage: 0,
            close_envelope_decode_jobs_drained: false,
            close_store_replacement_cursor: 0,
            close_store_replacement_jobs_drained: false,
            close_document_archive_cursor: 0,
            close_document_archive_loads_drained: false,
            close_envelope_field_decoder_cursor: 0,
            close_envelope_field_decoders_drained: false,
            close_envelope_completed_record_cursor: 0,
            close_envelope_completed_records_drained: false,
            close_snapshot_reads_drained: false,
            close_owned_stage: 0,
            close_owned_advance_ready: false,
            close_document_disposer: ManuallyDrop::new(A::build_document_store_disposer()),
            close_config_disposer: ManuallyDrop::new(A::build_config_store_disposer()),
            close_draft_disposer: ManuallyDrop::new(A::build_draft_store_disposer()),
            close_presence_disposer: ManuallyDrop::new(A::build_presence_store_disposer()),
            close_transient_disposer: ManuallyDrop::new(A::build_transient_store_disposer()),
            close_interaction_disposer: ManuallyDrop::new(Some(Box::new(ArtifactDocumentStoreDisposer::<protocol::InteractionState, InteractionConfigMutation>::new()))),
        }
    }
}
