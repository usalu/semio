import type { DirectoryCommand } from "../../../../🟦️";
import type { DirectoryCommandReceiptV1, DirectoryCommandRequestV1 } from "@semio-tech/framework/directory/schema";
import { directoryAdministrationCommandAllowedV1, sealDirectoryCommandRequestV1, directoryCommandRequestJson } from "@semio-tech/framework/directory/io/text";
import { completeDirectoryAdministrationDeletion, directoryAdministration, directoryAdministrationPageTermination, directoryClient, directoryCommandErrorCode, directoryRejectionStatus, liveDirectoryAdministration, loadDirectoryAdministrationPage, postDirectoryAdministrationState, terminateDirectoryAdministration } from "../🔐️hub-session/🟦️.ts";
import { post } from "../🧾️document-state/🟦️.ts";

/** ⛔️ Confirms deletion only after the exact space page is missing. A surviving page is not success. */
async function confirmDirectoryAdministrationDeletion(operationEpoch: number, receipt: DirectoryCommandReceiptV1): Promise<void> {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  const client = directoryClient;
  if (client === null) {
    terminateDirectoryAdministration(operation, "failed", "transport");
    return;
  }
  const read = new AbortController();
  operation.pageRead?.abort(new Error("directory administration deletion confirmation"));
  operation.pageRead = read;
  try {
    await client.spaceAdministrationPage(operation.spaceId, undefined, { signal: read.signal });
    const live = liveDirectoryAdministration(operationEpoch);
    if (live === null || live !== operation || operation.pageRead !== read) return;
    terminateDirectoryAdministration(operation, "failed", "invalid");
  } catch (error) {
    if (operation.closed || operation.pageRead !== read) return;
    if (read.signal.aborted) {
      terminateDirectoryAdministration(operation, "cancelled", "cancelled");
      return;
    }
    if (directoryRejectionStatus(error) === 404) {
      completeDirectoryAdministrationDeletion(operation, receipt);
      return;
    }
    const termination = directoryAdministrationPageTermination(error, false);
    terminateDirectoryAdministration(operation, termination.phase, termination.code);
  } finally {
    if (operation.pageRead === read) operation.pageRead = null;
  }
}

/** 📮️ Submits exactly one administration command and advances only on an exact server receipt.
 * The command is never retried: an indeterminate transport is terminal `failed`, so a `create-invite`
 * can never mint a second invite behind the operator's back. */
export async function submitDirectoryAdministrationCommand(operationEpoch: number, requestId: string, command: DirectoryCommand): Promise<void> {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  if (operation.requestId !== null || operation.phase === "submitting") {
    postDirectoryAdministrationState(operation, "capacity");
    return;
  }
  if (operation.phase !== "ready" || !directoryAdministrationCommandAllowedV1(operation.page, operation.spaceId, command)) {
    postDirectoryAdministrationState(operation, "invalid");
    return;
  }
  const client = directoryClient;
  if (client === null) {
    terminateDirectoryAdministration(operation, "failed", "transport");
    return;
  }
  let request: DirectoryCommandRequestV1;
  try {
    request = sealDirectoryCommandRequestV1(requestId, command);
    directoryCommandRequestJson(request);
  } catch {
    postDirectoryAdministrationState(operation, "invalid");
    return;
  }
  operation.requestId = requestId;
  operation.phase = "submitting";
  postDirectoryAdministrationState(operation);
  let receipt: DirectoryCommandReceiptV1;
  try {
    receipt = await client.command(request, { signal: operation.abort.signal });
  } catch (error) {
    if (operation.closed) return;
    const code = directoryCommandErrorCode(error, operation.abort.signal.aborted);
    if (code === "unauthorized" || code === "forbidden") {
      terminateDirectoryAdministration(operation, "denied", code);
      return;
    }
    if (code === "stale-session") {
      terminateDirectoryAdministration(operation, "stale", code);
      return;
    }
    terminateDirectoryAdministration(operation, code === "cancelled" ? "cancelled" : "failed", code);
    return;
  }
  const live = liveDirectoryAdministration(operationEpoch);
  if (live === null || live !== operation) return;
  if (command.kind === "delete-space") {
    if ((receipt.outcome !== "accepted" && receipt.outcome !== "previously-accepted") || receipt.result.kind !== "none") {
      terminateDirectoryAdministration(operation, "failed", "invalid");
      return;
    }
    operation.receiptSha256 = receipt.receiptSha256;
    operation.outcome = receipt.outcome;
    operation.inviteToken = null;
    operation.inviteCapabilityStatus = null;
    operation.inviteTransferEpoch = null;
    await confirmDirectoryAdministrationDeletion(operationEpoch, receipt);
    return;
  }
  operation.requestId = null;
  operation.receiptSha256 = receipt.receiptSha256;
  operation.outcome = receipt.outcome;
  operation.inviteToken = receipt.result.kind === "invite" ? receipt.result.inviteToken : null;
  operation.inviteCapabilityStatus = operation.inviteToken === null ? null : "available";
  operation.inviteTransferEpoch = null;
  operation.phase = "receipt";
  postDirectoryAdministrationState(operation);
  await loadDirectoryAdministrationPage(operationEpoch, undefined, "refreshing");
}

