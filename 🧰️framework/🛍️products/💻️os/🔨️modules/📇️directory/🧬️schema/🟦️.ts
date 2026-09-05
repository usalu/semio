/** 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
 * STUDIOS, contract C1) — TypeScript twin of `🦀️.rs`. Pure data, no fold logic (see the
 * module root `../🟦️.ts`'s `DirectoryReadModel`/`fold`).
 *
 * 🧭️ `space.created`'s and `create-space`'s space-kind fields are named `spaceKind`, not
 * contract-freeze.md's bare `kind` — both bodies are tagged on a `kind` discriminator, so a
 * same-named payload field would collide on the wire. Flagged as a `sharedFileRequest` in lane
 * 0-A's report. */

//#region 🔖️Vocabulary
export type DirectorySpaceKind = "atelier" | "studio" | "archive";
export type DirectorySpaceVisibility = "private" | "public";
export type DirectorySpaceRole = "author" | "spectator";
//#endregion 🔖️Vocabulary

//#region 🔖️Actor
export type DirectoryActorKind = "user" | "admin" | "system";

export interface DirectoryActor {
  kind: DirectoryActorKind;
  id: string;
}

/** 🕰️ Hybrid logical clock stamp. */
export interface Hlc {
  physicalMs: number;
  logical: number;
}
//#endregion 🔖️Actor

//#region 🔖️Event
export interface DirectoryEventUserCreated {
  kind: "user.created";
  userId: string;
  email: string;
  displayName: string;
}

export interface DirectoryEventSpaceCreated {
  kind: "space.created";
  spaceId: string;
  name: string;
  spaceKind: DirectorySpaceKind;
  visibility: DirectorySpaceVisibility;
  ownerUserId: string;
}

export interface DirectoryEventSpaceRenamed {
  kind: "space.renamed";
  spaceId: string;
  name: string;
}

export interface DirectoryEventSpaceVisibilityChanged {
  kind: "space.visibility-changed";
  spaceId: string;
  visibility: DirectorySpaceVisibility;
}

export interface DirectoryEventSpaceArchived {
  kind: "space.archived";
  spaceId: string;
}

export interface DirectoryEventSpaceDeleted {
  kind: "space.deleted";
  spaceId: string;
}

export interface DirectoryEventMemberUpserted {
  kind: "member.upserted";
  spaceId: string;
  userId: string;
  role: DirectorySpaceRole;
}

export interface DirectoryEventMemberRemoved {
  kind: "member.removed";
  spaceId: string;
  userId: string;
}

export interface DirectoryEventInviteRedeemed {
  kind: "invite.redeemed";
  spaceId: string;
  userId: string;
  inviteId: string;
  role: DirectorySpaceRole;
}

export interface DirectoryEventDocumentAnnounced {
  kind: "document.announced";
  descriptor: DocumentDescriptor;
}

export interface DirectoryEventArtifactCheckpointPublished {
  kind: "artifact.checkpoint-published";
  checkpoint: PublishedArtifactCheckpoint;
}

export interface DirectoryEventArtifactRetentionAdvanced {
  kind: "artifact.retention-advanced";
  retention: ArtifactRetention;
}

export type DirectoryEventBody =
  | DirectoryEventUserCreated
  | DirectoryEventSpaceCreated
  | DirectoryEventSpaceRenamed
  | DirectoryEventSpaceVisibilityChanged
  | DirectoryEventSpaceArchived
  | DirectoryEventSpaceDeleted
  | DirectoryEventMemberUpserted
  | DirectoryEventMemberRemoved
  | DirectoryEventInviteRedeemed
  | DirectoryEventDocumentAnnounced
  | DirectoryEventArtifactCheckpointPublished
  | DirectoryEventArtifactRetentionAdvanced;

/** 📜️ One persisted, backend-assigned directory event — `seq` is dense and 1-based. */
export interface DirectoryEvent {
  seq: number;
  id: string;
  hlc: Hlc;
  actor: DirectoryActor;
  spaceId?: string;
  userId?: string;
  body: DirectoryEventBody;
  recordedAtMs: number;
}

export const DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS = 128;
export const DIRECTORY_EVENT_PAGE_MAX_BYTES = 64 * 1024;
export const DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES = 48 * 1024;

/** 📄️ One authenticated, receipt-bound bounded scan of the durable directory log. */
export interface DirectoryEventPageV1 {
  schema: "semio.directory.event-page.v1";
  sessionBindingSha256: string;
  authorizationGeneration: number;
  afterSeqExclusive: number;
  throughSeqInclusive: number;
  hasMore: boolean;
  events: DirectoryEvent[];
  receiptSha256: string;
}

function directoryEventPageObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("directory-event-page.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  if (required.some((key) => !(key in object)) || Object.keys(object).some((key) => !accepted.has(key))) throw new Error("directory-event-page.invalid-fields");
  return object;
}

function directoryEventPageHasControl(value: unknown): boolean {
  if (typeof value === "string") return /\p{Cc}/u.test(value);
  if (Array.isArray(value)) return value.some(directoryEventPageHasControl);
  return value !== null && typeof value === "object" && Object.entries(value).some(([key, child]) => /\p{Cc}/u.test(key) || directoryEventPageHasControl(child));
}

function directoryEventPageInteger(value: unknown, positive = false): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("directory-event-page.invalid-integer");
  return value;
}

function directoryEventPageHash(value: unknown, nonzero: boolean): string {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || (nonzero && /^0{64}$/u.test(value))) throw new Error("directory-event-page.invalid-hash");
  return value;
}

