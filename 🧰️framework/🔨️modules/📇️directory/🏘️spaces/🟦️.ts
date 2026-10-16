/** 🏘️ End-user spaces surface contract — the "browse my spaces, switch, create, see who is here,
 * invite, redeem" half of the directory that `🛂️SpaceAdministration` (an *admin* pane for exactly
 * one space) never covered. Pure projection + command construction: every mutation leaves as a
 * closed `DirectoryCommand` for `POST /directory/commands` (CQRS — this module never issues a CRUD
 * write), and invite redemption leaves as the hub's own `POST /directory/invites/{token}/redeem`
 * path. Sorting and filtering are locale-independent so the two renderers agree byte for byte. */

import type { DirectoryCommand, DirectoryEvent, DirectorySpaceKind, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility, MemberSpaceViewV1, PublicSpaceViewV1 } from "../🧬️schema/🟦️.ts";

//#region 🔖️Routes
export const DIRECTORY_COMMANDS_PATH_V1 = "/directory/commands";
export const DIRECTORY_SPACES_PATH_V1 = "/directory/spaces";
export const INVITE_TOKEN_MAX_BYTES = 256;
export const SPACE_NAME_MAX_BYTES = 128;
export const INVITE_TTL_CHOICES_SECS_V1 = [3600, 86400, 604800] as const;
export const INVITE_LINK_FRAGMENT_V1 = "#semio-invite=";

/** 🎟️ The hub's redemption route for one invite capability. The token is a capability, so it goes in
 * the path of a POST and never into a query string, a log line or the connection book. */
export function inviteRedeemPathV1(token: string): string {
  return `/directory/invites/${encodeURIComponent(parseInviteTokenV1(token))}/redeem`;
}

/** 📮️ Whether a `POST /directory/commands` answer carries a receipt: any 2xx, exactly as the Rust directory client
 * (`📇️directory/🔌️client`) reads it. The hub answers an accepted command `202 Accepted` with its canonical receipt, and
 * the receipt parser (bound to the sealed request) is what proves the answer; every other status is a refusal. */
export function directoryCommandAnsweredV1(status: number): boolean {
  return status >= 200 && status < 300;
}
//#endregion 🔖️Routes

//#region 🔖️Rows
export type SpaceAccessV1 = "author" | "member" | "public";

/** 🏠️ One space as the end-user browser renders it — a flattened
 * {@link DirectorySpaceListEntryV1} with the access class hoisted out of the union so a row needs
 * no narrowing to render. `role` is `null` for a public space the caller is not a member of. */
export interface SpaceRowV1 {
  readonly id: string;
  readonly name: string;
  readonly kind: DirectorySpaceKind;
  readonly visibility: DirectorySpaceVisibility;
  readonly access: SpaceAccessV1;
  readonly role: DirectorySpaceRole | null;
  readonly memberCount: number;
  readonly documentCount: number;
  readonly activeConnections: number;
  readonly updatedAtMs: number;
}

const ACCESS_ORDER: Readonly<Record<SpaceAccessV1, number>> = { author: 0, member: 1, public: 2 };

function spaceRow(entry: DirectorySpaceListEntryV1): SpaceRowV1 {
  const space: MemberSpaceViewV1 | PublicSpaceViewV1 = entry.space;
  const member = entry.access === "public" ? null : (entry.space as MemberSpaceViewV1);
  return {
    id: space.id,
    name: space.name,
    kind: space.kind,
    visibility: space.visibility,
    access: entry.access,
    role: member === null ? null : member.role,
    memberCount: space.memberCount,
    documentCount: space.documentCount,
    activeConnections: member === null ? 0 : member.activeConnections,
    updatedAtMs: space.updatedAtMs,
  };
}

/** 📇️ Projects the hub's space list into render rows: my spaces first (author, then member, then
 * public), each group most-recently-updated first, ties broken by id. Deterministic and
 * locale-independent — `localeCompare` would make two devices disagree on order. */
