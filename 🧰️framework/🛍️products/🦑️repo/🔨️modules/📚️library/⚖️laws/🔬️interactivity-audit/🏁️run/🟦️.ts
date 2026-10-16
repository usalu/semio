/** 🧭️ Interactivity audit: the run that reconciles findings against the allowlist.
 * @see 📜️script.ts
 */
import { interactivityToolRunLocalLifecycleFailures, interactivityToolRunLegacyTraceFailures, interactivityToolRunDeclarationFailures, interactivityToolRunReservedActionFailures } from "../../../🔍️discovery/📋️policy/⏯️tool-run/🟦️.ts";
import { POLICY_SPLIT_SOURCE_ROOTS, policyReadFileSafe, policyReadModuleTreeSource } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { interactivityArtifactHistoryFailures, interactivityArtifactSubmitFailures, interactivityDatabaseCapabilityOpenFailures, interactivityDatabaseCatalogReadFailures, interactivityDbCliFailures, interactivityDbIoB1B6Failures, interactivityDbIoCallerMigrationFailures, interactivityDbIoCursorCallerFailures, interactivityDbIoDirectWriterFailures, interactivityDbIoFailures, interactivityMcpHttpTransportFailures, interactivityPreparedRasterProducerFailures, interactivityShardExecutorFailures, interactivityStoreSyncFailures } from "../🗄️database/🟦️.ts";
import { INTERACTIVITY_AUDIT_ALLOWLIST, INTERACTIVITY_AUDIT_CANVAS_RASTER_FILE, INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY, INTERACTIVITY_AUDIT_DB_ARTIFACT_FILE, INTERACTIVITY_AUDIT_DB_CLI_FILE, INTERACTIVITY_AUDIT_DB_CLUSTER_FILE, INTERACTIVITY_AUDIT_DB_COMPACT_FILE, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY, INTERACTIVITY_AUDIT_DB_ENGINE_FILE, INTERACTIVITY_AUDIT_DB_INDEX_FILE, INTERACTIVITY_AUDIT_DB_NEO4J_FILE, INTERACTIVITY_AUDIT_DB_POSTGRES_FILE, INTERACTIVITY_AUDIT_DB_PROJECTION_FILE, INTERACTIVITY_AUDIT_DB_QUERY_FILE, INTERACTIVITY_AUDIT_DB_ROOT_FILE, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE, INTERACTIVITY_AUDIT_DB_SQLITE_FILE, INTERACTIVITY_AUDIT_DB_STATE_FILE, INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY, INTERACTIVITY_AUDIT_DB_STORAGE_FILE, INTERACTIVITY_AUDIT_DB_SYNC_FILE, INTERACTIVITY_AUDIT_DB_TESTKIT_FILE, INTERACTIVITY_AUDIT_DB_WAL_FAMILY, INTERACTIVITY_AUDIT_DB_WAL_FILE, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES, INTERACTIVITY_AUDIT_FIND_ITEMS_FILE, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY, INTERACTIVITY_AUDIT_HUB_BIN_FILE, INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE, INTERACTIVITY_AUDIT_KERNEL_FILE, INTERACTIVITY_AUDIT_KERNEL_UI_FILE, INTERACTIVITY_AUDIT_MCP_BRIDGE_FILE, INTERACTIVITY_AUDIT_MCP_BRIDGE_SESSION_FILE, INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE, INTERACTIVITY_AUDIT_MCP_ROOT_FILE, INTERACTIVITY_AUDIT_OS_ACTIVATION_FILE, INTERACTIVITY_AUDIT_OS_SERVICES_FILE, INTERACTIVITY_AUDIT_PACK_FORMAT_FILE, INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_REACTOR_FILE, INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE, INTERACTIVITY_AUDIT_REACTOR_TURN_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_RENDERER_RUNTIME_FILE, INTERACTIVITY_AUDIT_RUN_FILE, INTERACTIVITY_AUDIT_SHARD_FILE, INTERACTIVITY_AUDIT_SHELL_FILE, INTERACTIVITY_AUDIT_SPR_FORMAT_FILE, INTERACTIVITY_AUDIT_STORE_SYNC_FILE, INTERACTIVITY_AUDIT_UI_ACCESSIBILITY_FILE, INTERACTIVITY_AUDIT_UI_BUILDER_FILE, INTERACTIVITY_AUDIT_UI_COMPONENT_FILE, INTERACTIVITY_AUDIT_UI_DOCUMENT_FILE, INTERACTIVITY_AUDIT_UI_LAYOUT_FILE, INTERACTIVITY_AUDIT_UI_LIMITS_FILE, INTERACTIVITY_AUDIT_UI_PRESENT_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_FILE, INTERACTIVITY_AUDIT_UI_SURFACE_FILE, INTERACTIVITY_AUDIT_UI_VALUE_FILE, INTERACTIVITY_AUDIT_WINDOW_MEASURE_FILE, INTERACTIVITY_AUDIT_WORLD3D_FILES, type InteractivityAllowlistEntry, type InteractivityFinding } from "../🗂️files/🟦️.ts";
import { interactivityLiveReconcileFailures, interactivityMountedFrameTransactionFailures, interactivityMountedLayoutTextFailures } from "../🖥️mounted/🟦️.ts";
import { INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES, INTERACTIVITY_TOOL_RUN_REQUIREMENTS, interactivityPuzzleFillP4eFailures, interactivityPuzzleFillRunJobFailures, interactivityPuzzleFillSources, interactivityPuzzleFillTraceFailures, interactivityToolRunPolicySources } from "../🪣️tool-run-and-history/🟦️.ts";
import { interactivityAuditScan } from "../🔎️scan/🟦️.ts";
import { policyReadRustPolicyFamily, policyReadRustPolicySource, policyReadUiReconcileSource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import type { SelfTestLanes } from "../../🧪️self-test-lanes/🟦️.ts";

type InteractivityAuditReport = {
  findings: InteractivityFinding[];
  byCategory: Record<string, number>;
  blockingBridgeUnlisted: InteractivityFinding[];
  staleAllowlistEntries: InteractivityAllowlistEntry[];
  expectedNeverToMatchEntries: InteractivityAllowlistEntry[];
  preDeclaredOutOfScope: InteractivityAllowlistEntry[];
};

/** ⏱️Runs the scan and reconciles findings against [[INTERACTIVITY_AUDIT_ALLOWLIST]]. */
export function interactivityAuditRun(repoRoot: string, lanes: SelfTestLanes): InteractivityAuditReport {
  const findings = interactivityAuditScan(repoRoot, lanes);
  lanes.interactivityShardExecutorSelfTests();
  const executor = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES[0]);
  const shard = policyReadFileSafe(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs");
  for (const failure of interactivityShardExecutorFailures(executor, shard)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES[0], line: 0, text: failure });
  lanes.interactivityMcpHttpTransportSelfTests();
  const mcpTransport = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE);
  const mcpBridge = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_MCP_BRIDGE_FILE) + policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_MCP_BRIDGE_SESSION_FILE);
  const mcpRoot = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_MCP_ROOT_FILE);
  for (const failure of interactivityMcpHttpTransportFailures(mcpTransport, mcpBridge, mcpRoot)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE, line: 0, text: failure });
  lanes.interactivityStoreSyncSelfTests();
  const storeSync = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_STORE_SYNC_FILE);
  for (const failure of interactivityStoreSyncFailures(storeSync)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_STORE_SYNC_FILE, line: 0, text: failure });
  lanes.interactivityDbIoSelfTests();
  const dbStorage = policyReadRustPolicyFamily(repoRoot, INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY);
  const dbSqlite = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_SQLITE_FILE);
  const dbPostgres = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_POSTGRES_FILE);
  const dbNeo4j = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_NEO4J_FILE);
  const dbEngine = policyReadRustPolicyFamily(repoRoot, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY);
  const dbSync = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_SYNC_FILE);
  const dbArtifact = policyReadRustPolicyFamily(repoRoot, INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY);
  const dbWal = policyReadRustPolicyFamily(repoRoot, INTERACTIVITY_AUDIT_DB_WAL_FAMILY);
  const sprFormat = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_SPR_FORMAT_FILE);
  const packFormat = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_PACK_FORMAT_FILE);
  const dbSnapshot = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE);
  const dbIndex = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_INDEX_FILE);
  const dbCompact = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_COMPACT_FILE);
  const dbCluster = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_CLUSTER_FILE);
  const dbState = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_STATE_FILE);
  const dbQuery = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_QUERY_FILE);
  const dbProjection = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_PROJECTION_FILE);
  const dbRoot = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_ROOT_FILE);
  const dbCli = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_CLI_FILE);
  const dbTestkit = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_DB_TESTKIT_FILE);
  const hubBin = policyReadRustPolicyFamily(repoRoot, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY);
  for (const failure of interactivityDbIoFailures(dbStorage, dbSqlite, dbEngine, dbTestkit, hubBin)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_STORAGE_FILE, line: 0, text: failure });
  lanes.interactivityDbIoB1B6SelfTests();
  for (const failure of interactivityDbIoB1B6Failures(dbStorage, dbSqlite, dbPostgres, dbNeo4j, hubBin)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_STORAGE_FILE, line: 0, text: failure });
  lanes.interactivityDbIoCallerMigrationSelfTests();
  for (const failure of interactivityDbIoCallerMigrationFailures([
    [INTERACTIVITY_AUDIT_DB_STORAGE_FILE, dbStorage],
    [INTERACTIVITY_AUDIT_DB_SQLITE_FILE, dbSqlite],
    [INTERACTIVITY_AUDIT_DB_POSTGRES_FILE, dbPostgres],
    [INTERACTIVITY_AUDIT_DB_NEO4J_FILE, dbNeo4j],
    [INTERACTIVITY_AUDIT_DB_ENGINE_FILE, dbEngine],
    [INTERACTIVITY_AUDIT_DB_SYNC_FILE, dbSync],
    [INTERACTIVITY_AUDIT_DB_ARTIFACT_FILE, dbArtifact],
    [INTERACTIVITY_AUDIT_DB_WAL_FILE, dbWal],
    [INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE, dbSnapshot],
    [INTERACTIVITY_AUDIT_DB_INDEX_FILE, dbIndex],
    [INTERACTIVITY_AUDIT_DB_COMPACT_FILE, dbCompact],
    [INTERACTIVITY_AUDIT_DB_CLUSTER_FILE, dbCluster],
    [INTERACTIVITY_AUDIT_DB_STATE_FILE, dbState],
    [INTERACTIVITY_AUDIT_DB_QUERY_FILE, dbQuery],
    [INTERACTIVITY_AUDIT_DB_PROJECTION_FILE, dbProjection],
    [INTERACTIVITY_AUDIT_DB_ROOT_FILE, dbRoot],
    [INTERACTIVITY_AUDIT_DB_TESTKIT_FILE, dbTestkit],
    [INTERACTIVITY_AUDIT_HUB_BIN_FILE, hubBin],
  ])) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_STORAGE_FILE, line: 0, text: failure });
  lanes.interactivityDbIoDirectWriterSelfTests();
  for (const failure of interactivityDbIoDirectWriterFailures(dbEngine, dbSnapshot, dbIndex, dbWal, sprFormat, packFormat)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_STORAGE_FILE, line: 0, text: failure });
  for (const failure of interactivityDbIoCursorCallerFailures(dbSync)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_SYNC_FILE, line: 0, text: failure });
  lanes.interactivityArtifactSubmitSelfTests();
  for (const failure of interactivityArtifactSubmitFailures(dbEngine, dbArtifact)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_ENGINE_FILE, line: 0, text: failure });
  lanes.interactivityDatabaseCapabilityOpenSelfTests();
  for (const failure of interactivityDatabaseCapabilityOpenFailures(dbEngine)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_ENGINE_FILE, line: 0, text: failure });
  lanes.interactivityDatabaseCatalogReadSelfTests();
  for (const failure of interactivityDatabaseCatalogReadFailures(dbEngine)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_ENGINE_FILE, line: 0, text: failure });
  lanes.interactivityArtifactHistorySelfTests();
  for (const failure of interactivityArtifactHistoryFailures(dbEngine, dbArtifact, dbWal)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_ENGINE_FILE, line: 0, text: failure });
  lanes.interactivityDbCliSelfTests();
  for (const failure of interactivityDbCliFailures(dbCli)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_DB_CLI_FILE, line: 0, text: failure });
  lanes.interactivityPreparedRasterProducerSelfTests();
  const preparedRaster = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE);
  const preparedRasterDraw = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE);
  const preparedRasterGpu = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE);
  const canvasRaster = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_CANVAS_RASTER_FILE);
  const interpreterRaster = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE);
  const rendererGlue = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE);
  const rendererHost = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE);
  for (const failure of interactivityPreparedRasterProducerFailures(preparedRaster, preparedRasterDraw, preparedRasterGpu, canvasRaster, interpreterRaster, rendererGlue, rendererHost)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, line: 0, text: failure });
  const puzzleFill = interactivityPuzzleFillSources(repoRoot);
  for (const failure of interactivityPuzzleFillP4eFailures(puzzleFill.precompute, puzzleFill.fill, puzzleFill.geometry)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES.fill, line: 0, text: failure });
  for (const failure of interactivityPuzzleFillRunJobFailures(puzzleFill)) findings.push({ category: "tool-run", file: INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES.fill, line: 0, text: failure });
  for (const failure of interactivityPuzzleFillTraceFailures(puzzleFill)) findings.push({ category: "tool-run", file: INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES.renderer, line: 0, text: failure });
  lanes.interactivityToolRunPolicySelfTests();
  const toolRunSources = interactivityToolRunPolicySources(repoRoot, INTERACTIVITY_TOOL_RUN_REQUIREMENTS);
  for (const finding of [
    ...interactivityToolRunLocalLifecycleFailures(toolRunSources, INTERACTIVITY_TOOL_RUN_REQUIREMENTS),
    ...interactivityToolRunLegacyTraceFailures(toolRunSources),
    ...interactivityToolRunDeclarationFailures(toolRunSources, INTERACTIVITY_TOOL_RUN_REQUIREMENTS),
    ...interactivityToolRunReservedActionFailures(toolRunSources),
  ]) findings.push({ category: "tool-run", ...finding });
  lanes.interactivityLiveReconcileSelfTests(repoRoot);
  const uiReconcile = policyReadUiReconcileSource(repoRoot);
  const uiValue = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_UI_VALUE_FILE);
  const kernelTurn = [INTERACTIVITY_AUDIT_KERNEL_UI_FILE, INTERACTIVITY_AUDIT_KERNEL_FILE].map((file) => { const text = policyReadRustPolicySource(repoRoot, file); return text.slice(text.indexOf("//#region 🔖️TurnResult"), text.indexOf("//#endregion 🔖️TurnResult")); }).join("\n");
  const pluginCentral = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE);
  const tableKit = pluginCentral.slice(pluginCentral.indexOf("pub const TABLE_WINDOW_COLUMNS"), pluginCentral.indexOf("//#endregion 🔖️TableWindowKit"));
  const tableKitFixtures = ["table_rows_max_plus_one_returns_the_exact_row_owner", "abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity"].filter((fixture) => pluginCentral.includes(`fn ${fixture}`)).join("\n");
  const commandBridge = pluginCentral.slice(pluginCentral.indexOf("pub const UI_COMMAND_VALUE_DEPTH"), pluginCentral.indexOf("//#endregion 🔖️ActionFactory"));
  const commandBridgeFixtures = ["retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner", "retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original"].filter((fixture) => pluginCentral.includes(`fn ${fixture}`)).join("\n");
  const uiSchema = [
    INTERACTIVITY_AUDIT_UI_BUILDER_FILE,
    INTERACTIVITY_AUDIT_UI_COMPONENT_FILE,
    INTERACTIVITY_AUDIT_UI_ACCESSIBILITY_FILE,
    INTERACTIVITY_AUDIT_UI_SURFACE_FILE,
    INTERACTIVITY_AUDIT_UI_DOCUMENT_FILE,
    INTERACTIVITY_AUDIT_UI_LIMITS_FILE,
    INTERACTIVITY_AUDIT_UI_LAYOUT_FILE,
    INTERACTIVITY_AUDIT_UI_PRESENT_FILE,
  ].map((file) => policyReadFileSafe(repoRoot, file)).join("\n") + kernelTurn + tableKit + tableKitFixtures + commandBridge + commandBridgeFixtures;
  const reactorPatches = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE);
  const reactor = [INTERACTIVITY_AUDIT_REACTOR_FILE, INTERACTIVITY_AUDIT_REACTOR_TURN_FILE, INTERACTIVITY_AUDIT_SHARD_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, INTERACTIVITY_AUDIT_RUN_FILE, INTERACTIVITY_AUDIT_OS_ACTIVATION_FILE, INTERACTIVITY_AUDIT_RENDERER_RUNTIME_FILE, INTERACTIVITY_AUDIT_WINDOW_MEASURE_FILE, INTERACTIVITY_AUDIT_SHELL_FILE].map((file) => POLICY_SPLIT_SOURCE_ROOTS.includes(file) ? policyReadModuleTreeSource(repoRoot, file) : policyReadFileSafe(repoRoot, file)).join("\n");
  for (const failure of interactivityLiveReconcileFailures(uiReconcile, reactorPatches, reactor, uiValue, uiSchema)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_UI_RECONCILE_FILE, line: 0, text: failure });
  lanes.interactivityMountedLayoutTextSelfTests(repoRoot);
  const mountedLayout = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs");
  const mountedEngine = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs");
  const mountedTree = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/🦀️.rs");
  const mountedPaint = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs");
  const mountedEvents = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs");
  const mountedSlots = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📨️scene_slots/🦀️.rs");
  const mountedInterpreter = policyReadModuleTreeSource(repoRoot, INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE);
  for (const failure of interactivityMountedLayoutTextFailures(mountedLayout, mountedEngine, mountedTree, mountedPaint, mountedEvents, mountedSlots, mountedInterpreter, rendererGlue)) findings.push({ category: "blocking-bridge", file: "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs", line: 0, text: failure });
  lanes.interactivityMountedFrameTransactionSelfTests(repoRoot);
  const mountedFrameJob = policyReadFileSafe(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
  const mountedFrameHost = policyReadFileSafe(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
  const mountedFrameSnapshot = policyReadFileSafe(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs");
  const headlessUiRuntimeGlue = policyReadFileSafe(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️.rs");
  const mountedFrameShell = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_SHELL_FILE) + "\n" + policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_FIND_ITEMS_FILE);
  const mountedFrameServices = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_OS_SERVICES_FILE);
  const mountedFrameEngineCanvas = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE);
  const mountedFrameWorld3d = INTERACTIVITY_AUDIT_WORLD3D_FILES.map(file => policyReadRustPolicySource(repoRoot, file)).join("\n");
  const mountedFrameScenes = policyReadModuleTreeSource(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs");
  for (const failure of interactivityMountedFrameTransactionFailures(rendererGlue, mountedFrameJob, mountedFrameHost, mountedFrameSnapshot, headlessUiRuntimeGlue, mountedFrameShell, mountedFrameEngineCanvas, mountedFrameWorld3d, preparedRaster, preparedRasterGpu, preparedRasterDraw, mountedInterpreter, mountedEngine, mountedPaint, mountedSlots, mountedFrameScenes, mountedFrameServices)) findings.push({ category: "blocking-bridge", file: INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, line: 0, text: failure });
  const byCategory: Record<string, number> = {};
  for (const f of findings) byCategory[f.category] = (byCategory[f.category] ?? 0) + 1;

  const blockingBridgeFindings = findings.filter((f) => f.category === "blocking-bridge");
  const inScopeAllowlist = INTERACTIVITY_AUDIT_ALLOWLIST.filter((e) => e.inScope && !e.expectedNeverToMatch);
  const expectedNeverToMatchEntries = INTERACTIVITY_AUDIT_ALLOWLIST.filter((e) => e.inScope && e.expectedNeverToMatch);
  const preDeclaredOutOfScope = INTERACTIVITY_AUDIT_ALLOWLIST.filter((e) => !e.inScope);

  const blockingBridgeUnlisted = blockingBridgeFindings.filter((f) => !inScopeAllowlist.some((e) => e.file === f.file && f.text.includes(e.pattern)));
  const staleAllowlistEntries = inScopeAllowlist.filter((e) => !blockingBridgeFindings.some((f) => f.file === e.file && f.text.includes(e.pattern)));

  return { findings, byCategory, blockingBridgeUnlisted, staleAllowlistEntries, expectedNeverToMatchEntries, preDeclaredOutOfScope };
}
