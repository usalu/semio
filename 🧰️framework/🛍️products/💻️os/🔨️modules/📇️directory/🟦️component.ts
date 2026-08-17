/** 📇️ Directory read model — pure fold over `DirectoryEvent` (ticket 26/08/16/HUB-SPACES-LIVE-
 * PRESENCE-AND-COLLABORATIVE-STUDIOS, contract C1). TypeScript twin of `🦀️component.rs` — byte-
 * identical projection over the golden fixture `../../🧫️fixtures/📇️directory/🧾️events.json` (parity
 * asserted in `../../🟦️component.ts`'s `🔖️Directory` region, the only file this package's vitest
 * config scans for `import.meta.vitest` suites). */

export type {
  ConnectionView,
  DirectoryActor,
  DirectoryActorKind,
  DirectoryCommand,
  DirectoryConnectionPhase,
  DirectoryEvent,
  DirectoryEventBody,
  DirectoryEventInviteRedeemed,
  DirectoryEventMemberRemoved,
  DirectoryEventMemberUpserted,
  DirectoryEventSpaceArchived,
  DirectoryEventSpaceCreated,
  DirectoryEventSpaceDeleted,
  DirectoryEventSpaceRenamed,
  DirectoryEventSpaceVisibilityChanged,
  DirectoryEventUserCreated,
  DirectorySpaceKind,
  DirectorySpaceRole,
  DirectorySpaceVisibility,
  DirectoryStreamMessage,
  DocumentView,
  Hlc,
  InviteView,
  MemberView,
  SpaceView,
  UserView,
} from "./🧬️schema/🟦️component.ts";

import type { DirectoryCommand, DirectoryEvent, DirectoryEventBody, DirectoryStreamMessage, MemberView, SpaceView, UserView } from "./🧬️schema/🟦️component.ts";

//#region 🔖️ReadModel
/** 🏠️ One projected space: its `SpaceView` plus the current member roster. */
export interface DirectorySpace {
  view: SpaceView;
  members: MemberView[];
}

/** 📇️ The directory's whole projected state, folded from the event log. `users` is a side-table
 * (not part of contract-freeze.md's C1 prose shape) that backfills `MemberView.email`/
 * `displayName` from `user.created` — see this file's header. */
export interface DirectoryReadModel {
  spaces: Map<string, DirectorySpace>;
  cursor: number;
  users: Map<string, UserView>;
}

export function emptyDirectoryReadModel(): DirectoryReadModel {
  return { spaces: new Map(), cursor: 0, users: new Map() };
}

/** 🔗️ Upserts `userId`/`role` into `space.members`, joining `email`/`displayName` from `users`
 * when known (falls back to empty strings for a member added before their own `user.created`). */
function upsertMember(space: DirectorySpace, users: Map<string, UserView>, userId: string, role: MemberView["role"], updatedAtMs: number): void {
  const user = users.get(userId);
  const existing = space.members.find((member) => member.userId === userId);
  if (existing) {
    existing.role = role;
  } else {
    space.members.push({ userId, email: user?.email ?? "", displayName: user?.displayName ?? "", role });
  }
  space.view.memberCount = space.members.length;
  space.view.updatedAtMs = updatedAtMs;
}

/** 🧮️ Pure fold: `model × event -> model`. Idempotent — an event whose `seq` does not strictly
 * advance `model.cursor` (already-applied or out-of-order-old) is ignored wholesale. Returns a new
 * model (shallow-copies `spaces`/`users`, deep-copies only the touched space) rather than mutating
 * `model` in place, matching the Rust twin's by-value `fold(model, event) -> model` signature. */
export function fold(model: DirectoryReadModel, event: DirectoryEvent): DirectoryReadModel {
  if (event.seq <= model.cursor) return model;
  const spaces = new Map(model.spaces);
  const users = new Map(model.users);
  const next: DirectoryReadModel = { spaces, cursor: event.seq, users };
  const body: DirectoryEventBody = event.body;

  const withSpace = (spaceId: string, mutate: (space: DirectorySpace) => void): void => {
    const existing = spaces.get(spaceId);
    if (!existing) return;
    const copy: DirectorySpace = { view: { ...existing.view }, members: existing.members.map((member) => ({ ...member })) };
    mutate(copy);
    spaces.set(spaceId, copy);
  };

  switch (body.kind) {
    case "user.created":
      users.set(body.userId, { id: body.userId, email: body.email, displayName: body.displayName, createdAtMs: event.recordedAtMs });
      break;
    case "space.created":
      spaces.set(body.spaceId, {
        view: {
          id: body.spaceId,
          name: body.name,
          kind: body.spaceKind,
          visibility: body.visibility,
          ownerUserId: body.ownerUserId,
          memberCount: 0,
          documentCount: 0,
          activeConnections: 0,
          createdAtMs: event.recordedAtMs,
          updatedAtMs: event.recordedAtMs,
        },
        members: [],
      });
      break;
    case "space.renamed":
      withSpace(body.spaceId, (space) => {
        space.view.name = body.name;
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "space.visibility-changed":
      withSpace(body.spaceId, (space) => {
        space.view.visibility = body.visibility;
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "space.archived":
      withSpace(body.spaceId, (space) => {
        space.view.kind = "archive";
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "space.deleted":
      spaces.delete(body.spaceId);
      break;
    case "member.upserted":
      withSpace(body.spaceId, (space) => upsertMember(space, users, body.userId, body.role, event.recordedAtMs));
      break;
    case "member.removed":
      withSpace(body.spaceId, (space) => {
        space.members = space.members.filter((member) => member.userId !== body.userId);
        space.view.memberCount = space.members.length;
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "invite.redeemed":
      withSpace(body.spaceId, (space) => upsertMember(space, users, body.userId, body.role, event.recordedAtMs));
      break;
  }
  return next;
}

/** 🔁️ Folds every event in order. */
export function foldAll(model: DirectoryReadModel, events: readonly DirectoryEvent[]): DirectoryReadModel {
  return events.reduce(fold, model);
}
//#endregion 🔖️ReadModel

//#region 🔖️TypeGuards
/** 🛡️ Narrows a `DirectoryEventBody` to one `kind` variant. */
export function isDirectoryEventBodyKind<K extends DirectoryEventBody["kind"]>(body: DirectoryEventBody, kind: K): body is Extract<DirectoryEventBody, { kind: K }> {
  return body.kind === kind;
}

/** 🛡️ Narrows a `DirectoryCommand` to one `kind` variant. */
export function isDirectoryCommandKind<K extends DirectoryCommand["kind"]>(command: DirectoryCommand, kind: K): command is Extract<DirectoryCommand, { kind: K }> {
  return command.kind === kind;
}

/** 🛡️ Narrows a `DirectoryStreamMessage` to one `kind` variant. */
export function isDirectoryStreamMessageKind<K extends DirectoryStreamMessage["kind"]>(message: DirectoryStreamMessage, kind: K): message is Extract<DirectoryStreamMessage, { kind: K }> {
  return message.kind === kind;
}
//#endregion 🔖️TypeGuards