export function spaceRowsV1(entries: readonly DirectorySpaceListEntryV1[]): readonly SpaceRowV1[] {
  return sortSpaceRows(entries.map(spaceRow));
}

function sortSpaceRows(rows: SpaceRowV1[]): readonly SpaceRowV1[] {
  return rows.sort((left, right) => ACCESS_ORDER[left.access] - ACCESS_ORDER[right.access] || right.updatedAtMs - left.updatedAtMs || (left.id < right.id ? -1 : left.id > right.id ? 1 : 0));
}

/** 🧾️ Read-your-writes for the space list: folds the directory events of one command receipt (in receipt order) into
 * the caller's rows, so a space the caller just created, renamed, archived, joined, left or deleted shows at once,
 * whatever the list query answers (it lags, fails or times out on a loaded hub). Only what an event states exactly is
 * applied — a membership change of another user on a listed space keeps its member count until the next list, which
 * replaces the rows wholesale. Mirrors the directory read-model fold (`📇️directory/🟦️.ts` `fold`), projected onto the
 * caller: a space enters the rows with the caller's own membership, a role picks the access (`author` → `author`,
 * `spectator` → `member`), a public space the caller leaves stays listed as `public`. The Rust twin is
 * `space_rows_after_events` in `🏘️SpaceBrowser/🎯️targets/🧊️wgpu`; both are held to `🏘️spaces/🔣️.json` `receiptFolds`. */
export function spaceRowsAfterEventsV1(rows: readonly SpaceRowV1[], events: readonly DirectoryEvent[], userId: string): readonly SpaceRowV1[] {
  const next = rows.map((row) => ({ ...row }));
  const created = new Map<string, { name: string; kind: DirectorySpaceKind; visibility: DirectorySpaceVisibility; members: Set<string> }>();
  const at = (spaceId: string): number => next.findIndex((row) => row.id === spaceId);
  const touch = (spaceId: string, recordedAtMs: number, change: (row: SpaceRowV1) => SpaceRowV1): void => {
    const index = at(spaceId);
    if (index >= 0) next[index] = { ...change(next[index]!), updatedAtMs: recordedAtMs };
  };
  const join = (spaceId: string, member: string, role: DirectorySpaceRole, recordedAtMs: number, redeemed: boolean): void => {
    const fresh = created.get(spaceId);
    fresh?.members.add(member);
    const access: SpaceAccessV1 = role === "author" ? "author" : "member";
    if (member === userId && at(spaceId) < 0 && fresh !== undefined) {
      next.push({ id: spaceId, name: fresh.name, kind: fresh.kind, visibility: fresh.visibility, access, role, memberCount: fresh.members.size, documentCount: 0, activeConnections: 0, updatedAtMs: recordedAtMs });
      return;
    }
    touch(spaceId, recordedAtMs, (row) => ({
      ...row,
      ...(member === userId ? { access, role } : {}),
      memberCount: fresh !== undefined ? fresh.members.size : (member === userId && row.access === "public") || (member !== userId && redeemed) ? row.memberCount + 1 : row.memberCount,
    }));
  };
  for (const event of events) {
    const body = event.body;
    switch (body.kind) {
      case "space.created":
        if (body.ownerUserId === userId) created.set(body.spaceId, { name: body.name, kind: body.spaceKind, visibility: body.visibility, members: new Set() });
        break;
      case "space.renamed":
        touch(body.spaceId, event.recordedAtMs, (row) => ({ ...row, name: body.name }));
        break;
      case "space.visibility-changed":
        touch(body.spaceId, event.recordedAtMs, (row) => ({ ...row, visibility: body.visibility }));
        break;
      case "space.archived":
        touch(body.spaceId, event.recordedAtMs, (row) => ({ ...row, kind: "archive", ...(row.role === "author" ? { role: "spectator" as const, access: "member" as const } : {}) }));
        break;
      case "space.deleted":
        if (at(body.spaceId) >= 0) next.splice(at(body.spaceId), 1);
        break;
      case "member.upserted":
        join(body.spaceId, body.userId, body.role, event.recordedAtMs, false);
        break;
      case "invite.redeemed":
        join(body.spaceId, body.userId, body.role, event.recordedAtMs, true);
        break;
      case "member.removed": {
        created.get(body.spaceId)?.members.delete(body.userId);
        const index = at(body.spaceId);
        if (index < 0) break;
        const row = next[index]!;
        if (body.userId === userId && row.visibility !== "public") next.splice(index, 1);
        else next[index] = { ...row, ...(body.userId === userId ? { access: "public" as const, role: null, activeConnections: 0 } : {}), memberCount: Math.max(0, row.memberCount - 1), updatedAtMs: event.recordedAtMs };
        break;
      }
      case "document.announced":
        touch(body.descriptor.spaceId, event.recordedAtMs, (row) => ({ ...row, documentCount: row.documentCount + 1 }));
        break;
      default:
        break;
    }
  }
  return sortSpaceRows(next);
}

