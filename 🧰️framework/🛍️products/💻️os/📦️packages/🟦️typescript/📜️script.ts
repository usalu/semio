#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { BundleScript, ScriptRouter, getWorkspaceRoot, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "vitest.config.ts");
  }
}

/** 🏗️ Routes the package generator through the shared workspace implementation. */
class GenerateWgpuScript extends BundleScript {
  async run(): Promise<void> {
    await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "generate");
  }
}
/** 🔎️ Checks the exact package artifacts without writing outputs. */
class CheckWgpuScript extends BundleScript {
  async run(): Promise<void> {
    await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "check");
  }
}
/** 🔮️ Streams the canonical read-only package preview. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "preview");
  }
}

//#region 💡️InferencePortCheck
/** 💡️ One rendered phase of the host-owned inference port, restated independently of production. */
type OraclePreview = { schema: string; jobId: string; proposalHash: string; regionId: string; ring: readonly (readonly [number, number])[] };
type OracleStatus = { phase: string; jobId: string | null; cursor: number; completed: number; total: number; proposalHash: string | null; preview?: OraclePreview; cancelRequested: boolean; code: string | null };

/** 🎬️ One closed input the port's state machine accepts, as the neutral corpus spells it. */
type OracleEvent = { kind: string; receipt?: Record<string, unknown>; page?: Record<string, unknown>; code?: string };

/** 🧬️ The exact shape of `💡️gis-map-inference-port-v1`, so the oracle reads a typed corpus. */
type OracleCorpus = {
  phases: readonly string[];
  ariaRoles: Readonly<Record<string, string>>;
  codes: readonly string[];
  limits: Readonly<Record<string, number>>;
  serviceId: string;
  sampleJobId: string;
  otherJobId: string;
  proposalHash: string;
  preview: OraclePreview;
  initial: OracleStatus;
  successLifecycle: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  cancelLifecycle: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  rejectLifecycle: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  mapOverlay: { regionId: string; viewBox: string; path: string };
  controls: Readonly<Record<string, Readonly<Record<"en" | "de", string>>>>;
  uiAffordances: Readonly<Record<string, { request: boolean; cancel: boolean; reject: boolean; approve: boolean; overlay: boolean }>>;
  uncertainLifecycle: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  cancelBeforeReceipt: { expectedBeforeReceipt: Pick<OracleStatus, "phase" | "jobId" | "cancelRequested">; expectedAfterReceipt: Pick<OracleStatus, "phase" | "jobId" | "cancelRequested"> };
  leaseRefusal: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  hostileTransitions: readonly { name: string; from: OracleStatus; event: OracleEvent; expected: OracleStatus }[];
  crossFixture: { approvalFixture: string; serverLifecycleKinds: readonly string[]; serverCancelLifecycleKinds: readonly string[]; serverKindToPhase: Readonly<Record<string, string>> };
  nonclaims: readonly string[];
};

const ORACLE_TERMINALS = new Set(["applied", "cancelled", "stale", "failed"]);

/** 🗺️ An independent projection of one server answer onto a rendered phase. It is hand-written
 * against the packet's own state law, imports nothing from the production schema module, and exists
 * exactly so a defect in `reduceGisMapInferencePortV1` cannot hide behind itself. */
function oracleServerPhase(page: { state: string; proposalState: string; stale: boolean }): string {
  if (page.stale) return "stale";
  if (page.proposalState === "stale") return "stale";
  if (page.state === "cancelled") return "cancelled";
  if (page.proposalState === "cancelled") return "cancelled";
  if (page.state === "failed") return "failed";
  if (page.proposalState === "approved") return "applied";
  if (page.proposalState === "offered") return "offered";
  if (page.state === "succeeded") return "offered";
  return "running";
}

/** 🧮️ The independent transition relation the corpus is checked against. */
/** 🗺️ Hand-written overlay projection. Imports no production module. */
function oracleProjectOverlay(preview: OraclePreview): { regionId: string; viewBox: "0 0 100 100"; path: string } {
  const lons = preview.ring.map((point) => point[0]);
  const lats = preview.ring.map((point) => point[1]);
  const minLon = Math.min(...lons);
  const maxLon = Math.max(...lons);
  const minLat = Math.min(...lats);
  const maxLat = Math.max(...lats);
  const lonSpan = maxLon - minLon || 1;
  const latSpan = maxLat - minLat || 1;
  const format = (value: number): string => (Number.isInteger(value) ? String(value) : String(value));
  const points = preview.ring.map(([lon, lat]) => `${format(((lon - minLon) / lonSpan) * 100)} ${format(((maxLat - lat) / latSpan) * 100)}`);
  return { regionId: preview.regionId, viewBox: "0 0 100 100", path: `M ${points.join(" L ")} Z` };
}

/** 🎛️ Hand-written chrome law. Reject uses the existing cancel intent; it never invents a route. */
function oracleAffordances(status: OracleStatus): { request: boolean; cancel: boolean; reject: boolean; approve: boolean; overlay: boolean } {
  const terminal = ORACLE_TERMINALS.has(status.phase);
  const previewOk = status.preview !== undefined && status.preview.proposalHash === status.proposalHash && status.preview.jobId === status.jobId;
  return {
    request: status.phase === "idle",
    cancel: !terminal && status.phase !== "idle" && status.phase !== "offered" && !status.cancelRequested,
    reject: status.phase === "offered" && previewOk && !status.cancelRequested,
    approve: status.phase === "offered" && status.proposalHash !== null && previewOk && !status.cancelRequested,
    overlay: (status.phase === "offered" || status.phase === "approving") && previewOk,
  };
}

