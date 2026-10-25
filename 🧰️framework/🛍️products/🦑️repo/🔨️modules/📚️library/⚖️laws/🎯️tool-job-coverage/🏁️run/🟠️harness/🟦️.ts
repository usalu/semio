/** 🟠️ The shared declarations of toolJobCoverageRun. */
import { toolJobRustBlock } from "../../📋️inventory/🟦️.ts";

// 🧬️ Faithful P6i structural mutations over exact live helpers, laws, and mounted callers.


export function toolJobArtifactRetainedCommandExact(source: string): boolean {
  const checkpointStart = source.indexOf("    fn checkpoint(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {");
  const checkpointOpen = checkpointStart < 0 ? -1 : source.indexOf("{", checkpointStart);
  const checkpoint = checkpointOpen < 0 ? undefined : toolJobRustBlock(source, checkpointOpen);
  const required = [
    "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;",
    "const ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES: usize = 48;",
    'const ARTIFACT_COMMAND_CHECKPOINT_MAGIC: [u8; 4] = *b"ARC1";',
    "fn encode_artifact_command_checkpoint(",
    "fn decode_artifact_command_checkpoint(",
    "bytes[4] != 3",
    "context_digest != current_context_digest",
    "workspace_identity != current_workspace_identity",
    "fn workspace_identity(&self) -> u64",
    "fn checkpoint(&self, _target: &mut [u8]) -> Result<usize, Fault>",
    "fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault>",
    "CheckpointPages,",
    "CheckpointRetire,",
    "WirePages,",
    "Decode,",
    "Preflight,",
    "Work,",
    "Publish,",
    "checkpoint_input: Option<RetainedToolWireInput>",
    "checkpoint_bytes: [u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES]",
    "checkpoint_pending: bool",
    "context: Option<Arc<ArtifactOwnedToolJobContext<A>>>",
    "pub fn from_wire_with_checkpoint(",
    "pub fn try_new_with_context(",
    "ArtifactRetainedCommandPhase::CheckpointPages =>",
    "ArtifactRetainedCommandPhase::CheckpointRetire =>",
    "checked_add(page.len()).filter(|end| *end <= ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES)",
    "decode_artifact_command_checkpoint(bytes, self.maximum_raw_bytes, input.page_count(), input.declared_bytes(), current_context_digest, current_workspace_identity)",
    "context_digest: self.context.as_ref().map_or(0, |context| context.identity_digest())",
    "workspace_identity: self.work.as_ref().map_or(0, |work| work.workspace_identity())",
    "self.work.as_mut().ok_or_else(|| Fault::from(\"retained-command-checkpoint-work-absent\"))?.restore(checkpoint.work)?",
    "self.raw.clear();",
    "self.raw_page_cursor = 0;",
    "self.checkpoint_pending = true;",
    "StepOutcome::CheckpointReady(Checkpoint",
    "checkpoint.begin_close();",
    "self.checkpoint_input.is_none()",
    "checkpoint_binary_matches_schema_fixture_and_owned_oracle",
    "checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift",
  ];
  return (
    required.every((fragment) => source.includes(fragment)) &&
    checkpoint !== undefined &&
    checkpoint.body.includes("let mut state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES]") &&
    checkpoint.body.includes("let mut work_state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES - ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES]") &&
    checkpoint.body.includes("work.checkpoint(&mut work_state)") &&
    !checkpoint.body.includes("to_vec()") &&
    !source.includes("checkpoint: Vec<u8>") &&
    !source.includes("checkpoint_bytes: Vec<u8>")
  );
}

export function toolJobArtifactRetainedCommandRuntimeLawExact(source: string): boolean {
  return [
    "shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close",
    "bus.dispatch_wire_retained_with_spec(",
    "bytes.get(4) == Some(&3) && bytes.get(5) == Some(&1) && bytes.get(48) == Some(&1)",
    "TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2",
    "semio_framework_job::StepOutcome::Cancelled",
    "original.job.terminal_is_empty()",
    "resumed.job.terminal_is_empty()",
    "cancelled.job.terminal_is_empty()",
  ].every((fragment) => source.includes(fragment));
}
