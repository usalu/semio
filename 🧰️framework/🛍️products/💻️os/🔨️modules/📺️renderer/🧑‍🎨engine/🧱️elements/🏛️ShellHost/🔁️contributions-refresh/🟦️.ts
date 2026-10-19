/** 🔁️ Contribution publishing, the ui refresh pass, its coalescing lane and refreshUi.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useContributionsAndRefresh` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { useCallback, useEffect, useRef } from "react";
// #region 🔌️Adapters
import {
  scopeContributionsJson,
  type Effect,
  normalizeAppLabelsOverlay,
  panelTabKindId,
  pendingWindowUiNode,
  type PluginAppLabelsOverlay,
  type PluginUiRefreshSectionResponse,
  resolveExternalSlots,
  hasExternalSlots,
  retireContributorInstances,
  partitionRefreshWindowInstancesV1,
  type BuiltNode,
  type UiDirtyScope
} from "@semio-tech/framework";
import { contributionsReceiverSessionV1, withProgramEntriesV1 } from "../🪟️spawned-program/🟦️.ts";
import { MEDIA_TRANSPORT_EXTENSION_ID } from "../../🎬️MediaTransportHost/🟦️.tsx";
import { type ActiveSession, type LoadedProgramState, localProgramsV1, programPluginIdV1, sessionProgramsV1 } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { type ExtraWindowInstance } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { toolRunTraceCursorViewState } from "../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import {
  applyFrameworkLayoutSeed,
  flattenPanelTabLeaves,
  withoutUiRefreshSectionsV1,
  pluginShouldReceiveContributions,
  retitleWindowLayoutNode
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import {
  applyUiRefreshResponseToCache,
  buildActiveUtilityByWindowId,
  buildUiRefreshRequest,
  createUiRefreshCoalescerV1,
  type UiRefreshCoalescerV1,
  uiRefreshAlreadyAnsweredV1,
  mergeRecordPreservingIdentity,
  mergeUiDirtyScopeV1,
  preserveJsonIdentity,
  sessionWindowInstances
} from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
// #endregion 🔌️Adapters
import { retireSkippedWindowBodies } from "../🪟️mounted-window-refresh/🟦️.ts";
import { createContributionsPublisher, type ContributionsPublishOutcome, type ContributionsSessionKey } from "../../🛠️ShellHelpers/🧩️contributions/🟦️.ts";
import { guestIngressGenerationV1 } from "../../🔌️PluginRuntime/🟦️.tsx";
import { hopTrace } from "@semio-tech/framework/trace";
import { seededActiveWindowId } from "../🎚️shell-model/🟦️.tsx";
import { appCommandTakesPageRun, appOwnsCommand, encodeAppCommandInvocation } from "../📨️window-invocation/🟦️.ts";
import { traceCommitToNextFrame } from "../🔌️extension-runtime/🟦️.ts";
import { logUnlessRetiredV1, publishPanelBodiesV1, withLocalizedWindowKindLabels } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterFocusAndInput } from "../🎮️focus-and-input/🟦️.tsx";

//#region FrameworkOsShell
/** 🧩️ What one contributions push is computed against — captured per publish, never read off a
 * render closure, so the unit a superseded refresh started still finishes against the closure it
 * was started with. */
type ContributionsEnvironment = {
  readonly loadedPlugins: readonly LoadedProgramState[];
  readonly disabledExtensionIds: ReadonlySet<string>;
  readonly hostMode: boolean;
  readonly session: ActiveSession;
  readonly targetViewState: ViewModel;
  /** 🎛️ The receiver's `consumes` row — the ONLY thing that scopes a capability pack. */
  readonly consumedTopics: readonly string[];
  /** 🔁️ Runs an install's own re-arm effects against the RECEIVER — the program the pack was pushed into,
   * with its own app — never against whatever session happens to be primary. */
  readonly dispatchDeferredEffects: (receiver: ActiveSession, effects: readonly Effect[]) => void;
};

