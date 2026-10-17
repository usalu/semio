/** 👀️ Directory read views: spaces as the hub renders them, members, users, connections and document descriptors. */

import { type ArtifactHash } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility } from "../🟦️.ts";

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
//#endregion 🔖️Views
