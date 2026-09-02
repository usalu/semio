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

export type DirectoryEventBody =
  | DirectoryEventUserCreated
  | DirectoryEventSpaceCreated
  | DirectoryEventSpaceRenamed
  | DirectoryEventSpaceVisibilityChanged
  | DirectoryEventSpaceArchived
  | DirectoryEventSpaceDeleted
  | DirectoryEventMemberUpserted
  | DirectoryEventMemberRemoved
  | DirectoryEventInviteRedeemed;

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
  | { kind: "revoke-invite"; spaceId: string; inviteId: string };
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

export interface DocumentView {
  id: string;
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

export type DirectoryStreamMessage =
  | { kind: "event"; event: DirectoryEvent }
  | { kind: "connection"; phase: DirectoryConnectionPhase; connection: ConnectionView }
  | { kind: "presence"; spaceId: string; documentId: string; actors: DirectoryPresenceActor[] }
  | { kind: "heartbeat"; headSeq: number };
//#endregion 🔖️Stream