function oracleReduce(current: OracleStatus, event: OracleEvent): OracleStatus {
  const idle: OracleStatus = { phase: "idle", jobId: null, cursor: 0, completed: 0, total: 0, proposalHash: null, cancelRequested: false, code: null };
  if (event.kind === "clear") return idle;
  if (ORACLE_TERMINALS.has(current.phase)) return current;
  if (event.kind === "start") return current.phase === "idle" ? { ...idle, phase: "submitting" } : current;
  if (event.kind === "lease-unverified") return current.phase === "idle" || current.phase === "submitting" ? { ...current, phase: "failed", code: "inference.lease-unverified" } : current;
  if (event.kind === "receipt") {
    if (current.phase !== "submitting" && !(current.phase === "indeterminate" && current.jobId === null)) return current;
    const receipt = event.receipt as { jobId: string; state: string; proposalState: string; proposalHash: string | null; cursor: number };
    const { preview: _preview, ...withoutPreview } = current;
    return { ...withoutPreview, phase: oracleServerPhase({ state: receipt.state, proposalState: receipt.proposalState, stale: false }), jobId: receipt.jobId, cursor: receipt.cursor, proposalHash: receipt.proposalHash ?? null, code: null };
  }
  if (event.kind === "page") {
    const page = event.page as { jobId: string; state: string; proposalState: string; cancelRequested: boolean; stale: boolean; proposalHash: string | null; preview?: OraclePreview; progress: { completed: number; total: number }[]; nextCursor: number };
    if (current.jobId === null || current.jobId !== page.jobId) return current;
    const server = oracleServerPhase(page);
    const phase = current.phase === "approving" && !ORACLE_TERMINALS.has(server) ? "approving" : server;
    const last = page.progress.length === 0 ? undefined : page.progress[page.progress.length - 1];
    return {
      phase,
      jobId: current.jobId,
      cursor: Math.max(current.cursor, page.nextCursor),
      completed: last === undefined ? current.completed : last.completed,
      total: last === undefined ? current.total : last.total,
      proposalHash: page.proposalHash ?? null,
      ...((phase === "offered" || phase === "approving") && page.preview !== undefined ? { preview: page.preview } : {}),
      cancelRequested: current.cancelRequested || page.cancelRequested,
      code: phase === "failed" ? "inference.storage" : null,
    };
  }
  if (event.kind === "approve")
    return current.phase === "offered" && current.proposalHash !== null && current.preview?.proposalHash === current.proposalHash && current.preview.jobId === current.jobId && !current.cancelRequested ? { ...current, phase: "approving" } : current;
  if (event.kind === "approval") {
    const receipt = event.receipt as { jobId: string; proposalHash: string; applied: boolean };
    if ((current.phase !== "approving" && current.phase !== "indeterminate") || current.jobId !== receipt.jobId || current.proposalHash !== receipt.proposalHash) return current;
    const { preview: _preview, ...withoutPreview } = current;
    return receipt.applied ? { ...withoutPreview, phase: "applied", code: null } : { ...withoutPreview, phase: "failed", code: "approval.commit-unavailable" };
  }
  if (event.kind === "cancel") return current.phase === "idle" ? current : { ...current, cancelRequested: true };
  if (event.kind === "indeterminate") {
    const { preview: _preview, ...withoutPreview } = current;
    return { ...withoutPreview, phase: "indeterminate", code: event.code ?? null };
  }
  if (event.kind === "failed") {
    const { preview: _preview, ...withoutPreview } = current;
    return { ...withoutPreview, phase: event.code === "inference.cancelled" ? "cancelled" : "failed", code: event.code ?? null };
  }
  throw new Error(`gis-map-inference-port oracle: unknown event ${event.kind}`);
}

/** ⚖️ Field-order-independent equality over the exact closed status shape. */
function sameStatus(left: OracleStatus, right: OracleStatus): boolean {
  const fields: readonly (keyof OracleStatus)[] = ["phase", "jobId", "cursor", "completed", "total", "proposalHash", "cancelRequested", "code"];
  return fields.every((field) => left[field] === right[field]) && JSON.stringify(left.preview) === JSON.stringify(right.preview);
}

type ApprovalHistoryFixtureRow = Readonly<{
  name: string;
  current: string | null;
  event: "approval-received" | "mounted-current" | "undo" | "rebootstrap" | "close";
  ownerMatches: boolean;
  mountedCurrentMatches: boolean;
  revalidationMatches: boolean;
  expected: Readonly<{ phase: string; canUndo: boolean; retained: boolean; oldOwnerRetired: boolean }>;
}>;

/** ↩️ Independently reduces the private durable-approval history lifecycle. */
function oracleApprovalHistory(row: ApprovalHistoryFixtureRow): ApprovalHistoryFixtureRow["expected"] {
  const current = row.current ?? "unavailable";
  if (row.event === "approval-received") return { phase: "unavailable", canUndo: false, retained: true, oldOwnerRetired: false };
  if (row.event === "close") return row.ownerMatches
    ? { phase: "unavailable", canUndo: false, retained: false, oldOwnerRetired: true }
    : { phase: current, canUndo: current === "available", retained: true, oldOwnerRetired: false };
  if (row.event === "rebootstrap") return row.ownerMatches
    ? { phase: "unavailable", canUndo: false, retained: true, oldOwnerRetired: true }
    : { phase: current, canUndo: current === "available", retained: true, oldOwnerRetired: false };
  if (row.event === "mounted-current") {
    if (!row.ownerMatches) return { phase: current, canUndo: current === "available", retained: true, oldOwnerRetired: false };
    if (!row.mountedCurrentMatches || !row.revalidationMatches) return { phase: "unavailable", canUndo: false, retained: false, oldOwnerRetired: true };
    return { phase: "available", canUndo: true, retained: true, oldOwnerRetired: false };
  }
  if (row.event === "undo" && row.ownerMatches && row.mountedCurrentMatches && row.revalidationMatches) return { phase: "submitting", canUndo: false, retained: true, oldOwnerRetired: false };
  return { phase: current, canUndo: current === "available", retained: true, oldOwnerRetired: false };
}

//#region 🧬️OwnedSchemaExports
const OWNED_SCHEMA_MODULES = {
  os: "🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
  renderer: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json",
} as const;

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
async function ownedExport(repoRoot: string, scope: keyof typeof OWNED_SCHEMA_MODULES, exportId: string) {
  const Ajv = (await import("ajv")).default;
  const doc = JSON.parse(readFileSync(join(repoRoot, OWNED_SCHEMA_MODULES[scope]), "utf8")) as { $id: string };
  const compiled = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-note").addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
  if (!compiled) throw new Error(`${scope} schema module publishes no export ${exportId}`);
  return compiled;
}
//#endregion 🧬️OwnedSchemaExports

