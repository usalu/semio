/** 🔶️ The scope phase of useShellUriAndDocuments. */
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { spaceIndexOf, resolveUiDirtyScope } from "@semio-tech/framework";
import { shellSessionRouteV1 } from "../../🔀️surface-switch/🟦️.ts";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { LOCAL_CATALOG_ACTIONS_V1, localCatalogRehydrationArgumentsV1 } from "../../🗂️local-catalog/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { buildSpacePanelState, parsePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { parseShellRoute } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { encodeWindowActionInvocation } from "../../📨️window-invocation/🟦️.ts";
import { findDialectApp, SPACE_INDEX_DOCUMENT_ID } from "../../🧾️space-artifact-creation/🟦️.ts";
import { logUnlessRetiredV1, requiredHostPanelLeafId } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterApplyHostEffects } from "../../⚡️host-effects/🟦️.ts";

/** 🔷 Builds scope, localDocumentOwnerRegistry, hostConfig, hostMode of useShellUriAndDocuments. */
export function useScopeState({ core }: { readonly core: ShellCoreAfterApplyHostEffects }) {
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
  return { resolveSyncTargetSession, loadedPlugins, ensureBackboneWorker, identityRef, hubEnv, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, retireBrowserActorUi, shellActorIdRef, socketActorReadyRef, postBrowserActorViewState, retireDocumentAttachment, spaceDirectoryHistoriesRef, directoryScopedOwnersRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, documentAttachmentLane, bindDocumentBackbone, dispatch, backboneWorkerRef, closeDocumentRef, openDocumentRef, sessionRef, activeToolId, activeUtilityByWindowId, extraWindowInstances, session, cancelSpaceArtifactCreationsForRuntime, backgroundSpaceIndexSessionsRef, inferencePortOpeningRef, inferencePortOwnerRef, inferencePortAuthorityRef, inferencePortEpochRef, localDocumentOwnerRegistry, liveDialogRef, closeOwnedDialog, tutorialDrivenRef, rebootstrapDiscardedSessionsRef, setInferenceHistoryByRuntimeKey, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument, scope, showTransientNoticeRef, syncBackboneUri, uiLocaleRef, hostApp, updateSpacePanel };
}
