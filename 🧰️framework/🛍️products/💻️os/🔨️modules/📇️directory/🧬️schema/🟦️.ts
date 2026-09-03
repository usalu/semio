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
  const object = documentOpenObject(value, ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "surface", "grant", "revalidation"], ["checkpoint"]);
  if (object.schema !== "semio.hub.document-open-plan/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const packageValue = documentOpenObject(object.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256"]);
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  const expiresAtUnixMs = documentOpenInteger(object.expiresAtUnixMs, true);
  if (expiresAtUnixMs <= nowMs || expiresAtUnixMs - nowMs > DOCUMENT_OPEN_PLAN_MAX_TTL_MS || (revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("document-open.expired-or-ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-open.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor")) throw new Error("document-open.invalid-surface");
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
