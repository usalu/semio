/** 🌱️ The space artifact creation catalog refresh, creation status and document check-in status messages of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { isTerminalDocumentCheckInPhaseV1 } from "@semio-tech/framework/document/check-in";
import { type BackboneWorkerResponse, encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { reduceArtifactCreationProgressUiV1 } from "../../🌱️artifact-creation/🟦️.tsx";
import { captureSpaceArtifactCreationCatalogAuthorityV1, SPACE_INDEX_DOCUMENT_ID, spaceArtifactCreationCatalogRefreshRequestV1, spaceArtifactCreationOwnerAcceptsStatus, type SpaceArtifactCreationOwnerV1, spaceArtifactCreationReadyOpening } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";

/** 🔄️ Requests a fresh creation catalog when the retained one lost its authority. */
export function handleSpaceArtifactCreationCatalogRefreshRequiredMessage({ spaceArtifactCreationOwnersRef, captureDialogOrigin, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, message, core, worker }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "spaceArtifactCreationOwnersRef" | "captureDialogOrigin" | "spaceArtifactCreationCatalogRef" | "spaceArtifactCreationCatalogUiRef" | "setSpaceArtifactCreationCatalog" | "setSpaceArtifactCreationCatalogUi"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "space-artifact-creation-catalog-refresh-required" }>>;
  readonly core: ShellCoreAfterIdentityAndDocumentPorts;
  readonly worker: Worker;
}) {
  const owner = spaceArtifactCreationOwnersRef.current.get(message.requestId) ?? null;
  const origin = captureDialogOrigin(core.shellStateRef.current.pluginRuntime.session);
  const authority = captureSpaceArtifactCreationCatalogAuthorityV1(
    spaceArtifactCreationCatalogRef.current,
    spaceArtifactCreationCatalogUiRef.current,
    origin,
  );
  const request = spaceArtifactCreationCatalogRefreshRequestV1(owner, authority, origin, message);
  if (request === null) return;
  const loading = {
    kind: "space-artifact-creation-catalog-status",
    clientInstanceId: request.clientInstanceId,
    spaceId: request.spaceId,
    phase: "loading",
  } as const;
  spaceArtifactCreationCatalogRef.current = null;
  spaceArtifactCreationCatalogUiRef.current = loading;
  setSpaceArtifactCreationCatalog(null);
  setSpaceArtifactCreationCatalogUi(loading);
  worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  return;
}

/** 📥️ Folds a document check-in status into the owner's ui state. */
export function handleDocumentCheckInStatusMessage({ openDocumentSessionsRef, core, message }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef"> & {
  readonly core: ShellCoreAfterIdentityAndDocumentPorts;
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "document-check-in-status" }>>;
}) {
  const owner = core.documentCheckInOwnersRef.current.get(message.status.requestId);
  const entry = owner === undefined ? undefined : openDocumentSessionsRef.current.get(owner.runtimeKey);
  if (owner === undefined || entry === undefined || entry.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId) return;
  if (isTerminalDocumentCheckInPhaseV1(message.status.phase)) core.documentCheckInOwnersRef.current.delete(message.status.requestId);
  core.setDocumentCheckInUi((current) => ({ ...current, [owner.runtimeKey]: { requestId: message.status.requestId, scope: message.scope, status: message.status } }));
  return;
}

/** 🏗️ Folds an artifact creation status into its owner and opens the ready artifact once. */
export function handleSpaceArtifactCreationStatusMessage({ spaceArtifactCreationOwnersRef, openDocumentSessionsRef, setSpaceArtifactCreationUi, openReadySpaceArtifactCreationRef, message, worker }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "spaceArtifactCreationOwnersRef" | "openDocumentSessionsRef" | "setSpaceArtifactCreationUi" | "openReadySpaceArtifactCreationRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "space-artifact-creation-status" }>>;
  readonly worker: Worker;
}) {
  const owner = spaceArtifactCreationOwnersRef.current.get(message.requestId);
  if (owner === undefined || !spaceArtifactCreationOwnerAcceptsStatus(owner, message)) return;
  const entry = openDocumentSessionsRef.current.get(owner.runtimeKey);
  if (
    entry === undefined
    || entry.clientInstanceId !== owner.clientInstanceId
    || entry.session.instanceId !== owner.sessionInstanceId
    || entry.scope?.spaceId !== owner.spaceId
    || entry.scope.documentId !== SPACE_INDEX_DOCUMENT_ID
  ) {
    spaceArtifactCreationOwnersRef.current.delete(owner.requestId);
    setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "cleared", requestId: owner.requestId }));
    if (message.phase !== "ready") worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId: owner.requestId, spaceId: owner.spaceId }) });
    return;
  }
  setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "status", message }));
  if (message.phase === "failed" || message.phase === "cancelled" || message.phase === "indeterminate") {
    spaceArtifactCreationOwnersRef.current.delete(owner.requestId);
    return;
  }
  const openingArgs = spaceArtifactCreationReadyOpening(message);
  if (openingArgs === null || owner.opening) return;
  const firstReady = owner.ready === null;
  const readyOwner: SpaceArtifactCreationOwnerV1 = firstReady ? { ...owner, ready: message } : owner;
  spaceArtifactCreationOwnersRef.current.set(owner.requestId, readyOwner);
  if (firstReady) openReadySpaceArtifactCreationRef.current(owner.requestId);
  return;
}
