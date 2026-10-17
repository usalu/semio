/** 🎫️ Directory commands: the closed `DirectoryCommand` union, the command request and receipt contract and its error classification. */

import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility } from "../🟦️.ts";
import type { DirectoryEvent } from "../📜️event/🟦️.ts";
import { directoryEventPageObject } from "../📄️event-page/🟦️.ts";
import type { DocumentDescriptor } from "../👀️views/🟦️.ts";

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
//#endregion 🔖️CommandReceipt
