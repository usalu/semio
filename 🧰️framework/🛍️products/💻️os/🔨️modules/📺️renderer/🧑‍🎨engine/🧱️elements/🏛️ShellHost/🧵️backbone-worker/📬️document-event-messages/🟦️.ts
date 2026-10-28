/** 📬️ The local catalog, identity and document event messages of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { decodePackValue } from "@semio-tech/framework/packs";
import { decodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { DocumentArchiveFaultError } from "@semio-tech/framework/app-channel";
import { type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { scopedPresencePeersV1, type ScopedPresencePeerV1 } from "../../👥️presence-scope/🟦️.ts";
import { publishArtifactPresenceRosterV1 } from "../../../👕️canvas-presence/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionV1 } from "../../🪟️spawned-program/🟦️.ts";
import { foldIdentityEvent } from "@semio-tech/framework/backbone/config-lane";
import { decodeLocalCatalogArchiveV1, foldLocalCatalogEnvelopesV1 } from "../../🗂️local-catalog/🟦️.ts";
import { decodeIdentityValue } from "@semio-tech/framework/config/mutations";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { beginDocumentSourceWorkV1 } from "../../../🛠️ShellHelpers/📝️source-revision/document.ts";
import { documentTransferNoticeTextV1, documentLoadCancelledV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { commandRejectionNoticeV1, appFaultNoticeV1, historyLaneNoticeV1, historyRefusalCodeV1, historyRefusalNoticeV1 } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { resolveManifestLabel, shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { reduceBootstrapUiState, type BootstrapUiState } from "../../🪪️host-bootstrap/🟦️.tsx";
import { canonicalSurfaceId, scopeRuntimeKey } from "../../🔐️identity-binding/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";
import type { Dispatch, RefObject, SetStateAction } from "../../../⚛️react-port/🟦️.ts";

/** 🗃️ Applies a local catalog document event. */
export function handleLocalCatalogEventMessage({ localCatalogClientInstanceIdRef, setLocalCatalog, message }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "localCatalogClientInstanceIdRef" | "setLocalCatalog"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "event" }>>;
}) {
  if (localCatalogClientInstanceIdRef.current !== message.clientInstanceId) return;
  const catalogEvent = message.event;
  if (catalogEvent.kind === "documentArchiveReplaced") {
    const decoded = decodeLocalCatalogArchiveV1(new Uint8Array(catalogEvent.archive));
    if (decoded !== undefined) setLocalCatalog(decoded);
  } else if (catalogEvent.kind === "remoteMutations") {
    setLocalCatalog((current) => foldLocalCatalogEnvelopesV1(current, catalogEvent.envelopes));
  }
  return;
}

/** 🪪️ Applies an identity document event. */
export function handleIdentityEventMessage({ identityClientInstanceIdRef, verifiedSessionAuthorityRef, setIdentity, identitySnapshotResolverRef, sessionRefreshRef, message }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "identityClientInstanceIdRef" | "verifiedSessionAuthorityRef" | "setIdentity" | "identitySnapshotResolverRef" | "sessionRefreshRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "event" }>>;
}) {
  if (identityClientInstanceIdRef.current !== message.clientInstanceId) return;
  const identityEvent = message.event;
  if (identityEvent.kind === "documentArchiveReplaced") {
    const archive = decodeDocumentArchiveBytes(new Uint8Array(identityEvent.archive));
    const decoded = decodeIdentityValue(decodePackValue(new Uint8Array(archive.parent_pack)));
    if (decoded !== undefined) {
      if (verifiedSessionAuthorityRef.current === null) setIdentity(decoded);
      identitySnapshotResolverRef.current?.(decoded);
      identitySnapshotResolverRef.current = null;
    }
  } else if (identityEvent.kind === "remoteMutations") {
    if (verifiedSessionAuthorityRef.current === null) setIdentity((current) => foldIdentityEvent(current, identityEvent));
    void sessionRefreshRef.current?.refresh();
  }
  return;
}

