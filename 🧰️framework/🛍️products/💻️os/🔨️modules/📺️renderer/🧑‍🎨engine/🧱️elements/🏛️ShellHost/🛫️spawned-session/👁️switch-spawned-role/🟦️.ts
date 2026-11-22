import { useCallback } from "../../../⚛️react-port/🟦️.ts";
import { type AppDefinition } from "@semio-tech/framework";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { buildSpacePanelState, parsePanelState, viewStateWithSpacePanel, type SpacePanelState, type SpawnedAppEntry } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { quiesceSessionWorkV1, surfaceSwitchBusyTextV1 } from "../../🔀️surface-switch/🟦️.ts";
import { prepareDocumentSurfaceV1 } from "../../🔀️surface-switch/📄️document/🟦️.ts";
import type { useDocumentServicesState } from "../🔶️document-services/🟦️.ts";
import type { useUpdateSpacePanelState } from "../🔸️update-space-panel/🟦️.ts";

import { browserDocumentActorIdentityV1 } from "../🟢️request-service-operation/🪪️owner/🟦️.ts";
import { settleRoleDocumentV1 } from "./🚦️settle-document/🟦️.ts";

/** 🪪️ Checks the exact Workflow surface being replaced, even when another node now has focus. */
function sameSpawnedSurface(entry: SpawnedAppEntry, source: SpawnedAppEntry): boolean {
  return entry.id === source.id && entry.pluginId === source.pluginId && entry.appId === source.appId && entry.instanceId === source.instanceId;
}

/** 🪟️ Replaces one live surface without changing the node ID, siblings or current panel focus. */
export function replaceSpawnedSurfaceV1(panel: SpacePanelState, source: SpawnedAppEntry, appId: string, instanceId: number): SpacePanelState {
  if (!panel.spawnedApps.some((entry) => sameSpawnedSurface(entry, source))) return panel;
  return buildSpacePanelState(
    panel.spawnedApps.map((entry) => sameSpawnedSurface(entry, source) ? { ...entry, appId, instanceId } : entry),
    panel.activePanelTab,
    panel.activeSpawnedId,
  );
}

type SpawnedRolePorts = Pick<ReturnType<typeof useDocumentServicesState>,
  "shellStateRef" | "loadedPluginsRef" | "openDocumentSessionsRef" | "openDocumentRef" | "closeDocumentRef"
  | "documentAttachmentLanesRef" | "sessionWorkRef" | "sealedInstancesRef" | "sessionSwitchGateRef"
  | "setSurfaceSwitchBusy" | "dispatch" | "showTransientNoticeRef" | "uiLocale" | "ensureBackboneWorker" | "browserActorUiByRuntimeKeyRef">
  & Pick<ReturnType<typeof useUpdateSpacePanelState>, "retireSessionInstance">;

