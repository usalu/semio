/** 🟠️ The switch plugin app phase of useSpawnedSession. */
import { DOCUMENT_SERVICE_TOPIC_V1 } from "@semio-tech/framework/inference/service";
import { admitDocumentServiceWireDeclarationV1 } from "@semio-tech/framework/inference/service/io";
import { prepareDocumentSurfaceV1 } from "../../🔀️surface-switch/📄️document/🟦️.ts";
import { createAdmittedShellInstanceV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { useCallback, useEffect } from "react";
import { type AppDefinition } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { quiesceSessionWorkV1, surfaceSwitchBusyTextV1 } from "../../🔀️surface-switch/🟦️.ts";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type ActiveSession, type SpaceProgramEntry } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { applyFrameworkLayoutSeed } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildSpacePanelState, panelJsonFromState, parsePanelState, studioPanelFocusingSpawned, type SpacePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { seededActiveWindowId } from "../../🎚️shell-model/🟦️.tsx";
import { encodeWindowActionInvocation } from "../../📨️window-invocation/🟦️.ts";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import { requiredHostPanelLeafId, withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useDocumentServicesState } from "../🔶️document-services/🟦️.ts";
import type { useUpdateSpacePanelState } from "../🔸️update-space-panel/🟦️.ts";

