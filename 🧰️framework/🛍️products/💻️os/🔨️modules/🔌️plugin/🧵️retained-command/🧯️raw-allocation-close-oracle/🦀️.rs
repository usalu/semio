//! 🧯️ Fixture-only birth of a retained command whose raw allocation is its sole live value.

use super::*;

#[doc(hidden)]
pub fn test_raw_allocation_close<A: ArtifactApp>(raw: Vec<u8>) -> ArtifactRetainedCommandJob<A> {
    ArtifactRetainedCommandJob {

    original_command_admitted:false,
    command: None,
    command_received:false,context_received:false,
    snapshot: None,
    snapshot_read: std::cell::RefCell::new(None),
    config: None,
    history: None,
    interaction_state: None,
    interaction_hover: None,
    context: None,
    operation: None,
    completion: None,
    source_binding: None,
    source_construction:Default::default(),
    command_id: |_| "fixture",
    maximum_raw_bytes: raw.capacity(),
    maximum_work_items: 1,
    work: None,
    producer_controller_id:None,
    producer_tool_id:None,
    producer_payload_schema_id:None,
    producer_window_config:None,
    producer_instance_operation_owner:None,
    producer_output_chunks:None,
    checkpoint_input: None,
    checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    checkpoint_target: [MaybeUninit::uninit(); ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    checkpoint_capture: None,
    publication: RetainedJobPublication::new(),
    fault_publication: RetainedFaultPublication::new(),
    pending_publication: None,
    publication_delivered: false,
    fault: None,
    fault_retirement: None,
    decode_error: None,
    checkpoint_byte_len: 0,
    checkpoint_page_cursor: 0,
    checkpoint_page_offset: 0,
    raw_input: None,
    raw,
    admission_refusal:None,
    raw_page_cursor: 0,
    raw_page_offset: 0,
    emit: None,
    download:None,
    download_retirement:None,
    completion_retirement:None,
    input_retirement:None,
    ephemeral: None,
    phase: ArtifactRetainedCommandPhase::Complete,
    checkpoint_pending: false,
    work_progress: 0,
    closing: false,
    }
}

impl<A: ArtifactApp> ArtifactRetainedCommandJob<A> {
    /// 🧾️ Borrows this fixture's allocation without exposing a way to mutate it.
    pub fn raw_allocation_fixture_bytes(&self) -> &Vec<u8> {
        &self.raw
    }
}