/** 🧪️ Validates the private approval-history contract against AJV, an independent reducer, the
 * Shell arbitration function, and source hostiles for authority retirement and reissue. */
async function proveGisMapApprovalHistory(repoRoot: string): Promise<Record<string, number>> {
  const root = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧫️fixtures", "↩️gis-map-approval-history-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as Readonly<{ cases: readonly ApprovalHistoryFixtureRow[]; ordinaryHistory: Readonly<Record<string, string>> }>;
  const validate = await ownedExport(repoRoot, "os", "GisMapApprovalHistoryV1");
  if (!validate(fixture)) throw new Error(`invalid GIS Map approval history corpus: ${JSON.stringify(validate.errors)}`);
  if (validate({ ...fixture, cases: fixture.cases.slice(1) })) throw new Error("approval history schema admitted a missing lifecycle law");
  const deepEqual = (await import("fast-deep-equal")).default;
  for (const row of fixture.cases) if (!deepEqual(oracleApprovalHistory(row), row.expected)) throw new Error(`approval history oracle disagrees at ${row.name}`);

  const production = await import("../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx");
  const routes = {
    remoteNewerThanLocal: production.shellHistoryUndoRouteV1({ phase: "available", canUndo: true, order: 2 }, { canUndo: true, order: 1 }),
    localNewerThanRemote: production.shellHistoryUndoRouteV1({ phase: "available", canUndo: true, order: 1 }, { canUndo: true, order: 2 }),
    remoteSubmitting: production.shellHistoryUndoRouteV1({ phase: "submitting", canUndo: false, order: 2 }, { canUndo: true, order: 1 }),
    noRemote: production.shellHistoryUndoRouteV1(null, { canUndo: true, order: 1 }),
  };
  if (!deepEqual(routes, fixture.ordinaryHistory)) throw new Error("Shell history arbitration disagrees with the neutral corpus");

  const worker = readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧵️backbone-worker.ts"), "utf8");
  const shell = readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
  const conforms = (candidateWorker: string, candidateShell: string): boolean =>
    candidateWorker.includes('owner.abort.abort(new Error("gis map approval undo owner rebootstrap"));')
    && candidateWorker.includes("idempotencyKey: owner.idempotencyKey,")
    && candidateWorker.includes("fields.revalidation.directoryRevision !== owner.sourceDirectoryRevision")
    && candidateWorker.includes("fields.revalidation.membershipGeneration !== owner.sourceMembershipGeneration")
    && candidateWorker.includes("fields.revalidation.sessionGeneration !== owner.sourceSessionGeneration")
    && candidateWorker.includes("fields.revalidation.shareGeneration !== owner.sourceShareGeneration")
    && candidateWorker.includes("currentState === undefined || !sameApprovalUndoMountV1(currentState, owner)")
    && candidateWorker.includes("revokeDirectoryAdministrationForScope(scope.spaceId);\n      closeArtifactRuntime(key);")
    && candidateShell.includes('kind: "inference-history-undo", historyEpoch: history.historyEpoch');
  if (!conforms(worker, shell)) throw new Error("approval history production closure is incomplete");
  const hostiles = [
    [worker.replace('owner.abort.abort(new Error("gis map approval undo owner rebootstrap"));', "void owner.abort;"), shell],
    [worker.replace("idempotencyKey: owner.idempotencyKey,", "idempotencyKey: mintInferenceApprovalUndoIdempotencyKeyV1(),"), shell],
    [worker.replace("fields.revalidation.directoryRevision !== owner.sourceDirectoryRevision", "false"), shell],
    [worker.replace("fields.revalidation.membershipGeneration !== owner.sourceMembershipGeneration", "false"), shell],
    [worker.replace("fields.revalidation.sessionGeneration !== owner.sourceSessionGeneration", "false"), shell],
    [worker.replace("fields.revalidation.shareGeneration !== owner.sourceShareGeneration", "false"), shell],
    [worker.replace("currentState === undefined || !sameApprovalUndoMountV1(currentState, owner)", "false"), shell],
    [worker.replace("revokeDirectoryAdministrationForScope(scope.spaceId);\n      closeArtifactRuntime(key);", "revokeDirectoryAdministrationForScope(scope.spaceId);"), shell],
    [worker, shell.replace('kind: "inference-history-undo", historyEpoch: history.historyEpoch', 'kind: "inference-close", operationEpoch: 0')],
  ] as const;
  hostiles.forEach(([candidateWorker, candidateShell], index) => {
    if (conforms(candidateWorker, candidateShell)) throw new Error(`approval history source oracle admitted hostile ${index}`);
  });
  return { ajv: 1, oracle: fixture.cases.length, historyRoutes: Object.keys(routes).length, sourceHostiles: hostiles.length };
}

/** 🧪️ Validates the neutral corpus with AJV 2020, walks both lifecycles and every hostile
 * transition through an INDEPENDENT hand-written state machine AND the production reducer, checks
 * the explicit EN/DE vocabulary is total with no default language, and cross-checks the corpus
 * against the hub's own `🗳️gis-map-proposal-approval-v1` lifecycles and limits plus the `🖥️shell`
 * cross-language twin. It never runs a real hub, a model, or a renderer. */