/** 📨️ Routes a document event to its open session: status, presence, backbone, archive, command outcome and conflict. */
export function handleDocumentEventMessage({ openDocumentSessionsRef, dispatch, receiveDocumentBackbone, uiTerminologyRef, uiLocaleRef, showTransientNoticeRef, closeDocumentRef, documentArchivePorts, failDocumentBackbone, historyStore, sessionRef, message, setPresencePeersByRuntimeKey, core, rebootstrapDiscardedSessionsRef, setBootstrapUiByDocument }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef" | "dispatch" | "receiveDocumentBackbone" | "uiTerminologyRef" | "uiLocaleRef" | "showTransientNoticeRef" | "closeDocumentRef" | "documentArchivePorts" | "failDocumentBackbone" | "historyStore" | "sessionRef"> & {
  readonly message: BackboneWorkerResponse;
  readonly setPresencePeersByRuntimeKey: Dispatch<SetStateAction<Readonly<Record<string, readonly ScopedPresencePeerV1[]>>>>;
  readonly core: ShellCoreAfterIdentityAndDocumentPorts;
  readonly rebootstrapDiscardedSessionsRef: RefObject<Map<string, ActiveSession>>;
  readonly setBootstrapUiByDocument: Dispatch<SetStateAction<BootstrapUiState>>;
}) {
  if (message.kind !== "event") return;
  const runtimeKey = message.scope === undefined ? message.documentId : scopeRuntimeKey(message);
  if (runtimeKey === null) return;
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  if (!entry || entry.clientInstanceId !== message.clientInstanceId) return;
  const { event } = message;
  if (event.kind === "status") {
    const status = event;
    (window as unknown as { semioWgpuHubProjection?: { publishDocumentStatus: (documentKey: string, remote: typeof status.remote | null) => boolean } }).semioWgpuHubProjection?.publishDocumentStatus(runtimeKey, status.remote);
    dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: runtimeKey, status: { persisted: event.persisted, pendingMutations: event.pendingMutations, remote: event.remote } });
  } else if (event.kind === "presence") {
    const peers = entry.scope === undefined ? [] : scopedPresencePeersV1(message, entry.scope, entry.session.app.dialect ? canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role) : null);
    setPresencePeersByRuntimeKey((current) => {
      if (current[runtimeKey] === peers) return current;
      return { ...current, [runtimeKey]: peers };
    });
    publishArtifactPresenceRosterV1(runtimeKey, event.peers);
  } else if (event.kind === "documentBackbone") {
    receiveDocumentBackbone(runtimeKey, entry, event.message);
  } else if (event.kind === "documentArchiveReplaced") {
    const sourceWork = beginDocumentSourceWorkV1({
      owners: core.localDocumentOwnerRegistry, sources: core.localSourceRevisions,
      refresh: (plugin, instanceId) => core.localSourceRefreshRef.current(plugin, instanceId),
    }, { pluginId: entry.session.pluginId, appId: entry.session.app.id, sessionInstanceId: entry.session.instanceId, runtimeKey, clientInstanceId: entry.clientInstanceId }, entry.plugin);
    const archiveBytes = new Uint8Array(event.archive);
    void (async () => {
      try {
        const archive = decodeDocumentArchiveBytes(archiveBytes);
        const held = () => openDocumentSessionsRef.current.get(runtimeKey) === entry;
        const merge = async (value: DocumentArchivePack) => {
          if (!entry.plugin.mergeAppDocumentArchive) throw new Error("document-backbone.archive-merger-unavailable");
          return entry.plugin.mergeAppDocumentArchive(entry.session.instanceId, value);
        };
        const file = resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id;
        const detach = async (failure: Readonly<{ error: unknown }> | null): Promise<void> => {
          if (!held()) return;
          if (failure !== null) {
            console.error("[os-shell] a folder read-back could not be restored", message.documentId, failure.error instanceof Error ? failure.error.message : String(failure.error));
            const told = failure.error instanceof DocumentArchiveFaultError && failure.error.fault !== null ? appFaultNoticeV1(failure.error.fault, entry.session.app, uiTerminologyRef.current) : null;
            if (told === null) core.documentTransferRef.current.notifyDocumentTransfer("load-failed", file, "error");
            else showTransientNoticeRef.current(told.text, "error", told.code);
          }
          core.documentTransferRef.current.notifyDocumentTransfer("folder-detached", file, "info");
          closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
          if (core.shellStateRef.current.sync.syncBackboneUri === `actor://${runtimeKey}`) {
            dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
            dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
          }
        };
        const outcome = await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge, cancelled: documentLoadCancelledV1, detach });
        sourceWork?.admit(undefined);
        if (outcome === "cancelled" && held()) core.documentTransferRef.current.notifyDocumentTransfer("load-cancelled", file, "info");
        if (outcome !== "held") return;
        if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        const discarded = rebootstrapDiscardedSessionsRef.current.get(runtimeKey);
        if (discarded) {
          rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
          dispatch({ type: "SET_SESSION", value: (current) => current ?? discarded });
        }
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "snapshot-replaced", documentId: message.documentId, ...(message.scope === undefined ? {} : { scope: message.scope }) }));
      } catch (replacementError) {
        if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== message.clientInstanceId) return;
        console.error("[os-shell] a folder read-back could not be restored", message.documentId, replacementError instanceof Error ? replacementError.message : String(replacementError));
        if (entry.creationMount !== null) {
          failDocumentBackbone(runtimeKey, entry, replacementError);
          return;
        }
        entry.rejectReady(replacementError instanceof Error ? replacementError : new Error(String(replacementError)));
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, {
          kind: "artifact-bootstrap-failed",
          documentId: message.documentId,
          clientInstanceId: message.clientInstanceId,
          ...(message.scope === undefined ? {} : { scope: message.scope }),
          code: "invalid-bootstrap",
          message: (replacementError instanceof DocumentArchiveFaultError && replacementError.fault !== null ? appFaultNoticeV1(replacementError.fault, entry.session.app, uiTerminologyRef.current)?.text : undefined)
            ?? documentTransferNoticeTextV1("load-failed", resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id, uiLocaleRef.current),
          retryable: false,
        }));
      } finally {
        sourceWork?.finish();
      }
    })();
  } else if (event.kind === "documentArchiveAbsent") {
    void entry.readBack.absent().catch(error => failDocumentBackbone(runtimeKey, entry, error));
  } else if (event.kind === "commandOutcome") {
    if (event.outcome.kind === "rejected") {
      const notice = commandRejectionNoticeV1(event.outcome);
      console.warn("[os-shell] a command batch was refused", message.documentId, event.outcome.code, event.outcome.reason, event.outcome.detail ?? {});
      showTransientNoticeRef.current(notice.text, notice.kind, notice.code);
    }
    else if (event.outcome.kind === "transformed") showTransientNoticeRef.current(shellLabel("ui.conflict.hubTransformed"), "info", "sync.command.transformed");
  } else if (event.kind === "conflict") {
    console.warn("[os-shell] sync conflict", message.documentId, event.message);
    const refusal = historyRefusalCodeV1(event.code);
    const notice = refusal !== null ? historyRefusalNoticeV1(refusal) : historyLaneNoticeV1({ code: typeof event.code === "string" ? event.code : undefined }, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(sessionRef.current), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount);
    if (notice !== null) showTransientNoticeRef.current(notice.text, notice.kind, notice.code);
  }
}
