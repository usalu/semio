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
import {
  type ActiveSession,
  type ExtraWindowInstance,
  type LoadedProgramState,
  localProgramsV1,
  programPluginIdV1,
  sessionProgramsV1
} from "../../🐚️Shell/🟦️.tsx";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { toolRunTraceCursorViewState } from "../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import {
  applyFrameworkLayoutSeed,
  applyUiRefreshResponseToCache,
  buildActiveUtilityByWindowId,
  buildUiRefreshRequest,
  createUiRefreshCoalescerV1,
  type UiRefreshCoalescerV1,
  uiRefreshAlreadyAnsweredV1,
  flattenPanelTabLeaves,
  mergeRecordPreservingIdentity,
  mergeUiDirtyScopeV1,
  withoutUiRefreshSectionsV1,
  pluginShouldReceiveContributions,
  preserveJsonIdentity,
  retitleWindowLayoutNode,
  sessionWindowInstances
} from "../../🛠️ShellHelpers/🟦️.tsx";
// #endregion 🔌️Adapters
import { retireSkippedWindowBodies } from "../🪟️mounted-window-refresh/🟦️.ts";
import { createContributionsPublisher, type ContributionsPublishOutcome, type ContributionsSessionKey } from "../../🛠️ShellHelpers/🧩️contributions/🟦️.ts";
import { guestIngressGenerationV1 } from "../../🔌️PluginRuntime/🟦️.tsx";
import { hopTrace } from "../../../../../../../../🔨️modules/⏱️trace/🟦️.ts";
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
      // 🔢 The identity of the closure the pack is cut from: a plugin load/unload or an extension
      // toggle is a new generation and re-runs the unit; a plain refresh is not.
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
            // 📦️ One pack crossing: handleCommand already encodePackValue's the invocation
            // (`PluginRuntime` performInvocation). The 4 KiB public-invocation string cap is the
            // JSON entry point, not this path. Paging that envelope was 99 guest turns / ~202 s
            // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). An app that declares pageCount still
            // receives page 0 of 1 so the addressed schema matches.
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
    // 🐢️ `applyHostEffects` is declared later in this component and is referenced in the body only,
    // never in this array — the same temporal-dead-zone avoidance `refreshUi` below documents.
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
    // 🪟️ `extraInstancesOverride` lets a caller that just synchronously computed a NEW extra-window list
    // (split/drop, layout/mode switch) hand it straight to this fetch instead of reading `extraWindowInstances`
    // from React state, which wouldn't reflect the just-dispatched change until the next render.
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }, extraInstancesOverride?: readonly ExtraWindowInstance[], replaceBodies = false) => {
      if (scopeArg.kind === "none") return;
      const refreshOwner = captureEffectOwner(nextSession, captureDialogOrigin(nextSession));
      const generation = ++refreshGenerationRef.current;
      if (replaceBodies) replaceBodiesGenerationRef.current = generation;
      let pendingRefreshEffects: readonly Effect[] = [];
      // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — reads `loadedPluginsRef`
      // (kept in sync every render, line ~1145), NOT the `loadedPlugins` array closed over by this
      // callback: this function's own identity depended on `loadedPlugins` (deps array below), so it was
      // RECREATED on every one of the ~50+ sequential catalogue plugin loads during boot — and every
      // `useEffect` that calls `refreshUi` (line ~2461's session-refresh effect, in particular) lists
      // `refreshUi` itself in its own deps array, so a fresh `refreshUi` identity re-fired a FULL
      // `refreshUi(session)` call on the SAME already-open session for each unrelated background plugin
      // finishing its load — live-confirmed (`🧪️5-e-live-postmessage-probe.md`) as the dominant contributor
      // to the `plugin.internal: plugin instance busy` storm: dozens of overlapping `refreshUi` calls
      // colliding on the wasm guest's single-flight `InstanceGuard`, each retried up to 8× on the worker
      // side and 8× more by `withSerializedPluginWasmHandle` on the host side. Mirrors lane 5-A's own
      // `readHistory` effect fix (line ~992) one level up the call graph — that fix stopped `readHistory`
      // itself from refiring but could not stop THIS callback's identity churn, since `readHistory`'s
      // effect and this one are independent call sites, not nested.
      const loadedPlugins = loadedPluginsRef.current;
      const program = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!program) return;
      // 🧩️ Started BEFORE this refresh's first await and never guarded by `generation`: the push is
      // its own owned unit (see the `🧩️ContributionsPush` region above). A refresh superseded mid
      // document-read therefore no longer abandons it, and a later refresh joins the same run rather
      // than starting a second one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      const contributionsInstalled = publishContributions(nextSession, sessionProgramsV1(loadedPlugins, nextSession.pluginId));
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      const isSessionSwitch = layoutSeedKeyRef.current !== layoutSeedKey;
      // 🐢️ A session switch invalidates every cached hash from the previous instance — force a full
      // fetch regardless of what scope this particular call was given.
      let scope = scopeArg;
      if (isSessionSwitch) {
        await retireContributorInstances(contributorInstancesRef.current, (entry) => !entry.ownerId.startsWith(`${nextSession.pluginId}:${nextSession.instanceId}:`));
        contributorPassesRef.current.clear();
        externalSlotNodesRef.current.clear();
        uiRefreshCacheRef.current = new Map();
        scope = { kind: "full" };
        // 🪟️ Tree windows name authored keys of the app that is going away; carrying them into the next
        // app asks a guest for containers it has never heard of, and the cached `TreePanelConfig`s hold
        // stores minted against the previous session's bodies.
        treeWindowSchedulerRef.current?.reset();
        panelTreeConfigCacheRef.current = new Map();
      }
      const cache = uiRefreshCacheRef.current;
      // 🪟️ On a session switch, seed the default layout's extra instances BEFORE fetching (not after), so
      // this very first fetch already requests every default-layout pane's body/measures/engagements
      // instead of leaving newly-seeded panes to show "missing window" until some later, unrelated refresh.
      const layoutSeed = isSessionSwitch ? applyFrameworkLayoutSeed(nextSession.app.defaultLayout, withLocalizedWindowKindLabels(nextSession.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale) : undefined;
      // 🪟️ Prefer the override, then the just-computed session-switch seed, then the live ref (never the
      // render-closure snapshot) so a concurrent refresh cannot drop default-layout panes.
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
      // 🪐️ Every loaded plugin's declared apps, flattened for the space app's catalogue — an opt-in
      // hint-push (below), never a view-state field, because the space app is its own wasm component:
      // `semio_framework_os::APP_REGISTRATIONS` (populated at native/test
      // `PluginHost::load_plugin`/`hot_swap_plugin` time) lives in a separate linear memory from the
      // space app's own statically-linked copy of the same os-core crate, so nothing crosses the wasm
      // boundary unless this shell pushes it explicitly.
      const appRegistrationsJson = JSON.stringify(localProgramsV1(loadedPlugins).flatMap((entry) => (entry.manifest.apps ?? []).map((app) => ({ pluginId: entry.handle.pluginId, app }))));
      // 🧩️ Contributions are deliberately absent here: `publishContributions` (the unit started above)
      // installs them into the guest through the paged `setContributions` run, which the guest folds
      // into its own registry. Riding the aggregated closure inside this view state put 248 635
      // characters against the view-context schema's 65 536-character `panelJson`/long-field bound —
      // rejected at `parseResolvedPluginViewState` as `view context: invalid panel data`, which
      // replaced every window body with a fault card (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      const viewState: ViewModel = injectActiveTool({
        ...nextSession.viewState,
        locale: uiLocale,
        terminology: uiTerminology,
        // 🪪️ The signed-in human, stamped at plugin-call time from the live ref — the same field, from
        // the same source, as `resolvedTargetViewState`. It is NOT optional chrome: a program that
        // names the human renders a different surface without it, and the refresh pass is where every
        // window body of every app is built. `🪐️space`'s Home reads
        // `crate::home_session_identity(view_state)` and renders its EMPTY case (`No studios yet`) on
        // `None` — a signed-out state, not a fault — so the studios table of a fully signed-in shell
        // sat empty for the whole session over a hub that held the human's studio, with the directory
        // bootstrap green, the receipt byte-exact and the ACK published (ticket 26/09/18, S9 §2;
        // measured live on serve 6190 → hub 7611, and named by the guest's own
        // `s.home.session-identity-required` on the dispatch twin below).
        sessionIdentity: identityRef.current ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName } : undefined,
        windowInstances: windowInstances.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        toolRunTraceCursorByWindowId: toolRunTraceCursorViewState(windowInstances.map((instance) => instance.id)),
        activeUtilityId: undefined,
        // 🎯️ The pane the user is LOOKING at, carried into every section of this refresh. Window
        // sections are re-projected per instance by the guest and ignore it; an app-level panel body
        // has no `windowId` at all and this is the only thing that lets one address a live pane.
        focusedWindowId: activeWindowIdRef.current ?? undefined,
        // 🪟️ Every body's open containers and on-screen rows, flattened — the same fields
        // `baseDispatchViewState` carries, so a render triggered by a refresh and a render triggered by
        // an action see exactly one set of windows and never disagree about what is materialised.
        ...treeWindowSchedulerRef.current!.viewStateFields(),
      });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      // 🐢️ One batched, hash-conditional round trip replaces the old ~12 sequential
      // render/utilities/windowEngagements/windowMeasures/appLabels calls — the plugin omits payloads for
      // any section whose hash still matches what `cache` already holds.
      // 🪟️ Only the windows the mode layout mounts are fetched. A declared-but-unmounted window is a
      // whole guest render per pass — in edit mode that meant re-serializing the generate-mode preview,
      // meshes included, twice per `flowEvalTick` hop. Its cached body is dropped so nothing can serve
      // it stale, and the effect beside `effectiveModeLayout` re-fetches it the moment it is mounted.
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
        // 🩺️ Which sections this pass ASKED for and which the guest actually re-serialized — the one
        // line that separates "the guest re-rendered the previous document" from "this pass never
        // applied" (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        // ⏱️ pending_effects (e.g. flowEvalTick) wait until contributions are installed — an earlier
        // tick faults `flow.extension-not-contributed` and retires the window-transient authority.
        pendingRefreshEffects = response.requestedEffects ?? [];
      }
      // 🐢️ Merge-with-identity-preservation: unrequested/unchanged sections keep exactly the object
      // reference already in `cache` (dispatched from a prior refresh), so `mergeRecordPreservingIdentity`
      // bails on them via reference equality — this is what lets `InterpretedUiNode`'s `React.memo` (and
      // `modeWindows`'s `useMemo`) skip reconciling the whole shell on every interaction.
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
      // 🖼️ React commit + paint: the state above is dispatched, not yet reconciled, so the cost of
      // turning it into pixels is the interval from here to the next frame the compositor runs.
      traceCommitToNextFrame({ instanceId: nextSession.instanceId, scope: scope.kind });
      if (isSessionSwitch && layoutSeed) {
        layoutSeedKeyRef.current = layoutSeedKey;
        extraWindowInstancesRef.current = layoutSeed.extraInstances;
        extraWindowCounterRef.current = layoutSeed.extraInstances.length;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: layoutSeed.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: layoutSeed.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(layoutSeed.modeLayout) });
      }
      // 🧩️ The contributions unit started at the top of this refresh. `await`ing it HERE, and not
      // running it here, is what keeps `pendingRefreshEffects` (an early `flowEvalTick`) waiting until
      // the closure is installed, while leaving the push itself outside this refresh's generation
      // race (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
      await contributionsInstalled;
      // 🤝️ Handed BACK to the lane, never awaited here: applying an effect re-enters `refreshUi`, and a
      // pass that awaits its own re-entrant request waits on itself (`createUiRefreshCoalescerV1`
      // property 3). The lane voids this application and the effect's refresh lands as the next pass.
      if (pendingRefreshEffects.length) owedPassEffectsRef.current = { effects: pendingRefreshEffects, session: nextSession, owner: refreshOwner, scope: scopeArg };
      if (appRegistrationsJson) {
        const appRegistrationsPushKey = `${nextSession.instanceId}::${appRegistrationsJson}`;
        if (appRegistrationsPushKey !== appRegistrationsJsonRef.current) {
          appRegistrationsJsonRef.current = appRegistrationsPushKey;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId);
          // 🪐️ The space app explicitly declares this hidden app command; other apps never receive it.
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
    // 🐢️ `owedPassEffectsRef` is declared just below (it belongs to the lane that applies what this pass
    // asked for) — referenced here in the body only, never added to this array, which both avoids a
    // temporal-dead-zone reference-before-init and keeps this callback's identity off the effect lane.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [appLabelsOverlay, hostMode, injectActiveTool, publishContributions, uiLocale, uiTerminology],
  );

  //#region 🤝️UiRefreshCoalescing
  
  // 🧰️ The lane is built ONCE (it owns the owed slot and the drain loop, which no render may reset), so
  // it reaches the current pass and the current effect applier through refs rather than through a
  // closure — the same reason `runUiRefreshPass` itself reads `loadedPluginsRef`.
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

  /** 🗣️ Keeps already-built window titles (workbench layout, extra spawned windows) in sync on every locale/terminology switch — `refreshUi` only rebuilds `shellLayout` from scratch on a session change, so an existing session's baked-in titles would otherwise go stale.
   *
   * 🌐️ And asks the GUEST to re-render, because the titles are only the shell's own half. Every label
   * inside a guest-authored body — the outliner's section headings, its per-row Hide/Lock actions, the
   * inspector's field names — is resolved by the program against `ViewModel::locale` at the moment it
   * rendered (`🗣️terminology/🦀️.rs`'s `puzzle3d_labels`), so a language switch that requests no refresh
   * leaves every one of those bodies in the previous language for as long as nothing else happens to
   * dirty them. Measured on `:6013`: with the actor-revocation of §1 fixed, `locale-switch` brought the
   * whole shell roster back German while the outliner still read "OBJECTS / Hide / Lock / REFERENCES"
   * (ticket 26/09/02 wave B40 §3). `replaceBodies` because a language change re-authors EVERY string:
   * the hash-conditional path must not be allowed to keep a body it cached under the old locale, and
   * the live UI stores hold rendered text of their own. */
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