async function proveGisMapInferencePortFixture(repoRoot: string): Promise<Record<string, number>> {
  const root = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧫️fixtures", "💡️gis-map-inference-port-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as OracleCorpus;
  const validate = await ownedExport(repoRoot, "os", "GisMapInferencePortV1");
  if (!validate(fixture)) throw new Error(`invalid GIS Map inference port corpus: ${JSON.stringify(validate.errors)}`);
  const hostileCorpora: readonly unknown[] = [
    { ...fixture, phases: fixture.phases.slice(1) },
    { ...fixture, ariaRoles: { ...fixture.ariaRoles, failed: "status" } },
    { ...fixture, limits: { ...fixture.limits, progressMaxCursor: 17 } },
    { ...fixture, codes: fixture.codes.slice(1) },
    { ...fixture, successLifecycle: fixture.successLifecycle.slice(1) },
    { ...fixture, nonclaims: [...fixture.nonclaims.slice(1), fixture.nonclaims[1]] },
    { ...fixture, serviceId: "s.gis.gismap.other" },
  ];
  for (const [index, candidate] of hostileCorpora.entries()) if (validate(candidate)) throw new Error(`GIS Map inference port corpus accepted hostile mutation ${index}`);

  const production = await import("../../🟦️.ts");
  if (JSON.stringify(production.parseGisMapInferencePreviewV1(fixture.preview)) !== JSON.stringify(fixture.preview)) throw new Error("production preview parser changed the validated projection");
  for (const candidate of [
    { ...fixture.preview, jobId: fixture.otherJobId },
    { ...fixture.preview, regionId: "substituted" },
    {
      ...fixture.preview,
      ring: [
        [7, 46],
        [9, 46],
        [8, 48],
        [7, 48],
        [7, 46],
      ],
    },
  ]) {
    try {
      production.parseGisMapInferencePreviewV1(candidate);
      throw new Error("production preview parser accepted substituted geometry");
    } catch (error) {
      if (error instanceof Error && error.message === "production preview parser accepted substituted geometry") throw error;
    }
  }
  let transitions = 0;
  const walk = (steps: readonly { name: string; event: OracleEvent; expected: OracleStatus }[]): void => {
    let oracleState: OracleStatus = structuredClone(fixture.initial);
    let productionState: OracleStatus = production.idleGisMapInferencePortStatusV1() as OracleStatus;
    for (const step of steps) {
      oracleState = oracleReduce(oracleState, step.event);
      productionState = production.reduceGisMapInferencePortV1(productionState as never, step.event as never) as unknown as OracleStatus;
      if (!sameStatus(oracleState, step.expected)) throw new Error(`independent oracle disagrees with the corpus at ${step.name}: ${JSON.stringify(oracleState)}`);
      if (!sameStatus(productionState, step.expected)) throw new Error(`production reducer disagrees with the corpus at ${step.name}: ${JSON.stringify(productionState)}`);
      transitions += 1;
    }
  };
  walk(fixture.successLifecycle);
  walk(fixture.cancelLifecycle);
  walk(fixture.rejectLifecycle);
  walk(fixture.uncertainLifecycle);
  let pendingOracle = oracleReduce(oracleReduce(fixture.initial, { kind: "start" }), { kind: "cancel" });
  let pendingProduction = production.reduceGisMapInferencePortV1(production.reduceGisMapInferencePortV1(production.idleGisMapInferencePortStatusV1(), { kind: "start" }), { kind: "cancel" });
  const projectPending = (status: OracleStatus) => JSON.stringify({ phase: status.phase, jobId: status.jobId, cancelRequested: status.cancelRequested });
  for (const state of [pendingOracle, pendingProduction]) if (projectPending(state) !== JSON.stringify(fixture.cancelBeforeReceipt.expectedBeforeReceipt)) throw new Error("inference pending cancellation intent was lost");
  const accepted = fixture.successLifecycle[1]!.event;
  pendingOracle = oracleReduce(pendingOracle, accepted);
  pendingProduction = production.reduceGisMapInferencePortV1(pendingProduction, accepted as never);
  for (const state of [pendingOracle, pendingProduction]) if (projectPending(state) !== JSON.stringify(fixture.cancelBeforeReceipt.expectedAfterReceipt)) throw new Error("inference receipt erased pending cancellation");
  transitions += 3;
  walk(fixture.leaseRefusal);
  for (const row of fixture.hostileTransitions) {
    const viaOracle = oracleReduce(structuredClone(row.from), row.event);
    const viaProduction = production.reduceGisMapInferencePortV1(structuredClone(row.from) as never, row.event as never) as unknown as OracleStatus;
    if (!sameStatus(viaOracle, row.expected)) throw new Error(`independent oracle disagrees with hostile row ${row.name}: ${JSON.stringify(viaOracle)}`);
    if (!sameStatus(viaProduction, row.expected)) throw new Error(`production reducer disagrees with hostile row ${row.name}: ${JSON.stringify(viaProduction)}`);
    transitions += 1;
  }

  let strings = 0;
  for (const phase of fixture.phases) {
    const row = production.GIS_MAP_INFERENCE_PORT_TEXT_V1[phase as keyof typeof production.GIS_MAP_INFERENCE_PORT_TEXT_V1];
    if (row === undefined || Object.keys(row).sort().join(",") !== "de,en" || row.en.length === 0 || row.de.length === 0 || row.en === row.de) throw new Error(`phase ${phase} has no explicit EN/DE text`);
    if (production.gisMapInferencePortRoleV1(phase as never) !== fixture.ariaRoles[phase]) throw new Error(`phase ${phase} has the wrong live-region politeness`);
    strings += 2;
  }
  for (const code of fixture.codes) {
    const row = production.GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1[code as keyof typeof production.GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1];
    if (row === undefined || Object.keys(row).sort().join(",") !== "de,en" || row.en === row.de) throw new Error(`code ${code} has no explicit EN/DE text`);
    strings += 2;
  }
  for (const [control, row] of Object.entries(production.GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1)) {
    if (Object.keys(row).sort().join(",") !== "de,en" || row.en === row.de) throw new Error(`control ${control} has no explicit EN/DE text`);
    strings += 2;
  }
  for (const [control, row] of Object.entries(fixture.controls)) {
    const productionRow = production.GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1[control as keyof typeof production.GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1];
    if (productionRow === undefined || productionRow.en !== row.en || productionRow.de !== row.de) throw new Error(`control ${control} drifted from the corpus`);
  }
  const oracleOverlay = oracleProjectOverlay(fixture.preview);
  if (JSON.stringify(oracleOverlay) !== JSON.stringify(fixture.mapOverlay)) throw new Error(`independent overlay disagrees with the corpus: ${JSON.stringify(oracleOverlay)}`);
  if (JSON.stringify(production.projectGisMapInferencePreviewOverlayV1(fixture.preview)) !== JSON.stringify(fixture.mapOverlay)) throw new Error("production overlay disagrees with the corpus");
  const offeredStatus = { ...fixture.initial, phase: "offered", jobId: fixture.sampleJobId, proposalHash: fixture.proposalHash, preview: fixture.preview };
  const runningStatus = { ...fixture.initial, phase: "running", jobId: fixture.sampleJobId };
  if (JSON.stringify(oracleAffordances(offeredStatus)) !== JSON.stringify(fixture.uiAffordances.offered)) throw new Error("independent offered affordances disagree");
  if (JSON.stringify(production.gisMapInferencePortAffordancesV1(offeredStatus as never)) !== JSON.stringify(fixture.uiAffordances.offered)) throw new Error("production offered affordances disagree");
  if (JSON.stringify(oracleAffordances(runningStatus)) !== JSON.stringify(fixture.uiAffordances.running)) throw new Error("independent running affordances disagree");
  if (JSON.stringify(production.gisMapInferencePortAffordancesV1(runningStatus as never)) !== JSON.stringify(fixture.uiAffordances.running)) throw new Error("production running affordances disagree");
  if (JSON.stringify(oracleAffordances(fixture.initial)) !== JSON.stringify(fixture.uiAffordances.idle)) throw new Error("independent idle affordances disagree");
  if (
    production.GIS_MAP_INFERENCE_REQUEST_MAX_BYTES !== fixture.limits.requestMaxBytes ||
    production.GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES !== fixture.limits.responseMaxBytes ||
    production.GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR !== fixture.limits.progressMaxCursor ||
    production.GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS !== fixture.limits.eventPageMaxItems ||
    production.GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS !== fixture.limits.jobMaxLifetimeMs ||
    production.GIS_MAP_INFERENCE_SERVICE_ID !== fixture.serviceId
  ) {
    throw new Error("the production port constants and the neutral corpus disagree");
  }

  const approval = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧫️fixtures", "🗳️gis-map-proposal-approval-v1", "🔣️.json"), "utf8")) as {
    lifecycle: readonly { kind: string }[];
    cancelLifecycle: readonly { kind: string }[];
    limits: Readonly<Record<string, number>>;
    proposalHash: string;
    sampleJobId: string;
    preview: OraclePreview;
  };
  const approvalKinds = approval.lifecycle.map((row) => row.kind);
  const approvalCancelKinds = approval.cancelLifecycle.map((row) => row.kind);
  if (JSON.stringify(approvalKinds) !== JSON.stringify(fixture.crossFixture.serverLifecycleKinds)) throw new Error("the hub approval lifecycle and this corpus disagree");
  if (JSON.stringify(approvalCancelKinds) !== JSON.stringify(fixture.crossFixture.serverCancelLifecycleKinds)) throw new Error("the hub cancel lifecycle and this corpus disagree");
  for (const kind of [...approvalKinds, ...approvalCancelKinds]) if (!(kind in fixture.crossFixture.serverKindToPhase)) throw new Error(`hub lifecycle kind ${kind} has no rendered phase`);
  if (
    approval.limits.progressMaxCursor !== fixture.limits.progressMaxCursor ||
    approval.limits.eventPageMaxItems !== fixture.limits.eventPageMaxItems ||
    approval.limits.jobMaxLifetimeMs !== fixture.limits.jobMaxLifetimeMs ||
    approval.limits.requestMaxBytes !== fixture.limits.requestMaxBytes
  ) {
    throw new Error("the hub approval limits and this corpus disagree");
  }
  if (approval.proposalHash !== fixture.proposalHash || approval.sampleJobId !== fixture.sampleJobId) throw new Error("the hub approval identity and this corpus disagree");
  if (JSON.stringify(approval.preview) !== JSON.stringify(fixture.preview)) throw new Error("the Hub and browser preview projections disagree");
  const rustClients = [
    readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🦀️.rs"), "utf8"),
    readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🌉️mcp", "💡️inference", "🦀️.rs"), "utf8"),
  ];
  for (const source of rustClients) {
    if (!source.includes("pub struct GisMapInferencePreviewV1") || !source.includes("pub preview: Option<GisMapInferencePreviewV1>") || !source.includes("pub ring: [[f64; 2]; 5]"))
      throw new Error("a strict Rust Hub client is missing the optional typed preview DTO");
    if (/derive\([^\n]*\bEq\b[^\n]*\)\n[^\n]*\npub struct GisMapInferenceEventPageV1/u.test(source)) throw new Error("a Rust events page unsafely derives Eq through floating-point preview geometry");
  }
  const directoryRust = rustClients[0]!;
  if (/derive\([^\n]*\bEq\b[^\n]*\)\n[^\n]*\npub struct GisMapInferencePortStatusV1/u.test(directoryRust) || !directoryRust.includes("next.preview = if matches!(phase") || !directoryRust.includes("let preview_matches = current.preview.as_ref()"))
    throw new Error("the shared Rust inference reducer does not retain and require the typed preview safely");

  const shellRoot = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🖥️shell");
  const mirror = readFileSync(join(shellRoot, "🤖️generated", "🟦️.ts"), "utf8");
  const declared = /export type InferencePortPhase = ([^;]+);/u.exec(mirror);
  if (declared === null) throw new Error("the 🖥️shell TypeScript mirror declares no InferencePortPhase");
  const mirrored = declared[1]!.split("|").map((part) => part.trim().replace(/^"|"$/gu, ""));
  if (JSON.stringify(mirrored) !== JSON.stringify(fixture.phases)) throw new Error(`the 🖥️shell twin's phases differ from this corpus: ${JSON.stringify(mirrored)}`);
  const shellRust = readFileSync(join(shellRoot, "🧬️schema", "🦀️.rs"), "utf8");
  let twinStrings = 0;
  for (const phase of fixture.phases) {
    const row = production.GIS_MAP_INFERENCE_PORT_TEXT_V1[phase as keyof typeof production.GIS_MAP_INFERENCE_PORT_TEXT_V1];
    for (const text of [row.en, row.de]) {
      if (!shellRust.includes(JSON.stringify(text))) throw new Error(`the 🖥️shell Rust twin is missing the ${phase} text ${JSON.stringify(text)}`);
      twinStrings += 1;
    }
  }

  const history = await proveGisMapApprovalHistory(repoRoot);
  return { ajv: 1, hostileCorpora: hostileCorpora.length, transitions, strings, twinStrings, crossFixture: 3, ...Object.fromEntries(Object.entries(history).map(([key, value]) => [`history${key[0]!.toUpperCase()}${key.slice(1)}`, value])) };
}