/** 🟡 Builds switchToPluginApp, syncSpawnedPluginDocument, ensureSpawnedPlugin of useSpawnedSession. */
export function useSwitchPluginAppState({ hostMode, hostApp, setSurfaceSwitchBusy, sessionSwitchGateRef, session, loadedPlugins, sessionWorkRef, sealedInstancesRef, loadedPluginsRef, sessionRef, openDocumentSessionsRef, openDocumentRef, closeDocumentRef, documentAttachmentLanesRef, retireSessionInstance, dispatch, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, landingAppId, rememberOpenSpaceId, openInstanceIdRef, refreshUi, showTransientNoticeRef, documentServices, documentServiceInventoryRef, backboneWorkerRef }: Pick<ReturnType<typeof useDocumentServicesState>, "hostMode" | "hostApp" | "setSurfaceSwitchBusy" | "sessionSwitchGateRef" | "session" | "loadedPlugins" | "sessionWorkRef" | "sealedInstancesRef" | "loadedPluginsRef" | "sessionRef" | "openDocumentSessionsRef" | "openDocumentRef" | "closeDocumentRef" | "documentAttachmentLanesRef" | "dispatch" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "extraWindowInstancesRef" | "extraWindowCounterRef" | "landingAppId" | "rememberOpenSpaceId" | "openInstanceIdRef" | "refreshUi" | "showTransientNoticeRef" | "documentServices" | "documentServiceInventoryRef" | "backboneWorkerRef"> & Pick<ReturnType<typeof useUpdateSpacePanelState>, "retireSessionInstance">) {
  const switchToPluginApp = useCallback(
    async (pluginId: string, appId: string, viewState?: ViewModel, documentSource?: ActiveSession): Promise<ActiveSession | null> => {
      const defaultViewState = (app: AppDefinition): ViewModel => ({ activeModeId: app.defaultModeId ?? app.modes[0]?.id, ...(hostMode ? { panelJson: panelJsonFromState(buildSpacePanelState([], requiredHostPanelLeafId(hostApp))) } : {}) });
      setSurfaceSwitchBusy(true);
      try {
        const outcome = await sessionSwitchGateRef.current.run(
          {
            session,
            resolveApp: (targetPluginId, targetAppId) => loadedPlugins.find((entry) => entry.handle.pluginId === targetPluginId)?.manifest.apps.find((candidate) => candidate.id === targetAppId) ?? null,
            appId: (app) => app.id,
            quiesce: async (retiring) => {
              const quiet = await quiesceSessionWorkV1(() => sessionWorkRef.current.pending(retiring.pluginId, retiring.instanceId));
              if (!quiet.settled) console.warn(`surface-switch draining ${retiring.pluginId}#${retiring.instanceId} ${sessionWorkRef.current.outstanding(retiring.pluginId, retiring.instanceId)}`);
              return quiet;
            },
            seal: (retiring) => sealedInstancesRef.current.seal(retiring.pluginId, retiring.instanceId),
            unseal: (kept) => sealedInstancesRef.current.unseal(kept.pluginId, kept.instanceId),
            createInstance: async (targetPluginId, app) => {
              const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === targetPluginId)?.handle;
              if (!handle) throw new Error(`switchToPluginApp: ${targetPluginId} is no longer loaded`);
              if (documentSource === undefined) return handle.createApp(app.id);
              const current = () => sessionRef.current?.pluginId === documentSource.pluginId && sessionRef.current?.instanceId === documentSource.instanceId;
              if (targetPluginId !== documentSource.pluginId || !current() || !app.dialect) throw new Error("surface-document.predecessor-stale");
              const readArchive = handle.readAppDocumentArchive;
              const loadArchive = handle.loadAppDocumentArchive;
              if (!readArchive || !loadArchive) throw new Error("surface-document.archive-port-unavailable");
              const owners = [...openDocumentSessionsRef.current.values()].filter(entry => entry.plugin === handle && entry.session.instanceId === documentSource.instanceId);
              if (owners.length > 1) throw new Error("surface-document.owner-ambiguous");
              const owner = owners[0];
              const surfaceId = canonicalSurfaceId(app.dialect,app.role);
              let receipt: DocumentOpeningReceiptV1 | null = null;
              return prepareDocumentSurfaceV1({
                current,
                capture: () => readArchive(documentSource.instanceId),
                create: () => handle.createApp(app.id),
                restore: (instanceId,archive) => loadArchive(instanceId,archive),
                attach: async instanceId => {
                  if (owner === undefined) return;
                  const nextSession: ActiveSession = { pluginId: targetPluginId,instanceId,app,viewState: viewState ?? defaultViewState(app),documentInitialized: true };
                  const bindings = owner.persistenceBindings.map(binding => binding.kind === "hub" ? { ...binding,requestedSurfaceId: surfaceId } : binding);
                  receipt = await openDocumentRef.current(owner.openingReference,bindings,{ plugin: handle,session: nextSession });
                  if (receipt === null) throw new Error("surface-document.attachment-refused");
                },
                release: async instanceId => {
                  if (receipt !== null) closeDocumentRef.current(receipt.runtimeKey,receipt.clientInstanceId);
                  await documentAttachmentLanesRef.current.get(handle)?.get(instanceId)?.drain();
                  await handle.destroyApp(instanceId);
                },
              });
            },
            retire: retireSessionInstance,
            defaultViewState,
            publish: (next) => dispatch({ type: "SET_SESSION", value: { ...next,documentInitialized: documentSource !== undefined } }),
            seedLayout: (app) => {
              const seeded = applyFrameworkLayoutSeed(app.defaultLayout, withLocalizedWindowKindLabels(app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
              extraWindowInstancesRef.current = seeded.extraInstances;
              extraWindowCounterRef.current = seeded.extraInstances.length;
              dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
              dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
              dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
              if (hostMode && app.id === landingAppId) {
                rememberOpenSpaceId(null);
                openInstanceIdRef.current = null;
              }
            },
            refresh: refreshUi,
          },
          { pluginId, appId, viewState },
          (retired, retireError) => console.warn(`switchToPluginApp: predecessor ${retired.pluginId}/${retired.app.id} retirement failed`, retireError),
        );
        if (outcome.status === "draining" || outcome.status === "busy") {
          showTransientNoticeRef.current(surfaceSwitchBusyTextV1(uiLocale), "info");
          return null;
        }
        return outcome.session === null || documentSource === undefined ? outcome.session : { ...outcome.session,documentInitialized: true };
      } finally {
        setSurfaceSwitchBusy(false);
      }
    },
    [loadedPlugins, refreshUi, retireSessionInstance, session, appLabelsOverlay, hostMode, hostApp, landingAppId, uiTerminology, uiLocale, rememberOpenSpaceId],
  );

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, artifactJson: string, viewState: ViewModel) => {
    try {
      const document = JSON.parse(artifactJson) as Record<string, unknown>;
      const targetSession: ActiveSession = { pluginId: plugin.pluginId, instanceId: pluginInstanceId, app, viewState: spawnedProgramViewStateV1(viewState, app) };
      await plugin.handleAction(pluginInstanceId, encodeWindowActionInvocation(targetSession, { controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: SpaceProgramEntry, isCurrent: () => boolean, label?: string, osInstanceId?: string, artifactJson?: string, sourceViewState?: ViewModel): Promise<SpacePanelState | null> => {
      if (!isCurrent()) return null;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return null;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(sourceViewState ?? session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (artifactJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, artifactJson, sourceViewState ?? session.viewState);
        }
        return isCurrent() ? studioPanelFocusingSpawned(currentPanel, existing) : null;
      }
      const instanceId = await createAdmittedShellInstanceV1(isCurrent, () => pluginEntry.handle.createApp(program.appId), (id) => pluginEntry.handle.destroyApp(id));
      if (instanceId === null) return null;
      if (artifactJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, artifactJson, sourceViewState ?? session.viewState);
      }
      if (!isCurrent()) {
        await pluginEntry.handle.destroyApp(instanceId);
        return null;
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      return studioPanelFocusingSpawned(currentPanel, {
        id: spawnedId,
        pluginId: program.pluginId,
        instanceId,
        appId: program.appId,
        label: label ?? program.label,
        breadcrumb: program.breadcrumb,
      });
    },
    [loadedPlugins, session, syncSpawnedPluginDocument, hostApp],
  );

  useEffect(() => {
    const services = documentServices.filter((presentation) => loadedPlugins.some((entry) => entry.handle.pluginId === presentation.owner && (entry.manifest.topicContributions ?? []).some((topic) => {
      if (topic.topic !== DOCUMENT_SERVICE_TOPIC_V1) return false;
      try { return admitDocumentServiceWireDeclarationV1(presentation.owner, topic.payload).serviceId === presentation.serviceId; } catch { return false; }
    }))).map(({owner, serviceId}) => ({owner, serviceId}));
    documentServiceInventoryRef.current=services;
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-contributions", services }) });
  }, [loadedPlugins, documentServices]);
  return { switchToPluginApp, ensureSpawnedPlugin };
}
