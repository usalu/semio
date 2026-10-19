/** 🗄️ `applyShellUri`, document open and close, local folders, sync targets and `spawnProgram`.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useShellUriAndDocuments` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import {
  FolderArchivePersistenceV1,
  FolderReadBackRouteV1,
  LatestDocumentReplacementV1,
  parkDocumentOpeningReplacementV1,
  runDocumentOpeningAttemptV1,
  settleDocumentOpeningReplacementV1,
  type DocumentOpeningReceiptV1
} from "../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { createArtifactCreationCatalogMountV1 } from "../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { useCallback, useEffect, useMemo, useState } from "react";
import { spaceIndexOf, resolveUiDirtyScope } from "@semio-tech/framework";
// #region 🔌️Adapters
import {
  type DocumentScope
} from "@semio-tech/framework/document/scope";
import {
  encodeDocumentArchiveBytes,
  type DocumentArchivePack
} from "@semio-tech/framework/wire-contracts";
import {
  type BackboneWorkerRequest,
  encodeBackboneWorkerRequest,
  type PersistenceBinding,
  documentRuntimeKeyV1
} from "@semio-tech/framework/backbone";
import { SpaceDirectoryHistoryV1 } from "../📇️space-directory/🟦️.ts";
import { shellSessionRouteV1 } from "../🔀️surface-switch/🟦️.ts";
import { spawnedProgramViewStateV1, spawnProgramRefusalCodeV1, spawnProgramRefusalNoticeTextV1, type SpawnProgramRefusalReasonV1 } from "../🪟️spawned-program/🟦️.ts";
import { LOCAL_CATALOG_ACTIONS_V1, localCatalogRehydrationArgumentsV1 } from "../🗂️local-catalog/🟦️.ts";
import { resolveDocumentOpeningBindings, resolveDocumentOpeningTarget, type DocumentOpeningReference } from "../🧭️opening/🟦️.ts";
import { type ActiveSession, type SpaceProgramEntry } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { buildSpacePanelState, parsePanelState, studioPanelFocusingSpawned } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { parseShellRoute } from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { syncAttachDocumentIdV1 } from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import {
  requestBackboneFilePath,
  requestBackboneFolderPath
} from "../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
// #endregion 🔌️Adapters
import { commitLocalFoldersConfigMutationV1, localFolderReconnectOfferV1, readLocalFolderBindingsV1, type LocalFolderIdentityV1 } from "../📎️local-folders/🟦️.tsx";
import { attachLocalFolder, detachLocalFolder, type LocalFolderBinding, type LocalFolderBindings } from "@semio-tech/framework/config/mutations";
import { documentOpeningDeadlineMs } from "@semio-tech/framework/browser-bundle/child/schema";
import { reduceBootstrapUiState, reduceExecutionTargetUiState, retainInferencePortOwnerAfterCloseV1 } from "../🪪️host-bootstrap/🟦️.tsx";
import { type SyncAttachTargetV1 } from "../../🔄️ShellSync/🟦️.tsx";
import { encodeWindowActionInvocation } from "../📨️window-invocation/🟦️.ts";
import { canonicalSurfaceId } from "../🔐️identity-binding/🟦️.ts";
import { findDialectApp, SPACE_INDEX_DOCUMENT_ID } from "../🧾️space-artifact-creation/🟦️.ts";
import { logUnlessRetiredV1, requiredHostPanelLeafId, tellSyncAttachFailureV1 } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterApplyHostEffects } from "../⚡️host-effects/🟦️.ts";
import type { OpenDocumentSessionTarget, OpenDocumentSession } from "../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";

export type ShellCoreAfterShellUriAndDocuments = ShellCoreAfterApplyHostEffects & ReturnType<typeof useShellUriAndDocuments>;

//#region FrameworkOsShell
export function useShellUriAndDocuments(core: ShellCoreAfterApplyHostEffects) {
  const {
    scope, localDocumentOwnerRegistry, hostConfig, hostMode, dispatch, loadedPlugins, session, hostPlugin, hostApp,
    landingAppId, hostAppId, activeUtilityByWindowId, activeToolId, extraWindowInstances, liveDialogRef, syncBackboneUri,
    openSpaceIdRef, rememberOpenSpaceId, openInstanceIdRef, sessionRef, applyShellUriRef, shellRouteLedgerRef,
    backboneWorkerRef, shellActorIdRef, identityRef, hubEnv, identityResolution, uiLocaleRef, showTransientNoticeRef,
    backgroundSpaceIndexSessionsRef, openDocumentRef, closeDocumentRef, documentAttachmentLane, retireDocumentAttachment,
    setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, cancelSpaceArtifactCreationsForRuntime,
    setHubWorkspaceOpen, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef,
    inferencePortOpeningRef, setInferenceHistoryByRuntimeKey, localCatalog, rehydratedLocalDocumentsRef,
    openDocumentSessionsRef, bindDocumentBackbone, captureDialogOrigin, captureEffectOwner, closeOwnedDialog,
    browserActorUiByRuntimeKeyRef, retireBrowserActorUi, directoryScopedOwnersRef, spaceDirectoryHistoriesRef,
    socketActorReadyRef, setBootstrapUiByDocument, setExecutionTargetUiByDocument, setPresencePeersByRuntimeKey,
    rebootstrapDiscardedSessionsRef, ensureBackboneWorker, readLocalDocumentRef, shellUri, sessionRouteRef,
    setSessionRoute, requestSessionRoute, panel, tutorialDrivenRef, postBrowserActorViewState, refreshUi,
    updateSpacePanel, switchToPluginApp, applyHostEffects
  } = core;
  const applyShellUri = useCallback(
    async (uri: string, preservedViewState?: ViewModel): Promise<boolean> => {
      const currentSession = sessionRef.current;
      if (!hostConfig || !currentSession || loadedPlugins.length === 0) return false;
      const path = uri.split("?")[0] ?? "/";
      const studioMatch = /^\/spaces\/([^/]+)\/studio(?:\/instances\/([^/]+))?$/.exec(path);
      const route = studioMatch ? ({ kind: "space" as const, spaceId: studioMatch[1]!, instanceId: studioMatch[2] } as const) : parseShellRoute(path);
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId)?.handle;
      if (!sPlugin) return false;
      if (route.kind === "landing") {
        rememberOpenSpaceId(null);
        openInstanceIdRef.current = null;
        if (!landingAppId) throw new Error("required landing app identity is unavailable");
        if (currentSession.app.id !== landingAppId) return (await switchToPluginApp(hostConfig.pluginId, landingAppId, preservedViewState)) !== null;
        return true;
      }
      if (route.kind === "notFound") {
        rememberOpenSpaceId(null);
        openInstanceIdRef.current = null;
        return true;
      }
      const { spaceId, instanceId } = route;
      if (!studioMatch && !instanceId) {
        const hostPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId);
        const spaceIndex = spaceIndexOf(hostPlugin?.manifest.topicContributions);
        const spaceApp = spaceIndex && (findDialectApp(hostPlugin, spaceIndex.dialect, "editor") ?? findDialectApp(hostPlugin, spaceIndex.dialect, "viewer"));
        if (!spaceIndex || !spaceApp || !hostPlugin) {
          console.warn("[os-shell] applyShellUri: the host plugin names no space index app (os.space-index) — not loaded yet");
          return false;
        }
        const spaceIndexAlreadyOpen = openSpaceIdRef.current === spaceId && currentSession.app.id === spaceApp.id;
        rememberOpenSpaceId(spaceId);
        openInstanceIdRef.current = null;
        if (spaceIndexAlreadyOpen) return true;
        const spaceSession = currentSession.app.id === spaceApp.id ? currentSession : await switchToPluginApp(hostConfig.pluginId, spaceApp.id, preservedViewState);
        if (!spaceSession) return false;
        void openDocumentRef.current({ documentId: SPACE_INDEX_DOCUMENT_ID, schema: spaceIndex.documentSchema, spaceId }, undefined, { session: spaceSession, plugin: hostPlugin.handle }).catch(logUnlessRetiredV1("[os-shell] space index opening failed"));
        return true;
      }
      const studioChanged = openSpaceIdRef.current !== spaceId;
      rememberOpenSpaceId(spaceId);
      if (!hostAppId) throw new Error("required host app identity is unavailable");
      const studioSession = currentSession.app.id === hostAppId ? currentSession : await switchToPluginApp(hostConfig.pluginId, hostAppId, preservedViewState);
      if (!studioSession) return false;
      const studioControllerId = studioSession.app.controllerId;
      const routeOwner = captureEffectOwner(studioSession, captureDialogOrigin(studioSession));
      if (studioChanged) {
        openInstanceIdRef.current = null;
        const openResponse = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "openSpace", args: { spaceId } }), studioSession.viewState);
        await applyHostEffects(openResponse.requestedEffects ?? [], studioSession, resolveUiDirtyScope(openResponse.uiScope), routeOwner);
      }
      if (openInstanceIdRef.current === (instanceId ?? null)) return true;
      openInstanceIdRef.current = instanceId ?? null;
      if (instanceId) {
        const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "openInstance", args: { instanceId } }), studioSession.viewState);
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope), routeOwner);
      } else {
        const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation(studioSession, { controllerId: studioControllerId, action: "closeFocusedInstance" }), studioSession.viewState);
        const currentPanel = parsePanelState(studioSession.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
        updateSpacePanel(buildSpacePanelState(currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope), routeOwner);
      }
      return true;
    },
    [applyHostEffects, loadedPlugins, refreshUi, hostConfig, hostApp, landingAppId, hostAppId, switchToPluginApp, updateSpacePanel, rememberOpenSpaceId],
  );
  applyShellUriRef.current = applyShellUri;

  useEffect(() => {
    const current = session;
    if (!current || !hostPlugin || current.pluginId !== hostPlugin.handle.pluginId || current.app.id !== landingAppId) return;
    const seen = rehydratedLocalDocumentsRef.current.get(current.instanceId) ?? new Set<string>();
    rehydratedLocalDocumentsRef.current.set(current.instanceId, seen);
    const pending = localCatalog.documents.filter((document) => !seen.has(`${document.documentId}@${document.admittedAtMs}`));
    if (pending.length === 0) return;
    let cancelled = false;
    void (async () => {
      for (const document of pending) {
        if (cancelled) return;
        seen.add(`${document.documentId}@${document.admittedAtMs}`);
        const archive = await readLocalDocumentRef.current(document);
        if (archive === null || cancelled) continue;
        const args = localCatalogRehydrationArgumentsV1({ document, archive });
        if (args === null) continue;
        const owner = captureEffectOwner(current, captureDialogOrigin(current));
        const response = await hostPlugin.handle.handleAction(current.instanceId, encodeWindowActionInvocation(current, { controllerId: current.app.controllerId, action: LOCAL_CATALOG_ACTIONS_V1.rehydrate, args }), current.viewState);
        if (!cancelled) await applyHostEffects(response.requestedEffects ?? [], current, resolveUiDirtyScope(response.uiScope), owner);
      }
    })().catch(logUnlessRetiredV1("[os-shell] local catalog re-hydration failed"));
    return () => {
      cancelled = true;
    };
  }, [applyHostEffects, captureDialogOrigin, captureEffectOwner, hostPlugin, landingAppId, localCatalog, session]);

  useEffect(() => {
    const { overlay, sessionRoute: route } = shellSessionRouteV1(shellUri, sessionRouteRef.current);
    sessionRouteRef.current = route;
    setSessionRoute(route);
    if (!hostMode || loadedPlugins.length === 0) return;
    setHubWorkspaceOpen(overlay);
    if (route === null || !shellRouteLedgerRef.current.shouldApply(route)) return;
    requestSessionRoute(route);
  }, [applyShellUri, loadedPlugins.length, shellUri, hostMode, session?.pluginId, session?.instanceId, identityResolution, requestSessionRoute]);

  const resolveSyncTargetSession = useCallback((): ActiveSession | null => {
    if (!session) return null;
    if (hostMode && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
      }
    }
    return session;
  }, [loadedPlugins, panel, session, hostMode]);

  /** 🧵️ Opens this exact document and optional shared space in its captured plugin session.
   * Shared requests require a signed-in identity and carry only a requested surface; the worker
   * verifies the Hub plan and catalog assets before acquiring socket authority. Explicit bindings
   * are used by the manual persistence picker. Local documents never inherit the current route. */
  const openDocument = useCallback(
    async (ref: DocumentOpeningReference, bindings?: readonly PersistenceBinding[], target?: OpenDocumentSessionTarget): Promise<DocumentOpeningReceiptV1 | null> => {
      const documentTarget = resolveDocumentOpeningTarget(target, resolveSyncTargetSession(), loadedPlugins);
      if (!documentTarget) return null;
      const { session: targetSession, plugin } = documentTarget;
      const worker = ensureBackboneWorker();
      const resolvedBindings = bindings ?? resolveDocumentOpeningBindings(ref, {
        identity: identityRef.current,
        dataDir: hubEnv?.dataDir,
        surface: targetSession.app.dialect ? canonicalSurfaceId(targetSession.app.dialect, targetSession.app.role) : undefined,
      });
      const hubBinding = resolvedBindings.find((binding): binding is Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> => binding.kind === "hub");
      if (target?.expectedCatalogGenerationId !== undefined && hubBinding === undefined) return null;
      const creationMount = target?.expectedCatalogGenerationId === undefined ? null : createArtifactCreationCatalogMountV1(target.expectedCatalogGenerationId);
      const scope: DocumentScope | undefined = hubBinding === undefined ? undefined : { spaceId: hubBinding.spaceId, documentId: ref.documentId };
      const spaceIndexDocument = ref.documentId === SPACE_INDEX_DOCUMENT_ID;
      const workerBindings = spaceIndexDocument ? resolvedBindings.filter((binding) => binding.kind !== "hub") : resolvedBindings;
      const runtimeKey = scope === undefined ? ref.documentId : documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...scope });
      const openingAttempt = { clientInstanceId: crypto.randomUUID() };
      const { clientInstanceId } = openingAttempt;
      let resolveReady!: () => void, rejectReady!: (error: Error) => void;
      const ready = new Promise<void>((resolve, reject) => { resolveReady = resolve; rejectReady = reject; });
      void ready.catch(() => {});
      const persistToFolder = resolvedBindings.some((binding) => binding.kind === "folder");
      const persistence = new FolderArchivePersistenceV1(async () => {
        if (!persistToFolder || openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        if (!plugin.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
        const archive = await plugin.readAppDocumentArchive(targetSession.instanceId);
        if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        const archiveBytes = encodeDocumentArchiveBytes(archive);
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "localDocumentArchive", archive: Array.from(archiveBytes) } }) });
      });
      const entry: OpenDocumentSession = { openingReference: { ...ref }, persistenceBindings: [...resolvedBindings], session: targetSession, plugin, documentId: ref.documentId, clientInstanceId, ...(scope === undefined ? {} : { scope }), port: null, pending: [], pendingBytes: 0, replacements: new LatestDocumentReplacementV1(), persistence, readBack: new FolderReadBackRouteV1<DocumentArchivePack>(persistence), creationMount, ready, resolveReady, rejectReady };
      const parked = parkDocumentOpeningReplacementV1({ runtimeKey, plugin, instanceId: targetSession.instanceId, background: target?.background === true }, openDocumentSessionsRef.current, entry);
      if (parked === null) return null;
      const predecessorActor = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      if (predecessorActor !== undefined && predecessorActor.clientInstanceId !== clientInstanceId) {
        retireBrowserActorUi(runtimeKey, "browser-actor-action: opening parked");
      }
      const request: BackboneWorkerRequest = {
        kind: "open",
        actorIdentityGrant: {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64},
        clientInstanceId,
        documentId: ref.documentId,
        schema: ref.schema,
        bindings: workerBindings,
        watchExternal: true,
        actor: shellActorIdRef.current,
      };
      const expectsSocketActor = workerBindings.some((binding) => binding.kind === "hub");
      const socketActor = expectsSocketActor
        ? new Promise<string>((resolve, reject) => socketActorReadyRef.current.set(runtimeKey, { clientInstanceId, resolve, reject }))
        : null;
      void socketActor?.catch(() => {});
      const uri = `actor://${runtimeKey}`;
      const committed = await runDocumentOpeningAttemptV1({
        deadlineMs: documentOpeningDeadlineMs(),
        current: () => openDocumentSessionsRef.current.get(runtimeKey) === entry && entry.creationMount?.current() !== false,
        socket: async () => {
          worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
          postBrowserActorViewState(worker, entry, targetSession);
          await socketActor;
        },
        retire: () => {
          const waiter = socketActorReadyRef.current.get(runtimeKey);
          if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        },
        close: () => {
          entry.creationMount?.close(new Error("document opening aborted"));
          entry.rejectReady(new Error("document opening aborted"));
          entry.replacements.invalidate();
          void entry.port?.retire().catch(() => {});
          const restored = settleDocumentOpeningReplacementV1(openDocumentSessionsRef.current, parked, false);
          const successorActor = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
          if (successorActor?.clientInstanceId === clientInstanceId) {
            retireBrowserActorUi(runtimeKey, "browser-actor-action: opening aborted");
          }
        },
        detach: () => retireDocumentAttachment(plugin, targetSession.instanceId, clientInstanceId),
        attach: async () => {
          if (hubBinding && scope) {
            if (scope.documentId === SPACE_INDEX_DOCUMENT_ID) {
              spaceDirectoryHistoriesRef.current.set(runtimeKey, new SpaceDirectoryHistoryV1(scope.spaceId));
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-space-open", baseUrl: hubBinding.baseUrl, spaceId: scope.spaceId }) });
            } else {
              directoryScopedOwnersRef.current.set(runtimeKey, scope);
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-scope-open", baseUrl: hubBinding.baseUrl, scope, since: 0 }) });
            }
            if (scope.documentId === SPACE_INDEX_DOCUMENT_ID && !target?.background) {
              setSpaceArtifactCreationCatalog(null);
              setSpaceArtifactCreationCatalogUi({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: scope.spaceId, phase: "loading" });
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: scope.spaceId }) });
            }
          }
          if (hubBinding || entry.replacements.pending) {
            if (entry.creationMount === null) await entry.ready;
            else await entry.creationMount.attach(entry.ready);
          }
          else await documentAttachmentLane(plugin, targetSession.instanceId).attach(clientInstanceId, () => openDocumentSessionsRef.current.get(runtimeKey) === entry, async () => {
            if (entry.port === null || entry.port.closing) await bindDocumentBackbone(runtimeKey, entry);
          });
        },
        commit: () => {
          entry.creationMount = null;
          if (target?.background) return;
          dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
          dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
        },
      });
      if (committed) {
        for (const [key, owner] of settleDocumentOpeningReplacementV1(openDocumentSessionsRef.current, parked, true)) {
          if (key === runtimeKey) {
            void owner.port?.retire().catch(() => {});
            void retireDocumentAttachment(owner.plugin, owner.session.instanceId, owner.clientInstanceId).catch((error) => undefined);
            backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: owner.documentId, clientInstanceId: owner.clientInstanceId, ...(owner.scope === undefined ? {} : { spaceId: owner.scope.spaceId }) }) });
            continue;
          }
          closeDocumentRef.current(key, owner.clientInstanceId);
        }
      }
      return committed ? { committed: true, runtimeKey, clientInstanceId } : null;
    },
    [bindDocumentBackbone, documentAttachmentLane, ensureBackboneWorker, loadedPlugins, postBrowserActorViewState, resolveSyncTargetSession, hubEnv, retireDocumentAttachment, retireBrowserActorUi],
  );
  openDocumentRef.current = openDocument;

  useEffect(() => {
    const worker = backboneWorkerRef.current;
    if (worker === null) return;
    for (const entry of openDocumentSessionsRef.current.values()) {
      if (entry.scope === undefined) continue;
      const current = sessionRef.current;
      const targetSession = current?.pluginId === entry.session.pluginId && current.instanceId === entry.session.instanceId ? current : entry.session;
      postBrowserActorViewState(worker, entry, targetSession);
    }
  }, [activeToolId, activeUtilityByWindowId, extraWindowInstances, postBrowserActorViewState, session?.viewState]);

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

  /** 🔗️ Attaches the focused program's document to a folder, a file or a hub. The document is addressed by the program's
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
  /** 📎️ The remembered folder offered back for the focused program's document, while that document is not attached. */
  const folderReconnectOffer = useMemo(
    () => localFolderReconnectOfferV1(localFolderBindings, syncProgramIdentity, new Set([...openDocumentSessionsRef.current.values()].filter((entry) => entry.scope === undefined).map((entry) => entry.documentId))),
    [localFolderBindings, syncProgramIdentity, syncBackboneUri],
  );
  const [folderReconnectBusy, setFolderReconnectBusy] = useState(false);
  /** 📎️ The person's own gesture reattaches the remembered folder; its archive is restored like a fresh load. */
  const reconnectLocalFolder = useCallback((binding: LocalFolderBinding) => {
    setFolderReconnectBusy(true);
    void attachSyncTarget({ kind: "folder", path: binding.folder.path }).finally(() => setFolderReconnectBusy(false));
  }, [attachSyncTarget]);

  const browseSyncBackbonePath = useCallback(async (kind: "file" | "folder") => {
    const path = kind === "folder" ? await requestBackboneFolderPath() : await requestBackboneFilePath();
    if (path) dispatch({ type: "SET_SYNC_DRAFT_PATH", value: path });
  }, []);

  const spawnProgram = useCallback(
    async (program: SpaceProgramEntry) => {
      const refuse = (reason: SpawnProgramRefusalReasonV1, ...detail: readonly unknown[]): void => {
        console.warn(`[os-shell] spawnProgram ${program.pluginId} ${program.appId}: ${spawnProgramRefusalCodeV1(reason)}`, ...detail);
        showTransientNoticeRef.current(spawnProgramRefusalNoticeTextV1(reason, program.breadcrumb.join(" · "), uiLocaleRef.current), "warning", spawnProgramRefusalCodeV1(reason));
      };
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry) return refuse("program-not-installed");
      if (!session) return refuse("session-not-ready");
      let instanceId: number;
      try {
        instanceId = await pluginEntry.handle.createApp(program.appId);
      } catch (openError) {
        return refuse("open-failed", openError);
      }
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateSpacePanel(
        studioPanelFocusingSpawned(currentPanel, {
          id: spawnedId,
          pluginId: program.pluginId,
          instanceId,
          appId: program.appId,
          label: program.label,
          breadcrumb: program.breadcrumb,
        }),
      );
    },
    [loadedPlugins, session, updateSpacePanel, hostApp],
  );

  return {
    resolveSyncTargetSession, openDocument, closeDocument, forgetLocalFolder, attachSyncTarget, detachSyncBackbone,
    folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, browseSyncBackbonePath, spawnProgram
  };
}
//#endregion FrameworkOsShell
