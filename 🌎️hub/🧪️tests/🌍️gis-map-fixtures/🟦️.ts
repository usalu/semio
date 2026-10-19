/** 🧭️ GIS map approval, retained runtime, checkpoint, proposal and two-author composition fixtures.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { captureOwnedProcess } from "@semio-tech/process/execution";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { parseDocumentOpenPlanV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { assertObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES } from "../../💡️inference/🧬️schema/🟦️.ts";
import { readHubScriptModule } from "../../🧪️testing/📜️script-sources/🟦️.ts";
import { assertGisInferenceCheckpointControlFrame, finishGisMapProcessDocumentSocket, type GisMapProcessDocumentSocketV1 } from "../🎮️gis-map-process-support/🟦️.ts";
import { moduleRustSource, readHubBinSource, readHubInferenceRuntimeSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

/** ↩️ Pins witness-derived durable GIS approval undo and its tagged history dispatch. */
async function proveGisMapApprovalUndoFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "↩️gis-map-approval-undo-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateTarget = hubSchemaExport(repoRoot, "schema://hub.inference/GisMapApprovalUndoTargetV1");
  const validateRequest = hubSchemaExport(repoRoot, "schema://hub.inference/GisMapApprovalUndoRequestV1");
  if (!validateTarget(fixture.target)) throw new Error("GIS Map approval undo target is not a hub.inference/GisMapApprovalUndoTargetV1");
  if (!validateRequest(fixture.request)) throw new Error("GIS Map approval undo request is not a hub.inference/GisMapApprovalUndoRequestV1");
  if (fixture.schema !== "semio.hub.gis-map-approval-undo-fixture/v1") throw new Error("GIS Map approval undo fixture envelope drifted");
  if (fixture.traces.length !== 6 || new Set(fixture.traces.map((trace: { name: string }) => trace.name)).size !== 6 || fixture.sourceHostiles.length !== 8 || new Set(fixture.sourceHostiles).size !== 8)
    throw new Error("GIS Map approval undo fixture inventory drifted");
  if (fixture.request.targetId !== fixture.target.targetId) throw new Error("GIS Map approval undo request does not pin its own witness-derived target");
  const historyDispatch = [
    { tag: "localGuestTransaction", dispatch: "plugin-channel" },
    { tag: "hubGisMapApproval", dispatch: "history-undo-port" },
  ];
  if (JSON.stringify(fixture.historyMembers.map((member: { tag: string; dispatch: string }) => ({ tag: member.tag, dispatch: member.dispatch }))) !== JSON.stringify(historyDispatch))
    throw new Error("GIS Map approval undo history dispatch tags drifted");
  const hubMember = fixture.historyMembers[1];
  if (!/^https:\/\//.test(hubMember.hubOrigin) || hubMember.targetId !== fixture.target.targetId || hubMember.idempotencyKey !== fixture.request.idempotencyKey || hubMember.spaceId !== fixture.target.scope.spaceId || hubMember.documentId !== fixture.target.scope.documentId)
    throw new Error("GIS Map approval undo history member is not bound to the server-minted target");
  const genesisFirstUndo = {
    initialShape: "scope-bound-zero-frontier",
    firstCommandOrdinalDelta: 1,
    undoCommandOrdinalDelta: 1,
    restoresExactInitialSnapshot: true,
    restoresGenesisFrontier: false,
    syntheticGenesisEdit: false,
  };
  if (JSON.stringify(genesisFirstUndo) !== JSON.stringify(fixture.genesisFirstUndo)) throw new Error("GIS Map genesis-first undo contract mismatch");
  for (const trace of fixture.traces) {
    const owner = trace.actor === "owner";
    const exactTail = trace.current === "after";
    const replay = trace.current === "undone" && trace.idempotency === "same";
    const outcome = !owner ? "denied" : replay ? "applied" : !exactTail ? "conflict" : "applied";
    const actual = { outcome, walEvents: outcome === "applied" && !replay ? 1 : 0, publications: outcome === "applied" && !replay ? 1 : 0, replayed: replay };
    if (JSON.stringify(actual) !== JSON.stringify(trace.expected)) throw new Error(`GIS Map approval undo trace mismatch: ${trace.name}`);
  }
  const runtime = readHubInferenceRuntimeSource(repoRoot);
  const sqlite = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🪶️sqlite", "🦀️.rs"));
  const wal = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🧾️wal", "🦀️.rs"), "utf8");
  const mcpDispatch = join(repoRoot, "🧰️framework", "\u{1F6CD}\uFE0Fproducts", "💻️os", "🔨️modules", "🌉️mcp", "🔀️dispatch");
  const mcp = readFileSync(join(mcpDispatch, "🦀️.rs"), "utf8") + readFileSync(join(mcpDispatch, "🔌️channel-port", "🦀️.rs"), "utf8");
  const conforms = (runtimeSource: string, sqliteSource: string, walSource: string, mcpSource: string): boolean =>
    runtimeSource.includes("GisMapApprovalUndoTargetV1") &&
    runtimeSource.includes("undo_gis_map_approval") &&
    runtimeSource.includes("expected_current") &&
    runtimeSource.includes("checkpoint_applied") &&
    sqliteSource.includes("inference_approval_undo_v1") &&
    sqliteSource.includes("committed_witness_digest") &&
    sqliteSource.includes("idempotency_key") &&
    sqliteSource.includes("original_command") &&
    walSource.includes("approval_undo_witness_digest") &&
    mcpSource.includes("HubInferenceApproval(HubInferenceApprovalUndoMemberV1)") &&
    mcpSource.includes("HistoryUndoPort") &&
    !runtimeSource.includes("client_inverse") &&
    !/pub struct HubInferenceApprovalUndoMemberV1 \{[^}]*\b(?:instance|inverse)/u.test(mcpSource);
  if (!conforms(runtime, sqlite, wal, mcp)) throw new Error("durable GIS Map approval undo source boundary is incomplete");
  const hostiles = [
    [runtime.replaceAll("GisMapApprovalUndoTargetV1", "ClientSuppliedInverseV1"), sqlite, wal, mcp],
    [runtime, sqlite.replaceAll("committed_witness_digest", "unverified_witness"), wal, mcp],
    [runtime.replaceAll("expected_current", "advisory_current"), sqlite, wal, mcp],
    [runtime.replaceAll("checkpoint_applied", "publish_before_wal"), sqlite, wal, mcp],
    [runtime, sqlite.replaceAll("idempotency_key", "retry_key_ignored"), wal, mcp],
    [runtime, sqlite, wal.replaceAll("approval_undo_witness_digest", "raw_receipt_digest"), mcp],
    [runtime, sqlite, wal, mcp.replaceAll("HistoryUndoPort", "ArtifactChannel")],
    [runtime, sqlite, wal, mcp.replace("pub struct HubInferenceApprovalUndoMemberV1 {", "pub struct HubInferenceApprovalUndoMemberV1 {\n    pub instance: u32,")],
  ];
  hostiles.forEach(([candidateRuntime, candidateSqlite, candidateWal, candidateMcp], index) => {
    if (conforms(candidateRuntime!, candidateSqlite!, candidateWal!, candidateMcp!)) throw new Error(`GIS Map approval undo source oracle admitted ${fixture.sourceHostiles[index]}`);
  });
  console.log(`gis-map-approval-undo-oracle: AJV=1 exact=${fixture.traces.length} genesis-first=1 history=${fixture.historyMembers.length} hostile=${hostiles.length}`);
  return fixture.traces.length + fixture.historyMembers.length + hostiles.length + 1;
}

