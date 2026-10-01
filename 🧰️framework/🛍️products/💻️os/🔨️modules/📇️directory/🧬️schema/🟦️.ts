import {
  type DocumentOpenBrowserActorV1,
  type DocumentExecutionTargetBrowserActorV1,
  parseDocumentOpenBrowserActorV1,
  parseDocumentExecutionTargetBrowserActorV1,
  documentBrowserActorLeaseFromPlanV1,
  sameDocumentBrowserActorV1,
} from "./🌐️browser-actor/🟦️.ts";
/** 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
 * STUDIOS, contract C1) — TypeScript twin of `🦀️.rs`. Pure data, no fold logic (see the
 * module root `../🟦️.ts`'s `DirectoryReadModel`/`fold`).
 *
 * 🧭️ `space.created`'s and `create-space`'s space-kind fields are named `spaceKind`, not
 * contract-freeze.md's bare `kind` — both bodies are tagged on a `kind` discriminator, so a
 * same-named payload field would collide on the wire. Flagged as a `sharedFileRequest` in lane
 * 0-A's report. */

export {
  DOCUMENT_BROWSER_ACTOR_INTERFACES,
  DOCUMENT_BROWSER_ACTOR_MAX_BYTES,
  documentBrowserActorLeaseFromPlanV1,
  parseDocumentExecutionTargetBrowserActorV1,
  parseDocumentOpenBrowserActorV1,
  sameDocumentBrowserActorV1,
} from "./🌐️browser-actor/🟦️.ts";
export type { DocumentBrowserActorSourceV1, DocumentClosedBrowserActorV1, DocumentExecutionTargetBrowserActorV1, DocumentOpenBrowserActorV1 } from "./🌐️browser-actor/🟦️.ts";
import { type EditedArtifactFrontierV1, isEditedArtifactFrontierV1 as editedArtifactFrontierIsValid } from "./📌️document-check-in-v1/🟦️.ts";
export {
  DOCUMENT_CHECK_IN_MAX_BYTES,
  DOCUMENT_CHECK_IN_SCHEMA_V1,
  DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1,
  documentCheckInCanonicalJson,
  isEditedArtifactFrontierV1,
  isTerminalDocumentCheckInPhaseV1,
  parseDocumentCheckInStatusV1,
  parseDocumentCheckInV1,
} from "./📌️document-check-in-v1/🟦️.ts";
export type {
  DocumentCheckInPhaseV1,
  DocumentCheckInReadyV1,
  DocumentCheckInRefusalV1,
  DocumentCheckInStatusV1,
  DocumentCheckInV1,
  EditedArtifactFrontierV1,
} from "./📌️document-check-in-v1/🟦️.ts";

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

/** 🧊️ Atomically freezes the space and demotes every current Author membership to Spectator. */
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

/** 🏷️ Directory-owned display metadata, not executable descriptor authority. */
export interface DocumentIndexEntryV1 {
  name: string;
  dialect: { artifactKind: string; standard: string; subset: string };
}

/** 🧾️ One descriptor-bound presentation row derived from ordered Directory events. */
export interface DirectoryIndexedDocumentViewV1 {
  descriptor: DocumentDescriptor;
  descriptorDigestV1: ArtifactHash;
  entry: DocumentIndexEntryV1;
  createdAtMs: number;
  createdBy: string;
}

export interface DirectoryEventDocumentIndexed {
  kind: "document.indexed";
  scope: DocumentScope;
  descriptorDigestV1: ArtifactHash;
  entry: DocumentIndexEntryV1;
}

/** 🌟 The dialect grammar's any-subset coordinate, mirrored from `io_schema::SubsetId::ANY` and from
 * this law's Rust twin (`📇️document-index-v1/🦀️.rs`). */
export const DOCUMENT_INDEX_ANY_SUBSET_V1 = "*";

/** 🛡️ Validates the bounded presentation shape without authorizing document execution.
 *
 * 🌟 `subset` additionally admits the ONE canonical any-subset coordinate (`*`): it is the dialect
 * grammar's own wildcard, the value every dialect that declares no narrowed subset carries —
 * including the `parentDialect` the Directory's own document-open admission returns — so rejecting it
 * drops the presentation row of practically every indexed document, and with it the whole event page
 * that row rides on. It stays a bounded, non-executable literal: exactly `*`, never a pattern
 * embedded in a longer identity. The Rust twin has always said so; this side had not.
 *
 * 🪢 `dialect.artifactKind` is the owning app's `Dialect` coordinate (an installed artifact coordinate),
 * NOT the manifest `ArtifactKindSpec.id` its descriptor carries (`2d.note`, `stdio.json`) — two id
 * spaces that coincide for `gis` alone — so it is bounded by its own canonical grammar and never
 * compared to the descriptor. The plugin segment is compared to nothing: `demonstrator` ships apps
 * whose `Dialect` belongs to another plugin. */
export function validDocumentIndexEntryV1(entry: DocumentIndexEntryV1): boolean {
  const identity = (value: unknown): boolean => typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value);
  const subsetIdentity = (value: unknown): boolean => value === DOCUMENT_INDEX_ANY_SUBSET_V1 || identity(value);
  const canonicalDialectArtifactKind = (value: unknown): boolean => typeof value === "string" && value.length <= 256 && /^s(\.[a-z0-9]+(-[a-z0-9]+)*){1,2}$/u.test(value);
  return typeof entry.name === "string" && entry.name.length > 0 && [...entry.name].length <= 128 && !entry.name.startsWith(" ") && !entry.name.endsWith(" ") && !/[\u0000-\u001f\u007f-\u009f\uD800-\uDFFF]/u.test(entry.name)
    && canonicalDialectArtifactKind(entry.dialect.artifactKind) && identity(entry.dialect.standard) && subsetIdentity(entry.dialect.subset);
}

export interface DirectoryEventArtifactCheckpointPublished {
  kind: "artifact.checkpoint-published";
  checkpoint: PublishedArtifactCheckpoint;
}

export interface DirectoryEventArtifactRetentionAdvanced {
  kind: "artifact.retention-advanced";
  retention: ArtifactRetention;
}

/** 🎚️ One preference change of ONE user (`schema` names the vocabulary, `mutation` is its canonical JSON text — the hub
 * never reads it). Only that user sees it, and only on the preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): the
 * default page and the Home fold never carry it, so a guest folding directory pages never meets a kind it does not
 * know (ticket 26/09/23 U5). */
export interface DirectoryEventUserPreferenceRecorded {
  kind: "user.preference-recorded";
  userId: string;
  schema: string;
  mutation: string;
}

/** 🎚️ The preference lane's page route: the principal's own `user.preference-recorded` events on the page machinery of
 * `/directory/event-page/v1`, nothing else. */
export const DIRECTORY_PREFERENCE_PAGE_PATH_V1 = "/directory/preference-page/v1";
export const USER_PREFERENCE_SCHEMA_ID_MAX_BYTES = 128;
export const USER_PREFERENCE_MUTATION_MAX_BYTES = 4096;

/** 🛡️ A preference vocabulary id (`os.config.ui-preferences.v1`) and one JSON object text within its bound — the Rust
 * twin's `valid_user_preference_record_v1`. The hub never reads the object; the vocabulary's own clients do. */