/** ⚖️ `os:gis-map-inference-port-check` — the neutral corpus, its independent oracle, and the
 * browser port's own vitest laws. `--browser` additionally runs the worker suite. */
class GisMapInferencePortCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const browser = segments.includes("--browser");
    const { rest } = resolveTestLevel(segments.filter((segment) => segment !== "--browser"));
    const receipts = await proveGisMapInferencePortFixture(this.repoRoot);
    console.log(
      `gis-map-inference-port-oracle: ${Object.entries(receipts)
        .map(([key, value]) => `${key}=${value}`)
        .join(" ")}`,
    );
    if (browser) await runVitest(this.root, ["--testNamePattern", "gis map inference port", ...rest], "vitest.config.ts");
    console.log("gis-map-inference-port-check: no WGPU map rendering, no external model provider and no two-user process journey is run or claimed here.");
  }
}
//#endregion 💡️InferencePortCheck

//#region 🪪️DocumentOpeningAttemptCheck
/** 🪪️ Proves one Shell opening attempt owns every asynchronous outer worker lifecycle frame. */
async function proveDocumentOpeningAttempt(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory");
  const fixture = JSON.parse(readFileSync(join(root, "🧵️document-opening-attempt-v1.json"), "utf8"));
  const validate = await ownedExport(repoRoot, "directory", "DocumentOpeningAttemptV1");
  if (!validate(fixture)) throw new Error(`invalid document opening attempt fixture: ${JSON.stringify(validate.errors)}`);
  const deepEqual = (await import("fast-deep-equal")).default;
  const observed = fixture.cases.map((row: Record<string, unknown>) => {
    const current = row.current as string | null;
    const incoming = row.incoming as string | null;
    const valid = incoming === "a" || incoming === "b";
    if (!valid) return { accepted: false, replaced: false, currentAfter: current };
    if (row.kind === "open") {
      if (current === incoming) return { accepted: false, replaced: false, currentAfter: current };
      return { accepted: true, replaced: current !== null, currentAfter: incoming };
    }
    const accepted = current === incoming;
    return { accepted, replaced: false, currentAfter: accepted && row.kind === "close" ? null : current };
  });
  const expected = fixture.cases.map((row: Record<string, unknown>) => ({ accepted: row.accepted, replaced: row.replaced, currentAfter: row.currentAfter }));
  if (!deepEqual(observed, expected)) throw new Error("document opening attempt independent model differs from the corpus");
  const rebootstrapObserved = fixture.rebootstrapCases.map((row: Record<string, unknown>) => {
    const accepted = row.entryClient === row.messageClient;
    const removed = accepted && row.currentOwner !== null;
    const ownerAfter = removed ? null : row.currentOwner;
    return { messageAccepted: accepted, ownerRemoved: removed, ownerAfter, freshBaseZeroAccepted: accepted && ownerAfter === null };
  });
  const rebootstrapExpected = fixture.rebootstrapCases.map((row: Record<string, unknown>) => ({ messageAccepted: row.messageAccepted, ownerRemoved: row.ownerRemoved, ownerAfter: row.ownerAfter, freshBaseZeroAccepted: row.freshBaseZeroAccepted }));
  if (!deepEqual(rebootstrapObserved, rebootstrapExpected)) throw new Error("document rebootstrap owner retirement differs from the corpus");

  const protocol = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"), "utf8");
  const storeWire = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"), "utf8");
  const storeWorker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs"), "utf8");
  const patchHandoff = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts"), "utf8");
  const conforms = (candidateProtocol: string, candidateWorker: string, candidateShell: string, candidateStoreWire: string, candidateStoreWorker: string): boolean =>
    candidateProtocol.includes('readonly kind: "open"; readonly clientInstanceId?: string') &&
    candidateProtocol.includes('readonly kind: "close"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string') &&
    candidateProtocol.includes('readonly kind: "send"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string') &&
    candidateProtocol.includes("workerWireClientInstanceIdV1") &&
    candidateProtocol.includes('readonly clientInstanceId: string') &&
    candidateWorker.includes('type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; clientInstanceId: string; spaceId?: string }>') &&
    candidateWorker.includes("previous.clientInstanceId") &&
    candidateWorker.includes("request.clientInstanceId !== current.clientInstanceId") &&
    candidateWorker.includes("openClientInstanceId: request.clientInstanceId ?? crypto.randomUUID()") &&
    candidateWorker.includes('post({ kind: "event", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId') &&
    candidateWorker.includes("post({ ...offer, clientInstanceId: this.state.openClientInstanceId })") &&
    candidateWorker.includes("retireArtifactBeforeReplacement(runtimeKey);") &&
    candidateShell.includes('if (message.kind === "artifact-rebootstrap-required") {\n          if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);\n          const active = sessionRef.current;') &&
    candidateShell.includes("const openingAttempt = { clientInstanceId: crypto.randomUUID() }") &&
    candidateShell.includes('const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string;') &&
    candidateShell.includes("if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId)") &&
    candidateShell.includes('const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId') &&
    candidateStoreWire.includes("client_instance_id: Option<String>") &&
    candidateStoreWorker.includes("entry.client_instance_id") &&
    !patchHandoff.includes("clientInstanceId") &&
    !candidateProtocol.includes("type Bad =");
  if (!conforms(protocol, worker, shell, storeWire, storeWorker)) throw new Error("document opening attempt production correlation is incomplete");
  const hostiles = [
    [protocol, worker, shell.replace("const openingAttempt = { clientInstanceId: crypto.randomUUID() }", 'const openingAttempt = { clientInstanceId: "" }'), storeWire, storeWorker],
    [protocol, worker, shell.replace('const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string;', 'const socketActorReadyRef = useRef<Map<string, {'), storeWire, storeWorker],
    [protocol, worker, shell.replace("if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId)", "if (waiter !== undefined)"), storeWire, storeWorker],
    [protocol, worker, shell.replace('const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId', 'const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId'), storeWire, storeWorker],
    [protocol, worker.replace("openClientInstanceId: request.clientInstanceId ?? crypto.randomUUID()", "openClientInstanceId: crypto.randomUUID()"), shell, storeWire, storeWorker],
    [protocol, worker.replace('type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; clientInstanceId: string; spaceId?: string }>', 'type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; spaceId?: string }>'), shell, storeWire, storeWorker],
    [protocol, worker.replace("request.clientInstanceId !== current.clientInstanceId", "false"), shell, storeWire, storeWorker],
    [protocol, worker.replace("previous.clientInstanceId", "request.clientInstanceId"), shell, storeWire, storeWorker],
    [protocol, worker.replace('post({ kind: "event", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId', 'post({ kind: "event", documentId: state.config.documentId'), shell, storeWire, storeWorker],
    [protocol, worker.replace("post({ ...offer, clientInstanceId: this.state.openClientInstanceId })", "post(offer)"), shell, storeWire, storeWorker],
    [`${protocol}\ntype Bad = BrowserActorUiPatchOfferV1 & { readonly clientInstanceId: string };`, worker, shell, storeWire, storeWorker],
    [protocol, worker.replace("retireArtifactBeforeReplacement(runtimeKey);", "closeArtifactRuntime(runtimeKey);"), shell, storeWire, storeWorker],
    [protocol, worker, shell.replace('if (message.kind === "artifact-rebootstrap-required") {\n          if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);', 'if (message.kind === "artifact-rebootstrap-required") {'), storeWire, storeWorker],
  ];
  if (hostiles.length !== fixture.sourceHostiles.length) throw new Error("document opening attempt hostile count differs");
  hostiles.forEach(([candidateProtocol, candidateWorker, candidateShell, candidateStoreWire, candidateStoreWorker], index) => {
    if (conforms(candidateProtocol!, candidateWorker!, candidateShell!, candidateStoreWire!, candidateStoreWorker!)) throw new Error(`document opening attempt source oracle admitted ${fixture.sourceHostiles[index]}`);
  });
  console.log(`document-opening-attempt-oracle: AJV=1 opening=${observed.length} rebootstrap=${rebootstrapObserved.length} source-hostiles=${hostiles.length}`);
  return 1 + observed.length + rebootstrapObserved.length + hostiles.length;
}

class DocumentOpeningAttemptCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("document-opening-attempt-check accepts no arguments");
    const checks = await proveDocumentOpeningAttempt(this.repoRoot);
    await runVitest(this.root, ["--testNamePattern", "document opening attempt"], "vitest.config.ts");
    console.log(`document-opening-attempt-check: checks=${checks}`);
  }
}
//#endregion 🪪️DocumentOpeningAttemptCheck

//#region 🗺️GisMapPeerRebootstrapCheck
async function proveGisMapPeerRebootstrap(repoRoot: string): Promise<number> {
  const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/🗺️gis-map-peer-rebootstrap-v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = await ownedExport(repoRoot, "os", "GisMapPeerRebootstrapV1");
  if (!validate(fixture)) throw new Error(`invalid GIS Map peer rebootstrap fixture: ${JSON.stringify(validate.errors)}`);
  const deepEqual = (await import("fast-deep-equal")).default;
  const observed = fixture.clients.map((client: { clientInstanceId: string }) => {
    let phase = "live",
      currentPair: string | null = "prior",
      uiOwner: string | null = "prior",
      frontier: unknown = null,
      scene: unknown = null;
    for (const event of fixture.order as string[]) {
      if (event === "rebootstrap-required") {
        phase = "connecting";
        currentPair = null;
        uiOwner = null;
      } else if (event === "fresh-authority") phase = "authorized";
      else if (event === "artifact-bootstrap") currentPair = fixture.published.aggregateSha256;
      else if (event === "applied") frontier = fixture.published.frontier;
      else if (event === "scene-patch") {
        scene = fixture.published.scene;
        uiOwner = client.clientInstanceId;
      } else if (event === "patch-ack") phase = "live";
    }
    return { phase, currentPair, uiOwner, frontier, scene };
  });
  const expected = fixture.clients.map((client: { clientInstanceId: string }) => ({ phase: "live", currentPair: fixture.published.aggregateSha256, uiOwner: client.clientInstanceId, frontier: fixture.published.frontier, scene: fixture.published.scene }));
  if (!deepEqual(observed, expected) || !deepEqual(observed[0]?.frontier, observed[1]?.frontier) || !deepEqual(observed[0]?.scene, observed[1]?.scene)) throw new Error("GIS Map peer rebootstrap model diverged");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"), "utf8");
  const conforms = (candidateWorker: string, candidateShell: string): boolean =>
    candidateWorker.includes("control.space_id !== binding.spaceId || control.document_id !== state.config.documentId") &&
    candidateWorker.includes("await requireArtifactRebootstrap(state);") &&
    candidateWorker.includes("state.currentPack = null;") &&
    candidateWorker.includes("const authority = await requestDocumentSocketAuthority(state, binding);") &&
    candidateWorker.includes("await startArtifactBootstrap(state, bootstrap.ArtifactBootstrap") &&
    candidateWorker.includes("owner.assertApplied(status, lifetime);") &&
    candidateWorker.includes("await this.renderSurface(child, assertCurrent);") &&
    candidateWorker.includes("if (!equalFrontiers(bootstrap.required_tail_frontier, serverFrontier))") &&
    candidateShell.includes('retireBrowserActorUi(runtimeKey, "browser-actor-action: rebootstrap required");');
  if (!conforms(worker, shell)) throw new Error("GIS Map peer production rebootstrap closure is incomplete");
  const hostiles = [
    [worker.replace("control.space_id !== binding.spaceId || control.document_id !== state.config.documentId", "false"), shell],
    [worker, shell.replace('retireBrowserActorUi(runtimeKey, "browser-actor-action: rebootstrap required");', "void runtimeKey;")],
    [worker.replace("const authority = await requestDocumentSocketAuthority(state, binding);", "const authority = null as never;"), shell],
    [worker.replace("await startArtifactBootstrap(state, bootstrap.ArtifactBootstrap", "await Promise.resolve(state, bootstrap.ArtifactBootstrap"), shell],
    [worker.replace("owner.assertApplied(status, lifetime);", "void status; void lifetime;"), shell],
    [worker.replaceAll("await this.renderSurface(child, assertCurrent);", "void child; void assertCurrent;"), shell],
    [worker.replace("if (!equalFrontiers(bootstrap.required_tail_frontier, serverFrontier))", "if (false)"), shell],
  ];
  if (hostiles.length !== fixture.sourceHostiles.length) throw new Error("GIS Map peer source hostile count differs");
  hostiles.forEach(([candidateWorker, candidateShell], index) => {
    if (conforms(candidateWorker!, candidateShell!)) throw new Error(`GIS Map peer source oracle admitted ${fixture.sourceHostiles[index]}`);
  });
  console.log(`gis-map-peer-rebootstrap-oracle: AJV=1 deep-equal=${observed.length} source-hostiles=${hostiles.length}`);
  return 1 + observed.length + hostiles.length;
}
//#endregion 🗺️GisMapPeerRebootstrapCheck

