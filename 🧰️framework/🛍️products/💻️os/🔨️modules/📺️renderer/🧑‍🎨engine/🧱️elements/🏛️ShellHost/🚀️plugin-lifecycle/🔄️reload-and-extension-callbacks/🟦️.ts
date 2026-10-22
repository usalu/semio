/** 🔄️ The plugin reload, space extension operation and extension install, uninstall and enable callbacks. */
import { useCallback, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type ProgramHotSwapEvent, retireContributorInstances, panelViewContext } from "@semio-tech/framework";
import { loadPluginModuleResilient } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildActiveUtilityByWindowId, sessionWindowInstances } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { panelJsonFromState, parsePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { encodeWindowActionInvocation } from "../../📨️window-invocation/🟦️.ts";
import { reloadRetainsActiveApp } from "../../🔐️identity-binding/🟦️.ts";
import type { ShellCoreAfterDirectoryAndRoute } from "../../🛂️directory-route/🟦️.ts";
import type { usePluginInstallCallbacks, usePluginLifecycleState } from "../🧱️lifecycle-state-and-install/🟦️.ts";
import type { ExtensionLedgerEntry } from "../🧰️lifecycle-types/🟦️.ts";

/** 🔁 Builds the reload plugin callback and the space extension operation dispatcher. */
export function usePluginReloadCallbacks({ pluginOpInFlightRef, loadedPluginsRef, installPlugin, dispatch, pluginSource, sessionRef, directoryHomeOwnerRef, backboneWorkerRef, retireDirectoryHomeOwner, core, contributorInstancesRef, hostMode, pluginModuleUrlByIdRef, recordPluginArtifactRebuiltAt, establishPrimarySession, primaryPluginId, pluginArtifactRebuiltAtRef, catalog, uiLocaleRef, uiTerminologyRef, extraWindowInstancesRef }: Pick<ReturnType<typeof usePluginLifecycleState>, "pluginOpInFlightRef" | "loadedPluginsRef" | "dispatch" | "pluginSource" | "sessionRef" | "directoryHomeOwnerRef" | "backboneWorkerRef" | "retireDirectoryHomeOwner" | "contributorInstancesRef" | "hostMode" | "pluginModuleUrlByIdRef" | "recordPluginArtifactRebuiltAt" | "establishPrimarySession" | "primaryPluginId" | "pluginArtifactRebuiltAtRef" | "catalog" | "uiLocaleRef" | "uiTerminologyRef" | "extraWindowInstancesRef"> & Pick<ReturnType<typeof usePluginInstallCallbacks>, "installPlugin"> & { readonly core: ShellCoreAfterDirectoryAndRoute }) {
  /** 🔌️ Hot-swaps an already-loaded plugin to a newly built module — mirrors the os-core kernel's
   * `PluginHost::hot_swap_plugin` contract (validate → destroy affected instances → swap → recreate the
   * session if it was this plugin's → release the old module) without inventing a separate one:
   * acquires the new module BEFORE tearing anything down (the old handle keeps serving concurrent
   * traffic during the swap), validates that a session-owning plugin still declares the session's app
   * id, then only commits. Extension-only programs intentionally declare no apps. A validation failure
   * disposes the new lease and leaves the old plugin exactly as it was — nothing destroyed, status back
   * to `"loaded"`. */
  const reloadPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return installPlugin(pluginId, rebuiltAt);
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "reloading" });
      dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
      let newHandle: PluginWasmHandle | null = null;
      let committed = false;
      let ownsSession = false;
      try {
        const acquired = await pluginSource.acquireModule(pluginId, rebuiltAt, { signal: new AbortController().signal });
        newHandle = await loadPluginModuleResilient(pluginId, acquired.moduleUrl);
        if (!newHandle) throw new Error(`program ${pluginId} failed to reload`);
        const activeSession = sessionRef.current;
        ownsSession = activeSession?.pluginId === pluginId;
        const activeAppId = ownsSession ? activeSession?.app.id : undefined;
        if (!reloadRetainsActiveApp(newHandle.manifest.apps, activeAppId)) {
          throw new Error(`program ${pluginId} reload dropped the active session's app "${activeAppId}"`);
        }

        const oldAppIds = new Set(current.manifest.apps.map((app) => app.id));
        const newAppIds = new Set(newHandle.manifest.apps.map((app) => app.id));
        const hotSwapEvent: ProgramHotSwapEvent = {
          pluginId,
          version: newHandle.manifest.version,
          addedApps: [...newAppIds].filter((id) => !oldAppIds.has(id)),
          removedApps: [...oldAppIds].filter((id) => !newAppIds.has(id)),
        };

        const directoryHomeOwner = directoryHomeOwnerRef.current;
        if (directoryHomeOwner?.plugin.pluginId === pluginId) {
          directoryHomeOwnerRef.current = null;
          const worker = backboneWorkerRef.current;
          await retireDirectoryHomeOwner(directoryHomeOwner, worker);
        }

        if (ownsSession && activeSession) {
          dispatch({ type: "SET_SESSION", value: null });
          await current.handle.destroyApp(activeSession.instanceId).catch(() => {});
        }
        for (const spawned of core.spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {});
        }
        await retireContributorInstances(contributorInstancesRef.current, (entry) => entry.pluginId === pluginId);
        if (hostMode && activeSession) {
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? undefined : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => (nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession),
            });
          }
        }

        pluginModuleUrlByIdRef.current.set(pluginId, acquired.moduleUrl);
        recordPluginArtifactRebuiltAt(pluginId, acquired.rebuiltAt);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle: newHandle, manifest: newHandle.manifest } });
        committed = true;

        if (ownsSession) await establishPrimarySession(newHandle);
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });

        await current.handle.dispose();
      } catch (error) {
        if (committed) {
          console.warn(`hot-swap ${pluginId} retained its committed handle; predecessor retirement failed`, error);
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
        } else {
          await newHandle?.dispose();
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
        }
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [installPlugin, establishPrimarySession, hostMode, pluginSource, retireDirectoryHomeOwner, recordPluginArtifactRebuiltAt],
  );

  /** 🔌️ Removes an already-loaded plugin: refuses the host/primary plugin and whichever plugin owns the
   * active session (there is nothing to fall back to), otherwise destroys its live instances the same
   * way `reloadPlugin` does, drops it from `loadedPlugins`, and evicts its module lease immediately
   * (rather than the pool's normal 30s linger — freeing it right away is the point of an explicit
   * uninstall). */
  const uninstallPlugin = useCallback(
    async (pluginId: string) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return;
      if (pluginId === primaryPluginId) {
        return;
      }
      if (sessionRef.current?.pluginId === pluginId) {
        return;
      }
      pluginOpInFlightRef.current.add(pluginId);
      try {
        for (const spawned of core.spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {});
        }
        await retireContributorInstances(contributorInstancesRef.current, (entry) => entry.pluginId === pluginId);
        if (hostMode && sessionRef.current) {
          const activeSession = sessionRef.current;
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? undefined : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => (nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession),
            });
          }
        }
        dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "available" });
        await current.handle.dispose();
        pluginModuleUrlByIdRef.current.delete(pluginId);
        pluginArtifactRebuiltAtRef.current.delete(pluginId);
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [primaryPluginId, hostMode],
  );
  
  const [extensionLedger, setExtensionLedger] = useState<readonly ExtensionLedgerEntry[]>([]);
  const extensionLedgerRef = useRef(extensionLedger);
  extensionLedgerRef.current = extensionLedger;
  const extensionTargetById = useMemo(() => new Map(catalog.extensions.map((target) => [target.pluginId, target] as const)), [catalog]);
  const extensionIdSet = useMemo(() => new Set(catalog.extensions.map((target) => target.pluginId)), [catalog]);

  const dispatchSpaceExtensionOp = useCallback(async (action: string, args: Record<string, unknown>) => {
    const active = sessionRef.current;
    if (!active) return;
    const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === active.pluginId);
    if (!pluginEntry) return;
    try {
      const viewState = panelViewContext({
        ...active.viewState,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        windowInstances: sessionWindowInstances(active.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(core.activeUtilityByWindowIdRef.current),
      });
      const wire = encodeWindowActionInvocation({ ...active, viewState }, { controllerId: active.app.controllerId, action, args }, extraWindowInstancesRef.current);
      await pluginEntry.handle.handleAction(active.instanceId, wire, viewState);
    } catch (error) {
    }
  }, []);
  return { extensionTargetById, setExtensionLedger, dispatchSpaceExtensionOp, reloadPlugin, uninstallPlugin, extensionLedger, extensionLedgerRef, extensionIdSet };
}