export function validUserPreferenceRecordV1(schema: unknown, mutation: unknown): boolean {
  if (typeof schema !== "string" || schema.length === 0 || new TextEncoder().encode(schema).length > USER_PREFERENCE_SCHEMA_ID_MAX_BYTES || !/^[a-z0-9][a-z0-9.-]*[a-z0-9]$/u.test(schema)) return false;
  if (typeof mutation !== "string" || mutation.length < 2 || new TextEncoder().encode(mutation).length > USER_PREFERENCE_MUTATION_MAX_BYTES) return false;
  try {
    const value: unknown = JSON.parse(mutation);
    return value !== null && typeof value === "object" && !Array.isArray(value);
  } catch {
    return false;
  }
}

export type DirectoryEventBody =
  | DirectoryEventUserPreferenceRecorded
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
  | DirectoryEventDocumentIndexed
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
  if (body.kind === "document.indexed") {
    scope(body.scope);
    hash(body.descriptorDigestV1);
    if (!(body.descriptorDigestV1 as number[]).some((byte) => byte !== 0)) throw new Error("directory-event-page.invalid-index-digest");
    const entry = exact(body.entry, ["name", "dialect"]);
    exact(entry.dialect, ["artifactKind", "standard", "subset"]);
    if (!validDocumentIndexEntryV1(entry as unknown as DocumentIndexEntryV1)) throw new Error("directory-event-page.invalid-index");
  }
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

/** 📇️ Validates one directory event outside a page (the space index lane's events), by the page's own event rules. */
export function parseDirectoryEventV1(value: unknown): DirectoryEvent {
  return directoryEventPageEvent(value);
}

function directoryEventPageEvent(value: unknown): DirectoryEvent {
  const event = directoryEventPageObject(value, ["seq", "id", "hlc", "actor", "body", "recordedAtMs"], ["spaceId", "userId"]);
  directoryEventPageInteger(event.seq, true);
  if (
    typeof event.id !== "string" ||
    typeof event.recordedAtMs !== "number" ||
    !Number.isSafeInteger(event.recordedAtMs) ||
    (event.spaceId !== undefined && typeof event.spaceId !== "string") ||
    (event.userId !== undefined && typeof event.userId !== "string")
  )
    throw new Error("directory-event-page.invalid-event");
  directoryEventPageInteger(directoryEventPageObject(event.hlc, ["physicalMs", "logical"]).logical);
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
    "document.indexed": ["kind", "scope", "descriptorDigestV1", "entry"],
    "artifact.checkpoint-published": ["kind", "checkpoint"],
    "artifact.retention-advanced": ["kind", "retention"],
    "user.preference-recorded": ["kind", "userId", "schema", "mutation"],
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
    "user.preference-recorded": ["userId", "schema", "mutation"],
  };
  if ((textFields[body.kind as string] ?? []).some((field) => typeof body[field] !== "string")) throw new Error("directory-event-page.invalid-event-text");
  if (body.kind === "user.preference-recorded" && (!validUserPreferenceRecordV1(body.schema, body.mutation) || event.spaceId !== undefined || event.userId !== body.userId || (body.userId as string).length === 0)) throw new Error("directory-event-page.invalid-user-preference");
  if (
    (body.kind === "space.created" && body.spaceKind !== "atelier" && body.spaceKind !== "studio" && body.spaceKind !== "archive") ||
    ((body.kind === "space.created" || body.kind === "space.visibility-changed") && body.visibility !== "private" && body.visibility !== "public") ||
    ((body.kind === "member.upserted" || body.kind === "invite.redeemed") && body.role !== "author" && body.role !== "spectator")
  )
    throw new Error("directory-event-page.invalid-event-vocabulary");
  directoryEventPageNestedShapes(body);
  if (body.kind === "document.indexed" && (event.spaceId !== (body.scope as DocumentScope).spaceId || typeof event.userId !== "string" || event.userId.length === 0 || new TextEncoder().encode(event.userId).length > 256)) throw new Error("directory-event-page.invalid-index-author");
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
  if ((await directoryEventPageSha256(JSON.stringify(unsigned))) !== receiptSha256) throw new Error("directory-event-page.receipt-mismatch");
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
  | { kind: "announce-document"; descriptor: DocumentDescriptor }
  | { kind: "record-user-preference"; schema: string; mutation: string };
//#endregion 🔖️Command


//#region 🌊️EditedArtifactFrontier
/** 🌊️ Reads one edited ledger point in its exact wire grammar; throws for anything else. */
function editedArtifactFrontier(value: unknown): EditedArtifactFrontierV1 {
  if (!editedArtifactFrontierIsValid(value)) throw new Error("edited-artifact-frontier.invalid");
  return { documentId: value.documentId, headEditOrdinal: value.headEditOrdinal, headEditId: value.headEditId, lastCommitSeq: value.lastCommitSeq, chainSha256: value.chainSha256 };
}
//#endregion 🌊️EditedArtifactFrontier

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

/** 🧾️ Every member of {@link DirectoryCommandOutcomeV1}, in wire order — the one place the vocabulary is enumerated. */
export const DIRECTORY_COMMAND_OUTCOMES_V1: readonly DirectoryCommandOutcomeV1[] = ["accepted", "previously-accepted", "secret-undeliverable"];

/** 🧾️ Narrows an untyped disposition to an owned one, refusing anything the vocabulary does not name. */
export function parseDirectoryCommandOutcomeV1(value: string): DirectoryCommandOutcomeV1 {
  const outcome = DIRECTORY_COMMAND_OUTCOMES_V1.find((member) => member === value);
  if (outcome === undefined) throw new Error("directory command outcome: unowned");
  return outcome;
}

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
export type DirectoryCommandErrorCodeV1 = "unauthorized" | "forbidden" | "stale-session" | "request-conflict" | "invalid" | "overloaded" | "too-large" | "capacity" | "closed" | "cancelled" | "transport";

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
  "record-user-preference": ["kind", "schema", "mutation"],
};

function directoryCommandRequestId(value: unknown): string {
  if (typeof value !== "string" || value.length !== DIRECTORY_COMMAND_REQUEST_ID_LEN || !/^[0-9a-f]+$/u.test(value) || /^0+$/u.test(value)) throw new Error("directory-command.invalid-request-id");
  return value;
}

