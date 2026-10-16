/** 🏁️ Tool-job coverage: retained-command checkpoint laws and the census run behind `verify interactivity tool-jobs`.
 * @see 📜️script.ts
 */
import { toolJobMemberStoreProducerExact, type ToolJobMemberStoreProducerSources } from "../../../../../../💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts";
import { readMcpGatewaySource } from "../../../../../../💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { policyReadFileSafe, policyReadModuleTreeSource } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { relative } from "node:path";
import { policyAllRustFiles } from "../../🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { policyReadMemberStoreProducerSources, policyReadOsStoreSource, policyReadRustPolicySource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { toolJobArtifactEnvelopeOwnedCodecExact, toolJobArtifactStoreStructuralOwnersExact, toolJobChildContentRootExact, toolJobChildRetirementInventoryExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobImmutableOperationRootsExact, toolJobJackEnvelopeCallerRetainedExact, toolJobPagedIngressExact, toolJobPeerInteractionRootsExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact } from "../🗄️artifact-store/🟦️.ts";
import { toolJobFrameworkReservedRoutesExact, toolJobMicrosecondWorkerExact, toolJobPluginProductionSources, toolJobProofCatalogFailures } from "../🏭️factory/🟦️.ts";
import { type FixedOperationFixtureOutput, toolJobDrawingGestureOperationOwnerExact, toolJobFixedOperationFixtureRun, toolJobFixedOperationRegistryExact, toolJobFixedOperationRustFixtureSource, toolJobSharedFrameworkActionFixtureRun } from "../⚙️fixed-operation/🟦️.ts";
import { TOOL_JOB_FRAMEWORK_RESERVED_IDS, TOOL_JOB_PLUGIN_RESERVED_IDS, toolJobAppOwnedRows, type ToolJobCoverageReport, toolJobDispositions, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobProductionSource, toolJobProofIdentity, toolJobProofs, toolJobRustBlock, toolJobScanThenMonolithRows, toolJobStaticRows } from "../📋️inventory/🟦️.ts";
import { toolJobCancellationScopesBounded, toolJobDecodeAfterAdmission, toolJobDropCancellationBounded, toolJobExternalCancellationOwned, toolJobHardBoundedCloseExact, toolJobLiveInstanceIsolated, toolJobMediaExportBounded, toolJobOpaqueFutureProductionFailClosed, toolJobQualifiedProofBeforeDecode, toolJobReactorCloseBounded, toolJobReservedImporterOwners, toolJobRuntimeProofQualified, toolJobRuntimeRegistryFixedClose, toolJobTypedDispatchExact, toolJobWireDispatchExact } from "../🔒️lifecycle/🟦️.ts";
import { toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact, toolJobFemNumericalMicrocursorExact, toolJobUniversalRetainedOwnershipExact } from "../🔁️live-replay/🟦️.ts";
import { toolJobEphemeralOneItemPublicationBounded, toolJobFullOperationBounded, toolJobImportPreparationBounded, toolJobMountedDispatchOneTurnExact, toolJobPublicationFreshnessBeforeEveryTurn, toolJobStoreBatchPublicationBounded, toolJobTypedPersistentFoundation, toolJobTypedRouteFailsClosedBeforePreparation } from "../📤️publication/🟦️.ts";
import { toolJobPuzzle2dReservedRoutesExact, toolJobPuzzleReservedRoutesExact, toolJobSegmentedTerminalDrainExact } from "../🛣️routes/🟦️.ts";
import type { SelfTestLanes } from "../../🧪️self-test-lanes/🟦️.ts";

//#region 🎯️ToolJobCoverage
// 🧬️ Faithful P6i structural mutations over exact live helpers, laws, and mounted callers.


//#region 🧵️ Artifact Retained Command Checkpoint
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


//#endregion 🧵️ Artifact Retained Command Checkpoint