async function proveMountedGisMapProbe(repoRoot: string): Promise<number> {
  const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔬️mounted-gis-map-probe-v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validateSource = await ownedExport(repoRoot, "renderer", "MountedGisMapProbeSourceV1");
  const validateProbe = await ownedExport(repoRoot, "renderer", "MountedGisMapProbeV1");
  if (!validateSource(fixture.source)) throw new Error(`invalid mounted GIS Map probe source: ${JSON.stringify(validateSource.errors)}`);
  if (!validateProbe(fixture.expected)) throw new Error(`invalid mounted GIS Map probe projection: ${JSON.stringify(validateProbe.errors)}`);
  const deepEqual = (await import("fast-deep-equal")).default;
  const observed = {
    scope: fixture.source.scope,
    clientInstanceId: fixture.source.clientInstanceId,
    activationGeneration: fixture.source.activationGeneration,
    catalogGenerationId: fixture.source.catalogGenerationId,
    componentSha256: fixture.source.componentSha256,
    descriptorSha256: fixture.source.descriptorSha256,
    browserActorSha256: fixture.source.browserActorSha256,
    activeCheckpointId: fixture.source.activeCheckpointId,
    descriptorDigestV1: fixture.source.descriptorDigestV1,
    frontier: structuredClone(fixture.source.frontier),
    uiRevision: fixture.source.uiRevision,
    rootKind: "tiled-map",
    regionIds: fixture.source.regions.map((region: { id: string }) => region.id).sort(),
  };
  if (!deepEqual(observed, fixture.expected)) throw new Error("mounted GIS Map independent projection differs from the corpus");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"), "utf8");
  const conforms = (candidateWorker: string, candidateShell: string): boolean =>
    candidateWorker.includes('kind: "browser-actor-ui-mounted"')
    && candidateWorker.includes("this.acknowledgedUiRevision !== this.renderedUiRevision")
    && candidateWorker.includes("activeCheckpointId: identity.checkpoint.checkpointId")
    && candidateWorker.includes("descriptorDigestV1: identity.checkpoint.descriptorDigestV1")
    && candidateWorker.includes("frontier,\n      uiRevision: this.renderedUiRevision")
    && candidateShell.includes("state.revision !== source.uiRevision")
    && candidateShell.includes("activeCheckpointId: source.activeCheckpointId")
    && candidateShell.includes("frontier: Object.freeze({ ...source.frontier")
    && candidateShell.includes('root?.component.type !== "surface" || root.component.kind !== "tiled-map"')
    && candidateShell.includes("decodePackValue(new Uint8Array(root.component.doc.bytes))")
    && candidateShell.includes("regionIds.includes(id)")
    && candidateShell.includes("__semioMountedGisMapProbe")
    && candidateShell.includes("retained?.identity === null || retained?.identity === undefined");
  if (!conforms(worker, shell)) throw new Error("mounted GIS Map production probe closure is incomplete");
  const hostiles = [
    [worker.replace('kind: "browser-actor-ui-mounted"', 'kind: "browser-actor-ui-patch"'), shell],
    [worker.replace("this.acknowledgedUiRevision !== this.renderedUiRevision", "false"), shell],
    [worker, shell.replace("state.revision !== source.uiRevision", "false")],
    [worker.replace("activeCheckpointId: identity.checkpoint.checkpointId", "activeCheckpointId: identity.package.componentSha256"), shell],
    [worker.replace("frontier,\n      uiRevision: this.renderedUiRevision", "frontier: identity.checkpoint.baselineFrontier,\n      uiRevision: this.renderedUiRevision"), shell],
    [worker, shell.replace('root?.component.type !== "surface" || root.component.kind !== "tiled-map"', "false")],
    [worker, shell.replace("decodePackValue(new Uint8Array(root.component.doc.bytes))", "{}")],
    [worker, shell.replace("regionIds.includes(id)", "false")],
  ];
  if (hostiles.length !== fixture.hostile.length) throw new Error("mounted GIS Map probe hostile count differs");
  hostiles.forEach(([candidateWorker, candidateShell], index) => {
    if (conforms(candidateWorker!, candidateShell!)) throw new Error(`mounted GIS Map probe source oracle admitted ${fixture.hostile[index]}`);
  });
  console.log(`mounted-gis-map-probe-oracle: AJV=1 deep-equal=1 source-hostiles=${hostiles.length}`);
  return 2 + hostiles.length;
}

/** 🧵️ Executes the authenticated Session lifecycle and exact 64 KiB page-transfer browser laws. */
class ColdDocumentPairBrowserCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const checks = (await proveGisMapPeerRebootstrap(this.repoRoot)) + (await proveMountedGisMapProbe(this.repoRoot));
    await runVitest(
      this.root,
      ["--testNamePattern", "(?:mounted GIS map probe|browser document first open (?:verifies server assets without a prior installed target|rejects hostile assets and retired owners before socket authority)|browser document actor (?:reservation activates only after an exact current socket Session|transfers one verified cold pair only after lifecycle ACK and exact page receipts)|browser document peers refetch the same exact pair after scoped rebootstrap|browser actor patch handoff validates the neutral schema)", ...rest],
      "vitest.config.ts",
    );
    console.log(`cold-document-pair-browser-check: peer-checks=${checks}`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("generate-wgpu", GenerateWgpuScript)
  .register("check-wgpu", CheckWgpuScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("gis-map-inference-port-check", GisMapInferencePortCheckScript)
  .register("document-opening-attempt-check", DocumentOpeningAttemptCheckScript)
  .register("cold-document-pair-browser-check", ColdDocumentPairBrowserCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