/** 🧭 Reconstructs one closed command in declaration order after an order-independent carrier. */
export function canonicalDirectoryCommandV1(value: unknown): DirectoryCommand {
  const command = directoryEventPageObject(value, ["kind"], Object.values(DIRECTORY_COMMAND_FIELDS).flat());
  const fields = typeof command.kind === "string" ? DIRECTORY_COMMAND_FIELDS[command.kind] : undefined;
  if (!fields) throw new Error("directory-command.invalid-kind");
  directoryEventPageObject(command, fields);
  const canonical: Record<string, unknown> = {};
  for (const field of fields) canonical[field] = command[field];
  if (directoryEventPageHasControl(command)) throw new Error("directory-command.control-character");
  const text = (field: string): string => {
    const value = command[field];
    if (typeof value !== "string") throw new Error(`directory-command.invalid-${field}`);
    return value;
  };
  const role = (value: unknown): DirectorySpaceRole => {
    if (value !== "author" && value !== "spectator") throw new Error("directory-command.invalid-role");
    return value;
  };
  const visibility = (value: unknown): DirectorySpaceVisibility => {
    if (value !== "private" && value !== "public") throw new Error("directory-command.invalid-visibility");
    return value;
  };
  switch (command.kind) {
    case "create-space":
      text("name");
      if (command.spaceKind !== "atelier" && command.spaceKind !== "studio" && command.spaceKind !== "archive") throw new Error("directory-command.invalid-space-kind");
      visibility(command.visibility);
      break;
    case "rename-space": text("spaceId"); text("name"); break;
    case "set-visibility": text("spaceId"); visibility(command.visibility); break;
    case "archive-space":
    case "delete-space": text("spaceId"); break;
    case "upsert-member": text("spaceId"); text("email"); role(command.role); break;
    case "remove-member": text("spaceId"); text("userId"); break;
    case "create-invite": text("spaceId"); role(command.role); directoryEventPageInteger(command.ttlSecs, true); break;
    case "revoke-invite": text("spaceId"); text("inviteId"); break;
    case "record-user-preference": if (!validUserPreferenceRecordV1(command.schema, command.mutation)) throw new Error("directory-command.invalid-user-preference"); break;
    case "announce-document": {
      directoryEventPageNestedShapes({ kind: "document.announced", descriptor: command.descriptor });
      const descriptor = command.descriptor as Record<string, unknown>;
      const owner = descriptor.owner as Record<string, unknown>;
      const frontier = descriptor.bootstrapFrontier as Record<string, unknown>;
      for (const [field, candidate] of [
        ["spaceId", descriptor.spaceId], ["documentId", descriptor.documentId], ["artifactKind", descriptor.artifactKind], ["artifactSchema", descriptor.artifactSchema],
        ["owner.pluginId", owner.pluginId], ["owner.packageId", owner.packageId], ["owner.version", owner.version],
      ] as const) if (typeof candidate !== "string" || candidate.trim().length === 0) throw new Error(`directory-command.invalid-${field}`);
      for (const [field, candidate] of [["owner.packageHash", owner.packageHash], ["packSchemaHash", descriptor.packSchemaHash], ["bootstrapSnapshotHash", descriptor.bootstrapSnapshotHash]] as const) {
        if (typeof candidate !== "string" || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(candidate)) throw new Error(`directory-command.invalid-${field}`);
      }
      if ((frontier.commitSeq as number) > (frontier.headSeq as number)) throw new Error("directory-command.invalid-bootstrap-frontier");
      canonical.descriptor = {
        spaceId: descriptor.spaceId,
        documentId: descriptor.documentId,
        artifactKind: descriptor.artifactKind,
        artifactSchema: descriptor.artifactSchema,
        owner: { pluginId: owner.pluginId, packageId: owner.packageId, version: owner.version, packageHash: owner.packageHash },
        packSchemaHash: descriptor.packSchemaHash,
        bootstrapVersion: descriptor.bootstrapVersion,
        bootstrapFrontier: { headSeq: frontier.headSeq, commitSeq: frontier.commitSeq, epoch: frontier.epoch },
        bootstrapSnapshotHash: descriptor.bootstrapSnapshotHash,
      };
      break;
    }
  }
  return canonical as unknown as DirectoryCommand;
}

/** 🛡️ Decodes one declaration-ordered canonical command and validates every scalar. */
export function parseDirectoryCommandV1(value: unknown): DirectoryCommand {
  const canonical = canonicalDirectoryCommandV1(value);
  if (JSON.stringify(canonical) !== JSON.stringify(value)) throw new Error("directory-command.noncanonical-command");
  return canonical;
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
  return { schema: "semio.directory.command-request.v1", requestId: directoryCommandRequestId(requestId), command: parseDirectoryCommandV1(command) };
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
  const request: DirectoryCommandRequestV1 = { schema: object.schema, requestId: directoryCommandRequestId(object.requestId), command: parseDirectoryCommandV1(object.command) };
  if (JSON.stringify(request) !== source) throw new Error("directory-command.noncanonical");
  return request;
}

/** 🔐️ Seals one completion by hashing its declaration-ordered unsigned canonical JSON — the exact
 * twin of the Rust `DirectoryCommandReceiptV1::seal`. */
export async function sealDirectoryCommandReceiptV1(requestId: string, commandSha256: string, outcome: DirectoryCommandOutcomeV1, events: readonly DirectoryEvent[], result: DirectoryCommandResultV1): Promise<DirectoryCommandReceiptV1> {
  const unsigned = { schema: "semio.directory.command-receipt.v1" as const, requestId: directoryCommandRequestId(requestId), commandSha256: directoryEventPageHash(commandSha256, false), outcome, events: [...events], result };
  return { ...unsigned, receiptSha256: await directoryEventPageSha256(JSON.stringify(unsigned)) };
}

