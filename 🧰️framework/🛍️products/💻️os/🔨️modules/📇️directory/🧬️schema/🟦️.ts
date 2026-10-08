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
  isEditedArtifactFrontierV1,
  isTerminalDocumentCheckInPhaseV1,
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

export function directoryEventPageObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("directory-event-page.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  if (required.some((key) => !(key in object)) || Object.keys(object).some((key) => !accepted.has(key))) throw new Error("directory-event-page.invalid-fields");
  return object;
}

export function directoryEventPageHasControl(value: unknown): boolean {
  if (typeof value === "string") return /\p{Cc}/u.test(value);
  if (Array.isArray(value)) return value.some(directoryEventPageHasControl);
  return value !== null && typeof value === "object" && Object.entries(value).some(([key, child]) => /\p{Cc}/u.test(key) || directoryEventPageHasControl(child));
}

export function directoryEventPageInteger(value: unknown, positive = false): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("directory-event-page.invalid-integer");
  return value;
}

export function directoryEventPageHash(value: unknown, nonzero: boolean): string {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || (nonzero && /^0{64}$/u.test(value))) throw new Error("directory-event-page.invalid-hash");
  return value;
}

export function directoryEventPageNestedShapes(body: Record<string, unknown>): void {
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






/** 📥️ Parses one canonical page and verifies exact fields, ranges, size, and SHA-256 receipt. */

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

export const DIRECTORY_COMMAND_FIELDS: Record<string, readonly string[]> = {
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

export function directoryCommandRequestId(value: unknown): string {
  if (typeof value !== "string" || value.length !== DIRECTORY_COMMAND_REQUEST_ID_LEN || !/^[0-9a-f]+$/u.test(value) || /^0+$/u.test(value)) throw new Error("directory-command.invalid-request-id");
  return value;
}

/** 🧭 Reconstructs one closed command in declaration order after an order-independent carrier. */


/** 🛡️ Decodes one declaration-ordered canonical command and validates every scalar. */


export function directoryCommandCanonicalResult(value: unknown): DirectoryCommandResultV1 {
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


/** 🆕️ Seals one request around an already-minted correlation id. */


/** 🧾️ Returns the canonical UTF-8 JSON both peers hash and count bytes over. */


/** 📥️ Parses exactly one canonical request, rejecting padding, unknown fields, and oversize bodies. */


/** 🔐️ Seals one completion by hashing its declaration-ordered unsigned canonical JSON — the exact
 * twin of the Rust `DirectoryCommandReceiptV1::seal`. */


/** 📥️ Parses exactly one canonical receipt bound to the request that asked for it. */

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
export function directoryAdministrationCapabilityAllowsV1(page: DirectorySpaceAdministrationPageV1 | null, spaceId: string, command: DirectoryCommand): boolean {
  if (page?.access !== "author" || page.spaceId !== spaceId || page.space.id !== spaceId) return false;
  const canonical = command;
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

export function administrationObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("space-administration-page.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  if (required.some((key) => !(key in object)) || Object.keys(object).some((key) => !accepted.has(key))) throw new Error("space-administration-page.invalid-fields");
  return object;
}

export function administrationText(value: unknown, maximum = DOCUMENT_OPEN_ID_MAX_BYTES, allowEmpty = false): string {
  if (typeof value !== "string" || (!allowEmpty && value.length === 0) || new TextEncoder().encode(value).length > maximum || /\p{Cc}/u.test(value)) throw new Error("space-administration-page.invalid-text");
  return value;
}

export function administrationTime(value: unknown): number {
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

export function administrationWindow<Row>(value: unknown, row: (value: unknown) => Row): DirectorySpaceAdministrationWindowV1<Row> {
  const object = administrationObject(value, ["rows"], ["nextCursor"]);
  if (!Array.isArray(object.rows) || object.rows.length > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS) throw new Error("space-administration-page.invalid-window");
  const rows = object.rows.map(row);
  const nextCursor = administrationCursor(object);
  return nextCursor === undefined ? { rows } : { rows, nextCursor };
}

export function administrationMemberRow(value: unknown): DirectorySpaceAdministrationMemberRowV1 {
  const object = administrationObject(value, ["userId", "email", "displayName", "role", "owner"]);
  return {
    userId: administrationText(object.userId),
    email: administrationText(object.email, DOCUMENT_OPEN_ID_MAX_BYTES, true),
    displayName: administrationText(object.displayName, DOCUMENT_OPEN_ID_MAX_BYTES, true),
    role: administrationRole(object.role),
    owner: administrationBoolean(object.owner),
  };
}

export function administrationInviteRow(value: unknown): DirectorySpaceAdministrationInviteRowV1 {
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

export function administrationCapabilities(value: unknown): DirectorySpaceAdministrationCapabilitiesV1 {
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



/** 📥️ Parses one canonical administration page and verifies fields, ordering, size, and receipt. */


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

export const DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 = 23;

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

/** 🪪️ Validates immutable descriptor metadata without physical encoding or hashing. */
export function validateDocumentDescriptorV1(descriptor: DocumentDescriptor): void {
  if (descriptor.bootstrapVersion === 0) throw new Error("descriptor.invalid-bootstrap-version");
  if (descriptor.bootstrapFrontier.commitSeq > descriptor.bootstrapFrontier.headSeq) throw new Error("descriptor.invalid-bootstrap-frontier");
  for (const [field, value] of [
    ["space-id", descriptor.spaceId], ["document-id", descriptor.documentId],
    ["artifact-kind", descriptor.artifactKind], ["artifact-schema", descriptor.artifactSchema],
    ["owner-plugin-id", descriptor.owner.pluginId], ["owner-package-id", descriptor.owner.packageId], ["owner-version", descriptor.owner.version],
  ] as const) if (value.length === 0) throw new Error(`descriptor.empty-${field}`);
  for (const [field, value] of [
    ["owner-package-hash", descriptor.owner.packageHash], ["pack-schema-hash", descriptor.packSchemaHash], ["bootstrap-snapshot-hash", descriptor.bootstrapSnapshotHash],
  ] as const) if (!/^[0-9a-f]{64}$/.test(value) || /^0{64}$/.test(value)) throw new Error(`descriptor.invalid-${field}`);
  for (const [field, value, maximum] of [
    ["bootstrap-version", descriptor.bootstrapVersion, 0xffff_ffff],
    ["bootstrap-head-seq", descriptor.bootstrapFrontier.headSeq, Number.MAX_SAFE_INTEGER],
    ["bootstrap-commit-seq", descriptor.bootstrapFrontier.commitSeq, Number.MAX_SAFE_INTEGER],
    ["bootstrap-epoch", descriptor.bootstrapFrontier.epoch, Number.MAX_SAFE_INTEGER],
  ] as const) if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error(`descriptor.invalid-${field}`);
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

/** 🎫️ Native-admitted checkpoint metadata for pure typed identity decisions. */
export interface AdmittedCheckpointSelectionV1 {
  checkpointId: ArtifactHash;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  aggregateSha256: ArtifactHash;
}

function sameArtifactHashV1(left: ArtifactHash, right: ArtifactHash): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

/** 🎫️ Admits a decoded pair only as exactly the checkpoint the hub authorized for this open (the execution-target
 * lease's or the open plan's `checkpoint`): a checkpoint that moved, a changed descriptor or a foreign scope is
 * refused by name, never mounted — the twin of the kernel's `CanonicalCheckpointPairV1::admit`. */
export function admitCanonicalCheckpointPairV1(pair: CanonicalCheckpointPairV1, scope: DocumentScope, expected: AdmittedCheckpointSelectionV1): void {
  if (pair.scope.spaceId !== scope.spaceId || pair.scope.documentId !== scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (!sameArtifactHashV1(pair.activeCheckpointId, expected.checkpointId)) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (!sameArtifactHashV1(pair.descriptorDigestV1, expected.descriptorDigestV1)) throw new Error("canonical-checkpoint-pair.descriptor");
  const baseline = expected.baselineFrontier;
  const frontier = pair.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || !sameArtifactHashV1(frontier.chainHash, baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
  if (!sameArtifactHashV1(pair.aggregateSha256, expected.aggregateSha256)) throw new Error("canonical-checkpoint-pair.aggregate");
}

/** 🛟️ Admits a decoded pair only as exactly the checkpoint a hub `RebootstrapRequired` control names — the twin of the kernel's
 * `CanonicalCheckpointPairV1::admit_rebootstrap` (the control carries no aggregate; the digests prove the bytes). */
export function admitCanonicalCheckpointPairForRebootstrapV1(pair: CanonicalCheckpointPairV1, control: { readonly scope: DocumentScope; readonly checkpointId: ArtifactHash; readonly descriptorDigestV1: ArtifactHash; readonly baselineFrontier: ArtifactFrontier }): void {
  if (pair.scope.spaceId !== control.scope.spaceId || pair.scope.documentId !== control.scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (!sameArtifactHashV1(pair.activeCheckpointId, control.checkpointId)) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (!sameArtifactHashV1(pair.descriptorDigestV1, control.descriptorDigestV1)) throw new Error("canonical-checkpoint-pair.descriptor");
  const frontier = pair.baselineFrontier;
  const baseline = control.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || !sameArtifactHashV1(frontier.chainHash, baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
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
