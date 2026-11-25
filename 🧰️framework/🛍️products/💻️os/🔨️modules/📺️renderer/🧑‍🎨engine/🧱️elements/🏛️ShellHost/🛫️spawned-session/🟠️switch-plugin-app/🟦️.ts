/** 🟠️ The switch plugin app phase of useSpawnedSession. */
import { DOCUMENT_SERVICE_TOPIC_V1 } from "@semio-tech/framework/inference/service";
import { admitDocumentServiceWireDeclarationV1 } from "@semio-tech/framework/inference/service/io";
import { prepareDocumentSurfaceV1, restoreDocumentSurfaceV1 } from "../../🔀️surface-switch/📄️document/🟦️.ts";
import { documentLoadCancelledV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { createAdmittedShellInstanceV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { type AppDefinition } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { quiesceSessionWorkV1, surfaceSwitchBusyTextV1 } from "../../🔀️surface-switch/🟦️.ts";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type ActiveSession, type SpaceProgramEntry } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { applyFrameworkLayoutSeed } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildSpacePanelState, panelJsonFromState, parsePanelState, studioPanelFocusingSpawned, type SpacePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { captureProgramRetirementOwnersV1, sameProgramRetirementOwnersV1, type ProgramRetirementRequestV1, type ProgramRetirementLeaseV1, type ProgramRetirementWorkV1 } from "../../🚪️program-retirement/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { seededActiveWindowId } from "../../🎚️shell-model/🟦️.tsx";
import { encodeWindowActionInvocation } from "../../📨️window-invocation/🟦️.ts";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import type { DocumentOpeningReference } from "../../🧭️opening/🟦️.ts";
import { requiredHostPanelLeafId, withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useDocumentServicesState } from "../🔶️document-services/🟦️.ts";
import type { useUpdateSpacePanelState } from "../🔸️update-space-panel/🟦️.ts";

type PreparedPrimarySessionV1 = Readonly<{
  handle: PluginWasmHandle;
  instanceId: number;
  retirement: ProgramRetirementWorkV1;
  current: () => boolean;
  onPublished: () => void;
}>;

/** 🟡 Builds switchToPluginApp, syncSpawnedPluginDocument, ensureSpawnedPlugin of useSpawnedSession. */
export function useSwitchPluginAppState({ pluginOpInFlightRef, admitProgramRetirementRef, hostMode, hostApp, setSurfaceSwitchBusy, sessionSwitchGateRef, session, loadedPlugins, sessionWorkRef, sealedInstancesRef, loadedPluginsRef, sessionRef, openDocumentSessionsRef, openDocumentRef, closeDocumentRef, documentAttachmentLanesRef, retireSessionInstance, dispatch, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, landingAppId, rememberOpenSpaceId, openInstanceIdRef, refreshUi, reportRefreshFault, showTransientNoticeRef, documentServices, documentServiceInventoryRef, backboneWorkerRef, documentTransfer }: Pick<ReturnType<typeof useDocumentServicesState>, "hostMode" | "hostApp" | "setSurfaceSwitchBusy" | "sessionSwitchGateRef" | "session" | "loadedPlugins" | "sessionWorkRef" | "sealedInstancesRef" | "loadedPluginsRef" | "sessionRef" | "openDocumentSessionsRef" | "openDocumentRef" | "closeDocumentRef" | "documentAttachmentLanesRef" | "dispatch" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "extraWindowInstancesRef" | "extraWindowCounterRef" | "landingAppId" | "rememberOpenSpaceId" | "openInstanceIdRef" | "refreshUi" | "reportRefreshFault" | "showTransientNoticeRef" | "documentServices" | "documentServiceInventoryRef" | "backboneWorkerRef" | "documentTransfer"> & Pick<ReturnType<typeof useUpdateSpacePanelState>, "retireSessionInstance"> & { readonly pluginOpInFlightRef: { current: ReadonlySet<string> }; readonly admitProgramRetirementRef: { current: ProgramRetirementRequestV1 } }) {
  const switchAppSession = useCallback(
    async (pluginId: string, appId: string, viewState?: ViewModel, documentSource?: ActiveSession, prepared?: PreparedPrimarySessionV1): Promise<ActiveSession | null> => {
      const defaultViewState = (app: AppDefinition): ViewModel => ({ activeModeId: app.defaultModeId ?? app.modes[0]?.id, ...(hostMode ? { panelJson: panelJsonFromState(buildSpacePanelState([], requiredHostPanelLeafId(hostApp))) } : {}) });
      setSurfaceSwitchBusy(true);
      let retirement: ProgramRetirementLeaseV1 | null = null;
      const predecessor = sessionRef.current;
      const predecessorHandle = loadedPluginsRef.current.find(entry => entry.handle.pluginId === predecessor?.pluginId)?.handle;
      try {
        if (prepared && (hostMode || !prepared.current() || predecessor === null || (predecessorHandle === prepared.handle && predecessor.instanceId === prepared.instanceId))) return null;
        if (documentSource === undefined && predecessor !== null && (prepared !== undefined || predecessor.pluginId !== pluginId || predecessor.app.id !== appId)) {
          const owners = captureProgramRetirementOwnersV1(loadedPluginsRef.current, predecessor);
          const current = (): boolean => {
            const host = sessionRef.current;
            return (prepared?.current() ?? true) && host?.pluginId === predecessor.pluginId && host.app.id === predecessor.app.id && host.instanceId === predecessor.instanceId && sameProgramRetirementOwnersV1(owners, captureProgramRetirementOwnersV1(loadedPluginsRef.current, host));
          };
          retirement = await admitProgramRetirementRef.current(owners, current, prepared?.retirement);
          if (retirement === null || !retirement.current()) return null;
        }
        try {
          const outcome = await sessionSwitchGateRef.current.run(
            {
              session: predecessor,
              resolveApp: (targetPluginId, targetAppId) => loadedPlugins.find((entry) => entry.handle.pluginId === targetPluginId)?.manifest.apps.find((candidate) => candidate.id === targetAppId) ?? null,
              appId: (app) => app.id,
              quiesce: async (retiring) => {
                const quiet = await quiesceSessionWorkV1(() => sessionWorkRef.current.pending(retiring.pluginId, retiring.instanceId));
                if (!quiet.settled) console.warn(`surface-switch draining ${retiring.pluginId}#${retiring.instanceId} ${sessionWorkRef.current.outstanding(retiring.pluginId, retiring.instanceId)}`);
                return quiet;
              },
              seal: (retiring) => { if (!predecessorHandle) throw new Error("program-retirement.owner-unavailable"); sealedInstancesRef.current.seal(predecessorHandle, retiring.pluginId, retiring.instanceId); },
              unseal: (kept) => { if (predecessorHandle) sealedInstancesRef.current.unseal(predecessorHandle, kept.pluginId, kept.instanceId); },
              createInstance: async (targetPluginId, app) => {
                if (retirement !== null && !retirement.current()) throw new Error("program-retirement.owner-stale");
                const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === targetPluginId)?.handle;
                if (pluginOpInFlightRef.current.has(targetPluginId)) throw new Error("program-retirement.program-busy");
                if (!handle) throw new Error(`switchToPluginApp: ${targetPluginId} is no longer loaded`);
                if (prepared !== undefined) {
                  if (handle !== prepared.handle || !prepared.current()) throw new Error("archive-file.owner-retired");
                  return prepared.instanceId;
                }
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
                const documentFile = resolveManifestLabel(app.label,uiTerminology,uiLocale) || app.id;
                let receipt: DocumentOpeningReceiptV1 | null = null;
                return prepareDocumentSurfaceV1({
                  current,
                  capture: () => readArchive(documentSource.instanceId),
                  create: () => handle.createApp(app.id),
                  restore: (instanceId,archive) => restoreDocumentSurfaceV1(documentTransfer().trackDocumentTransfer(documentFile,targetPluginId),(captured,signal,progress) => loadArchive(instanceId,captured,signal,progress),archive),
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
              retire: async retired => { if (retirement === null) return retireSessionInstance(retired as ActiveSession); if (!await retirement.retire()) throw new Error("program-retirement.owner-stale"); },
              defaultViewState,
              publish: (next) => {
                dispatch({ type: "SET_SESSION", value: { ...next, documentInitialized: documentSource !== undefined || prepared !== undefined } });
                prepared?.onPublished();
              },
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
              refresh: prepared === undefined ? refreshUi : (next) => refreshUi(next).catch(error => reportRefreshFault(next.pluginId, error)),
            },
            { pluginId, appId, viewState, replaceCurrent: prepared !== undefined },
            (retired, retireError) => console.warn(`switchToPluginApp: predecessor ${retired.pluginId}/${retired.app.id} retirement failed`, retireError),
          );
          if (outcome.status === "draining" || outcome.status === "busy") {
            showTransientNoticeRef.current(surfaceSwitchBusyTextV1(uiLocale), "info");
            return null;
          }
          return outcome.session === null || (documentSource === undefined && prepared === undefined) ? outcome.session : { ...outcome.session, documentInitialized: true };
        } catch (switchError) {
          if (documentSource === undefined || !documentLoadCancelledV1(switchError)) throw switchError;
          documentTransfer().notifyDocumentTransfer("load-cancelled", resolveManifestLabel(documentSource.app.label, uiTerminology, uiLocale) || documentSource.app.id, "info");
          return null;
        }
      } finally {
        retirement?.release();
        setSurfaceSwitchBusy(false);
      }
    },
    [loadedPlugins, refreshUi, reportRefreshFault, retireSessionInstance, session, appLabelsOverlay, hostMode, hostApp, landingAppId, uiTerminology, uiLocale, rememberOpenSpaceId],
  );

  const switchToPluginApp: (pluginId: string, appId: string, viewState?: ViewModel, documentSource?: ActiveSession) => Promise<ActiveSession | null> = switchAppSession;

  /** 📥️ Mounts an already initialized standalone archive through the session switch and retirement owner. */
  const adoptPreparedPrimarySession = useCallback(async (appId: string, prepared: PreparedPrimarySessionV1): Promise<void> => {
    await switchAppSession(prepared.handle.pluginId, appId, undefined, undefined, prepared);
  }, [switchAppSession]);

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, artifactJson: string, viewState: ViewModel) => {
    try {
      const document = JSON.parse(artifactJson) as Record<string, unknown>;
      const targetSession: ActiveSession = { pluginId: plugin.pluginId, instanceId: pluginInstanceId, app, viewState: spawnedProgramViewStateV1(viewState, app) };
      await plugin.handleAction(pluginInstanceId, encodeWindowActionInvocation(targetSession, { controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: SpaceProgramEntry, isCurrent: () => boolean, label?: string, osInstanceId?: string, artifactJson?: string, sourceViewState?: ViewModel, document?: DocumentOpeningReference): Promise<SpacePanelState | null> => {
      if (!isCurrent()) return null;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      const host = sessionRef.current;
      if (!pluginEntry || !host) return null;
      const held = (): boolean => isCurrent() && !pluginOpInFlightRef.current.has(program.pluginId) && loadedPluginsRef.current.some(entry => entry.handle === pluginEntry.handle) && sessionRef.current?.pluginId === host.pluginId && sessionRef.current.app.id === host.app.id && sessionRef.current.instanceId === host.instanceId;
      if (!held()) return null;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      if (document !== undefined && app === undefined) return null;
      const currentPanel = parsePanelState(sourceViewState ?? host.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      const attach = async (instanceId: number, ownerApp: AppDefinition): Promise<DocumentOpeningReceiptV1 | null> => {
        if (document === undefined) return null;
        const targetSession: ActiveSession = { pluginId: program.pluginId, instanceId, app: ownerApp, viewState: spawnedProgramViewStateV1(sourceViewState ?? host.viewState, ownerApp), documentInitialized: true };
        return openDocumentRef.current(document, undefined, { session: targetSession, plugin: pluginEntry.handle, current: held, predecessorClientInstanceId: null });
      };
      if (existing) {
        if (document !== undefined) {
          const ownerApp = pluginEntry.manifest.apps.find((candidate) => candidate.id === existing.appId);
          if (ownerApp === undefined || existing.pluginId !== program.pluginId) return null;
          const mounted = [...openDocumentSessionsRef.current.values()].find((entry) => entry.plugin === pluginEntry.handle && entry.session.instanceId === existing.instanceId);
          if (mounted) {
            const reference = mounted.openingReference;
            if (reference.documentId !== document.documentId || reference.schema !== document.schema || reference.spaceId !== document.spaceId) return null;
          } else {
            const receipt = await attach(existing.instanceId, ownerApp);
            if (receipt === null) return null;
            if (!held()) {
              closeDocumentRef.current(receipt.runtimeKey, receipt.clientInstanceId);
              return null;
            }
          }
        }
        if (artifactJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, artifactJson, sourceViewState ?? host.viewState);
        }
        return held() ? studioPanelFocusingSpawned(currentPanel, existing) : null;
      }
      const instanceId = await createAdmittedShellInstanceV1(held, () => pluginEntry.handle.createApp(program.appId), (id) => pluginEntry.handle.destroyApp(id));
      if (instanceId === null) return null;
      let receipt: DocumentOpeningReceiptV1 | null = null;
      let opened = false;
      try {
        if (document !== undefined && app !== undefined) {
          receipt = await attach(instanceId, app);
          if (receipt === null) return null;
        } else if (artifactJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, artifactJson, sourceViewState ?? host.viewState);
        }
        opened = held();
        if (!opened) return null;
        return studioPanelFocusingSpawned(currentPanel, {
          id: osInstanceId ?? `${program.pluginId}-${instanceId}`,
          pluginId: program.pluginId,
          instanceId,
          appId: program.appId,
          label: label ?? program.label,
          breadcrumb: program.breadcrumb,
        });
      } finally {
        if (!opened) {
          if (receipt !== null) closeDocumentRef.current(receipt.runtimeKey, receipt.clientInstanceId);
          await documentAttachmentLanesRef.current.get(pluginEntry.handle)?.get(instanceId)?.drain();
          await pluginEntry.handle.destroyApp(instanceId);
        }
      }
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
  return { switchToPluginApp, adoptPreparedPrimarySession, ensureSpawnedPlugin };
}