/** 📥️ Parses exactly one canonical receipt bound to the request that asked for it. */
export async function parseDirectoryCommandReceiptV1(source: string, request: DirectoryCommandRequestV1): Promise<DirectoryCommandReceiptV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES) throw new Error("directory-command.receipt-too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "requestId", "commandSha256", "outcome", "events", "result", "receiptSha256"]);
  if (
    object.schema !== "semio.directory.command-receipt.v1" ||
    (object.outcome !== "accepted" && object.outcome !== "previously-accepted" && object.outcome !== "secret-undeliverable") ||
    !Array.isArray(object.events) ||
    object.events.length > DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS
  )
    throw new Error("directory-command.invalid-envelope");
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
  if (receipt.requestId !== request.requestId || receipt.commandSha256 !== (await directoryCommandSha256(request.command))) throw new Error("directory-command.request-mismatch");
  if (JSON.stringify(receipt) !== source) throw new Error("directory-command.noncanonical");
  const { receiptSha256, ...unsigned } = receipt;
  if ((await directoryEventPageSha256(JSON.stringify(unsigned))) !== receiptSha256) throw new Error("directory-command.receipt-mismatch");
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

export type AdminIntentStateV1 = "succeeded" | "accepted" | "indeterminate" | "failed" | "cancelled";

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

export type DirectorySpaceListEntryV1 = { access: "public"; space: PublicSpaceViewV1 } | { access: "member"; space: MemberSpaceViewV1 } | { access: "author"; space: MemberSpaceViewV1 };

export const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS = 64;
export const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES = 48 * 1024;
export const DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES = 1024;
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

/** 🛂️ Binds an administration command to its verified page and declared capability, before sealing. */
export function directoryAdministrationCommandAllowedV1(page: DirectorySpaceAdministrationPageV1 | null, spaceId: string, command: unknown): boolean {
  if (page?.access !== "author" || page.spaceId !== spaceId || page.space.id !== spaceId) return false;
  let canonical: DirectoryCommand;
  try { canonical = parseDirectoryCommandV1(command); } catch { return false; }
  if (!("spaceId" in canonical) || canonical.spaceId !== spaceId) return false;
  switch (canonical.kind) {
    case "rename-space": return page.capabilities.renameSpace === true;
    case "set-visibility": return page.capabilities.setVisibility === true;
    case "delete-space": return page.capabilities.deleteSpace === true;
    case "upsert-member": return page.capabilities.upsertMember === true;
    case "remove-member": return page.capabilities.removeMember === true;
    case "create-invite": return page.capabilities.createInvite === true;
    case "revoke-invite": return page.capabilities.revokeInvite === true;
    default: return false;
  }
}

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
  const shape =
    access === "public"
      ? [...common, "documents", "receiptSha256"]
      : access === "member"
        ? [...common, "members", "documents", "receiptSha256"]
        : access === "author"
          ? [...common, "members", "documents", "invites", "capabilities", "receiptSha256"]
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
  const page = (access === "public"
    ? { ...base, documents, receiptSha256 }
    : access === "member"
      ? { ...base, members, documents, receiptSha256 }
      : { ...base, members, documents, invites, capabilities, receiptSha256 }) as unknown as DirectorySpaceAdministrationPageV1;
  if (JSON.stringify(page) !== source) throw new Error("space-administration-page.noncanonical");
  const { receiptSha256: _receipt, ...unsigned } = page as Record<string, unknown> & { receiptSha256: string };
  if ((await administrationSha256(JSON.stringify(unsigned))) !== receiptSha256) throw new Error("space-administration-page.receipt-mismatch");
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

export const DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 = 20;

export interface DocumentExecutionProtocolV1 {
  appChannelVersion: typeof DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1;
}

export interface DocumentOpenPackageV1 {
  pluginId: string;
  packageId: string;
  version: string;
  componentSha256: string;
  componentBlake3: string;
  descriptorByteSha256: string;
  executionProtocol: DocumentExecutionProtocolV1;
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
  browserActor: DocumentOpenBrowserActorV1;
  grant: DocumentOpenGrantV1;
  checkpoint: DocumentOpenCheckpointV1;
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
  // 🩺️ The refusal names the keys it refused on. This check is the closed-object gate for the plan
  // and for every object nested inside it, and a bare code left a caller unable to tell an added
  // field from a missing one, in which object — including the common case where the body is not a
  // plan at all but a two-field `document-open-plan-error/v1`, whose `code` is the real answer.
  const unexpected = Object.keys(object).filter((key) => !accepted.has(key));
  const missing = required.filter((key) => !(key in object));
  if (unexpected.length !== 0 || missing.length !== 0) throw new Error(`document-open.invalid-fields unexpected=[${unexpected.join(",")}] missing=[${missing.join(",")}] accepted=[${[...accepted].join(",")}]`);
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

function parseDocumentOpenFrontier(value: unknown, scope: DocumentScope): ArtifactFrontier {
  const object = documentOpenObject(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
  const chainHash = object.chainHash;
  if (!Array.isArray(chainHash) || chainHash.length !== 32 || chainHash.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255))
    throw new Error("document-open.invalid-frontier-hash");
  const frontier = {
    documentId: documentOpenText(object.documentId),
    headEditOrdinal: documentOpenInteger(object.headEditOrdinal),
    headEditId: object.headEditId === "" ? "" : documentOpenText(object.headEditId),
    lastCommitSeq: documentOpenInteger(object.lastCommitSeq),
    chainHash: chainHash as number[],
  };
  if (!(artifactFrontierIsGenesisForV1(scope, frontier) || artifactFrontierIsEditedForV1(scope, frontier)) || frontier.lastCommitSeq > frontier.headEditOrdinal) throw new Error("document-open.stale-frontier");
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
  const object = documentOpenObject(value, ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "parentDialect", "surface", "browserActor", "grant", "checkpoint", "revalidation"]);
  if (object.schema !== "semio.hub.document-open-plan/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const packageValue = documentOpenObject(object.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256", "executionProtocol"]);
  const executionProtocol = documentOpenObject(packageValue.executionProtocol, ["appChannelVersion"]);
  if (executionProtocol.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error("document-open.unsupported-execution-protocol");
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const parentDialect = documentOpenObject(object.parentDialect, ["artifactKind", "standard", "subset"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  const expiresAtUnixMs = documentOpenInteger(object.expiresAtUnixMs, true);
  if (expiresAtUnixMs <= nowMs || expiresAtUnixMs - nowMs > DOCUMENT_OPEN_PLAN_MAX_TTL_MS || (revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined))
    throw new Error("document-open.expired-or-ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-open.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor"))
    throw new Error("document-open.invalid-surface");
  const parsedParentDialect = {
    artifactKind: documentOpenText(parentDialect.artifactKind),
    standard: documentOpenText(parentDialect.standard),
    subset: documentOpenText(parentDialect.subset),
  };
  if (Object.values(parsedParentDialect).some((value) => value.trim() !== value)) throw new Error("document-open.invalid-parent-dialect");
  const checkpoint = documentOpenObject(object.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  if (checkpoint.descriptorDigestV1 !== descriptorDigestV1) throw new Error("document-open.stale-checkpoint");
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
      executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 },
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
    browserActor: parseDocumentOpenBrowserActorV1(object.browserActor, { componentSha256: documentOpenHash(packageValue.componentSha256), descriptorByteSha256: documentOpenHash(packageValue.descriptorByteSha256) }, surface.rendererTarget),
    grant: { read: true, write: grant.write, observe: true },
    checkpoint: {
      checkpointId: documentOpenHash(checkpoint.checkpointId),
      descriptorDigestV1,
      baselineFrontier: parseDocumentOpenFrontier(checkpoint.baselineFrontier, scope),
      aggregateSha256: documentOpenHash(checkpoint.aggregateSha256),
    },
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
  browserActor: DocumentExecutionTargetBrowserActorV1;
  artifact: DocumentOpenArtifactV1;
  parentDialect: DocumentOpenParentDialectV1;
  surface: DocumentOpenSurfaceV1;
  grant: DocumentOpenGrantV1;
  checkpoint: DocumentOpenCheckpointV1;
  revalidation: DocumentOpenRevalidationV1;
}

function documentExecutionTargetByteLength(value: unknown, maxBytes: number): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 1 || value > maxBytes) throw new Error("document-execution-target-lease.invalid-byte-length");
  return value;
}

/** ✅️ Strictly parses the public lease fields and enforces every byte/identity invariant: both
 * component digests and the descriptor digest must equal the package projection, the parent dialect
 * must share the artifact kind, the grant must follow the surface role, and the required checkpoint
 * must carry the same descriptor digest. */
