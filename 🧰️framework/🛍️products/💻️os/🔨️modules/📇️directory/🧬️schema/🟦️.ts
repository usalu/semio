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
