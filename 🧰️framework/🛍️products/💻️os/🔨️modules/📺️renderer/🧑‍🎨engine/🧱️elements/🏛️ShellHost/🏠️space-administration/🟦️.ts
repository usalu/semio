/** 🏠️ Shell space administration state, capability check, reducer, request builder and the directory invite clipboard copy.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { directoryAdministrationCommandAllowedV1 } from "../../../../../📇️directory/🚪️io/📝️text/🟦️.ts";
// #region 🔌️Adapters
import {
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type DirectoryAdministrationPhaseV1,
  type DirectoryAdministrationInviteCapabilityStatusV1,
  type DirectoryCommand,
  type DirectoryCommandErrorCodeV1,
  type DirectorySpaceAdministrationPageV1
} from "@semio-tech/framework-os";
// #endregion 🔌️Adapters
import {
  spaceAdministrationCapabilities,
  spaceAdministrationInviteRevocable,
  spaceAdministrationMemberRemovable,
  spaceAdministrationNameValid,
  type SpaceAdministrationIntentV1
} from "../../🛂️SpaceAdministration/🟦️.tsx";

//#region FrameworkOsShell
//#region 🐚️ShellMount
//#region 🔖️Identity
//#region 🏛️SpaceAdministration
/** 🏛️ The renderer-visible state of the one retained administration operation. `page` is the parsed
 * form of the exact canonical bytes the hub sealed; it is never merged, patched, or folded, only
 * replaced wholesale by a newer page. */
export interface ShellSpaceAdministrationStateV1 {
  readonly operationEpoch: number;
  readonly spaceId: string;
  readonly phase: DirectoryAdministrationPhaseV1;
  readonly page: DirectorySpaceAdministrationPageV1 | null;
  readonly canonicalJson?: string;
  readonly receiptSha256?: string;
  readonly code?: DirectoryCommandErrorCodeV1;
  readonly inviteCapabilityPending?: boolean;
  readonly inviteCapabilityStatus?: DirectoryAdministrationInviteCapabilityStatusV1;
}

const SHELL_SPACE_ADMINISTRATION_TERMINAL: readonly DirectoryAdministrationPhaseV1[] = ["cancelled", "denied", "stale", "failed"];

/** 🏛️ The exact Home-effect bridge into one retained canonical administration operation. */
export function shellSpaceAdministrationOpening(
  actionId: string,
  args: Readonly<Record<string, unknown>> | undefined,
  operationEpoch: number,
): { readonly state: ShellSpaceAdministrationStateV1; readonly request: BackboneWorkerRequest } | null {
  const spaceId = args?.spaceId;
  if (actionId !== "os.directory.open-administration" || typeof spaceId !== "string" || spaceId.length === 0 || new TextEncoder().encode(spaceId).byteLength > 256 || !Number.isSafeInteger(operationEpoch) || operationEpoch < 0) return null;
  return {
    state: { operationEpoch, spaceId, phase: "loading", page: null },
    request: { kind: "directory-administration-open", operationEpoch, spaceId },
  };
}

/** 🎟️ Accepts a worker capability only for the exact currently rendered author page. */
export function shellSpaceAdministrationCapabilityAllowed(state: ShellSpaceAdministrationStateV1 | null, operationEpoch: number): boolean {
  return state !== null
    && state.operationEpoch === operationEpoch
    && state.phase === "ready"
    && state.page?.access === "author"
    && state.inviteCapabilityPending === true;
}

/** 🧮️ Pure reducer over one worker administration message. A message for a superseded operation is
 * ignored wholesale; a terminal phase erases the page, receipt, and capability marker in the same
 * transition, so no renderer can read stale authority after a denial or an identity change. */
export function reduceShellSpaceAdministrationState(
  current: ShellSpaceAdministrationStateV1 | null,
  message: Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }>,
  operationEpoch: number,
  page: DirectorySpaceAdministrationPageV1 | null,
): ShellSpaceAdministrationStateV1 | null {
  if (message.operationEpoch !== operationEpoch) return current;
  if (current === null || current.operationEpoch !== operationEpoch || current.spaceId !== message.spaceId) return current;
  if (message.phase === "deleted") {
    if ((message.outcome !== "accepted" && message.outcome !== "previously-accepted") || message.receiptSha256 === undefined || !/^[0-9a-f]{64}$/u.test(message.receiptSha256) || message.receiptSha256 === "0".repeat(64)) return current;
    return { operationEpoch, spaceId: message.spaceId, phase: "deleted", page: null, receiptSha256: message.receiptSha256 };
  }
  if (SHELL_SPACE_ADMINISTRATION_TERMINAL.includes(message.phase)) {
    return { operationEpoch, spaceId: message.spaceId, phase: message.phase, page: null, ...(message.code === undefined ? {} : { code: message.code }) };
  }
  return {
    operationEpoch,
    spaceId: message.spaceId,
    phase: message.phase,
    page,
    ...(message.canonicalJson === undefined ? {} : { canonicalJson: message.canonicalJson }),
    ...(message.receiptSha256 === undefined ? {} : { receiptSha256: message.receiptSha256 }),
    ...(message.code === undefined ? {} : { code: message.code }),
    ...(message.inviteCapabilityPending === true ? { inviteCapabilityPending: true } : {}),
    ...(message.inviteCapabilityStatus === undefined ? {} : { inviteCapabilityStatus: message.inviteCapabilityStatus }),
  };
}