function directoryEventPageNestedShapes(body: Record<string, unknown>): void {
  const exact = (value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> => directoryEventPageObject(value, required, optional);
  const texts = (object: Record<string, unknown>, fields: readonly string[]): void => {
    if (fields.some((field) => typeof object[field] !== "string")) throw new Error("directory-event-page.invalid-text");
  };
  const hash = (value: unknown): void => {
    if (!Array.isArray(value) || value.length !== 32 || value.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error("directory-event-page.invalid-byte-hash");
  };
  const scope = (value: unknown): void => {
    const object = exact(value, ["spaceId", "documentId"]);
    texts(object, ["spaceId", "documentId"]);
  };
  const frontier = (value: unknown): void => {
    const object = exact(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
    texts(object, ["documentId", "headEditId"]);
    directoryEventPageInteger(object.headEditOrdinal);
    directoryEventPageInteger(object.lastCommitSeq);
    hash(object.chainHash);
  };
  const descriptor = (value: unknown): void => {
    const object = exact(value, ["spaceId", "documentId", "artifactKind", "artifactSchema", "owner", "packSchemaHash", "bootstrapVersion", "bootstrapFrontier", "bootstrapSnapshotHash"]);
    texts(object, ["spaceId", "documentId", "artifactKind", "artifactSchema", "packSchemaHash", "bootstrapSnapshotHash"]);
    const owner = exact(object.owner, ["pluginId", "packageId", "version", "packageHash"]);
    texts(owner, ["pluginId", "packageId", "version", "packageHash"]);
    const bootstrap = exact(object.bootstrapFrontier, ["headSeq", "commitSeq", "epoch"]);
    directoryEventPageInteger(object.bootstrapVersion, true);
    for (const field of ["headSeq", "commitSeq", "epoch"]) directoryEventPageInteger(bootstrap[field]);
  };
  if (body.kind === "document.announced") descriptor(body.descriptor);
  if (body.kind === "artifact.checkpoint-published") {
    const checkpoint = exact(body.checkpoint, ["scope", "checkpointId", "descriptorDigestV1", "baselineFrontier", "pack", "spr", "aggregateSha256", "publishedAtMs"], ["parentCheckpointId"]);
    scope(checkpoint.scope);
    hash(checkpoint.checkpointId);
    if (checkpoint.parentCheckpointId !== undefined) hash(checkpoint.parentCheckpointId);
    hash(checkpoint.descriptorDigestV1);
    frontier(checkpoint.baselineFrontier);
    for (const field of ["pack", "spr"] as const) {
      const blob = exact(checkpoint[field], ["sha256", "byteLength"]);
      hash(blob.sha256);
      directoryEventPageInteger(blob.byteLength, true);
    }
    hash(checkpoint.aggregateSha256);
    directoryEventPageInteger(checkpoint.publishedAtMs);
  }
  if (body.kind === "artifact.retention-advanced") {
    const retention = exact(body.retention, ["scope", "retainedCheckpointId", "retainedFloor", "checkpointLineageHead"]);
    scope(retention.scope);
    hash(retention.retainedCheckpointId);
    frontier(retention.retainedFloor);
    hash(retention.checkpointLineageHead);
  }
}

function directoryEventPageEvent(value: unknown): DirectoryEvent {
  const event = directoryEventPageObject(value, ["seq", "id", "hlc", "actor", "body", "recordedAtMs"], ["spaceId", "userId"]);
  directoryEventPageInteger(event.seq, true);
  if (typeof event.id !== "string" || typeof event.recordedAtMs !== "number" || !Number.isSafeInteger(event.recordedAtMs) || (event.spaceId !== undefined && typeof event.spaceId !== "string") || (event.userId !== undefined && typeof event.userId !== "string")) throw new Error("directory-event-page.invalid-event");
  directoryEventPageInteger((directoryEventPageObject(event.hlc, ["physicalMs", "logical"])).logical);
  const physicalMs = (event.hlc as Record<string, unknown>).physicalMs;
  if (typeof physicalMs !== "number" || !Number.isSafeInteger(physicalMs)) throw new Error("directory-event-page.invalid-time");
  const actor = directoryEventPageObject(event.actor, ["kind", "id"]);
  if ((actor.kind !== "user" && actor.kind !== "admin" && actor.kind !== "system") || typeof actor.id !== "string") throw new Error("directory-event-page.invalid-actor");
  if (event.body === null || typeof event.body !== "object" || Array.isArray(event.body)) throw new Error("directory-event-page.invalid-event-body");
  const body = event.body as Record<string, unknown>;
  const bodyFields: Record<string, readonly string[]> = {
    "user.created": ["kind", "userId", "email", "displayName"],
    "space.created": ["kind", "spaceId", "name", "spaceKind", "visibility", "ownerUserId"],
    "space.renamed": ["kind", "spaceId", "name"],
    "space.visibility-changed": ["kind", "spaceId", "visibility"],
    "space.archived": ["kind", "spaceId"],
    "space.deleted": ["kind", "spaceId"],
    "member.upserted": ["kind", "spaceId", "userId", "role"],
    "member.removed": ["kind", "spaceId", "userId"],
    "invite.redeemed": ["kind", "spaceId", "userId", "inviteId", "role"],
    "document.announced": ["kind", "descriptor"],
    "artifact.checkpoint-published": ["kind", "checkpoint"],
    "artifact.retention-advanced": ["kind", "retention"],
  };
  const fields = typeof body.kind === "string" ? bodyFields[body.kind] : undefined;
  if (!fields) throw new Error("directory-event-page.invalid-event-kind");
  directoryEventPageObject(body, fields);
  const textFields: Record<string, readonly string[]> = {
    "user.created": ["userId", "email", "displayName"],
    "space.created": ["spaceId", "name", "ownerUserId"],
    "space.renamed": ["spaceId", "name"],
    "space.visibility-changed": ["spaceId"],
    "space.archived": ["spaceId"],
    "space.deleted": ["spaceId"],
    "member.upserted": ["spaceId", "userId"],
    "member.removed": ["spaceId", "userId"],
    "invite.redeemed": ["spaceId", "userId", "inviteId"],
  };
  if ((textFields[body.kind as string] ?? []).some((field) => typeof body[field] !== "string")) throw new Error("directory-event-page.invalid-event-text");
  if ((body.kind === "space.created" && body.spaceKind !== "atelier" && body.spaceKind !== "studio" && body.spaceKind !== "archive")
    || ((body.kind === "space.created" || body.kind === "space.visibility-changed") && body.visibility !== "private" && body.visibility !== "public")
    || ((body.kind === "member.upserted" || body.kind === "invite.redeemed") && body.role !== "author" && body.role !== "spectator")) throw new Error("directory-event-page.invalid-event-vocabulary");
  directoryEventPageNestedShapes(body);
  if (directoryEventPageHasControl(event)) throw new Error("directory-event-page.control-character");
  if (new TextEncoder().encode(JSON.stringify(event)).length > DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES) throw new Error("directory-event-page.event-too-large");
  return event as unknown as DirectoryEvent;
}

async function directoryEventPageSha256(text: string): Promise<string> {
  const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 📥️ Parses one canonical page and verifies exact fields, ranges, size, and SHA-256 receipt. */
export async function parseDirectoryEventPageV1(source: string): Promise<DirectoryEventPageV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_EVENT_PAGE_MAX_BYTES) throw new Error("directory-event-page.too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "sessionBindingSha256", "authorizationGeneration", "afterSeqExclusive", "throughSeqInclusive", "hasMore", "events", "receiptSha256"]);
  if (object.schema !== "semio.directory.event-page.v1" || typeof object.hasMore !== "boolean" || !Array.isArray(object.events) || object.events.length > DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS) throw new Error("directory-event-page.invalid-envelope");
  const afterSeqExclusive = directoryEventPageInteger(object.afterSeqExclusive);
  const throughSeqInclusive = directoryEventPageInteger(object.throughSeqInclusive);
  if (afterSeqExclusive > throughSeqInclusive) throw new Error("directory-event-page.invalid-range");
  const events = object.events.map(directoryEventPageEvent);
  let previous = afterSeqExclusive;
  for (const event of events) {
    if (event.seq <= previous || event.seq > throughSeqInclusive) throw new Error("directory-event-page.invalid-event-range");
    previous = event.seq;
  }
  const page: DirectoryEventPageV1 = {
    schema: object.schema,
    sessionBindingSha256: directoryEventPageHash(object.sessionBindingSha256, true),
    authorizationGeneration: directoryEventPageInteger(object.authorizationGeneration, true),
    afterSeqExclusive,
    throughSeqInclusive,
    hasMore: object.hasMore,
    events,
    receiptSha256: directoryEventPageHash(object.receiptSha256, false),
  };
  if (JSON.stringify(page) !== source) throw new Error("directory-event-page.noncanonical");
  const { receiptSha256, ...unsigned } = page;
  if (await directoryEventPageSha256(JSON.stringify(unsigned)) !== receiptSha256) throw new Error("directory-event-page.receipt-mismatch");
  return page;
}
//#endregion 🔖️Event

//#region 🔖️Command
export type DirectoryCommand =
  | { kind: "create-space"; name: string; spaceKind: DirectorySpaceKind; visibility: DirectorySpaceVisibility }
  | { kind: "rename-space"; spaceId: string; name: string }
  | { kind: "set-visibility"; spaceId: string; visibility: DirectorySpaceVisibility }
  | { kind: "archive-space"; spaceId: string }
  | { kind: "delete-space"; spaceId: string }
  | { kind: "upsert-member"; spaceId: string; email: string; role: DirectorySpaceRole }
  | { kind: "remove-member"; spaceId: string; userId: string }
  | { kind: "create-invite"; spaceId: string; role: DirectorySpaceRole; ttlSecs: number }
  | { kind: "revoke-invite"; spaceId: string; inviteId: string }
  | { kind: "announce-document"; descriptor: DocumentDescriptor };
//#endregion 🔖️Command

//#region 🔖️CommandReceipt
export const DIRECTORY_COMMAND_REQUEST_MAX_BYTES = 8 * 1024;
export const DIRECTORY_COMMAND_RECEIPT_MAX_BYTES = 64 * 1024;
export const DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS = 4;
export const DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES = 256;
export const DIRECTORY_COMMAND_REQUEST_ID_LEN = 32;

/** 🆔️ One sealed, idempotency-correlated directory command. `requestId` is a correlation, never a
 * capability — the hub re-runs authentication and authorization before returning any completion. */
export interface DirectoryCommandRequestV1 {
  schema: "semio.directory.command-request.v1";
  requestId: string;
  command: DirectoryCommand;
}

/** 🧾️ Closed disposition of one durable command request. */
export type DirectoryCommandOutcomeV1 = "accepted" | "previously-accepted" | "secret-undeliverable";

/** 🎁️ Closed command-result grammar; the invite capability never leaves the live operation. */
export type DirectoryCommandResultV1 = { kind: "none" } | { kind: "invite"; inviteToken: string };

/** 🧾️ One authoritative, receipt-bound completion of exactly one command request. */
export interface DirectoryCommandReceiptV1 {
  schema: "semio.directory.command-receipt.v1";
  requestId: string;
  commandSha256: string;
  outcome: DirectoryCommandOutcomeV1;
  events: DirectoryEvent[];
  result: DirectoryCommandResultV1;
  receiptSha256: string;
}

/** 🚫️ Closed transport denial classes; the first six are the only codes the hub puts on the wire. */
export type DirectoryCommandErrorCodeV1 =
  | "unauthorized"
  | "forbidden"
  | "stale-session"
  | "request-conflict"
  | "invalid"
  | "overloaded"
  | "too-large"
  | "capacity"
  | "closed"
  | "cancelled"
  | "transport";

const DIRECTORY_COMMAND_TRANSIENT_CODES: readonly DirectoryCommandErrorCodeV1[] = ["overloaded", "transport"];

/** 🔁️ Only transient faults may retry the byte-identical sealed request. */
export function directoryCommandErrorIsTransient(code: DirectoryCommandErrorCodeV1): boolean {
  return DIRECTORY_COMMAND_TRANSIENT_CODES.includes(code);
}

/** 🌐️ Maps one non-2xx status to its closed code without preserving any response body. */
export function directoryCommandErrorFromStatus(status: number): DirectoryCommandErrorCodeV1 {
  if (status === 401) return "unauthorized";
  if (status === 403) return "forbidden";
  if (status === 409) return "request-conflict";
  if (status === 410) return "stale-session";
  if (status === 413) return "too-large";
  if (status === 503) return "overloaded";
  return "invalid";
}

const DIRECTORY_COMMAND_FIELDS: Record<string, readonly string[]> = {
  "create-space": ["kind", "name", "spaceKind", "visibility"],
  "rename-space": ["kind", "spaceId", "name"],
  "set-visibility": ["kind", "spaceId", "visibility"],
  "archive-space": ["kind", "spaceId"],
  "delete-space": ["kind", "spaceId"],
  "upsert-member": ["kind", "spaceId", "email", "role"],
  "remove-member": ["kind", "spaceId", "userId"],
  "create-invite": ["kind", "spaceId", "role", "ttlSecs"],
  "revoke-invite": ["kind", "spaceId", "inviteId"],
  "announce-document": ["kind", "descriptor"],
};

function directoryCommandRequestId(value: unknown): string {
  if (typeof value !== "string" || value.length !== DIRECTORY_COMMAND_REQUEST_ID_LEN || !/^[0-9a-f]+$/u.test(value) || /^0+$/u.test(value)) throw new Error("directory-command.invalid-request-id");
  return value;
}

function directoryCommandCanonicalCommand(value: unknown): DirectoryCommand {
  const command = directoryEventPageObject(value, ["kind"], Object.values(DIRECTORY_COMMAND_FIELDS).flat());
  const fields = typeof command.kind === "string" ? DIRECTORY_COMMAND_FIELDS[command.kind] : undefined;
  if (!fields) throw new Error("directory-command.invalid-kind");
  directoryEventPageObject(command, fields);
  const canonical: Record<string, unknown> = {};
  for (const field of fields) canonical[field] = command[field];
  if (JSON.stringify(canonical) !== JSON.stringify(command)) throw new Error("directory-command.noncanonical-command");
  if (command.kind === "create-invite") directoryEventPageInteger(command.ttlSecs, true);
  if (directoryEventPageHasControl(command)) throw new Error("directory-command.control-character");
  return canonical as unknown as DirectoryCommand;
}

function directoryCommandCanonicalResult(value: unknown): DirectoryCommandResultV1 {
  const result = directoryEventPageObject(value, ["kind"], ["inviteToken"]);
  if (result.kind === "none") {
    directoryEventPageObject(result, ["kind"]);
    return { kind: "none" };
  }
  if (result.kind !== "invite") throw new Error("directory-command.invalid-result");
  directoryEventPageObject(result, ["kind", "inviteToken"]);
  const inviteToken = result.inviteToken;
  if (typeof inviteToken !== "string" || inviteToken.length === 0 || new TextEncoder().encode(inviteToken).length > DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES || /\p{Cc}/u.test(inviteToken)) throw new Error("directory-command.invalid-invite-token");
  return { kind: "invite", inviteToken };
}

/** 🔐️ The one canonical command digest both the hub and every client derive independently. */
export async function directoryCommandSha256(command: DirectoryCommand): Promise<string> {
  return directoryEventPageSha256(JSON.stringify(command));
}

/** 🆕️ Seals one request around an already-minted correlation id. */
export function sealDirectoryCommandRequestV1(requestId: string, command: DirectoryCommand): DirectoryCommandRequestV1 {
  return { schema: "semio.directory.command-request.v1", requestId: directoryCommandRequestId(requestId), command: directoryCommandCanonicalCommand(command) };
}

/** 🧾️ Returns the canonical UTF-8 JSON both peers hash and count bytes over. */
export function directoryCommandRequestJson(request: DirectoryCommandRequestV1): string {
  const canonical: DirectoryCommandRequestV1 = { schema: request.schema, requestId: request.requestId, command: request.command };
  const json = JSON.stringify(canonical);
  if (new TextEncoder().encode(json).length > DIRECTORY_COMMAND_REQUEST_MAX_BYTES) throw new Error("directory-command.request-too-large");
  return json;
}

/** 📥️ Parses exactly one canonical request, rejecting padding, unknown fields, and oversize bodies. */
export function parseDirectoryCommandRequestV1(source: string): DirectoryCommandRequestV1 {
  if (new TextEncoder().encode(source).length > DIRECTORY_COMMAND_REQUEST_MAX_BYTES) throw new Error("directory-command.request-too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "requestId", "command"]);
  if (object.schema !== "semio.directory.command-request.v1") throw new Error("directory-command.invalid-envelope");
  const request: DirectoryCommandRequestV1 = { schema: object.schema, requestId: directoryCommandRequestId(object.requestId), command: directoryCommandCanonicalCommand(object.command) };
  if (JSON.stringify(request) !== source) throw new Error("directory-command.noncanonical");
  return request;
}

/** 📥️ Parses exactly one canonical receipt bound to the request that asked for it. */
export async function parseDirectoryCommandReceiptV1(source: string, request: DirectoryCommandRequestV1): Promise<DirectoryCommandReceiptV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES) throw new Error("directory-command.receipt-too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "requestId", "commandSha256", "outcome", "events", "result", "receiptSha256"]);
  if (object.schema !== "semio.directory.command-receipt.v1"
    || (object.outcome !== "accepted" && object.outcome !== "previously-accepted" && object.outcome !== "secret-undeliverable")
    || !Array.isArray(object.events)
    || object.events.length > DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS) throw new Error("directory-command.invalid-envelope");
  const events = object.events.map(directoryEventPageEvent);
  let previous = 0;
  for (const event of events) {
    if (event.seq <= previous) throw new Error("directory-command.invalid-event-range");
    previous = event.seq;
  }
  const receipt: DirectoryCommandReceiptV1 = {
    schema: object.schema,
    requestId: directoryCommandRequestId(object.requestId),
    commandSha256: directoryEventPageHash(object.commandSha256, false),
    outcome: object.outcome,
    events,
    result: directoryCommandCanonicalResult(object.result),
    receiptSha256: directoryEventPageHash(object.receiptSha256, false),
  };
  if (receipt.outcome !== "accepted" && (receipt.result.kind !== "none" || receipt.events.length > 0)) throw new Error("directory-command.redaction-violated");
  if (receipt.requestId !== request.requestId || receipt.commandSha256 !== await directoryCommandSha256(request.command)) throw new Error("directory-command.request-mismatch");
  if (JSON.stringify(receipt) !== source) throw new Error("directory-command.noncanonical");
  const { receiptSha256, ...unsigned } = receipt;
  if (await directoryEventPageSha256(JSON.stringify(unsigned)) !== receiptSha256) throw new Error("directory-command.receipt-mismatch");
  return receipt;
}
//#endregion 🔖️CommandReceipt

//#region 🔖️Admin
export type AdminIntentV1 =
  | { kind: "create-space"; requestId: string; name: string; spaceKind: DirectorySpaceKind; visibility: DirectorySpaceVisibility }
  | { kind: "rename-space"; requestId: string; spaceId: string; name: string }
  | { kind: "set-space-visibility"; requestId: string; spaceId: string; visibility: DirectorySpaceVisibility }
  | { kind: "archive-space"; requestId: string; spaceId: string }
  | { kind: "delete-space"; requestId: string; spaceId: string }
  | { kind: "upsert-space-member"; requestId: string; spaceId: string; email: string; role: DirectorySpaceRole }
  | { kind: "remove-space-member"; requestId: string; spaceId: string; userId: string }
  | { kind: "create-space-invite"; requestId: string; spaceId: string; role: DirectorySpaceRole; ttlSecs: number }
  | { kind: "revoke-space-invite"; requestId: string; spaceId: string; inviteId: string }
  | { kind: "issue-document-share"; requestId: string; scope: DocumentScope; ttlSecs: number }
  | { kind: "revoke-document-share"; requestId: string; scope: DocumentScope; shareId: string; reasonCode: string }
  | { kind: "revoke-user-sessions"; requestId: string; userId: string; reasonCode: string }
  | { kind: "kick-connection"; requestId: string; syncSessionId: string; reasonCode: string }
  | { kind: "rebuild-directory-projections"; requestId: string; expectedHeadSeq: number };

export type AdminIntentStateV1 = "succeeded" | "accepted" | "failed" | "cancelled";

export interface AdminIntentOutcomeV1 {
  code: string;
  durable: boolean;
  kickAttempted?: number;
  kickSignalled?: number;
}

export interface AdminIntentResultV1 {
  inviteToken?: string;
  shareToken?: string;
}

export interface AdminIntentReceiptV1 {
  operationId: string;
  correlationId: string;
  state: AdminIntentStateV1;
  eventSeqFirst?: number;
  eventSeqLast?: number;
  result?: AdminIntentResultV1;
  outcome: AdminIntentOutcomeV1;
}

export interface AdminOperationProgressV1 {
  completedEvents: number;
  totalEvents: number;
  cancelRequested: boolean;
}

export interface AdminOperationStatusV1 {
  receipt: AdminIntentReceiptV1;
  progress?: AdminOperationProgressV1;
}

export interface AdminPageV1<T> {
  rows: T[];
  nextCursor?: string;
  observedAtMs: number;
}

export interface AdminRecordedConnectionV1 {
  syncSessionId: string;
  scope: DocumentScope;
  authenticatedUserId?: string;
  email?: string;
  role?: DirectorySpaceRole;
  connectedAtMs: number;
  source: "recorded-sync-session";
}

export interface AdminConnectionSnapshotV1 extends AdminPageV1<AdminRecordedConnectionV1> {
  source: "recorded-sync-sessions";
  headSeq: number;
}

export type AdminOperationAuditPhaseV1 = "accepted" | "succeeded" | "failed" | "cancelled";

export interface AdminOperationAuditV1 {
  sequence: number;
  operationId: string;
  occurredAtMs: number;
  phase: AdminOperationAuditPhaseV1;
  intentKind: string;
  targetKind: string;
  targetId: string;
  principalUserId: string;
  principalSessionId: string;
  principalGeneration: number;
  correlationId: string;
  eventSeqFirst?: number;
  eventSeqLast?: number;
  outcomeCode: string;
  reasonCode?: string;
}
//#endregion 🔖️Admin

//#region 🔖️Views
/** 🏠️ One space, as the hub's REST/read surface renders it. `role` is the calling user's
 * membership role (server-filled per request), never derived by the pure fold. */
export interface SpaceView {
  id: string;
  name: string;
  kind: DirectorySpaceKind;
  visibility: DirectorySpaceVisibility;
  ownerUserId: string;
  role?: DirectorySpaceRole;
  memberCount: number;
  documentCount: number;
  activeConnections: number;
  createdAtMs: number;
  updatedAtMs: number;
}

/** 🌐️ Discoverable metadata with no account identity, caller role, or live activity. */
export interface PublicSpaceViewV1 {
  id: string;
  name: string;
  kind: DirectorySpaceKind;
  visibility: DirectorySpaceVisibility;
  memberCount: number;
  documentCount: number;
  createdAtMs: number;
  updatedAtMs: number;
}

/** 🔐️ Membership-qualified metadata with a required caller role. */
export interface MemberSpaceViewV1 {
  id: string;
  name: string;
  kind: DirectorySpaceKind;
  visibility: DirectorySpaceVisibility;
  ownerUserId: string;
  role: DirectorySpaceRole;
  memberCount: number;
  documentCount: number;
  activeConnections: number;
  createdAtMs: number;
  updatedAtMs: number;
}

/** 📖️ Discoverable document identity without replication/currentness metadata. */
export interface PublicDocumentCatalogEntryV1 {
  documentId: string;
  artifactKind: string;
  artifactSchema: string;
  owner: DocumentOwner;
  packSchemaHash: string;
}

export type DirectorySpaceListEntryV1 =
  | { access: "public"; space: PublicSpaceViewV1 }
  | { access: "member"; space: MemberSpaceViewV1 }
  | { access: "author"; space: MemberSpaceViewV1 };

export const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS = 64;
export const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES = 48 * 1024;
export const DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES = 512;
export const DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA = "semio.directory.space-administration-page.v1";

/** 🗂️ The one independently paged window a cursor may advance. */
export type DirectorySpaceAdministrationSectionV1 = "members" | "invites" | "documents";

/** 🧑️ One administration-page member row; never carries a credential or provider column. */
export interface DirectorySpaceAdministrationMemberRowV1 {
  userId: string;
  email: string;
  displayName: string;
  role: DirectorySpaceRole;
  owner: boolean;
}

/** 🎟️ One administration-page invite row; never carries the selector, digest, or capability. */
export interface DirectorySpaceAdministrationInviteRowV1 {
  inviteId: string;
  role: DirectorySpaceRole;
  createdAtMs: number;
  expiresAtMs: number;
  revoked: boolean;
  accepted: boolean;
}

/** 🪟️ One bounded window; `nextCursor` is present exactly when more rows remain. */
export interface DirectorySpaceAdministrationWindowV1<Row> {
  rows: Row[];
  nextCursor?: string;
}

/** 🛂️ Server-decided administration affordances; the only authority a renderer may consult. */
export interface DirectorySpaceAdministrationCapabilitiesV1 {
  renameSpace: boolean;
  setVisibility: boolean;
  deleteSpace: boolean;
  upsertMember: boolean;
  removeMember: boolean;
  createInvite: boolean;
  revokeInvite: boolean;
}

/** 🏛️ One authenticated, receipt-bound bounded administration projection of exactly one space. */
export type DirectorySpaceAdministrationPageV1 =
  | {
      access: "public";
      schema: typeof DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA;
      sessionBindingSha256: string;
      authorizationGeneration: number;
      spaceId: string;
      space: PublicSpaceViewV1;
      documents: DirectorySpaceAdministrationWindowV1<PublicDocumentCatalogEntryV1>;
      receiptSha256: string;
    }
  | {
      access: "member";
      schema: typeof DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA;
      sessionBindingSha256: string;
      authorizationGeneration: number;
      spaceId: string;
      space: MemberSpaceViewV1;
      members: DirectorySpaceAdministrationWindowV1<DirectorySpaceAdministrationMemberRowV1>;
      documents: DirectorySpaceAdministrationWindowV1<DocumentView>;
      receiptSha256: string;
    }
  | {
      access: "author";
      schema: typeof DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA;
      sessionBindingSha256: string;
      authorizationGeneration: number;
      spaceId: string;
      space: MemberSpaceViewV1;
      members: DirectorySpaceAdministrationWindowV1<DirectorySpaceAdministrationMemberRowV1>;
      documents: DirectorySpaceAdministrationWindowV1<DocumentView>;
      invites: DirectorySpaceAdministrationWindowV1<DirectorySpaceAdministrationInviteRowV1>;
      capabilities: DirectorySpaceAdministrationCapabilitiesV1;
      receiptSha256: string;
    };

function administrationObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("space-administration-page.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  if (required.some((key) => !(key in object)) || Object.keys(object).some((key) => !accepted.has(key))) throw new Error("space-administration-page.invalid-fields");
  return object;
}

function administrationText(value: unknown, maximum = DOCUMENT_OPEN_ID_MAX_BYTES, allowEmpty = false): string {
  if (typeof value !== "string" || (!allowEmpty && value.length === 0) || new TextEncoder().encode(value).length > maximum || /\p{Cc}/u.test(value)) throw new Error("space-administration-page.invalid-text");
  return value;
}

function administrationTime(value: unknown): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("space-administration-page.invalid-time");
  return value;
}

function administrationBoolean(value: unknown): boolean {
  if (typeof value !== "boolean") throw new Error("space-administration-page.invalid-boolean");
  return value;
}

function administrationRole(value: unknown): DirectorySpaceRole {
  if (value !== "author" && value !== "spectator") throw new Error("space-administration-page.invalid-role");
  return value;
}

function administrationCursor(object: Record<string, unknown>): string | undefined {
  if (!("nextCursor" in object)) return undefined;
  const cursor = object.nextCursor;
  if (typeof cursor !== "string" || cursor.length === 0 || cursor.length > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !/^[A-Za-z0-9._-]+$/u.test(cursor)) throw new Error("space-administration-page.invalid-cursor");
  return cursor;
}

function administrationWindow<Row>(value: unknown, row: (value: unknown) => Row): DirectorySpaceAdministrationWindowV1<Row> {
  const object = administrationObject(value, ["rows"], ["nextCursor"]);
  if (!Array.isArray(object.rows) || object.rows.length > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS) throw new Error("space-administration-page.invalid-window");
  const rows = object.rows.map(row);
  const nextCursor = administrationCursor(object);
  return nextCursor === undefined ? { rows } : { rows, nextCursor };
}

function administrationMemberRow(value: unknown): DirectorySpaceAdministrationMemberRowV1 {
  const object = administrationObject(value, ["userId", "email", "displayName", "role", "owner"]);
  return {
    userId: administrationText(object.userId),
    email: administrationText(object.email, DOCUMENT_OPEN_ID_MAX_BYTES, true),
    displayName: administrationText(object.displayName, DOCUMENT_OPEN_ID_MAX_BYTES, true),
    role: administrationRole(object.role),
    owner: administrationBoolean(object.owner),
  };
}

function administrationInviteRow(value: unknown): DirectorySpaceAdministrationInviteRowV1 {
  const object = administrationObject(value, ["inviteId", "role", "createdAtMs", "expiresAtMs", "revoked", "accepted"]);
  return {
    inviteId: administrationText(object.inviteId),
    role: administrationRole(object.role),
    createdAtMs: administrationTime(object.createdAtMs),
    expiresAtMs: administrationTime(object.expiresAtMs),
    revoked: administrationBoolean(object.revoked),
    accepted: administrationBoolean(object.accepted),
  };
}

function administrationCapabilities(value: unknown): DirectorySpaceAdministrationCapabilitiesV1 {
  const object = administrationObject(value, ["renameSpace", "setVisibility", "deleteSpace", "upsertMember", "removeMember", "createInvite", "revokeInvite"]);
  return {
    renameSpace: administrationBoolean(object.renameSpace),
    setVisibility: administrationBoolean(object.setVisibility),
    deleteSpace: administrationBoolean(object.deleteSpace),
    upsertMember: administrationBoolean(object.upsertMember),
    removeMember: administrationBoolean(object.removeMember),
    createInvite: administrationBoolean(object.createInvite),
    revokeInvite: administrationBoolean(object.revokeInvite),
  };
}

async function administrationSha256(text: string): Promise<string> {
  const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 📥️ Parses one canonical administration page and verifies fields, ordering, size, and receipt. */
export async function parseDirectorySpaceAdministrationPageV1(source: string): Promise<DirectorySpaceAdministrationPageV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES) throw new Error("space-administration-page.too-large");
  const parsed = JSON.parse(source);
  const access = (parsed as Record<string, unknown> | null)?.access;
  const common = ["access", "schema", "sessionBindingSha256", "authorizationGeneration", "spaceId", "space"];
  const shape = access === "public" ? [...common, "documents", "receiptSha256"]
    : access === "member" ? [...common, "members", "documents", "receiptSha256"]
    : access === "author" ? [...common, "members", "documents", "invites", "capabilities", "receiptSha256"]
    : undefined;
  if (shape === undefined) throw new Error("space-administration-page.invalid-access");
  const object = administrationObject(parsed, shape);
  if (object.schema !== DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA) throw new Error("space-administration-page.invalid-schema");
  const sessionBindingSha256 = administrationText(object.sessionBindingSha256);
  if (!/^[0-9a-f]{64}$/u.test(sessionBindingSha256)) throw new Error("space-administration-page.invalid-binding");
  const authorizationGeneration = administrationTime(object.authorizationGeneration);
  const spaceId = administrationText(object.spaceId);
  const receiptSha256 = administrationText(object.receiptSha256);
  if (!/^[0-9a-f]{64}$/u.test(receiptSha256)) throw new Error("space-administration-page.invalid-receipt");
  const anonymous = authorizationGeneration === 0 && /^0{64}$/u.test(sessionBindingSha256);
  const bound = authorizationGeneration >= 1 && !/^0{64}$/u.test(sessionBindingSha256);
  if (!(anonymous || bound) || (access !== "public" && !bound)) throw new Error("space-administration-page.invalid-binding");
  const space = object.space as Record<string, unknown> | null;
  if (space === null || typeof space !== "object" || space.id !== spaceId) throw new Error("space-administration-page.space-mismatch");
  if (access === "public" && space.visibility !== "public") throw new Error("space-administration-page.space-mismatch");
  if (access === "member" && space.role !== "spectator") throw new Error("space-administration-page.space-mismatch");
  if (access === "author" && space.role !== "author") throw new Error("space-administration-page.space-mismatch");
  const documents = administrationWindow<unknown>(object.documents, (row) => row);
  const members = access === "public" ? undefined : administrationWindow(object.members, administrationMemberRow);
  if (members !== undefined) {
    let previous: string | undefined;
    for (const row of members.rows) {
      if (previous !== undefined && previous >= row.userId) throw new Error("space-administration-page.member-order");
      previous = row.userId;
    }
  }
  const invites = access === "author" ? administrationWindow(object.invites, administrationInviteRow) : undefined;
  if (invites !== undefined) {
    let previous: readonly [number, string] | undefined;
    for (const row of invites.rows) {
      if (previous !== undefined && !(previous[0] > row.createdAtMs || (previous[0] === row.createdAtMs && previous[1] > row.inviteId))) throw new Error("space-administration-page.invite-order");
      previous = [row.createdAtMs, row.inviteId];
    }
  }
  const capabilities = access === "author" ? administrationCapabilities(object.capabilities) : undefined;
  const base = { access, schema: object.schema, sessionBindingSha256, authorizationGeneration, spaceId, space: object.space };
  const page = (access === "public" ? { ...base, documents, receiptSha256 }
    : access === "member" ? { ...base, members, documents, receiptSha256 }
    : { ...base, members, documents, invites, capabilities, receiptSha256 }) as unknown as DirectorySpaceAdministrationPageV1;
  if (JSON.stringify(page) !== source) throw new Error("space-administration-page.noncanonical");
  const { receiptSha256: _receipt, ...unsigned } = page as Record<string, unknown> & { receiptSha256: string };
  if (await administrationSha256(JSON.stringify(unsigned)) !== receiptSha256) throw new Error("space-administration-page.receipt-mismatch");
  return page;
}

export interface MemberView {
  userId: string;
  email: string;
  displayName: string;
  role: DirectorySpaceRole;
}

export interface UserView {
  id: string;
  email: string;
  displayName: string;
  createdAtMs: number;
}

export interface ConnectionView {
  syncSessionId: string;
  spaceId: string;
  documentId: string;
  surface: string;
  actor: string;
  userId?: string;
  email?: string;
  role: DirectorySpaceRole;
  connectedAtMs: number;
  presenceKnown: boolean;
}

export interface DocumentOwner {
  pluginId: string;
  packageId: string;
  version: string;
  packageHash: string;
}

export interface DocumentScope {
  spaceId: string;
  documentId: string;
}

export type ArtifactHash = readonly number[];
export type CheckpointId = ArtifactHash;

export interface DocumentFrontier {
  headSeq: number;
  commitSeq: number;
  epoch: number;
}

export interface DocumentDescriptor {
  spaceId: string;
  documentId: string;
  artifactKind: string;
  artifactSchema: string;
  owner: DocumentOwner;
  packSchemaHash: string;
  bootstrapVersion: number;
  bootstrapFrontier: DocumentFrontier;
  bootstrapSnapshotHash: string;
}

export const DOCUMENT_OPEN_ID_MAX_BYTES = 256;
export const DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES = 128;
export const DOCUMENT_OPEN_PLAN_MAX_TTL_MS = 30_000;

export interface DocumentOpenIntentV1 {
  schema: "semio.hub.document-open-intent/v1";
  version: 1;
  scope: DocumentScope;
  requestedSurfaceId?: string;
  clientInstanceId: string;
}

export type DocumentOpenRendererTargetV1 = "react" | "wgpu" | "wasm";
export type DocumentOpenSurfaceRoleV1 = "viewer" | "editor";

export interface DocumentOpenCatalogV1 {
  generationId: string;
}

export interface DocumentOpenPackageV1 {
  pluginId: string;
  packageId: string;
  version: string;
  componentSha256: string;
  componentBlake3: string;
  descriptorByteSha256: string;
}

export interface DocumentOpenArtifactV1 {
  kind: string;
  schema: string;
  packSchemaHash: string;
}

export interface DocumentOpenParentDialectV1 {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface DocumentOpenSurfaceV1 {
  surfaceId: string;
  appId: string;
  windowKindId: string;
  role: DocumentOpenSurfaceRoleV1;
  rendererTarget: DocumentOpenRendererTargetV1;
}

export interface DocumentOpenGrantV1 {
  read: true;
  write: boolean;
  observe: true;
}

export interface DocumentOpenCheckpointV1 {
  checkpointId: string;
  descriptorDigestV1: string;
  baselineFrontier: ArtifactFrontier;
  aggregateSha256: string;
}

export interface DocumentOpenRevalidationV1 {
  directoryRevision: number;
  membershipGeneration: number;
  sessionGeneration?: number;
  shareGeneration?: number;
}

export interface DocumentOpenPlanV1 {
  schema: "semio.hub.document-open-plan/v1";
  version: 1;
  receipt: string;
  expiresAtUnixMs: number;
  scope: DocumentScope;
  descriptorDigestV1: string;
  catalog: DocumentOpenCatalogV1;
  package: DocumentOpenPackageV1;
  artifact: DocumentOpenArtifactV1;
  parentDialect: DocumentOpenParentDialectV1;
  surface: DocumentOpenSurfaceV1;
  grant: DocumentOpenGrantV1;
  checkpoint?: DocumentOpenCheckpointV1;
  revalidation: DocumentOpenRevalidationV1;
}

export interface DocumentPlanSocketGrantIntentV1 {
  schema: "semio.hub.document-plan-socket-grant-intent/v1";
  version: 1;
  planReceipt: string;
}

export type DocumentOpenPlanErrorCodeV1 = "denied" | "not-found" | "catalog-unavailable" | "component-unavailable" | "stale" | "expired" | "already-consumed" | "cancelled" | "deadline-exceeded";

export interface DocumentOpenPlanErrorV1 {
  schema: "semio.hub.document-open-plan-error/v1";
  code: DocumentOpenPlanErrorCodeV1;
}

function documentOpenObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("document-open.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  if (Object.keys(object).some((key) => !accepted.has(key)) || required.some((key) => !(key in object))) throw new Error("document-open.invalid-fields");
  return object;
}

function documentOpenText(value: unknown, maxBytes = DOCUMENT_OPEN_ID_MAX_BYTES): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > maxBytes || /\p{Cc}/u.test(value)) throw new Error("document-open.invalid-text");
  return value;
}

