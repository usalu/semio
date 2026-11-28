/** 🔶️ The scope phase of useShellUriAndDocuments. */
import { useCallback, useEffect, useRef } from "../../../⚛️react-port/🟦️.ts";
import { dialectCoordinate, spaceIndexOf, resolveUiDirtyScope, type LocalizedLabel } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { shellSessionRouteV1 } from "../../🔀️surface-switch/🟦️.ts";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { LOCAL_CATALOG_ACTIONS_V1, localCatalogRehydrationArgumentsV1 } from "../../🗂️local-catalog/🟦️.ts";
import { localProgramsV1, type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { buildSpacePanelState, parsePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { parseShellRoute } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { appOwnsCommand, encodeWindowActionInvocation, invocationPluginIdV1 } from "../../📨️window-invocation/🟦️.ts";
import { findDialectApp, mintDirectoryCommandRequestId, SPACE_INDEX_DOCUMENT_ID, type SpaceArtifactCreationOwnerV1 } from "../../🧾️space-artifact-creation/🟦️.ts";
import { browserActorWorkflowNodeIsMountedV1 } from "../../🛫️spawned-session/🟢️request-service-operation/🪪️owner/🟦️.ts";
import { openStudioWorkflowV1 } from "../../🧾️space-artifact-creation/🔁️workflow/🟦️.ts";
import { reduceArtifactCreationProgressUiV1 } from "../../🌱️artifact-creation/🟦️.tsx";
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { logUnlessRetiredV1, requiredHostPanelLeafId } from "../../🧱️built-node-stores/🟦️.ts";
import { pluginInstanceWasRetiredV1 } from "../../../🔌️PluginRuntime/🪦️instance-retirement/🟦️.ts";
import type { ShellCoreAfterApplyHostEffects } from "../../⚡️host-effects/🟦️.ts";

/** 🎯️ The session an export or sync addresses: the active spawned program while its instance lives; once that instance is RETIRED (windows closed, channel disposed) the newest spawned program still alive, and the primary session only when none remains. */
export function resolveSyncTargetSessionV1({ session, panel, hostMode, loadedPlugins, retired }: {
  readonly session: ActiveSession | null;
  readonly panel: ReturnType<typeof parsePanelState> | null | undefined;
  readonly hostMode: boolean;
  readonly loadedPlugins: ReadonlyArray<{ readonly handle: { readonly pluginId: string }; readonly manifest: { readonly apps: ReadonlyArray<ActiveSession["app"]> } }>;
  readonly retired: (pluginId: string, instanceId: number) => boolean;
}): ActiveSession | null {
  if (!session) return null;
  if (!hostMode || !panel?.activeSpawnedId) return session;
  const active = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
  const candidates = [...(active ? [active] : []), ...[...panel.spawnedApps].reverse().filter((entry) => entry !== active)];
  for (const spawned of candidates) {
    if (retired(spawned.pluginId, spawned.instanceId)) continue;
    const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
    if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
  }
  return session;
}

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
    browserActorUiByRuntimeKeyRef, browserActorUiVersion, retireBrowserActorUi, directoryScopedOwnersRef, spaceDirectoryHistoriesRef,
    socketActorReadyRef, setBootstrapUiByDocument, setExecutionTargetUiByDocument, setPresencePeersByRuntimeKey,
    rebootstrapDiscardedSessionsRef, ensureBackboneWorker, readLocalDocumentRef, shellUri, sessionRouteRef,
    setSessionRoute, requestSessionRoute, panel, tutorialDrivenRef, postBrowserActorViewState, refreshUi,
    updateSpacePanel, updateSpacePanelFrom, loadedPluginsRef, switchToPluginApp, applyHostEffects
  } = core;
  const openedStudioRef = useRef<Readonly<{ spaceId: string; instanceId: number; epoch: number; workflowArtifactId?: string }> | null>(null);
  const applyShellUri = useCallback(
    async (uri: string, preservedViewState?: ViewModel): Promise<boolean> => {
      const currentSession = sessionRef.current;
      if (!hostConfig || !currentSession || loadedPlugins.length === 0) return false;
      const route = parseShellRoute(uri);
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
      if (!route.studio && !instanceId) {
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
      rememberOpenSpaceId(spaceId);
      const epoch = core.openSpaceEpochRef.current;
      if (!hostAppId) throw new Error("required host app identity is unavailable");
      const studioSession = currentSession.app.id === hostAppId ? currentSession : await switchToPluginApp(hostConfig.pluginId, hostAppId, preservedViewState);
      if (!studioSession) return false;
      const studioControllerId = studioSession.app.controllerId;
      const studioOpened = openedStudioRef.current;
      const viewerIdentity = identityRef.current?.userId;
      const current = (): boolean => core.openSpaceEpochRef.current === epoch && openSpaceIdRef.current === spaceId && sessionRouteRef.current === uri
        && identityRef.current?.userId === viewerIdentity && shellDialogSessionIsCurrentV1(studioSession, core.shellStateRef.current.pluginRuntime.session);
      const studio = studioSession.app;
      let workflowArtifactId = [...openDocumentSessionsRef.current.values()].find((entry) => entry.plugin === sPlugin
        && entry.session.instanceId === studioSession.instanceId && entry.scope?.spaceId === spaceId)?.scope?.documentId;
      const runStudio = async (action: string, args?: Record<string, unknown>): Promise<boolean> => {
        const owner = captureEffectOwner(studioSession, captureDialogOrigin(studioSession));
        const viewState = core.resolvedTargetViewState(studioSession);
        if (workflowArtifactId !== undefined) {
          const outcome = await core.onActionRef.current({
            controllerId: studioControllerId, action, args,
            provenance: { origin: "guest", causedBy: null, windowId: studio.windowKinds[0]?.id ?? null },
          }, undefined, false, undefined, owner);
          return outcome.kind === "applied" && current();
        }
        const response = await sPlugin.handleAction(studioSession.instanceId, encodeWindowActionInvocation({ ...studioSession, viewState }, { controllerId: studioControllerId, action, args }), viewState);
        if (!current()) return false;
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope), owner);
        return current();
      };
      if (studioOpened?.spaceId !== spaceId || studioOpened.instanceId !== studioSession.instanceId || studioOpened.epoch !== epoch || studioOpened.workflowArtifactId !== workflowArtifactId) {
        openInstanceIdRef.current = null;
        if (spaceId !== "demo" && hubEnv && identityResolution === "signed-in" && studio.dialect && studio.io?.artifactSchema) {
          const worker = ensureBackboneWorker();
          const requestId = mintDirectoryCommandRequestId();
          const dialect = studio.dialect;
          const artifactSchema = studio.io.artifactSchema;
          const result = await openStudioWorkflowV1({
            worker, baseUrl: hubEnv.hubBaseUrl, spaceId, requestId, current,
            app: { dialect, io: { artifactSchema } },
            name: resolveManifestLabel(studio.label as LocalizedLabel | string, core.uiTerminologyRef.current, uiLocaleRef.current),
            create: (request) => {
              const progressOwner = {
                requestId, spaceId, name: request.name, kindId: request.kindId,
                expectedCatalogGenerationId: request.expectedCatalogGenerationId,
                runtimeKey: `studio:${spaceId}:${studioSession.instanceId}`, clientInstanceId: requestId,
                sessionInstanceId: studioSession.instanceId,
              };
              const owner: SpaceArtifactCreationOwnerV1 = {
                ...progressOwner, opening: false, cancelRequested: false, ready: null,
                workflow: { current, expectedSchema: artifactSchema, expectedDialect: dialectCoordinate(dialect), finish: async () => true },
              };
              core.spaceArtifactCreationOwnersRef.current.set(requestId, owner);
              core.setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: progressOwner, atMs: Date.now() }));
              worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
            },
          });
          if (!current() || result === null || result === "missing-workflow") return false;
          if (typeof result === "object") workflowArtifactId = result.artifactId;
        }
        for (const [runtimeKey, entry] of openDocumentSessionsRef.current) {
          if (entry.plugin !== sPlugin || entry.session.instanceId !== studioSession.instanceId
            || (workflowArtifactId !== undefined && entry.scope?.spaceId === spaceId && entry.scope.documentId === workflowArtifactId)) continue;
          closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
          await retireDocumentAttachment(sPlugin, studioSession.instanceId, entry.clientInstanceId);
        }
        if (!current()) return false;
        if (workflowArtifactId !== undefined && studio.io?.artifactSchema) {
          const attached = [...openDocumentSessionsRef.current.values()].some((entry) => entry.plugin === sPlugin
            && entry.session.instanceId === studioSession.instanceId && entry.scope?.spaceId === spaceId && entry.scope?.documentId === workflowArtifactId);
          if (!attached) {
            const receipt = await openDocumentRef.current({ spaceId, documentId: workflowArtifactId, schema: studio.io.artifactSchema }, undefined, { session: studioSession, plugin: sPlugin, current });
            if (receipt === null || !current()) return false;
          }
        }
        if (workflowArtifactId !== undefined && appOwnsCommand(studio, "setAppRegistrations")) {
          const actor = core.directBrowserActorForSession(studioSession);
          if (actor === null) return false;
          const registrations = localProgramsV1(loadedPlugins).flatMap((entry) => (entry.manifest.apps ?? []).map((app) => ({ pluginId: entry.handle.pluginId, app })));
          for (const registration of registrations) {
            if (!current()) return false;
            await core.dispatchDirectBrowserActorCommand(actor, {
              address: { owner: { app: { pluginId: invocationPluginIdV1(studioSession.pluginId), appId: studio.id } }, commandId: "setAppRegistrations" },
              arguments: { json: JSON.stringify([registration]) },
            }, core.resolvedTargetViewState(studioSession));
          }
        }
        if (!await runStudio("openSpace", { spaceId, ...(workflowArtifactId === undefined ? {} : { workflowArtifactId }) })) return false;
        openedStudioRef.current = { spaceId, instanceId: studioSession.instanceId, epoch, workflowArtifactId };
      }
      const mounted = (nodeId: string): boolean => browserActorWorkflowNodeIsMountedV1(
        core.shellStateRef.current.pluginRuntime.session, nodeId,
        workflowArtifactId === undefined ? undefined : {
          spaceId, workflowArtifactId, documents: openDocumentSessionsRef.current,
          actors: browserActorUiByRuntimeKeyRef.current, retired: core.retiredSessionDocumentOwnersRef.current,
        },
      );
      if (instanceId ? mounted(instanceId) : openInstanceIdRef.current === null) return true;
      if (!await runStudio(instanceId ? "openInstance" : "closeFocusedInstance", instanceId ? { instanceId } : undefined)) return false;
      if (instanceId) return mounted(instanceId);
      openInstanceIdRef.current = null;
      const currentPanel = parsePanelState(core.shellStateRef.current.pluginRuntime.session?.viewState ?? studioSession.viewState)
        ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      updateSpacePanel(buildSpacePanelState(currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
      return true;
    },
    [applyHostEffects, loadedPlugins, refreshUi, hostConfig, hostApp, landingAppId, hostAppId, switchToPluginApp, updateSpacePanel, rememberOpenSpaceId, hubEnv, identityResolution, ensureBackboneWorker, retireDocumentAttachment],
  );
  applyShellUriRef.current = applyShellUri;

  useEffect(() => {
    const current = session;
    if (!current || !hostPlugin || current.pluginId !== hostPlugin.handle.pluginId || current.app.id !== landingAppId) return;
    const seen = rehydratedLocalDocumentsRef.current.get(current.instanceId) ?? new Set<string>();
    rehydratedLocalDocumentsRef.current.set(current.instanceId, seen);
    const pending = localCatalog.documents.filter((document) => document.surface === undefined && !seen.has(`${document.documentId}@${document.admittedAtMs}`));
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

  const resolveSyncTargetSession = useCallback((): ActiveSession | null => resolveSyncTargetSessionV1({ session, panel, hostMode, loadedPlugins, retired: pluginInstanceWasRetiredV1 }), [loadedPlugins, panel, session, hostMode]);
  return { resolveSyncTargetSession, loadedPlugins, ensureBackboneWorker, identityRef, hubEnv, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, retireBrowserActorUi, shellActorIdRef, socketActorReadyRef, postBrowserActorViewState, retireDocumentAttachment, spaceDirectoryHistoriesRef, directoryScopedOwnersRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, documentAttachmentLane, bindDocumentBackbone, dispatch, backboneWorkerRef, closeDocumentRef, openDocumentRef, sessionRef, activeToolId, activeUtilityByWindowId, extraWindowInstances, session, cancelSpaceArtifactCreationsForRuntime, backgroundSpaceIndexSessionsRef, inferencePortOpeningRef, inferencePortOwnerRef, inferencePortAuthorityRef, inferencePortEpochRef, localDocumentOwnerRegistry, liveDialogRef, closeOwnedDialog, tutorialDrivenRef, rebootstrapDiscardedSessionsRef, setInferenceHistoryByRuntimeKey, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument, scope, showTransientNoticeRef, syncBackboneUri, uiLocaleRef, hostApp, updateSpacePanel, updateSpacePanelFrom, loadedPluginsRef };
}