/** 🔎️ Case-insensitive substring filter over name and id. An empty query keeps every row. */
export function filterSpaceRowsV1(rows: readonly SpaceRowV1[], query: string): readonly SpaceRowV1[] {
  const needle = query.trim().toLowerCase();
  return needle.length === 0 ? rows : rows.filter((row) => row.name.toLowerCase().includes(needle) || row.id.toLowerCase().includes(needle));
}

/** ✍️ Whether the caller may create documents in this space — the only authority a row carries. */
export function spaceRowWritableV1(row: SpaceRowV1): boolean {
  return row.access === "author" || row.role === "author";
}

/** 🎟️ Whether the caller may issue invitations for this space. Membership alone is never enough. */
export function spaceRowInvitableV1(row: SpaceRowV1): boolean {
  return row.access === "author";
}
//#endregion 🔖️Rows

//#region 🔖️Presence
/** 👥️ One member of a space, joined with whether they are connected right now. */
export interface SpaceMemberPresenceV1 {
  readonly userId: string;
  readonly displayName: string;
  readonly role: DirectorySpaceRole;
  readonly owner: boolean;
  readonly online: boolean;
}

/** 👥️ Joins the member roster with the set of user ids the presence lane reports as connected.
 * Owners first, then authors, then spectators, each alphabetically by user id so the roster does not
 * reshuffle on every presence tick. */
export function spaceMemberPresenceV1(members: readonly Readonly<{ userId: string; displayName: string; email: string; role: DirectorySpaceRole; owner: boolean }>[], onlineUserIds: readonly string[]): readonly SpaceMemberPresenceV1[] {
  const online = new Set(onlineUserIds);
  return members
    .map((member) => ({ userId: member.userId, displayName: member.displayName.length > 0 ? member.displayName : member.email, role: member.role, owner: member.owner, online: online.has(member.userId) }))
    .sort((left, right) => Number(right.owner) - Number(left.owner) || (left.role === right.role ? 0 : left.role === "author" ? -1 : 1) || (left.userId < right.userId ? -1 : left.userId > right.userId ? 1 : 0));
}
//#endregion 🔖️Presence

//#region 🔖️Commands
/** 🏗️ Builds the `create-space` command in the exact canonical field order
 * (`kind, name, spaceKind, visibility`) `parseDirectoryCommandV1` re-serializes and compares against
 * — a different key order is a `noncanonical-command` refusal at seal time, not at the hub. */
export function createSpaceCommandV1(name: string, spaceKind: DirectorySpaceKind, visibility: DirectorySpaceVisibility): DirectoryCommand {
  const trimmed = name.trim();
  if (trimmed.length === 0 || new TextEncoder().encode(trimmed).byteLength > SPACE_NAME_MAX_BYTES || /\p{Cc}/u.test(trimmed)) throw new Error("directory.spaces.invalid-name");
  return { kind: "create-space", name: trimmed, spaceKind, visibility };
}