function documentOpenHash(value: unknown): string {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || /^0{64}$/u.test(value)) throw new Error("document-open.invalid-hash");
  return value;
}

function documentOpenInteger(value: unknown, positive = false): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("document-open.invalid-integer");
  return value;
}

function documentOpenReceipt(value: unknown): string {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
  if (typeof value !== "string" || !/^open\.v1\.[A-Za-z0-9_-]{43}$/u.test(value) || (alphabet.indexOf(value.at(-1)!) & 0b11) !== 0) throw new Error("document-open.invalid-receipt");
  return value;
}

function parseDocumentOpenScope(value: unknown): DocumentScope {
  const object = documentOpenObject(value, ["spaceId", "documentId"]);
  return { spaceId: documentOpenText(object.spaceId), documentId: documentOpenText(object.documentId) };
}

function parseDocumentOpenFrontier(value: unknown, documentId: string): ArtifactFrontier {
  const object = documentOpenObject(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
  const chainHash = object.chainHash;
  if (!Array.isArray(chainHash) || chainHash.length !== 32 || chainHash.every((byte) => byte === 0) || chainHash.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error("document-open.invalid-frontier-hash");
  const frontier = {
    documentId: documentOpenText(object.documentId),
    headEditOrdinal: documentOpenInteger(object.headEditOrdinal),
    headEditId: documentOpenText(object.headEditId),
    lastCommitSeq: documentOpenInteger(object.lastCommitSeq),
    chainHash: chainHash as number[],
  };
  if (frontier.documentId !== documentId || frontier.lastCommitSeq > frontier.headEditOrdinal) throw new Error("document-open.stale-frontier");
  return frontier;
}

export function parseDocumentOpenIntentV1(value: unknown): DocumentOpenIntentV1 {
  const object = documentOpenObject(value, ["schema", "version", "scope", "clientInstanceId"], ["requestedSurfaceId"]);
  if (object.schema !== "semio.hub.document-open-intent/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  return {
    schema: object.schema,
    version: object.version,
    scope: parseDocumentOpenScope(object.scope),
    ...(object.requestedSurfaceId === undefined ? {} : { requestedSurfaceId: documentOpenText(object.requestedSurfaceId) }),
    clientInstanceId: documentOpenText(object.clientInstanceId, DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES),
  };
}

export function parseDocumentPlanSocketGrantIntentV1(value: unknown): DocumentPlanSocketGrantIntentV1 {
  const object = documentOpenObject(value, ["schema", "version", "planReceipt"]);
  if (object.schema !== "semio.hub.document-plan-socket-grant-intent/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  return { schema: object.schema, version: object.version, planReceipt: documentOpenReceipt(object.planReceipt) };
}

export function parseDocumentOpenPlanV1(value: unknown, nowMs: number): DocumentOpenPlanV1 {
  const object = documentOpenObject(value, ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "parentDialect", "surface", "grant", "revalidation"], ["checkpoint"]);
  if (object.schema !== "semio.hub.document-open-plan/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const packageValue = documentOpenObject(object.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256"]);
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const parentDialect = documentOpenObject(object.parentDialect, ["artifactKind", "standard", "subset"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  const expiresAtUnixMs = documentOpenInteger(object.expiresAtUnixMs, true);
  if (expiresAtUnixMs <= nowMs || expiresAtUnixMs - nowMs > DOCUMENT_OPEN_PLAN_MAX_TTL_MS || (revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("document-open.expired-or-ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-open.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor")) throw new Error("document-open.invalid-surface");
  const parsedParentDialect = {
    artifactKind: documentOpenText(parentDialect.artifactKind),
    standard: documentOpenText(parentDialect.standard),
    subset: documentOpenText(parentDialect.subset),
  };
  if (parsedParentDialect.artifactKind !== artifact.kind || Object.values(parsedParentDialect).some((value) => value.trim() !== value)) throw new Error("document-open.invalid-parent-dialect");
  const checkpoint = object.checkpoint === undefined ? undefined : documentOpenObject(object.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  if (checkpoint && checkpoint.descriptorDigestV1 !== descriptorDigestV1) throw new Error("document-open.stale-checkpoint");
  return {
    schema: object.schema,
    version: object.version,
    receipt: documentOpenReceipt(object.receipt),
    expiresAtUnixMs,
    scope,
    descriptorDigestV1,
    catalog: { generationId: documentOpenHash(catalog.generationId) },
    package: {
      pluginId: documentOpenText(packageValue.pluginId),
      packageId: documentOpenText(packageValue.packageId),
      version: documentOpenText(packageValue.version),
      componentSha256: documentOpenHash(packageValue.componentSha256),
      componentBlake3: documentOpenHash(packageValue.componentBlake3),
      descriptorByteSha256: documentOpenHash(packageValue.descriptorByteSha256),
    },
    artifact: { kind: documentOpenText(artifact.kind), schema: documentOpenText(artifact.schema), packSchemaHash: documentOpenHash(artifact.packSchemaHash) },
    parentDialect: parsedParentDialect,
    surface: {
      surfaceId: documentOpenText(surface.surfaceId),
      appId: documentOpenText(surface.appId),
      windowKindId: documentOpenText(surface.windowKindId),
      role: surface.role,
      rendererTarget: surface.rendererTarget,
    },
    grant: { read: true, write: grant.write, observe: true },
    ...(checkpoint ? {
      checkpoint: {
        checkpointId: documentOpenHash(checkpoint.checkpointId),
        descriptorDigestV1,
        baselineFrontier: parseDocumentOpenFrontier(checkpoint.baselineFrontier, scope.documentId),
        aggregateSha256: documentOpenHash(checkpoint.aggregateSha256),
      },
    } : {}),
    revalidation: {
      directoryRevision: documentOpenInteger(revalidation.directoryRevision, true),
      membershipGeneration: documentOpenInteger(revalidation.membershipGeneration, true),
      ...(revalidation.sessionGeneration === undefined ? {} : { sessionGeneration: documentOpenInteger(revalidation.sessionGeneration, true) }),
      ...(revalidation.shareGeneration === undefined ? {} : { shareGeneration: documentOpenInteger(revalidation.shareGeneration, true) }),
    },
  };
}

//#region 🪪️ExecutionTargetLease
/** 🧯️ Exact maximum accepted bytes for one verified execution-target component — the trusted
 * catalog's own `TRUSTED_COMPONENT_MAX_BYTES`, shared verbatim by every transport. */
export const DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES = 64 * 1024 * 1024;
/** 🧯️ Exact maximum accepted bytes for one verified raw package descriptor — the trusted catalog's
 * own `TRUSTED_DESCRIPTOR_MAX_BYTES`, shared verbatim by every transport. */
export const DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;

/** 🧱️ Exact byte identity of the verified component, duplicated from {@link DocumentOpenPackageV1}
 * so a server byte response can never silently answer with a different object. */
export interface DocumentExecutionTargetComponentV1 {
  sha256: string;
  blake3: string;
  byteLength: number;
}

/** 📜️ Exact byte identity of the verified raw package descriptor. */
export interface DocumentExecutionTargetDescriptorV1 {
  sha256: string;
  byteLength: number;
}

/** 🪪️ Receipt-free public fields of one document execution-target lease. It carries no plan
 * receipt, socket grant, session token, hub origin, raw path or module URL: those are never lease
 * fields and never lease constructors. */
export interface DocumentExecutionTargetLeaseFieldsV1 {
  schema: "semio.os.document-execution-target-lease/v1";
  version: 1;
  scope: DocumentScope;
  descriptorDigestV1: string;
  catalog: DocumentOpenCatalogV1;
  package: DocumentOpenPackageV1;
  component: DocumentExecutionTargetComponentV1;
  descriptor: DocumentExecutionTargetDescriptorV1;
  artifact: DocumentOpenArtifactV1;
  parentDialect: DocumentOpenParentDialectV1;
  surface: DocumentOpenSurfaceV1;
  grant: DocumentOpenGrantV1;
  checkpoint?: DocumentOpenCheckpointV1;
  revalidation: DocumentOpenRevalidationV1;
}

function documentExecutionTargetByteLength(value: unknown, maxBytes: number): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 1 || value > maxBytes) throw new Error("document-execution-target-lease.invalid-byte-length");
  return value;
}

/** ✅️ Strictly parses the public lease fields and enforces every byte/identity invariant: both
 * component digests and the descriptor digest must equal the package projection, the parent dialect
 * must share the artifact kind, the grant must follow the surface role, and an optional checkpoint
 * must carry the same descriptor digest. */
export function parseDocumentExecutionTargetLeaseFieldsV1(value: unknown): DocumentExecutionTargetLeaseFieldsV1 {
  const object = documentOpenObject(value, ["schema", "version", "scope", "descriptorDigestV1", "catalog", "package", "component", "descriptor", "artifact", "parentDialect", "surface", "grant", "revalidation"], ["checkpoint"]);
  if (object.schema !== "semio.os.document-execution-target-lease/v1" || object.version !== 1) throw new Error("document-execution-target-lease.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const packageValue = documentOpenObject(object.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256"]);
  const component = documentOpenObject(object.component, ["sha256", "blake3", "byteLength"]);
  const descriptor = documentOpenObject(object.descriptor, ["sha256", "byteLength"]);
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const parentDialect = documentOpenObject(object.parentDialect, ["artifactKind", "standard", "subset"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  if ((revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("document-execution-target-lease.ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-execution-target-lease.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor")) throw new Error("document-execution-target-lease.invalid-surface");
  const parsedParentDialect = {
    artifactKind: documentOpenText(parentDialect.artifactKind),
    standard: documentOpenText(parentDialect.standard),
    subset: documentOpenText(parentDialect.subset),
  };
  if (parsedParentDialect.artifactKind !== artifact.kind || Object.values(parsedParentDialect).some((entry) => entry.trim() !== entry)) throw new Error("document-execution-target-lease.invalid-parent-dialect");
  const parsedPackage = {
    pluginId: documentOpenText(packageValue.pluginId),
    packageId: documentOpenText(packageValue.packageId),
    version: documentOpenText(packageValue.version),
    componentSha256: documentOpenHash(packageValue.componentSha256),
    componentBlake3: documentOpenHash(packageValue.componentBlake3),
    descriptorByteSha256: documentOpenHash(packageValue.descriptorByteSha256),
  };
  const parsedComponent = {
    sha256: documentOpenHash(component.sha256),
    blake3: documentOpenHash(component.blake3),
    byteLength: documentExecutionTargetByteLength(component.byteLength, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES),
  };
  const parsedDescriptor = {
    sha256: documentOpenHash(descriptor.sha256),
    byteLength: documentExecutionTargetByteLength(descriptor.byteLength, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES),
  };
  if (parsedComponent.sha256 !== parsedPackage.componentSha256 || parsedComponent.blake3 !== parsedPackage.componentBlake3 || parsedDescriptor.sha256 !== parsedPackage.descriptorByteSha256) throw new Error("document-execution-target-lease.unbound-bytes");
  const checkpoint = object.checkpoint === undefined ? undefined : documentOpenObject(object.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  if (checkpoint && checkpoint.descriptorDigestV1 !== descriptorDigestV1) throw new Error("document-execution-target-lease.stale-checkpoint");
  return {
    schema: object.schema,
    version: object.version,
    scope,
    descriptorDigestV1,
    catalog: { generationId: documentOpenHash(catalog.generationId) },
    package: parsedPackage,
    component: parsedComponent,
    descriptor: parsedDescriptor,
    artifact: { kind: documentOpenText(artifact.kind), schema: documentOpenText(artifact.schema), packSchemaHash: documentOpenHash(artifact.packSchemaHash) },
    parentDialect: parsedParentDialect,
    surface: {
      surfaceId: documentOpenText(surface.surfaceId),
      appId: documentOpenText(surface.appId),
      windowKindId: documentOpenText(surface.windowKindId),
      role: surface.role,
      rendererTarget: surface.rendererTarget,
    },
    grant: { read: true, write: grant.write, observe: true },
    ...(checkpoint ? {
      checkpoint: {
        checkpointId: documentOpenHash(checkpoint.checkpointId),
        descriptorDigestV1,
        baselineFrontier: parseDocumentOpenFrontier(checkpoint.baselineFrontier, scope.documentId),
        aggregateSha256: documentOpenHash(checkpoint.aggregateSha256),
      },
    } : {}),
    revalidation: {
      directoryRevision: documentOpenInteger(revalidation.directoryRevision, true),
      membershipGeneration: documentOpenInteger(revalidation.membershipGeneration, true),
      ...(revalidation.sessionGeneration === undefined ? {} : { sessionGeneration: documentOpenInteger(revalidation.sessionGeneration, true) }),
      ...(revalidation.shareGeneration === undefined ? {} : { shareGeneration: documentOpenInteger(revalidation.shareGeneration, true) }),
    },
  };
}

/** 🧾️ Projects one already-validated plan into receipt-free lease fields. The plan constrains every
 * identity but no byte length, so both lengths come from the installation being compared and are
 * independently enforced against the exact streamed bytes before a lease is ever minted. */
export function leaseFieldsFromPlanV1(plan: DocumentOpenPlanV1, byteLengths: { readonly component: number; readonly descriptor: number }): DocumentExecutionTargetLeaseFieldsV1 {
  return parseDocumentExecutionTargetLeaseFieldsV1({
    schema: "semio.os.document-execution-target-lease/v1",
    version: 1,
    scope: plan.scope,
    descriptorDigestV1: plan.descriptorDigestV1,
    catalog: plan.catalog,
    package: plan.package,
    component: { sha256: plan.package.componentSha256, blake3: plan.package.componentBlake3, byteLength: byteLengths.component },
    descriptor: { sha256: plan.package.descriptorByteSha256, byteLength: byteLengths.descriptor },
    artifact: plan.artifact,
    parentDialect: plan.parentDialect,
    surface: plan.surface,
    grant: plan.grant,
    ...(plan.checkpoint ? { checkpoint: plan.checkpoint } : {}),
    revalidation: plan.revalidation,
  });
}

/** ⚖️ The one shared full-field lease relation. Every transport compares every field through it; a
 * browser or native subset comparison is never permitted. */
export function sameLeaseFieldsV1(left: DocumentExecutionTargetLeaseFieldsV1, right: DocumentExecutionTargetLeaseFieldsV1): boolean {
  const sameCheckpoint = left.checkpoint === undefined || right.checkpoint === undefined
    ? left.checkpoint === right.checkpoint
    : left.checkpoint.checkpointId === right.checkpoint.checkpointId
      && left.checkpoint.descriptorDigestV1 === right.checkpoint.descriptorDigestV1
      && left.checkpoint.aggregateSha256 === right.checkpoint.aggregateSha256
      && left.checkpoint.baselineFrontier.documentId === right.checkpoint.baselineFrontier.documentId
      && left.checkpoint.baselineFrontier.headEditOrdinal === right.checkpoint.baselineFrontier.headEditOrdinal
      && left.checkpoint.baselineFrontier.headEditId === right.checkpoint.baselineFrontier.headEditId
      && left.checkpoint.baselineFrontier.lastCommitSeq === right.checkpoint.baselineFrontier.lastCommitSeq
      && left.checkpoint.baselineFrontier.chainHash.length === right.checkpoint.baselineFrontier.chainHash.length
      && left.checkpoint.baselineFrontier.chainHash.every((byte, index) => byte === right.checkpoint!.baselineFrontier.chainHash[index]);
  return left.schema === right.schema
    && left.version === right.version
    && left.scope.spaceId === right.scope.spaceId
    && left.scope.documentId === right.scope.documentId
    && left.descriptorDigestV1 === right.descriptorDigestV1
    && left.catalog.generationId === right.catalog.generationId
    && left.package.pluginId === right.package.pluginId
    && left.package.packageId === right.package.packageId
    && left.package.version === right.package.version
    && left.package.componentSha256 === right.package.componentSha256
    && left.package.componentBlake3 === right.package.componentBlake3
    && left.package.descriptorByteSha256 === right.package.descriptorByteSha256
    && left.component.sha256 === right.component.sha256
    && left.component.blake3 === right.component.blake3
    && left.component.byteLength === right.component.byteLength
    && left.descriptor.sha256 === right.descriptor.sha256
    && left.descriptor.byteLength === right.descriptor.byteLength
    && left.artifact.kind === right.artifact.kind
    && left.artifact.schema === right.artifact.schema
    && left.artifact.packSchemaHash === right.artifact.packSchemaHash
    && left.parentDialect.artifactKind === right.parentDialect.artifactKind
    && left.parentDialect.standard === right.parentDialect.standard
    && left.parentDialect.subset === right.parentDialect.subset
    && left.surface.surfaceId === right.surface.surfaceId
    && left.surface.appId === right.surface.appId
    && left.surface.windowKindId === right.surface.windowKindId
    && left.surface.role === right.surface.role
    && left.surface.rendererTarget === right.surface.rendererTarget
    && left.grant.read === right.grant.read
    && left.grant.write === right.grant.write
    && left.grant.observe === right.grant.observe
    && sameCheckpoint
    && left.revalidation.directoryRevision === right.revalidation.directoryRevision
    && left.revalidation.membershipGeneration === right.revalidation.membershipGeneration
    && left.revalidation.sessionGeneration === right.revalidation.sessionGeneration
    && left.revalidation.shareGeneration === right.revalidation.shareGeneration;
}

/** 🌐️ Complete localized execution-target status vocabulary. No code carries an origin, URL, path,
 * receipt, grant, digest or user identity; EN and DE are both explicit with no default language. */
export type DocumentExecutionTargetStatusCodeV1 = "verifying" | "integrity-failed" | "stale" | "cancelled" | "renderer-unavailable";

export const DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1: Readonly<Record<DocumentExecutionTargetStatusCodeV1, Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  verifying: Object.freeze({ en: "Verifying document component…", de: "Dokumentkomponente wird überprüft…" }),
  "integrity-failed": Object.freeze({ en: "The document component could not be verified. Reopen the document.", de: "Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut." }),
  stale: Object.freeze({ en: "The document target changed. Reopen the document.", de: "Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut." }),
  cancelled: Object.freeze({ en: "Opening the document was cancelled.", de: "Das Öffnen des Dokuments wurde abgebrochen." }),
  "renderer-unavailable": Object.freeze({ en: "The verified document component is ready, but this renderer is unavailable.", de: "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar." }),
});

/** 🔊️ ARIA live-region politeness for one execution-target status: progress announces, every
 * terminal integrity/stale/renderer outcome asserts. */
export function documentExecutionTargetStatusRoleV1(code: DocumentExecutionTargetStatusCodeV1): "status" | "alert" {
  return code === "verifying" ? "status" : "alert";
}

/** 📈️ Bounded install progress. It never carries bytes, paths, receipts or full digests. */
export interface DocumentExecutionTargetProgressV1 {
  stage: "manifest" | "component" | "descriptor" | "verify";
  completedBytes: number;
  totalBytes: number;
}
//#endregion 🪪️ExecutionTargetLease

export interface ArtifactFrontier {
  documentId: string;
  headEditOrdinal: number;
  headEditId: string;
  lastCommitSeq: number;
  chainHash: ArtifactHash;
}

export interface ArtifactBlobRef {
  sha256: ArtifactHash;
  byteLength: number;
  storageKey: string;
}

export interface PublishedArtifactBlob {
  sha256: ArtifactHash;
  byteLength: number;
}

export interface PublishedArtifactCheckpoint {
  scope: DocumentScope;
  checkpointId: CheckpointId;
  parentCheckpointId?: CheckpointId;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  pack: PublishedArtifactBlob;
  spr: PublishedArtifactBlob;
  aggregateSha256: ArtifactHash;
  publishedAtMs: number;
}

export interface ArtifactCheckpoint {
  scope: DocumentScope;
  checkpointId: CheckpointId;
  parentCheckpointId?: CheckpointId;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  pack: ArtifactBlobRef;
  spr: ArtifactBlobRef;
  aggregateSha256: ArtifactHash;
  publishedAtMs: number;
}

export interface ArtifactRetention {
  scope: DocumentScope;
  retainedCheckpointId: CheckpointId;
  retainedFloor: ArtifactFrontier;
  checkpointLineageHead: CheckpointId;
}

export const DESCRIPTOR_DIGEST_V1_DOMAIN = "semio.document-descriptor.digest.v1\0";

function descriptorDigestInteger(value: number, width: 4 | 8, field: string): Uint8Array {
  if (!Number.isSafeInteger(value) || value < 0 || (width === 4 && value > 0xffff_ffff)) throw new Error(`descriptor.invalid-${field}`);
  const output = new Uint8Array(width);
  let remaining = BigInt(value);
  for (let index = width - 1; index >= 0; index--) {
    output[index] = Number(remaining & 0xffn);
    remaining >>= 8n;
  }
  return output;
}

function descriptorDigestHash(value: string, field: string): Uint8Array {
  if (!/^[0-9a-f]{64}$/.test(value) || /^0{64}$/.test(value)) throw new Error(`descriptor.invalid-${field}`);
  return Uint8Array.from({ length: 32 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
}

function descriptorDigestText(value: string, field: string): Uint8Array {
  if (value.length === 0) throw new Error(`descriptor.empty-${field}`);
  return new TextEncoder().encode(value);
}

/** 🧬️ Domain plus declaration-ordered descriptor leaves, each encoded as
 * `u64_be(payload byte length) || payload`; text is UTF-8, integers are unsigned big-endian fixed-
 * width payloads, and hash text is decoded to 32 bytes. JSON serialization never participates. */
export function descriptorDigestEncodingV1(descriptor: DocumentDescriptor): Uint8Array {
  if (descriptor.bootstrapVersion === 0) throw new Error("descriptor.invalid-bootstrap-version");
  if (descriptor.bootstrapFrontier.commitSeq > descriptor.bootstrapFrontier.headSeq) throw new Error("descriptor.invalid-bootstrap-frontier");
  const fields = [
    descriptorDigestText(descriptor.spaceId, "space-id"),
    descriptorDigestText(descriptor.documentId, "document-id"),
    descriptorDigestText(descriptor.artifactKind, "artifact-kind"),
    descriptorDigestText(descriptor.artifactSchema, "artifact-schema"),
    descriptorDigestText(descriptor.owner.pluginId, "owner-plugin-id"),
    descriptorDigestText(descriptor.owner.packageId, "owner-package-id"),
    descriptorDigestText(descriptor.owner.version, "owner-version"),
    descriptorDigestHash(descriptor.owner.packageHash, "owner-package-hash"),
    descriptorDigestHash(descriptor.packSchemaHash, "pack-schema-hash"),
    descriptorDigestInteger(descriptor.bootstrapVersion, 4, "bootstrap-version"),
    descriptorDigestInteger(descriptor.bootstrapFrontier.headSeq, 8, "bootstrap-head-seq"),
    descriptorDigestInteger(descriptor.bootstrapFrontier.commitSeq, 8, "bootstrap-commit-seq"),
    descriptorDigestInteger(descriptor.bootstrapFrontier.epoch, 8, "bootstrap-epoch"),
    descriptorDigestHash(descriptor.bootstrapSnapshotHash, "bootstrap-snapshot-hash"),
  ];
  const domain = new TextEncoder().encode(DESCRIPTOR_DIGEST_V1_DOMAIN);
  const total = fields.reduce((length, field) => length + 8 + field.length, domain.length);
  if (!Number.isSafeInteger(total)) throw new Error("descriptor.length-overflow");
  const output = new Uint8Array(total);
  output.set(domain);
  let offset = domain.length;
  for (const field of fields) {
    output.set(descriptorDigestInteger(field.length, 8, "field-length"), offset);
    offset += 8;
    output.set(field, offset);
    offset += field.length;
  }
  return output;
}

/** 🔐️ Host-Web-Crypto SHA-256 over {@link descriptorDigestEncodingV1}. */
export async function descriptorDigestV1(descriptor: DocumentDescriptor): Promise<Uint8Array> {
  return new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", descriptorDigestEncodingV1(descriptor)));
}

export interface DocumentView {
  descriptor: DocumentDescriptor;
  headSeq: number;
  commitSeq: number;
  epoch: number;
}

export interface InviteView {
  id: string;
  spaceId: string;
  role: DirectorySpaceRole;
  createdAtMs: number;
  expiresAtMs: number;
  revoked: boolean;
}
//#endregion 🔖️Views

//#region 🔖️Stream
export type DirectoryConnectionPhase = "opened" | "closed";

/** 👥️ One live presence actor in a document's roster (Amendment 3 to C1) — the hub knows all four
 * fields without ever decoding the actor's opaque `PresencePeer` bytes. */
export interface DirectoryPresenceActor {
  actor: string;
  userId?: string;
  surface: string;
  color: number;
}

/** 🛟️ Public checkpoint identity that makes a lagged client discard discontinuous live state. */
export interface RebootstrapRequired {
  scope: DocumentScope;
  checkpointId: ArtifactHash;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
}

export type DirectoryStreamMessage =
  | { kind: "event"; event: DirectoryEvent }
  | { kind: "connection"; phase: DirectoryConnectionPhase; connection: ConnectionView }
  | { kind: "presence"; spaceId: string; documentId: string; actors: DirectoryPresenceActor[] }
  | { kind: "heartbeat"; headSeq: number }
  | { kind: "rebootstrap-required"; control: RebootstrapRequired };
//#endregion 🔖️Stream
