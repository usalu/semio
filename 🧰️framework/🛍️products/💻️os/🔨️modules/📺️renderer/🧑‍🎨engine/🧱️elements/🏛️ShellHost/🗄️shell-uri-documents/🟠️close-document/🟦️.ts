/** 🟠️ The close document phase of useShellUriAndDocuments. */
import { useCallback, useEffect, useMemo, useState } from "../../../⚛️react-port/🟦️.ts";
import { type BackboneWorkerRequest, encodeBackboneWorkerRequest, type PersistenceBinding } from "@semio-tech/framework/backbone";
import { syncAttachDocumentIdV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { requestBackboneFilePath, requestBackboneFolderPath } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { commitLocalFoldersConfigMutationV1, localFolderReconnectOfferV1, readLocalFolderBindingsV1, type LocalFolderIdentityV1 } from "../../📎️local-folders/🟦️.tsx";
import { attachLocalFolder, detachLocalFolder, type LocalFolderBinding, type LocalFolderBindings } from "@semio-tech/framework/config/mutations";
import { reduceBootstrapUiState, reduceExecutionTargetUiState, retainInferencePortOwnerAfterCloseV1 } from "../../🪪️host-bootstrap/🟦️.tsx";
import { type SyncAttachTargetV1 } from "../../../🔄️ShellSync/🟦️.tsx";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import { SPACE_INDEX_DOCUMENT_ID } from "../../🧾️space-artifact-creation/🟦️.ts";
import { tellSyncAttachFailureV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterApplyHostEffects } from "../../⚡️host-effects/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";
import type { useOpenDocument } from "../🔸️open-document/🟦️.ts";

/** 🟡 Builds closeDocument, localFolderBindings, setLocalFolderBindings, rememberLocalFolder of useShellUriAndDocuments. */
export function useCloseDocumentState({ openDocumentSessionsRef, cancelSpaceArtifactCreationsForRuntime, backboneWorkerRef, backgroundSpaceIndexSessionsRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, socketActorReadyRef, inferencePortOpeningRef, inferencePortOwnerRef, inferencePortAuthorityRef, inferencePortEpochRef, dispatch, retireDocumentAttachment, localDocumentOwnerRegistry, liveDialogRef, closeOwnedDialog, core, tutorialDrivenRef, retireBrowserActorUi, rebootstrapDiscardedSessionsRef, setInferenceHistoryByRuntimeKey, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument, directoryScopedOwnersRef, spaceDirectoryHistoriesRef, closeDocumentRef, scope, resolveSyncTargetSession, loadedPlugins, showTransientNoticeRef, openDocument, syncBackboneUri }: Pick<ReturnType<typeof useScopeState>, "openDocumentSessionsRef" | "cancelSpaceArtifactCreationsForRuntime" | "backboneWorkerRef" | "backgroundSpaceIndexSessionsRef" | "setSpaceArtifactCreationCatalog" | "setSpaceArtifactCreationCatalogUi" | "socketActorReadyRef" | "inferencePortOpeningRef" | "inferencePortOwnerRef" | "inferencePortAuthorityRef" | "inferencePortEpochRef" | "dispatch" | "retireDocumentAttachment" | "localDocumentOwnerRegistry" | "liveDialogRef" | "closeOwnedDialog" | "tutorialDrivenRef" | "retireBrowserActorUi" | "rebootstrapDiscardedSessionsRef" | "setInferenceHistoryByRuntimeKey" | "setPresencePeersByRuntimeKey" | "setBootstrapUiByDocument" | "setExecutionTargetUiByDocument" | "directoryScopedOwnersRef" | "spaceDirectoryHistoriesRef" | "closeDocumentRef" | "scope" | "resolveSyncTargetSession" | "loadedPlugins" | "showTransientNoticeRef" | "syncBackboneUri"> & { readonly core: ShellCoreAfterApplyHostEffects } & Pick<ReturnType<typeof useOpenDocument>, "openDocument">) {
  const closeDocument = useCallback((runtimeKey: string, clientInstanceId?: string) => {
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    if (!entry) return;
    if (clientInstanceId !== undefined && entry.clientInstanceId !== clientInstanceId) return;
    (window as unknown as { semioWgpuHubProjection?: { publishDocumentStatus: (documentKey: string, remote: null) => boolean } }).semioWgpuHubProjection?.publishDocumentStatus(runtimeKey, null);
    entry.creationMount?.close(new Error("document closed"));
    entry.rejectReady(new Error("document closed"));
    entry.replacements.invalidate();
    const retirement = entry.port?.retire();
    void retirement?.catch(error => undefined);
    entry.port = null;
    entry.pending = [];
    entry.pendingBytes = 0;
    cancelSpaceArtifactCreationsForRuntime(runtimeKey, backboneWorkerRef.current);
    if (entry.scope?.documentId === SPACE_INDEX_DOCUMENT_ID) {
      void backgroundSpaceIndexSessionsRef.current.retire(entry.scope.spaceId, owned => owned.clientInstanceId === entry.clientInstanceId).catch(error => undefined);
      setSpaceArtifactCreationCatalog((catalog) => catalog?.clientInstanceId === entry.clientInstanceId ? null : catalog);
      setSpaceArtifactCreationCatalogUi((status) => status?.clientInstanceId === entry.clientInstanceId ? null : status);
    }
    const waiter = socketActorReadyRef.current.get(runtimeKey);
    if (waiter?.clientInstanceId === entry.clientInstanceId) {
      waiter.reject(new Error("document closed"));
      socketActorReadyRef.current.delete(runtimeKey);
    }
    const pendingInference = inferencePortOpeningRef.current;
    if (pendingInference?.owner.runtimeKey === runtimeKey) pendingInference.mailbox.close("inference-opening: document retired");
    const inferenceOwner = inferencePortOwnerRef.current;
    const retainedInferenceOwner = retainInferencePortOwnerAfterCloseV1(inferenceOwner, runtimeKey);
    if (inferenceOwner !== null && retainedInferenceOwner === null) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: inferenceOwner.owner, serviceId: inferenceOwner.serviceId, action: "close", operationEpoch: inferenceOwner.operationEpoch, payload: {} }) });
      inferencePortOwnerRef.current = retainedInferenceOwner;
      inferencePortAuthorityRef.current = null;
      inferencePortEpochRef.current = inferenceOwner.operationEpoch + 1;
      dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey });
    }
    void retireDocumentAttachment(entry.plugin, entry.session.instanceId, entry.clientInstanceId).catch(error => undefined);
    localDocumentOwnerRegistry.retire({ pluginId: entry.session.pluginId, appId: entry.session.app.id, sessionInstanceId: entry.session.instanceId, runtimeKey, clientInstanceId: entry.clientInstanceId });
    openDocumentSessionsRef.current.delete(runtimeKey);
    const dialog = liveDialogRef.current;
    if (dialog?.origin.document?.runtimeKey === runtimeKey && dialog.origin.document.clientInstanceId === entry?.clientInstanceId) closeOwnedDialog(dialog.openingId);
    const tutorial = core.tutorialRunRef.current;
    if (tutorial?.origin.document?.runtimeKey === runtimeKey && tutorial.origin.document.clientInstanceId === entry?.clientInstanceId) {
      core.tutorialTransitionEpochRef.current += 1;
      tutorialDrivenRef.current.retire();
      core.tutorialClockRef.current?.pause();
      void tutorial.stop().catch((error) => undefined);
      dispatch({ type: "SET_TUTORIAL", value: null });
    }
    retireBrowserActorUi(runtimeKey, "browser-actor-action: document retired");
    rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
    setInferenceHistoryByRuntimeKey((current) => {
      if (!(runtimeKey in current)) return current;
      const next = { ...current };
      delete next[runtimeKey];
      return next;
    });
    setPresencePeersByRuntimeKey((current) => {
      if (!(runtimeKey in current)) return current;
      const next = { ...current };
      delete next[runtimeKey];
      return next;
    });
    setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "detached", documentId: entry.documentId, ...(entry.scope === undefined ? {} : { scope: entry.scope }) }));
    setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: entry.documentId, ...(entry.scope === undefined ? {} : { scope: entry.scope }) }));
    if (entry.scope !== undefined && directoryScopedOwnersRef.current.delete(runtimeKey)) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-scope-close", scope: entry.scope }) });
    }
    const spaceDirectory = spaceDirectoryHistoriesRef.current.get(runtimeKey);
    if (spaceDirectory !== undefined) {
      spaceDirectoryHistoriesRef.current.delete(runtimeKey);
      if (![...spaceDirectoryHistoriesRef.current.values()].some((history) => history.spaceId === spaceDirectory.spaceId)) backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-space-close", spaceId: spaceDirectory.spaceId }) });
    }
    const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }) };
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, [localDocumentOwnerRegistry, retireBrowserActorUi]);
  closeDocumentRef.current = closeDocument;

  /** 🔗️ Opens the sync card's typed target ({@link SyncAttachTargetV1}) through {@link openDocument}: a hub space's
   * document, a folder, or a file's folder, as the session app's document.
   *
   * 🪪️ A hub binding names the surface it wants — the store worker refuses a hub binding that carries neither
   * `requestedSurfaceId` nor a previously installed target (`socket-actor-failed … installed-target-unavailable`) — the
   * session app's own canonical one, the same expression `openDocument` resolves bindings with.
   *
   * 📐️ The schema is the artifact's OWN (`AppDefinition.io.artifactSchema`), not the chrome breadcrumb: a hub plan
   * states the document's schema and `documentOpenPlanAuthority` refuses an open that differs (the gis map answered
   * `document open: authority mismatch` against the breadcrumb `"semio.gis.2d"`). The breadcrumb stays the fallback
   * for an app that declares no artifact io. */
  /** 📁️ This device's remembered folder bindings (`os.config.local-folders`, an event-sourced config facet persisted local-only
   * in this shell's own storage: never shared, never in a URL), with the attach and detach that record them. */
  const [localFolderBindings, setLocalFolderBindings] = useState<LocalFolderBindings>(() => readLocalFolderBindingsV1(scope.storage));
  const rememberLocalFolder = useCallback((binding: LocalFolderBinding) => setLocalFolderBindings(commitLocalFoldersConfigMutationV1(scope.storage, attachLocalFolder(binding))), [scope.storage]);
  const forgetLocalFolder = useCallback((documentId: string) => setLocalFolderBindings(commitLocalFoldersConfigMutationV1(scope.storage, detachLocalFolder(documentId))), [scope.storage]);

  /** 🧷️ Attaches the focused program's document to a folder, a file or a hub. The document is addressed by the program's
   * own document identity — the id its store stamps on every envelope it publishes — so the actor admits the program's
   * edits as its own document's instead of refusing each batch as another document's (`local.backbone-scope-mismatch`,
   * e2e R2-4); a program without a document has nothing to attach. */
  const openSyncTarget = useCallback(
    async (target: SyncAttachTargetV1) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      const identity = (target.kind === "remote" && target.documentId !== null) || plugin === undefined ? null : await plugin.readAppDocumentIdentity(targetSession.instanceId);
      const documentId = syncAttachDocumentIdV1(target, identity);
      if (documentId === null) {
        showTransientNoticeRef.current(shellLabel("ui.sync.documentUnidentified"), "warning", "sync.attach.document-unidentified");
        return;
      }
      const folderPath = target.kind === "folder" ? target.path : target.kind === "file" ? target.folder : null;
      const requestedSurfaceId = targetSession.app.dialect ? canonicalSurfaceId(targetSession.app.dialect, targetSession.app.role) : undefined;
      const bindings: PersistenceBinding[] =
        target.kind === "remote"
          ? [{ kind: "hub", dataClass: "persistedShared", baseUrl: `http://${target.hostPort}`, spaceId: target.spaceId, ...(requestedSurfaceId === undefined ? {} : { requestedSurfaceId }) }]
          : [{ kind: "folder", dataClass: "persistedLocalOnly", path: target.kind === "folder" ? target.path : target.folder }];
      const appArtifactSchema = (targetSession.app as unknown as { readonly io?: { readonly artifactSchema?: string } }).io?.artifactSchema;
      const receipt = await openDocument({ documentId, schema: appArtifactSchema && appArtifactSchema.length > 0 ? appArtifactSchema : targetSession.app.breadcrumb.join(".") }, bindings);
      if (receipt !== null && folderPath !== null) rememberLocalFolder({ documentId, pluginId: targetSession.pluginId, appId: targetSession.app.id, folder: { kind: "path", path: folderPath } });
    },
    [loadedPlugins, openDocument, rememberLocalFolder, resolveSyncTargetSession],
  );
  /** 📎️ Attaches the focused program's document to a sync target as the person's gesture. Every gesture goes through here —
   * the card's Attach, the attach action, the reconnect offer — so a failure is told and logged
   * ({@link tellSyncAttachFailureV1}) and never dropped as an unhandled rejection (live fault F9: the card's `onAttach`
   * dropped the promise, so a document that did not attach said nothing). */
  const attachSyncTarget = useCallback((target: SyncAttachTargetV1): Promise<void> => openSyncTarget(target).catch(tellSyncAttachFailureV1(showTransientNoticeRef.current)), [openSyncTarget]);

  /** ✂️ Detaches the attached document; a local folder binding is forgotten with it, so a reload offers nothing back. */
  const detachSyncBackbone = useCallback(() => {
    if (syncBackboneUri) {
      const runtimeKey = syncBackboneUri.replace(/^actor:\/\//, "");
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (entry !== undefined && entry.scope === undefined && entry.persistenceBindings.some((binding) => binding.kind === "folder")) forgetLocalFolder(entry.documentId);
      closeDocument(runtimeKey);
    }
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [closeDocument, forgetLocalFolder, syncBackboneUri]);

  /** 🪪️ The document identity the focused program holds, read once per program (retried while its channel comes up): the
   * key a remembered folder binding is offered back under. */
  const syncTargetProgram = resolveSyncTargetSession();
  const [syncProgramIdentity, setSyncProgramIdentity] = useState<LocalFolderIdentityV1 | null>(null);
  useEffect(() => {
    const pluginId = syncTargetProgram?.pluginId;
    const instanceId = syncTargetProgram?.instanceId;
    const appId = syncTargetProgram?.app.id;
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle;
    setSyncProgramIdentity(null);
    if (pluginId === undefined || instanceId === undefined || appId === undefined || plugin === undefined) return;
    let live = true;
    let timer: ReturnType<typeof setTimeout> | null = null;
    const read = (attempt: number): void => {
      void plugin.readAppDocumentIdentity(instanceId).then((identity) => {
        if (live) setSyncProgramIdentity(identity.parent_document_id === null ? null : { documentId: identity.parent_document_id, pluginId, appId });
      }, () => {
        if (live && attempt < 10) timer = setTimeout(() => read(attempt + 1), 1_000);
      });
    };
    read(0);
    return () => {
      live = false;
      if (timer !== null) clearTimeout(timer);
    };
  }, [syncTargetProgram?.pluginId, syncTargetProgram?.instanceId, syncTargetProgram?.app.id, loadedPlugins]);
  /** 📂️ The remembered folder offered back for the focused program's document, while that document is not attached. */
  const folderReconnectOffer = useMemo(
    () => localFolderReconnectOfferV1(localFolderBindings, syncProgramIdentity, new Set([...openDocumentSessionsRef.current.values()].filter((entry) => entry.scope === undefined).map((entry) => entry.documentId))),
    [localFolderBindings, syncProgramIdentity, syncBackboneUri],
  );
  const [folderReconnectBusy, setFolderReconnectBusy] = useState(false);
  /** 🔁️ The person's own gesture reattaches the remembered folder; its archive is restored like a fresh load. */
  const reconnectLocalFolder = useCallback((binding: LocalFolderBinding) => {
    setFolderReconnectBusy(true);
    void attachSyncTarget({ kind: "folder", path: binding.folder.path }).finally(() => setFolderReconnectBusy(false));
  }, [attachSyncTarget]);

  const browseSyncBackbonePath = useCallback(async (kind: "file" | "folder") => {
    const path = kind === "folder" ? await requestBackboneFolderPath() : await requestBackboneFilePath();
    if (path) dispatch({ type: "SET_SYNC_DRAFT_PATH", value: path });
  }, []);
  return { closeDocument, forgetLocalFolder, attachSyncTarget, detachSyncBackbone, folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, browseSyncBackbonePath };
}