export function parseDocumentExecutionTargetLeaseFieldsV1(value: unknown): DocumentExecutionTargetLeaseFieldsV1 {
  const object = documentOpenObject(value, ["schema", "version", "scope", "descriptorDigestV1", "catalog", "package", "component", "descriptor", "browserActor", "artifact", "parentDialect", "surface", "grant", "checkpoint", "revalidation"]);
  if (object.schema !== "semio.os.document-execution-target-lease/v1" || object.version !== 1) throw new Error("document-execution-target-lease.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const packageValue = documentOpenObject(object.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256", "executionProtocol"]);
  const executionProtocol = documentOpenObject(packageValue.executionProtocol, ["appChannelVersion"]);
  if (executionProtocol.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error("document-execution-target-lease.unsupported-execution-protocol");
  const component = documentOpenObject(object.component, ["sha256", "blake3", "byteLength"]);
  const descriptor = documentOpenObject(object.descriptor, ["sha256", "byteLength"]);
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const parentDialect = documentOpenObject(object.parentDialect, ["artifactKind", "standard", "subset"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  if ((revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("document-execution-target-lease.ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-execution-target-lease.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor"))
    throw new Error("document-execution-target-lease.invalid-surface");
  const parsedParentDialect = {
    artifactKind: documentOpenText(parentDialect.artifactKind),
    standard: documentOpenText(parentDialect.standard),
    subset: documentOpenText(parentDialect.subset),
  };
  if (Object.values(parsedParentDialect).some((entry) => entry.trim() !== entry)) throw new Error("document-execution-target-lease.invalid-parent-dialect");
  const parsedPackage: DocumentOpenPackageV1 = {
    pluginId: documentOpenText(packageValue.pluginId),
    packageId: documentOpenText(packageValue.packageId),
    version: documentOpenText(packageValue.version),
    componentSha256: documentOpenHash(packageValue.componentSha256),
    componentBlake3: documentOpenHash(packageValue.componentBlake3),
    descriptorByteSha256: documentOpenHash(packageValue.descriptorByteSha256),
    executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 },
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
  if (parsedComponent.sha256 !== parsedPackage.componentSha256 || parsedComponent.blake3 !== parsedPackage.componentBlake3 || parsedDescriptor.sha256 !== parsedPackage.descriptorByteSha256)
    throw new Error("document-execution-target-lease.unbound-bytes");
  const checkpoint = documentOpenObject(object.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  if (checkpoint.descriptorDigestV1 !== descriptorDigestV1) throw new Error("document-execution-target-lease.stale-checkpoint");
  return {
    schema: object.schema,
    version: object.version,
    scope,
    descriptorDigestV1,
    catalog: { generationId: documentOpenHash(catalog.generationId) },
    package: parsedPackage,
    component: parsedComponent,
    descriptor: parsedDescriptor,
    browserActor: parseDocumentExecutionTargetBrowserActorV1(object.browserActor, parsedPackage, surface.rendererTarget),
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
    checkpoint: {
      checkpointId: documentOpenHash(checkpoint.checkpointId),
      descriptorDigestV1,
      baselineFrontier: parseDocumentOpenFrontier(checkpoint.baselineFrontier, scope),
      aggregateSha256: documentOpenHash(checkpoint.aggregateSha256),
    },
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
export function leaseFieldsFromPlanV1(plan: DocumentOpenPlanV1, byteLengths: { readonly component: number; readonly descriptor: number; readonly browserActor?: number }): DocumentExecutionTargetLeaseFieldsV1 {
  return parseDocumentExecutionTargetLeaseFieldsV1({
    schema: "semio.os.document-execution-target-lease/v1",
    version: 1,
    scope: plan.scope,
    descriptorDigestV1: plan.descriptorDigestV1,
    catalog: plan.catalog,
    package: plan.package,
    component: { sha256: plan.package.componentSha256, blake3: plan.package.componentBlake3, byteLength: byteLengths.component },
    descriptor: { sha256: plan.package.descriptorByteSha256, byteLength: byteLengths.descriptor },
    browserActor: documentBrowserActorLeaseFromPlanV1(plan.browserActor, plan.package, plan.surface.rendererTarget, byteLengths.browserActor),
    artifact: plan.artifact,
    parentDialect: plan.parentDialect,
    surface: plan.surface,
    grant: plan.grant,
    checkpoint: plan.checkpoint,
    revalidation: plan.revalidation,
  });
}

/** 🗂️ One artifact kind a descriptor declares, reduced to the pair a document open is admitted on. */
export type SurfaceArtifactKindV1 = { readonly id: string; readonly schema: string };

/** 🎯️ One surface app of a verified descriptor as the pairing rule reads it: its role, its full dialect coordinate, the
 * artifact kinds it declares itself and the kind its io presents (`null` when it presents none). */
export type SurfaceKindAppV1 = {
  readonly role: "editor" | "viewer";
  readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string };
  readonly artifactKinds: readonly SurfaceArtifactKindV1[];
  readonly presents: SurfaceArtifactKindV1 | null;
};

/** 🗂️ The one surface ↔ artifact-kind pairing rule, the browser twin of the hub's `app_opens_kind`
 * (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`) that publishes and verifies every open target: a plugin-level
 * kind (`PluginBuilder::artifact_kind`) is opened only by the surfaces whose own dialect names it, even when a sibling app
 * lists it as an input; a HOSTED kind (`hostedArtifactKinds`) by the surfaces whose dialect names it or whose io presents it
 * and by the viewers of a presenting editor's exact dialect; any other kind is opened by the editor that declares it itself
 * (a plugin on the declaration tree
 * stitches its spec onto the app) and by the viewers of that editor's dialect — the read-only surface of the same
 * documents. Replayed from `🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json` and from the hub's own
 * `🔏️trusted-catalog/🧫️fixtures/🎯️descriptor-open-targets/🔣️.json`.
 *
 * 🧯️ The browser used to admit on the plugin-level list alone, so every document of a plugin migrated onto the
 * declaration tree (note, draw, writer, puzzle, …: `manifest.artifactKinds = []`) was refused after its
 * verified download with "The document component could not be verified" (measured inside `s` against hub 7800,
 * ticket 26/09/23 S15). It then still refused every viewer the hub issued (a viewer declares no kinds of its own), so a
 * Spectator's open ended in "The document target changed" and an unmounted viewer (ticket 26/09/23 C13, row 3.4). */
export function surfaceOpensArtifactKindV1(
  pluginArtifactKinds: readonly SurfaceArtifactKindV1[],
  hostedArtifactKinds: readonly SurfaceArtifactKindV1[],
  apps: readonly SurfaceKindAppV1[],
  app: SurfaceKindAppV1,
  artifact: { readonly kind: string; readonly schema: string },
): boolean {
  const declares = (kinds: readonly SurfaceArtifactKindV1[]): boolean => kinds.some((kind) => kind.id === artifact.kind && kind.schema === artifact.schema);
  if (declares(pluginArtifactKinds)) return app.dialect.artifactKind === artifact.kind;
  if (declares(hostedArtifactKinds)) {
    const presents = (candidate: SurfaceKindAppV1): boolean => candidate.presents !== null && candidate.presents.id === artifact.kind && candidate.presents.schema === artifact.schema;
    return (
      app.dialect.artifactKind === artifact.kind ||
      presents(app) ||
      (app.role === "viewer" &&
        apps.some(
          (editor) =>
            editor.role === "editor" &&
            editor.dialect.artifactKind === app.dialect.artifactKind &&
            editor.dialect.standard === app.dialect.standard &&
            editor.dialect.subset === app.dialect.subset &&
            presents(editor),
        ))
    );
  }
  return (
    declares(app.artifactKinds) ||
    (app.role === "viewer" &&
      apps.some(
        (editor) =>
          editor.role === "editor" &&
          editor.dialect.artifactKind === app.dialect.artifactKind &&
          editor.dialect.standard === app.dialect.standard &&
          editor.dialect.subset === app.dialect.subset &&
          declares(editor.artifactKinds),
      ))
  );
}

/** ⚖️ The one shared full-field lease relation. Every transport compares every field through it; a
 * browser or native subset comparison is never permitted. */
export function sameLeaseFieldsV1(left: DocumentExecutionTargetLeaseFieldsV1, right: DocumentExecutionTargetLeaseFieldsV1): boolean {
  const sameCheckpoint =
    left.checkpoint === undefined || right.checkpoint === undefined
      ? left.checkpoint === right.checkpoint
      : left.checkpoint.checkpointId === right.checkpoint.checkpointId &&
        left.checkpoint.descriptorDigestV1 === right.checkpoint.descriptorDigestV1 &&
        left.checkpoint.aggregateSha256 === right.checkpoint.aggregateSha256 &&
        left.checkpoint.baselineFrontier.documentId === right.checkpoint.baselineFrontier.documentId &&
        left.checkpoint.baselineFrontier.headEditOrdinal === right.checkpoint.baselineFrontier.headEditOrdinal &&
        left.checkpoint.baselineFrontier.headEditId === right.checkpoint.baselineFrontier.headEditId &&
        left.checkpoint.baselineFrontier.lastCommitSeq === right.checkpoint.baselineFrontier.lastCommitSeq &&
        left.checkpoint.baselineFrontier.chainHash.length === right.checkpoint.baselineFrontier.chainHash.length &&
        left.checkpoint.baselineFrontier.chainHash.every((byte, index) => byte === right.checkpoint!.baselineFrontier.chainHash[index]);
  return (
    left.schema === right.schema &&
    left.version === right.version &&
    left.scope.spaceId === right.scope.spaceId &&
    left.scope.documentId === right.scope.documentId &&
    left.descriptorDigestV1 === right.descriptorDigestV1 &&
    left.catalog.generationId === right.catalog.generationId &&
    left.package.pluginId === right.package.pluginId &&
    left.package.packageId === right.package.packageId &&
    left.package.version === right.package.version &&
    left.package.componentSha256 === right.package.componentSha256 &&
    left.package.componentBlake3 === right.package.componentBlake3 &&
    left.package.descriptorByteSha256 === right.package.descriptorByteSha256 &&
    left.package.executionProtocol.appChannelVersion === right.package.executionProtocol.appChannelVersion &&
    left.component.sha256 === right.component.sha256 &&
    left.component.blake3 === right.component.blake3 &&
    left.component.byteLength === right.component.byteLength &&
    left.descriptor.sha256 === right.descriptor.sha256 &&
    left.descriptor.byteLength === right.descriptor.byteLength &&
    sameDocumentBrowserActorV1(left.browserActor, right.browserActor) &&
    left.artifact.kind === right.artifact.kind &&
    left.artifact.schema === right.artifact.schema &&
    left.artifact.packSchemaHash === right.artifact.packSchemaHash &&
    left.parentDialect.artifactKind === right.parentDialect.artifactKind &&
    left.parentDialect.standard === right.parentDialect.standard &&
    left.parentDialect.subset === right.parentDialect.subset &&
    left.surface.surfaceId === right.surface.surfaceId &&
    left.surface.appId === right.surface.appId &&
    left.surface.windowKindId === right.surface.windowKindId &&
    left.surface.role === right.surface.role &&
    left.surface.rendererTarget === right.surface.rendererTarget &&
    left.grant.read === right.grant.read &&
    left.grant.write === right.grant.write &&
    left.grant.observe === right.grant.observe &&
    sameCheckpoint &&
    left.revalidation.directoryRevision === right.revalidation.directoryRevision &&
    left.revalidation.membershipGeneration === right.revalidation.membershipGeneration &&
    left.revalidation.sessionGeneration === right.revalidation.sessionGeneration &&
    left.revalidation.shareGeneration === right.revalidation.shareGeneration
  );
}

/** ⏯️ Whether `next` — the fields a reconnect's fresh plan projects — names the SAME execution target under the SAME grant as `current`,
 * the lease of a child a link shortage suspended, so that child resumes instead of reopening. Two fields legitimately advance while a
 * document stays open and are adopted, only ever forward: the active `checkpoint` (the hub cut a newer one; the child is already
 * past it) and the `revalidation` witness (its `directoryRevision` is the hub directory's global head, which any directory event on
 * the hub moves). Every other field goes through {@link sameLeaseFieldsV1} — the full-field relation plus an explicit monotone
 * freshness rule, never a subset comparison. Before, a resume demanded the old witness verbatim, so on a shared hub every short
 * link cut reopened the document (ticket 26/09/23 C12, run `c12short5`).
 * @see ../../../🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json */
export function sameExecutionTargetV1(current: DocumentExecutionTargetLeaseFieldsV1, next: DocumentExecutionTargetLeaseFieldsV1): boolean {
  const forward = (from: number | undefined, to: number | undefined): boolean => (from === undefined ? to === undefined : to !== undefined && to >= from);
  const checkpointForward =
    current.checkpoint === undefined || next.checkpoint === undefined
      ? current.checkpoint === next.checkpoint
      : next.checkpoint.descriptorDigestV1 === current.checkpoint.descriptorDigestV1 &&
        next.checkpoint.baselineFrontier.documentId === current.checkpoint.baselineFrontier.documentId &&
        forward(current.checkpoint.baselineFrontier.headEditOrdinal, next.checkpoint.baselineFrontier.headEditOrdinal) &&
        forward(current.checkpoint.baselineFrontier.lastCommitSeq, next.checkpoint.baselineFrontier.lastCommitSeq);
  return (
    checkpointForward &&
    forward(current.revalidation.directoryRevision, next.revalidation.directoryRevision) &&
    forward(current.revalidation.membershipGeneration, next.revalidation.membershipGeneration) &&
    forward(current.revalidation.sessionGeneration, next.revalidation.sessionGeneration) &&
    forward(current.revalidation.shareGeneration, next.revalidation.shareGeneration) &&
    sameLeaseFieldsV1(current, { ...next, checkpoint: current.checkpoint, revalidation: current.revalidation })
  );
}

/** 🌐️ Complete localized execution-target status vocabulary. No code carries an origin, URL, path,
 * receipt, grant, digest or user identity; EN and DE are both explicit with no default language. */
export type DocumentExecutionTargetStatusCodeV1 = "verifying" | "retrying" | "catching-up" | "integrity-failed" | "stale" | "cancelled" | "renderer-unavailable" | "link-expired" | "access-revoked";

export const DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1: Readonly<Record<DocumentExecutionTargetStatusCodeV1, Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  verifying: Object.freeze({ en: "Verifying document component…", de: "Dokumentkomponente wird überprüft…" }),
  retrying: Object.freeze({ en: "The hub is busy. Asking again for the document component…", de: "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…" }),
  "catching-up": Object.freeze({ en: "Catching up with the hub…", de: "Gleiche mit dem Hub ab…" }),
  "integrity-failed": Object.freeze({ en: "The document component could not be verified. Reopen the document.", de: "Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut." }),
  stale: Object.freeze({ en: "The document target changed. Reopen the document.", de: "Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut." }),
  cancelled: Object.freeze({ en: "Opening the document was cancelled.", de: "Das Öffnen des Dokuments wurde abgebrochen." }),
  "renderer-unavailable": Object.freeze({ en: "The verified document component is ready, but this renderer is unavailable.", de: "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar." }),
  "link-expired": Object.freeze({ en: "The connection was lost for too long. Reconnect to keep editing this document.", de: "Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten." }),
  "access-revoked": Object.freeze({ en: "Your access to this document was removed.", de: "Ihr Zugriff auf dieses Dokument wurde entfernt." }),
});

/** 🔊️ ARIA live-region politeness for one execution-target status: progress and a retry of a declared transient answer
 * announce (the opening still runs and can be cancelled), every terminal integrity/stale/renderer outcome asserts. */
export function documentExecutionTargetStatusRoleV1(code: DocumentExecutionTargetStatusCodeV1): "status" | "alert" {
  return code === "verifying" || code === "retrying" || code === "catching-up" ? "status" : "alert";
}

/** 🪜️ Every stage one execution-target install passes through. The first five are the owner's own
 * fetch and verification; the `actor-` stages are the browser-actor child's, relayed verbatim from
 * `BROWSER_ACTOR_CHILD_LOAD_STAGES` so the load phase — which used to report nothing at all between
 * the last fetched byte and either a live component or a silent stall — names the stage it is in. */
export type DocumentExecutionTargetProgressStageV1 =
  | "manifest"
  | "component"
  | "descriptor"
  | "browser-actor"
  | "verify"
  | "actor-transfer"
  | "actor-received"
  | "actor-verified"
  | "actor-importing"
  | "actor-imported"
  | "actor-decode"
  | "actor-compile"
  | "actor-instantiate"
  | "actor-activating"
  | "actor-active"
  | "actor-describe"
  | "actor-verify"
  | "actor-open"
  | "actor-cold"
  | "actor-view"
  | "actor-ready"
  | "canonical-pair"
  | "catch-up";

/** 📈️ Bounded install progress. It never carries bytes, paths, receipts or full digests. For the `catch-up` stage the two counts
 * are hub tail messages (delivered / retained), not bytes. */
export interface DocumentExecutionTargetProgressV1 {
  stage: DocumentExecutionTargetProgressStageV1;
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

/** 🌱️ Exact scope-bound empty history, never an invented edit identifier. */
export function artifactFrontierIsGenesisForV1(scope: DocumentScope, frontier: ArtifactFrontier): boolean {
  const text = (value: string): boolean => value.length > 0 && new TextEncoder().encode(value).length <= DOCUMENT_OPEN_ID_MAX_BYTES && !/[\u0000-\u001f\u007f-\u009f\ud800-\udfff]/u.test(value);
  return text(scope.spaceId) && text(scope.documentId) && frontier.documentId === scope.documentId && frontier.headEditOrdinal === 0 && frontier.headEditId === "" && frontier.lastCommitSeq === 0 && frontier.chainHash.length === 32 && frontier.chainHash.every((byte) => byte === 0);
}

/** 🌿️ Normal edited history has exact positive counters and a nonzero authenticated head. */
export function artifactFrontierIsEditedForV1(scope: DocumentScope, frontier: ArtifactFrontier): boolean {
  const text = (value: string): boolean => value.length > 0 && new TextEncoder().encode(value).length <= DOCUMENT_OPEN_ID_MAX_BYTES && !/[\u0000-\u001f\u007f-\u009f\ud800-\udfff]/u.test(value);
  return text(scope.spaceId) && frontier.documentId === scope.documentId && text(frontier.documentId) && text(frontier.headEditId)
    && Number.isSafeInteger(frontier.headEditOrdinal) && frontier.headEditOrdinal > 0 && Number.isSafeInteger(frontier.lastCommitSeq) && frontier.lastCommitSeq > 0
    && frontier.chainHash.length === 32 && frontier.chainHash.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255) && frontier.chainHash.some((byte) => byte !== 0);
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
export function descriptorDigestEncodingV1(descriptor: DocumentDescriptor): Uint8Array<ArrayBuffer> {
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
export async function descriptorDigestV1(descriptor: DocumentDescriptor): Promise<Uint8Array<ArrayBuffer>> {
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

//#region 🪢️CanonicalCheckpointPair
/** 🪢️ The hub's canonical-checkpoint-pair media type — the ONE transport that hands a cold client
 * the published pack+SPR pair of a document's active checkpoint (`GET
 * /spaces/{spaceId}/documents/{documentId}/active-checkpoint/pair`). The socket's `Welcome` never
 * carries a pair: its bootstrap plan is computed by the database replay, whose only outcomes are
 * `None`, `Tail` and the database-private `Snapshot` an artifact client refuses by contract. */
export const CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1 = "application/vnd.semio.canonical-checkpoint-pair.v1";
export const CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES = 16 * 1024;
export const CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES = 4 * 1024;
export const CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS = 16_384;
export const CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES = 64 * 1024 * 1024;
const CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION = 1;
const CANONICAL_CHECKPOINT_PAIR_HEADER = 1;
const CANONICAL_CHECKPOINT_PAIR_DATA = 2;
const CANONICAL_CHECKPOINT_PAIR_TERMINAL = 3;
const CANONICAL_CHECKPOINT_PAIR_PART_PACK = 1;
const CANONICAL_CHECKPOINT_PAIR_PART_SPR = 2;
const CANONICAL_CHECKPOINT_PAIR_TERMINAL_COMPLETE = 0;

/** 🪢️ One decoded, length-and-shape-checked canonical pair: its selection identity and its bytes.
 * The bytes are NOT yet proven against the selection's digests — `verifyCanonicalCheckpointPairV1`
 * does that, because hashing needs `crypto.subtle` and this decoder stays synchronous. */
export interface CanonicalCheckpointPairV1 {
  scope: DocumentScope;
  descriptorDigestV1: ArtifactHash;
  activeCheckpointId: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  pack: PublishedArtifactBlob;
  spr: PublishedArtifactBlob;
  aggregateSha256: ArtifactHash;
  packBytes: Uint8Array;
  sprBytes: Uint8Array;
}

class CanonicalPairCursor {
  private readonly bytes: Uint8Array;
  offset: number;
  constructor(bytes: Uint8Array, offset = 0) {
    this.bytes = bytes;
    this.offset = offset;
  }

  get exhausted(): boolean {
    return this.offset === this.bytes.length;
  }

  take(length: number): Uint8Array {
    const end = this.offset + length;
    if (!Number.isSafeInteger(end) || end > this.bytes.length) throw new Error("canonical-checkpoint-pair.truncated");
    const value = this.bytes.subarray(this.offset, end);
    this.offset = end;
    return value;
  }

  byte(): number {
    return this.take(1)[0] as number;
  }

  u32(): number {
    const slice = this.take(4);
    return (((slice[0] as number) << 24) >>> 0) + ((slice[1] as number) << 16) + ((slice[2] as number) << 8) + (slice[3] as number);
  }

  u64(): number {
    const slice = this.take(8);
    let value = 0n;
    for (const byte of slice) value = (value << 8n) | BigInt(byte);
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("canonical-checkpoint-pair.capacity");
    return Number(value);
  }

  hash(): ArtifactHash {
    return Array.from(this.take(32));
  }

  /** #️⃣ A hash the hub refuses to publish as all-zero: an unset digest must never read as a match. */
  digest(): ArtifactHash {
    const hash = this.hash();
    if (hash.every((byte) => byte === 0)) throw new Error("canonical-checkpoint-pair.zero-digest");
    return hash;
  }

  text(maximum: number): string {
    const length = this.u32();
    if (length > maximum) throw new Error("canonical-checkpoint-pair.oversized-text");
    return new TextDecoder("utf-8", { fatal: true }).decode(this.take(length));
  }

  frame(maximum: number): CanonicalPairCursor {
    const length = this.u32();
    if (length === 0 || length > maximum) throw new Error("canonical-checkpoint-pair.frame-length");
    return new CanonicalPairCursor(this.take(length));
  }
}

/** 🧮️ Exact record count the hub emits for a pair of these two lengths — pack records first, then
 * SPR records, each `CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES` except the last of each part. */
function canonicalCheckpointPairRecordCountV1(packLength: number, sprLength: number): number {
  return Math.ceil(packLength / CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES) + Math.ceil(sprLength / CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES);
}

/** 🪢️ Decodes the hub's canonical-checkpoint-pair body, the exact inverse of the hub's
 * `append_canonical_pair_{header,data,terminal}`: a stream of `u32be length | payload` frames whose
 * first payload is the selection header, whose middle payloads carry strictly ordered pack-then-SPR
 * data records at contiguous offsets, and whose last payload is the `Complete` terminal. Every
 * refusal is named; a partial, reordered, over-long or unterminated body is never half-accepted. */
export function decodeCanonicalCheckpointPairV1(input: Uint8Array): CanonicalCheckpointPairV1 {
  const maximumWire = CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES + CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES + CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS * 22 + 6;
  if (input.length > maximumWire) throw new Error("canonical-checkpoint-pair.oversized");
  const stream = new CanonicalPairCursor(input);
  const header = stream.frame(CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES);
  if (header.byte() !== CANONICAL_CHECKPOINT_PAIR_HEADER || header.u32() !== CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION) throw new Error("canonical-checkpoint-pair.header");
  const scope: DocumentScope = { spaceId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES), documentId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES) };
  const descriptorDigestV1 = header.digest();
  const activeCheckpointId = header.digest();
  const baselineFrontier: ArtifactFrontier = {
    documentId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES),
    headEditOrdinal: header.u64(),
    headEditId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES),
    lastCommitSeq: header.u64(),
    chainHash: header.hash(),
  };
  const pack: PublishedArtifactBlob = { sha256: header.digest(), byteLength: header.u64() };
  const spr: PublishedArtifactBlob = { sha256: header.digest(), byteLength: header.u64() };
  const aggregateSha256 = header.digest();
  if (!header.exhausted) throw new Error("canonical-checkpoint-pair.header-trailing-bytes");
  if (!artifactFrontierIsGenesisForV1(scope, baselineFrontier) && !artifactFrontierIsEditedForV1(scope, baselineFrontier)) throw new Error("canonical-checkpoint-pair.baseline-frontier");
  if (pack.byteLength + spr.byteLength > CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES || pack.byteLength === 0 || spr.byteLength === 0) throw new Error("canonical-checkpoint-pair.pair-length");
  const records = canonicalCheckpointPairRecordCountV1(pack.byteLength, spr.byteLength);
  if (records > CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS) throw new Error("canonical-checkpoint-pair.record-count");
  const packBytes = new Uint8Array(pack.byteLength);
  const sprBytes = new Uint8Array(spr.byteLength);
  let packFilled = 0;
  let sprFilled = 0;
  for (let ordinal = 0; ordinal < records; ordinal += 1) {
    const record = stream.frame(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES + 18);
    if (record.byte() !== CANONICAL_CHECKPOINT_PAIR_DATA) throw new Error("canonical-checkpoint-pair.record-tag");
    const part = record.byte();
    if (record.u32() !== ordinal) throw new Error("canonical-checkpoint-pair.record-ordinal");
    const offset = record.u64();
    const length = record.u32();
    if (length === 0 || length > CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES) throw new Error("canonical-checkpoint-pair.record-length");
    const bytes = record.take(length);
    if (!record.exhausted) throw new Error("canonical-checkpoint-pair.record-trailing-bytes");
    if (part === CANONICAL_CHECKPOINT_PAIR_PART_PACK && packFilled < pack.byteLength) {
      if (offset !== packFilled || packFilled + length > pack.byteLength) throw new Error("canonical-checkpoint-pair.record-offset");
      packBytes.set(bytes, packFilled);
      packFilled += length;
    } else if (part === CANONICAL_CHECKPOINT_PAIR_PART_SPR && packFilled === pack.byteLength) {
      if (offset !== sprFilled || sprFilled + length > spr.byteLength) throw new Error("canonical-checkpoint-pair.record-offset");
      sprBytes.set(bytes, sprFilled);
      sprFilled += length;
    } else throw new Error("canonical-checkpoint-pair.record-part");
  }
  const terminal = stream.frame(2);
  if (terminal.byte() !== CANONICAL_CHECKPOINT_PAIR_TERMINAL || terminal.byte() !== CANONICAL_CHECKPOINT_PAIR_TERMINAL_COMPLETE) throw new Error("canonical-checkpoint-pair.terminal");
  if (!stream.exhausted || packFilled !== pack.byteLength || sprFilled !== spr.byteLength) throw new Error("canonical-checkpoint-pair.incomplete");
  return { scope, descriptorDigestV1, activeCheckpointId, baselineFrontier, pack, spr, aggregateSha256, packBytes, sprBytes };
}

/** 🔡️ Canonical lowercase hexadecimal of one 32-byte hash. */
function canonicalCheckpointPairHexV1(hash: ArtifactHash): string {
  return hash.map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 🎫️ Admits a decoded pair only as exactly the checkpoint the hub authorized for this open (the execution-target
 * lease's or the open plan's `checkpoint`): a checkpoint that moved, a changed descriptor or a foreign scope is
 * refused by name, never mounted — the twin of the kernel's `CanonicalCheckpointPairV1::admit`. */
export function admitCanonicalCheckpointPairV1(pair: CanonicalCheckpointPairV1, scope: DocumentScope, expected: DocumentOpenCheckpointV1): void {
  if (pair.scope.spaceId !== scope.spaceId || pair.scope.documentId !== scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (canonicalCheckpointPairHexV1(pair.activeCheckpointId) !== expected.checkpointId) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (canonicalCheckpointPairHexV1(pair.descriptorDigestV1) !== expected.descriptorDigestV1) throw new Error("canonical-checkpoint-pair.descriptor");
  const baseline = expected.baselineFrontier;
  const frontier = pair.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || canonicalCheckpointPairHexV1(frontier.chainHash) !== canonicalCheckpointPairHexV1(baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
  if (canonicalCheckpointPairHexV1(pair.aggregateSha256) !== expected.aggregateSha256) throw new Error("canonical-checkpoint-pair.aggregate");
}

/** 🛟️ Admits a decoded pair only as exactly the checkpoint a hub `RebootstrapRequired` control names — the twin of the kernel's
 * `CanonicalCheckpointPairV1::admit_rebootstrap` (the control carries no aggregate; the digests prove the bytes). */
export function admitCanonicalCheckpointPairForRebootstrapV1(pair: CanonicalCheckpointPairV1, control: { readonly scope: DocumentScope; readonly checkpointId: ArtifactHash; readonly descriptorDigestV1: ArtifactHash; readonly baselineFrontier: ArtifactFrontier }): void {
  if (pair.scope.spaceId !== control.scope.spaceId || pair.scope.documentId !== control.scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (canonicalCheckpointPairHexV1(pair.activeCheckpointId) !== canonicalCheckpointPairHexV1(control.checkpointId)) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (canonicalCheckpointPairHexV1(pair.descriptorDigestV1) !== canonicalCheckpointPairHexV1(control.descriptorDigestV1)) throw new Error("canonical-checkpoint-pair.descriptor");
  const frontier = pair.baselineFrontier;
  const baseline = control.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || canonicalCheckpointPairHexV1(frontier.chainHash) !== canonicalCheckpointPairHexV1(baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
}
//#endregion 🪢️CanonicalCheckpointPair

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

/** 🔑️ Which way one reader's own access to a space moved. */
export type DirectoryAccessChange = "granted" | "revoked";

export type DirectoryStreamMessage =
  | { kind: "event"; event: DirectoryEvent }
  | { kind: "connection"; phase: DirectoryConnectionPhase; connection: ConnectionView }
  | { kind: "presence"; spaceId: string; documentId: string; actors: DirectoryPresenceActor[] }
  | { kind: "heartbeat"; headSeq: number }
  | { kind: "rebootstrap-required"; control: RebootstrapRequired }
  | { kind: "access-changed"; spaceId: string; change: DirectoryAccessChange };
//#endregion 🔖️Stream
