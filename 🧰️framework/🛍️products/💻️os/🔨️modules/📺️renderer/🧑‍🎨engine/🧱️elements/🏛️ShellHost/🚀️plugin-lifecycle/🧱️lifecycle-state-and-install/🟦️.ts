/** 🧱️ The plugin lifecycle state, refs and effects and the plugin and hub document program install callbacks. */
import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { createBundledPluginSource, createDevPluginSource, createExtensionSource, extensionRegistryFromCatalog, multiplexPluginSources, type PluginModuleAcquired, type PluginSource, resolvePlaygroundDefaultAppId, resolvePluginRegistryId } from "@semio-tech/framework";
import { DEV_STREAM_ROUTES } from "@semio-tech/framework/backbone";
import { resolveBootPrimaryAppV1 } from "../../🔀️surface-switch/🟦️.ts";
import { type UiStatus } from "@semio-tech/ui-react";
import { type LoadedProgramState, resolvePluginCanvasStatus } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { upsertLoadedProgramV1 } from "../../../🐚️Shell/♻️slice-reducers/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { EMPTY_APP_LABELS_OVERLAY, shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { applyFrameworkLayoutSeed, loadPluginModuleResilient, pluginInstallProgressTotalV1, pluginModuleStoreNoticeTextV1, pluginShouldEstablishSession, type PluginInstallOutcome } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildSpacePanelState, panelJsonFromState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { onPluginInstancesLost, PluginBootShardLostError, type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { isShardLostError } from "@semio-tech/framework/actor/shard-client/heartbeat";
import { streamMuxWatchV1 } from "@semio-tech/framework/io/stream-mux";
import type { HubPluginSourceV1 } from "@semio-tech/framework/plugin-module/hub-source/port";
import { hubCatalogClosureV1, hubCatalogOwnerOfDialectV1, hubProgramIdV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import type { TrustedPluginModuleIndexEntryV1 } from "@semio-tech/framework/plugin-module";
import { resolveRequiredHostApps } from "../../🪪️host-bootstrap/🟦️.tsx";
import { commandStallWatch, registerProgramExtensionV1 } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { seededActiveWindowId } from "../../🎚️shell-model/🟦️.tsx";
import { announcedPageStreams } from "../../🔐️identity-binding/🟦️.ts";
import { requiredHostPanelLeafId, withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterDirectoryAndRoute } from "../../🛂️directory-route/🟦️.ts";

/**
 * 🗃 Binds the shell core, the plugin lifecycle refs and the effects that keep them current.
 *
 * 🚑️ A LIVE session whose worker was taken down is recoverable exactly the way a killed boot is:
 * `ShardClient.rebuild` has already replaced the worker and `PluginRuntime` has already retired the
 * dead instance's bookkeeping (see `onPluginInstancesLost`), so a fresh `createApp` on the rebuilt
 * shard re-opens the document and the session continues.
 *
 * Without this the shard restore brought the ACTOR back and nothing else: the guest returned empty,
 * every later command page failed its liveness check with `plugin.command-page-invalid`, and the
 * session was dead until the user reloaded — measured on 2026-09-12 after a 16 s `brep.bool.cut`
 * tripped the watchdog (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️extension-evaluate-budget-2026-09-12.md`).
 *
 * If the rebuild ALSO fails there is nothing left to try in-page, so the shell says so in the
 * user's own language rather than leaving a session that answers nothing.
 */
export function usePluginLifecycleState({ core }: { readonly core: ShellCoreAfterDirectoryAndRoute }) {
  const {
    catalog, pluginFilter, appId, appRole, hostConfig, hostMode, dispatch, pluginStatusById, pluginSupervisorById,
    session, error, contributorInstancesRef, extraWindowCounterRef, extraWindowInstancesRef, rememberOpenSpaceId,
    openInstanceIdRef, sessionRef, shellRouteLedgerRef, backboneWorkerRef, hubEnv, directoryHomeOwnerRef, uiLocaleRef,
    uiTerminologyRef, retireDirectoryHomeOwner, showTransientNoticeRef, extensionFetchAbortRef, loadedPluginsRef,
    pluginModuleUrlByIdRef, pluginArtifactRebuiltAtRef, recordPluginArtifactRebuiltAt, pluginOpInFlightRef,
    pluginInstallAbortsRef, pluginInstallProgressById, notePluginInstallProgress, registry, hostPorts
  } = core;
  const extensionRoute = hostPorts?.pluginDistribution?.extensionRoute;
  const extensionStoreUrl = (suffix: string): string => {
    if (extensionRoute === undefined) throw new Error("extension-route-absent: the composition root supplied no plugin distribution port");
    return `${extensionRoute}${suffix}`;
  };
  //#region 🔌️PluginRuntime
  /** 🔌️ The one registry entry the shell must have loaded before it can create a session — the studio
   * host plugin (`hostConfig.pluginId`) in studio mode, otherwise the resolved single-app variant.
   * Every other registry entry streams in independently and is never fatal to boot. */
  const primaryPluginId = useMemo(() => hostConfig?.pluginId ?? (pluginFilter ? resolvePluginRegistryId(catalog, pluginFilter) : undefined) ?? registry[0]?.pluginId, [catalog, hostConfig, pluginFilter, registry]);
  const shellPluginCanvasStatus = useMemo((): UiStatus | undefined => {
    return resolvePluginCanvasStatus(!!session, error, primaryPluginId ? pluginStatusById[primaryPluginId] : undefined, primaryPluginId ? pluginSupervisorById[primaryPluginId] : undefined);
  }, [session, error, primaryPluginId, pluginStatusById, pluginSupervisorById]);
  /** 🌎️ A shell with a hub installs a module this device does not serve from the hub's trusted catalog into its
   * persisted plugin module store (the plugin distribution port's hub source); `null` without a hub. Its notices reach the person
   * through the shell's own transient notice, in their language. */
  const hubPluginSource: HubPluginSourceV1 | null = useMemo(
    () =>
      hubEnv
        ? hostPorts?.pluginDistribution?.hubPluginSource({
            onNotice: (notice) => showTransientNoticeRef.current(pluginModuleStoreNoticeTextV1(notice, uiLocaleRef.current), notice.kind === "quota-exceeded" ? "error" : "info", `plugin-module-store.${notice.kind}`),
          }) ?? null
        : null,
    [hubEnv, hostPorts?.pluginDistribution],
  );
  /** 🔌️ Dev hosts stream availability on the page's stream channel (`DEV_STREAM_ROUTES`, one WebSocket for every stream of
   * the page and its workers, never an HTTP/1.1 connection held per stream); shipped static bundles replay one immediate
   * `snapshot` per tree ({@link createBundledPluginSource}) so dependency contributors install without
   * Vite middleware. The modules staged beside the shell answer first; the hub source answers what this
   * device does not serve — local-first, and only when something opens it: the hub source announces nothing. */
  const extensionRegistry = useMemo(() => extensionRegistryFromCatalog(catalog), [catalog]);
  const pluginSource: PluginSource = useMemo(() => {
    const local = import.meta.env.PROD
      ? [createBundledPluginSource(registry), createBundledPluginSource(extensionRegistry)]
      : [
          createDevPluginSource(registry, streamMuxWatchV1(announcedPageStreams(), DEV_STREAM_ROUTES.pluginModules)),
          createExtensionSource(catalog, streamMuxWatchV1(announcedPageStreams(), DEV_STREAM_ROUTES.extensionModules)),
        ];
    return multiplexPluginSources(...local, ...(hubPluginSource ? [hubPluginSource] : []));
  }, [catalog, extensionRegistry, registry, hubPluginSource]);

  /** 🔌️ Recreates the primary session instance for `handle` — the exact `hostConfig`/non-studio
   * app-resolution logic the boot effect used to run once inline, now shared with `reloadPlugin` so a
   * hot-swap of the session-owning plugin re-establishes the session the same way boot does.
   *
   * 🗣️ Locale and terminology are read through their refs, and are deliberately NOT dependencies. They
   * only label the layout SEED, i.e. they matter at the instant a session is established — which is
   * exactly what a ref answers — while an already-established session's baked-in titles are kept in the
   * current language by the retitle effect below (`syncShellLabelLocale`'s neighbour). Depending on them
   * reactively made this callback's identity change on every language switch, which propagated through
   * `installPlugin`/`reloadPlugin` into the `PluginSource` subscription effect's dep array; that effect's
   * re-run opens a FRESH `EventSource`, whose connect-time `snapshot` replays every already-built plugin
   * as a hot-swap, and a hot-swap of the session-owning plugin destroys the live instance —
   * `actor-activation.revoked` followed by `no channel for instance N` for everything still addressed to
   * it (ticket 26/09/02 wave B38 §1.7, fixed in wave B40). A UI-language change is a view-context
   * update; it must never reach the plugin install graph. */
  const establishPrimarySession = useCallback(
    async (handle: PluginWasmHandle) => {
      const manifest = handle.manifest;
      if (hostConfig) {
        const hostApps = resolveRequiredHostApps(manifest.apps, hostConfig);
        const sApp = hostApps.landing;
        const panelState = buildSpacePanelState([], requiredHostPanelLeafId(hostApps.host));
        const instanceId = await handle.createApp(sApp.id);
        rememberOpenSpaceId(null);
        openInstanceIdRef.current = null;
        shellRouteLedgerRef.current.invalidate();
        const viewState: ViewModel = { activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id, panelJson: panelJsonFromState(panelState) };
        const seeded = applyFrameworkLayoutSeed(sApp.defaultLayout, withLocalizedWindowKindLabels(sApp.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminologyRef.current, uiLocaleRef.current);
        extraWindowInstancesRef.current = seeded.extraInstances;
        extraWindowCounterRef.current = seeded.extraInstances.length;
        dispatch({ type: "SET_SESSION", value: { pluginId: handle.pluginId, instanceId, app: sApp, viewState } });
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
        dispatch({ type: "SET_ERROR", value: null });
        return;
      }
      const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(catalog, pluginFilter) : undefined;
      const primaryApp = resolveBootPrimaryAppV1(manifest.apps, appId, defaultAppId, appRole);
      if (appId !== undefined && primaryApp === undefined) {
        throw new Error(`primary plugin ${handle.pluginId} does not declare pinned app ${appId}`);
      }
      if (!primaryApp) return;
      const instanceId = await handle.createApp(primaryApp.id);
      const seeded = applyFrameworkLayoutSeed(primaryApp.defaultLayout, withLocalizedWindowKindLabels(primaryApp.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminologyRef.current, uiLocaleRef.current);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({
        type: "SET_SESSION",
        value: { pluginId: handle.pluginId, instanceId, app: primaryApp, viewState: { activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id } },
      });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
      dispatch({ type: "SET_ERROR", value: null });
    },
    [catalog, hostConfig, appId, appRole, pluginFilter, rememberOpenSpaceId],
  );

  /** 🚑️ ONE watchdog kill must not be a fatal boot. Losing the shard that was running the primary
   * plugin's first turn is a recoverable event by construction — `ShardClient.rebuild` has already
   * replaced the worker and `ActivationRegistry.handleShardLost` has already bumped and resumed every
   * actor that was on it — so the session is established once more against the rebuilt shard before
   * anything is reported. Only a SECOND loss is real, and it fails as a typed
   * {@link PluginBootShardLostError} (`plugin.boot.shard-lost`) that names the cause instead of
   * surfacing a bare `shard 0 terminated`. Every other boot error propagates untouched and unretried. */
  const establishPrimaryWithShardRetry = useCallback(
    async (handle: PluginWasmHandle): Promise<void> => {
      try {
        await establishPrimarySession(handle);
      } catch (bootError) {
        if (!isShardLostError(bootError)) throw bootError;
        try {
          await establishPrimarySession(handle);
        } catch (retryError) {
          if (!isShardLostError(retryError)) throw retryError;
          throw new PluginBootShardLostError(handle.pluginId, retryError);
        }
      }
    },
    [establishPrimarySession],
  );

  useEffect(() => {
    return onPluginInstancesLost((_shardIndex, lost) => {
      const session = sessionRef.current;
      if (!session) return;
      const ours = lost.find((entry) => entry.pluginId === session.pluginId && entry.instanceId === session.instanceId);
      if (!ours) return;
      const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === ours.pluginId)?.handle;
      if (!handle) {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.workerLost") });
        return;
      }
      void establishPrimaryWithShardRetry(handle).catch((error: unknown) => {
        console.error(`ShellHost: could not re-establish ${ours.pluginId} after a worker loss`, error);
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.workerLost") });
      });
    });
  }, [establishPrimaryWithShardRetry]);
  return { pluginOpInFlightRef, loadedPluginsRef, hubPluginSource, registry, pluginSource, dispatch, pluginInstallAbortsRef, notePluginInstallProgress, pluginModuleUrlByIdRef, recordPluginArtifactRebuiltAt, primaryPluginId, sessionRef, establishPrimaryWithShardRetry, appId, pluginStatusById, pluginInstallProgressById, extensionRegistry, directoryHomeOwnerRef, backboneWorkerRef, retireDirectoryHomeOwner, contributorInstancesRef, hostMode, establishPrimarySession, pluginArtifactRebuiltAtRef, catalog, uiLocaleRef, uiTerminologyRef, extraWindowInstancesRef, extensionStoreUrl, extensionFetchAbortRef, extensionRoute, shellPluginCanvasStatus };
}

/** 📥 Builds the install plugin callback and the hub document program install callbacks. */
export function usePluginInstallCallbacks({ pluginOpInFlightRef, loadedPluginsRef, hubPluginSource, registry, pluginSource, dispatch, pluginInstallAbortsRef, notePluginInstallProgress, pluginModuleUrlByIdRef, recordPluginArtifactRebuiltAt, primaryPluginId, sessionRef, establishPrimaryWithShardRetry, appId, pluginStatusById, pluginInstallProgressById, extensionRegistry }: Pick<ReturnType<typeof usePluginLifecycleState>, "pluginOpInFlightRef" | "loadedPluginsRef" | "hubPluginSource" | "registry" | "pluginSource" | "dispatch" | "pluginInstallAbortsRef" | "notePluginInstallProgress" | "pluginModuleUrlByIdRef" | "recordPluginArtifactRebuiltAt" | "primaryPluginId" | "sessionRef" | "establishPrimaryWithShardRetry" | "appId" | "pluginStatusById" | "pluginInstallProgressById" | "extensionRegistry">) {
  /** 🔌️ Installs a registry entry that isn't loaded yet: acquires its module (worker-backed, refcounted
   * — see `acquirePluginModule`), upserts it into `loadedPlugins`, and — if this is the primary plugin
   * and no session exists yet — establishes the session. Shared by the boot effect (primary plugin
   * only) and the `PluginSource` subscription effect (every other plugin, as its build lands).
   * `loadedPluginsRef` advances with the upsert, not a render later, so a caller that awaited this install
   * (an opening that first installs its kind's owner) routes over the set that already holds it. */
  const installPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number): Promise<PluginInstallOutcome> => {
      if (pluginOpInFlightRef.current.has(pluginId)) return "in-flight";
      if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === pluginId)) return "already-loaded";
      pluginOpInFlightRef.current.add(pluginId);
      await hubPluginSource?.ready().catch((error: unknown) => console.warn("plugin module store unavailable", error));
      const entry = registry.find((candidate) => candidate.pluginId === pluginId) ?? (await pluginSource.list().catch(() => [])).find((candidate) => candidate.pluginId === pluginId);
      if (!entry) {
        pluginOpInFlightRef.current.delete(pluginId);
        return "missing-registry";
      }
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "installing" });
      const abort = new AbortController();
      pluginInstallAbortsRef.current.set(pluginId, abort);
      try {
        let acquired: PluginModuleAcquired;
        try {
          acquired = await pluginSource.acquireModule(pluginId, rebuiltAt, { signal: abort.signal, onProgress: (progress) => notePluginInstallProgress(pluginId, progress) });
        } catch (error) {
          if (abort.signal.aborted) console.debug(`plugin install cancelled ${pluginId}`);
          else console.error("program module unavailable", pluginId, error);
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "failed" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
          return "failed";
        }
        const handle = await loadPluginModuleResilient(pluginId, acquired.moduleUrl, abort.signal);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "failed" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
          return "failed";
        }
        pluginModuleUrlByIdRef.current.set(pluginId, acquired.moduleUrl);
        recordPluginArtifactRebuiltAt(pluginId, acquired.rebuiltAt);
        loadedPluginsRef.current = upsertLoadedProgramV1(loadedPluginsRef.current, { handle, manifest: handle.manifest });
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "loaded" });
        const shouldEstablish = pluginShouldEstablishSession(pluginId, primaryPluginId, sessionRef.current !== null);
        if (shouldEstablish) {
          try {
            await establishPrimaryWithShardRetry(handle);
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "running" });
          } catch (bootError) {
            console.error("Framework OS boot failed", bootError);
            dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
            return "failed";
          }
        }
        return "loaded";
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
        notePluginInstallProgress(pluginId, null);
        if (pluginInstallAbortsRef.current.get(pluginId) === abort) pluginInstallAbortsRef.current.delete(pluginId);
      }
    },
    [registry, pluginSource, hubPluginSource, primaryPluginId, establishPrimaryWithShardRetry, appId, recordPluginArtifactRebuiltAt, notePluginInstallProgress],
  );

  /** 🎬️ Every plugin whose install is running right now, from the SAME `"installing"` status the
   * install path already dispatches — no second notion of "an install is happening" to keep in sync. */
  const installingPluginIds = useMemo(() => Object.keys(pluginStatusById).filter((pluginId) => pluginStatusById[pluginId] === "installing").sort(), [pluginStatusById]);
  const pluginInstallProgress = useMemo(() => pluginInstallProgressTotalV1(pluginInstallProgressById, installingPluginIds), [pluginInstallProgressById, installingPluginIds]);

  /** 🛑️ Stops every install currently in flight — what the install band's one cancel control does.
   * Each aborted load settles as a refusal through `loadPluginModuleResilient`'s own `null`, so the
   * plugin lands in `"failed"` exactly as any other refused install would, with no second path. */
  const cancelPluginInstalls = useCallback(() => {
    for (const abort of pluginInstallAbortsRef.current.values()) abort.abort();
  }, []);

  /** ⏱️ Every command turn holding its program's lane past the stall bound (`⏱️command-stall`), with a once-a-second clock
   * while any is held so the band's seconds count up; its cancel rejects that turn and releases the lane. */
  const commandStalls = useSyncExternalStore(commandStallWatch.subscribe, commandStallWatch.stalls);
  const [commandStallClockMs, setCommandStallClockMs] = useState(() => performance.now());
  useEffect(() => {
    if (commandStalls.length === 0) return;
    setCommandStallClockMs(performance.now());
    const timer = setInterval(() => setCommandStallClockMs(performance.now()), 1_000);
    return () => clearInterval(timer);
  }, [commandStalls.length]);

  /** 🌎️ Installs one module of a catalog generation as a hub program (`🌎️hub-source/🔍️resolution`) — from the store, this
   * device's byte-identical staged module, or the hub — with the same install band, byte progress and cancel as every
   * other install, and loads it beside the device's own program of that plugin. Concurrent openings join one install. */
  const hubProgramInstallsRef = useRef(new Map<string, Promise<LoadedProgramState | null>>());
  const installHubProgram = useCallback(
    (generationId: string, entry: TrustedPluginModuleIndexEntryV1, peers: Readonly<Record<string, string>>): Promise<LoadedProgramState | null> => {
      const programId = hubProgramIdV1(entry.pluginId, entry.bundleSha256);
      const loaded = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === programId);
      if (loaded !== undefined) return Promise.resolve(loaded);
      const running = hubProgramInstallsRef.current.get(programId);
      if (running !== undefined) return running;
      const install = (async (): Promise<LoadedProgramState | null> => {
        if (hubPluginSource === null) return null;
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "installing" });
        const abort = new AbortController();
        pluginInstallAbortsRef.current.set(programId, abort);
        try {
          const staged = registry.find((candidate) => candidate.pluginId === entry.pluginId) ?? extensionRegistry.find((candidate) => candidate.pluginId === entry.pluginId);
          const program = await hubPluginSource.installProgram(generationId, entry, staged?.moduleUrl ?? null, { signal: abort.signal, onProgress: (progress) => notePluginInstallProgress(programId, progress) });
          const handle = await loadPluginModuleResilient(programId, program.moduleUrl, abort.signal, entry.pluginId);
          if (!handle) {
            dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "failed" });
            return null;
          }
          const state: LoadedProgramState = { handle, manifest: handle.manifest, catalogModule: { pluginId: entry.pluginId, generationId, bundleSha256: entry.bundleSha256, source: program.source, peers } };
          console.info(`[os-shell] hub program ${programId} of generation ${generationId} loaded from ${program.source}`);
          loadedPluginsRef.current = upsertLoadedProgramV1(loadedPluginsRef.current, state);
          dispatch({ type: "UPSERT_LOADED_PLUGIN", value: state });
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "loaded" });
          return state;
        } catch (error) {
          if (abort.signal.aborted) console.debug(`hub program install cancelled ${programId}`);
          else console.error("hub program unavailable", programId, error);
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "failed" });
          return null;
        } finally {
          notePluginInstallProgress(programId, null);
          if (pluginInstallAbortsRef.current.get(programId) === abort) pluginInstallAbortsRef.current.delete(programId);
        }
      })();
      hubProgramInstallsRef.current.set(programId, install);
      void install.finally(() => {
        if (hubProgramInstallsRef.current.get(programId) === install) hubProgramInstallsRef.current.delete(programId);
      });
      return install;
    },
    [hubPluginSource, registry, extensionRegistry, notePluginInstallProgress],
  );

  /** 🌎️ The program a hub document of `artifactKind` runs on: the hub catalog's current generation names the package
   * that opens the kind; that package and its same-generation closure (dependencies, extensions) are installed as hub
   * programs, dependencies first, and every extension is indexed under its parent program before anything activates.
   * `null` while the hub is unreachable, when no single package opens the kind, or when an install fails. */
  const installHubDocumentProgram = useCallback(
    async (artifactKind: string): Promise<LoadedProgramState | null> => {
      if (hubPluginSource === null) return null;
      const index = await hubPluginSource.catalog(new AbortController().signal);
      if (index === null) return null;
      const owner = hubCatalogOwnerOfDialectV1(index, artifactKind);
      const closure = owner === null ? null : hubCatalogClosureV1(index, owner.pluginId);
      if (owner === null || closure === null) return null;
      const peers: Readonly<Record<string, string>> = Object.freeze(Object.fromEntries(closure.map((entry) => [entry.pluginId, hubProgramIdV1(entry.pluginId, entry.bundleSha256)])));
      for (const entry of closure) if ((await installHubProgram(index.generationId, entry, peers)) === null) return null;
      for (const entry of closure) if (entry.extendsPluginId !== null) registerProgramExtensionV1(peers[entry.extendsPluginId]!, peers[entry.pluginId]!);
      return loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === peers[owner.pluginId]) ?? null;
    },
    [hubPluginSource, installHubProgram],
  );
  return { installPlugin, installingPluginIds, pluginInstallProgress, cancelPluginInstalls, commandStalls, commandStallClockMs, installHubDocumentProgram };
}
