/** 💡️ Scope `hub.inference` — the closed GIS inference wire contracts hub decodes and enforces.
 *
 * Schema authority: `./🔣️.json` (`https://semio.tech/schema/hub/inference/schema.json`).
 * Rust decoder authority: `./🦀️.rs`. Every `parse*` below mirrors one `$defs.<Export>`.
 */

export const INFERENCE_REQUEST_MAX_BYTES = 1024;
export const INFERENCE_SERVER_ID_MAX_BYTES = 96;
export const INFERENCE_INPUT_MAX_BYTES = 65536;
export const INFERENCE_RESULT_MAX_BYTES = 16384;
export const INFERENCE_PROPOSAL_MAX_BYTES = 4096;
export const INFERENCE_COMMAND_MAX_BYTES = 8192;
export const INFERENCE_IDENTITY_JSON_MAX_BYTES = 8192;
export const INFERENCE_SCHEMA_SCOPE = "hub.inference";
export const INFERENCE_SCHEMA_ID = "https://semio.tech/schema/hub/inference/schema.json";

const fail = (name: string): never => {
  throw new Error(`invalid ${name}`);
};
const rows = (value: unknown, keys: readonly string[], name: string, optional: readonly string[] = []): Record<string, unknown> => {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return fail(name);
  const row = value as Record<string, unknown>;
  const present = Object.keys(row);
  if (keys.some((key) => !Object.hasOwn(row, key)) || present.some((key) => !keys.includes(key) && !optional.includes(key))) return fail(name);
  return row;
};
const hex = (value: unknown, length: number): boolean => typeof value === "string" && value.length === length && /^[0-9a-f]+$/.test(value);
const serverId = (value: unknown): boolean => typeof value === "string" && value.length >= 1 && value.length <= INFERENCE_SERVER_ID_MAX_BYTES && /^[A-Za-z0-9._:-]+$/.test(value);
const editId = (value: unknown): boolean => typeof value === "string" && value.length <= INFERENCE_SERVER_ID_MAX_BYTES && /^[A-Za-z0-9._:-]*$/.test(value);
const uint = (value: unknown): boolean => typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
const text = (value: unknown, maximum: number): boolean => typeof value === "string" && value.length >= 1 && value.length <= maximum;
const hexBytes = (value: unknown, maximum: number): boolean => typeof value === "string" && value.length <= maximum && /^(?:[0-9a-f]{2})*$/.test(value);
const member = <T extends string>(value: unknown, allowed: readonly T[]): boolean => typeof value === "string" && (allowed as readonly string[]).includes(value);

/** 🆔️ Every server-minted identifier hub renders on the inference wire. */
export type InferenceServerIdV1 = string;
export function parseInferenceServerIdV1(value: unknown): InferenceServerIdV1 {
  return serverId(value) ? (value as string) : fail("hub.inference/InferenceServerIdV1");
}

export type InferenceDocumentScopeV1 = { readonly spaceId: string; readonly documentId: string };
export function parseInferenceDocumentScopeV1(value: unknown): InferenceDocumentScopeV1 {
  const row = rows(value, ["spaceId", "documentId"], "hub.inference/InferenceDocumentScopeV1");
  if (!serverId(row.spaceId) || !serverId(row.documentId)) return fail("hub.inference/InferenceDocumentScopeV1");
  return { spaceId: row.spaceId as string, documentId: row.documentId as string };
}

export const INFERENCE_JOB_STATES = ["accepted", "running", "succeeded", "failed", "cancelled"] as const;
export type InferenceJobStateV1 = (typeof INFERENCE_JOB_STATES)[number];
export function parseInferenceJobStateV1(value: unknown): InferenceJobStateV1 {
  return member(value, INFERENCE_JOB_STATES) ? (value as InferenceJobStateV1) : fail("hub.inference/InferenceJobStateV1");
}

export const INFERENCE_PROPOSAL_STATES = ["none", "offered", "approved", "stale", "cancelled"] as const;
export type InferenceProposalStateV1 = (typeof INFERENCE_PROPOSAL_STATES)[number];
export function parseInferenceProposalStateV1(value: unknown): InferenceProposalStateV1 {
  return member(value, INFERENCE_PROPOSAL_STATES) ? (value as InferenceProposalStateV1) : fail("hub.inference/InferenceProposalStateV1");
}

export const INFERENCE_LIFECYCLE_KINDS = ["accepted", "running", "succeeded", "failed", "cancelled", "cancel-requested", "proposal-cancelled", "proposal-stale", "approval-prepared", "approved"] as const;
export type InferenceLifecycleKindV1 = (typeof INFERENCE_LIFECYCLE_KINDS)[number];
export function parseInferenceLifecycleKindV1(value: unknown): InferenceLifecycleKindV1 {
  return member(value, INFERENCE_LIFECYCLE_KINDS) ? (value as InferenceLifecycleKindV1) : fail("hub.inference/InferenceLifecycleKindV1");
}

/** 💡️ Closed client intent; all identity, snapshots, and execution authority remain server-owned. */
export type InferenceRequestV1 = {
  readonly schema: "semio.hub.inference-request/v1";
  readonly version: 1;
  readonly requestId: string;
  readonly serviceId: "s.gis.gismap.inference";
  readonly policyVersion: 1;
  readonly lifetimeMs: number;
};
export function parseInferenceRequestV1(value: unknown): InferenceRequestV1 {
  const row = rows(value, ["schema", "version", "requestId", "serviceId", "policyVersion", "lifetimeMs"], "hub.inference/InferenceRequestV1");
  if (row.schema !== "semio.hub.inference-request/v1" || row.version !== 1 || !hex(row.requestId, 32) || row.serviceId !== "s.gis.gismap.inference" || row.policyVersion !== 1
    || !uint(row.lifetimeMs) || (row.lifetimeMs as number) < 1 || (row.lifetimeMs as number) > 120000) return fail("hub.inference/InferenceRequestV1");
  return { schema: "semio.hub.inference-request/v1", version: 1, requestId: row.requestId as string, serviceId: "s.gis.gismap.inference", policyVersion: 1, lifetimeMs: row.lifetimeMs as number };
}

export const INFERENCE_RECONCILE_REQUEST_MAX_BYTES = 256;
export type InferenceJobReconcileRequestV1 = { readonly schema: "semio.hub.inference-job-reconcile/v1"; readonly version: 1; readonly requestId: string };
export function parseInferenceJobReconcileRequestV1(value: unknown): InferenceJobReconcileRequestV1 {
  const name = "hub.inference/InferenceJobReconcileRequestV1";
  const row = rows(value, ["schema", "version", "requestId"], name);
  if (row.schema !== "semio.hub.inference-job-reconcile/v1" || row.version !== 1 || !hex(row.requestId, 32)) return fail(name);
  return { schema: "semio.hub.inference-job-reconcile/v1", version: 1, requestId: row.requestId as string };
}

