/** 💡️ Independent Bun/AJV oracle for the MCP ↔ hub GIS Map inference bridge (ticket
 * `26/09/02/COMPLETE-SEMIO-END-TO-END`, lane `fable-mcp-inference-bridge`).
 *
 * It observes two things it never imports:
 *
 * 1. the **neutral fixture** `🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1` — the same
 *    language-agnostic corpus the hub's own Rust laws and Bun oracle read. Reused, never forked, so
 *    a hub-side change to the closed error vocabulary, the lifecycle, the visibility law or the
 *    fixed limits fails here loudly instead of drifting;
 * 2. the **hub's registered axum routes**, read as text out of `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`,
 *    so the MCP client's four path builders are checked against the server that serves them.
 *
 * The wire-shape half is a real third-party oracle: AJV 2020-12 compiles schemas declared HERE, not
 * exported from Rust, for the four closed request/response bodies and the two-field error body —
 * the TypeScript twin of `deny_unknown_fields` plus each DTO's own `validate`.
 *
 * Nonclaims: no external model provider, no WGPU or browser rendering, no live hub, no two-user
 * process journey. This module proves shapes, vocabulary and paths — never that a job ran. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";

//#region 🔖️Surface
/** 🎯️ The four tool names `🦀️.rs`'s `GATEWAY_TOOL_NAMES` gains. Duplicated on purpose: this file is
 * an independent observer of the running binary and must not import the value it checks. */
export const INFERENCE_JOB_TOOLS = ["inference_submit", "inference_events", "inference_cancel", "inference_approve"] as const;

/** 🔤️ Percent-encoding for one path segment, matching the Rust client's own reserved-character set
 * (`A-Za-z0-9-._~` unreserved, everything else `%XX` upper-hex). */