/** 🎁️ Offers the worker-retained capability for one operation-bound clipboard attempt. */
export function requestDirectoryAdministrationCapability(operationEpoch: number): void {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  if (!operation.authorPage) {
    operation.inviteToken = null;
    operation.inviteCapabilityStatus = null;
    operation.inviteTransferEpoch = null;
    post({ kind: "directory-administration-capability-rejected", operationEpoch, code: "already-settled" });
    return;
  }
  const inviteToken = operation.inviteToken;
  if (inviteToken === null) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, code: "already-settled" });
    return;
  }
  if (operation.inviteTransferEpoch !== null) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, transferEpoch: operation.inviteTransferEpoch, code: "capacity" });
    return;
  }
  const transferEpoch = operation.nextInviteTransferEpoch + 1;
  operation.nextInviteTransferEpoch = transferEpoch;
  operation.inviteTransferEpoch = transferEpoch;
  operation.inviteCapabilityStatus = "copying";
  postDirectoryAdministrationState(operation);
  post({ kind: "directory-administration-capability", operationEpoch, transferEpoch, inviteToken });
}

/** 📋️ Erases the capability only after an exact successful clipboard result; failure keeps it retryable. */
export function settleDirectoryAdministrationCapability(operationEpoch: number, transferEpoch: number, copied: boolean): void {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  if (operation.inviteTransferEpoch === null) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, transferEpoch, code: "already-settled" });
    return;
  }
  if (!Number.isSafeInteger(transferEpoch) || transferEpoch <= 0 || transferEpoch !== operation.inviteTransferEpoch) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, transferEpoch, code: "mismatch" });
    return;
  }
  operation.inviteTransferEpoch = null;
  if (copied) {
    operation.inviteToken = null;
    operation.inviteCapabilityStatus = null;
  } else {
    operation.inviteCapabilityStatus = "failed";
  }
  postDirectoryAdministrationState(operation);
}

/** 🧯️ Retires the retained operation when its exact space loses membership through a scoped 4401.
 * Any other space's revocation leaves the operation untouched — one revoked document scope is not a
 * reason to tear down administration of an unrelated space. */
export function revokeDirectoryAdministrationForScope(spaceId: string): void {
  const operation = directoryAdministration;
  if (operation === null || operation.closed || operation.spaceId !== spaceId) return;
  terminateDirectoryAdministration(operation, "denied", "forbidden");
}

/** 🛑️ Renderer unmount: cancel, erase, and report `cancelled` without touching any other lane. */
export function closeDirectoryAdministration(operationEpoch: number): void {
  const operation = directoryAdministration;
  if (operation === null || operation.operationEpoch !== operationEpoch) return;
  terminateDirectoryAdministration(operation, "cancelled", "cancelled");
}
