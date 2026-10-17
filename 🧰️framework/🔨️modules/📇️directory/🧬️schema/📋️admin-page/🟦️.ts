/** 📋️ Space administration page: the bounded page, member and invite rows, capabilities and the parsers of the canonical page. */

import { DOCUMENT_OPEN_ID_MAX_BYTES } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DirectorySpaceRole } from "../🟦️.ts";
import type { DirectoryCommand } from "../🎫️command/🟦️.ts";
import type { MemberSpaceViewV1, PublicDocumentCatalogEntryV1, PublicSpaceViewV1 } from "../👀️views/🟦️.ts";
import type { DocumentView } from "../🧊️artifact-checkpoints/🟦️.ts";

//#region 🔖️Views
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
//#endregion 🔖️Views