/** ⏯️ Pins the retained GIS inference worker lifecycle independently of the Rust runtime. */
async function proveGisInferenceRetainedRuntimeFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗺️gis-inference-retained-runtime-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const limits = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceLimitsV1");
  const canonicalLimits = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗳️gis-map-proposal-approval-v1", "🔣️.json"), "utf8")).limits;
  if (!limits(canonicalLimits)) throw new Error("retained GIS inference limits reference is not a hub.inference/InferenceLimitsV1");
  if (fixture.schema !== "semio.hub.gis-inference-retained-runtime/v1") throw new Error("retained GIS inference fixture envelope drifted");
  if (
    fixture.limits.operations !== canonicalLimits.operationCapacity ||
    fixture.limits.workUnits !== canonicalLimits.workUnitLimit ||
    fixture.limits.claimLeaseMs !== canonicalLimits.claimLeaseMaxMs ||
    fixture.limits.jobLifetimeMs !== canonicalLimits.jobMaxLifetimeMs ||
    fixture.limits.closeMs !== 5000
  )
    throw new Error("retained GIS inference limits disagree with the hub.inference limits contract");
  if (fixture.traces.length !== 9 || fixture.sourceHostiles.length !== 10 || new Set(fixture.sourceHostiles).size !== 10) throw new Error("retained GIS inference fixture inventory drifted");
  if (new Set(fixture.traces.map((trace: any) => trace.name)).size !== fixture.traces.length) throw new Error("retained GIS inference trace names are not unique");
  for (const trace of fixture.traces) {
    const events = ["accepted"];
    let retained = 1;
    let executions = 0;
    let offers = 0;
    let cancelled = false;
    let revoked = false;
    let refused = false;
    for (const step of trace.steps.slice(1)) {
      if (step === "spawn-refused") {
        refused = true;
        retained = 0;
        events.push("failed");
      } else if (step === "compute-enter") {
        if (!events.includes("running")) events.push("running");
        executions = 1;
      } else if ((step === "cancel" || step === "close") && !cancelled) {
        cancelled = true;
        events.push("cancel-requested", "cancelled");
      } else if (step === "revoke-author") revoked = true;
      else if (step === "terminal") {
        if (!cancelled && !refused) {
          events.push(revoked ? "cancelled" : "succeeded");
          offers = revoked ? 0 : 1;
        }
        retained = 0;
      }
    }
    const actual = {
      response: refused ? "capacity" : "running",
      events,
      retainedAtResponse: refused ? 0 : 1,
      retainedAtTerminal: retained,
      offers,
      executions,
    };
    if (JSON.stringify(actual) !== JSON.stringify(trace.expected)) throw new Error(`retained GIS inference trace mismatch: ${trace.name}`);
  }
  const runtime = readHubInferenceRuntimeSource(repoRoot);
  const sqlite = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🪶️sqlite", "🦀️.rs"));
  const conforms = (runtimeSource: string, sqliteSource: string): boolean => {
    const supervisor = runtimeSource.slice(runtimeSource.indexOf("struct InferenceRunKeyV1"), runtimeSource.indexOf("/// 🏃️ The hub's only inference authority"));
    const worker = runtimeSource.slice(runtimeSource.indexOf("async fn finish_retained_gis_map_job"), runtimeSource.indexOf("fn owner_page_receipt"));
    return (
      ["RetainedInferenceRunSupervisorV1", "InferenceRunKeyV1", "reap_finished_operations", "same_run", "signal_and_join_operations"].every((symbol) => supervisor.includes(symbol)) &&
      ["spawn_blocking", "fresh_now_ms", "compare_frozen", "check_live_inference_author"].every((symbol) => worker.includes(symbol)) &&
      !runtimeSource.includes("operations.insert(job_id.to_owned(), control)") &&
      !runtimeSource.includes("handle.abort()") &&
      sqliteSource.includes("pub fn heartbeat(") &&
      sqliteSource.includes("lease_expires_at") &&
      sqliteSource.includes("now >= lease") &&
      sqliteSource.includes("current_epoch != run_epoch")
    );
  };
  if (!conforms(runtime, sqlite)) throw new Error("retained GIS inference runtime source lost supervisor, heartbeat, join, or final authority fencing");
  const hostiles: [string,string][] = [
    [runtime.replaceAll("tokio::task::spawn_blocking", "inline_infer"), sqlite],
    [runtime.replaceAll("signal_and_join_operations", "abort_operations"), sqlite],
    [runtime.replaceAll("check_live_inference_author", "accept_author_without_check"), sqlite],
    [runtime.replaceAll("compare_frozen", "accept_base_without_check"), sqlite],
    [runtime.replaceAll("same_run", "same_job"), sqlite],
    [runtime.replaceAll("fresh_now_ms", "request_now_ms"), sqlite],
    [runtime, sqlite.replaceAll("pub fn heartbeat(", "pub fn stale_progress(")],
    [runtime, sqlite.replaceAll("now >= lease", "false")],
  ];
  hostiles.forEach(([runtimeSource, sqliteSource], index) => {
    if (runtimeSource === runtime && sqliteSource === sqlite) throw new Error(`retained GIS inference hostile ${index} did not mutate source`);
    if (conforms(runtimeSource, sqliteSource)) throw new Error(`retained GIS inference source oracle admitted hostile ${index}`);
  });
  return fixture.traces.length;
}

/** 🗺️ Independently pins the whole GIS Map proposal/approval contract without any Rust codec. */
async function proveGisInferenceCheckpointControlFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "⏸️gis-inference-checkpoint-control-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateFrame = hubSchemaExport(repoRoot, "schema://hub.inference/GisInferenceCheckpointControlFrameV1");
  const validateDirection = hubSchemaExport(repoRoot, "schema://hub.inference/GisInferenceCheckpointControlDirectionV1");
  if (fixture.schema !== "semio.hub.gis-inference-checkpoint-control-fixture/v1" || fixture.version !== 1) throw new Error("GIS inference checkpoint control fixture envelope drifted");
  const expectedHostiles = ["wrong-schema", "wrong-version", "wrong-sequence", "wrong-kind", "zero-progress-cursor", "zero-completed", "completed-past-total", "unknown-field", "zero-length", "oversized-frame", "closed-before-release"];
  if (JSON.stringify(fixture.hostiles) !== JSON.stringify(expectedHostiles) || fixture.descriptor !== 4 || fixture.frameMaximumBytes !== GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES) {
    throw new Error("GIS inference checkpoint control fixture lost its closed bounded protocol");
  }
  if (JSON.stringify(fixture.frames.map((row: any) => row.direction)) !== JSON.stringify(["hub-to-runner", "runner-to-hub"])) throw new Error("GIS inference checkpoint control frame directions drifted");
  for (const row of fixture.frames) {
    if (!validateDirection(row.direction)) throw new Error("GIS inference checkpoint control direction is not a hub.inference/GisInferenceCheckpointControlDirectionV1");
    if (!validateFrame(row.frame)) throw new Error("GIS inference checkpoint control frame is not a hub.inference/GisInferenceCheckpointControlFrameV1");
  }
  const [entered, release] = fixture.frames.map((row: any) => row.frame);
  assertGisInferenceCheckpointControlFrame(entered, 1, "progress-persisted", entered.jobId);
  assertGisInferenceCheckpointControlFrame(release, 2, "release", entered.jobId);
  if (JSON.stringify({ progressCursor: entered.progressCursor, completed: entered.completed, total: entered.total }) !== JSON.stringify({ progressCursor: release.progressCursor, completed: release.completed, total: release.total })) {
    throw new Error("GIS inference checkpoint release does not echo the exact persisted progress identity");
  }
  const malformed = [
    { ...entered, schema: `${entered.schema}.substituted` },
    { ...entered, version: 2 },
    { ...entered, sequence: 2 },
    { ...entered, kind: "release" },
    { ...entered, progressCursor: 0 },
    { ...entered, completed: 0 },
    { ...entered, completed: entered.total + 1 },
    { ...entered, unknown: true },
    { ...entered, jobId: "f".repeat(GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES) },
  ];
  for (const [index, frame] of malformed.entries()) {
    try {
      assertGisInferenceCheckpointControlFrame(frame, 1, "progress-persisted", entered.jobId);
    } catch {
      continue;
    }
    throw new Error(`GIS inference checkpoint control exact oracle admitted hostile ${index}`);
  }
  const runtime = readHubInferenceRuntimeSource(repoRoot);
  const hubBin = readHubBinSource(repoRoot);
  const runner = readFileSync(join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust", "📜️script.ts"), "utf8");
  // 🧬️The frame TYPE is a `hub.inference` scope export and is declared in the scope module; the
  // runtime owns only the framing that reads and writes it over the inherited descriptor.
  const scopeModule = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🧬️schema", "🦀️.rs"), "utf8");
  for (const symbol of ["pub struct GisInferenceCheckpointControlFrameV1", 'serde(deny_unknown_fields, rename_all = "camelCase")'])
    if (!scopeModule.includes(symbol)) throw new Error(`hub.inference scope module is missing ${symbol}`);
  for (const symbol of [
    "pub fn open_inherited",
    "write_inference_checkpoint_control_frame",
    "read_inference_checkpoint_control_frame",
    "GisInferenceCheckpointControlFrameV1",
    "INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES: usize = 256",
    "control.checkpoint(control.progress().0)",
    "progress_persisted(job_id, progress_cursor, completed, total)",
  ])
    if (!runtime.includes(symbol)) throw new Error(`GIS inference checkpoint runtime is missing ${symbol}`);
  if (!hubBin.includes('std::env::var_os("OS_HUB_TEST_INFERENCE_CHECKPOINT_FD")') || !hubBin.includes('descriptor.to_str() != Some("4")') || !hubBin.includes("mode != HubMode::Development")) {
    throw new Error("GIS inference checkpoint control is not integration-fixtures-only on fixed inherited descriptor 4");
  }
  for (const symbol of ["inferenceCheckpointControl: true", "waitForGisInferenceCheckpointControl(run, jobIdB)", "releaseGisInferenceCheckpointControl(run, jobIdB)", 'name: "inference_cancel"'])
    if (!runner.includes(symbol)) throw new Error(`GIS inference process runner is missing ${symbol}`);
  const heartbeat = runtime.indexOf("ledger().heartbeat(");
  const pause = runtime.indexOf("gate.checkpoint(&owner.control", heartbeat);
  if (heartbeat < 0 || pause <= heartbeat) throw new Error("GIS inference test checkpoint is not downstream of its durable heartbeat");
  console.log(`gis-inference-checkpoint-control-oracle: ajv=1 exact=2 hostile=${fixture.hostiles.length} fd=4 post-heartbeat=1 integration-fixtures-only=1 passed`);
  return fixture.hostiles.length;
}

