import { toolJobArtifactRetainedCommandExact, toolJobArtifactRetainedCommandRuntimeLawExact } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes tool job artifact retained command policy assertions. */
export function toolJobArtifactRetainedCommandSelfTests(source: string, runtimeLawSource: string): number {
  if (!toolJobArtifactRetainedCommandExact(source)) throw new Error("[verify interactivity tool-jobs] live shared retained command checkpoint was falsely rejected.");
  if (!toolJobArtifactRetainedCommandRuntimeLawExact(runtimeLawSource)) throw new Error("[verify interactivity tool-jobs] live shared retained command runtime replay law was falsely rejected.");
  const mutations: readonly [string, string, string][] = [
    ["resizable work checkpoint", "let mut work_state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES - ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES];", "let mut work_state = Vec::new();"],
    ["resume constructor", "pub fn from_wire_with_checkpoint(", "pub fn from_wire_without_checkpoint("],
    ["checkpoint close owner", "checkpoint.begin_close();", "drop(checkpoint);"],
    ["terminal checkpoint owner", "&& self.checkpoint_input.is_none()", "&& true"],
    ["work cursor restore", ".restore(checkpoint.work)?", ".restore(&[])?"],
    ["context identity restore", "context_digest != current_context_digest", "false"],
    ["workspace identity restore", "workspace_identity != current_workspace_identity", "false"],
    ["checkpoint version", "bytes[4] != 3", "bytes[4] != 2"],
    ["checkpoint maximum", "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;", "pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = usize::MAX;"],
  ];
  for (const [name, from, to] of mutations) {
    if (!source.includes(from)) throw new Error(`[verify interactivity tool-jobs] shared retained checkpoint mutation source missing: ${name}.`);
    if (toolJobArtifactRetainedCommandExact(source.replace(from, to))) throw new Error(`[verify interactivity tool-jobs] shared retained checkpoint mutation ${name} was falsely accepted.`);
  }
  const runtimeMutations: readonly [string, string, string][] = [
    ["runtime replay cursor", "TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2", "TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 3"],
    ["runtime cancellation", "semio_framework_job::StepOutcome::Cancelled", "semio_framework_job::StepOutcome::Yield"],
  ];
  for (const [name, from, to] of runtimeMutations) {
    if (!runtimeLawSource.includes(from)) throw new Error(`[verify interactivity tool-jobs] shared retained runtime mutation source missing: ${name}.`);
    if (toolJobArtifactRetainedCommandRuntimeLawExact(runtimeLawSource.replaceAll(from, to))) throw new Error(`[verify interactivity tool-jobs] shared retained runtime mutation ${name} was falsely accepted.`);
  }
  return mutations.length + runtimeMutations.length + 2;
}