/** 🧬️ The exact retained parent dialect the frozen Map binding admitted; never a client label. */
export type InferenceParentDialectV1 = { readonly artifactKind: "s.gis.gismap"; readonly standard: "1"; readonly subset: "*" };
export function parseInferenceParentDialectV1(value: unknown): InferenceParentDialectV1 {
  const row = rows(value, ["artifactKind", "standard", "subset"], "hub.inference/InferenceParentDialectV1");
  if (row.artifactKind !== "s.gis.gismap" || row.standard !== "1" || row.subset !== "*") return fail("hub.inference/InferenceParentDialectV1");
  return { artifactKind: "s.gis.gismap", standard: "1", subset: "*" };
}

/** 🧊️ Every frozen executable fact of the selected GIS Map binding, carried inside job identity. */
export type InferenceBindingIdentityV1 = {
  readonly digest: string;
  readonly catalogGenerationId: string;
  readonly packageId: "semio:gis";
  readonly packageVersion: string;
  readonly componentSha256: string;
  readonly componentBlake3: string;
  readonly artifactKind: "s.gis.gismap";
  readonly documentSchema: "gis.map";
  readonly parentDialect: InferenceParentDialectV1;
  readonly surfaceId: "s.gis.gismap@1/*#editor";
  readonly grantedMode: "read-write-observe";
  readonly serviceId: "s.gis.gismap.inference";
  readonly serviceVersion: 1;
  readonly algorithmVersion: 1;
};
export function parseInferenceBindingIdentityV1(value: unknown): InferenceBindingIdentityV1 {
  const name = "hub.inference/InferenceBindingIdentityV1";
  const row = rows(value, ["digest", "catalogGenerationId", "packageId", "packageVersion", "componentSha256", "componentBlake3", "artifactKind", "documentSchema", "parentDialect", "surfaceId", "grantedMode", "serviceId", "serviceVersion", "algorithmVersion"], name);
  if (!["digest", "catalogGenerationId", "componentSha256", "componentBlake3"].every((key) => hex(row[key], 64)) || row.packageId !== "semio:gis" || !serverId(row.packageVersion)
    || row.artifactKind !== "s.gis.gismap" || row.documentSchema !== "gis.map" || row.surfaceId !== "s.gis.gismap@1/*#editor" || row.grantedMode !== "read-write-observe"
    || row.serviceId !== "s.gis.gismap.inference" || row.serviceVersion !== 1 || row.algorithmVersion !== 1) return fail(name);
  return {
    digest: row.digest as string, catalogGenerationId: row.catalogGenerationId as string, packageId: "semio:gis", packageVersion: row.packageVersion as string,
    componentSha256: row.componentSha256 as string, componentBlake3: row.componentBlake3 as string, artifactKind: "s.gis.gismap", documentSchema: "gis.map",
    parentDialect: parseInferenceParentDialectV1(row.parentDialect), surfaceId: "s.gis.gismap@1/*#editor", grantedMode: "read-write-observe",
    serviceId: "s.gis.gismap.inference", serviceVersion: 1, algorithmVersion: 1,
  };
}

/** 🪪️ The immutable server-selected identity of one accepted inference job. */
export type InferenceIdentityV1 = {
  readonly request: InferenceRequestV1;
  readonly userId: string;
  readonly sessionId: string;
  readonly authorizationGeneration: number;
  readonly spaceId: string;
  readonly documentId: string;
  readonly descriptorDigest: string;
  readonly binding: InferenceBindingIdentityV1;
  readonly headOrdinal: number;
  readonly headEditId: string;
  readonly lastCommitSeq: number;
  readonly chainHash: string;
  readonly inputHash: string;
};
export function parseInferenceIdentityV1(value: unknown): InferenceIdentityV1 {
  const name = "hub.inference/InferenceIdentityV1";
  const row = rows(value, ["request", "userId", "sessionId", "authorizationGeneration", "spaceId", "documentId", "descriptorDigest", "binding", "headOrdinal", "headEditId", "lastCommitSeq", "chainHash", "inputHash"], name);
  if (!["userId", "sessionId", "spaceId", "documentId"].every((key) => serverId(row[key])) || !["descriptorDigest", "chainHash", "inputHash"].every((key) => hex(row[key], 64))
    || !uint(row.authorizationGeneration) || (row.authorizationGeneration as number) < 1 || !uint(row.headOrdinal) || !uint(row.lastCommitSeq) || !editId(row.headEditId)
    || (row.headEditId === "" && row.headOrdinal !== 0)) return fail(name);
  return {
    request: parseInferenceRequestV1(row.request), userId: row.userId as string, sessionId: row.sessionId as string, authorizationGeneration: row.authorizationGeneration as number,
    spaceId: row.spaceId as string, documentId: row.documentId as string, descriptorDigest: row.descriptorDigest as string, binding: parseInferenceBindingIdentityV1(row.binding),
    headOrdinal: row.headOrdinal as number, headEditId: row.headEditId as string, lastCommitSeq: row.lastCommitSeq as number, chainHash: row.chainHash as string, inputHash: row.inputHash as string,
  };
}

/** ✅️ Approval identifies an offered job and digest; execution authority is never client supplied. */
export type InferenceApprovalRequestV1 = { readonly schema: "semio.hub.inference-approval/v1"; readonly version: 1; readonly jobId: string; readonly proposalHash: string };
export function parseInferenceApprovalRequestV1(value: unknown): InferenceApprovalRequestV1 {
  const name = "hub.inference/InferenceApprovalRequestV1";
  const row = rows(value, ["schema", "version", "jobId", "proposalHash"], name);
  if (row.schema !== "semio.hub.inference-approval/v1" || row.version !== 1 || !hex(row.jobId, 32) || !hex(row.proposalHash, 64)) return fail(name);
  return { schema: "semio.hub.inference-approval/v1", version: 1, jobId: row.jobId as string, proposalHash: row.proposalHash as string };
}