/** 🧩 Builds the install, uninstall and enable extension callbacks. */
export function useExtensionCallbacks({ extensionStoreUrl, extensionFetchAbortRef, pluginSource, pluginOpInFlightRef, dispatch, pluginModuleUrlByIdRef, extensionTargetById, setExtensionLedger, dispatchSpaceExtensionOp, extensionRoute, loadedPluginsRef, contributorInstancesRef, pluginArtifactRebuiltAtRef }: Pick<ReturnType<typeof usePluginLifecycleState>, "extensionStoreUrl" | "extensionFetchAbortRef" | "pluginSource" | "pluginOpInFlightRef" | "dispatch" | "pluginModuleUrlByIdRef" | "extensionRoute" | "loadedPluginsRef" | "contributorInstancesRef" | "pluginArtifactRebuiltAtRef"> & Pick<ReturnType<typeof usePluginReloadCallbacks>, "extensionTargetById" | "setExtensionLedger" | "dispatchSpaceExtensionOp">) {
  /** 🧩️ Installs an extension package from a URL via the extension-store HTTP endpoint when
   * available, loads its module into `loadedPlugins`, and records it on the durable ledger. */
  const installExtension = useCallback(
    async (sourceUri: string) => {
      let extensionId = "";
      let version = "0.0.0";
      let moduleUrl = "";
      let packageHash = "";
      try {
        const response = await fetch(extensionStoreUrl("/install"), {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ url: sourceUri }),
          signal: extensionFetchAbortRef.current.signal,
        });
        if (!response.ok) {
          const body = await response.text();
          throw new Error(`extension store install failed (${response.status}): ${body}`);
        }
        const result = (await response.json()) as { extensionId: string; version: string; moduleUrl: string; packageHash?: string };
        extensionId = result.extensionId;
        version = result.version;
        moduleUrl = result.moduleUrl;
        packageHash = result.packageHash ?? "";
      } catch (error) {
        const guessedId = sourceUri.split("/").filter(Boolean).pop()?.replace(/\.sxt$/i, "") ?? "";
        if (!guessedId) return;
        extensionId = guessedId;
        try {
          moduleUrl = (await pluginSource.acquireModule(extensionId, undefined, { signal: extensionFetchAbortRef.current.signal })).moduleUrl;
        } catch (resolveError) {
          return;
        }
      }
      if (!extensionId || !moduleUrl) return;
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "installing" });
      try {
        const handle = await loadPluginModuleResilient(extensionId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "failed" });
          return;
        }
        pluginModuleUrlByIdRef.current.set(extensionId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: extensionId, value: "loaded" });
        const extendsHost = extensionTargetById.get(extensionId)?.extends ?? "unscoped";
        const entry: ExtensionLedgerEntry = {
          extensionId,
          version: handle.manifest.version || version,
          sourceUri,
          packageHash,
          enabled: true,
          extendsHost,
        };
        setExtensionLedger((prev) => {
          const next = prev.filter((existing) => existing.extensionId !== extensionId);
          next.push(entry);
          return next;
        });
        void dispatchSpaceExtensionOp("installExtension", {
          extensionId: entry.extensionId,
          version: entry.version,
          sourceUri: entry.sourceUri,
          packageHash: entry.packageHash,
          enabled: entry.enabled,
        });
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionTargetById, pluginSource, extensionRoute],
  );

  /** 🧩️ Installs an extension package from a local `.sxt` / `.semio` file via the extension store. */
  const installExtensionFromFile = useCallback(
    async (file: File) => {
      let extensionId = "";
      let version = "0.0.0";
      let moduleUrl = "";
      let packageHash = "";
      try {
        const bytes = await file.arrayBuffer();
        const response = await fetch(extensionStoreUrl("/install"), {
          method: "POST",
          headers: { "content-type": "application/octet-stream" },
          body: bytes,
          signal: extensionFetchAbortRef.current.signal,
        });
        if (!response.ok) {
          const body = await response.text();
          throw new Error(`extension store install failed (${response.status}): ${body}`);
        }
        const result = (await response.json()) as { extensionId: string; version: string; moduleUrl: string; packageHash?: string };
        extensionId = result.extensionId;
        version = result.version;
        moduleUrl = result.moduleUrl;
        packageHash = result.packageHash ?? "";
      } catch (error) {
        return;
      }
      if (!extensionId || !moduleUrl) return;
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "installing" });
      const sourceUri = `file:${file.name}`;
      try {
        const handle = await loadPluginModuleResilient(extensionId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "failed" });
          return;
        }
        pluginModuleUrlByIdRef.current.set(extensionId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: extensionId, value: "loaded" });
        const extendsHost = extensionTargetById.get(extensionId)?.extends ?? "unscoped";
        const entry: ExtensionLedgerEntry = {
          extensionId,
          version: handle.manifest.version || version,
          sourceUri,
          packageHash,
          enabled: true,
          extendsHost,
        };
        setExtensionLedger((prev) => {
          const next = prev.filter((existing) => existing.extensionId !== extensionId);
          next.push(entry);
          return next;
        });
        void dispatchSpaceExtensionOp("installExtension", {
          extensionId: entry.extensionId,
          version: entry.version,
          sourceUri: entry.sourceUri,
          packageHash: entry.packageHash,
          enabled: entry.enabled,
        });
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionTargetById, extensionRoute],
  );

  /** 🧩️ Unloads an extension, drops it from the ledger, and best-effort asks the store / space
   * document to forget it. */
  const uninstallExtension = useCallback(
    async (extensionId: string) => {
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      try {
        const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === extensionId);
        if (current) {
          await retireContributorInstances(contributorInstancesRef.current, (entry) => entry.pluginId === extensionId);
          dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId: extensionId });
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "available" });
          await current.handle.dispose();
          pluginModuleUrlByIdRef.current.delete(extensionId);
          pluginArtifactRebuiltAtRef.current.delete(extensionId);
        }
        setExtensionLedger((prev) => prev.filter((entry) => entry.extensionId !== extensionId));
        void dispatchSpaceExtensionOp("uninstallExtension", { extensionId });
        try {
          await fetch(extensionStoreUrl(`/install?extensionId=${encodeURIComponent(extensionId)}`), { method: "DELETE", signal: extensionFetchAbortRef.current.signal });
        } catch {}
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionRoute],
  );

  /** 🧩️ Toggles whether an installed extension's contributions are pushed to host plugins. */
  const setExtensionEnabled = useCallback(
    async (extensionId: string, enabled: boolean) => {
      setExtensionLedger((prev) => {
        const existing = prev.find((entry) => entry.extensionId === extensionId);
        if (existing) {
          return prev.map((entry) => (entry.extensionId === extensionId ? { ...entry, enabled } : entry));
        }
        const target = extensionTargetById.get(extensionId);
        return [
          ...prev,
          {
            extensionId,
            version: "0.0.0",
            sourceUri: "",
            packageHash: "",
            enabled,
            extendsHost: target?.extends ?? "unscoped",
          },
        ];
      });
      void dispatchSpaceExtensionOp("setExtensionEnabled", { extensionId, enabled });
    },
    [dispatchSpaceExtensionOp, extensionTargetById],
  );
  return { installExtension, installExtensionFromFile, uninstallExtension, setExtensionEnabled };
}