/** 🎯️ Phase-8 source/runtime contract census used by `verify interactivity tool-jobs`. */
export async function toolJobCoverageRun(root: string, lanes: SelfTestLanes): Promise<ToolJobCoverageReport> {
  toolJobSharedFrameworkActionFixtureRun(root);
  const macroFiles = new Set<string>();
  const fixtureMacroFiles = new Set<string>();
  const commandRows: { file: string; id: string }[] = [];
  const fixtureCommandRows: { file: string; id: string }[] = [];
  let macroInvocations = 0;
  let fixtureMacroInvocations = 0;
  const fixtureHost = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
  for (const file of policyAllRustFiles(root)) {
    const source = policyReadFileSafe(root, file);
    const invocation = /^\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*app_commands!\s*\{/gm;
    let match: RegExpExecArray | null;
    while ((match = invocation.exec(source))) {
      const open = source.indexOf("{", match.index);
      const block = toolJobRustBlock(source, open);
      if (!block) break;
      const fixture = file === fixtureHost;
      (fixture ? fixtureMacroFiles : macroFiles).add(file);
      if (fixture) fixtureMacroInvocations += 1;
      else macroInvocations += 1;
      const rows = /^\s*"([^"]+)"(?:\s+as\s+"[^"]+")?\s*=>/gm;
      let row: RegExpExecArray | null;
      while ((row = rows.exec(block.body))) (fixture ? fixtureCommandRows : commandRows).push({ file, id: row[1]! });
      invocation.lastIndex = block.end;
    }
  }
  const duplicateRows: { file: string; id: string }[] = [];
  const seenRows = new Set<string>();
  for (const row of commandRows) {
    const key = `${row.file}\0${row.id}`;
    if (seenRows.has(key)) duplicateRows.push(row);
    seenRows.add(key);
  }
  const intentionalWriterDuplicates = duplicateRows.filter((row) => row.file.includes("✏️s/🔌️plugins/✒️writer/") && row.id === "setEditorSetting");
  const productionFiles = toolJobPluginProductionSources(root);
  const staticRows = toolJobStaticRows(productionFiles);
  const appOwnedRows = toolJobAppOwnedRows(productionFiles);
  const registeredAppOwnedKeys = new Set(appOwnedRows.map((row) => `${row.ownerFile}\0${row.toolId}`));
  const appOwnedKeys = new Set(appOwnedRows.filter((row) => row.publicationReady).map((row) => `${row.ownerFile}\0${row.toolId}`));
  const dispositions = toolJobDispositions(productionFiles);
  const batchOnlyRows = commandRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "BatchOnlyPendingRewrite").length;
  const forbiddenRows = commandRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "ForbiddenFromUi").length;
  const deletedRows = commandRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "Deleted").length;
  const actionBus = policyReadFileSafe(root, "🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs");
  const platform = policyReadFileSafe(root, "🧰️framework/🔨️modules/🖥️platform/🦀️.rs");
  const glue = policyReadFileSafe(root, "🧰️framework/📦️packages/🦀️rust/🦀️.rs");
  const plugin = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
  const artifactRetainedCommand = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs");
  const store = policyReadOsStoreSource(root);
  const presenceRetirement = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs");
  const vcs = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs");
  const causal = policyReadFileSafe(root, "🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs");
  const channel = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs");
  const actor = policyReadFileSafe(root, "🧰️framework/🔨️modules/🎭️actor/🦀️.rs");
  const kernel = policyReadFileSafe(root, "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs");
  const pluginHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs");
  const mcpWorkspace = readMcpGatewaySource(root, (file) => policyReadFileSafe(root, relative(root, file)));
  const runHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs");
  const wgpuHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
  const nativeIo = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs");
  const shardHost = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs");
  const jobRuntime = policyReadFileSafe(root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs");
  const puzzle5d = policyReadFileSafe(root, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const puzzle2d = policyReadModuleTreeSource(root, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const semio = policyReadFileSafe(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs");
  const manifest = policyReadFileSafe(root, "🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🦀️.rs");
  const componentWit = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit");
  const reactor = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs");
  const reactorRequests = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📮️requests/🦀️.rs");
  const reactorExecutor = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs");
  const reactorJobs = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs");
  const fem2dSession = policyReadFileSafe(root, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
  const fem2dEditor = policyReadFileSafe(root, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs");
  const fem2dModel = policyReadFileSafe(root, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs");
  const femPluginRoot = policyReadFileSafe(root, "🌎️hub/🧩️compositions/🏗️fem/🦀️.rs");
  const femGlue = policyReadFileSafe(root, "🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/🦀️.rs");
  const femAnalyses = policyReadFileSafe(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs");
  const femMesh = policyReadFileSafe(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs");
  const femSparse = policyReadFileSafe(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs");
  const femEngineModel = policyReadFileSafe(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs");
  const femElements2d = policyReadFileSafe(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs");
  const presentationEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const presentationWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs");
  const writerEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const writerEditor = policyReadFileSafe(root, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const writerWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs");
  const jackEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const jackEditor = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const jackWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs");
  const trinityRewrite = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs");
  const gisMapEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const gisMapEditor = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const gisMapWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs");
  const rasterRetainedSource = (file: string): string => {
    const source = policyReadFileSafe(root, file);
    if (source.trim() === "") throw new Error(`[verify interactivity tool-jobs] retained Raster envelope source ${JSON.stringify(file)} is missing or empty; the Raster envelope law cannot be evaluated against a path that does not resolve.`);
    return source;
  };
  const rasterMountedOutputSerializers = [
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs",
  ]
    .map(rasterRetainedSource);
  const rasterEnvelopeCodec = [
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs",
  ]
    .map(rasterRetainedSource)
    .concat(rasterMountedOutputSerializers)
    .join("\n");
  const rasterEditor = rasterRetainedSource("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const drawingEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs");
  const drawingEditorSources = [
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs",
  ].map((file) => policyReadFileSafe(root, file));
  const drawingEditorTestLaws = drawingEditorSources[0] ?? "";
  const drawingEditor = drawingEditorSources
    .map(toolJobProductionSource)
    .join("\n");
  const drawingConfig = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs");
  const drawingConfigProto = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto");
  const allRustFiles = new Map(policyAllRustFiles(root).map((file) => [file, policyReadFileSafe(root, file)]));
  const proofs = toolJobProofs(productionFiles);
  const proofIdentities = new Set(proofs.map(toolJobProofIdentity));
  const scanThenMonolithRows = toolJobScanThenMonolithRows(productionFiles, proofs);
  const scanThenMonolithKeys = new Set(scanThenMonolithRows.map((row) => `${row.file}\0${row.id}`));
  let fixedOperationFixture: FixedOperationFixtureOutput | undefined;
  let fixedOperationRustFixtureFresh = false;
  try {
    fixedOperationFixture = toolJobFixedOperationFixtureRun(root);
    fixedOperationRustFixtureFresh =
      policyReadFileSafe(root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs") === toolJobFixedOperationRustFixtureSource(root);
  } catch {
    fixedOperationFixture = undefined;
  }
  const selfTests = (await lanes.toolJobCoverageSelfTests()) + lanes.semioMemberStoreProducerSourceSelfTests((candidate) => toolJobMemberStoreProducerExact(candidate as ToolJobMemberStoreProducerSources), policyReadMemberStoreProducerSources(root), JSON.parse(policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧬️schema/🔣️.json"))) + lanes.toolJobFixedOperationRegistrySelfTests(jobRuntime) + lanes.toolJobDrawingGestureOperationOwnerSelfTests() + lanes.toolJobArtifactRetainedCommandSelfTests(artifactRetainedCommand, plugin) + (fixedOperationFixture?.results.length ?? 0) + Number(fixedOperationRustFixtureFresh);
  const factoryContracts: { file: string; line: number; factory: string; status: "explicit" | "missing" }[] = [];
  for (const [file, source] of allRustFiles) {
    const implementation = /impl(?:<[^>]+>)?\s+(?:semio_framework::)?ToolJobFactory\s+for\s+([^\s<{]+(?:<[^>{]+>)?)\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = implementation.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) break;
      factoryContracts.push({ file, line: source.slice(0, match.index).split("\n").length, factory: match[1]!, status: /fn\s+execution_contract\s*\(/.test(block.body) ? "explicit" : "missing" });
      implementation.lastIndex = block.end;
    }
  }
  const globalPayloadStoreInventory = toolJobGlobalPayloadStoreInventory(productionFiles);
  const globalPayloadStoreExemptions = globalPayloadStoreInventory.filter(toolJobGlobalPayloadStoreIsSharedFixedAuthority);
  const globalPayloadStores = globalPayloadStoreInventory.filter((row) => !toolJobGlobalPayloadStoreIsSharedFixedAuthority(row));
  const failures: string[] = [];
  if (!toolJobArtifactRetainedCommandExact(artifactRetainedCommand) || !toolJobArtifactRetainedCommandRuntimeLawExact(plugin)) failures.push("shared app-owned retained command shell lacks fixed ARC1 checkpoints, bounded restore/replay, incremental close, executable runtime replay/cancel laws, or hostile laws");
  if (!toolJobFixedOperationRegistryExact(jobRuntime)) failures.push("fixed operation scheduler authority lacks exact operation/generation admission, owner handback, bounded close, or hostile laws");
  if (!fixedOperationFixture) failures.push("fixed operation scheduler authority lacks its schema-first empty/single/max/max+1/collision/cancel/stale/ABA/interrupted-close executable fixture");
  if (!fixedOperationRustFixtureFresh) failures.push("production FixedOperationRegistry does not execute the exact generated language-neutral law stream");
  if (!toolJobDrawingGestureOperationOwnerExact(drawingEditor, drawingConfig, drawingConfigProto, plugin)) failures.push("Drawing gesture operation state remains process-global or persisted through Config/Draft/document lanes instead of one instance-retained worker owner");
  if (staticRows.length < commandRows.length) failures.push(`literal registration census regressed below macro census (${staticRows.length} < ${commandRows.length})`);
  if (!plugin.includes("pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];")) failures.push("app_commands! does not generate the exact ToolCommandCatalog row list");
  if ((plugin.match(/const TOOL_JOB_IDS: &'static \[&'static str\] = Self::TOOL_JOB_IDS;/g) ?? []).length !== 6) failures.push("app_commands! does not publish every generated row list through both typed catalog traits");
  if (!actionBus.includes("pub struct ActionBus {") || !actionBus.includes("trait ErasedToolJobFactory")) failures.push("ActionBus is not a heterogeneous erased-factory registry");
  if (actionBus.includes("NoToolJobFactories") || actionBus.includes("NoInteractiveJobs") || platform.includes("NoToolJobFactories") || glue.includes("NoToolJobFactories")) failures.push("a hard-coded no-factory sentinel remains in production framework wiring");
  if (!platform.includes("pub action_bus: ActionBus,") || !platform.includes("ActionBus::production()")) failures.push("Platform does not own the shared production heterogeneous ActionBus");
  if (!plugin.includes("ActionBus::production()") || /register_once\(AppCommandJobFactory/.test(plugin)) failures.push("generic AppCommandJobFactory registration remains or production ActionBus activation is absent");
  if (!plugin.includes("<A::Command as ::protocol::OpBinary>::TOOL_JOB_IDS")) failures.push("production factory registration is not generated from the typed command schema");
  if (!plugin.includes("ToolOperationSpec::new(") || !plugin.includes(".tool_jobs\n                .dispatch(")) failures.push("typed app commands do not dispatch a typed ToolOperationSpec through ActionBus");
  if (!actionBus.includes("pub fn admit_exact_wire") || !plugin.includes(".admit_exact_wire(")) failures.push("typed command admission does not enforce the exact owner/tool/schema raw-wire contract before decoding");
  if (!toolJobTypedDispatchExact(actionBus) || !actionBus.includes('bus.dispatch(operation_spec("app", "alias"))')) failures.push("public typed ActionBus dispatch still permits alias fallback");
  if (!toolJobWireDispatchExact(actionBus) || !actionBus.includes('bus.dispatch_wire("app", "alias"')) failures.push("public wire ActionBus dispatch still permits alias fallback");
  if (!toolJobDecodeAfterAdmission(plugin)) failures.push("a typed action, command, or intent decoder runs before exact wire admission");
  if (!toolJobQualifiedProofBeforeDecode(plugin)) failures.push("a typed JSON or intent route decodes before selecting its exact qualified proof identity");
  const frameworkReservedExact = toolJobFrameworkReservedRoutesExact(plugin);
  const puzzleJobs = policyReadFileSafe(root, "✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs");
  const puzzle5dEvidence = policyReadRustPolicySource(root, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const puzzleReservedExact = toolJobPuzzleReservedRoutesExact(puzzle5dEvidence, plugin, puzzleJobs);
  const puzzle2dReservedExact = toolJobPuzzle2dReservedRoutesExact(puzzle2d, plugin, puzzleJobs);
  const importPreparationBounded = toolJobImportPreparationBounded(plugin);
  const fullToolOperationJobBounded = toolJobFullOperationBounded(plugin);
  const storeOneItemPublicationBounded = toolJobStoreBatchPublicationBounded(store, plugin);
  const ephemeralOneItemPublicationBounded = toolJobEphemeralOneItemPublicationBounded(store, plugin);
  const fullToolOperationBounded = fullToolOperationJobBounded && storeOneItemPublicationBounded && ephemeralOneItemPublicationBounded;
  if (!frameworkReservedExact) failures.push("framework-owned reserved routes lack exact route-specific resumable factories, direct-branch closure, binary fail-closure, or commit-held cancellation");
  if (!importPreparationBounded) failures.push("shared import submission serializes or clones the whole media envelope before resumable job construction");
  if (!fullToolOperationJobBounded) failures.push("shared typed-command preparation remains outside the resumable job protocol");
  if (!storeOneItemPublicationBounded) failures.push("shared one-item Store publication still enters whole replay/apply/outbound work instead of a domain-prepared persistent publication");
  if (!ephemeralOneItemPublicationBounded) failures.push("shared Presence/Transient one-item publication lacks retained preparation, displaced-root retirement, ACK, cancellation, or terminal-empty closure");
  if (!fullToolOperationBounded && (!toolJobTypedRouteFailsClosedBeforePreparation(plugin) || !toolJobTypedPersistentFoundation(plugin))) failures.push("incomplete typed-command route is reachable before preparation or retains no saturation-safe persistent operation foundation");
  if (!toolJobImmutableOperationRootsExact(store)) failures.push("document, presence, or transient command preparation lacks exact O(1) immutable event-maintained roots");
  if (!toolJobChildContentRootExact(store, plugin)) failures.push("typed command preparation lacks a fixed-width event-maintained immutable child-content root and no-default terminal-witnessed old-root retirement authority");
  if (!toolJobMemberStoreProducerExact(policyReadMemberStoreProducerSources(root))) failures.push("generated member create/open permits a missing, partial, default, or wrapper-bypassable snapshot/whole-store disposal owner");
  if (!toolJobArtifactStoreStructuralOwnersExact(store, semio, causal, vcs)) failures.push("artifact store structural owners still use resizable string-key indexes, deep direct replacement, or lack bounded conflict/backbone/DAG/history disassembly and terminal Drop authority");
  if (!toolJobArtifactEnvelopeOwnedCodecExact(store, allRustFiles)) failures.push("ArtifactEnvelope ingress retains public serde Deserialize or lacks fixed-page owned preflight/decode/error-close authority across every production caller");
  if (!toolJobPresentationEnvelopeCallerRetainedExact(presentationEnvelopeCodec, presentationWasm, plugin)) failures.push("Presentation envelope representative caller exposes a raw job/whole-string Wasm route or lacks fixed app-retained worker/result/close ownership");
  if (!toolJobWriterEnvelopeCallerRetainedExact(writerEnvelopeCodec, writerEditor, writerWasm, plugin)) failures.push("Writer envelope caller lacks retained fixed-page ingress, initializer recovery, maintenance publication, cancellation, or exact completion acknowledgement");
  if (!toolJobJackEnvelopeCallerRetainedExact(store, jackEnvelopeCodec, jackEditor, jackWasm, plugin)) failures.push("Jack `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, exact child retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement");
  if (!toolJobTrinityRewriteEnvelopeCallerRetainedExact(store, jackEnvelopeCodec, jackEditor, trinityRewrite, plugin)) failures.push("Trinity Rewrite envelope caller lacks the Jack-owned fixed-page operation store, generation handle, bounded progress/cancel/close, exact rejected-page handback, or completion acknowledgement");
  if (!toolJobGisMapEnvelopeCallerRetainedExact(store, gisMapEnvelopeCodec, gisMapEditor, gisMapWasm, plugin)) failures.push("GIS Map `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, exact drawing/image/value child retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement");
  if (!toolJobRasterEnvelopeCallerRetainedExact(store, rasterEnvelopeCodec, rasterEditor, plugin)) failures.push("Raster `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, recursive domain retirement, the editor-hosted credit-before-page fixed-page ingress cohort (submit/pump/exact-ack plus cancel), initializer recovery, the guarded owned-map `dsl::ToValue`/`dsl::FromValue` output authority, or its eight mounted export serializers are not the six real format encoders plus two typed `RASTER_*_EXPORT_UNSUPPORTED` declines");
  if (!toolJobDrawingEnvelopeCallerRetainedExact(store, drawingEnvelopeCodec, drawingEditor, plugin, drawingEditorTestLaws)) failures.push("Drawing `.spr`/`.ops` envelope caller lacks recursive layer/style/asset preflight, one-item clone/retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement");
  if (!toolJobChildRetirementInventoryExact(root, allRustFiles)) failures.push("child snapshot retirement domain cohorts or callsites do not match the exact machine-readable owner inventory");
  if (!toolJobPeerInteractionRootsExact(plugin, store, channel, presenceRetirement)) failures.push("peer ingress, app-typed presence, or interaction roots lack reserve-before-decode retained per-entry publication, atomic validated commit, or O(1) immutable capture");
  if (!toolJobPagedIngressExact(kernel, reactor, plugin, pluginHost, channel, componentWit, mcpWorkspace, runHost, wgpuHost)) failures.push("paged command ingress lacks fixed-page ownership, retained streaming decode/fault closure, generic multi-page ACK ordering, or terminal-close registries across MCP/run/WGPU callers");
  if (!toolJobActorProgressOverlayExact(actor, shardHost, wgpuHost)) failures.push("actor Job publications lack a fixed preview/progress overlay, independent autonomous-shard authority, presenter-adopted ACK, exact rejected-owner handback, or app/fault/realm retirement");
  if (!toolJobUniversalRetainedOwnershipExact(jobRuntime, nativeIo, wgpuHost, allRustFiles)) failures.push("universal retained job ownership still permits optional job close, post-work fault allocation, legacy drains/constructors, resizable Native I/O outputs, or future-driven mounted session progress");
  if (!toolJobFem2dMountedSessionExact(fem2dSession, fem2dEditor, fem2dModel, femMesh, femPluginRoot, femGlue, reactorJobs, femAnalyses, reactor, store, plugin)) failures.push("live FEM2D revisions lack a fixed generation-tagged retained worker session, mounted visual/mesh/stiffness backing, exact snapshot-return witness, or live visual consumer");
  if (!toolJobFemNumericalMicrocursorExact(femSparse, femMesh, femAnalyses, femEngineModel, femElements2d, fem2dSession, jobRuntime)) failures.push("P6h FEM numerical LDLT, subspace, constraint recovery, or fixed-schema stiffness loops lack exact persistent microcursors, owner credit, identity, or hostile laws");
  if (!puzzleReservedExact) failures.push("Puzzle5d clipboard/import routes lack exact owner-qualified route-specific resumable factories and state machines");
  if (!puzzle2dReservedExact) failures.push("Puzzle2d clipboard/import routes lack an app-owned fragment vocabulary, the four reserved route branches, or a cancellable complete-before-output clipboard step");
  if (!toolJobMediaExportBounded(`${plugin}\n${jobRuntime}`)) failures.push("owned media export lacks nonblocking per-instance polling, exact sealed structural output credit, or pending/drop/isolation regressions");
  if (!toolJobExternalCancellationOwned(plugin)) failures.push("typed jobs lack externally reachable operation/document/app/generation cancellation through close and supersession");
  if (!toolJobDropCancellationBounded(plugin)) failures.push("VcsArtifactApp Drop synchronously drains the whole live-operation cancellation map instead of O(1) scope cancellation with bounded asynchronous cleanup");
  if (!toolJobCancellationScopesBounded(plugin)) failures.push("tool cancellation begin, supersession, document close, or app close performs collection-wide work instead of parent-scope O(1) cancellation");
  if (!toolJobRuntimeRegistryFixedClose(plugin, reactor)) failures.push("runtime instance/actor authority still grows, hashes, scans, shifts, blocks, or drops the detached app on InstanceClose");
  if (!toolJobHardBoundedCloseExact(plugin, reactor)) failures.push("instance close still permits implicit nested payload destruction or lacks saturation-safe bounded cleanup job ownership");
  if (!toolJobReactorCloseBounded(reactor, reactorRequests, reactorExecutor)) failures.push("reactor task/request/open-instance close still scans resizable maps/vectors, retains ready queues, or wakes an unbounded waiter list");
  if (!toolJobOpaqueFutureProductionFailClosed(reactorJobs)) failures.push("production reactor jobs still carry an opaque Future path, a cfg-split admission, or an undeclared builtin set instead of one bounded state-machine registry");
  if (!toolJobSegmentedTerminalDrainExact(`${plugin}\n${componentWit}`)) failures.push("segmented download authority lacks its exact WIT guest bridge or is removed before the host observes terminal None");
  if (!toolJobLiveInstanceIsolated(plugin)) failures.push("typed operation/cancellation scope does not distinguish live app instances from static owner/controller identity");
  failures.push(...toolJobProofCatalogFailures(allRustFiles, staticRows, dispositions, proofs));
  if (proofIdentities.size !== proofs.length) failures.push("bounded reducer proof identities are not unique");
  if (!toolJobRuntimeProofQualified(plugin)) failures.push("runtime activation or public DFF admission drops the compiler-derived owner witness or exact controller/owner/factory/tool/schema proof identity");
  if (proofs.some((proof, index) => proofs.slice(index + 1).some((other) => other.controllerId === proof.controllerId && other.toolId === proof.toolId && other.values[0] !== proof.values[0]))) failures.push("same-controller/tool qualified proofs disagree on their public raw-wire limit");
  const innerStart = plugin.lastIndexOf("async fn dispatch_typed_command_inner");
  const innerOpen = innerStart < 0 ? -1 : plugin.indexOf("{", innerStart);
  const inner = innerOpen < 0 ? undefined : toolJobRustBlock(plugin, innerOpen);
  if (!inner || inner.body.includes("A::handle(&command")) failures.push("dispatch_typed_command_inner still bypasses the job bus with a direct A::handle call");
  if (!toolJobMountedDispatchOneTurnExact(plugin)) failures.push("production typed dispatch is not one explicitly admitted WorkerJobSession turn");
  if (!toolJobMicrosecondWorkerExact(plugin, jobRuntime, policyReadFileSafe(root, "🧰️framework/🔨️modules/⏱️trace/🦀️.rs"), policyReadFileSafe(root, "🧰️framework/🔨️modules/🧵️job/📬️outcome/🤝️loan/🦀️.rs"))) failures.push("bounded command worker does not enforce decoded, work, exact microsecond step-time, and output contract limits");
  const forKindStart = manifest.indexOf("pub fn for_kind(kind: ActionKind)");
  const forKindOpen = forKindStart < 0 ? -1 : manifest.indexOf("{", forKindStart);
  const forKind = forKindOpen < 0 ? undefined : toolJobRustBlock(manifest, forKindOpen);
  if (!forKind || forKind.body.includes("InteractiveJobClassification::Migrated")) failures.push("ActionSemantics::for_kind still blanket-classifies declarations as Migrated");
  if (/pub fn bounded_catalog\([\s\S]{0,500}?InteractiveJobClassification::Migrated/.test(manifest)) failures.push("bounded_catalog still self-certifies Migrated");
  if (factoryContracts.some((factory) => factory.status === "missing")) failures.push(`${factoryContracts.filter((factory) => factory.status === "missing").length} ToolJobFactory implementation(s) lack explicit execution contracts`);
  if (globalPayloadStores.length > 0) failures.push(`${globalPayloadStores.length} process-global payload store candidate(s) require operation-owned state or an explicit static exemption`);
  if (scanThenMonolithRows.length > 0) failures.push(`${scanThenMonolithRows.length} resumable route(s) only scan inputs before invoking a monolithic reducer; see scanThenMonolithRows ledger`);
  if (appOwnedRows.some((row) => row.publicationLanes.length === 0)) failures.push(`${appOwnedRows.filter((row) => row.publicationLanes.length === 0).length} app-owned retained route(s) lack an exact nonempty publication-lane contract`);
  if (appOwnedRows.some((row) => row.publicationLanes.length > 0 && !row.publicationReady)) failures.push(`${appOwnedRows.filter((row) => row.publicationLanes.length > 0 && !row.publicationReady).length} app-owned retained route(s) declare Store publication lanes without their exact app-owned preparation authority`);
  if (/"typed-command"\.to_string\(\)/.test(inner?.body ?? "")) failures.push("typed-command dispatch fallback remains");
  if (!toolJobPublicationFreshnessBeforeEveryTurn(plugin))
    failures.push("commit freshness is not validated before every pending or newly begun result-publication turn");
  const remainingCommands = staticRows
    .filter((row) => {
      const disposition = dispositions.get(`${row.file}\0${row.id}`);
      const proof = proofs.find((candidate) => candidate.ownerFile === row.file && candidate.ownerTypeName.length > 0 && candidate.controllerId.length > 0 && candidate.factory.length > 0 && candidate.toolId === row.id && candidate.documentSchema.length > 0);
      return (
        !["BatchOnlyPendingRewrite", "ForbiddenFromUi", "Deleted"].includes(disposition ?? "") &&
        !(
          fullToolOperationBounded &&
          disposition === "Migrated" &&
          proof &&
          proofIdentities.has(toolJobProofIdentity(proof)) &&
          appOwnedKeys.has(`${row.file}\0${row.id}`) &&
          !scanThenMonolithKeys.has(`${row.file}\0${row.id}`)
        )
      );
    })
    .map((row) => {
      const disposition = dispositions.get(`${row.file}\0${row.id}`);
      const reason =
        disposition !== "Migrated"
          ? "no explicit factory, bounded-first-step proof, or fail-closed disposition"
          : !registeredAppOwnedKeys.has(`${row.file}\0${row.id}`)
            ? "classification proof exists but no exact registered app-owned retained reducer factory and builder"
            : !appOwnedKeys.has(`${row.file}\0${row.id}`)
              ? "app-owned retained reducer lacks an exact publishable completion-lane contract or preparation owner"
              : scanThenMonolithKeys.has(`${row.file}\0${row.id}`)
                ? "resumable route scans inputs then invokes a monolithic reducer"
                : "owner-local retained route exists but full prepare/job/commit operation is not bounded";
      return { ...row, reason };
    });
  const remainingCommandKeys = new Set(remainingCommands.map((row) => `${row.file}\0${row.id}`));
  const acceptedCommandRows = [
    ...staticRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "Migrated" && !remainingCommandKeys.has(`${row.file}\0${row.id}`)),
    ...(puzzleReservedExact ? TOOL_JOB_PLUGIN_RESERVED_IDS.map((id) => ({
      file: "✏️s/🔌️plugins/🧩️puzzle/🗀️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
      id,
      source: "literal" as const,
    })) : []),
  ];
  const frameworkReservedRoutes = [
    ...(frameworkReservedExact ? [] : TOOL_JOB_FRAMEWORK_RESERVED_IDS),
    ...(puzzleReservedExact ? [] : TOOL_JOB_PLUGIN_RESERVED_IDS),
  ].map((id) => ({ id, status: "failClosedPendingFactory" as const }));
  const reservedImporters = toolJobReservedImporterOwners(root, productionFiles);
  failures.push(...reservedImporters.failures);
  if (frameworkReservedRoutes.length > 0) failures.push(`${frameworkReservedRoutes.length} framework-reserved route(s) remain fail-closed pending explicit resumable factories`);
  if (reservedImporters.pending.length > 0) failures.push(`${reservedImporters.pending.length} app-owned import-media route(s) remain fail-closed pending explicit resumable factories`);
  if (remainingCommands.length > 0) failures.push(`${remainingCommands.length} live command registration(s) remain fail-closed; see remainingCommands ledger`);
  if (!actionBus.includes("duplicate_key_inside_one_factory_is_rejected_atomically")) failures.push("factory key ownership lacks an atomic duplicate/bijection test");
  if (!plugin.includes("generated_tool_job_catalog_is_an_exact_bijection_with_rows")) failures.push("generated command rows lack an exact catalog bijection test");
  if (!plugin.includes("activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations")) failures.push("activation lacks an exact migrated-declaration/factory-key bijection test");
  if (!plugin.includes("copied_app_type_cannot_inherit_same_controller_schema_and_id_proof") || !plugin.includes("TypedCommandFullOperationJobFactory::<CopyDrawApp>::from_proof")) failures.push("activation lacks an executable copied-owner compiler-witness fail-closed proof");
  if (!plugin.includes("unproved_command_fails_before_an_overrun_reducer_can_start")) failures.push("unproved typed operations lack executable fail-closure before reducer watchdog work starts");
  return {
    macroHostFiles: macroFiles.size,
    macroInvocations,
    commandRows: commandRows.length,
    uniqueCommandRows: seenRows.size,
    duplicateRows,
    fixtureMacroHostFiles: fixtureMacroFiles.size,
    fixtureMacroInvocations,
    fixtureCommandRows: fixtureCommandRows.length,
    boundedRows: staticRows.length - remainingCommands.length - batchOnlyRows - forbiddenRows - deletedRows,
    batchOnlyRows,
    forbiddenRows,
    deletedRows,
    productionFactories: factoryContracts.length,
    productionRegistrations: appOwnedRows.length,
    productionDispatches: (plugin.match(/\.tool_jobs\s*\.dispatch\(/g) ?? []).length,
    aliases: (actionBus.match(/register_alias\(/g) ?? []).length,
    factoryContracts,
    frameworkReservedRoutes,
    reservedImporterOwners: reservedImporters.pending,
    globalPayloadStores,
    globalPayloadStoreExemptions,
    scanThenMonolithRows,
    literalRegistrations: staticRows.filter((row) => row.source === "literal").length,
    acceptedCommandRows,
    acceptedReservedRoutes: [
      ...(frameworkReservedExact ? TOOL_JOB_FRAMEWORK_RESERVED_IDS : []),
    ],
    remainingCommands,
    pipelineGates: {
      fullToolOperationJobBounded,
      durableOneItemPublicationBounded: storeOneItemPublicationBounded,
      ephemeralOneItemPublicationBounded,
      fullToolOperationBounded,
    },
    selfTests,
    failures,
  };
}
//#endregion 🎯️ToolJobCoverage