/** 🧭️ The exact durable document tail an undo intent or handle pins. */
export type GisMapDocumentFrontierV1 = { readonly documentId: string; readonly headEditOrdinal: number; readonly headEditId: string; readonly lastCommitSeq: number; readonly chainSha256: string };
export function parseGisMapDocumentFrontierV1(value: unknown): GisMapDocumentFrontierV1 {
  const name = "hub.inference/GisMapDocumentFrontierV1";
  const row = rows(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainSha256"], name);
  if (!serverId(row.documentId) || !uint(row.headEditOrdinal) || !editId(row.headEditId) || !uint(row.lastCommitSeq) || !hex(row.chainSha256, 64)) return fail(name);
  return { documentId: row.documentId as string, headEditOrdinal: row.headEditOrdinal as number, headEditId: row.headEditId as string, lastCommitSeq: row.lastCommitSeq as number, chainSha256: row.chainSha256 as string };
}

/** ↩️ Owner-bound durable undo locator minted only from a committed GIS approval witness. */
export type GisMapApprovalUndoHandleV1 = { readonly targetId: string; readonly expectedCurrent: GisMapDocumentFrontierV1 };
export function parseGisMapApprovalUndoHandleV1(value: unknown): GisMapApprovalUndoHandleV1 {
  const name = "hub.inference/GisMapApprovalUndoHandleV1";
  const row = rows(value, ["targetId", "expectedCurrent"], name);
  if (!hex(row.targetId, 32)) return fail(name);
  return { targetId: row.targetId as string, expectedCurrent: parseGisMapDocumentFrontierV1(row.expectedCurrent) };
}

/** 📨️ Closed undo intent: a server-minted target, exact tail expectation and retry identity. */
export type GisMapApprovalUndoRequestV1 = { readonly schema: "semio.hub.gis-map-approval-undo/v1"; readonly version: 1; readonly targetId: string; readonly idempotencyKey: string; readonly expectedCurrent: GisMapDocumentFrontierV1 };
export function parseGisMapApprovalUndoRequestV1(value: unknown): GisMapApprovalUndoRequestV1 {
  const name = "hub.inference/GisMapApprovalUndoRequestV1";
  const row = rows(value, ["schema", "version", "targetId", "idempotencyKey", "expectedCurrent"], name);
  if (row.schema !== "semio.hub.gis-map-approval-undo/v1" || row.version !== 1 || !hex(row.targetId, 32) || !hex(row.idempotencyKey, 32)) return fail(name);
  return { schema: "semio.hub.gis-map-approval-undo/v1", version: 1, targetId: row.targetId as string, idempotencyKey: row.idempotencyKey as string, expectedCurrent: parseGisMapDocumentFrontierV1(row.expectedCurrent) };
}

/** 🎯️ The server-minted undo target a committed approval witness materialized. */
export type GisMapApprovalUndoTargetV1 = {
  readonly schema: "semio.hub.gis-map-approval-undo-target/v1";
  readonly targetId: string;
  readonly scope: InferenceDocumentScopeV1;
  readonly originalJobId: string;
  readonly originalMutationId: string;
  readonly originalCommandHash: string;
  readonly proposalHash: string;
  readonly committedWitnessDigest: string;
  readonly beforeFrontier: GisMapDocumentFrontierV1;
  readonly afterFrontier: GisMapDocumentFrontierV1;
  readonly ownerUserId: string;
  readonly ownerSessionId: string;
  readonly authorizationGeneration: number;
};
export function parseGisMapApprovalUndoTargetV1(value: unknown): GisMapApprovalUndoTargetV1 {
  const name = "hub.inference/GisMapApprovalUndoTargetV1";
  const row = rows(value, ["schema", "targetId", "scope", "originalJobId", "originalMutationId", "originalCommandHash", "proposalHash", "committedWitnessDigest", "beforeFrontier", "afterFrontier", "ownerUserId", "ownerSessionId", "authorizationGeneration"], name);
  if (row.schema !== "semio.hub.gis-map-approval-undo-target/v1" || !["targetId", "originalJobId", "originalMutationId"].every((key) => hex(row[key], 32))
    || !["originalCommandHash", "proposalHash", "committedWitnessDigest"].every((key) => hex(row[key], 64)) || !serverId(row.ownerUserId) || !serverId(row.ownerSessionId)
    || !uint(row.authorizationGeneration) || (row.authorizationGeneration as number) < 1) return fail(name);
  return {
    schema: "semio.hub.gis-map-approval-undo-target/v1", targetId: row.targetId as string, scope: parseInferenceDocumentScopeV1(row.scope), originalJobId: row.originalJobId as string,
    originalMutationId: row.originalMutationId as string, originalCommandHash: row.originalCommandHash as string, proposalHash: row.proposalHash as string,
    committedWitnessDigest: row.committedWitnessDigest as string, beforeFrontier: parseGisMapDocumentFrontierV1(row.beforeFrontier), afterFrontier: parseGisMapDocumentFrontierV1(row.afterFrontier),
    ownerUserId: row.ownerUserId as string, ownerSessionId: row.ownerSessionId as string, authorizationGeneration: row.authorizationGeneration as number,
  };
}

/** 🧾️ Durable inverse receipt published only after the second verified WAL decision and pair. */
export type GisMapApprovalUndoReceiptV1 = {
  readonly schema: "semio.hub.gis-map-approval-undo-receipt/v1";
  readonly targetId: string;
  readonly originalJobId: string;
  readonly mutationId: string;
  readonly commandHash: string;
  readonly applied: boolean;
  readonly replayed: boolean;
  readonly frontier: GisMapDocumentFrontierV1;
};
export function parseGisMapApprovalUndoReceiptV1(value: unknown): GisMapApprovalUndoReceiptV1 {
  const name = "hub.inference/GisMapApprovalUndoReceiptV1";
  const row = rows(value, ["schema", "targetId", "originalJobId", "mutationId", "commandHash", "applied", "replayed", "frontier"], name);
  if (row.schema !== "semio.hub.gis-map-approval-undo-receipt/v1" || !["targetId", "originalJobId", "mutationId"].every((key) => hex(row[key], 32)) || !hex(row.commandHash, 64)
    || typeof row.applied !== "boolean" || typeof row.replayed !== "boolean") return fail(name);
  return { schema: "semio.hub.gis-map-approval-undo-receipt/v1", targetId: row.targetId as string, originalJobId: row.originalJobId as string, mutationId: row.mutationId as string, commandHash: row.commandHash as string, applied: row.applied, replayed: row.replayed, frontier: parseGisMapDocumentFrontierV1(row.frontier) };
}

/** ✅️ The closed approval outcome; `applied` is true only after a real committed-WAL witness. */
export type InferenceApprovalReceiptV1 = {
  readonly schema: "semio.hub.inference-approval-receipt/v1";
  readonly jobId: string;
  readonly mutationId: string;
  readonly commandHash: string;
  readonly proposalHash: string;
  readonly applied: boolean;
  readonly undo: GisMapApprovalUndoHandleV1;
};
export function parseInferenceApprovalReceiptV1(value: unknown): InferenceApprovalReceiptV1 {
  const name = "hub.inference/InferenceApprovalReceiptV1";
  const row = rows(value, ["schema", "jobId", "mutationId", "commandHash", "proposalHash", "applied", "undo"], name);
  if (row.schema !== "semio.hub.inference-approval-receipt/v1" || !hex(row.jobId, 32) || !hex(row.mutationId, 32) || !hex(row.commandHash, 64) || !hex(row.proposalHash, 64) || typeof row.applied !== "boolean") return fail(name);
  return { schema: "semio.hub.inference-approval-receipt/v1", jobId: row.jobId as string, mutationId: row.mutationId as string, commandHash: row.commandHash as string, proposalHash: row.proposalHash as string, applied: row.applied, undo: parseGisMapApprovalUndoHandleV1(row.undo) };
}

/** 📤️ The ledger-only outbox row a prepared approval reconciles exactly once. */
export type InferenceApprovalOutboxV1 = {
  readonly proposal: "ledger-only-proposal";
  readonly commandHex: string;
  readonly jobId: string;
  readonly mutationId: string;
  readonly proposalHash: string;
  readonly commandHash: string;
  readonly preparedCount: 1;
  readonly reconciledCount: 1;
};
export function parseInferenceApprovalOutboxV1(value: unknown): InferenceApprovalOutboxV1 {
  const name = "hub.inference/InferenceApprovalOutboxV1";
  const row = rows(value, ["proposal", "commandHex", "jobId", "mutationId", "proposalHash", "commandHash", "preparedCount", "reconciledCount"], name);
  if (row.proposal !== "ledger-only-proposal" || !hexBytes(row.commandHex, 16384) || (row.commandHex as string).length < 2 || !hex(row.jobId, 32) || !hex(row.mutationId, 32)
    || !hex(row.proposalHash, 64) || !hex(row.commandHash, 64) || row.preparedCount !== 1 || row.reconciledCount !== 1) return fail(name);
  return { proposal: "ledger-only-proposal", commandHex: row.commandHex as string, jobId: row.jobId as string, mutationId: row.mutationId as string, proposalHash: row.proposalHash as string, commandHash: row.commandHash as string, preparedCount: 1, reconciledCount: 1 };
}

/** ⏸️ One integration-fixtures checkpoint control frame on the fixed inherited descriptor. */
export const GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES = 256;
export type GisInferenceCheckpointControlFrameV1 = {
  readonly schema: "semio.hub.gis-inference-checkpoint-control/v1";
  readonly version: 1;
  readonly sequence: number;
  readonly kind: "progress-persisted" | "release";
  readonly jobId: string;
  readonly progressCursor: number;
  readonly completed: number;
  readonly total: number;
};
export function parseGisInferenceCheckpointControlFrameV1(value: unknown): GisInferenceCheckpointControlFrameV1 {
  const name = "hub.inference/GisInferenceCheckpointControlFrameV1";
  const row = rows(value, ["schema", "version", "sequence", "kind", "jobId", "progressCursor", "completed", "total"], name);
  if (row.schema !== "semio.hub.gis-inference-checkpoint-control/v1" || row.version !== 1 || !uint(row.sequence) || (row.sequence as number) < 1 || (row.sequence as number) > 2
    || !member(row.kind, ["progress-persisted", "release"] as const) || !hex(row.jobId, 32) || !uint(row.progressCursor) || (row.progressCursor as number) < 1 || (row.progressCursor as number) > 16
    || !uint(row.completed) || (row.completed as number) < 1 || !uint(row.total) || (row.total as number) < 1 || (row.completed as number) > (row.total as number)) return fail(name);
  return {
    schema: "semio.hub.gis-inference-checkpoint-control/v1", version: 1, sequence: row.sequence as number, kind: row.kind as "progress-persisted" | "release", jobId: row.jobId as string,
    progressCursor: row.progressCursor as number, completed: row.completed as number, total: row.total as number,
  };
}

export const GIS_INFERENCE_CHECKPOINT_CONTROL_DIRECTIONS = ["hub-to-runner", "runner-to-hub"] as const;
export type GisInferenceCheckpointControlDirectionV1 = (typeof GIS_INFERENCE_CHECKPOINT_CONTROL_DIRECTIONS)[number];
export function parseGisInferenceCheckpointControlDirectionV1(value: unknown): GisInferenceCheckpointControlDirectionV1 {
  return member(value, GIS_INFERENCE_CHECKPOINT_CONTROL_DIRECTIONS) ? (value as GisInferenceCheckpointControlDirectionV1) : fail("hub.inference/GisInferenceCheckpointControlDirectionV1");
}

/** 🗺️ The bounded, host-only geometry an owner may inspect before approving a proposal. */
export type GisMapInferencePreviewV1 = { readonly schema: "semio.hub.gis-map-inference-preview/v1"; readonly jobId: string; readonly proposalHash: string; readonly regionId: string; readonly ring: readonly (readonly number[])[] };
export function parseGisMapInferencePreviewV1(value: unknown): GisMapInferencePreviewV1 {
  const name = "hub.inference/GisMapInferencePreviewV1";
  const row = rows(value, ["schema", "jobId", "proposalHash", "regionId", "ring"], name);
  const ring = row.ring;
  if (row.schema !== "semio.hub.gis-map-inference-preview/v1" || !hex(row.jobId, 32) || !hex(row.proposalHash, 64) || typeof row.regionId !== "string" || !/^inference-[0-9a-f]{32}$/.test(row.regionId)
    || !Array.isArray(ring) || ring.length !== 5
    || !ring.every((point) => Array.isArray(point) && point.length === 2 && point.every((axis) => typeof axis === "number" && Number.isFinite(axis)))
    || !ring.every((point) => (point as number[])[0] >= -180 && (point as number[])[0] <= 180 && (point as number[])[1] >= -90 && (point as number[])[1] <= 90)) return fail(name);
  return { schema: "semio.hub.gis-map-inference-preview/v1", jobId: row.jobId as string, proposalHash: row.proposalHash as string, regionId: row.regionId, ring: (ring as number[][]).map((point) => [...point]) };
}

export type InferenceMapBoundsV1 = { readonly lonMin: number; readonly lonMax: number; readonly latMin: number; readonly latMax: number };
export function parseInferenceMapBoundsV1(value: unknown): InferenceMapBoundsV1 {
  const name = "hub.inference/InferenceMapBoundsV1";
  const row = rows(value, ["lonMin", "lonMax", "latMin", "latMax"], name);
  const finite = (key: string, limit: number): boolean => typeof row[key] === "number" && Number.isFinite(row[key]) && Math.abs(row[key] as number) <= limit;
  if (!finite("lonMin", 180) || !finite("lonMax", 180) || !finite("latMin", 90) || !finite("latMax", 90)) return fail(name);
  return { lonMin: row.lonMin as number, lonMax: row.lonMax as number, latMin: row.latMin as number, latMax: row.latMax as number };
}

export type InferenceMapSummaryV1 = { readonly positionCount: number; readonly routeCount: number; readonly regionCount: number; readonly bounds: InferenceMapBoundsV1 };
export function parseInferenceMapSummaryV1(value: unknown): InferenceMapSummaryV1 {
  const name = "hub.inference/InferenceMapSummaryV1";
  const row = rows(value, ["positionCount", "routeCount", "regionCount", "bounds"], name);
  if (!["positionCount", "routeCount", "regionCount"].every((key) => uint(row[key]) && (row[key] as number) <= 65536)) return fail(name);
  return { positionCount: row.positionCount as number, routeCount: row.routeCount as number, regionCount: row.regionCount as number, bounds: parseInferenceMapBoundsV1(row.bounds) };
}

export type InferenceProgressV1 = { readonly cursor: number; readonly runEpoch: number; readonly completed: number; readonly total: number; readonly atMs: number };
export function parseInferenceProgressV1(value: unknown): InferenceProgressV1 {
  const name = "hub.inference/InferenceProgressV1";
  const row = rows(value, ["cursor", "runEpoch", "completed", "total", "atMs"], name);
  if (!["cursor", "runEpoch", "completed", "total", "atMs"].every((key) => uint(row[key]))) return fail(name);
  return { cursor: row.cursor as number, runEpoch: row.runEpoch as number, completed: row.completed as number, total: row.total as number, atMs: row.atMs as number };
}

export type InferenceEventV1 = { readonly ordinal: number; readonly kind: InferenceLifecycleKindV1; readonly atMs: number };
export function parseInferenceEventV1(value: unknown): InferenceEventV1 {
  const name = "hub.inference/InferenceEventV1";
  const row = rows(value, ["ordinal", "kind", "atMs"], name);
  if (!uint(row.ordinal) || !uint(row.atMs)) return fail(name);
  return { ordinal: row.ordinal as number, kind: parseInferenceLifecycleKindV1(row.kind), atMs: row.atMs as number };
}

/** 🧾️ The closed receipt a submitted job returns; it never carries private result or base bytes. */
export type InferenceJobReceiptV1 = {
  readonly schema: "semio.hub.inference-job-receipt/v1";
  readonly jobId: string;
  readonly state: InferenceJobStateV1;
  readonly proposalState: InferenceProposalStateV1;
  readonly proposalHash: string | null;
  readonly cursor: number;
  readonly expiresAtMs: number;
};
export function parseInferenceJobReceiptV1(value: unknown): InferenceJobReceiptV1 {
  const name = "hub.inference/InferenceJobReceiptV1";
  const row = rows(value, ["schema", "jobId", "state", "proposalState", "proposalHash", "cursor", "expiresAtMs"], name);
  if (row.schema !== "semio.hub.inference-job-receipt/v1" || !hex(row.jobId, 32) || (row.proposalHash !== null && !hex(row.proposalHash, 64)) || !uint(row.cursor) || !uint(row.expiresAtMs)) return fail(name);
  return { schema: "semio.hub.inference-job-receipt/v1", jobId: row.jobId as string, state: parseInferenceJobStateV1(row.state), proposalState: parseInferenceProposalStateV1(row.proposalState), proposalHash: row.proposalHash as string | null, cursor: row.cursor as number, expiresAtMs: row.expiresAtMs as number };
}

export const INFERENCE_JOB_RECONCILE_APPROVAL_STATES = ["available", "undo-prepared", "undone"] as const;
export type InferenceJobReconcileApprovalStateV1 = (typeof INFERENCE_JOB_RECONCILE_APPROVAL_STATES)[number];
export type InferenceJobReconcileApprovalV1 = { readonly state: InferenceJobReconcileApprovalStateV1; readonly receipt: InferenceApprovalReceiptV1 | null };
export function parseInferenceJobReconcileApprovalV1(value: unknown): InferenceJobReconcileApprovalV1 {
  const name = "hub.inference/InferenceJobReconcileApprovalV1";
  const row = rows(value, ["state", "receipt"], name);
  if (!member(row.state, INFERENCE_JOB_RECONCILE_APPROVAL_STATES)) return fail(name);
  const state = row.state as InferenceJobReconcileApprovalStateV1;
  if (state === "available") return { state, receipt: parseInferenceApprovalReceiptV1(row.receipt) };
  if (row.receipt !== null) return fail(name);
  return { state, receipt: null };
}

export type InferenceJobReconcilePageV1 = {
  readonly jobId: string;
  readonly state: InferenceJobStateV1;
  readonly proposalState: InferenceProposalStateV1;
  readonly cancelRequested: boolean;
  readonly expired: boolean;
  readonly proposalHash: string | null;
  readonly events: readonly InferenceEventV1[];
  readonly progress: readonly InferenceProgressV1[];
  readonly nextCursor: number;
};
export function parseInferenceJobReconcilePageV1(value: unknown): InferenceJobReconcilePageV1 {
  const name = "hub.inference/InferenceJobReconcilePageV1";
  const row = rows(value, ["jobId", "state", "proposalState", "cancelRequested", "expired", "proposalHash", "events", "progress", "nextCursor"], name);
  if (!hex(row.jobId, 32) || typeof row.cancelRequested !== "boolean" || typeof row.expired !== "boolean" || (row.proposalHash !== null && !hex(row.proposalHash, 64))
    || !Array.isArray(row.events) || row.events.length > 8 || !Array.isArray(row.progress) || row.progress.length > 16 || !uint(row.nextCursor)) return fail(name);
  return {
    jobId: row.jobId as string, state: parseInferenceJobStateV1(row.state), proposalState: parseInferenceProposalStateV1(row.proposalState), cancelRequested: row.cancelRequested,
    expired: row.expired, proposalHash: row.proposalHash as string | null, events: row.events.map(parseInferenceEventV1), progress: row.progress.map(parseInferenceProgressV1), nextCursor: row.nextCursor as number,
  };
}

export type InferenceJobReconcileJobV1 = { readonly receipt: InferenceJobReceiptV1; readonly page: InferenceJobReconcilePageV1; readonly approval: InferenceJobReconcileApprovalV1 | null };
export function parseInferenceJobReconcileJobV1(value: unknown): InferenceJobReconcileJobV1 {
  const name = "hub.inference/InferenceJobReconcileJobV1";
  const row = rows(value, ["receipt", "page", "approval"], name);
  const receipt = parseInferenceJobReceiptV1(row.receipt);
  const page = parseInferenceJobReconcilePageV1(row.page);
  if (receipt.jobId !== page.jobId || receipt.state !== page.state || receipt.proposalState !== page.proposalState || receipt.proposalHash !== page.proposalHash || receipt.cursor !== page.nextCursor) return fail(name);
  const approval = row.approval === null ? null : parseInferenceJobReconcileApprovalV1(row.approval);
  if ((page.proposalState === "approved") !== (approval !== null)) return fail(name);
  if (approval !== null && (page.proposalHash === null || (approval.receipt !== null && (approval.receipt.jobId !== page.jobId || !approval.receipt.applied || approval.receipt.proposalHash !== page.proposalHash)))) return fail(name);
  return { receipt, page, approval };
}

export type InferenceJobReconcileResultV1 = {
  readonly schema: "semio.hub.inference-job-reconcile-result/v1";
  readonly version: 1;
  readonly requestId: string;
  readonly found: boolean;
  readonly job: InferenceJobReconcileJobV1 | null;
};
export function parseInferenceJobReconcileResultV1(value: unknown): InferenceJobReconcileResultV1 {
  const name = "hub.inference/InferenceJobReconcileResultV1";
  const row = rows(value, ["schema", "version", "requestId", "found", "job"], name);
  if (row.schema !== "semio.hub.inference-job-reconcile-result/v1" || row.version !== 1 || !hex(row.requestId, 32) || typeof row.found !== "boolean") return fail(name);
  if (!row.found) {
    if (row.job !== null) return fail(name);
    return { schema: "semio.hub.inference-job-reconcile-result/v1", version: 1, requestId: row.requestId as string, found: false, job: null };
  }
  if (row.job === null) return fail(name);
  return { schema: "semio.hub.inference-job-reconcile-result/v1", version: 1, requestId: row.requestId as string, found: true, job: parseInferenceJobReconcileJobV1(row.job) };
}

/** 📃️ The owner-private bounded page a single `events` read returns. */
export type InferenceEventPageV1 = {
  readonly schema: "semio.hub.inference-job-events/v1";
  readonly jobId: string;
  readonly state: InferenceJobStateV1;
  readonly proposalState: InferenceProposalStateV1;
  readonly cancelRequested: boolean;
  readonly stale: boolean;
  readonly proposalHash: string | null;
  readonly preview?: GisMapInferencePreviewV1;
  readonly events: readonly InferenceEventV1[];
  readonly progress: readonly InferenceProgressV1[];
  readonly nextCursor: number;
};
export function parseInferenceEventPageV1(value: unknown): InferenceEventPageV1 {
  const name = "hub.inference/InferenceEventPageV1";
  const row = rows(value, ["schema", "jobId", "state", "proposalState", "cancelRequested", "stale", "proposalHash", "events", "progress", "nextCursor"], name, ["preview"]);
  if (row.schema !== "semio.hub.inference-job-events/v1" || !hex(row.jobId, 32) || typeof row.cancelRequested !== "boolean" || typeof row.stale !== "boolean"
    || (row.proposalHash !== null && !hex(row.proposalHash, 64)) || !Array.isArray(row.events) || row.events.length > 8 || !Array.isArray(row.progress) || row.progress.length > 16 || !uint(row.nextCursor)) return fail(name);
  const page = {
    schema: "semio.hub.inference-job-events/v1" as const, jobId: row.jobId as string, state: parseInferenceJobStateV1(row.state), proposalState: parseInferenceProposalStateV1(row.proposalState),
    cancelRequested: row.cancelRequested, stale: row.stale, proposalHash: row.proposalHash as string | null,
    events: row.events.map(parseInferenceEventV1), progress: row.progress.map(parseInferenceProgressV1), nextCursor: row.nextCursor as number,
  };
  return Object.hasOwn(row, "preview") ? { ...page, preview: parseGisMapInferencePreviewV1(row.preview) } : page;
}

export type InferenceHybridLogicalTimestampV1 = { readonly actor: number; readonly physicalMs: number; readonly logical: number };
export function parseInferenceHybridLogicalTimestampV1(value: unknown): InferenceHybridLogicalTimestampV1 {
  const name = "hub.inference/InferenceHybridLogicalTimestampV1";
  const row = rows(value, ["actor", "physicalMs", "logical"], name);
  if (!["actor", "physicalMs", "logical"].every((key) => uint(row[key]))) return fail(name);
  return { actor: row.actor as number, physicalMs: row.physicalMs as number, logical: row.logical as number };
}

export type InferenceCommandPayloadV1 = { readonly schema: "gis.map"; readonly payloadHex: string };
export function parseInferenceCommandPayloadV1(value: unknown): InferenceCommandPayloadV1 {
  const name = "hub.inference/InferenceCommandPayloadV1";
  const row = rows(value, ["schema", "payloadHex"], name);
  if (row.schema !== "gis.map" || !hexBytes(row.payloadHex, 4096)) return fail(name);
  return { schema: "gis.map", payloadHex: row.payloadHex as string };
}

/** ✉️ The server-derived fields one approval envelope carries; no client bytes enter it. */
export type InferenceCommandV1 = {
  readonly mutationId: string;
  readonly documentId: string;
  readonly actor: string;
  readonly dependencies: readonly string[];
  readonly diff: InferenceCommandPayloadV1;
  readonly inverse: InferenceCommandPayloadV1;
  readonly timestamp: InferenceHybridLogicalTimestampV1;
};
export function parseInferenceCommandV1(value: unknown): InferenceCommandV1 {
  const name = "hub.inference/InferenceCommandV1";
  const row = rows(value, ["mutationId", "documentId", "actor", "dependencies", "diff", "inverse", "timestamp"], name);
  if (!hex(row.mutationId, 32) || !text(row.documentId, 256) || !text(row.actor, 256) || !Array.isArray(row.dependencies) || row.dependencies.length > 64 || !row.dependencies.every((entry) => hex(entry, 32))) return fail(name);
  return { mutationId: row.mutationId as string, documentId: row.documentId as string, actor: row.actor as string, dependencies: [...(row.dependencies as string[])], diff: parseInferenceCommandPayloadV1(row.diff), inverse: parseInferenceCommandPayloadV1(row.inverse), timestamp: parseInferenceHybridLogicalTimestampV1(row.timestamp) };
}

export type InferenceCommandLimitsV1 = { readonly commandBytes: 8192; readonly textBytes: 256; readonly dependencyCount: 64; readonly payloadBytes: 4096; readonly integerMaximum: 9007199254740991 };
export function parseInferenceCommandLimitsV1(value: unknown): InferenceCommandLimitsV1 {
  const name = "hub.inference/InferenceCommandLimitsV1";
  const row = rows(value, ["commandBytes", "textBytes", "dependencyCount", "payloadBytes", "integerMaximum"], name);
  if (row.commandBytes !== 8192 || row.textBytes !== 256 || row.dependencyCount !== 64 || row.payloadBytes !== 4096 || row.integerMaximum !== 9007199254740991) return fail(name);
  return { commandBytes: 8192, textBytes: 256, dependencyCount: 64, payloadBytes: 4096, integerMaximum: 9007199254740991 };
}

/** ⛓️ The exact chain-hashing identity a verified inference WAL segment must carry. */
export type InferenceWalChainPolicyV1 = {
  readonly hashAlgorithm: "blake3-256";
  readonly requiredFlags: 1;
  readonly recordDigest: "full-frame-including-length-crc-and-back-length";
  readonly commitDigest: "previous-chain-hash-followed-by-ordered-record-digests";
};
export function parseInferenceWalChainPolicyV1(value: unknown): InferenceWalChainPolicyV1 {
  const name = "hub.inference/InferenceWalChainPolicyV1";
  const row = rows(value, ["hashAlgorithm", "requiredFlags", "recordDigest", "commitDigest"], name);
  if (row.hashAlgorithm !== "blake3-256" || row.requiredFlags !== 1 || row.recordDigest !== "full-frame-including-length-crc-and-back-length" || row.commitDigest !== "previous-chain-hash-followed-by-ordered-record-digests") return fail(name);
  return { hashAlgorithm: "blake3-256", requiredFlags: 1, recordDigest: "full-frame-including-length-crc-and-back-length", commitDigest: "previous-chain-hash-followed-by-ordered-record-digests" };
}

/** 🎯️ The fenced scope one committed-WAL approval witness search is bound to. */
export type InferenceWalTargetV1 = { readonly scope: InferenceDocumentScopeV1; readonly documentKey: string; readonly generation: number; readonly jobId: string; readonly proposalHash: string; readonly maximumRecords: number };
export function parseInferenceWalTargetV1(value: unknown): InferenceWalTargetV1 {
  const name = "hub.inference/InferenceWalTargetV1";
  const row = rows(value, ["scope", "documentKey", "generation", "jobId", "proposalHash", "maximumRecords"], name);
  if (!text(row.documentKey, 256) || !uint(row.generation) || (row.generation as number) < 1 || !hex(row.jobId, 32) || !hex(row.proposalHash, 64)
    || !uint(row.maximumRecords) || (row.maximumRecords as number) < 1 || (row.maximumRecords as number) > 65536) return fail(name);
  return { scope: parseInferenceDocumentScopeV1(row.scope), documentKey: row.documentKey as string, generation: row.generation as number, jobId: row.jobId as string, proposalHash: row.proposalHash as string, maximumRecords: row.maximumRecords as number };
}

/** 🚧️ Every hub-owned bound one inference job execution is fenced by. */
export type InferenceLimitsV1 = {
  readonly requestMaxBytes: 1024; readonly inputMaxBytes: 65536; readonly resultMaxBytes: 16384; readonly proposalMaxBytes: 4096; readonly commandMaxBytes: 8192;
  readonly identityJsonMaxBytes: 8192; readonly jobCapacity: 128; readonly operationCapacity: 32; readonly documentGateCapacity: 64; readonly progressMaxCursor: 16;
  readonly eventPageMaxItems: 8; readonly claimLeaseMaxMs: 30000; readonly jobMaxLifetimeMs: 120000; readonly workUnitLimit: 4096; readonly recursionDepth: 32;
  readonly allocationBytes: 1048576; readonly approvalMaxRecords: 8;
};
const INFERENCE_LIMITS: InferenceLimitsV1 = {
  requestMaxBytes: 1024, inputMaxBytes: 65536, resultMaxBytes: 16384, proposalMaxBytes: 4096, commandMaxBytes: 8192, identityJsonMaxBytes: 8192, jobCapacity: 128,
  operationCapacity: 32, documentGateCapacity: 64, progressMaxCursor: 16, eventPageMaxItems: 8, claimLeaseMaxMs: 30000, jobMaxLifetimeMs: 120000, workUnitLimit: 4096,
  recursionDepth: 32, allocationBytes: 1048576, approvalMaxRecords: 8,
};
export function parseInferenceLimitsV1(value: unknown): InferenceLimitsV1 {
  const name = "hub.inference/InferenceLimitsV1";
  const keys = Object.keys(INFERENCE_LIMITS);
  const row = rows(value, keys, name);
  if (keys.some((key) => row[key] !== (INFERENCE_LIMITS as unknown as Record<string, number>)[key])) return fail(name);
  return INFERENCE_LIMITS;
}

export type InferenceCatalogOwnerV1 = { readonly pluginId: "gis"; readonly packageId: "semio:gis"; readonly version: "1.0.0"; readonly packageHash: string };
export function parseInferenceCatalogOwnerV1(value: unknown): InferenceCatalogOwnerV1 {
  const name = "hub.inference/InferenceCatalogOwnerV1";
  const row = rows(value, ["pluginId", "packageId", "version", "packageHash"], name);
  if (row.pluginId !== "gis" || row.packageId !== "semio:gis" || row.version !== "1.0.0" || !hex(row.packageHash, 64)) return fail(name);
  return { pluginId: "gis", packageId: "semio:gis", version: "1.0.0", packageHash: row.packageHash as string };
}

export type InferenceCatalogFrontierV1 = { readonly headSeq: 0; readonly commitSeq: 0; readonly epoch: 0 };
export function parseInferenceCatalogFrontierV1(value: unknown): InferenceCatalogFrontierV1 {
  const name = "hub.inference/InferenceCatalogFrontierV1";
  const row = rows(value, ["headSeq", "commitSeq", "epoch"], name);
  if (row.headSeq !== 0 || row.commitSeq !== 0 || row.epoch !== 0) return fail(name);
  return { headSeq: 0, commitSeq: 0, epoch: 0 };
}

export type InferenceCatalogDescriptorV1 = {
  readonly spaceId: string; readonly documentId: string; readonly artifactKind: "s.gis.gismap"; readonly artifactSchema: "gis.map"; readonly owner: InferenceCatalogOwnerV1;
  readonly packSchemaHash: string; readonly bootstrapVersion: 1; readonly bootstrapFrontier: InferenceCatalogFrontierV1; readonly bootstrapSnapshotHash: string;
};
export function parseInferenceCatalogDescriptorV1(value: unknown): InferenceCatalogDescriptorV1 {
  const name = "hub.inference/InferenceCatalogDescriptorV1";
  const row = rows(value, ["spaceId", "documentId", "artifactKind", "artifactSchema", "owner", "packSchemaHash", "bootstrapVersion", "bootstrapFrontier", "bootstrapSnapshotHash"], name);
  if (!hex(row.spaceId, 32) || !hex(row.documentId, 32) || row.artifactKind !== "s.gis.gismap" || row.artifactSchema !== "gis.map" || !hex(row.packSchemaHash, 64) || row.bootstrapVersion !== 1 || !hex(row.bootstrapSnapshotHash, 64)) return fail(name);
  return { spaceId: row.spaceId as string, documentId: row.documentId as string, artifactKind: "s.gis.gismap", artifactSchema: "gis.map", owner: parseInferenceCatalogOwnerV1(row.owner), packSchemaHash: row.packSchemaHash as string, bootstrapVersion: 1, bootstrapFrontier: parseInferenceCatalogFrontierV1(row.bootstrapFrontier), bootstrapSnapshotHash: row.bootstrapSnapshotHash as string };
}

export type InferenceCatalogPackageV1 = { readonly pluginId: "gis"; readonly packageId: "semio:gis"; readonly version: "1.0.0"; readonly componentSha256: string };
export function parseInferenceCatalogPackageV1(value: unknown): InferenceCatalogPackageV1 {
  const name = "hub.inference/InferenceCatalogPackageV1";
  const row = rows(value, ["pluginId", "packageId", "version", "componentSha256"], name);
  if (row.pluginId !== "gis" || row.packageId !== "semio:gis" || row.version !== "1.0.0" || !hex(row.componentSha256, 64)) return fail(name);
  return { pluginId: "gis", packageId: "semio:gis", version: "1.0.0", componentSha256: row.componentSha256 as string };
}

export type InferenceCatalogServiceV1 = {
  readonly owner: "gis"; readonly contributor: "gis"; readonly artifactKind: "s.gis.gismap"; readonly artifactSchema: "s.gis.gismap"; readonly artifactSchemaVersion: 1;
  readonly documentSchema: "gis.map"; readonly documentSchemaVersion: 1; readonly inferenceSchema: "s.gis.gismap.inference"; readonly inferenceSchemaVersion: 1;
  readonly algorithmVersion: 1; readonly policyVersion: 1; readonly dependsOn: readonly never[];
};
const INFERENCE_CATALOG_SERVICE: InferenceCatalogServiceV1 = {
  owner: "gis", contributor: "gis", artifactKind: "s.gis.gismap", artifactSchema: "s.gis.gismap", artifactSchemaVersion: 1, documentSchema: "gis.map",
  documentSchemaVersion: 1, inferenceSchema: "s.gis.gismap.inference", inferenceSchemaVersion: 1, algorithmVersion: 1, policyVersion: 1, dependsOn: [],
};
export function parseInferenceCatalogServiceV1(value: unknown): InferenceCatalogServiceV1 {
  const name = "hub.inference/InferenceCatalogServiceV1";
  const keys = Object.keys(INFERENCE_CATALOG_SERVICE);
  const row = rows(value, keys, name);
  if (!Array.isArray(row.dependsOn) || row.dependsOn.length !== 0
    || keys.filter((key) => key !== "dependsOn").some((key) => row[key] !== (INFERENCE_CATALOG_SERVICE as unknown as Record<string, unknown>)[key])) return fail(name);
  return INFERENCE_CATALOG_SERVICE;
}

export type InferenceCatalogSelectionV1 = {
  readonly scope: { readonly spaceId: string; readonly documentId: string };
  readonly descriptor: InferenceCatalogDescriptorV1;
  readonly package: InferenceCatalogPackageV1;
  readonly services: readonly InferenceCatalogServiceV1[];
};
export function parseInferenceCatalogSelectionV1(value: unknown): InferenceCatalogSelectionV1 {
  const name = "hub.inference/InferenceCatalogSelectionV1";
  const row = rows(value, ["scope", "descriptor", "package", "services"], name);
  const scope = rows(row.scope, ["spaceId", "documentId"], name);
  if (!hex(scope.spaceId, 32) || !hex(scope.documentId, 32) || !Array.isArray(row.services) || row.services.length !== 1) return fail(name);
  return { scope: { spaceId: scope.spaceId as string, documentId: scope.documentId as string }, descriptor: parseInferenceCatalogDescriptorV1(row.descriptor), package: parseInferenceCatalogPackageV1(row.package), services: row.services.map(parseInferenceCatalogServiceV1) };
}

export type GisMapFrozenExecutionProtocolV1 = { readonly appChannelVersion: 15 };
export function parseGisMapFrozenExecutionProtocolV1(value: unknown): GisMapFrozenExecutionProtocolV1 {
  const row = rows(value, ["appChannelVersion"], "hub.inference/GisMapFrozenExecutionProtocolV1");
  return row.appChannelVersion === 15 ? { appChannelVersion: 15 } : fail("hub.inference/GisMapFrozenExecutionProtocolV1");
}

export type GisMapFrozenPackageV1 = {
  readonly pluginId: "gis"; readonly packageId: "semio:gis"; readonly version: "0.1.0"; readonly componentSha256: string; readonly componentBlake3: string;
  readonly descriptorByteSha256: string; readonly executionProtocol: GisMapFrozenExecutionProtocolV1;
};
export function parseGisMapFrozenPackageV1(value: unknown): GisMapFrozenPackageV1 {
  const name = "hub.inference/GisMapFrozenPackageV1";
  const row = rows(value, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256", "executionProtocol"], name);
  if (row.pluginId !== "gis" || row.packageId !== "semio:gis" || row.version !== "0.1.0" || !["componentSha256", "componentBlake3", "descriptorByteSha256"].every((key) => hex(row[key], 64))) return fail(name);
  return { pluginId: "gis", packageId: "semio:gis", version: "0.1.0", componentSha256: row.componentSha256 as string, componentBlake3: row.componentBlake3 as string, descriptorByteSha256: row.descriptorByteSha256 as string, executionProtocol: parseGisMapFrozenExecutionProtocolV1(row.executionProtocol) };
}

export type GisMapFrozenArtifactV1 = { readonly kind: "s.gis.gismap"; readonly schema: "gis.map"; readonly packSchemaHash: string };
export function parseGisMapFrozenArtifactV1(value: unknown): GisMapFrozenArtifactV1 {
  const name = "hub.inference/GisMapFrozenArtifactV1";
  const row = rows(value, ["kind", "schema", "packSchemaHash"], name);
  if (row.kind !== "s.gis.gismap" || row.schema !== "gis.map" || !hex(row.packSchemaHash, 64)) return fail(name);
  return { kind: "s.gis.gismap", schema: "gis.map", packSchemaHash: row.packSchemaHash as string };
}

export type GisMapFrozenSurfaceV1 = { readonly surfaceId: "s.gis.gismap@1/*#editor"; readonly appId: "s.gis.gismap@1/*#editor"; readonly windowKindId: "gis2d-main"; readonly role: "editor"; readonly rendererTarget: "wasm" };
export function parseGisMapFrozenSurfaceV1(value: unknown): GisMapFrozenSurfaceV1 {
  const name = "hub.inference/GisMapFrozenSurfaceV1";
  const row = rows(value, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"], name);
  if (row.surfaceId !== "s.gis.gismap@1/*#editor" || row.appId !== "s.gis.gismap@1/*#editor" || row.windowKindId !== "gis2d-main" || row.role !== "editor" || row.rendererTarget !== "wasm") return fail(name);
  return { surfaceId: "s.gis.gismap@1/*#editor", appId: "s.gis.gismap@1/*#editor", windowKindId: "gis2d-main", role: "editor", rendererTarget: "wasm" };
}

export type GisMapFrozenGrantV1 = { readonly read: true; readonly write: true; readonly observe: true };
export function parseGisMapFrozenGrantV1(value: unknown): GisMapFrozenGrantV1 {
  const name = "hub.inference/GisMapFrozenGrantV1";
  const row = rows(value, ["read", "write", "observe"], name);
  if (row.read !== true || row.write !== true || row.observe !== true) return fail(name);
  return { read: true, write: true, observe: true };
}

/** 🧊️ The complete frozen GIS Map selection one inference job may ever execute against. */
export type GisMapFrozenBindingV1 = {
  readonly catalogGenerationId: string; readonly package: GisMapFrozenPackageV1; readonly artifact: GisMapFrozenArtifactV1; readonly parentDialect: InferenceParentDialectV1;
  readonly surface: GisMapFrozenSurfaceV1; readonly grant: GisMapFrozenGrantV1; readonly service: InferenceCatalogServiceV1; readonly nativeExecutable: "semio_s_plugin_gis::gis_map_inference_service";
};
export function parseGisMapFrozenBindingV1(value: unknown): GisMapFrozenBindingV1 {
  const name = "hub.inference/GisMapFrozenBindingV1";
  const row = rows(value, ["catalogGenerationId", "package", "artifact", "parentDialect", "surface", "grant", "service", "nativeExecutable"], name);
  if (!hex(row.catalogGenerationId, 64) || row.nativeExecutable !== "semio_s_plugin_gis::gis_map_inference_service") return fail(name);
  return {
    catalogGenerationId: row.catalogGenerationId as string, package: parseGisMapFrozenPackageV1(row.package), artifact: parseGisMapFrozenArtifactV1(row.artifact),
    parentDialect: parseInferenceParentDialectV1(row.parentDialect), surface: parseGisMapFrozenSurfaceV1(row.surface), grant: parseGisMapFrozenGrantV1(row.grant),
    service: parseInferenceCatalogServiceV1(row.service), nativeExecutable: "semio_s_plugin_gis::gis_map_inference_service",
  };
}