/** 🎟️ Builds the `create-invite` command in canonical field order (`kind, spaceId, role, ttlSecs`).
 * The hub answers with a one-shot capability in the command receipt's `result`; this module never
 * sees it, so it cannot leak it. */
export function createInviteCommandV1(spaceId: string, role: DirectorySpaceRole, ttlSecs: number): DirectoryCommand {
  if (spaceId.length === 0 || /\p{Cc}/u.test(spaceId)) throw new Error("directory.spaces.invalid-space");
  if (!Number.isSafeInteger(ttlSecs) || ttlSecs <= 0) throw new Error("directory.spaces.invalid-ttl");
  return { kind: "create-invite", spaceId, role, ttlSecs };
}

/** 🚪️ Builds the `archive-space` command — the end-user "leave this behind" verb; deletion stays in
 * the admin pane, which gates it on the server-declared `deleteSpace` capability. */
export function archiveSpaceCommandV1(spaceId: string): DirectoryCommand {
  if (spaceId.length === 0 || /\p{Cc}/u.test(spaceId)) throw new Error("directory.spaces.invalid-space");
  return { kind: "archive-space", spaceId };
}
//#endregion 🔖️Commands

//#region 🔖️Invites
/** 🎟️ Accepts either a bare capability or a whole invitation link and yields the bare capability.
 * Pasting a link is what humans actually do, so the fragment form
 * (`https://hub…/#semio-invite=<token>`) is parsed here rather than refused with a lecture. */
export function parseInviteTokenV1(text: string): string {
  const trimmed = text.trim();
  const fragment = trimmed.indexOf(INVITE_LINK_FRAGMENT_V1);
  const token = fragment === -1 ? trimmed : trimmed.slice(fragment + INVITE_LINK_FRAGMENT_V1.length);
  if (token.length === 0 || new TextEncoder().encode(token).byteLength > INVITE_TOKEN_MAX_BYTES) throw new Error("directory.spaces.invalid-invite");
  if (!/^[A-Za-z0-9._~-]+$/u.test(token)) throw new Error("directory.spaces.invalid-invite");
  return token;
}

/** 🔗️ Renders one invitation link for a hub origin and capability, for the human to send through a
 * channel of their choosing. The capability lives in the fragment, which browsers never put on the
 * wire, so it cannot reach a server log by being clicked. */
export function inviteLinkV1(origin: string, token: string): string {
  return `${origin}/${INVITE_LINK_FRAGMENT_V1}${parseInviteTokenV1(token)}`;
}

/** 🚫️ Closed redemption denial classes, so the pane names a cause instead of a status. */
export type InviteRedemptionErrorCodeV1 = "invalid-invite" | "expired-invite" | "already-member" | "unauthorized" | "unreachable" | "hub-refused" | "cancelled";

/** 🌐️ Maps one redemption status to its closed code. */
export function inviteRedemptionErrorFromStatusV1(status: number): InviteRedemptionErrorCodeV1 {
  if (status === 400 || status === 404) return "invalid-invite";
  if (status === 410) return "expired-invite";
  if (status === 409) return "already-member";
  if (status === 401 || status === 403) return "unauthorized";
  if (status === 408 || status === 429 || status >= 500) return "unreachable";
  return "hub-refused";
}
//#endregion 🔖️Invites

//#region 🔖️Phase
/** 🔄️ The spaces surface's own load/mutate phase. `stale` is the local-first state: rows are being
 * rendered from the last projection while the hub is unreachable, and the app stays usable. */
export type SpaceBrowserPhaseV1 = "loading" | "ready" | "stale" | "submitting" | "failed";

/** 🏠️ Whether the rows on screen may still be opened. Only an empty first load blocks. */
export function spaceBrowserRowsUsableV1(phase: SpaceBrowserPhaseV1, rowCount: number): boolean {
  return rowCount > 0 && phase !== "loading";
}
//#endregion 🔖️Phase