export async function proveGisMapProposalApprovalFixture(repoRoot: string): Promise<number> {
  const checkpointControlCases = await proveGisInferenceCheckpointControlFixture(repoRoot);
  const retainedRuntimeCases = await proveGisInferenceRetainedRuntimeFixture(repoRoot);
  const durableUndoCases = await proveGisMapApprovalUndoFixture(repoRoot);
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗳️gis-map-proposal-approval-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateBinding = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceBindingIdentityV1");
  const validateLimits = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceLimitsV1");
  const validatePreview = hubSchemaExport(repoRoot, "schema://hub.inference/GisMapInferencePreviewV1");
  const validateSummary = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceMapSummaryV1");
  const validateLifecycleKind = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceLifecycleKindV1");
  if (fixture.schema !== "semio.hub.gis-map-proposal-approval-fixture/v1" || fixture.version !== 1) throw new Error("GIS Map proposal fixture envelope drifted");
  if (!validateBinding(fixture.binding)) throw new Error("GIS Map proposal binding is not a hub.inference/InferenceBindingIdentityV1");
  if (!validateLimits(fixture.limits)) throw new Error("GIS Map proposal limits are not a hub.inference/InferenceLimitsV1");
  if (!validatePreview(fixture.preview)) throw new Error("GIS Map proposal preview is not a hub.inference/GisMapInferencePreviewV1");
  if (!validateSummary(fixture.base.expectedInference)) throw new Error("GIS Map proposal base summary is not a hub.inference/InferenceMapSummaryV1");
  for (const event of [...fixture.lifecycle, ...fixture.cancelLifecycle]) if (!validateLifecycleKind(event.kind)) throw new Error(`GIS Map proposal lifecycle kind ${event.kind} is not a hub.inference/InferenceLifecycleKindV1`);
  const scalarLaws: readonly (readonly [string, unknown])[] = [
    ["limits.proposalMaxBytes", 4096],
    ["binding.grantedMode", "read-write-observe"],
    ["binding.surfaceId", "s.gis.gismap@1/*#editor"],
    ["binding.parentDialect.subset", "*"],
    ["committer.capacity", 64],
    ["committer.witnessSource", "sole-committed-wal-event"],
    ["committer.retryOwner", "same-document-three-store-host"],
    ["committer.frontierChain", "canonical-lowercase-hex-with-genesis-zero"],
    ["committer.documentFence", "mounted-actor-generation-including-zero"],
    ["committer.genesisPublication", "active-indexed-genesis-parent-before-first-edit"],
    ["committer.beforePublicAck.ledgerApplied", false],
    ["committer.beforePublicAck.documentWrite", "retained"],
    ["committer.beforePublicAck.state", "publishing"],
    ["committer.publicationFailure.ledgerApplied", false],
    ["committer.publicationFailure.documentWrite", "retained"],
    ["committer.publicationFailure.retry", "same-event-same-owner"],
    ["committer.publicationFailure.retryDeadline", "process-owned-bounded-attempt"],
    ["committer.ingressAuthority.owner", "hub-private-sorted-binding-guards"],
    ["committer.ingressAuthority.documentWrite", "runtime-acquired"],
    ["committer.ingressAuthority.cutover", "durable-wal-receipt"],
    ["committer.ingressAuthority.delivery", "fresh-revalidation-under-same-guards"],
    ["committer.ingressAuthority.recovery", "committed-wal-provenance"],
    ["committer.abandonedRequest.owner", "runtime-document-maintenance"],
    ["committer.abandonedRequest.preparedOwner", "armed-before-committer-preflight"],
    ["committer.abandonedRequest.driver", "document-write-request-token-single-poller"],
    ["committer.abandonedRequest.cleanupCapacity", 128],
    ["committer.abandonedRequest.runtimeShutdown", "reinsert-exact-owner-before-task-drop"],
    ["committer.abandonedRequest.terminalClose", "fence-new-admission-until-cleanup-empty"],
    ["committer.abandonedRequest.outbox", "abandoned-until-fresh-identical-retry"],
    ["committer.abandonedRequest.commitCutover", "awaiting-ack"],
    ["committer.storeIdentities.parentDialect", "s.gis.gismap@1/*"],
    ["committer.storeIdentities.childOwnerParent", "exact-document-parent-reference"],
    ["committer.peerNotification.frame", "rebootstrap-required"],
    ["committer.peerNotification.source", "active-checkpoint"],
    ["committer.peerNotification.command", "server-stamped-create-region"],
    ["committer.peerNotification.peers", 2],
    ["committer.peerNotification.publicationFailure", "none"],
    ["committer.peerNotification.publicationCancellation", "none"],
  ];
  for (const [name, expected] of scalarLaws) {
    const actual = name.split(".").reduce<any>((at, key) => at?.[key], fixture);
    if (actual !== expected) throw new Error(`GIS Map proposal committer law ${name} is ${JSON.stringify(actual)}, expected ${JSON.stringify(expected)}`);
  }
  const orderedLaws: readonly (readonly [string, readonly unknown[]])[] = [
    ["committer.roles", ["map", "drawing", "value"]],
    ["committer.children", ["gismap-drawing", "gismap-value"]],
    ["committer.phases", ["ready", "assembly", "journal", "verification", "publishing", "published", "closing"]],
    ["committer.publicationOrder", ["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"]],
    ["committer.peerNotification.after", ["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"]],
    ["committer.ingressAuthority.bindings", ["user", "session", "space-authority", "membership"]],
    ["committer.abandonedRequest.abortPhases", ["no-runtime-unpolled", "unpolled", "rejected-preflight", "ready", "preflight", "assembly", "journal-no-receipt"]],
    ["committer.abandonedRequest.postCutover", ["verification", "public-checkpoint", "ledger-applied"]],
    ["committer.storeIdentities.childReferences", ["gismap-drawing!s.stdio.semio@v1/drawing", "gismap-value!s.stdio.semio@v1/value"]],
    ["committer.storeIdentities.childOwnerRoles", ["drawing", "value"]],
    ["nonclaims", ["no-external-model-provider", "no-wgpu-rendering", "no-client-supplied-map-pack", "no-auto-apply"]],
  ];
  for (const [name, expected] of orderedLaws) {
    const actual = name.split(".").reduce<any>((at, key) => at?.[key], fixture);
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`GIS Map proposal ordered law ${name} is ${JSON.stringify(actual)}, expected ${JSON.stringify(expected)}`);
  }
  const committerLaws = scalarLaws.length + orderedLaws.length;
  if (fixture.lifecycle.length !== 5 || fixture.cancelLifecycle.length !== 4 || fixture.visibility.length !== 7 || fixture.errors.length !== 11 || fixture.approvalRejections.length !== 11 || new Set(fixture.nonclaims).size !== 4)
    throw new Error("GIS Map proposal fixture inventory drifted");
  for (const row of [...fixture.errors, ...fixture.approvalRejections, ...fixture.visibility.filter((entry: { expectedCode: string | null }) => entry.expectedCode !== null).map((entry: { expectedCode: string }) => ({ code: entry.expectedCode }))])
    if (!/^(inference|approval)\.[a-z-]+$/.test(row.code)) throw new Error(`GIS Map proposal error code ${row.code} is outside the closed inference/approval namespace`);
  const hash = (value: string): string => createHash("sha256").update(value, "utf8").digest("hex");
  if (hash(fixture.proposalCanonical) !== fixture.proposalHash || hash(fixture.inverseCanonical) !== fixture.inverseHash) throw new Error("independent proposal/inverse canonical hashes differ");
  const proposal = JSON.parse(fixture.proposalCanonical);
  const inverse = JSON.parse(fixture.inverseCanonical);
  const region = proposal.CreateRegion;
  const regionId = `inference-${fixture.sampleJobId}`;
  if (Object.keys(proposal).length !== 1 || !region || region.index !== fixture.base.snapshot.regions.length || region.item.id !== regionId || region.item.data.id !== regionId || region.item.data.kind !== "inference-bounds") {
    throw new Error("proposal is not exactly one server-stamped CreateRegion for this job");
  }
  if (!Array.isArray(inverse) || inverse.length !== 1 || Object.keys(inverse[0]).length !== 1 || inverse[0].DeleteRegion?.id !== regionId) throw new Error("inverse is not exactly one DeleteRegion for the created id");
  const points: number[][] = [];
  const scan = (value: unknown): void => {
    if (Array.isArray(value)) {
      if (value.length === 2 && value.every((part) => typeof part === "number" && Number.isFinite(part))) points.push(value as number[]);
      else value.forEach(scan);
    } else if (typeof value === "object" && value !== null) {
      const row = value as Record<string, unknown>;
      if (typeof row.lon === "number" && Number.isFinite(row.lon) && typeof row.lat === "number" && Number.isFinite(row.lat)) points.push([row.lon, row.lat]);
      Object.values(row).forEach(scan);
    }
  };
  for (const item of [...fixture.base.snapshot.positions, ...fixture.base.snapshot.routes, ...fixture.base.snapshot.regions]) scan(item.data);
  const fold = points.reduce((box, [lon, lat]) => [Math.min(box[0]!, lon!), Math.max(box[1]!, lon!), Math.min(box[2]!, lat!), Math.max(box[3]!, lat!)], [Infinity, -Infinity, Infinity, -Infinity]);
  const sorted = { lon: [...points.map((point) => point[0]!)].sort((a, b) => a - b), lat: [...points.map((point) => point[1]!)].sort((a, b) => a - b) };
  const bounds = { lonMin: sorted.lon[0]!, lonMax: sorted.lon[sorted.lon.length - 1]!, latMin: sorted.lat[0]!, latMax: sorted.lat[sorted.lat.length - 1]! };
  if (fold[0] !== bounds.lonMin || fold[1] !== bounds.lonMax || fold[2] !== bounds.latMin || fold[3] !== bounds.latMax) throw new Error("two independent bound folds disagree");
  const expected = fixture.base.expectedInference;
  if (JSON.stringify({ positionCount: fixture.base.snapshot.positions.length, routeCount: fixture.base.snapshot.routes.length, regionCount: fixture.base.snapshot.regions.length, bounds }) !== JSON.stringify(expected))
    throw new Error("independent counts/bounds differ from the fixture");
  const ring: number[][] = region.item.data.ring;
  const corners = [
    [bounds.lonMin, bounds.latMin],
    [bounds.lonMax, bounds.latMin],
    [bounds.lonMax, bounds.latMax],
    [bounds.lonMin, bounds.latMax],
    [bounds.lonMin, bounds.latMin],
  ];
  if (ring.length !== 5 || JSON.stringify(ring) !== JSON.stringify(corners)) throw new Error("proposal ring is not the closed bounds rectangle of the base snapshot");
  if (
    fixture.preview.schema !== "semio.hub.gis-map-inference-preview/v1" ||
    fixture.preview.jobId !== fixture.sampleJobId ||
    fixture.preview.proposalHash !== fixture.proposalHash ||
    fixture.preview.regionId !== regionId ||
    JSON.stringify(fixture.preview.ring) !== JSON.stringify(corners)
  ) {
    throw new Error("owner preview is not the exact bounded projection of the canonical proposal");
  }
  const transitions: Record<string, readonly string[]> = {
    accepted: ["running", "cancel-requested", "cancelled", "failed"],
    running: ["succeeded", "cancel-requested", "cancelled", "failed"],
    succeeded: ["approval-prepared", "cancel-requested", "proposal-cancelled", "proposal-stale"],
    "cancel-requested": ["cancelled", "proposal-cancelled"],
    "approval-prepared": ["approved"],
  };
  for (const trace of [fixture.lifecycle, fixture.cancelLifecycle]) {
    if (trace[0].kind !== "accepted" || trace[0].ordinal !== 1) throw new Error("every private job stream starts at accepted");
    for (let index = 1; index < trace.length; index++) {
      if (trace[index].ordinal !== index + 1) throw new Error("private job event ordinals are not dense");
      if (!(transitions[trace[index - 1].kind] ?? []).includes(trace[index].kind)) throw new Error(`illegal private job transition ${trace[index - 1].kind} -> ${trace[index].kind}`);
    }
    if (trace.length > 6) throw new Error("private job stream exceeds its bounded event ordinal");
  }
  const owners = fixture.visibility.filter((row: { readProposal: boolean }) => row.readProposal);
  if (owners.length !== 1 || owners[0].role !== "author-owner" || !owners[0].approve || owners[0].expectedCode !== null) throw new Error("exactly one original Author owner may read and approve");
  for (const row of fixture.visibility) if (row.role !== "author-owner" && (row.readEvents || row.readProposal || row.approve || row.expectedCode !== "inference.denied")) throw new Error(`non-owner ${row.role} was granted private job access`);
  const statuses = new Map<string, number>();
  for (const row of fixture.errors) {
    if (statuses.has(row.code) && statuses.get(row.code) !== row.status) throw new Error(`error code ${row.code} maps to two statuses`);
    statuses.set(row.code, row.status);
  }
  if (statuses.get("approval.commit-unavailable") !== 503) throw new Error("a missing composition transaction must fail closed with 503");
  for (const rejection of fixture.approvalRejections) if (!statuses.has(rejection.code)) throw new Error(`approval rejection ${rejection.name} uses an unpublished code`);
  const ledger = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
  const committer = fixture.committer;
  if (
    committer.capacity !== fixture.limits.documentGateCapacity ||
    JSON.stringify(committer.roles) !== JSON.stringify(["map", "drawing", "value"]) ||
    JSON.stringify(committer.children) !== JSON.stringify(["gismap-drawing", "gismap-value"]) ||
    JSON.stringify(committer.phases) !== JSON.stringify(["ready", "assembly", "journal", "verification", "publishing", "published", "closing"]) ||
    committer.witnessSource !== "sole-committed-wal-event" ||
    committer.retryOwner !== "same-document-three-store-host" ||
    committer.frontierChain !== "canonical-lowercase-hex-with-genesis-zero" ||
    ledger.identity.chainHash !== "0".repeat(64) ||
    JSON.stringify(committer.storeIdentities) !==
      JSON.stringify({
        parentDialect: "s.gis.gismap@1/*",
        childReferences: ["gismap-drawing!s.stdio.semio@v1/drawing", "gismap-value!s.stdio.semio@v1/value"],
        childOwnerRoles: ["drawing", "value"],
        childOwnerParent: "exact-document-parent-reference",
      }) ||
    JSON.stringify(committer.publicationOrder) !== JSON.stringify(["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"]) ||
    JSON.stringify(committer.beforePublicAck) !== JSON.stringify({ ledgerApplied: false, documentWrite: "retained", state: "publishing" }) ||
    JSON.stringify(committer.publicationFailure) !== JSON.stringify({ ledgerApplied: false, documentWrite: "retained", retry: "same-event-same-owner", retryDeadline: "process-owned-bounded-attempt" }) ||
    JSON.stringify(committer.ingressAuthority) !==
      JSON.stringify({
        owner: "hub-private-sorted-binding-guards",
        bindings: ["user", "session", "space-authority", "membership"],
        documentWrite: "runtime-acquired",
        cutover: "durable-wal-receipt",
        delivery: "fresh-revalidation-under-same-guards",
        recovery: "committed-wal-provenance",
      }) ||
    JSON.stringify(committer.abandonedRequest) !==
      JSON.stringify({
        owner: "runtime-document-maintenance",
        preparedOwner: "armed-before-committer-preflight",
        driver: "document-write-request-token-single-poller",
        cleanupCapacity: 128,
        runtimeShutdown: "reinsert-exact-owner-before-task-drop",
        terminalClose: "fence-new-admission-until-cleanup-empty",
        abortPhases: ["no-runtime-unpolled", "unpolled", "rejected-preflight", "ready", "preflight", "assembly", "journal-no-receipt"],
        outbox: "abandoned-until-fresh-identical-retry",
        commitCutover: "awaiting-ack",
        postCutover: ["verification", "public-checkpoint", "ledger-applied"],
      }) ||
    committer.documentFence !== "mounted-actor-generation-including-zero" ||
    committer.genesisPublication !== "active-indexed-genesis-parent-before-first-edit" ||
    JSON.stringify(committer.peerNotification) !==
      JSON.stringify({
        after: ["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"],
        frame: "rebootstrap-required",
        source: "active-checkpoint",
        command: "server-stamped-create-region",
        peers: 2,
        publicationFailure: "none",
        publicationCancellation: "none",
      })
  )
    throw new Error("retained fixed-three committer contract drifted");
  const runtime = readHubInferenceRuntimeSource(repoRoot);
  const gisBinding = readModuleTreeSource(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"));
  if (runtime.includes("semio_s_artifact_") || !runtime.includes("impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B>"))
    throw new Error("the retained committer names an artifact crate instead of staying generic over the registered composed-map binding");
  for (const symbol of ["impl ComposedMapCommitBindingV1 for GisMapInferenceBindingV1", "impl MapInferenceProposalBindingV1 for GisMapInferenceBindingV1", "gis_map_parent_stamped_one_item_preparation_factory"])
    if (!gisBinding.includes(symbol)) throw new Error(`GIS Map inference binding source is missing ${symbol}`);
  for (const symbol of [
    "RetainedGisMapApprovalCommitterV1",
    "RetainedGisMapDocumentStateV1",
    "DurableOwnedThreeStoreMapAssemblyV1",
    "durable_group_journal_sink",
    "InferenceWalVerifierV1",
    "parse_frontier_chain_hash",
    "GisMapApprovalCheckpointPublisherV1",
    "checkpoint_applied",
    "GisMapApprovalIngressAuthorityV1",
    "GisMapApprovalRequestOwnerV1",
    "GisMapPreparedApprovalV1",
    "GisMapAbandonedRequestTaskV1",
    "GisMapPreparedApprovalTaskV1",
    "resume_parked_requests",
    "resume_parked_prepared",
    "drive_abandoned_request",
    "drive_abandoned_prepared",
    "abandon_prepared_approval",
    "owners.fence.clone()",
    "RetainedGisMapDocumentStateV1::Publishing",
    "B::parent_preparation_factory(",
    "sole committed WAL event",
  ])
    if (!runtime.includes(symbol)) throw new Error(`retained GIS Map committer source is missing ${symbol}`);
  const walRuntime = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🧾️wal", "🦀️.rs"), "utf8");
  if (!walRuntime.includes("active: AtomicBool") || !walRuntime.includes("self.active.store(false, Ordering::Release)") || walRuntime.includes("self.generation.store(0, Ordering::Release)")) {
    throw new Error("mounted actor generation zero is not independently fenced from invalidation");
  }
  const sqliteRuntime = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🪶️sqlite", "🦀️.rs"));
  if (sqliteRuntime.includes("document_generation == 0") || !sqliteRuntime.includes("witness.matches(&scope, document_generation")) {
    throw new Error("initial actor generation zero cannot reach exact fenced ledger reconciliation");
  }
  const retainedCommitter = runtime.slice(runtime.indexOf("pub struct RetainedGisMapApprovalCommitterV1"), runtime.indexOf("/// 🗺️ The server-materialized Map base"));
  const storeConstruction = gisBinding.slice(gisBinding.indexOf("fn parent_store("), gisBinding.indexOf("pub fn gis_map_region_preview("));
  const verify = retainedCommitter.slice(retainedCommitter.indexOf("async fn verify("), retainedCommitter.indexOf("async fn commit_retained("));
  const publishCall = verify.indexOf("self.publish_checkpoint(");
  const reconcileCall = verify.indexOf(".reconcile_committed_approval(");
  const checkpointAppliedCall = verify.indexOf(".checkpoint_applied(");
  const documentWriteRelease = verify.indexOf("drop(document_write)");
  if (
    !runtime.includes("const GIS_MAP_COMMITTER_CAPACITY: usize = 64;") ||
    !storeConstruction.includes('artifact_kind: "s.gis.gismap"') ||
    !storeConstruction.includes('artifact_kind: "s.stdio.semio"') ||
    !storeConstruction.includes('slot: "drawing"') ||
    !storeConstruction.includes('slot: "value"') ||
    !gisBinding.includes("let drawing_id = snapshot.drawing.child_id.clone();") ||
    !gisBinding.includes("let value_id = snapshot.value.child_id.clone();") ||
    !retainedCommitter.includes("B::member_stores(&key, snapshot, actor)") ||
    retainedCommitter.includes("ArtifactHandle::submit") ||
    publishCall < 0 ||
    reconcileCall <= publishCall ||
    !retainedCommitter.includes("document_write: GisMapDocumentWriteLeaseV1") ||
    checkpointAppliedCall <= reconcileCall ||
    documentWriteRelease <= checkpointAppliedCall
  )
    throw new Error("retained GIS Map committer capacity, write authority, typed journal, or publication-before-apply boundary drifted");
  const hubBin = readHubBinSource(repoRoot);
  // 🧪️The binary's own laws live beside it in `🌎️hub/🧪️tests/**`, not inside `🏗️bootstrap/🦀️.rs`, so the law
  // names are asserted against the whole test tree — a further test-layout move cannot silently red this.
  const hubBinLaws = moduleRustSource(join(repoRoot, "🌎️hub", "🧪️tests"));
  for (const symbol of ["HubGisMapApprovalIngressAuthorityV1", "acquire_gis_map_approval_ingress", "revalidate_gis_map_approval_authority", "InferenceApprovalRouteContextV1", "inference_runtime.close().await", "publish_gis_map_checkpoint_change"])
    if (!hubBin.includes(symbol)) throw new Error(`Hub GIS Map approval ingress source is missing ${symbol}`);
  for (const law of ["gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write", "gis_map_applied_checkpoint_notifies_two_peers_with_one_exact_rebootstrap_pair"])
    if (!hubBinLaws.includes(law)) throw new Error(`Hub GIS Map approval ingress laws are missing ${law}`);
  const currentMatchesBase = hubBin.slice(hubBin.indexOf("fn current_matches_base("), hubBin.indexOf("impl GisMapApprovalCheckpointPublisherV1 for GisMapApprovalCheckpointPublisherV1Impl"));
  if (
    !hubBin.includes("Self::current_matches_base(&request.scope") ||
    !currentMatchesBase.includes("Some(checkpoint) => checkpoint.scope == *scope && checkpoint.baseline_frontier == *base") ||
    !currentMatchesBase.includes("None => false")
  ) {
    throw new Error("Hub GIS Map checkpoint publication does not require the exact active Directory genesis or edited parent");
  }
  const ingressSource = hubBin.slice(hubBin.indexOf("async fn acquire_gis_map_approval_ingress"), hubBin.indexOf("fn inference_context"));
  const ingressBindings = ["SocketBindingKeyV1::User", "SocketBindingKeyV1::Session", "SocketBindingKeyV1::DirectorySpaceAuthority", "SocketBindingKeyV1::Membership"];
  let ingressCursor = -1;
  for (const binding of ingressBindings) {
    const index = ingressSource.indexOf(binding);
    if (index <= ingressCursor) throw new Error(`Hub approval ingress does not acquire ${binding} in the shared total order`);
    ingressCursor = index;
  }
  if (ingressSource.includes("SocketBindingKeyV1::DocumentWrite")) throw new Error("Hub approval ingress outer-locks DocumentWrite before the retained runtime");
  const approvalHandler = hubBin.slice(hubBin.indexOf("async fn post_inference_gis_map_job_approval"), hubBin.indexOf("fn inference_routes"));
  const acquired = approvalHandler.indexOf("acquire_gis_map_approval_ingress(");
  if (!approvalHandler.includes("InferenceApprovalRouteContextV1 { route, ingress }") || acquired < 0 || acquired > approvalHandler.indexOf("approve_gis_map_job") || ingressSource.indexOf("revalidate_gis_map_approval_authority(") <= ingressSource.indexOf("share_bindings("))
    throw new Error("Hub approval route does not retain ingress through the lower call or revalidate under its retained guards");
  const terminalClose = hubBin.lastIndexOf("inference_runtime.close().await");
  const artifactClose = hubBin.lastIndexOf("artifact_maintenance.shutdown().await");
  if (terminalClose < 0 || artifactClose <= terminalClose) throw new Error("Hub process teardown does not close inference before artifact/process owners");
  const inferenceRegistration = hubBin.slice(hubBin.indexOf("let inference_runtime = match gis_map_binding.as_ref()"), hubBin.indexOf("let inference_ready = inference_runtime.is_some()"));
  if (!inferenceRegistration.includes("RetainedGisMapApprovalCommitterV1::new") || !inferenceRegistration.includes("GisMapApprovalCheckpointPublisherV1Impl") || inferenceRegistration.includes("UnavailableGisMapApprovalCommitterV1"))
    throw new Error("production GIS Map approval did not register its retained three-Store checkpoint committer");
  const frozen = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🧊️gis-map-frozen-binding-v1", "🔣️.json"), "utf8"));
  if (
    fixture.binding.digest !== frozen.expectedDigest ||
    fixture.binding.componentBlake3 !== frozen.binding.package.componentBlake3 ||
    fixture.binding.packageVersion !== frozen.binding.package.version ||
    JSON.stringify(fixture.binding) !== JSON.stringify(ledger.identity.binding)
  )
    throw new Error("proposal, frozen-binding and ledger corpora disagree on the frozen executable identity");
  const runner = readFileSync(join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust", "📜️script.ts"), "utf8");
  const processStart = runner.indexOf("\nasync function proveGisMapProposalProcess");
  const processEnd = runner.indexOf("\n/** 🗺️ The exact GIS Map proposal/approval gate", processStart);
  const cancellationStart = runner.indexOf("\nasync function cancelGisMapProcessMcpOwner(");
  const cancellationEnd = runner.indexOf("\n}\n", cancellationStart);
  const processSource = processStart >= 0 && processEnd > processStart && cancellationStart >= 0 && cancellationEnd > cancellationStart
    ? runner.slice(processStart, processEnd).replace("const jobIdB = await cancelGisMapProcessMcpOwner(run, mcpB, fixture.documentId);", runner.slice(cancellationStart, cancellationEnd)) : "";
  const processConforms = (source: string): boolean => {
    const materialize = source.indexOf("await withMaterializedTrustedCatalogBundleV1(");
    const validate = source.indexOf("await validateAndPublishTrustedStdioGisCandidate(");
    const current = source.indexOf("const current = trustedBootstrapCurrent(dataRoot)");
    const publication = source.indexOf("await checkInProcessHeadV1(");
    const peerSubmit = source.indexOf('name: "inference_submit"');
    const checkpointEntered = source.indexOf("await waitForGisInferenceCheckpointControl(");
    const persistedProgress = source.indexOf("const observedB = await callGisMapProcessMcp(", checkpointEntered);
    const peerCancel = source.indexOf('name: "inference_cancel"');
    const checkpointRelease = source.indexOf("await releaseGisInferenceCheckpointControl(");
    const peerCancelled = source.indexOf('page.state === "cancelled"');
    const ownerSubmit = source.indexOf("const submittedA = await gisMapProcessHttp(");
    const approval = source.indexOf('"POST", `${inferenceRoot}/${jobId}/approval`');
    const approvalRebootstrap = source.indexOf('"Author A RebootstrapRequired"');
    const refreshed = source.indexOf("const refreshedA = await readGisMapProcessCheckpoint(");
    const undo = source.indexOf('"POST", undoRoot, undoBody');
    const undoRebootstrap = source.indexOf('"Author A undo RebootstrapRequired"');
    const undone = source.indexOf("const undoneA = await readGisMapProcessCheckpoint(");
    const undoReplay = source.indexOf("const replay = await gisMapProcessHttp(");
    return (
      materialize >= 0 &&
      validate > materialize &&
      current > validate &&
      publication > current &&
      peerSubmit > publication &&
      checkpointEntered > peerSubmit &&
      persistedProgress > checkpointEntered &&
      peerCancel > persistedProgress &&
      checkpointRelease > peerCancel &&
      peerCancelled > checkpointRelease &&
      ownerSubmit > peerCancelled &&
      approval > ownerSubmit &&
      approvalRebootstrap > approval &&
      refreshed > approvalRebootstrap &&
      undo > refreshed &&
      undoRebootstrap > undo &&
      undone > undoRebootstrap &&
      undoReplay > undone &&
      (source.match(/await startGisMapProcessMcpClient\(/gu)?.length ?? 0) === 2 &&
      (source.match(/await openGisMapProcessDocumentSocket\(/gu)?.length ?? 0) === 2 &&
      source.includes("inferenceCheckpointControl: true") &&
      source.includes('selected.executionProtocol?.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1') &&
      source.includes("cancelledBPage?.jobId !== jobIdB") &&
      source.includes("observedProgress.completed !== persistedProgress.completed") &&
      source.includes("observedProgress.total !== persistedProgress.total") &&
      source.includes("observedBPage.progress.length > 16") &&
      source.includes('!cancelledKinds.includes("cancel-requested")') &&
      source.includes('!cancelledKinds.includes("cancelled")') &&
      source.includes('terminalB?.proposalHash !== null') &&
      source.includes('denied.value?.code !== "inference.denied"') &&
      source.includes("JSON.stringify(refreshedA) !== JSON.stringify(refreshedB)") &&
      source.includes("JSON.stringify(undoHandle.expectedCurrent) !== JSON.stringify(refreshedA.frontier)") &&
      source.includes("peerUndo.status !== 403") &&
      source.includes("staleUndo.status !== 409") &&
      source.includes("undone.value?.replayed !== false") &&
      source.includes("JSON.stringify(undoneA) !== JSON.stringify(undoneB)") &&
      source.includes("undoneA.packSha256 !== initialA.packSha256") &&
      source.includes("replay.value?.replayed !== true") &&
      source.includes('"no-shell-scene"') &&
      !source.includes('"no-durable-collaborative-redo"')
    );
  };
  if (!processConforms(processSource)) throw new Error("GIS Map proposal process source lost its compiled-generation, two-Author, private-owner, or durable-pair fence");
  const processHostiles = [
    processSource.replace("await validateAndPublishTrustedStdioGisCandidate(", "void validateAndPublishTrustedStdioGisCandidate("),
    processSource.replace("selected.executionProtocol?.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1", "false"),
    processSource.replace(/await startGisMapProcessMcpClient\(/u, "await Promise.resolve("),
    processSource.replace(/await openGisMapProcessDocumentSocket\(/u, "await Promise.resolve("),
    processSource.replace("inferenceCheckpointControl: true", "inferenceCheckpointControl: false"),
    processSource.replace('name: "inference_submit"', 'name: "inference_events"'),
    processSource.replace("await waitForGisInferenceCheckpointControl(", "void waitForGisInferenceCheckpointControl("),
    processSource.replace("observedProgress.completed !== persistedProgress.completed", "false"),
    processSource.replace('name: "inference_cancel"', 'name: "inference_events"'),
    processSource.replace("await releaseGisInferenceCheckpointControl(", "void releaseGisInferenceCheckpointControl("),
    processSource.replace('page.state === "cancelled"', 'page.state === "succeeded"'),
    processSource.replace("cancelledBPage?.jobId !== jobIdB", "false"),
    processSource.replace('!cancelledKinds.includes("cancel-requested")', "false"),
    processSource.replace('!cancelledKinds.includes("cancelled")', "false"),
    processSource.replace('terminalB?.proposalHash !== null', "false"),
    processSource.replace('denied.value?.code !== "inference.denied"', "false"),
    processSource.replace('"Author A RebootstrapRequired"', '"substituted"'),
    processSource.replace("JSON.stringify(refreshedA) !== JSON.stringify(refreshedB)", "false"),
    processSource.replace("JSON.stringify(undoHandle.expectedCurrent) !== JSON.stringify(refreshedA.frontier)", "false"),
    processSource.replace("peerUndo.status !== 403", "false"),
    processSource.replace("staleUndo.status !== 409", "false"),
    processSource.replace("undone.value?.replayed !== false", "false"),
    processSource.replace('"Author A undo RebootstrapRequired"', '"substituted"'),
    processSource.replace("JSON.stringify(undoneA) !== JSON.stringify(undoneB)", "false"),
    processSource.replace("undoneA.packSha256 !== initialA.packSha256", "false"),
    processSource.replace("replay.value?.replayed !== true", "false"),
  ];
  processHostiles.forEach((source, index) => {
    if (source === processSource || processConforms(source)) throw new Error(`GIS Map proposal process source oracle admitted hostile ${index}`);
  });
  console.log(
    `gis-map-proposal-oracle: scope-exports=5 committer-laws=${committerLaws} node-sha256=2 independent-bounds=2 preview=1 lifecycle=${fixture.lifecycle.length + fixture.cancelLifecycle.length} checkpoint-control=${checkpointControlCases} retained-runtime=${retainedRuntimeCases} durable-undo=${durableUndoCases} committer=7 visibility=${fixture.visibility.length} errors=${fixture.errors.length} approval-rejections=${fixture.approvalRejections.length} cross-fixture=1 process-source=${processHostiles.length + 1}; no external model provider, no WGPU rendering`,
  );
  return committerLaws;
}

/** 🧊️ Independently pins every retained GIS Map catalog and executable binding fact. */
async function proveGisMapFrozenBindingFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🧊️gis-map-frozen-binding-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = hubSchemaExport(repoRoot, "schema://hub.inference/GisMapFrozenBindingV1");
  if (fixture.schema !== "semio.hub.gis-map-frozen-binding-fixture/v1") throw new Error("frozen GIS Map binding fixture envelope drifted");
  if (!validate(fixture.binding)) throw new Error("frozen GIS Map binding is not a hub.inference/GisMapFrozenBindingV1");
  if (!/^[0-9a-f]{64}$/.test(fixture.expectedDigest) || fixture.hostile.length < 24) throw new Error("frozen GIS Map binding fixture inventory drifted");
  const digest = (binding: unknown): string =>
    createHash("sha256")
      .update(Buffer.concat([Buffer.from("semio.hub.gis-map-frozen-binding/v1", "utf8"), Buffer.from([0])]))
      .update(JSON.stringify(binding))
      .digest("hex");
  if (digest(fixture.binding) !== fixture.expectedDigest) throw new Error("frozen GIS Map binding digest differs from the neutral fixture");
  const leafPaths = (value: unknown, path: string[] = []): string[] =>
    value !== null && typeof value === "object" && !Array.isArray(value) ? Object.entries(value).flatMap(([key, child]) => leafPaths(child, [...path, key])) : [JSON.stringify(path)];
  const remainingPaths = new Set(leafPaths(fixture.binding));
  const seen = new Set<string>();
  for (const hostile of fixture.hostile) {
    if (seen.has(hostile.name)) throw new Error(`duplicate frozen binding hostile ${hostile.name}`);
    seen.add(hostile.name);
    if (!remainingPaths.delete(JSON.stringify(hostile.path))) throw new Error(`duplicate or unknown frozen binding field: ${hostile.name}`);
    const candidate = structuredClone(fixture.binding);
    let at = candidate;
    for (const key of hostile.path.slice(0, -1)) at = at[key];
    at[hostile.path.at(-1)] = hostile.value;
    const admitted = validate(candidate) && digest(candidate) === fixture.expectedDigest;
    if (admitted !== hostile.accepted) throw new Error(`frozen GIS Map binding substitution accepted: ${hostile.name}`);
    assertObservationExpectation(`frozen GIS Map binding ${hostile.name}`, { stage: "contract", result: hostile.accepted ? "accepted" : "rejected", code: hostile.name }, admitted);
  }
  if (remainingPaths.size !== 0) throw new Error(`frozen binding fields lack substitution coverage: ${[...remainingPaths].join(", ")}`);
  const catalog = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "📇️catalog", "🦀️.rs"), "utf8");
  const gisBinding = readFileSync(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"), "utf8");
  if (!gisBinding.includes("semio_s_artifact_gis_gismap::gis_map_inference_service()")) throw new Error("the GIS Map inference binding no longer registers the literal native executable");
  const trusted = readModuleTreeSource(join(repoRoot, "🌎️hub", "🗿️artifact-authority", "🔏️trusted-catalog", "🦀️.rs"));
  const startup = readHubBinSource(repoRoot);
  const required = [
    "VerifiedGisMapArtifactBindingV1",
    "Arc<VerifiedTrustedCatalog>",
    "selected_document_open",
    "proposals.inference_service()",
    "executable_identity",
    "component_blake3",
    "descriptor_byte_sha256",
    "execution_protocol",
    "parent_dialect",
    "DocumentOpenSurfaceRoleV1::Editor",
    "grant.read",
    "grant.write",
    "grant.observe",
    "semio.hub.gis-map-frozen-binding/v1\\0",
  ];
  if (
    required.some((needle) => !catalog.includes(needle)) ||
    !trusted.includes("pub fn selected_document_open") ||
    !startup.includes("verified_catalog: Option<Arc<VerifiedTrustedCatalog>>") ||
    !startup.includes("gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>")
  ) {
    throw new Error("hub does not retain the exact verified GIS Map catalog and executable binding");
  }
  const checks = fixture.hostile.length + required.length + 5;
  console.log(`gis-map-frozen-binding-check: checks=${checks} clean; no route, provider, inference execution, or publication claim`);
  return checks;
}

/** 🧷 Independently evaluates the exact GIS selection and admission fences. */
export async function proveGisNativeProviderSelectionFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️tests/🧫️fixtures/🌍️gis-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  if (fixture.schema !== "semio.hub.gis-native-provider-selection/v1" || fixture.cases.length !== 8 || new Set(fixture.cases.map((row: any) => row.name)).size !== fixture.cases.length) throw new Error("GIS provider selection envelope differs");
  const receipts = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🔣️.json"), "utf8"));
  if (fixture.packageVersion !== receipts.packageVersion || fixture.codecCount !== receipts.receipts.length) throw new Error("GIS selection differs from the package-owned closure");
  let exact = 0;
  let unlinkedAccepted = 0;
  for (const row of fixture.cases) {
    const inBounds = !row.cancelled && row.nowMs < row.deadlineMs;
    const exactPackage = row.pluginId === receipts.pluginId && row.packageId === receipts.packageId && row.version === receipts.packageVersion;
    const unlinked = row.pluginId !== receipts.pluginId && row.packageId !== receipts.packageId;
    const result = inBounds && (exactPackage || unlinked);
    assertObservationExpectation(`gis-native-provider-selection/${row.name}`, { stage: row.stage, result: row.accepted ? "accepted" : "rejected", code: row.code }, result);
    if (result !== row.accepted) throw new Error(`GIS native selection mismatch: ${row.name}`);
    if (row.bindings !== (result && exactPackage ? receipts.receipts.length : 0)) throw new Error(`GIS native selection binding count mismatch: ${row.name}`);
    if (result && exactPackage) exact++;
    if (result && unlinked) unlinkedAccepted++;
  }
  if (exact !== 1 || unlinkedAccepted !== 1) throw new Error("GIS provider corpus must admit exactly one linked selection and one unlinked package");
  console.log(`gis-native-provider-selection-oracle: cases=${fixture.cases.length} exact=${exact} unlinked=${unlinkedAccepted}; no native or catalog activation claim`);
}

export class GisMapFrozenBindingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("gis-map-frozen-binding-check accepts only --native");
    await proveGisMapFrozenBindingFixture(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{ package: "semio-hub-inference-catalog", target: { kind: "lib" }, laws: ["gis_map_verified_binding_freezes_catalog_selection_and_native_executable"] }, { package: "semio-hub-trusted-catalog-laws", target: { kind: "lib" }, laws: ["gis_map_binding_constructs_from_loaded_catalog_and_refuses_tampered_retained_bytes"] }],progress(event) {
          console.log(`gis-map-frozen-binding ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`gis-map-frozen-binding-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🤝️ Pins one selected-current, private-owner Shell journey before any process is started. */
async function proveGisMapSocketRetirement(invocation: ScriptInvocation, repoRoot: string, fixture: { maximumMs: number; hostileDeadlineMs: number; cases: string[] }): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const observed: string[] = [];
  class HeldSocket extends EventTarget {
    readyState: number = WebSocket.OPEN;
    onmessage: null = null;
    close(): void { this.readyState = WebSocket.CLOSING; }
  }
  const live = (socket: unknown): GisMapProcessDocumentSocketV1 => ({ socket: socket as WebSocket, frames: [{ fixture: true }], plan: { receipt: "retirement-fixture-only" } as ReturnType<typeof parseDocumentOpenPlanV1>, error: undefined });
  const closed = new HeldSocket(); closed.readyState = WebSocket.CLOSED;
  const closedLive = live(closed);
  await finishGisMapProcessDocumentSocket(closedLive, fixture.maximumMs);
  assert.equal(closedLive.plan.receipt, ""); assert.deepEqual(closedLive.frames, []);
  observed.push("closed-at-entry");
  const pending = new HeldSocket(); let completed = false;
  const operation = finishGisMapProcessDocumentSocket(live(pending), fixture.maximumMs).then(() => { completed = true; });
  await Bun.sleep(0); assert.equal(completed, false); assert.equal(pending.readyState, WebSocket.CLOSING);
  pending.readyState = WebSocket.CLOSED; pending.dispatchEvent(new Event("close"));
  await operation; assert.equal(completed, true); observed.push("await-close-event");
  const refused = live(new HeldSocket());
  await assert.rejects(finishGisMapProcessDocumentSocket(refused, fixture.hostileDeadlineMs), /retirement deadline/);
  assert.equal(refused.plan.receipt, ""); assert.deepEqual(refused.frames, []); observed.push("timeout-refused");
  let serverClosed = 0;
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0,
    fetch(request, server) { return server.upgrade(request) ? undefined : new Response("", { status: 400 }); },
    websocket: { message() {}, close() { serverClosed++; } },
  });
  const sockets: WebSocket[] = [];
  try {
    for (const [id, Socket] of [["bun-websocket-close", WebSocket]] as const) {
      const socket = new Socket(`ws://127.0.0.1:${server.port}`); sockets.push(socket);
      await new Promise<void>((resolveOpen, rejectOpen) => {
        const timer = setTimeout(() => finish(new Error("socket retirement oracle open deadline exceeded")), fixture.maximumMs);
        const finish = (error?: Error): void => { clearTimeout(timer); socket.removeEventListener("open", opened); socket.removeEventListener("error", failed); if (error) rejectOpen(error); else resolveOpen(); };
        const opened = (): void => finish(); const failed = (event: Event): void => finish(new Error(`${id} retirement oracle failed to open: ${String((event as ErrorEvent).message).slice(0, 256)}`));
        socket.addEventListener("open", opened, { once: true }); socket.addEventListener("error", failed, { once: true });
      });
      const owned = live(socket);
      await finishGisMapProcessDocumentSocket(owned, fixture.maximumMs);
      assert.equal(socket.readyState, WebSocket.CLOSED); assert.deepEqual(owned.frames, []); assert.equal(owned.plan.receipt, ""); observed.push(id);
    }
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot || !isAbsolute(artifactRoot) || !artifactRoot.split(/[/\\]/u).includes("🗑️generated")) throw new Error("socket retirement oracle requires ticket-generated artifacts");
    const evidence = mkdtempSync(join(artifactRoot, "gis-socket-retirement-"));
    const thirdParty = `
const assert = require("node:assert/strict");
const WebSocket = require(${JSON.stringify(join(repoRoot, "node_modules/ws/index.js"))});
${finishGisMapProcessDocumentSocket.toString()}
(async () => {
  const socket = new WebSocket(${JSON.stringify(`ws://127.0.0.1:${server.port}`)});
  await new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("ws oracle open deadline")), ${fixture.maximumMs});
    socket.once("open", () => { clearTimeout(timer); resolve(); });
    socket.once("error", error => { clearTimeout(timer); reject(error); });
  });
  const live = { socket, frames: [{}], plan: { receipt: "retirement-fixture-only" } };
  await finishGisMapProcessDocumentSocket(live, ${fixture.maximumMs});
  assert.equal(socket.readyState, WebSocket.CLOSED);
  assert.deepEqual(live.frames, []); assert.equal(live.plan.receipt, "");
  process.stdout.write(JSON.stringify({ id: "ws-websocket-close", closed: true, frames: 0, receipt: "" }));
})().catch(error => { process.stderr.write(String(error).slice(0, 1024)); process.exitCode = 1; });`;
    const result = await captureOwnedProcess("node", ["--eval", thirdParty], {
      invocation, cwd: repoRoot, env: { PATH: process.env.PATH, SystemRoot: process.env.SystemRoot }, budgetMs: fixture.maximumMs * 3, maxOutputBytes: 4_096,
      stdoutPath: join(evidence, "ws.stdout"), stderrPath: join(evidence, "ws.stderr"),
    });
    assert.equal(result.status, 0, `third-party ws retirement failed: ${result.reason} ${result.stderr}`);
    assert.equal(result.signal, null);
    assert.deepEqual(JSON.parse(result.stdout), { id: "ws-websocket-close", closed: true, frames: 0, receipt: "" });
    observed.push("ws-websocket-close");
    const deadline = Date.now() + fixture.maximumMs;
    while (serverClosed !== 2 && Date.now() < deadline) await Bun.sleep(5);
    assert.equal(serverClosed, 2); assert.deepEqual(observed, fixture.cases);
  } finally {
    await server.stop(true);
    for (const socket of sockets) if (socket.readyState < WebSocket.CLOSING) socket.close();
  }
  console.log(`GIS Map witness socket retirement: neutral=${observed.length} Bun=1 third-party-ws=1 exact-close-events=2; no Hub restart claim`);
}

/** 🪢️ Validates the neutral shared-current workflow and its private owner boundaries. */
export async function proveGisMapTwoAuthorCompositionFixture(invocation: ScriptInvocation, repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🤝️two-author-shell-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  assert.equal(fixture.selection.executionProtocol, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, "two-author fixture must name the currently compiled AppChannel");
  const laws: readonly (readonly [string, unknown])[] = [
    ["schema", "semio.hub.gis-map-two-author-composition-fixture/v1"],
    ["version", 1],
    ["selection.closure", ["stdio", "gis"]],
    ["selection.executionProtocol", DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1],
    ["selection.sameDataRoot", true],
    ["selection.receiptIdentity", ["generationId", "bundleSha256", "profileId", "publicationRevision", "currentSha256"]],
    ["selection.browserHostReceipt", ["moduleSetSha256", "activationReceiptSha256", "spaceComponentByteLength", "spaceComponentSha256", "spaceCoreSha256", "spaceDescriptorSha256", "selectedGisGenerationId", "selectedGisCurrentSha256", "selectedGisComponentSha256", "selectedGisDescriptorSha256", "selectedGisStagedDescriptorSha256", "selectedGisBridgeSha256", "selectedGisGeneratedCores", "selectedGisMaterializationReceiptSha256"]],
    ["authors", ["author-a", "author-b"]],
    ["locales", { "author-a": "en", "author-b": "de" }],
    ["artifactOpen", { en: "Open", de: "Öffnen", exact: true, scope: "artifact-row" }],
    ["observations.mapSurface", ["actual-ui-document-store", "tiled-map", "actual-worker-ui-patch-ack"]],
    ["observations.cancel", ["running-receipt-before-compute", "fd4-entered", "author-b-own-private-mcp-cancel", "cancelled-without-offer"]],
    ["observations.approval", ["author-a-shell-worker-approval", "one-server-stamped-create-region", "same-peer-rebootstrap-control", "same-pair-and-frontier", "author-b-private-job-denied"]],
    ["observations.undo", ["ordinary-shell-undo-action", "private-worker-handle", "durable-undo-receipt", "region-absent-on-both-maps"]],
    ["observations.restart", ["same-current-receipt", "same-durable-post-undo-frontier"]],
    ["observations.redo", ["ordinary-shell-redo-action", "durable-collaborative-redo", "region-restored-on-both-maps", "redo-stack-survived-hub-restart"]],
    ["nonclaims", ["external-model-provider", "browser-qualified-current-publication", "private-job-restart-recovery"]],
    ["socketRetirement", { maximumMs: 5000, hostileDeadlineMs: 15, cases: ["closed-at-entry", "await-close-event", "timeout-refused", "bun-websocket-close", "ws-websocket-close"] }],
  ];
  for (const [name, expected] of laws) {
    const actual = name.split(".").reduce<any>((at, key) => at?.[key], fixture);
    assert.equal(JSON.stringify(actual), JSON.stringify(expected), `two-author composition law ${name}`);
  }
  assert.equal(fixture.steps.length, 19, "two-author composition step count");
  assert.equal(new Set(fixture.steps).size, 19, "two-author composition steps are not unique");
  assert.equal(fixture.steps[0], "materialize-and-validate-current");
  assert.deepEqual(fixture.steps.slice(3, 8), ["create-shared-space-and-admit-author-b", "mount-author-a-real-shell-and-create-map", "open-two-authenticated-mcp-clients", "open-two-authenticated-document-sockets", "mount-author-b-real-shell-worker-map"]);
  assert.equal(fixture.steps.at(-1), "both-peers-observe-region-restored-by-durable-redo");
  assert.equal(fixture.hostiles.length, 11, "two-author composition hostile count");
  assert.equal(new Set(fixture.hostiles).size, 11, "two-author composition hostiles are not unique");
  const hostiles = fixture.hostiles;
  await proveGisMapSocketRetirement(invocation, repoRoot, fixture.socketRetirement);
  const source = readHubScriptModule(repoRoot, "🧪️tests/🕹️gis-map-process");
  const start = source.indexOf("\nasync function proveGisMapTwoAuthorShellProcess(");
  const end = source.indexOf("\n/** 🤝️ Drives the bounded two-Author Hub/MCP proposal", start);
  const processOwner = source.slice(start, end);
  const peerStart = source.indexOf("\nasync function startGisMapShellPeerV1(");
  const peerEnd = source.indexOf("\ntype GisMapTwoAuthorPreparedV1", peerStart);
  const peerOwner = source.slice(peerStart, peerEnd);
  assert.ok(start >= 0 && end > start, "same-data-root Shell composition owner is missing");
  assert.ok(processOwner.indexOf("creation: { kindId: target.artifactKind") < processOwner.indexOf("await openPeers(false)"), "ordinary author-a Shell creation must precede document-scoped MCP and peer join");
  assert.ok(processOwner.includes('"private-job-restart-recovery"'), "process receipt must not claim an unobserved private job after restart");
  assert.ok(!processOwner.includes('"durable-collaborative-redo"') || processOwner.includes("author-a-shell-ordinary-redo-after-hub-restart") || processOwner.includes(".redo()"), "two-author shell must exercise durable collaborative redo");
  assert.ok(peerStart >= 0 && peerEnd > peerStart, "ticket-owned browser peer owner is missing");
  assert.ok(peerOwner.includes('artifactRow.getByRole("button", { name: options.locale === "de" ? "Öffnen" : "Open", exact: true })'), "each author must open its exact artifact row using its explicitly selected locale");
  assert.ok(!processOwner.includes("withMaterializedTrustedCatalogBundleV1(") && !processOwner.includes("validateAndPublishTrustedStdioGisCandidate("), "Shell composition cannot create another current or data root");
  assert.ok(processOwner.includes("assertGisMapCompositionCurrent(prepared)") && processOwner.includes("dataDir: prepared.dataRoot"), "Shell starts and restarts must retain the prepared current owner");
  assert.ok(processOwner.includes("shells[0]!.redo()") && processOwner.includes('"durable-collaborative-redo"'), "Shell composition must prove durable collaborative redo after hub restart");
  assert.ok(processOwner.includes("withTestBrowserHostV1({") && processOwner.includes("browserHost: browserHost.receipt"), "Shell composition must close one selected ticket browser host before launch");
  for (const name of ["SEMIO_TEST_ARTIFACT_DIR", "SEMIO_TEST_BROWSER_MODULE_ROOT", "SEMIO_TEST_BROWSER_ACTIVATION_ROOT", "SEMIO_TEST_BROWSER_HOST_RECEIPT"]) assert.ok(peerOwner.includes(name), `Shell peer omits ${name}`);
  assert.ok(!peerOwner.includes("developmentRuntimeRoot") && !peerOwner.includes("pluginOutRoot"), "Shell peer must not resolve a repository-global browser host");
  console.log(`GIS Map two-author composition fixture laws=${laws.length} hostile=${hostiles.length} current-coordinates=${fixture.selection.receiptIdentity.length}; real mounted journey remains separately required`);
}