export const encodeSegment = (value: string): string =>
  [...Buffer.from(value, "utf8")].map((byte) => (/[A-Za-z0-9\-._~]/.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, "0")}`)).join("");

export const jobsPath = (space: string, document: string): string => `/spaces/${encodeSegment(space)}/documents/${encodeSegment(document)}/inference/gis-map/jobs`;
export const eventsPath = (space: string, document: string, job: string, after: number): string => `${jobsPath(space, document)}/${encodeSegment(job)}/events?after=${after}`;
export const cancelPath = (space: string, document: string, job: string): string => `${jobsPath(space, document)}/${encodeSegment(job)}/cancel`;
export const approvalPath = (space: string, document: string, job: string): string => `${jobsPath(space, document)}/${encodeSegment(job)}/approval`;

const JOB_STATES = ["accepted", "running", "succeeded", "failed", "cancelled"];
const PROPOSAL_STATES = ["none", "offered", "approved", "stale", "cancelled"];
const HEX32 = "^[0-9a-f]{32}$";
const HEX64 = "^[0-9a-f]{64}$";

export const submitRequestSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  properties: {
    schema: { const: "semio.hub.inference-request/v1" },
    version: { const: 1 },
    requestId: { type: "string", pattern: HEX32 },
    serviceId: { const: "s.gis.gismap.inference" },
    policyVersion: { const: 1 },
    lifetimeMs: { type: "integer", minimum: 1, maximum: 120000 },
  },
  required: ["schema", "version", "requestId", "serviceId", "policyVersion", "lifetimeMs"],
  additionalProperties: false,
};

export const approvalRequestSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  properties: { schema: { const: "semio.hub.inference-approval/v1" }, version: { const: 1 }, jobId: { type: "string", pattern: HEX32 }, proposalHash: { type: "string", pattern: HEX64 } },
  required: ["schema", "version", "jobId", "proposalHash"],
  additionalProperties: false,
};

export const jobReceiptSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  properties: {
    schema: { const: "semio.hub.inference-job-receipt/v1" },
    jobId: { type: "string", pattern: HEX32 },
    state: { enum: JOB_STATES },
    proposalState: { enum: PROPOSAL_STATES },
    proposalHash: { anyOf: [{ type: "string", pattern: HEX64 }, { type: "null" }] },
    cursor: { type: "integer", minimum: 0, maximum: 16 },
    expiresAtMs: { type: "integer", minimum: 0 },
  },
  required: ["schema", "jobId", "state", "proposalState", "proposalHash", "cursor", "expiresAtMs"],
  additionalProperties: false,
};

export const previewSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  properties: {
    schema: { const: "semio.hub.gis-map-inference-preview/v1" },
    jobId: { type: "string", pattern: HEX32 },
    proposalHash: { type: "string", pattern: HEX64 },
    regionId: { type: "string", pattern: "^inference-[0-9a-f]{32}$" },
    ring: { type: "array", minItems: 5, maxItems: 5, items: { type: "array", minItems: 2, maxItems: 2, items: { type: "number" } } },
  },
  required: ["schema", "jobId", "proposalHash", "regionId", "ring"],
  additionalProperties: false,
};

export const eventPageSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  properties: {
    schema: { const: "semio.hub.inference-job-events/v1" },
    jobId: { type: "string", pattern: HEX32 },
    state: { enum: JOB_STATES },
    proposalState: { enum: PROPOSAL_STATES },
    cancelRequested: { type: "boolean" },
    stale: { type: "boolean" },
    proposalHash: { anyOf: [{ type: "string", pattern: HEX64 }, { type: "null" }] },
    preview: {
      type: "object",
      properties: {
        schema: { const: "semio.hub.gis-map-inference-preview/v1" },
        jobId: { type: "string", pattern: HEX32 },
        proposalHash: { type: "string", pattern: HEX64 },
        regionId: { type: "string", pattern: "^inference-[0-9a-f]{32}$" },
        ring: { type: "array", minItems: 5, maxItems: 5, items: { type: "array", minItems: 2, maxItems: 2, items: { type: "number" } } },
      },
      required: ["schema", "jobId", "proposalHash", "regionId", "ring"],
      additionalProperties: false,
    },
    events: {
      type: "array",
      maxItems: 8,
      items: { type: "object", properties: { ordinal: { type: "integer", minimum: 1 }, kind: { type: "string" }, atMs: { type: "integer", minimum: 0 } }, required: ["ordinal", "kind", "atMs"], additionalProperties: false },
    },
    progress: {
      type: "array",
      maxItems: 8,
      items: {
        type: "object",
        properties: { cursor: { type: "integer", minimum: 1, maximum: 16 }, runEpoch: { type: "integer", minimum: 1 }, completed: { type: "integer", minimum: 0 }, total: { type: "integer", minimum: 0 }, atMs: { type: "integer", minimum: 0 } },
        required: ["cursor", "runEpoch", "completed", "total", "atMs"],
        additionalProperties: false,
      },
    },
    nextCursor: { type: "integer", minimum: 0, maximum: 16 },
  },
  required: ["schema", "jobId", "state", "proposalState", "cancelRequested", "stale", "proposalHash", "events", "progress", "nextCursor"],
  additionalProperties: false,
};

export const approvalReceiptSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  properties: {
    schema: { const: "semio.hub.inference-approval-receipt/v1" },
    jobId: { type: "string", pattern: HEX32 },
    mutationId: { type: "string", pattern: HEX32 },
    commandHash: { type: "string", pattern: HEX64 },
    proposalHash: { type: "string", pattern: HEX64 },
    applied: { type: "boolean" },
  },
  required: ["schema", "jobId", "mutationId", "commandHash", "proposalHash", "applied"],
  additionalProperties: false,
};
//#endregion 🔖️Surface

//#region 🔖️Oracle
export type InferenceBridgeReport = {
  readonly ajv: number;
  readonly hostile: number;
  readonly errors: number;
  readonly visibility: number;
  readonly lifecycle: number;
  readonly routes: number;
  readonly limits: number;
};

const compile = (schema: object) => new Ajv2020({ strict: true }).compile(schema);

function must(condition: unknown, message: string): void {
  if (!condition) throw new Error(`inference-bridge-oracle: ${message}`);
}

/** 💡️ Runs every source-level law and returns the counts a gate prints. It throws on the first
 * violation, so a zero-exit run is itself the receipt. */
export function proveMcpInferenceBridgeFixture(repoRoot: string): InferenceBridgeReport {
  const fixtureRoot = resolve(repoRoot, "🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1");
  const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️.json"), "utf8"));
  const fixtureSchema = JSON.parse(readFileSync(resolve(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const hubBinSource = readFileSync(resolve(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");

  let ajv = 0;
  const fixtureValidator = new Ajv2020({ strict: true });
  fixtureValidator.addSchema(JSON.parse(readFileSync(resolve(repoRoot, "🌎️hub/💡️inference/🧬️schema/🔣️.json"), "utf8")));
  const validateFixture = fixtureValidator.compile(fixtureSchema);
  must(validateFixture(fixture), `the shared fixture failed its shared schema: ${JSON.stringify(validateFixture.errors)}`);
  ajv += 1;

  const errorRows = fixture.errors as Array<{ name: string; code: string; status: number }>;
  must(new Set(errorRows.map((row) => row.code)).size === errorRows.length, "the closed error vocabulary has a duplicate code");
  const byStatus = new Map<number, string[]>();
  for (const row of errorRows) byStatus.set(row.status, [...(byStatus.get(row.status) ?? []), row.code].sort());
  must(JSON.stringify(byStatus.get(503)) === JSON.stringify(["approval.commit-unavailable", "inference.storage", "inference.unavailable"]), "503 is not the three-code ambiguous status the client must decode past");
  must(JSON.stringify(byStatus.get(409)) === JSON.stringify(["inference.cancelled", "inference.conflict"]), "409 is not the two-code ambiguous status");
  must(errorRows.some((row) => row.name === "no-binding" && row.code === "inference.unavailable" && row.status === 503), "the retryable missing-binding member is not pinned");

  const visibility = fixture.visibility as Array<{ role: string; readEvents: boolean; readProposal: boolean; approve: boolean; expectedCode: string | null }>;
  const owners = visibility.filter((row) => row.readEvents || row.readProposal || row.approve);
  must(owners.length === 1 && owners[0].role === "author-owner", "exactly one role may observe a job");
  for (const row of visibility.filter((candidate) => candidate.role !== "author-owner")) must(row.expectedCode === "inference.denied", `${row.role} is not denied`);

  let lifecycle = 0;
  for (const [name, trace] of [["lifecycle", fixture.lifecycle], ["cancelLifecycle", fixture.cancelLifecycle]] as Array<[string, Array<{ ordinal: number; kind: string }>]>) {
    must(trace.every((row, index) => row.ordinal === index + 1), `${name} is not strictly ordered from one`);
    must(trace.length <= fixture.limits.eventPageMaxItems, `${name} exceeds one bounded page`);
    lifecycle += trace.length;
  }
  must(JSON.stringify((fixture.lifecycle as Array<{ kind: string }>).map((row) => row.kind)) === JSON.stringify(["accepted", "running", "succeeded", "approval-prepared", "approved"]), "the success lifecycle drifted");
  must(JSON.stringify((fixture.cancelLifecycle as Array<{ kind: string }>).map((row) => row.kind)) === JSON.stringify(["accepted", "running", "cancel-requested", "cancelled"]), "the cancel lifecycle drifted");

  const limits = { requestMaxBytes: 1024, jobMaxLifetimeMs: 120000, progressMaxCursor: 16, eventPageMaxItems: 8 };
  for (const [key, value] of Object.entries(limits)) must(fixture.limits[key] === value, `the client mirrors ${key}=${value} but the corpus says ${fixture.limits[key]}`);
  must(JSON.stringify([...(fixture.nonclaims as string[])].sort()) === JSON.stringify(["no-auto-apply", "no-client-supplied-map-pack", "no-external-model-provider", "no-wgpu-rendering"]), "the four nonclaims drifted");

  const jobId: string = fixture.sampleJobId;
  const proposalHash: string = fixture.proposalHash;
  let hostile = 0;
  const reject = (validate: ReturnType<typeof compile>, candidate: unknown, label: string) => {
    must(!validate(candidate), `${label} was admitted`);
    hostile += 1;
  };

  const submit = compile(submitRequestSchema);
  ajv += 1;
  const submitBody = { schema: "semio.hub.inference-request/v1", version: 1, requestId: jobId, serviceId: "s.gis.gismap.inference", policyVersion: 1, lifetimeMs: fixture.limits.jobMaxLifetimeMs };
  must(submit(submitBody), `a well-formed intent was rejected: ${JSON.stringify(submit.errors)}`);
  must(JSON.stringify(submitBody).length <= fixture.limits.requestMaxBytes, "the canonical intent does not fit the fixed bound");
  reject(submit, { ...submitBody, schema: "semio.hub.inference-request/v2" }, "wrong-schema");
  reject(submit, { ...submitBody, version: 2 }, "wrong-version");
  reject(submit, { ...submitBody, requestId: "A".repeat(32) }, "upper-hex-request-id");
  reject(submit, { ...submitBody, requestId: `${jobId}ff` }, "long-request-id");
  reject(submit, { ...submitBody, serviceId: "s.gis.gismap.other" }, "foreign-service");
  reject(submit, { ...submitBody, policyVersion: 2 }, "wrong-policy");
  reject(submit, { ...submitBody, lifetimeMs: 0 }, "zero-lifetime");
  reject(submit, { ...submitBody, lifetimeMs: fixture.limits.jobMaxLifetimeMs + 1 }, "over-lifetime");
  reject(submit, { ...submitBody, mapPack: "client-supplied" }, "smuggled-map-pack");

  const approvalRequest = compile(approvalRequestSchema);
  ajv += 1;
  const approvalBody = { schema: "semio.hub.inference-approval/v1", version: 1, jobId, proposalHash };
  must(approvalRequest(approvalBody), `a well-formed approval was rejected: ${JSON.stringify(approvalRequest.errors)}`);
  reject(approvalRequest, { ...approvalBody, proposalHash: proposalHash.slice(0, 63) }, "short-proposal-hash");
  reject(approvalRequest, { ...approvalBody, jobId: "not-hex" }, "non-hex-job");
  reject(approvalRequest, { ...approvalBody, actor: "user:forged" }, "client-supplied-actor");
  reject(approvalRequest, { ...approvalBody, command: "client-stamped" }, "client-stamped-command");

  const receipt = compile(jobReceiptSchema);
  ajv += 1;
  const receiptBody = { schema: "semio.hub.inference-job-receipt/v1", jobId, state: "succeeded", proposalState: "offered", proposalHash, cursor: 4, expiresAtMs: 1000 };
  must(receipt(receiptBody), `a well-formed receipt was rejected: ${JSON.stringify(receipt.errors)}`);
  must(receipt({ ...receiptBody, proposalHash: null, proposalState: "none", cursor: 0 }), "an unoffered receipt must decode with a null proposal hash");
  reject(receipt, { ...receiptBody, proposal: "private bytes" }, "leaked-proposal-bytes");
  reject(receipt, { ...receiptBody, state: "offered" }, "state-and-proposal-state-confused");

  const preview = compile(previewSchema);
  ajv += 1;
  const fixturePreview = fixture.preview as { schema: string; jobId: string; proposalHash: string; regionId: string; ring: number[][] };
  must(preview(fixturePreview), `the corpus preview was rejected: ${JSON.stringify(preview.errors)}`);
  must(fixturePreview.jobId === jobId && fixturePreview.proposalHash === proposalHash, "the corpus preview is not bound to the sample job and its proposal digest");
  must(fixturePreview.regionId === `inference-${jobId}`, "the preview region id is not derived from the job alone");
  const [firstX, firstY] = fixturePreview.ring[0];
  const [lastX, lastY] = fixturePreview.ring[4];
  must(firstX === lastX && firstY === lastY, "the preview ring is not closed");
  const lonMin = Math.min(...fixturePreview.ring.map((point) => point[0]));
  const lonMax = Math.max(...fixturePreview.ring.map((point) => point[0]));
  const latMin = Math.min(...fixturePreview.ring.map((point) => point[1]));
  const latMax = Math.max(...fixturePreview.ring.map((point) => point[1]));
  must(
    JSON.stringify(fixturePreview.ring) === JSON.stringify([[lonMin, latMin], [lonMax, latMin], [lonMax, latMax], [lonMin, latMax], [lonMin, latMin]]),
    "the preview ring is not the axis-aligned corner order the hub publishes",
  );
  const base = fixture.base.expectedInference.bounds as { lonMin: number; lonMax: number; latMin: number; latMax: number };
  must(lonMin === base.lonMin && lonMax === base.lonMax && latMin === base.latMin && latMax === base.latMax, "the preview ring does not fold to the corpus's own independently computed bounds");
  reject(preview, { ...fixturePreview, ring: fixturePreview.ring.slice(0, 4) }, "open-preview-ring");
  reject(preview, { ...fixturePreview, regionId: "inference-forged" }, "forged-preview-region-id");
  reject(preview, { ...fixturePreview, mutation: "client-applied" }, "preview-carrying-a-mutation");

  const page = compile(eventPageSchema);
  ajv += 1;
  const events = (fixture.lifecycle as Array<{ ordinal: number; kind: string }>).map((row) => ({ ...row, atMs: 1000 + row.ordinal }));
  const progress = [1, 2, 3, 4].map((cursor) => ({ cursor, runEpoch: 1, completed: cursor, total: fixture.limits.workUnitLimit, atMs: 1000 + cursor }));
  const pageBody = { schema: "semio.hub.inference-job-events/v1", jobId, state: "succeeded", proposalState: "offered", cancelRequested: false, stale: false, proposalHash, events, progress, nextCursor: 4 };
  must(page(pageBody), `a well-formed page was rejected: ${JSON.stringify(page.errors)}`);
  must(page({ ...pageBody, preview: fixturePreview }), `an offered page carrying the corpus preview was rejected: ${JSON.stringify(page.errors)}`);
  reject(page, { ...pageBody, preview: { ...fixturePreview, ring: [] } }, "page-preview-with-an-empty-ring");
  reject(page, { ...pageBody, progress: [...progress, { cursor: 17, runEpoch: 1, completed: 17, total: 4096, atMs: 1 }] }, "cursor-past-the-fixed-maximum");
  reject(page, { ...pageBody, nextCursor: fixture.limits.progressMaxCursor + 1 }, "next-cursor-past-the-fixed-maximum");
  reject(page, { ...pageBody, proposal: "private bytes" }, "leaked-page-proposal");
  reject(page, { ...pageBody, events: [...events, ...events] }, "page-over-the-fixed-item-bound");

  const approvalReceipt = compile(approvalReceiptSchema);
  ajv += 1;
  const approvalReceiptBody = { schema: "semio.hub.inference-approval-receipt/v1", jobId, mutationId: jobId, commandHash: proposalHash, proposalHash, applied: false };
  must(approvalReceipt(approvalReceiptBody), `a well-formed approval receipt was rejected: ${JSON.stringify(approvalReceipt.errors)}`);
  reject(approvalReceipt, { ...approvalReceiptBody, command: "server bytes" }, "leaked-command-bytes");

  const errorBody = compile({
    $schema: "https://json-schema.org/draft/2020-12/schema",
    type: "object",
    properties: { schema: { const: "semio.hub.inference-error/v1" }, code: { enum: errorRows.map((row) => row.code) } },
    required: ["schema", "code"],
    additionalProperties: false,
  });
  ajv += 1;
  for (const row of errorRows) must(errorBody({ schema: "semio.hub.inference-error/v1", code: row.code }), `${row.code} is not accepted by the closed error body`);
  reject(errorBody, { schema: "semio.hub.inference-error/v1", code: "inference.not-a-code" }, "unknown-error-code");
  reject(errorBody, { schema: "semio.hub.inference-error/v1", code: "inference.denied", detail: "which private object" }, "error-body-detail-leak");

  const registered = [...hubBinSource.matchAll(/\.route\("(\/spaces\/\{space_id\}\/documents\/\{document_id\}\/inference\/[^"]+)"/g)].map((match) => match[1]).sort();
  const expected = [
    "/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs",
    "/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/approval",
    "/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/cancel",
    "/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/events",
  ].sort();
  must(JSON.stringify(registered) === JSON.stringify(expected), `the hub binary registers ${JSON.stringify(registered)}, not the four routes this client calls`);
  const render = (template: string) => template.replace("{space_id}", encodeSegment("space:alpha")).replace("{document_id}", encodeSegment("doc:tokyo")).replace("{job_id}", encodeSegment(jobId));
  must(jobsPath("space:alpha", "doc:tokyo") === render(expected.find((route) => route.endsWith("/jobs"))!), "the submit path builder drifted from the registered route");
  must(cancelPath("space:alpha", "doc:tokyo", jobId) === render(expected.find((route) => route.endsWith("/cancel"))!), "the cancel path builder drifted");
  must(approvalPath("space:alpha", "doc:tokyo", jobId) === render(expected.find((route) => route.endsWith("/approval"))!), "the approval path builder drifted");
  must(eventsPath("space:alpha", "doc:tokyo", jobId, 4) === `${render(expected.find((route) => route.endsWith("/events"))!)}?after=4`, "the events path builder drifted");

  return { ajv, hostile, errors: errorRows.length, visibility: visibility.length, lifecycle, routes: expected.length, limits: Object.keys(limits).length };
}
//#endregion 🔖️Oracle