/** 🎬️ Maps one pane intent onto the retained operation's worker request. Returns `null` for an
 * intent the current page does not authorize, so an unauthorized control can never reach the wire
 * even if a hostile renderer synthesizes the event. */
export function shellSpaceAdministrationRequest(
  state: ShellSpaceAdministrationStateV1,
  intent: SpaceAdministrationIntentV1,
  requestId: string,
): BackboneWorkerRequest | null {
  const operationEpoch = state.operationEpoch;
  if (intent.kind === "close") return { kind: "directory-administration-close", operationEpoch };
  if (intent.kind === "page") return { kind: "directory-administration-refresh", operationEpoch, cursor: intent.cursor };
  const capabilities = spaceAdministrationCapabilities(state.page);
  if (capabilities === null || state.phase !== "ready" || state.page === null || state.page.access !== "author" || state.page.spaceId !== state.spaceId || state.page.space.id !== state.spaceId) return null;
  if (intent.kind === "copy-invite-capability") return state.inviteCapabilityPending === true ? { kind: "directory-administration-capability-request", operationEpoch } : null;
  const spaceId = state.spaceId;
  const submit = (command: DirectoryCommand): BackboneWorkerRequest | null => directoryAdministrationCommandAllowedV1(state.page, spaceId, command) ? { kind: "directory-administration-submit", operationEpoch, requestId, command } : null;
  if (intent.kind === "delete-space") return submit({ kind: "delete-space", spaceId });
  if (intent.kind === "rename-space") {
    if (!spaceAdministrationNameValid(intent.name)) return null;
    return submit({ kind: "rename-space", spaceId, name: intent.name });
  }
  if (intent.kind === "set-visibility") {
    if (intent.visibility !== "public" && intent.visibility !== "private") return null;
    return submit({ kind: "set-visibility", spaceId, visibility: intent.visibility });
  }
  if (intent.kind === "set-role") {
    const row = state.page.members.rows.find((member) => member.userId === intent.userId);
    if (row === undefined) return null;
    return submit({ kind: "upsert-member", spaceId, email: row.email, role: intent.role });
  }
  if (intent.kind === "remove-member") {
    const row = state.page.members.rows.find((member) => member.userId === intent.userId);
    if (row === undefined || !spaceAdministrationMemberRemovable(row, capabilities)) return null;
    return submit({ kind: "remove-member", spaceId, userId: intent.userId });
  }
  if (intent.kind === "create-invite") {
    return submit({ kind: "create-invite", spaceId, role: intent.role, ttlSecs: SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS });
  }
  const row = state.page.invites.rows.find((invite) => invite.inviteId === intent.inviteId);
  if (row === undefined || !spaceAdministrationInviteRevocable(row, capabilities)) return null;
  return submit({ kind: "revoke-invite", spaceId, inviteId: intent.inviteId });
}

/** ⏳️ One hour: the one invite lifetime the administration pane issues, stated once here rather
 * than smuggled through a renderer-supplied number. */
export const SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS = 3600;

/** 📋️ Minimal clipboard boundary; callers can prove unavailable and rejected writes without
 * exporting a browser implementation type. */
export interface DirectoryInviteClipboardV1 {
  writeText(text: string): Promise<void>;
}

/** 📋️ Writes the one-shot invite capability to the clipboard. The worker remains its owner until
 * the exact transfer result returns; the token is never logged, folded, put in a URL, or React state. */
export async function copyDirectoryInviteCapabilityV1(
  inviteToken: string,
  clipboard: DirectoryInviteClipboardV1 | undefined = globalThis.navigator?.clipboard,
): Promise<boolean> {
  if (clipboard === undefined) return false;
  try {
    await clipboard.writeText(inviteToken);
    return true;
  } catch {
    return false;
  }
}
//#endregion 🏛️SpaceAdministration
//#endregion 🔖️Identity
//#endregion 🐚️ShellMount
//#endregion FrameworkOsShell