/** 🚪️ `hostInputs` says this request carries a render input the GUEST has not seen — view state, an
 * armed utility, a locale switch, a newly mounted window. It is the default, because a request that
 * cannot prove otherwise must never be joined onto a pass that crossed before it; only the
 * completion path, which knows whether its own effects rewrote a guest render input, lowers it. */
type UiRefreshLaneRequest = { readonly session: ActiveSession; readonly scope: UiDirtyScope; readonly extraInstances?: readonly ExtraWindowInstance[]; readonly replaceBodies: boolean; readonly hostInputs: boolean };

export type ShellCoreAfterContributionsAndRefresh = ShellCoreAfterFocusAndInput & ReturnType<typeof useContributionsAndRefresh>;

export function useContributionsAndRefresh(core: ShellCoreAfterFocusAndInput) {
  const {
    hostMode, dispatch, session, uiLocale, uiTerminology, forceReloadLiveUiStoresV1, appLabelsOverlay,
    refreshGenerationRef, replaceBodiesGenerationRef, appRegistrationsJsonRef, contributorInstancesRef,
    contributorPassesRef, externalSlotNodesRef, layoutSeedKeyRef, extraWindowCounterRef, extraWindowInstancesRef,
    uiRefreshCacheRef, sessionRef, treeWindowSchedulerRef, refreshUiRef, panelTreeConfigCacheRef, builtNodeStoreCacheRef,
    identityRef, refreshDirectoryHomeRef, captureDialogOrigin, captureEffectOwner, captureProgramEffectOwner,
    browserActorForSessionV1, publishUiRefreshSectionsV1, loadedPluginsRef, registry, extensionLedgerRef, spawnedIdsRef,
    activeUtilityByWindowIdRef, activeWindowIdRef, shellStateRef, injectActiveTool, resolvedTargetViewState
  } = core;
  //#region 🧩️ContributionsPush
  
  /**
   * 🧩️ The host→guest contributions push, owned OUTSIDE `refreshUi`'s generation race.
   *
   * `refreshUi` bumps `refreshGenerationRef` on every call and abandons itself whenever the
   * generation moves under one of its awaits. The push used to sit inside that guard and `await` a
   * guest document read, so a guest re-arming a faulting `flowEvalTick` (which settles into another
   * refresh) superseded every push mid-read and the closure never crossed — 45 s of live console
   * with zero `[TRACE] contributions …` lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The unit
   * now lives in {@link createContributionsPublisher} and awaits nothing before the crossing: the pack
   * is cut by the receiver's `consumes` row alone (never by a document read), once per registry
   * generation, and installed keyed by `(instanceId, content)`.
   */
  const contributionsPublisherRef = useRef<ReturnType<typeof createContributionsPublisher<ContributionsEnvironment>> | null>(null);
  if (contributionsPublisherRef.current === null) {
    contributionsPublisherRef.current = createContributionsPublisher<ContributionsEnvironment>({
      registryGeneration: (environment) => `${environment.hostMode ? "host" : "focused"}|${environment.loadedPlugins.map((entry) => entry.handle.pluginId).join(",")}|${[...environment.disabledExtensionIds].sort().join(",")}`,
      buildPack: (session, environment) => {
        const loadedForScope = environment.loadedPlugins.filter((entry) => !environment.disabledExtensionIds.has(programPluginIdV1(entry))).map((entry) => ({ pluginId: programPluginIdV1(entry), manifest: { topicContributions: entry.manifest.topicContributions } }));
        const receiver = environment.loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId);
        const scopedContributionsJson = scopeContributionsJson(loadedForScope, receiver === undefined ? session.pluginId : programPluginIdV1(receiver), environment.consumedTopics);
        return scopedContributionsJson;
      },
      install: async (session, json, environment) => {
        for (const pluginEntry of environment.loadedPlugins) {
          if (!pluginEntry.manifest.apps?.length) continue;
          if (!pluginShouldReceiveContributions(pluginEntry.handle.pluginId, session.pluginId, environment.hostMode)) continue;
          const isActive = pluginEntry.handle.pluginId === session.pluginId;
          const targetApp = isActive ? environment.session.app : pluginEntry.manifest.apps.find((app) => appOwnsCommand(app, "setContributions"));
          const instances = isActive ? [session.instanceId] : await Promise.all([...contributorInstancesRef.current.values()].filter((entry) => entry.pluginId === pluginEntry.handle.pluginId && entry.appId === targetApp?.id).map((entry) => entry.instance));
          for (const instanceId of instances) {
            const skipped = !targetApp || !appOwnsCommand(targetApp, "setContributions") ? "app-owns-no-setContributions" : !pluginEntry.handle.handleCommand ? "handle-cannot-command" : instanceId == null ? "no-bound-instance" : undefined;
            const takesPageRun = targetApp !== undefined && appCommandTakesPageRun(targetApp, "setContributions");
            if (!targetApp || !pluginEntry.handle.handleCommand || instanceId == null || skipped !== undefined) continue;
            const args = takesPageRun ? { json, page: 0, pageCount: 1 } : { json };
            try {
              const wire = encodeAppCommandInvocation(pluginEntry.handle.pluginId, targetApp, "setContributions", args);
              const contributionResponse = await pluginEntry.handle.handleCommand(instanceId, wire, environment.targetViewState);
              if (contributionResponse.requestedEffects?.length) {
                environment.dispatchDeferredEffects(isActive ? environment.session : { pluginId: pluginEntry.handle.pluginId, instanceId, app: targetApp, viewState: environment.session.viewState }, contributionResponse.requestedEffects);
              }
            } catch (error) {
              console.error("setContributions command failed", pluginEntry.handle.pluginId, error instanceof Error ? error.message : String(error));
            }
          }
        }
      },
    });
  }

  /** 🧩️ Starts (or joins) the contributions unit for one session and reports what it decided. The
   * environment is captured HERE, where the live host bindings are in scope, so the unit never reads
   * a stale render closure and `refreshUi` owes it nothing but an `await`.
   *
   * 🪟️ `spawned` marks a program spawned into the host's panel as its OWN receiver. In host mode the primary
   * session is the host (Home), so the primary push is scoped to the host's `consumes` and document and never reaches a
   * spawned parent: measured live 2026-09-27 on a local-only `s` serve, no closure was installed for any of the seven
   * extension parents (flow, procedural 3d, process, sourcing, cad, imperative, playbook), playbook's block palette lacked
   * the contributed "Building Component" and procedural 3d painted no preview. A spawned push is scoped to the spawned
   * plugin's `consumes` and its own document, and installs into exactly that instance (`hostMode: false`). */
  const publishContributions = useCallback(
    async (nextSession: ActiveSession, loaded: readonly LoadedProgramState[], spawned = false): Promise<ContributionsPublishOutcome> => {
      const key: ContributionsSessionKey = { pluginId: nextSession.pluginId, instanceId: nextSession.instanceId };
      const disabledExtensionIds = new Set(extensionLedgerRef.current.filter((entry) => !entry.enabled).map((entry) => entry.extensionId));
      const outcome = await contributionsPublisherRef.current!.publish(key, {
        loadedPlugins: loaded,
        disabledExtensionIds,
        hostMode: spawned ? false : hostMode,
        session: nextSession,
        targetViewState: resolvedTargetViewState(nextSession),
        consumedTopics: registry.find((entry) => entry.pluginId === (loaded.find((program) => program.handle.pluginId === nextSession.pluginId)?.catalogModule?.pluginId ?? nextSession.pluginId))?.consumes ?? [],
        dispatchDeferredEffects: (receiver, deferredEffects) =>
          queueMicrotask(() => {
            const live = sessionRef.current;
            if (!live) {
              console.error(`[os-shell] ${deferredEffects.length} deferred effect(s) for "${receiver.pluginId}" dropped: no live session`);
              return;
            }
            const target = contributionsReceiverSessionV1(receiver, live);
            void core.applyHostEffects(deferredEffects, target, { kind: "full" }, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1(`[os-shell] contributions re-arm of "${receiver.pluginId}" #${receiver.instanceId} failed`));
          }),
      });
      return outcome;
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [captureProgramEffectOwner, hostMode, registry, resolvedTargetViewState],
  );
  /** 🔚 A session switch abandons whatever contributions unit is still resolving for the instance
   * that is going away: its closure must never be installed into an instance nobody is looking at,
   * and the next session re-resolves its own operator scope from scratch. */
  const contributionsInstanceRef = useRef<number | null>(null);
  useEffect(() => {
    const previous = contributionsInstanceRef.current;
    contributionsInstanceRef.current = session?.instanceId ?? null;
    if (previous != null && previous !== (session?.instanceId ?? null)) contributionsPublisherRef.current?.retire(previous);
  }, [session?.instanceId]);
  //#endregion 🧩️ContributionsPush

  /** 🪟️ The window ids the CURRENT mode layout mounts — assigned every render from
   * `effectiveModeLayout` (declared far below, where the layout itself is resolved). A refresh pass
   * asks the guest only for these; see {@link partitionRefreshWindowInstancesV1}. */
  const mountedWindowIdsRef = useRef<ReadonlySet<string>>(new Set<string>());
  /** 🪟️ Windows a pass skipped because the layout did not mount them. The effect beside
   * `effectiveModeLayout` fetches any of these the moment the layout DOES mount them, so a mode switch
   * that races a refresh cannot leave a freshly mounted pane on a pending body. */
  const unmountedSkippedWindowBodiesRef = useRef<ReadonlyMap<string, string>>(new Map<string, string>());
  const runUiRefreshPass = useCallback(
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }, extraInstancesOverride?: readonly ExtraWindowInstance[], replaceBodies = false) => {
      if (scopeArg.kind === "none") return;
      const refreshOwner = captureEffectOwner(nextSession, captureDialogOrigin(nextSession));
      const generation = ++refreshGenerationRef.current;
      if (replaceBodies) replaceBodiesGenerationRef.current = generation;
      let pendingRefreshEffects: readonly Effect[] = [];
      const loadedPlugins = loadedPluginsRef.current;
      const program = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!program) return;
      const contributionsInstalled = publishContributions(nextSession, sessionProgramsV1(loadedPlugins, nextSession.pluginId));
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      const isSessionSwitch = layoutSeedKeyRef.current !== layoutSeedKey;
      let scope = scopeArg;
      if (isSessionSwitch) {
        await retireContributorInstances(contributorInstancesRef.current, (entry) => !entry.ownerId.startsWith(`${nextSession.pluginId}:${nextSession.instanceId}:`));
        contributorPassesRef.current.clear();
        externalSlotNodesRef.current.clear();
        uiRefreshCacheRef.current = new Map();
        scope = { kind: "full" };
        treeWindowSchedulerRef.current?.reset();
        panelTreeConfigCacheRef.current = new Map();
      }
      const cache = uiRefreshCacheRef.current;
      const layoutSeed = isSessionSwitch ? applyFrameworkLayoutSeed(nextSession.app.defaultLayout, withLocalizedWindowKindLabels(nextSession.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale) : undefined;
      const extraInstancesForFetch = extraInstancesOverride ?? layoutSeed?.extraInstances ?? extraWindowInstancesRef.current;
      const windowInstances = sessionWindowInstances(nextSession.app, extraInstancesForFetch);
      dispatch({
        type: "SET_WINDOW_UI_BY_WINDOW_ID",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            windowInstances.map((instance) => [instance.id, current[instance.id] ?? pendingWindowUiNode()] as const),
          ),
      });
      const appRegistrationsJson = JSON.stringify(localProgramsV1(loadedPlugins).flatMap((entry) => (entry.manifest.apps ?? []).map((app) => ({ pluginId: entry.handle.pluginId, app }))));
      const viewState: ViewModel = injectActiveTool({
        ...nextSession.viewState,
        locale: uiLocale,
        terminology: uiTerminology,
        sessionIdentity: identityRef.current ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName } : undefined,
        windowInstances: windowInstances.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        toolRunTraceCursorByWindowId: toolRunTraceCursorViewState(windowInstances.map((instance) => instance.id)),
        activeUtilityId: undefined,
        focusedWindowId: activeWindowIdRef.current ?? undefined,
        ...treeWindowSchedulerRef.current!.viewStateFields(),
      });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      const { fetched: fetchWindowInstances, skipped: unmountedWindowInstances } = partitionRefreshWindowInstancesV1(windowInstances, isSessionSwitch ? new Set<string>() : mountedWindowIdsRef.current);
      if (unmountedWindowInstances.length) {
        for (const instance of unmountedWindowInstances) {
          cache.delete(`window:${instance.id}`);
          externalSlotNodesRef.current.delete(`${nextSession.pluginId}:${nextSession.instanceId}:window:${instance.id}`);
        }
        await retireSkippedWindowBodies(unmountedWindowInstances, () => unmountedSkippedWindowBodiesRef.current, value => { unmountedSkippedWindowBodiesRef.current = value; }, async instance => {
          const ownerId = `${nextSession.pluginId}:${nextSession.instanceId}:window:${instance.id}`;
          await retireContributorInstances(contributorInstancesRef.current, entry => entry.ownerId === ownerId);
        });
      }
      const built = buildUiRefreshRequest(scope, fetchWindowInstances, panelTabLeaves, viewState, cache);
      const request = browserActorForSessionV1(nextSession) === null ? built : withoutUiRefreshSectionsV1(built);
      if (request) {
        const response = await hopTrace.timeAsync("refresh.guest", { instanceId: nextSession.instanceId, scope: scope.kind, windows: (request.windows ?? []).length, panels: (request.panels ?? []).length }, () =>
          program.refreshUi(nextSession.instanceId, request),
        );
        if (generation !== refreshGenerationRef.current && !(replaceBodies && generation === replaceBodiesGenerationRef.current)) return;
        const slotContext = {
          plugins: new Map(loadedPlugins.filter((entry) => !extensionLedgerRef.current.some((extension) => extension.extensionId === entry.handle.pluginId && !extension.enabled)).map((entry) => [entry.handle.pluginId, entry.handle])),
          contributorInstances: contributorInstancesRef.current,
          contributorPasses: contributorPassesRef.current,
          viewState,
          onError: (extension: string, error: unknown) => console.error("[os-shell] extension render failed", extension, error),
          hostExtensions: new Set([MEDIA_TRANSPORT_EXTENSION_ID]),
        };
        const resolveIfChanged = async (entry: PluginUiRefreshSectionResponse, section: string): Promise<PluginUiRefreshSectionResponse> => {
          const ownerId = `${nextSession.pluginId}:${nextSession.instanceId}:${section}:${entry.key}`;
          const raw = (entry.value as BuiltNode | undefined) ?? externalSlotNodesRef.current.get(ownerId);
          if (!raw) return entry;
          if (hasExternalSlots(raw)) externalSlotNodesRef.current.set(ownerId, raw);
          else externalSlotNodesRef.current.delete(ownerId);
          return { ...entry, value: await resolveExternalSlots(raw, { ...slotContext, ownerId }) };
        };
        const [resolvedWindows, resolvedPanels] = await hopTrace.timeAsync("refresh.slots", { instanceId: nextSession.instanceId, scope: scope.kind }, () =>
          Promise.all([Promise.all((response.windows ?? []).map((entry) => resolveIfChanged(entry, "window"))), Promise.all((response.panels ?? []).map((entry) => resolveIfChanged(entry, "panel")))]),
        );
        if (generation !== refreshGenerationRef.current && !(replaceBodies && generation === replaceBodiesGenerationRef.current)) return;
        applyUiRefreshResponseToCache(cache, { ...response, windows: resolvedWindows, panels: resolvedPanels });
        pendingRefreshEffects = response.requestedEffects ?? [];
      }
      const closeApplySpan = hopTrace.open("refresh.apply", { instanceId: nextSession.instanceId, scope: scope.kind });
      dispatch({
        type: "SET_WINDOW_UI_BY_WINDOW_ID",
        value: (current) => {
          const entries = windowInstances.map((instance) => [instance.id, (cache.get(`window:${instance.id}`)?.value as BuiltNode | undefined) ?? current[instance.id] ?? pendingWindowUiNode()] as const);
          if (!replaceBodies) return mergeRecordPreservingIdentity(current, entries);
          const next: Record<string, BuiltNode> = { ...current };
          for (const [id, node] of entries) next[id] = node;
          return next;
        },
      });
      publishUiRefreshSectionsV1(cache, browserActorForSessionV1(nextSession));
      const freshAppLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value as Partial<PluginAppLabelsOverlay> | undefined);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, freshAppLabelsOverlay) });
      const panelStores = publishPanelBodiesV1(
        builtNodeStoreCacheRef.current,
        panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined] as const),
      );
      dispatch({ type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, spawnedIdsRef.current, panelStores)) });
      if (replaceBodies) forceReloadLiveUiStoresV1(cache);
      closeApplySpan();
      traceCommitToNextFrame({ instanceId: nextSession.instanceId, scope: scope.kind });
      if (isSessionSwitch && layoutSeed) {
        layoutSeedKeyRef.current = layoutSeedKey;
        extraWindowInstancesRef.current = layoutSeed.extraInstances;
        extraWindowCounterRef.current = layoutSeed.extraInstances.length;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: layoutSeed.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: layoutSeed.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(layoutSeed.modeLayout) });
      }
      await contributionsInstalled;
      if (pendingRefreshEffects.length) owedPassEffectsRef.current = { effects: pendingRefreshEffects, session: nextSession, owner: refreshOwner, scope: scopeArg };
      if (appRegistrationsJson) {
        const appRegistrationsPushKey = `${nextSession.instanceId}::${appRegistrationsJson}`;
        if (appRegistrationsPushKey !== appRegistrationsJsonRef.current) {
          appRegistrationsJsonRef.current = appRegistrationsPushKey;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId);
          if (pluginEntry?.handle.handleCommand && appOwnsCommand(nextSession.app, "setAppRegistrations")) {
            try {
              const wire = encodeAppCommandInvocation(nextSession.pluginId, nextSession.app, "setAppRegistrations", { json: appRegistrationsJson });
              await pluginEntry.handle.handleCommand(nextSession.instanceId, wire, resolvedTargetViewState(nextSession));
            } catch (error) {
              console.error("setAppRegistrations command failed", error instanceof Error ? error.message : String(error));
            }
          }
        }
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [appLabelsOverlay, hostMode, injectActiveTool, publishContributions, uiLocale, uiTerminology],
  );

  //#region 🤝️UiRefreshCoalescing
  
  const runUiRefreshPassRef = useRef(runUiRefreshPass);
  runUiRefreshPassRef.current = runUiRefreshPass;
  const applyHostEffectsRef = useRef<(effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<typeof captureEffectOwner>) => Promise<void>>(async () => {});
  core.applyHostEffectsRef = applyHostEffectsRef;
  /** 🤝️ The shell's one ui-refresh lane — see {@link createUiRefreshCoalescerV1} for the three
   * properties it holds (every request owed until a pass covers it, a failed pass never ends the lane,
   * and a pass may ask for another pass but never wait for one).
   *
   * Why the lane exists at all: a pass abandons itself when `refreshGenerationRef` moves under one of
   * its awaits, which is right only while a pass finishes faster than passes are requested. On the
   * served procedural editor it does not — every `flowEvalTick` completion of a converging preview
   * demands a full pass, and one guest crossing during a brep solve was measured at 15-16 s while
   * completions arrived every few seconds, so each pass was superseded before it applied and NOTHING
   * the guest re-rendered reached the shell (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
   *
   * 🔁️ A pass's own `requestedEffects` are applied OUTSIDE the pass (`owedPassEffectsRef`, voided
   * below): applying an effect re-enters `refreshUi`, and a pass that awaits its own re-entrant
   * request waits on itself. */
  const owedPassEffectsRef = useRef<{ effects: readonly Effect[]; session: ActiveSession; owner: ReturnType<typeof captureEffectOwner>; scope: UiDirtyScope } | null>(null);
  const uiRefreshLaneRef = useRef<UiRefreshCoalescerV1<UiRefreshLaneRequest> | null>(null);
  /** 🚪️ The guest ingress generation the pass in flight submitted its own turn under, `null` while no
   * pass is running. It is captured INSIDE the pass, immediately before the guest crossing, because
   * that instant — not the instant the request was made — is what a joiner must be compared against. */
  const passIngressRef = useRef<number | null>(null);
  if (!uiRefreshLaneRef.current) {
    uiRefreshLaneRef.current = createUiRefreshCoalescerV1<UiRefreshLaneRequest>(
      async (request) => {
        passIngressRef.current = guestIngressGenerationV1(request.session.instanceId);
        try {
          await hopTrace.timeAsync("refresh", { instanceId: request.session.instanceId, scope: request.scope.kind }, () =>
            runUiRefreshPassRef.current(request.session, request.scope, request.extraInstances, request.replaceBodies),
          );
        } finally {
          passIngressRef.current = null;
        }
        const owedEffects = owedPassEffectsRef.current;
        owedPassEffectsRef.current = null;
        if (owedEffects) void applyHostEffectsRef.current(owedEffects.effects, owedEffects.session, owedEffects.scope, owedEffects.owner).catch(logUnlessRetiredV1("refresh-owed host effects failed"));
      },
      (owed, next) => ({ session: next.session, scope: mergeUiDirtyScopeV1(owed.scope, next.scope), extraInstances: next.extraInstances ?? owed.extraInstances, replaceBodies: owed.replaceBodies || next.replaceBodies, hostInputs: owed.hostInputs || next.hostInputs }),
      () => {},
      (running, next) =>
        uiRefreshAlreadyAnsweredV1(
          { instanceId: running.session.instanceId, scope: running.scope, replaceBodies: running.replaceBodies, ingressAtSubmit: passIngressRef.current },
          { instanceId: next.session.instanceId, scope: next.scope, replaceBodies: next.replaceBodies, hostInputs: next.hostInputs },
          guestIngressGenerationV1(next.session.instanceId),
        ),
    );
  }
  const refreshUi = useCallback(
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }, extraInstancesOverride?: readonly ExtraWindowInstance[], replaceBodies = false, hostInputs = true) =>
      uiRefreshLaneRef.current!.request({ session: nextSession, scope: scopeArg, extraInstances: extraInstancesOverride, replaceBodies, hostInputs }),
    [],
  );
  refreshUiRef.current = refreshUi;
  //#endregion 🤝️UiRefreshCoalescing
  refreshDirectoryHomeRef.current = async (nextSession) => refreshUi(nextSession);

  useEffect(() => {
    const windowKinds = session?.app.windowKinds;
    if (!windowKinds) return;
    dispatch({
      type: "SET_SHELL_LAYOUT",
      value: (current) => (current ? retitleWindowLayoutNode(current, withLocalizedWindowKindLabels(windowKinds), extraWindowInstancesRef.current, uiTerminology, uiLocale, shellStateRef.current.layout.windowTitlesById) : current),
    });
    const live = sessionRef.current;
    if (live) void refreshUi(live, { kind: "full" }, undefined, true).catch(logUnlessRetiredV1("[os-shell] locale refresh failed"));
  }, [uiTerminology, uiLocale, refreshUi]);

  return {
    contributionsPublisherRef, publishContributions, mountedWindowIdsRef, unmountedSkippedWindowBodiesRef, refreshUi
  };
}
//#endregion FrameworkOsShell