/** 👁️ Switches the same mounted Workflow document through the shared document-surface preparation. */
export function useSwitchSpawnedRole({ shellStateRef, loadedPluginsRef, openDocumentSessionsRef, openDocumentRef, closeDocumentRef, documentAttachmentLanesRef,
  sessionWorkRef, sealedInstancesRef, sessionSwitchGateRef, setSurfaceSwitchBusy, dispatch, showTransientNoticeRef, uiLocale, ensureBackboneWorker, browserActorUiByRuntimeKeyRef, retireSessionInstance }: SpawnedRolePorts) {
  return useCallback(async (source: SpawnedAppEntry, app: AppDefinition): Promise<void> => {
    const host = shellStateRef.current.pluginRuntime.session;
    const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === source.pluginId);
    const predecessorApp = pluginEntry?.manifest.apps.find((candidate) => candidate.id === source.appId);
    if (!host || !pluginEntry || !predecessorApp || !app.dialect) return;
    const { handle } = pluginEntry;
    const { readAppDocumentArchive, loadAppDocumentArchive } = handle;
    const owners = [...openDocumentSessionsRef.current].filter(([, entry]) => entry.plugin === handle && entry.session.instanceId === source.instanceId && entry.session.app.id === source.appId);
    if (owners.length !== 1) throw new Error("surface-document.owner-unavailable");
    const [[runtimeKey, owner]] = owners;
    const scope = owner.scope;
    const identity = browserDocumentActorIdentityV1(owner, browserActorUiByRuntimeKeyRef.current.get(runtimeKey));
    if (scope === undefined && (!readAppDocumentArchive || !loadAppDocumentArchive)) throw new Error("surface-document.archive-port-unavailable");
    if (scope !== undefined && identity === null) throw new Error("surface-document.browser-actor-unavailable");
    const current = (): boolean => {
      const live = shellStateRef.current.pluginRuntime.session;
      return live !== null && shellDialogSessionIsCurrentV1(host, live)
        && loadedPluginsRef.current.some((entry) => entry.handle === handle)
        && parsePanelState(live.viewState)?.spawnedApps.some((entry) => sameSpawnedSurface(entry, source)) === true;
    };
    const gate = sessionSwitchGateRef.current;
    if (!current() || gate.busy()) return;
    const predecessor: ActiveSession = { pluginId: source.pluginId, instanceId: source.instanceId, app: predecessorApp, viewState: spawnedProgramViewStateV1(host.viewState, predecessorApp) };
    const targetState = spawnedProgramViewStateV1(host.viewState, app);
    const targetSurfaceId = canonicalSurfaceId(app.dialect, app.role);
    const bindings = owner.persistenceBindings.map((binding) => binding.kind === "hub" ? { ...binding, requestedSurfaceId: targetSurfaceId } : binding);
    setSurfaceSwitchBusy(true);
    try {
      const outcome = await gate.run({
        session: predecessor,
        resolveApp: () => app,
        appId: (definition) => definition.id,
        quiesce: () => quiesceSessionWorkV1(() => sessionWorkRef.current.pending(source.pluginId, source.instanceId)),
        seal: () => sealedInstancesRef.current.seal(handle, source.pluginId, source.instanceId),
        unseal: () => sealedInstancesRef.current.unseal(handle, source.pluginId, source.instanceId),
        createInstance: async () => {
          let receipt: DocumentOpeningReceiptV1 | null = null;
          return prepareDocumentSurfaceV1({
            current,
            capture: async () => {
              if (scope === undefined) return readAppDocumentArchive!(source.instanceId);
              const attached = (): boolean => current() && openDocumentSessionsRef.current.get(runtimeKey) === owner
                && browserDocumentActorIdentityV1(owner, browserActorUiByRuntimeKeyRef.current.get(runtimeKey)) === identity;
              if (!await settleRoleDocumentV1(ensureBackboneWorker(), scope, owner.clientInstanceId, attached)) throw new Error("surface-document.shared-head-unconfirmed");
              return null;
            },
            create: () => handle.createApp(app.id),
            restore: async (instanceId, archive) => { if (archive !== null) await loadAppDocumentArchive!(instanceId, archive); },
            attach: async (instanceId) => {
              const session: ActiveSession = { pluginId: source.pluginId, instanceId, app, viewState: targetState, ...(scope === undefined ? { documentInitialized: true } : {}) };
              receipt = await openDocumentRef.current(owner.openingReference, bindings, { plugin: handle, session, current, predecessorClientInstanceId: owner.clientInstanceId });
              if (receipt !== null) return;
              if (scope !== undefined && current() && openDocumentSessionsRef.current.get(runtimeKey) === owner
                && browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.clientInstanceId !== owner.clientInstanceId) {
                const restored = await openDocumentRef.current(owner.openingReference, owner.persistenceBindings, {
                  plugin: handle, session: predecessor, current, predecessorClientInstanceId: owner.clientInstanceId,
                });
                if (restored === null && current()) throw new Error("surface-document.predecessor-restore-failed");
              }
              throw new Error("surface-document.attachment-refused");
            },
            release: async (instanceId) => {
              if (receipt !== null) closeDocumentRef.current(receipt.runtimeKey, receipt.clientInstanceId);
              await documentAttachmentLanesRef.current.get(handle)?.get(instanceId)?.drain();
              await handle.destroyApp(instanceId);
            },
          });
        },
        retire: () => retireSessionInstance(predecessor),
        defaultViewState: () => targetState,
        publish: (next) => dispatch({ type: "SET_SESSION", value: (live) => {
          if (!live || !shellDialogSessionIsCurrentV1(host, live)) return live;
          const panel = parsePanelState(live.viewState);
          if (panel === null) return live;
          const replacement = replaceSpawnedSurfaceV1(panel, source, app.id, next.instanceId);
          return replacement === panel ? live : { ...live, viewState: viewStateWithSpacePanel(live.viewState, replacement) };
        } }),
        seedLayout: () => {},
        refresh: async () => {},
      }, { pluginId: source.pluginId, appId: app.id });
      if (outcome.status === "draining") showTransientNoticeRef.current(surfaceSwitchBusyTextV1(uiLocale), "info");
    } finally {
      setSurfaceSwitchBusy(false);
    }
  }, [dispatch, ensureBackboneWorker, retireSessionInstance, setSurfaceSwitchBusy, uiLocale]);
}
