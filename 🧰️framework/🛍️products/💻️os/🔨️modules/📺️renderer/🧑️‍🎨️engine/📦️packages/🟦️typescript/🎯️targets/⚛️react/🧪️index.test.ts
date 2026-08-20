import { createElement, useState, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  deriveUtilityNodes,
  resolveWindowActions,
  resolveModeTools,
  partitionWindowMeasures,
  SET_ACTIVE_TOOL_ACTION_ID,
  type ActionArgDef,
  type ActionDefinition,
  type AppDefinition,
  type AppModeDefinition,
  type AppWindowKindDefinition,
  type CommandDefinition,
  type UtilityDefinition,
  type ToolDefinition,
  type WindowMeasure,
  type UtilityNode,
  type Component,
  type UiNodeRecord,
  type UiSnapshot,
  type ActionBinding,
  type LayoutSpec,
  createMemoryStoragePort,
  createTurnOutcomeBroadcast,
  type TurnOutcome,
} from "@semio-tech/framework";
import {
  ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND,
  ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND,
  ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND,
} from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🏷️brand/📦️index.ts";
import {
  ENTWERFEN_MIT_BESTAND_BRAND_IDS,
  ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION,
} from "../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts";
import { Footer, navbarFillItem, SelectionMarquee, uiDataLabel, formatKeybindingShortcut, buildKeysByActionId, type PanelTabNode, type TreeDataSection } from "@semio-tech/ui-react";
import {
  aProjectOfLuhUdkFooterItem,
  fundedByZukunftBauFooterItem,
  LUH_LOGO_URL,
  LUH_URL,
  UDK_LOGO_URL,
  UDK_URL,
  ZUKUNFT_BAU_PROJECT_URL,
} from "../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx";
import {
  Canvas2dHost,
  worldToScreenLogical,
  readCanvas2dSurfaceColors,
  Board2dHost,
  board2dCameraActionArgs,
  beginPuzzle2dPeerGesture,
  collectPuzzle2dLiveMirrorMutations,
  coalesceBoard2dEvents,
  endPuzzle2dPeerGesture,
  mapContextMenuSpecs,
  surfaceContextMenuTitleKey,
  suggestionMenuItems,
  notifyPuzzle2dPeersGestureEnded,
  parsePuzzle2dCatalogueDragPayload,
  board2dPeers,
  puzzle2dFixtureDropPreviewJson,
  puzzle2dPeerOwnsGesture,
  puzzle2dScreenToWorld,
  puzzle2dWorldToScreen,
  pushPuzzle2dLiveMirrorMutations,
  registerBoard2dPeer,
  unregisterBoard2dPeer,
  NodeGraphHost,
  catalogueGhostDescriptorJson,
  computeDagMarqueeOverlay,
  flowCatalogueItemDescriptor,
  flowRankCatalogueSuggestions,
  flowSpotlightSuggestionListScrollClass,
  nodeGraphViewportActionArgs,
  parseCatalogueAppDragPayload,
  parseDagSliderOverlays,
  GraphSliderOverlays,
  resolveFixtureWidgetInstanceId,
  Paint2dHost,
  TableHost,
  GraphTimelineHost,
  TextEditorHost,
  lineRangeAt,
  multiSpanReplace,
  World3dHost,
  brushObjectPlacementArgs,
  brushPreviewGhostMeshUrl,
  parsePuzzle3dCatalogueDragPayload,
  mergeWorldViewportCamera,
  raycastGroundPoint,
  resolveMeshStyle,
  resolveMeshSelectionPreviewStyle,
  semanticColorsFromPalette,
  celebrateWorldInstances,
  isWorldInstanceCelebrating,
  isCurveOnlyWorldMesh,
  meshBoundsCorners,
  resolveVortexPointerDownIntent,
  worldMeshMaterialRevision,
  worldVortexMaterialRevision,
  worldSuggestionMenuOwnsWindow,
  resolveWorldMergeMode,
  resolveWorldContextMenuTarget,
  shouldReattachWorldViewportCamera,
  worldCameraPoseApproxEqual,
  buildWorldCameraDispatchArgs,
  worldCameraSetCameraDispatchArgs,
  snapWorldPointToGrid,
  world3dViewportCameraSeedKey,
  worldInstancePickBlocked,
  parseWorldTerrainStyle,
  clearWorldCatalogueDropPreview,
  getWorldCatalogueDropPreview,
  clearWorldSelectionPreview,
  getWorldSelectionPreview,
  pushPuzzle2dFixtureDropPreview,
  registerWorldCatalogueDropHost,
  setWorldCatalogueDropPreview,
  subscribeWorldCatalogueDropPreview,
  setWorldSelectionPreview,
  subscribeWorldSelectionPreview,
  worldCatalogueDropHostContainsPoint,
  InkCanvasHost,
  inkItemBounds,
  eraseInkStrokePointsInItem,
  inkHtmlToParagraphs,
  inkParagraphsToHtml,
  inkResizeBounds,
  inkScaleItemWithinGroup,
  inkClipboardPayload,
  inkItemsFromClipboardPayload,
  screenToWorld,
  worldToScreen,
  type InkDocument,
  type InkStrokeItem,
  appBreadcrumb,
  appWindowLabel,
  adaptPluginHandle,
  fetchDescriptorManifest,
  applyUiPatchToRetained,
  UiDocumentStore,
  type UiInterpreterContext,
  UiPresenceOverlayContext,
  type UiPresenceOverlayEntry,
  serializePerActor,
  applyUiRefreshResponseToCache,
  resolveAppBreadcrumb,
  buildUtilityRibbonSegments,
  buildActiveUtilityByWindowId,
  buildUiRefreshRequest,
  dedupeUtilityNodesById,
  flattenPanelTabLeaves,
  groupUtilityNodesByCategory,
  initialShellState,
  selectOpenConflicts,
  selectQuarantinedConflicts,
  isFlowGraphScene,
  mergeRecordPreservingIdentity,
  parseSpaceShellPath,
  parseShellRoute,
  shellActorId,
  canonicalSurfaceId,
  directoryCommandFromAction,
  AUTO_CHECKIN_IDLE_MS,
  AUTO_CHECKIN_EDIT_THRESHOLD,
  AutoCheckinScheduler,
  canCheckIn,
  computeSyncPillState,
  syncPillText,
  ShellFaultBoundary,
  preserveJsonIdentity,
  reconcileUtilityPath,
  studioPanelFocusingSpawned,
  viewStateWithSpacePanel,
  findPressedUtilityLeafId,
  resolveUtilityNodes,
  resolveUtilities,
  panelTabDefinitionToNode,
  actionStageKey,
  actionRequiresStagedForm,
  resolveKeybindingIntent,
  resolveUtilityActivation,
  isWorldTransformGumballMode,
  worldGumballConfigForProjection,
  gumballTransformDeltaBetweenPoses,
  gumballLivePreviewDeltaBetweenPoses,
  applyGumballLivePreviewDeltaToPose,
  WindowActionPane,
  resolveCommands,
  commandAddressKey,
  commandCategories,
  buildCommandCategoryTree,
  buildCommandCategoryTabs,
  buildOsCommands,
  createLatestAsyncDispatcher,
  createDirectionalAsyncDispatcher,
  createInFlightSkippingInterval,
  createCoalescingActionDispatcher,
  createRevealCutoffStore,
  worldRevealCutoffStore,
  reconcileCommittedRevealCutoffs,
  isRevealCutoffHidden,
  PUZZLE3D_FILL_REVEAL_GROUP_ID,
  dispatchOsCommand,
  classifyWindowLayoutChange,
  buildNoteShellCommandAction,
  encodeEffectActionInvocation,
  encodeEffectCommandInvocation,
  TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS,
  mergeShellLockSources,
  parseSpacePanelState,
  resolveBootExampleId,
  resolveShellDefaults,
  resolveShellLocks,
  shouldPersistIntroductionSeen,
  shouldReplayIntroductionOnLoad,
  isEphemeralShellBrand,
  clearDurableShellStorage,
  type ResolvedCommand,
  shellReducer,
  sortUtilityNodes,
  spawnedWindowChromeForKind,
  UtilityTree,
  type UiRefreshCache,
  UIFind,
  UIFindProvider,
  uiNodeToTreePanelConfig,
  UISearch,
  type UISearchItem,
  useUIFind,
  interpretUiNode,
  dagOverlayLabelFill,
  dagOverlayLabelFillHex,
  dispatchOpenedFiles,
  scheduleDispatchAction,
  sampleMediaFrameTimestampsMs,
  runTier2VideoFrames,
  runRequestMediaFrames,
  createFrameworkDisplayPanelTabs,
  type DisplayHostApi,
  createFrameworkSettingsPanelTab,
  createFrameworkMarketplacePanelTab,
  type MarketplaceExtensionEntry,
  type MarketplaceHostApi,
  type MarketplacePluginEntry,
  type PluginPanelStatus,
  type PluginManifest,
  type PluginWasmHandle,
  resolveFrameworkLayoutSeed,
  retitleWindowLayoutNode,
  introductionTargetsWindow,
  windowMeasureTreeContainsId,
  renderWindowMeasuresTree,
  buildToolTabs,
  toolIdFromPanelTabId,
  sceneToSyncPack,
  FrameworkOsShell,
  TutorialRecorder,
  synthesizeLocalizedLabel,
  resolveManifestLabel,
  type ShellPresencePeer,
  derivePeerInteractionByDomain,
  peerIdsSelecting,
  peerIdsHovering,
} from "./📦️index.tsx";
import { decodeWorldProjectionTemplateId, encodeWorldProjectionTemplateId } from "@semio-tech/infinite-world-r3f";

//#region 🔌️jsdom polyfills
// cmdk (used by UISearch/UIFind's CommandDialog) calls ResizeObserver on mount; jsdom does not implement it.
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}
if (!globalThis.ResizeObserver) globalThis.ResizeObserver = ResizeObserverMock as unknown as typeof ResizeObserver;
// cmdk calls scrollIntoView on the active item; jsdom does not implement it.
if (!Element.prototype.scrollIntoView) Element.prototype.scrollIntoView = () => {};
//#endregion 🔌️jsdom polyfills

const noopAction = () => {};

//#region 🧪️Contract test fixtures
// 🧬️ MIGRATION (react-tests packet, ticket 26/08/20): helpers for building `UiSnapshot` fixtures
// directly against the semantic contract, mirroring `UiDocumentStore`'s/`Interpreter`'s own inline
// test `leaf`/`snapshot` helpers so fixture shape stays one convention across the package, not a
// second drifting copy.
type ContractNodeSpec = {
  readonly key: string;
  readonly component: Component;
  readonly layout?: LayoutSpec;
  readonly disabled?: boolean;
  readonly bindings?: readonly ActionBinding[];
  readonly children?: readonly ContractNodeSpec[];
};

const CONTRACT_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };

function buildContractSnapshot(root: ContractNodeSpec): UiSnapshot {
  const nodes: UiNodeRecord[] = [];
  let nextId = 0;
  const walk = (spec: ContractNodeSpec): number => {
    const id = nextId;
    nextId += 1;
    const children = (spec.children ?? []).map(walk);
    nodes.push({
      id,
      key: spec.key,
      component: spec.component,
      layout: spec.layout ?? CONTRACT_LEAF_LAYOUT,
      style: {},
      activity: "idle",
      disabled: spec.disabled ?? false,
      transition: null,
      accessibility: {},
      bindings: spec.bindings ?? [],
      menu: null,
      children,
    });
    return id;
  };
  const rootId = walk(root);
  return { surface: "test", revision: 0, root: rootId, nodes, layoutEpoch: 0 } as UiSnapshot;
}

/** 🌳️ Renders `root` (and its nested `children`) through the real `UiDocumentStore`/`interpretUiNode`
 * production path — never a hand-rolled shadow renderer — optionally under a `UiPresenceOverlayContext`
 * so hover/selection-driven markup (never a document field, per the contract) can be exercised too.
 *
 * 🪲️ PRODUCTION BUG (reported, not fixed — forbidden file): `UiDocumentStore`'s `useUiNode` calls
 * `useSyncExternalStore(subscribe, getSnapshot)` with only two arguments — no `getServerSnapshot` —
 * which React's SSR path (`renderToStaticMarkup`/`renderToString`) throws on ("Missing
 * getServerSnapshot, which is required for server-rendered content"). Every pre-migration test in
 * this describe block used `renderToStaticMarkup` (the old Interpreter had no store/hook to trip
 * this on); this helper uses client-side `render()` instead, which does not hit the SSR path — a
 * test-only workaround, not a fix for the underlying gap. See `UiDocumentStore/🟦️component.tsx`'s
 * `useUiNode`/`useUiDocumentRoot`/`useUiDocumentRevision`.
 */
function renderContractTree(root: ContractNodeSpec, presenceByKey?: Readonly<Record<string, UiPresenceOverlayEntry>>): string {
  const store = new UiDocumentStore("test");
  store.loadSnapshot(buildContractSnapshot(root));
  const context: UiInterpreterContext = { store, onAction: noopAction, onIntent: () => {} };
  const tree = interpretUiNode(store, context);
  const element = presenceByKey ? createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map(Object.entries(presenceByKey)) } }, tree) : (tree as ReactElement);
  const { container } = render(element);
  const markup = container.innerHTML;
  cleanup();
  return markup;
}
//#endregion 🧪️Contract test fixtures

describe("framework sync utilities", () => {
  it("builds three sync backbone toggles", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    expect(utilities).toHaveLength(3);
    expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
    expect(utilities[0]?.pressed).toBe(true);
  });

  it("has no active toggle when detached", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities(null);
    expect(utilities.every((utility) => !utility.pressed)).toBe(true);
  });

  it("groups File, Folder, and Remote under a single Sync category collection", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    const grouped = groupUtilityNodesByCategory(utilities as unknown as UtilityNode[], ["sync"]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0]).toMatchObject({ id: "sync", kind: "collection" });
    expect(grouped[0].kind === "collection" ? grouped[0].children.map((child) => child.id) : []).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
  });
});

describe("live measure dispatch", () => {
  it("serializes document updates and skips stale slider values", async () => {
    const values: number[] = [];
    let finishFirst = () => {};
    const dispatch = createLatestAsyncDispatcher((value: number) => {
      values.push(value);
      if (value === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });

    dispatch(1);
    dispatch(2);
    dispatch(19);
    dispatch(24);
    expect(values).toEqual([1]);

    finishFirst();
    await vi.waitFor(() => expect(values).toEqual([1, 24]));
  });

  it("coalesces straight slider movement but preserves a down-up reversal", async () => {
    const values: number[] = [];
    const finishes: Array<() => void> = [];
    const dispatch = createDirectionalAsyncDispatcher(
      (value) =>
        new Promise<void>((resolve) => {
          values.push(value);
          finishes.push(resolve);
        }),
    );

    dispatch(20);
    dispatch(18);
    dispatch(12);
    dispatch(8);
    dispatch(14);
    dispatch(20);
    expect(values).toEqual([20]);

    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toEqual([20, 8]));
    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toEqual([20, 8, 20]));
    finishes.shift()?.();
  });

  it("caps queued direction reversals so a jittery drag cannot grow the queue unbounded", async () => {
    const values: number[] = [];
    const finishes: Array<() => void> = [];
    const dispatch = createDirectionalAsyncDispatcher(
      (value) =>
        new Promise<void>((resolve) => {
          values.push(value);
          finishes.push(resolve);
        }),
    );

    dispatch(0); // starts immediately, in flight
    // Many alternating up/down values while the first dispatch is still in flight — each reversal used
    // to push a new entry onto `queued` with no bound.
    for (let i = 1; i <= 40; i += 1) {
      dispatch(i % 2 === 0 ? 100 + i : -100 - i);
    }
    expect(values).toEqual([0]);

    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toHaveLength(2));
    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toHaveLength(3));
    finishes.shift()?.();
    // Never more than 3 total dispatches (the in-flight one plus at most the capped 2 queued) despite
    // 40 requested reversals.
    expect(values).toHaveLength(3);
  });
});

describe("reveal cutoff store", () => {
  it("notifies only same-group subscribers and reflects the latest set value", () => {
    const store = createRevealCutoffStore();
    const seen: Array<number | undefined> = [];
    const unsubscribe = store.subscribe("puzzle3d-fill", (value) => seen.push(value));
    expect(store.get("puzzle3d-fill")).toBeUndefined();

    store.set("puzzle3d-fill", 5);
    expect(store.get("puzzle3d-fill")).toBe(5);
    expect(seen).toEqual([5]);

    store.set("other-group", 9);
    expect(seen, "an unrelated group id must not notify").toEqual([5]);

    unsubscribe();
    store.set("puzzle3d-fill", 12);
    expect(seen, "an unsubscribed listener must not fire again").toEqual([5]);
    expect(store.get("puzzle3d-fill")).toBe(12);
  });

  it("isRevealCutoffHidden hides instances at or past the live cutoff, and never instances without a revealIndex", () => {
    worldRevealCutoffStore.set(PUZZLE3D_FILL_REVEAL_GROUP_ID, 5);
    expect(isRevealCutoffHidden({})).toBe(false);
    expect(isRevealCutoffHidden({ revealIndex: 4 })).toBe(false);
    expect(isRevealCutoffHidden({ revealIndex: 5 })).toBe(true);
    expect(isRevealCutoffHidden({ revealIndex: 9 })).toBe(true);

    worldRevealCutoffStore.set(PUZZLE3D_FILL_REVEAL_GROUP_ID, 100);
    expect(isRevealCutoffHidden({ revealIndex: 9 })).toBe(false);
  });

  it("isRevealCutoffHidden treats a JSON null revealIndex as untagged, even at the boot cutoff of 0", () => {
    worldRevealCutoffStore.set(PUZZLE3D_FILL_REVEAL_GROUP_ID, 0);
    expect(isRevealCutoffHidden({ revealIndex: null as unknown as undefined })).toBe(false);
    expect(isRevealCutoffHidden({})).toBe(false);
    expect(isRevealCutoffHidden({ revealIndex: 0 })).toBe(true);
  });

  it("committed reveal cutoff reconciliation ignores same-value identity churn so a live fill drag is not reset by fillBuildTick", () => {
    const committedRef: { current: Readonly<Record<string, number>> } = { current: {} };

    reconcileCommittedRevealCutoffs(worldRevealCutoffStore, committedRef, { [PUZZLE3D_FILL_REVEAL_GROUP_ID]: 0 });
    expect(worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID)).toBe(0);

    worldRevealCutoffStore.set(PUZZLE3D_FILL_REVEAL_GROUP_ID, 17);
    reconcileCommittedRevealCutoffs(worldRevealCutoffStore, committedRef, { [PUZZLE3D_FILL_REVEAL_GROUP_ID]: 0 });
    expect(worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID), "fillBuildTick must not clobber a live slider drag back to the still-committed 0").toBe(17);

    reconcileCommittedRevealCutoffs(worldRevealCutoffStore, committedRef, { [PUZZLE3D_FILL_REVEAL_GROUP_ID]: 17 });
    expect(worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID)).toBe(17);
  });
});

describe("in-flight skipping interval", () => {
  it("drops overlapping ticks instead of queueing them behind a slow run", async () => {
    const runs: number[] = [];
    let finishFirst = () => {};
    const timers: Array<() => void> = [];
    const stop = createInFlightSkippingInterval(
      () => {
        runs.push(runs.length + 1);
        if (runs.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
      },
      10,
      (fn) => {
        timers.push(fn as () => void);
        return 1 as unknown as ReturnType<typeof setInterval>;
      },
      () => {},
    );

    expect(timers).toHaveLength(1);
    timers[0]!();
    timers[0]!();
    timers[0]!();
    expect(runs).toEqual([1]);

    finishFirst();
    await Promise.resolve();
    timers[0]!();
    expect(runs).toEqual([1, 2]);
    stop();
  });
});

describe("coalescing action dispatcher", () => {
  it("dedupes unchanged values and keeps at most one in-flight dispatch", async () => {
    const calls: string[] = [];
    let finishFirst = () => {};
    const send = createCoalescingActionDispatcher<string | null>((value) => {
      calls.push(value ?? "null");
      if (calls.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });
    send("a");
    send("a");
    send("b");
    expect(calls).toEqual(["a"]);
    finishFirst();
    await Promise.resolve();
    expect(calls).toEqual(["a", "b"]);
    send("b");
    expect(calls).toEqual(["a", "b"]);
  });
});

describe("shell store reducer", () => {
  const baseState = () => initialShellState({ plugins: [], storage: createMemoryStoragePort() });

  it("starts every panel anchor at the same 300px width", () => {
    const widths = Object.values(baseState().layout.panels).map((panel) => panel.size);
    expect(new Set(widths)).toEqual(new Set([300]));
  });

  it("toggles the overlays slice via a direct value without touching unrelated slices", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_SEARCH_OPEN", value: true });
    expect(next.overlays.searchOpen).toBe(true);
    expect(next.overlays.findOpen).toBe(false);
    expect(next.pluginRuntime).toBe(state.pluginRuntime);
    expect(next.uiPrefs).toBe(state.uiPrefs);
  });

  it("starts, advances, and dismisses an introduction via SET_INTRODUCTION_STEP without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.introductionStepIndex).toBeNull();
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.layout).toBe(state.layout);
    const advanced = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: (prev) => (prev ?? 0) + 1 });
    expect(advanced.overlays.introductionStepIndex).toBe(1);
    const dismissed = shellReducer(advanced, { type: "SET_INTRODUCTION_STEP", value: null });
    expect(dismissed.overlays.introductionStepIndex).toBeNull();
  });

  it("auto-starts each introduction launch once and keeps a skipped replay-on-load introduction dismissed", () => {
    const state = baseState();
    const started = shellReducer(state, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:app" });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.overlays.introductionAutoStartedKeys).toEqual(["demonstrator:app"]);

    const skipped = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: null });
    const repeatedAutoStart = shellReducer(skipped, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:app" });
    expect(repeatedAutoStart.overlays.introductionStepIndex).toBeNull();
    expect(repeatedAutoStart.overlays).toBe(skipped.overlays);

    const manuallyAdvanced = shellReducer(repeatedAutoStart, { type: "SET_INTRODUCTION_STEP", value: 2 });
    const nextApp = shellReducer(manuallyAdvanced, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:other-app" });
    expect(nextApp.overlays.introductionStepIndex).toBe(0);
    expect(nextApp.overlays.introductionAutoStartedKeys).toEqual(["demonstrator:app", "demonstrator:other-app"]);
  });

  it("COMPLETE_INTRODUCTION_INTERACTION appends and dedupes indices; SET_INTRODUCTION_STEP resets them", () => {
    const state = baseState();
    expect(state.overlays.introductionCompletedInteractions).toEqual([]);
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    const first = shellReducer(started, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 1 });
    expect(first.overlays.introductionCompletedInteractions).toEqual([1]);
    const second = shellReducer(first, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 0 });
    expect(second.overlays.introductionCompletedInteractions).toEqual([1, 0]);
    const deduped = shellReducer(second, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 1 });
    expect(deduped.overlays.introductionCompletedInteractions).toEqual([1, 0]);
    expect(deduped.layout).toBe(state.layout);
    const nextStep = shellReducer(deduped, { type: "SET_INTRODUCTION_STEP", value: 1 });
    expect(nextStep.overlays.introductionCompletedInteractions).toEqual([]);
  });

  it("opens, replaces, and closes a dialog via SET_DIALOG without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.dialog).toBeNull();
    const opened = shellReducer(state, { type: "SET_DIALOG", value: { dialogId: "addObject", seedArgs: { objectKind: "Object" } } });
    expect(opened.overlays.dialog).toEqual({ dialogId: "addObject", seedArgs: { objectKind: "Object" } });
    expect(opened.layout).toBe(state.layout);
    expect(opened.pluginRuntime).toBe(state.pluginRuntime);
    const replaced = shellReducer(opened, { type: "SET_DIALOG", value: { dialogId: "confirmDelete" } });
    expect(replaced.overlays.dialog).toEqual({ dialogId: "confirmDelete" });
    const closed = shellReducer(replaced, { type: "SET_DIALOG", value: null });
    expect(closed.overlays.dialog).toBeNull();
  });

  it("toggles the layout slice via an updater function", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const toggled = shellReducer(opened, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: (prev) => !prev });
    expect(opened.layout.panels["top-left"].visible).toBe(true);
    expect(toggled.layout.panels["top-left"].visible).toBe(false);
    expect(toggled.overlays).toBe(opened.overlays);
  });

  it("toggles a middle anchor via SET_PANEL_VISIBLE the same way as a corner", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-middle", value: true });
    expect(opened.layout.panels["top-middle"].visible).toBe(true);
    expect(opened.layout.panels["top-left"].visible).toBe(state.layout.panels["top-left"].visible);
  });

  it("rewrites window icons via SET_WINDOW_ICON for extras and base kinds", () => {
    const state = shellReducer(baseState(), {
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: [{ id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" }],
    });
    const renamedExtra = shellReducer(state, { type: "SET_WINDOW_ICON", windowId: "puzzle3d-main-top", iconId: "projection-orthographic" });
    expect(renamedExtra.layout.windowIconsById["puzzle3d-main-top"]).toBe("projection-orthographic");
    const renamedBase = shellReducer(renamedExtra, { type: "SET_WINDOW_ICON", windowId: "puzzle3d-main", iconId: "projection-three-point" });
    expect(renamedBase.layout.windowIconsById["puzzle3d-main"]).toBe("projection-three-point");
    expect(renamedBase.layout.windowIconsById["puzzle3d-main-top"]).toBe("projection-orthographic");
  });

  it("rewrites window titles via SET_WINDOW_TITLE for extras and base kinds", () => {
    const state = shellReducer(baseState(), {
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: [{ id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" }],
    });
    const renamedExtra = shellReducer(state, { type: "SET_WINDOW_TITLE", windowId: "puzzle3d-main-top", title: "Front" });
    expect(renamedExtra.layout.windowTitlesById["puzzle3d-main-top"]).toBe("Front");
    expect(renamedExtra.layout.extraWindowInstances[0]?.title).toBe("Front");
    expect(renamedExtra.overlays).toBe(state.overlays);
    const renamedBase = shellReducer(renamedExtra, { type: "SET_WINDOW_TITLE", windowId: "puzzle3d-main", title: "Isometric" });
    expect(renamedBase.layout.windowTitlesById["puzzle3d-main"]).toBe("Isometric");
    expect(renamedBase.layout.extraWindowInstances[0]?.title).toBe("Front");
  });

  it("resets the dock override, every anchor's active path/visible/size, drill-down memory, and tree expansion via RESET_DOCK", () => {
    const state = baseState();
    const rearranged = shellReducer(state, {
      type: "SET_DOCK_OVERRIDE",
      value: {
        version: 3,
        anchors: { "top-left": [{ id: "moved" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      },
    });
    const withPath = shellReducer(rearranged, { type: "SET_PANEL_PATH", anchor: "top-left", value: ["moved"] });
    const withVisible = shellReducer(withPath, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const withSize = shellReducer(withVisible, { type: "SET_PANEL_SIZE", anchor: "top-left", value: 999 });
    const withMemory = shellReducer(withSize, { type: "SET_PANEL_PATH_MEMORY", value: { moved: "child" } });
    const withTreeOpen = shellReducer(withMemory, { type: "SET_TREE_OPEN_STATE", id: "unit:section", open: true });
    const reset = shellReducer(withTreeOpen, { type: "RESET_DOCK" });
    expect(reset.layout.dockOverride).toBeNull();
    expect(reset.layout.panels["top-left"].path).toEqual([]);
    expect(reset.layout.panels["top-left"].visible).toBe(false);
    expect(reset.layout.panels["top-left"].size).toBe(state.layout.panels["top-left"].size);
    expect(reset.layout.panelPathMemory).toEqual({});
    expect(reset.layout.treeOpenStates).toEqual({});
  });

  it("HYDRATE_DOCK_UI restores a persisted size for the top-middle anchor", () => {
    const state = baseState();
    const hydrated = shellReducer(state, { type: "HYDRATE_DOCK_UI", value: { version: 3, anchors: { "top-middle": { visible: true, size: 420 } } } });
    expect(hydrated.layout.panels["top-middle"].visible).toBe(true);
    expect(hydrated.layout.panels["top-middle"].size).toBe(420);
    expect(hydrated.layout.panels["top-left"]).toEqual(state.layout.panels["top-left"]);
  });

  it("updates the uiPrefs slice and leaves the sync slice referentially unchanged", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_UI_DRIVER_ID", value: "compact" });
    expect(next.uiPrefs.uiDriverId).toBe("compact");
    expect(next.sync).toBe(state.sync);
  });

  it("action-panel slice: fold/expand/stage/reset/active-utility update only their own keys and preserve identity on no-operations", () => {
    const state = baseState();

    const folded = shellReducer(state, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false });
    expect(folded.actionPane.foldedByWindowId).toEqual({ w1: false });
    expect(folded.layout).toBe(state.layout);
    // no-operation fold keeps the whole slice referentially stable
    expect(shellReducer(folded, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false }).actionPane).toBe(folded.actionPane);

    const expanded = shellReducer(folded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" });
    expect(expanded.actionPane.expandedByWindowId).toEqual({ w1: "extrude" });
    expect(shellReducer(expanded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" }).actionPane).toBe(expanded.actionPane);

    const staged = shellReducer(expanded, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 });
    expect(staged.actionPane.stagedArgsByKey).toEqual({ "w1:extrude": { depth: 3 } });
    const stagedMore = shellReducer(staged, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "segments", value: 2 });
    expect(stagedMore.actionPane.stagedArgsByKey["w1:extrude"]).toEqual({ depth: 3, segments: 2 });
    expect(shellReducer(stagedMore, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 }).actionPane).toBe(stagedMore.actionPane);

    const reset = shellReducer(stagedMore, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" });
    expect(reset.actionPane.stagedArgsByKey["w1:extrude"]).toBeUndefined();
    // reset keeps the panel expanded
    expect(reset.actionPane.expandedByWindowId["w1"]).toBe("extrude");
    expect(shellReducer(reset, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" }).actionPane).toBe(reset.actionPane);

    const activated = shellReducer(reset, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" });
    expect(activated.actionPane.activeUtilityByWindowId).toEqual({ w1: "pen" });
    expect(shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" }).actionPane).toBe(activated.actionPane);
    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: null });
    expect(deactivated.actionPane.activeUtilityByWindowId["w1"]).toBeNull();
  });

  it("actionPane slice: SET_ACTIVE_TOOL updates only activeToolId and preserves identity on no-operations (mode-scoped, not per-window)", () => {
    const state = baseState();
    expect(state.actionPane.activeToolId).toBeNull();

    const activated = shellReducer(state, { type: "SET_ACTIVE_TOOL", toolId: "fill" });
    expect(activated.actionPane.activeToolId).toBe("fill");
    expect(activated.actionPane.activeUtilityByWindowId).toBe(state.actionPane.activeUtilityByWindowId);
    expect(shellReducer(activated, { type: "SET_ACTIVE_TOOL", toolId: "fill" }).actionPane).toBe(activated.actionPane);

    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_TOOL", toolId: null });
    expect(deactivated.actionPane.activeToolId).toBeNull();
  });

  it("commandPanel slice: expand/collapse and stage/reset update only their own keys and preserve identity on no-operations (category active/fold state now lives in layout.panels['bottom-middle'], not this slice)", () => {
    const state = baseState();

    const expanded = shellReducer(state, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");
    expect(expanded.layout).toBe(state.layout);
    expect(shellReducer(expanded, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" }).commandPanel).toBe(expanded.commandPanel);

    const staged = shellReducer(expanded, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" });
    expect(staged.commandPanel.stagedArgsByCommandId).toEqual({ "os.setThemeId": { themeId: "semio" } });
    expect(shellReducer(staged, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" }).commandPanel).toBe(staged.commandPanel);

    const reset = shellReducer(staged, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" });
    expect(reset.commandPanel.stagedArgsByCommandId["os.setThemeId"]).toBeUndefined();
    expect(shellReducer(reset, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" }).commandPanel).toBe(reset.commandPanel);
  });

  it("command palette category is the bottom-middle anchor's own SET_PANEL_PATH; the UI's category-switch handler additionally dispatches SET_COMMAND_EXPANDED:null (reproducing the old single-action collapse-on-switch behavior across two actions)", () => {
    const state = baseState();
    const onCategory = shellReducer(state, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.appearance"] });
    expect(onCategory.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.appearance"]);

    const expanded = shellReducer(onCategory, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");

    const switchedPath = shellReducer(expanded, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.layout"] });
    const switched = shellReducer(switchedPath, { type: "SET_COMMAND_EXPANDED", value: null });
    expect(switched.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.layout"]);
    expect(switched.commandPanel.expandedCommandId).toBeNull();
  });

  it("tutorial slice: SET_TUTORIAL starts a tutorial, resets rate/deviated, and clears an active introduction (mutual exclusivity)", () => {
    const introducing = shellReducer(baseState(), { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(introducing.overlays.introductionStepIndex).toBe(0);
    const started = shellReducer(introducing, { type: "SET_TUTORIAL", value: "welcome-tour" });
    expect(started.tutorial).toEqual({ activeTutorialId: "welcome-tour", playing: false, rate: 1, muted: false, captionsOn: true, recording: false, deviated: false });
    expect(started.overlays.introductionStepIndex).toBeNull();
  });

  it("tutorial slice: SET_INTRODUCTION_STEP (non-null) clears an active tutorial (mutual exclusivity, reverse direction)", () => {
    const started = shellReducer(baseState(), { type: "SET_TUTORIAL", value: "welcome-tour" });
    const playing = shellReducer(started, { type: "SET_TUTORIAL_PLAYING", value: true });
    expect(playing.tutorial.playing).toBe(true);
    const introduced = shellReducer(playing, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(introduced.tutorial.activeTutorialId).toBeNull();
    expect(introduced.tutorial.playing).toBe(false);
    expect(introduced.overlays.introductionStepIndex).toBe(0);
  });

  it("tutorial slice: play/pause resets deviated only when transitioning to playing; rate/muted/captions/recording/deviated update independently", () => {
    const started = shellReducer(baseState(), { type: "SET_TUTORIAL", value: "welcome-tour" });
    const deviated = shellReducer(started, { type: "SET_TUTORIAL_DEVIATED", value: true });
    expect(deviated.tutorial.deviated).toBe(true);
    const stillPaused = shellReducer(deviated, { type: "SET_TUTORIAL_PLAYING", value: false });
    expect(stillPaused.tutorial.deviated).toBe(true);
    const resumed = shellReducer(deviated, { type: "SET_TUTORIAL_PLAYING", value: true });
    expect(resumed.tutorial.deviated).toBe(false);
    const rated = shellReducer(resumed, { type: "SET_TUTORIAL_RATE", value: 2 });
    expect(rated.tutorial.rate).toBe(2);
    const muted = shellReducer(rated, { type: "SET_TUTORIAL_MUTED", value: true });
    expect(muted.tutorial.muted).toBe(true);
    const captionsOff = shellReducer(muted, { type: "SET_TUTORIAL_CAPTIONS", value: false });
    expect(captionsOff.tutorial.captionsOn).toBe(false);
    const recording = shellReducer(captionsOff, { type: "SET_TUTORIAL_RECORDING", value: true });
    expect(recording.tutorial.recording).toBe(true);
  });

  it("APPLY_TUTORIAL_UI_SNAPSHOT atomically restores layout/panels/tree/utility/tool/dialog/search across their owning slices", () => {
    const state = baseState();
    const snapshot = shellReducer(state, {
      type: "APPLY_TUTORIAL_UI_SNAPSHOT",
      snapshot: {
        activeWindowId: "puzzle3d-main",
        shellLayout: { kind: "window", id: "puzzle3d-main" },
        extraWindowInstances: [],
        panelPatches: { "top-left": { visible: true, path: ["catalogue"] } },
        treeOpenStates: { "catalogue.section": true },
        activeUtilityByWindowId: { "puzzle3d-main": "transform" },
        activeToolId: "fill",
        openDialogId: "addObject",
        commandPanelOpen: true,
      },
    });
    expect(snapshot.layout.activeWindowId).toBe("puzzle3d-main");
    expect(snapshot.layout.panels["top-left"]).toMatchObject({ visible: true, path: ["catalogue"] });
    expect(snapshot.layout.treeOpenStates).toEqual({ "catalogue.section": true });
    expect(snapshot.actionPane.activeUtilityByWindowId).toEqual({ "puzzle3d-main": "transform" });
    expect(snapshot.actionPane.activeToolId).toBe("fill");
    expect(snapshot.overlays.dialog).toEqual({ dialogId: "addObject" });
    expect(snapshot.overlays.searchOpen).toBe(true);
    expect(snapshot.pluginRuntime).toBe(state.pluginRuntime);
  });

  //#region 🔌️PluginRuntime hot-swap actions
  function fakeLoadedPlugin(pluginId: string, version = "0"): { readonly handle: PluginWasmHandle; readonly manifest: PluginManifest } {
    const manifest: PluginManifest = { pluginId, label: pluginId, version, apps: [], workflows: [], examples: [] };
    const handle = {
      pluginId,
      manifest,
      createApp: async () => 0,
      destroyApp: async () => {},
      handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics: [], requestedEffects: [], events: [] }),
      refreshUi: async () => ({}),
      contextMenu: async () => [],
      dispose: () => {},
    } as unknown as PluginWasmHandle;
    return { handle, manifest };
  }

  it("UPSERT_LOADED_PLUGIN inserts a new pluginId and replaces an existing one in place (order preserved)", () => {
    const state = baseState();
    const note1 = fakeLoadedPlugin("note", "1");
    const withNote = shellReducer(state, { type: "UPSERT_LOADED_PLUGIN", value: note1 });
    expect(withNote.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note"]);
    const withS = shellReducer(withNote, { type: "UPSERT_LOADED_PLUGIN", value: fakeLoadedPlugin("s") });
    expect(withS.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note", "s"]);
    const note2 = fakeLoadedPlugin("note", "2");
    const reloaded = shellReducer(withS, { type: "UPSERT_LOADED_PLUGIN", value: note2 });
    expect(reloaded.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note", "s"]);
    expect(reloaded.pluginRuntime.loadedPlugins[0]!.manifest.version).toBe("2");
    expect(reloaded.layout).toBe(withS.layout);
  });

  it("REMOVE_LOADED_PLUGIN drops only the matching pluginId", () => {
    const withBoth = [fakeLoadedPlugin("note"), fakeLoadedPlugin("s")].reduce((state, entry) => shellReducer(state, { type: "UPSERT_LOADED_PLUGIN", value: entry }), baseState());
    const removed = shellReducer(withBoth, { type: "REMOVE_LOADED_PLUGIN", pluginId: "note" });
    expect(removed.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["s"]);
  });

  it("SET_PLUGIN_STATUS tracks per-pluginId status independent of loadedPlugins membership", () => {
    const state = baseState();
    const installing = shellReducer(state, { type: "SET_PLUGIN_STATUS", pluginId: "note", value: "installing" });
    expect(installing.pluginRuntime.pluginStatusById).toEqual({ note: "installing" });
    const loaded = shellReducer(installing, { type: "SET_PLUGIN_STATUS", pluginId: "note", value: "loaded" });
    const failed = shellReducer(loaded, { type: "SET_PLUGIN_STATUS", pluginId: "s", value: "failed" });
    expect(failed.pluginRuntime.pluginStatusById).toEqual({ note: "loaded", s: "failed" });
  });
  //#endregion 🔌️PluginRuntime hot-swap actions
});

// 🕹️wave-2b: `derivePeerInteractionByDomain`/`peerIdsSelecting`/`peerIdsHovering` regroup the typed,
// wave-0 `PresenceInteraction` roster field into an app-agnostic per-domain shape — the replacement for
// today's per-app `presencePeersJson` decoding (see "s workflow flow routing"'s "renders presence peers
// from the scene payload" above, one of the only apps that renders peer selection at all today).
describe("Shell peer interaction (generic, app-agnostic)", () => {
  const peer = (clientId: string, name: string, interaction?: ShellPresencePeer["interaction"]): ShellPresencePeer => ({ clientId, name, interaction });

  it("regroups per-peer PresenceInteraction domains into a per-domain roster, keyed by clientId", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1", "n2"], hovered: [] }] }),
      peer("client-b", "Bo", { appId: "lowpoly", domains: [{ domain: "mesh", granularity: "face", selected: [], hovered: ["f7"] }] }),
    ]);
    expect(roster["graph"]).toEqual({ selectedByPeer: { "client-a": ["n1", "n2"] }, hoveredByPeer: {} });
    expect(roster["mesh"]).toEqual({ selectedByPeer: {}, hoveredByPeer: { "client-b": ["f7"] } });
  });

  it("is app-agnostic: two different apps sharing one domain id merge into the same entry", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1"], hovered: [] }] }),
      peer("client-b", "Bo", { appId: "dag", domains: [{ domain: "graph", granularity: "node", selected: ["n2"], hovered: [] }] }),
    ]);
    expect(roster["graph"]?.selectedByPeer).toEqual({ "client-a": ["n1"], "client-b": ["n2"] });
  });

  it("peerIdsSelecting/peerIdsHovering find which peers have a given target id", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1", "n2"], hovered: ["n3"] }] }),
      peer("client-b", "Bo", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n2"], hovered: [] }] }),
    ]);
    expect(peerIdsSelecting(roster, "graph", "n1")).toEqual(["client-a"]);
    expect(peerIdsSelecting(roster, "graph", "n2").sort()).toEqual(["client-a", "client-b"]);
    expect(peerIdsSelecting(roster, "graph", "n99")).toEqual([]);
    expect(peerIdsHovering(roster, "graph", "n3")).toEqual(["client-a"]);
  });

  it("is defensive about an absent interaction field (older heartbeat, or wave 2a's wire field not yet landed) and an unknown domain", () => {
    const roster = derivePeerInteractionByDomain([peer("client-a", "Ada", undefined), peer("client-b", "Bo")]);
    expect(roster).toEqual({});
    expect(peerIdsSelecting(roster, "graph", "n1")).toEqual([]);
    expect(peerIdsHovering({}, "unknown-domain", "x")).toEqual([]);
  });
});

// 🐢️ Puzzle 2D performance round 2: the per-interaction full-shell refresh cascade was dominated by
// React reconciling freshly-parsed-but-structurally-identical UiNode/engagement/measure trees on every
// action (select/camera/nodeMove). These helpers let unchanged bodies keep their object identity across
// a `refreshUi` so `InterpretedUiNode`'s `React.memo` (ui-interpreter.tsx) and `modeWindows`'s
// `useMemo` (os-shell.tsx) can bail instead of reconciling the whole shell every time.
describe("ui identity preservation (puzzle 2d perf)", () => {
  it("preserveJsonIdentity reuses the previous reference for structurally-equal values", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
  });

  it("preserveJsonIdentity returns the new reference when content actually differs", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "goodbye" };
    expect(preserveJsonIdentity(previous, next)).toBe(next);
  });

  it("preserveJsonIdentity treats nested arrays/objects structurally, not just top-level fields", () => {
    const previous = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    const next = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
    const moved = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 3 },
      ],
    };
    expect(preserveJsonIdentity(previous, moved)).toBe(moved);
  });

  it("preserveJsonIdentity treats undefined previous as always-changed", () => {
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(undefined, next)).toBe(next);
  });

  it("mergeRecordPreservingIdentity reuses the whole previous record when every key is unchanged", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(merged).toBe(prev);
  });

  it("mergeRecordPreservingIdentity reuses per-key references, replacing only the changed key", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "changed" }],
    ]);
    expect(merged).not.toBe(prev);
    expect(merged.overview).toBe(prev.overview);
    expect(merged.detail).not.toBe(prev.detail);
  });

  it("mergeRecordPreservingIdentity treats a key being added or removed as a change", () => {
    const prev = { overview: { type: "text", value: "a" } };
    const withNewKey = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(withNewKey).not.toBe(prev);
    expect(withNewKey.overview).toBe(prev.overview);
  });
});

// 🐢️ Puzzle 2D performance round 3: the batched, hash-conditional `refresh-ui` protocol that replaces
// ~12 sequential per-section WASM calls with one round trip. `buildUiRefreshRequest` restricts what's
// asked for by scope and attaches known hashes; `applyUiRefreshResponseToCache` writes back only the
// sections the plugin actually says changed.
describe("batched ui refresh request/response (puzzle 2d perf round 3)", () => {
  const windowKinds = [
    { id: "overview", bodyKey: "puzzle2d.play.overview" },
    { id: "detail", bodyKey: "puzzle2d.play.detail" },
  ];
  const panelTabLeaves = [{ kind: { kind: "app" as const, id: "framework.panel.artifact" }, bodyKey: "puzzle2d.play.layers" }];

  it("buildActiveUtilityByWindowId omits null utilities for batched refresh", () => {
    expect(buildActiveUtilityByWindowId({ top: "transform", perspective: null, brush: "brush" })).toEqual({ top: "transform", brush: "brush" });
  });

  it("buildUiRefreshRequest forwards per-window utility map on viewState without a focused-window singular leak", () => {
    const viewState = { activeUtilityByWindowId: { top: "transform", perspective: "brush" }, activeUtilityId: undefined };
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, viewState, new Map());
    expect(request?.viewState.activeUtilityByWindowId).toEqual({ top: "transform", perspective: "brush" });
    expect(request?.viewState.activeUtilityId).toBeUndefined();
  });

  it("buildActiveUtilityByWindowId makes a just-activated transform visible to refresh before the next React render", () => {
    // Regression: setActiveUtility must sync activeUtilityByWindowIdRef before refreshUi; otherwise the
    // program never stamps transform and the gumball stays hidden.
    const map: Record<string, string | null> = { "puzzle3d-main-top": null };
    map["puzzle3d-main-top"] = "transform";
    const activeUtilityByWindowId = buildActiveUtilityByWindowId(map);
    expect(activeUtilityByWindowId).toEqual({ "puzzle3d-main-top": "transform" });
    const request = buildUiRefreshRequest(
      { kind: "full" },
      [{ id: "puzzle3d-main-top", bodyKey: "puzzle3d.play.composite" }, { id: "puzzle3d-main-perspective", bodyKey: "puzzle3d.play.composite" }],
      [],
      { activeUtilityByWindowId, activeUtilityId: undefined },
      new Map(),
    );
    expect(request?.viewState.activeUtilityByWindowId).toEqual({ "puzzle3d-main-top": "transform" });
    expect(request?.viewState.activeUtilityId).toBeUndefined();
  });

  it("buildUiRefreshRequest for a full scope requests every window/panel/engagements/measures/labels section (utility bars are now registry-derived, not a plugin section)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview", "detail"]);
    expect(request?.panels?.map((p) => p.key)).toEqual(["framework.panel.artifact"]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeDefined();
    expect(request?.labels).toBeDefined();
  });

  it("buildUiRefreshRequest for none returns null", () => {
    expect(buildUiRefreshRequest({ kind: "none" }, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest for a partial scope requests only the listed window/panel bodies and flags", () => {
    const scope = { kind: "partial" as const, windowBodies: ["puzzle2d.play.overview"], panelBodies: [], engagements: true };
    const request = buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview"]);
    expect(request?.panels).toEqual([]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeUndefined();
    expect(request?.labels).toBeUndefined();
  });

  it("buildUiRefreshRequest returns null for a partial scope that matches nothing in this app", () => {
    const scope = { kind: "partial" as const, windowBodies: ["some-other-app.body"] };
    expect(buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest attaches the cached hash for a section that was already fetched once", () => {
    const cache: UiRefreshCache = new Map([["window:overview", { hash: "abc123", value: { type: "text", value: "x" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, cache);
    expect(request?.windows?.find((w) => w.key === "overview")?.hash).toBe("abc123");
    expect(request?.windows?.find((w) => w.key === "detail")?.hash).toBeUndefined();
  });

  // 🪟️ Two window INSTANCES of the same kind (e.g. a split top/perspective pane pair both rendering
  // `puzzle3d.play.main`) must get distinct request entries and distinct cache keys — never collapse
  // onto one shared entry, which is exactly the bug this ticket fixes.
  it("buildUiRefreshRequest gives two instances of the same window kind distinct keys and independent cached hashes", () => {
    const splitInstances = [
      { id: "puzzle3d-main", bodyKey: "puzzle3d.play.main" },
      { id: "puzzle3d-main-2", bodyKey: "puzzle3d.play.main" },
    ];
    const cache: UiRefreshCache = new Map([["window:puzzle3d-main", { hash: "base-hash", value: { type: "text", value: "base" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, splitInstances, [], {}, cache);
    expect(request?.windows?.map((w) => w.key)).toEqual(["puzzle3d-main", "puzzle3d-main-2"]);
    expect(request?.windows?.find((w) => w.key === "puzzle3d-main")?.hash).toBe("base-hash");
    expect(request?.windows?.find((w) => w.key === "puzzle3d-main-2")?.hash).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache writes changed sections and ignores hash-only (unchanged) ones", () => {
    const cache: UiRefreshCache = new Map([["window:detail", { hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } }]]);
    applyUiRefreshResponseToCache(cache, {
      windows: [
        { key: "overview", hash: "new-hash", value: { type: "text", value: "fresh" } },
        { key: "detail", hash: "old-hash" }, // unchanged: no `value` in the response
      ],
      engagements: { key: "engagements", hash: "eng-hash", value: { overview: {} } },
    });
    expect(cache.get("window:overview")).toEqual({ hash: "new-hash", value: { type: "text", value: "fresh" } });
    // Unchanged section: cache entry is untouched (still the old hash/value, not overwritten with nothing).
    expect(cache.get("window:detail")).toEqual({ hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } });
    expect(cache.get("engagements")).toEqual({ hash: "eng-hash", value: { overview: {} } });
  });

  it("buildUiRefreshRequest for a full scope also requests the mode-level tools section (keyed by tool id, not a window)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.tools).toBeDefined();
  });

  it("buildUiRefreshRequest for a partial scope requests tools only when the scope's `tools` flag is set", () => {
    const withTools = buildUiRefreshRequest({ kind: "partial" as const, tools: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(withTools?.tools).toBeDefined();
    const withoutTools = buildUiRefreshRequest({ kind: "partial" as const, engagements: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(withoutTools?.tools).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache caches the tools section same as measures/engagements/labels", () => {
    const cache: UiRefreshCache = new Map();
    applyUiRefreshResponseToCache(cache, { tools: { key: "tools", hash: "tools-hash", value: { fill: [] } } });
    expect(cache.get("tools")).toEqual({ hash: "tools-hash", value: { fill: [] } });
  });
});

describe("framework plugin runtime", () => {
  // 🔌️ HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS: the WIT ABI flipped from 14
  // per-verb `{json: string}` calls to `manifest`/`createApp`/`destroyApp` plus `protocol_channel::
  // AppCommand`/`AppFrame` bytes carried over the turn ABI, instead of the old flat `semio_plugin_*`
  // wasm-bindgen JSON exports, which no longer exist anywhere in the ABI
  // (`loadPluginModuleUncached`'s doc comment).
  // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H1-react, absorbing A4-channel's lease against this
  // file): channel v12 retired `AppCommand::RefreshUi`/`SectionProbe` and `AppFrame::UiSection` —
  // window-body refresh is no longer a request/response round trip at all, it is a
  // `Event::SurfaceVisible` submission read back through `TurnResult.uiPatches` via the
  // `ActivationRegistry`/`ShardClient` pair `loadPluginModule` owns (`🧱️elements/PluginRuntime/🟦️component.tsx`'s
  // `🔖️ActorAdapter` region). `adaptPluginHandle` alone (what this test constructs, via a bare
  // fake with no actor and no ShardClient) genuinely has no wire path left to ask for a
  // section body over, so its own `refreshUi` is an honest empty result — asserted here rather than
  // deleted, since "no wire path here anymore" is itself real, worth-pinning behavior.
  //
  // 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (`exchange-removal`): the raw
  // `KernelPluginWasmHandle`'s old synchronous `exchange(instanceId, frames) -> Promise<frames>`
  // per-call RPC is gone (`📌️important.md`'s "Replace, never wrap" list) — split into fire-and-forget
  // `enqueue(instanceId, events): void` plus the handle-wide `outcomes: AsyncIterable<TurnOutcome>`
  // broadcast `AppChannelClient` correlates FIFO against (`🎠️kernel/🟦️component.ts`'s
  // `PluginWasmHandle` header doc). This helper re-creates `exchange`'s old request/reply shape on
  // top of the two new primitives, purely for these fakes' own convenience — production code never
  // has a synchronous responder like this to call.
  function exchangeStyleChannel(
    respond: (instanceId: number, frames: Uint8Array[]) => Uint8Array[] | Promise<Uint8Array[]>,
  ): { readonly enqueue: (instanceId: number, events: readonly Uint8Array[]) => void; readonly outcomes: AsyncIterable<TurnOutcome> } {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    return {
      enqueue: (instanceId, events) => {
        void (async () => {
          try {
            const frames = await respond(instanceId, [...events]);
            broadcast.push({ instanceId, frames });
          } catch (error) {
            broadcast.push({ instanceId, error });
          }
        })();
      },
      outcomes: broadcast.stream,
    };
  }

  it("adaptPluginHandle's own refreshUi is an honest empty result — window-body refresh now lives in loadPluginModule's ActivationRegistry/ShardClient turn loop, which a bare no-command handle has no access to", async () => {
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: async () => encodePackValue({ pluginId: "mock-refresh", label: "Mock Refresh", version: "0", apps: [], programs: [], examples: [] }),
      createApp: async () => 7,
      destroyApp: async () => {},
      ...exchangeStyleChannel(() => {
        throw new Error("adaptPluginHandle.refreshUi must not call enqueue() — there is no AppCommand for it anymore");
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-refresh", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    await expect(handle.refreshUi(instanceId, { viewState: {}, windows: [{ key: "overview", bodyKey: "overview" }] })).resolves.toEqual({});
  });

  // 🧬️ H1-react — `acquirePluginModule`/`PluginModuleLease` (the refcounted per-plugin Worker lease)
  // are deleted outright (packet H2, `📓️terra-H2-web-shard-report.md`'s "must not exist" list),
  // replaced by `ActivationRegistry`'s manifest-only registration. `loadPluginModule` itself now
  // needs a real `Worker` (the shard pool) to fully exercise end to end, which this vitest
  // environment does not provide — the part of the old mechanism that WAS a pure, host-only
  // function (reading a build-time manifest without ever touching wasm) is `fetchDescriptorManifest`,
  // exercised directly here instead: honest-empty fallback when no `🔣️descriptor.json` is reachable
  // (true for every plugin but `🗒️note` as of this packet — `📓️status.md`'s "E2-builder-descriptor"
  // entry), and the real descriptor's `manifest` field surfacing through when one IS reachable.
  it("fetchDescriptorManifest falls back to an honest empty manifest when no 🔣️descriptor.json is reachable, and surfaces a real one when it is", async () => {
    const originalFetch = globalThis.fetch;
    try {
      globalThis.fetch = (async () => ({ ok: false, status: 404 })) as unknown as typeof fetch;
      const empty = await fetchDescriptorManifest("mock", "/plugin-modules/mock/index.js");
      expect(empty.pluginId).toBe("mock");
      expect(empty.apps).toEqual([]);

      globalThis.fetch = (async () => ({ ok: true, json: async () => ({ manifest: { pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] } }) })) as unknown as typeof fetch;
      const real = await fetchDescriptorManifest("mock", "/plugin-modules/mock/index.js");
      expect(real).toEqual({ pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] });
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  // 🧬️ H1-react — design-runtime.md §1 `SceneStore` / packet brief item 2: the retained-tree
  // reconciliation `PluginRuntime`'s `loadPluginModule` applies every `TurnResult.uiPatches` entry
  // through. Exercised directly (not through a fake wasm turn) since it is a pure function — real
  // coverage of "apply a `UiPatch` to a retained tree, honour `baseRevision`" that does not depend on
  // the unverified jco wasm boundary this file's `🔖️ActorAdapter` doc flags.
  describe("applyUiPatchToRetained", () => {
    // 🧬️ MIGRATION (react-tests packet): `PatchOp::Replace`'s payload is a whole `UiSnapshot` (root
    // pointer + flat node table), not a single recursive `UiNode` (deleted) — see
    // `PluginRuntime/🟦️component.tsx`'s own `🔖️RetainedUiPatch` doc. `RetainedSurface` is a
    // `UiDocumentState`, so a successful result's `surface.nodes` is a `ReadonlyMap`, not a bare node.
    const leaf = (id: number, value: string): UiNodeRecord => ({
      id,
      key: `leaf-${id}`,
      component: { type: "text", value, emphasize: null, dataAttributes: null },
      layout: { kind: "leaf", width: "hug", height: "hug" },
      style: {},
      activity: "idle",
      disabled: false,
      transition: null,
      accessibility: {},
      bindings: [],
      menu: null,
      children: [],
    });
    const snapshot = (revision: number, value: string): UiSnapshot => ({ surface: "s", revision, root: 0, nodes: [leaf(0, value)], layoutEpoch: 0 }) as UiSnapshot;

    it("a root Replace on a fresh surface (no previous body) is applied", () => {
      const result = applyUiPatchToRetained(null, { revision: 1, baseRevision: 0, ops: [{ kind: "Replace", path: [], snapshot: snapshot(1, "a") }] });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(1);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "a", emphasize: null, dataAttributes: null });
    });

    it("a root Replace with a matching baseRevision advances the retained body", () => {
      const { surface: previous } = applyUiPatchToRetained(null, { revision: 1, baseRevision: 0, ops: [{ kind: "Replace", path: [], snapshot: snapshot(1, "a") }] });
      const result = applyUiPatchToRetained(previous, { revision: 2, baseRevision: 1, ops: [{ kind: "Replace", path: [], snapshot: snapshot(2, "b") }] });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(2);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
    });

    it("a non-root op (no incremental walker yet) is an honest desync — the previous body is kept", () => {
      const { surface: previous } = applyUiPatchToRetained(null, { revision: 1, baseRevision: 0, ops: [{ kind: "Replace", path: [], snapshot: snapshot(1, "a") }] });
      const result = applyUiPatchToRetained(previous, { revision: 2, baseRevision: 1, ops: [{ kind: "SetProps", path: [0], props: {} }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });

    it("an ops-less patch with a stale baseRevision is a desync — nothing to reconcile against", () => {
      const { surface: previous } = applyUiPatchToRetained(null, { revision: 5, baseRevision: 0, ops: [{ kind: "Replace", path: [], snapshot: snapshot(5, "a") }] });
      const result = applyUiPatchToRetained(previous, { revision: 6, baseRevision: 1, ops: [] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });
  });

  it("parses a typed InvocationResponse, including requestedEffects, from a plugin handle-action response", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    const response = parseInvocationResponse(
      JSON.stringify({
        output: null,
        mutations: [{ diff: { payload: { schemaId: "draw.operation", document: { id: "forest" } } } }],
        inverseGroup: { invocationId: "setActiveExample:1:0", mutations: [], inverseMutations: [] },
        requestedEffects: [{ navigate: { uri: "/spaces/forest" } }],
      }),
    );
    expect(response.mutations).toHaveLength(1);
    expect(response.requestedEffects).toEqual([{ navigate: { uri: "/spaces/forest" } }]);
  });

  it("falls back to an empty InvocationResponse for malformed handle-action JSON", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    expect(parseInvocationResponse("not json")).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
    expect(parseInvocationResponse(JSON.stringify({ output: null }))).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
  });

  // 🧬️ H1-react — `withSerializedPluginWasmHandle` (which queued concurrent per-call requests
  // transparently against the old synchronous wasm handle) is deleted alongside `PluginWorkerClient`
  // (`🎠️kernel/🟦️component.ts`'s own doc comment names it). The reason it existed still applies:
  // `🟨️shard-worker.js` REJECTS (does not queue) a second in-flight `turn` for the same actor
  // (`inFlightTurnActors` guard, `🌐plugin-web-materialize.ts`). `serializePerActor`
  // (`PluginRuntime/🟦️component.tsx`'s `🔖️ActorAdapter` region) is `loadPluginModule`'s real
  // replacement — every `submitTurn` call for one actor funnels through it — exercised directly here
  // since it is a plain, generic per-key promise queue.
  it("serializePerActor queues concurrent turns for the same actor one at a time, never overlapping", async () => {
    let inFlight = 0;
    let maxInFlight = 0;
    const runOne = () =>
      serializePerActor("actor-1", async () => {
        inFlight += 1;
        maxInFlight = Math.max(maxInFlight, inFlight);
        await new Promise((resolve) => setTimeout(resolve, 5));
        inFlight -= 1;
        return "done";
      });
    const results = await Promise.all([runOne(), runOne(), runOne()]);
    expect(maxInFlight).toBe(1);
    expect(results).toEqual(["done", "done", "done"]);
  });

  it("serializePerActor keys independently per actor — different actors run concurrently, not queued behind each other", async () => {
    let concurrentAcrossActors = 0;
    let maxConcurrentAcrossActors = 0;
    const runOn = (actorId: string) =>
      serializePerActor(actorId, async () => {
        concurrentAcrossActors += 1;
        maxConcurrentAcrossActors = Math.max(maxConcurrentAcrossActors, concurrentAcrossActors);
        await new Promise((resolve) => setTimeout(resolve, 5));
        concurrentAcrossActors -= 1;
      });
    await Promise.all([runOn("actor-a"), runOn("actor-b")]);
    expect(maxConcurrentAcrossActors).toBe(2);
  });

  it("serializePerActor keeps queuing subsequent turns after an earlier one rejects", async () => {
    const order: string[] = [];
    const failing = serializePerActor("actor-2", async () => {
      order.push("first");
      throw new Error("turn faulted");
    });
    const succeeding = serializePerActor("actor-2", async () => {
      order.push("second");
      return "ok";
    });
    await expect(failing).rejects.toThrow("turn faulted");
    await expect(succeeding).resolves.toBe("ok");
    expect(order).toEqual(["first", "second"]);
  });

  // 🧬️ H1-react — `AppFrame::Effects`/`Events` no longer exist (channel v12, A4-channel). Effects
  // now travel as real `kernel::Effect` values directly on `TurnResult.effects`
  // (`⚛️reactor/🦀️component.rs`'s `poll`), demuxed by `loadPluginModule`'s turn loop into
  // `pendingTurnEffects`/drained by `performInvocation` — a mechanism a bare command-only fake (no
  // ShardClient turn ever runs) has nothing to populate, so `requestedEffects` is honestly `[]` here.
  // `output`/`uiScope`/`historyPatch` still arrive on the SAME `AppFrame::Invocation` frame, unchanged
  // by the flip — real wire coverage, kept.
  it("adaptPluginHandle.handleAction round-trips an action's output/uiScope/historyPatch from AppFrame::Invocation; requestedEffects is honestly empty for a bare command-only handle", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, decodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: async () => encodePackValue({ pluginId: "mock-action", label: "Mock Action", version: "0", apps: [], programs: [], examples: [] }),
      createApp: async () => 3,
      destroyApp: async () => {},
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("Command" in command)) throw new Error("expected a Command");
        const invocation = decodePackValue(new Uint8Array(command.Command.command));
        return [
          encodeAppFrame({ Invocation: { in_reply_to: command.Command.seq, output: Array.from(encodePackValue({ echo: invocation })), diagnostics: Array.from(encodePackValue([])), ui_scope: Array.from(encodePackValue({ kind: "partial", windowBodies: ["graph"], utilities: false })), history_patch: Array.from(encodePackValue({ cursor: 1, upserts: [] })), messages: [] } }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-action", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    const invocation = { address: { pluginId: "mock-action", appId: "main", modeId: "edit", windowKindId: "main", windowInstanceId: "main", actionId: "addShot" }, arguments: { format: "png" } };
    const response = await handle.handleAction(instanceId, JSON.stringify(invocation), {});
    expect(response.output).toEqual({ echo: invocation });
    expect(response.requestedEffects).toEqual([]);
    expect(response.uiScope).toEqual({ kind: "partial", windowBodies: ["graph"], utilities: false });
    expect(response.historyPatch).toEqual({ cursor: 1, upserts: [] });
  });

  it("adaptPluginHandle exposes setMergePolicy/resolveConflict/readConflicts and sends the real AppCommand wire frames — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane K2: the merge-policy Settings control and Conflicts panel Accept/Discard used to call `plugin.handle.setMergePolicy?.(…)` against a handle that never had the method, so the optional call silently no-opped — this asserts the method genuinely exists AND that calling it round-trips through the real `AppCommand`/`AppFrame` codecs, not an internal spy", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue } = await import("@semio-tech/framework-os");
    const sentCommands: unknown[] = [];
    const fakeHandle = {
      manifest: async () => encodePackValue({ pluginId: "mock-merge", label: "Mock Merge", version: "0", apps: [], programs: [], examples: [] }),
      createApp: async () => 9,
      destroyApp: async () => {},
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        sentCommands.push(command);
        if (command && typeof command === "object" && "setMergePolicy" in command) return [];
        if (command && typeof command === "object" && "resolveConflict" in command) {
          const seq = command.resolveConflict.seq;
          return [
            encodeAppFrame({ MergeReport: { in_reply_to: seq, report: Array.from(encodePackValue({ policy: "Normal", accepted: true, insertionIndex: 0, replayed: [], worst: null, conflict: null })) } }),
            encodeAppFrame({ Conflicts: { in_reply_to: seq, conflicts: Array.from(encodePackValue([])) } }),
          ];
        }
        if (command && typeof command === "object" && "readConflicts" in command) {
          return [encodeAppFrame({ Conflicts: { in_reply_to: command.readConflicts.seq, conflicts: Array.from(encodePackValue([])) } })];
        }
        throw new Error(`unexpected command ${JSON.stringify(command)}`);
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    expect(typeof handle.setMergePolicy).toBe("function");
    expect(typeof handle.resolveConflict).toBe("function");
    expect(typeof handle.readConflicts).toBe("function");
    const instanceId = await handle.createApp("main");

    // ⚖️ Same call the Settings merge-policy `Select`'s `dispatchSetMergePolicy` makes (`ShellHost/🟦️component.tsx`).
    await handle.setMergePolicy(instanceId, "Vigilant");
    expect(sentCommands[0]).toEqual({ setMergePolicy: { seq: 1, policy: 2 } });

    // ⚔️ Same call the Conflicts panel's Accept button makes (`ChromePanels`'s `onResolve` → `dispatchResolveConflict`).
    const resolved = await handle.resolveConflict(instanceId, "conflict-abc", "accept");
    expect(sentCommands[1]).toEqual({ resolveConflict: { seq: 2, conflict_id: "conflict-abc", resolution: 0 } });
    expect(resolved.mergeReport?.accepted).toBe(true);
    expect(resolved.conflicts).toEqual([]);

    await handle.readConflicts(instanceId);
    expect(sentCommands[2]).toEqual({ readConflicts: { seq: 3 } });
  });

  it("adaptPluginHandle.applyMutations decodes an unsolicited MergeReport/Conflicts reply and it reaches ShellState — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane L1 gap 1: a peer's ApplyEnvelopes ingest batches MergeReport/Conflicts frames alongside it (contract freeze §C6/§C9 'pushed unsolicited after every ingest'), but `applyMutations` used to only look for an Error frame and silently drop everything else — this asserts the guest's real roster survives the decode AND that dispatching it through `shellReducer`'s SET_CONFLICTS (what ShellHost's `applyRemoteMerge` does) lands a remote-origin quarantined conflict in both `selectOpenConflicts` and `selectQuarantinedConflicts`, exactly the panel/badge lane K2 wired", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, encodeMutationEnvelopesPack } = await import("@semio-tech/framework-os");
    const remoteConflict = {
      id: "conflict-remote-1",
      kind: { kind: "quarantined", envelopes: [] },
      status: "open",
      messages: [{ level: "error", code: "mutation.targetMissing", message: "peer deleted the renamed node" }],
      actors: ["peer-actor"],
      timestamp: { actor: 7, physical_ms: 1000, logical: 1 },
    };
    const fakeHandle = {
      manifest: async () => encodePackValue({ pluginId: "mock-remote-merge", label: "Mock Remote Merge", version: "0", apps: [], programs: [], examples: [] }),
      createApp: async () => 11,
      destroyApp: async () => {},
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("ApplyEnvelopes" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
        const seq = command.ApplyEnvelopes.seq;
        return [
          // ⚖️ Unsolicited: the command was ApplyEnvelopes, not ReadConflicts/ResolveConflict — this
          // is exactly the "pushed unsolicited after every ingest" reply shape contract freeze §C8
          // describes, alongside whatever `DocumentChanged`/effect frames a real ingest would also carry.
          encodeAppFrame({ MergeReport: { in_reply_to: seq, report: Array.from(encodePackValue({ policy: "Normal", accepted: false, insertionIndex: 0, replayed: [], worst: "error", conflict: remoteConflict.id })) } }),
          encodeAppFrame({ Conflicts: { in_reply_to: seq, conflicts: Array.from(encodePackValue([remoteConflict])) } }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-remote-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");

    // 🐛 Pre-fix, `applyMutations` returned `Promise<void>` and this whole roster was thrown away.
    const result = await handle.applyMutations(instanceId, encodeMutationEnvelopesPack([]));
    expect(result.mergeReport?.accepted).toBe(false);
    expect(result.mergeReport?.worst).toBe("error");
    expect(result.conflicts).toEqual([remoteConflict]);

    // ⚖️ The other half of the gap: ShellHost's `applyRemoteMerge` (fed by `applyRemoteMergeRef` from
    // the `remoteMutations` worker-event branch) just dispatches `SET_CONFLICTS` with this roster —
    // reproduced here at the reducer level so the assertion doesn't need a mounted `ShellHost`.
    const state = shellReducer(initialShellState({ plugins: [], storage: createMemoryStoragePort() }), { type: "SET_CONFLICTS", value: result.conflicts ?? [] });
    expect(selectOpenConflicts(state)).toEqual([remoteConflict]);
    expect(selectQuarantinedConflicts(state)).toEqual([remoteConflict]);
  });

  // 🪦️ H1-react — `isPluginInstanceBusyError`/`pluginErrorText` (detecting a jco "plugin instance
  // busy" error after a concurrent call raced past `withSerializedPluginWasmHandle`) are deleted
  // alongside it and `INSTANCE_GUARD`/`clear-instance-guard` (packet H2's/`📌️important.md`'s "must
  // not exist" list). Not a dropped-coverage gap: `serializePerActor` (tested above) makes a "busy"
  // race structurally impossible at this layer — every turn for one actor is queued, never
  // concurrent — so there is no busy-error shape left to detect. Prevention replaced detection.
});

describe("framework renderer types", () => {
  it("keeps window tabs concise while retaining the app fallback", () => {
    const app = {
      id: "puzzle3d-play",
      label: "Puzzle 3D",
      breadcrumb: ["semio", "puzzle", "3d"],
      terminologyBreadcrumbs: { reuse: ["Entwerfen mit Bestand", "Aggregator"] },
      controllerId: "puzzle3d-play",
      modes: [],
      windowKinds: [],
      panelTabs: [],
      keybindings: [],
    };
    expect(appBreadcrumb(app.breadcrumb)).toBe("semio · puzzle · 3d");
    expect(appWindowLabel(app, "native", "Flow")).toBe("Flow");
    expect(appWindowLabel(app, "native", "Preview")).toBe("Preview");
    expect(appWindowLabel(app, "native", "")).toBe("Puzzle 3D");
    expect(appWindowLabel(app, "reuse", "")).toBe("Aggregator");
    expect(resolveAppBreadcrumb(app, "native")).toEqual(["semio", "puzzle", "3d"]);
    expect(resolveAppBreadcrumb(app, "reuse")).toEqual(["Entwerfen mit Bestand", "Aggregator"]);
    expect(appBreadcrumb(resolveAppBreadcrumb(app, "reuse"))).toBe("Entwerfen mit Bestand · Aggregator");
  });

  it("survives an app whose manifest declares no breadcrumb at all", () => {
    // 🛡️ `AppDefinition.breadcrumb` is OPTIONAL, so an app may legitimately ship without one — and
    // `appBreadcrumb` runs inside `FrameworkOsShellInner`'s RENDER. Before this guard, one such app
    // threw `Cannot read properties of undefined (reading 'join')` and took the whole shell down
    // with it; in a multi-pane host (the demonstrator) that killed all six panes at once and left
    // the page blank. A nameless title is the correct degradation, never a dead host.
    const app = { breadcrumb: undefined, terminologyBreadcrumbs: undefined } as Parameters<typeof resolveAppBreadcrumb>[0];
    expect(resolveAppBreadcrumb(app, "native")).toEqual([]);
    expect(appBreadcrumb(resolveAppBreadcrumb(app, "native"))).toBe("");
    expect(appBreadcrumb(undefined)).toBe("");
  });

  it("flattens a recursive panelTabs tree to its leaves, depth-first", () => {
    const tabs = [
      { id: "framework.panel.artifact", label: "Artifact", group: "workbench", bodyKey: "doc" },
      {
        id: "framework.panel.catalogue",
        label: "Catalogue",
        group: "workbench",
        children: [
          { id: "framework.panel.catalogue.words", label: "Words", group: "workbench", bodyKey: "words" },
          { id: "framework.panel.catalogue.headings", label: "Headings", group: "workbench", bodyKey: "headings" },
        ],
      },
    ];
    const leaves = flattenPanelTabLeaves(tabs);
    expect(leaves.map((tab) => tab.id)).toEqual(["framework.panel.artifact", "framework.panel.catalogue.words", "framework.panel.catalogue.headings"]);
    expect(leaves.every((tab) => Boolean(tab.bodyKey))).toBe(true);
  });

  it("accepts component scene nodes", () => {
    const node = {
      type: "componentScene",
      surfaceId: "draw.play.composite",
      controllerId: "draw-play",
      componentKind: "canvas-2d",
      canvas2d: {
        cameraX: 0,
        cameraY: 0,
        zoom: 1,
        layersJson: "[]",
      },
    };
    expect(node.componentKind).toBe("canvas-2d");
  });

  it("accepts graph-timeline component scene nodes", () => {
    const node = {
      type: "componentScene",
      surfaceId: "vcs.play.history",
      controllerId: "vcs-play",
      componentKind: "graph-timeline",
      graphTimeline: {
        columnsJson: "[]",
      },
    };
    expect(node.componentKind).toBe("graph-timeline");
  });
});

describe("framework external slots", () => {
  // 🚧️ HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS: `resolveExternalSlots` (framework-core)
  // no longer has a `render`/`renderWithDocument` verb to call — rendering a contributor's UI body
  // through the new binary channel (`AppChannelClient.refreshUi`) is a dedicated follow-up work
  // package this ticket flags, so a found plugin/instance still degrades to "Extension unavailable"
  // (see that function's doc comment). This test now asserts that documented Wave 1 behavior instead
  // of the old per-verb `renderWithDocument` round trip.
  it("degrades a resolvable external slot to 'unavailable' until the binary-channel render path lands", async () => {
    const { resolveExternalSlots } = await import("@semio-tech/framework");
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const handle = {
      manifest: async () => encodePackValue({ pluginId: "forms-module-procedural", label: "Module", version: "0", apps: [], programs: [], examples: [] }),
      createApp: async () => 7,
      destroyApp: async () => {},
      // 🎫️ `exchange-removal`: this handle is only ever placed in `ExternalSlotResolverContext.plugins`
      // (typed `ReadonlyMap<string, PluginWasmHandle>`, `🎠️kernel/🟦️component.ts`) — `resolveExternalSlots`
      // degrades to "unavailable" before ever touching `enqueue`/`outcomes` (see this test's own header
      // doc), so these two only need to satisfy the shape, never actually fire.
      enqueue: () => {},
      outcomes: createTurnOutcomeBroadcast<TurnOutcome>().stream,
      dispose: () => {},
    };
    const resolved = await resolveExternalSlots(
      {
        type: "externalSlot",
        pluginId: "forms-module-procedural",
        appId: "forms-module-procedural",
        bodyKey: "preview",
        paramsJson: "{}",
      },
      {
        plugins: new Map([["forms-module-procedural", handle]]),
        contributorInstances: new Map(),
        viewState: {},
      },
    );
    expect(resolved).toEqual({ type: "text", value: "Extension unavailable: forms-module-procedural" });
  });

  it("renders external slot fallback text when unresolved", () => {
    // 🧬️ MIGRATION: `Component::Extension` (the old `ExternalSlot`) collapses `pluginId`/`appId`/
    // `bodyKey` into one opaque `extension` address string — see `ExtensionProps`'s own doc.
    const markup = renderContractTree({ key: "missing-module", component: { type: "extension", extension: "missing-module", props: {} } });
    expect(markup).toContain("Extension unavailable: missing-module");
  });
});

describe("declarative forms parity", () => {
  it("renders declarative text with appearance-aware foreground", () => {
    const markup = renderContractTree({ key: "text", component: { type: "text", value: "Hello flow", emphasize: null, dataAttributes: null } });
    expect(markup).toContain("text-foreground");
    expect(markup).toContain("Hello flow");
    const emphasized = renderContractTree({ key: "text", component: { type: "text", value: "Emphasized", emphasize: true, dataAttributes: null } });
    expect(emphasized).toContain("text-foreground");
    expect(emphasized).toContain("font-semibold");
  });

  it("dag overlay label fills resolve to Canvas2D-safe hex for appearance", () => {
    const chrome = { selectedIds: new Set<string>(["sel"]), highlightedIds: new Set<string>(["hi"]) };
    expect(dagOverlayLabelFill("plain", false, null, chrome)).toBe("var(--color-muted-foreground)");
    expect(dagOverlayLabelFill("sel", false, null, chrome)).toBe("var(--color-foreground)");
    const muted = dagOverlayLabelFillHex("plain", false, null, chrome);
    const selected = dagOverlayLabelFillHex("sel", false, null, chrome);
    const highlighted = dagOverlayLabelFillHex("hi", false, null, chrome);
    const hovered = dagOverlayLabelFillHex("plain", false, "plain", chrome);
    const ghost = dagOverlayLabelFillHex("ghost", true, null, chrome);
    const dimmed = dagOverlayLabelFillHex("plain", false, null, chrome, ["plain"]);
    for (const hex of [muted, selected, highlighted, hovered, ghost, dimmed]) {
      expect(hex).toMatch(/^#[0-9a-f]{6}$/iu);
      expect(hex).not.toBe("#000000");
    }
    expect(selected).toBe(hovered);
    expect(highlighted).toBe(ghost);
  });

  it("renders field description, required marker and inline error", () => {
    // 🧬️ MIGRATION: the old `field`/`input` `UiNode` pair collapses into one `Component::Container`
    // (`role: "field"`) whose single child IS the input — `ContainerProps`'s own doc.
    const markup = renderContractTree({
      key: "forms-try.name",
      component: { type: "container", role: "field", label: "Name", description: "Your full name", required: true, error: "Name is required", defaultOpen: null, dropOverlay: null },
      children: [{ key: "forms-try.name.input", component: { type: "input", kind: "text", value: "", placeholder: null, commit: null, min: null, max: null, step: null, accept: null } }],
    });
    expect(markup).toContain("Your full name");
    expect(markup).toContain("Name is required");
    expect(markup).toContain("*");
    expect(markup).toContain('data-slot="field-error"');
  });

  it("renders slider unit readout", () => {
    const markup = renderContractTree({ key: "forms-try.volume.slider", component: { type: "slider", value: 60, min: 0, max: 100, step: 5, unit: "%" } });
    expect(markup).toContain("60 %");
  });

  it("renders numberStepper as a single-border Stepper control, not hand-rolled double-bordered buttons", () => {
    const markup = renderContractTree({ key: "forms-try.height.stepper", component: { type: "numberStepper", value: 3, step: 1, uniform: true } });
    expect(markup).toContain('data-slot="stepper-group"');
    expect(markup).toContain('data-slot="stepper-minus"');
    expect(markup).toContain('data-slot="stepper-plus"');
    expect(markup).not.toContain("border-border");
  });

  it("shows the mixed-values placeholder on a non-uniform numberStepper", () => {
    const markup = renderContractTree({ key: "forms-try.height.stepper", component: { type: "numberStepper", value: 0, step: 1, uniform: false } });
    expect(markup).toContain('data-mixed="true"');
  });

  it("renders a group node as a labeled section nesting its child controls (Origin > X/Y/Z steppers)", () => {
    // 🧬️ MIGRATION: `group`/`field` both collapse into `Component::Container` (`role: "group"` /
    // `role: "field"`) — the old `child: Box<UiNode>` singular is simply `children[0]` on the record.
    const markup = renderContractTree({
      key: "puzzle3d-play-inspector.object.origin",
      component: { type: "container", role: "group", label: "Origin", description: null, required: null, error: null, defaultOpen: true, dropOverlay: null },
      children: [
        {
          key: "puzzle3d-play-inspector.object.origin.x",
          component: { type: "container", role: "field", label: "X", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
          children: [{ key: "puzzle3d-play-inspector.object.origin.x.stepper", component: { type: "numberStepper", value: 1, step: 0.1, uniform: true } }],
        },
        {
          key: "puzzle3d-play-inspector.object.origin.y",
          component: { type: "container", role: "field", label: "Y", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
          children: [{ key: "puzzle3d-play-inspector.object.origin.y.stepper", component: { type: "numberStepper", value: 2, step: 0.1, uniform: true } }],
        },
      ],
    });
    expect(markup).toContain(">Origin</h2>");
    expect(markup).toContain(">X</label>");
    expect(markup).toContain(">Y</label>");
    expect(markup).toContain('data-slot="stepper-group"');
  });

  // 🧬️ MIGRATION: `LayoutSpec`'s stack `gap`/`padding` are now a closed `SpaceToken` enum resolved to
  // inline CSS via `spaceTokenRem` (Interpreter's own `layoutSpecStyle`/`LayoutAndStyle` region) —
  // the new architecture's real replacement for "never a hardcoded raw rem" is a renderer-neutral
  // token resolved to CSS at read time, not the pre-migration Tailwind-gap-class scheme this test
  // used to assert (`not.toContain("style=")` is no longer true BY DESIGN, not a regression — see
  // `react-renderer` packet's own decisions doc). Rewritten to assert the token resolves through that
  // closed scale (never an arbitrary raw number), while separators still avoid the raw
  // `border-border` utility class.
  it("resolves stack gap/padding through the closed SpaceToken scale as inline CSS, and keeps separators off raw border-border", () => {
    const markup = renderContractTree({
      key: "forms-blueprint.section.q1",
      component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout: { kind: "stack", axis: "vertical", gap: "xs", padding: { all: "none" }, align: "start", justify: "start", grow: false, wrap: false },
      children: [
        { key: "text", component: { type: "text", value: "text · q1", emphasize: null, dataAttributes: null } },
        { key: "sep", component: { type: "separator" } },
      ],
    });
    expect(markup).toMatch(/gap:\s*0\.2rem/);
    expect(markup).not.toContain("border-border");
  });

  it("passes number bounds and file accept to inputs", () => {
    const numberMarkup = renderContractTree({ key: "forms-try.age.input", component: { type: "input", kind: "number", value: "28", placeholder: null, commit: null, min: 13, max: 120, step: 1, accept: null } });
    expect(numberMarkup).toContain('min="13"');
    expect(numberMarkup).toContain('max="120"');
    const fileMarkup = renderContractTree({ key: "forms-try.resume.input", component: { type: "input", kind: "file", value: "", placeholder: null, commit: null, min: null, max: null, step: null, accept: ".pdf,.doc" } });
    expect(fileMarkup).toContain('accept=".pdf,.doc"');
  });

  it("disables gated wizard buttons", () => {
    // 🧬️ MIGRATION: `disabled` moved off the component (`ButtonProps` no longer carries it) onto the
    // record itself (`record.disabled` — `ButtonView`'s own `disabled={record.disabled}`); `action`
    // moved to the record's `bindings`, keyed by `Trigger::Activate`.
    const markup = renderContractTree({
      key: "forms-try.next",
      component: { type: "button", icon: "chevron-right", label: "Next" },
      disabled: true,
      bindings: [{ trigger: "activate", action: { scope: "forms-play", name: "nextStep", version: 1 }, args: null, capability: null }],
    });
    expect(markup).toContain("disabled");
  });

  it("renders selectable builder cards with selection ring", () => {
    // 🧬️ MIGRATION: `selected` is no longer a document field — presence (hover/selection) is a
    // separate `UiPresenceOverlayContext` channel keyed by `UiNodeRecord.key`, fed from
    // `PresenceUpdate` wire messages, never part of the retained document (`PresenceOverlay`
    // region's own doc: "presence changes at input frequency and must not touch a document
    // revision"). `data-ui-path` (a tree-position string) is gone too — the record's own stable
    // `data-ui-node-id` is the only per-node DOM handle now.
    const markup = renderContractTree(
      {
        key: "forms-blueprint.card.q1",
        component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
        bindings: [{ trigger: "activate", action: { scope: "forms-play", name: "setSelection", version: 1 }, args: null, capability: null }],
        children: [{ key: "text", component: { type: "text", value: "text · q1", emphasize: null, dataAttributes: null } }],
      },
      { "forms-blueprint.card.q1": { selected: true } },
    );
    expect(markup).toContain('data-ui-node-id="0"');
    expect(markup).toContain('role="button"');
    expect(markup).toContain("ring-primary");
  });

  it("renders image nodes from url sources", () => {
    const markup = renderContractTree({ key: "forms-try.avatar.image", component: { type: "image", src: "https://example.com/avatar.png", alt: "Avatar" } });
    expect(markup).toContain('src="https://example.com/avatar.png"');
    expect(markup).toContain('alt="Avatar"');
  });

  // 🪦️ MIGRATION, deleted (not rewritten): `declarativeTreeDragController` — a standalone pure
  // function taking a whole tree `UiNode` + a dispatch callback and returning a
  // `TreeDragAndDropController` — was deliberately not ported forward (react-renderer packet's own
  // decisions doc; the barrel's in-file migration comment says so too). Drag/drop for a `tree`
  // component is now wired INSIDE `Interpreter`'s own `TreeView` (built from the record's own `drop`
  // `ActionBinding`, dispatched through `dispatchTrigger`/`emitIntent`), not a separately-importable
  // factory this file's OWNS can call in isolation — there is no equivalent unit boundary left.
  // See this packet's report for a production-bug flag this deletion surfaced: `TreeView`'s current
  // `handleDrop` (`Interpreter/🟦️component.tsx`) calls `dispatchTrigger(context, record, "drop")`
  // with NO input payload at all, discarding the drop event's target/payload/position entirely —
  // this test's old assertion (`args: { kind, targetId, dropPosition }`) has no successor to assert
  // against today.
});

describe("framework renderer hosts", () => {
  it("renders node graph host from workflow scene json", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodesJson: JSON.stringify([
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ]),
            edgesJson: "[]",
            viewportJson: '{"x":0,"y":0,"zoom":1}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("renders editable node graph host with find items", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodesJson: JSON.stringify([
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ]),
            edgesJson: "[]",
            viewportJson: '{"x":0,"y":0,"zoom":1}',
            editable: true,
            findItemsJson: JSON.stringify([{ id: "app-a", label: "Draw", category: "Workflow" }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("uses the live session camera for node graph wheel viewport actions", () => {
    expect(nodeGraphViewportActionArgs('{"x":12,"y":24,"zoom":1.75}')).toEqual({
      viewportJson: '{"x":12,"y":24,"zoom":1.75}',
    });
  });

  it("encodes node graph scenes as pack bytes for wasm sync", async () => {
    const { packValueToBase64 } = await import("@semio-tech/framework-os");
    const scene = {
      nodesJson: packValueToBase64([]),
      edgesJson: packValueToBase64([]),
      viewportJson: packValueToBase64({ x: 0, y: 0, zoom: 1 }),
    };
    const bytes = sceneToSyncPack(scene);
    expect(bytes.length).toBeGreaterThan(8);
  });

  it("parses slider overlay state json for flow graph hosts", () => {
    const sliders = parseDagSliderOverlays(
      JSON.stringify({
        camera: { x: 0, y: 0, zoom: 1 },
        sliders: [
          {
            widgetId: "slider_2",
            value: 2.2,
            min: 0,
            max: 10,
            step: 0.1,
            x: 100,
            y: 50,
            w: 120,
            h: 8,
          },
        ],
      }),
    );
    expect(sliders).toHaveLength(1);
    expect(sliders[0]?.widgetId).toBe("slider_2");
    expect(sliders[0]?.value).toBe(2.2);
  });

  it("renders graph slider overlays as track-only controls without a nested value readout", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 1 },
          sliders: [
            {
              widgetId: "slider_2",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-slot="slider"');
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).not.toContain('data-slot="slider-value"');
    expect(markup).not.toContain('data-slot="slider-row"');
  });

  it("scales graph slider overlay chrome with canvas zoom so the knob matches other elements", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 2 },
          sliders: [
            {
              widgetId: "slider_2",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-graph-slider-zoom="2"');
    expect(markup).toContain("translate(-50%, -50%) scale(2)");
    expect(markup).toContain("width:120px");
    expect(markup).toContain('data-slot="slider-thumb"');
  });

  it("renders canvas 2d host with infinite canvas session", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([{ id: "layer-1", name: "Layer 1", x: 0, y: 0, width: 120, height: 80 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });

  it("renders canvas 2d host with draw gradient/blend/overlay/meta scene records", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([
              { id: "meta:utility", role: "meta", utility: "selectDirect" },
              {
                id: "shape-1",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "line", to: [10, 10] }, { kind: "close" }],
                fill: {
                  kind: "linearGradient",
                  x1: 0,
                  y1: 0,
                  x2: 10,
                  y2: 10,
                  stops: [
                    { offset: 0, color: [1, 0, 0, 1] },
                    { offset: 1, color: [0, 0, 1, 1] },
                  ],
                },
                stroke: { color: [0, 0, 0, 1], width: 1, cap: "round", join: "round" },
                opacity: 1,
                blendMode: "multiply",
                visible: true,
                fillRule: "evenodd",
              },
              {
                id: "overlay:sel:shape-1",
                role: "overlay",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "close" }],
                fill: { kind: "solid", color: [0.98, 0.75, 0.14, 0.16] },
                stroke: { color: [0.98, 0.75, 0.14, 0.95], width: 2 },
                opacity: 1,
                blendMode: "normal",
                visible: true,
                fillRule: "evenodd",
              },
            ]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });

  it("renders puzzle 2d board host shell", () => {
    const markup = renderToStaticMarkup(
      createElement(Board2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle2d.play.composite.2d-overview",
          controllerId: "puzzle2d-play",
          componentKind: "board-2d",
          board2d: {
            fixtureJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            glyphCatalogsJson: "{}",
            selectionJson: "[]",
            interactive: true,
            selectionMethod: "rectangle",
            gridSnapEnabled: false,
            gridFactor: 1,
            suggestionOffset: 0,
            brushWeightsJson: "{}",
            placementCompatibilityJson: "[]",
            lodMode: "automatic",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-board-2d-host");
  });

  it("uses the live puzzle 2d board camera for wheel persistence actions", () => {
    expect(board2dCameraActionArgs('{"x":345,"y":-123,"zoom":4.25}')).toEqual({
      camera: { x: 345, y: -123, zoom: 4.25 },
    });
  });

  it("coalesces puzzle 2d board events: drops transients, keeps the latest camera, coalesces nodeMove per id", () => {
    const rows = [
      { name: "preselect", payload: { ids: ["a"] } },
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } },
      { name: "nodeMove", payload: { id: "alpha", x: 20, y: 20 } },
      { name: "nodeMove", payload: { id: "beta", x: 5, y: 5 } },
    ];
    const { flushNow, eventsJson } = coalesceBoard2dEvents(rows);
    const events = JSON.parse(eventsJson) as { name: string; payload: Record<string, unknown> }[];
    expect(flushNow).toBe(false);
    expect(events.find((event) => event.name === "preselect")).toBeUndefined();
    const cameraEvents = events.filter((event) => event.name === "camera");
    expect(cameraEvents).toHaveLength(1);
    expect(cameraEvents[0]?.payload).toEqual({ x: 2, y: 2, zoom: 1.5 });
    const alphaMoves = events.filter((event) => event.name === "nodeMove" && event.payload.id === "alpha");
    expect(alphaMoves).toHaveLength(1);
    expect(alphaMoves[0]?.payload).toEqual({ id: "alpha", x: 20, y: 20 });
  });

  it("coalesces puzzle 2d board events: drops nodeMove rows once a nodeDragEnd follows", () => {
    const rows = [
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "nodeDragEnd", payload: { moves: [{ id: "alpha", x: 20, y: 20 }] } },
    ];
    const { eventsJson } = coalesceBoard2dEvents(rows);
    const events = JSON.parse(eventsJson) as { name: string }[];
    expect(events.some((event) => event.name === "nodeMove")).toBe(false);
    expect(events.some((event) => event.name === "nodeDragEnd")).toBe(true);
  });

  it("flushes puzzle 2d board events immediately for select/brushPlace/edge/delete rows, not for camera/nodeMove alone", () => {
    expect(coalesceBoard2dEvents([{ name: "camera", payload: { x: 0, y: 0, zoom: 1 } }]).flushNow).toBe(false);
    expect(coalesceBoard2dEvents([{ name: "nodeMove", payload: { id: "alpha", x: 0, y: 0 } }]).flushNow).toBe(false);
    for (const name of ["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"]) {
      expect(coalesceBoard2dEvents([{ name, payload: {} }]).flushNow).toBe(true);
    }
  });

  it("collects live mirror mutations: coalesces nodeMove to the latest per id, ignores unrelated rows", () => {
    const mutations = collectPuzzle2dLiveMirrorMutations([
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 1, y: 1 } },
      { name: "brushPreview", payload: {} },
      { name: "nodeMove", payload: { id: "alpha", x: 9, y: 9 } },
      { name: "nodeMove", payload: { id: "beta", x: 2, y: 2 } },
    ]);
    expect(mutations.positions).toEqual([
      { id: "alpha", x: 9, y: 9 },
      { id: "beta", x: 2, y: 2 },
    ]);
    expect(mutations.selectionIds).toBeNull();
    expect(mutations.preselect).toBeNull();
    expect(mutations.clearPreselect).toBe(false);
  });

  it("collects live mirror mutations: nodeDragEnd.moves produce final positions", () => {
    const mutations = collectPuzzle2dLiveMirrorMutations([
      { name: "nodeMove", payload: { id: "alpha", x: 1, y: 1 } },
      {
        name: "nodeDragEnd",
        payload: {
          moves: [
            { id: "alpha", x: 20, y: 20 },
            { id: "beta", x: 5, y: 5 },
          ],
        },
      },
    ]);
    expect(mutations.positions).toEqual([
      { id: "alpha", x: 20, y: 20 },
      { id: "beta", x: 5, y: 5 },
    ]);
  });

  it("collects live mirror mutations: preselect sets the live highlight, select/preselectCancel commit selection and clear it", () => {
    expect(collectPuzzle2dLiveMirrorMutations([{ name: "preselect", payload: { ids: ["a", "b"], removedIds: ["c"] } }])).toMatchObject({
      preselect: { ids: ["a", "b"], removedIds: ["c"] },
      clearPreselect: false,
      selectionIds: null,
    });
    expect(collectPuzzle2dLiveMirrorMutations([{ name: "select", payload: { ids: ["a"] } }])).toMatchObject({
      selectionIds: ["a"],
      preselect: null,
      clearPreselect: true,
    });
    expect(collectPuzzle2dLiveMirrorMutations([{ name: "preselectCancel", payload: { ids: ["a", "b"] } }])).toMatchObject({
      selectionIds: ["a", "b"],
      preselect: null,
      clearPreselect: true,
    });
  });

  it("peer registry: registers/unregisters, excludes own surfaceId and other controllerIds", () => {
    const peerA = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerB = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerOther = { session: {} as never, onPeerGestureEnded: () => {} };
    registerBoard2dPeer("puzzle2d-play", "pane.a", peerA);
    registerBoard2dPeer("puzzle2d-play", "pane.b", peerB);
    registerBoard2dPeer("other-controller", "pane.a", peerOther);

    expect(board2dPeers("puzzle2d-play", "pane.a")).toEqual([peerB]);
    expect(board2dPeers("puzzle2d-play", "pane.b")).toEqual([peerA]);
    expect(board2dPeers("other-controller", "pane.z")).toEqual([peerOther]);

    unregisterBoard2dPeer("puzzle2d-play", "pane.b");
    expect(board2dPeers("puzzle2d-play", "pane.a")).toEqual([]);

    unregisterBoard2dPeer("puzzle2d-play", "pane.a");
    unregisterBoard2dPeer("other-controller", "pane.a");
  });

  it("peer gesture ownership: begin/end tracks the owning surfaceId; a pane never defers against its own gesture", () => {
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.a")).toBe(false);
    beginPuzzle2dPeerGesture("puzzle2d-play", "pane.a");
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.a")).toBe(false);
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.b")).toBe(true);
    endPuzzle2dPeerGesture("puzzle2d-play", "pane.a");
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.b")).toBe(false);
  });

  it("pushes live mirror mutations into peer sessions, skipping the source pane", () => {
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setNodePositionsJson: (json: string) => calls.push({ pane, method: "setNodePositionsJson", arg: json }),
        setSelectionIdsJsonSilent: (json: string) => calls.push({ pane, method: "setSelectionIdsJsonSilent", arg: json }),
        setPreselectStateJsonSilent: (json: string) => calls.push({ pane, method: "setPreselectStateJsonSilent", arg: json }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer("mirror-test", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer("mirror-test", "pane.sibling", makePeer("pane.sibling"));

    pushPuzzle2dLiveMirrorMutations("mirror-test", "pane.source", {
      positions: [{ id: "alpha", x: 1, y: 2 }],
      selectionIds: ["alpha"],
      preselect: null,
      clearPreselect: false,
    });

    expect(calls).toEqual([
      { pane: "pane.sibling", method: "setNodePositionsJson", arg: JSON.stringify([{ id: "alpha", x: 1, y: 2 }]) },
      { pane: "pane.sibling", method: "setSelectionIdsJsonSilent", arg: JSON.stringify(["alpha"]) },
    ]);

    unregisterBoard2dPeer("mirror-test", "pane.source");
    unregisterBoard2dPeer("mirror-test", "pane.sibling");
  });

  it("notifies peers when a gesture ends, skipping the source pane, passing whether it flushed", () => {
    const ended: { pane: string; flushed: boolean }[] = [];
    registerBoard2dPeer("notify-test", "pane.source", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.source", flushed }) });
    registerBoard2dPeer("notify-test", "pane.sibling", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.sibling", flushed }) });

    notifyPuzzle2dPeersGestureEnded("notify-test", "pane.source", true);
    expect(ended).toEqual([{ pane: "pane.sibling", flushed: true }]);

    unregisterBoard2dPeer("notify-test", "pane.source");
    unregisterBoard2dPeer("notify-test", "pane.sibling");
  });


  it("maps context menu specs onto UI items with icons, colors, hover, and select handlers", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        { id: "hide", label: "Hide", icon: "eye-off", color: "#ff0000", action: "setFlag", args: { flag: "hidden" }, hoverAction: "hoverFlag", hoverArgs: { index: 1 } },
        { id: "sep", separator: true },
        { id: "delete", label: "Delete", icon: "trash", destructive: true, action: "deleteSelection" },
      ],
      dispatch,
    );
    expect(items[0]).toMatchObject({ id: "hide", label: "Hide", icon: "eye-off", color: "#ff0000" });
    items[0]?.onSelect?.(new Event("select"));
    items[0]?.onHover?.();
    expect(dispatch).toHaveBeenCalledWith("setFlag", { flag: "hidden" });
    expect(dispatch).toHaveBeenCalledWith("hoverFlag", { index: 1 });
    expect(items[1]).toMatchObject({ id: "sep", separator: true });
    expect(items[2]).toMatchObject({ id: "delete", destructive: true, icon: "trash" });
  });

  it("maps suggestion-style specs without a color swatch field", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        {
          id: "suggestion-0",
          label: "Capsule · vortex 0",
          icon: "box",
          checked: true,
          action: "acceptSuggestion",
          args: { index: 0, fullId: "obj:v0" },
          hoverAction: "hoverSuggestion",
          hoverArgs: { index: 0 },
        },
      ],
      dispatch,
    );
    expect(items[0]).toMatchObject({ id: "suggestion-0", icon: "box", checked: true });
    expect(items[0]).not.toHaveProperty("color", expect.anything());
    expect(items[0]?.color).toBeUndefined();
  });

  it("enriches context menu shortcuts from app keybindings", () => {
    const keysByActionId = buildKeysByActionId([
      { action: { action: "deleteSelection" }, keys: "delete,backspace" },
      { action: { action: "duplicateSelection" }, keys: "mod+d" },
    ]);
    const items = mapContextMenuSpecs(
      [
        { id: "duplicate", label: "Duplicate", action: "duplicateSelection" },
        { id: "delete", label: "Delete", action: "deleteSelection" },
        { id: "custom", label: "Custom", action: "customAction", shortcut: "F2" },
      ],
      vi.fn(),
      keysByActionId,
    );
    expect(items[0]?.shortcut).toMatch(/D$/);
    expect(items[1]?.shortcut).toBe("⌦️");
    expect(items[2]?.shortcut).toBe("F2");
  });

  it("formats keybinding chords for menu shortcut labels", () => {
    expect(formatKeybindingShortcut("backspace")).toBe("⌫️");
    expect(formatKeybindingShortcut("delete,backspace")).toBe("⌦️");
    expect(formatKeybindingShortcut("mod+d")).toMatch(/D$/);
  });

  it("numbers suggestion menu rows with digit shortcuts for the first nine candidates", () => {
    const items = suggestionMenuItems(
      {
        open: true,
        pending: false,
        x: 0,
        y: 0,
        candidates: [
          { index: 2, objectLabel: "Capsule", vortexLabel: "port-a", icon: "box" },
          { index: 5, objectLabel: "Box", vortexLabel: "port-b", icon: "box" },
        ],
        vortexFullId: "obj:v0",
      },
      2,
      { checkingPlacement: uiDataLabel("Checking placement…"), noPlacement: uiDataLabel("No placement") },
    );
    expect(items[0]).toMatchObject({ checked: true });
    expect(items[1]).toMatchObject({ checked: false });
    expect(items[0]?.shortcut).toBeUndefined();
  });

  it("enriches context menu shortcuts from app keybindings via mapContextMenuSpecs", () => {
    const keys = new Map([["deleteSelection", "delete,backspace"]]);
    const items = mapContextMenuSpecs(
      [{ id: "delete-selection", label: "Delete Selection (8 nodes and 13 edges)", action: "deleteSelection", destructive: true }],
      () => {},
      keys,
    );
    expect(items[0]?.shortcut).toBeTruthy();
    expect(items[0]?.label).toContain("8 nodes and 13 edges");
  });




  it("parses a catalogue drag payload and builds a drop-preview JSON", () => {
    const encoded = JSON.stringify({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 });
    const payload = parsePuzzle2dCatalogueDragPayload(encoded);
    expect(payload).toEqual({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24, width: undefined, height: undefined, iconKind: undefined });
    expect(payload).not.toBeNull();
    expect(JSON.parse(puzzle2dFixtureDropPreviewJson(payload!, 100, 200))).toMatchObject({ nodeKind: "seed", x: 100, y: 200, shape: "circle", radius: 24 });
  });

  it("rejects a catalogue drag payload without a kindId", () => {
    expect(parsePuzzle2dCatalogueDragPayload(JSON.stringify({ catalogSlice: "nodes" }))).toBeNull();
    expect(parsePuzzle2dCatalogueDragPayload(null)).toBeNull();
  });

  it("parses a puzzle 3d catalogue drag payload and snaps drop origins to the grid", () => {
    const encoded = JSON.stringify({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(parsePuzzle3dCatalogueDragPayload(encoded)).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], true, 1)).toEqual([1, 3, 0]);
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], false, 1)).toEqual([1.2, 2.7, 0]);
    expect(parsePuzzle3dCatalogueDragPayload(JSON.stringify({ meshUrl: "puzzle3d://capsule" }))).toBeNull();
  });

  it("raycasts the Z=0 ground under orthographic top and perspective cameras", async () => {
    const { OrthographicCamera, PerspectiveCamera } = await import("three");
    const rect = { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 } as DOMRect;

    const ortho = new OrthographicCamera(-100, 100, 50, -50, 0.1, 1000);
    ortho.position.set(0, 0, 100);
    ortho.up.set(0, 1, 0);
    ortho.lookAt(0, 0, 0);
    ortho.updateMatrixWorld(true);
    ortho.updateProjectionMatrix();

    const orthoCenter = raycastGroundPoint(100, 50, rect, ortho);
    expect(orthoCenter).not.toBeNull();
    expect(orthoCenter![0]).toBeCloseTo(0, 5);
    expect(orthoCenter![1]).toBeCloseTo(0, 5);
    expect(orthoCenter![2]).toBeCloseTo(0, 5);

    const orthoRight = raycastGroundPoint(150, 50, rect, ortho);
    expect(orthoRight).not.toBeNull();
    expect(orthoRight![0]).toBeCloseTo(50, 5);
    expect(orthoRight![1]).toBeCloseTo(0, 5);
    expect(orthoRight![2]).toBeCloseTo(0, 5);

    const orthoUp = raycastGroundPoint(100, 25, rect, ortho);
    expect(orthoUp).not.toBeNull();
    expect(orthoUp![0]).toBeCloseTo(0, 5);
    expect(orthoUp![1]).toBeCloseTo(25, 5);
    expect(orthoUp![2]).toBeCloseTo(0, 5);

    const perspective = new PerspectiveCamera(50, 2, 0.1, 1000);
    perspective.position.set(0, 0, 10);
    perspective.up.set(0, 1, 0);
    perspective.lookAt(0, 0, 0);
    perspective.updateMatrixWorld(true);
    perspective.updateProjectionMatrix();

    const perspectiveCenter = raycastGroundPoint(100, 50, rect, perspective);
    expect(perspectiveCenter).not.toBeNull();
    expect(perspectiveCenter![0]).toBeCloseTo(0, 4);
    expect(perspectiveCenter![1]).toBeCloseTo(0, 4);
    expect(perspectiveCenter![2]).toBeCloseTo(0, 4);
  });

  it("shares the world catalogue drop preview across all registered hosts", () => {
    clearWorldCatalogueDropPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldCatalogueDropPreview>> = [];
    const unsub = subscribeWorldCatalogueDropPreview(() => {
      notifications.push(getWorldCatalogueDropPreview("puzzle3d-play"));
    });
    const unregisterA = registerWorldCatalogueDropHost("puzzle3d-play", "pane.a", (x, y) => x >= 0 && x < 100 && y >= 0 && y < 100);
    const unregisterB = registerWorldCatalogueDropHost("puzzle3d-play", "pane.b", (x, y) => x >= 100 && x < 200 && y >= 0 && y < 100);

    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 50, 50)).toBe(true);
    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 150, 50)).toBe(true);
    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 250, 50)).toBe(false);
    expect(worldCatalogueDropHostContainsPoint("other-controller", 50, 50)).toBe(false);

    setWorldCatalogueDropPreview("puzzle3d-play", { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] });
    expect(getWorldCatalogueDropPreview("puzzle3d-play")).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] });
    expect(getWorldCatalogueDropPreview("other-controller")).toBeNull();
    setWorldCatalogueDropPreview("puzzle3d-play", { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [3, 4, 0] });
    expect(getWorldCatalogueDropPreview("puzzle3d-play")?.origin).toEqual([3, 4, 0]);
    clearWorldCatalogueDropPreview("puzzle3d-play");
    expect(getWorldCatalogueDropPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([
      { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] },
      { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [3, 4, 0] },
      null,
    ]);

    unsub();
    unregisterA();
    unregisterB();
  });

  it("shares live world selection previews across sibling panes without allowing an idle pane to clear the active gesture", () => {
    clearWorldSelectionPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldSelectionPreview>> = [];
    const unsubscribe = subscribeWorldSelectionPreview(() => notifications.push(getWorldSelectionPreview("puzzle3d-play")));
    const preview = { sourceId: "pane.top", mergedComponentIds: null, mergedInstanceIds: ["object-a", "object-b"] } as const;

    setWorldSelectionPreview("puzzle3d-play", preview);
    clearWorldSelectionPreview("puzzle3d-play", "pane.perspective");
    expect(getWorldSelectionPreview("puzzle3d-play")).toEqual(preview);
    expect(getWorldSelectionPreview("other-controller")).toBeNull();

    setWorldSelectionPreview("puzzle3d-play", { ...preview, mergedInstanceIds: ["object-a", "object-b"] });
    expect(notifications).toEqual([preview]);

    clearWorldSelectionPreview("puzzle3d-play", "pane.top");
    expect(getWorldSelectionPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([preview, null]);
    unsubscribe();
  });

  it("pushes fixture-drop previews to every board2d peer on the same controller", () => {
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setFixtureDropPreviewJson: (json: string) => calls.push({ pane, method: "setFixtureDropPreviewJson", arg: json }),
        clearFixtureDropPreview: () => calls.push({ pane, method: "clearFixtureDropPreview", arg: "" }),
        renderFrame: () => calls.push({ pane, method: "renderFrame", arg: "" }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer("fixture-preview", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer("fixture-preview", "pane.sibling", makePeer("pane.sibling"));

    const preview = puzzle2dFixtureDropPreviewJson({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 }, 10, 20);
    pushPuzzle2dFixtureDropPreview("fixture-preview", preview);
    pushPuzzle2dFixtureDropPreview("fixture-preview", null);

    expect(calls).toEqual([
      { pane: "pane.source", method: "setFixtureDropPreviewJson", arg: preview },
      { pane: "pane.source", method: "renderFrame", arg: "" },
      { pane: "pane.sibling", method: "setFixtureDropPreviewJson", arg: preview },
      { pane: "pane.sibling", method: "renderFrame", arg: "" },
      { pane: "pane.source", method: "clearFixtureDropPreview", arg: "" },
      { pane: "pane.source", method: "renderFrame", arg: "" },
      { pane: "pane.sibling", method: "clearFixtureDropPreview", arg: "" },
      { pane: "pane.sibling", method: "renderFrame", arg: "" },
    ]);

    unregisterBoard2dPeer("fixture-preview", "pane.source");
    unregisterBoard2dPeer("fixture-preview", "pane.sibling");
  });

  it("inverts the canonical screen-to-world transform for a fixture drop", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const world = puzzle2dScreenToWorld(cameraJson, { w: 800, h: 600 }, { x: 400, y: 300 });
    expect(world).toEqual({ x: 120, y: 80 });
  });

  it("puzzle2dWorldToScreen is the exact inverse of puzzle2dScreenToWorld", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const containerSize = { w: 800, h: 600 };
    // 🎯️ The camera's own world position always maps to the viewport center.
    expect(puzzle2dWorldToScreen(cameraJson, containerSize, { x: 120, y: 80 })).toEqual({ x: 400, y: 300 });
    for (const screen of [{ x: 0, y: 0 }, { x: 400, y: 300 }, { x: 733, y: 12 }]) {
      const world = puzzle2dScreenToWorld(cameraJson, containerSize, screen);
      expect(world).not.toBeNull();
      const roundTrip = puzzle2dWorldToScreen(cameraJson, containerSize, world!);
      expect(roundTrip!.x).toBeCloseTo(screen.x, 5);
      expect(roundTrip!.y).toBeCloseTo(screen.y, 5);
    }
    expect(puzzle2dWorldToScreen("not json", containerSize, { x: 0, y: 0 })).toBeNull();
  });

  it("maps a world-centered node inside the viewport with canonical camera math", () => {
    const camera = { x: 120, y: 80, zoom: 2 };
    const viewportWidth = 800;
    const viewportHeight = 600;
    const screen = worldToScreenLogical(120, 80, camera, viewportWidth, viewportHeight);
    expect(screen.x).toBeCloseTo(viewportWidth * 0.5, 5);
    expect(screen.y).toBeCloseTo(viewportHeight * 0.5, 5);
    const layersJson = JSON.stringify([
      {
        id: "node-a",
        kind: "circle",
        role: "node",
        color: "#336699",
        selected: true,
        x: 110,
        y: 70,
        width: 20,
        height: 20,
      },
    ]);
    expect(layersJson).toContain('"role":"node"');
    expect(layersJson).toContain('"selected":true');
  });

  it("canvas-2d surface colors follow the light theme canvas token instead of a hardcoded dark fill", () => {
    document.documentElement.classList.remove("dark");
    const colors = readCanvas2dSurfaceColors();
    expect(colors.clear).toBe("rgba(240, 236, 221, 1)");
    expect(colors.clear.toLowerCase()).not.toContain("17, 19, 24");
    expect(colors.grid).toMatch(/^rgba\(/);
  });

  it("renders world 3d empty state without mounting r3f canvas", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.play.world",
          controllerId: "puzzle-play",
          componentKind: "world-3d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-empty");
    expect(markup).not.toContain("data-orbit-view-gizmo");
  });

  it("marks every non-empty world-3d host with the bottom-right orbit view gizmo", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.3d.play.viewport",
          controllerId: "puzzle3d-play",
          componentKind: "world-3d",
          world3d: {
            cameraJson: '{"position":[4,4,4],"target":[0,0,0],"zoom":1}',
            meshesJson: "[]",
            instancesJson: "[]",
            selectionJson: "{}",
            vorticesJson: "[]",
            attractionsJson: "[]",
            interactionJson: '{"activeUtility":"select"}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-host");
    expect(markup).toContain('data-orbit-view-gizmo=""');
    expect(markup).toContain('data-surface-id="puzzle.3d.play.viewport"');
    expect(markup).toContain("data-world-projection-kind-switch");
  });

  it("keeps world-3d orbit camera seed local per viewport once detached", () => {
    const sceneCamera = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).toBe(sceneCamera);
    expect(world3dViewportCameraSeedKey(sceneCamera, 1)).toBe("viewport:1");
    expect(world3dViewportCameraSeedKey(sceneCamera, 2)).toBe("viewport:2");
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).not.toBe(world3dViewportCameraSeedKey(sceneCamera, 1));
    expect(shouldReattachWorldViewportCamera(sceneCamera, sceneCamera)).toBe(false);
    expect(shouldReattachWorldViewportCamera(sceneCamera, '{"position":[9,9,9],"target":[0,0,0],"zoom":1}')).toBe(true);
    const merged = mergeWorldViewportCamera(
      { position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: true, up: [0, 0, 1] },
      { position: [4, 5, 6], target: [1, 1, 1], zoom: 2, projection: "orthographic", up: [0, 1, 0] },
    );
    expect(merged.position).toEqual([4, 5, 6]);
    expect(merged.target).toEqual([1, 1, 1]);
    expect(merged.zoom).toBe(2);
    expect(merged.projection).toBe("orthographic");
    expect(merged.fov).toBe(45);
    expect(merged.explicitProjection).toBe(true);
    expect(merged.up).toEqual([0, 1, 0]);
  });

  it("buildWorldCameraDispatchArgs carries position/target/zoom/up but never a projection field", () => {
    const withUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(withUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] });
    expect(withUp).not.toHaveProperty("projection");
    expect(withUp).not.toHaveProperty("projectionSpec");

    const withoutUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective" });
    expect(withoutUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1 });
    expect(withoutUp).not.toHaveProperty("up");
    expect(withoutUp).not.toHaveProperty("projection");
  });

  it("worldCameraSetCameraDispatchArgs nests the camera pose under a `camera` key, never flat alongside windowId", () => {
    const args = worldCameraSetCameraDispatchArgs("puzzle.3d.play.viewport", { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(args).toEqual({ windowId: "puzzle.3d.play.viewport", camera: { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] } });
    expect(args).not.toHaveProperty("position");
    expect(args).not.toHaveProperty("target");
    expect(args).not.toHaveProperty("zoom");
    expect(args).not.toHaveProperty("up");
  });

  it("worldCameraPoseApproxEqual matches exact poses and float-noise, rejects a genuinely different pose", () => {
    const base = { position: [1, 2, 3] as const, target: [0, 0, 0] as const, zoom: 1 };
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [1.0000001, 2.0000001, 3], target: [0, 0, 1e-9], zoom: 1.0000001 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [9, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(false);
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 5 })).toBe(false);
  });

  it("shouldReattachWorldViewportCamera suppresses a self-echo of the last dispatched camera but not a genuinely different pose", () => {
    const previous = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    const dispatched = { position: [4, 5, 6] as const, target: [0, 0, 0] as const, zoom: 2 };
    const echoedJson = '{"position":[4.0000001,5,6],"target":[0,0,0],"zoom":2}';
    const differentJson = '{"position":[9,9,9],"target":[0,0,0],"zoom":1}';
    expect(shouldReattachWorldViewportCamera(previous, echoedJson, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, dispatched)).toBe(true);
    expect(shouldReattachWorldViewportCamera(previous, previous, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, null)).toBe(true);
  });

  it("preserves projectionSpec.view from gizmo snaps instead of clobbering to top", () => {
    const merged = mergeWorldViewportCamera(
      { position: [0, 0, 10], target: [0, 0, 0], zoom: 50, projection: "orthographic", fov: 45, explicitProjection: true, projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } },
      { position: [0, -600, 0], target: [0, 0, 0], zoom: 50, projection: "orthographic", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } },
    );
    expect(merged.projectionSpec).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } });
  });

  it("accepts extended world 3d scene fields", () => {
    const node = {
      type: "componentScene",
      surfaceId: "puzzle.3d.play.viewport",
      controllerId: "puzzle3d-play",
      componentKind: "world-3d",
      world3d: {
        cameraJson: "{}",
        meshesJson: "[]",
        instancesJson: "[]",
        selectionJson: "{}",
        vorticesJson: "[]",
        attractionsJson: "[]",
        targetVolumesJson: "[]",
        referencesJson: "[]",
        brushPreviewJson: undefined,
        interactionJson: '{"activeUtility":"select"}',
        engagementPreviewJson: '[{"kind":"point","role":"origin","position":[0,0,0]},{"kind":"box-preview","role":"preview","cornerA":[0,0,0],"cornerB":[2,2,0]}]',
        // 🖱️ Context menus are no longer pushed as scene JSON — a right-click round-trips through
        // `requestContextMenu`/`ContextMenuItemSpec` on demand instead (see `openSurfaceContextMenu`,
        // renderer `📦️index.tsx`), so `World3dScene` has no `contextMenuJson` field to cover here.
        statusJson: '{"computing":true,"label":"Evaluating"}',
        terrainJson: '{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":1.5,"colorRamp":"hypsometric","minZoom":6,"maxZoom":14}',
      },
    };
    expect(node.world3d?.meshesJson).toBe("[]");
    expect(node.world3d?.vorticesJson).toBe("[]");
    expect(node.world3d?.interactionJson).toContain("select");
    expect(node.world3d?.engagementPreviewJson).toContain("box-preview");
    expect(node.world3d?.statusJson).toContain("computing");
    expect(node.world3d?.terrainJson).toContain("hypsometric");
  });

  it("parses GIS 3D terrain style JSON, defaulting missing fields, and rejects a missing tileUrlTemplate", () => {
    expect(parseWorldTerrainStyle(undefined)).toBeNull();
    expect(parseWorldTerrainStyle("not json")).toBeNull();
    expect(parseWorldTerrainStyle('{"projectOriginLon":1}')).toBeNull();
    const style = parseWorldTerrainStyle('{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":2}');
    expect(style).toMatchObject({
      tileUrlTemplate: "/dem/{z}/{x}/{y}.png",
      projectOriginLon: 9.7382,
      projectOriginLat: 52.3759,
      exaggeration: 2,
      colorRamp: "hypsometric",
      minZoom: 6,
      maxZoom: 14,
    });
  });

  it("blocks instance picking for fill, brush, and volume brush engagements but not move", () => {
    expect(worldInstancePickBlocked("brush")).toBe(true);
    expect(worldInstancePickBlocked("fill")).toBe(true);
    expect(worldInstancePickBlocked("volumeBrush")).toBe(true);
    expect(worldInstancePickBlocked("move")).toBe(false);
    expect(worldInstancePickBlocked(undefined)).toBe(false);
  });

  it("resolves vortex pointer-down to select in brush or vertex mode and click-or-drag otherwise", () => {
    expect(resolveVortexPointerDownIntent(true)).toBe("select");
    expect(resolveVortexPointerDownIntent(false, "vertex")).toBe("select");
    expect(resolveVortexPointerDownIntent(false)).toBe("click-or-drag");
    expect(resolveVortexPointerDownIntent(false, "mesh")).toBe("click-or-drag");
  });

  it("scopes suggestion menu ownership to the opening world window so sibling panes stay interactive", () => {
    expect(worldSuggestionMenuOwnsWindow(null, "puzzle3d-main-top")).toBe(false);
    expect(worldSuggestionMenuOwnsWindow({ open: false, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(false);
    expect(worldSuggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(true);
    expect(worldSuggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-perspective")).toBe(false);
    expect(worldSuggestionMenuOwnsWindow({ open: true }, "puzzle3d-main-perspective")).toBe(true);
  });

  it("revisions vortex materials when selection or hover state changes", () => {
    expect(worldVortexMaterialRevision()).toBe("neutral");
    expect(worldVortexMaterialRevision(false, true)).toBe("hovered");
    expect(worldVortexMaterialRevision(true, true)).toBe("selected");
  });

  it("revisions world mesh materials when style kind changes so deselection clears selected paint", () => {
    expect(worldMeshMaterialRevision("selected")).toBe("selected");
    expect(worldMeshMaterialRevision("neutral")).toBe("neutral");
    expect(worldMeshMaterialRevision("hovered")).toBe("hovered");
    expect(worldMeshMaterialRevision(resolveMeshStyle({ selected: false, hovered: false }))).toBe("neutral");
  });

  it("uses the world surface selection mode instead of a stale shared invertive mode", () => {
    // 🐚️ "invertive" here plays the role the old page-global `__selectionMode` used to (a shell's
    // persistent toolbar toggle) — passed explicitly now via the `persistentMode` param instead of a
    // `globalThis` singleton, but the priority rule under test is unchanged: an explicitly *configured*
    // world-surface mode still wins over it.
    expect(resolveWorldMergeMode("replace", {}, "invertive")).toBe("replace");
    expect(resolveWorldMergeMode("replace", { shiftKey: true }, "invertive")).toBe("additive");
    expect(resolveWorldMergeMode("invertive", {}, "invertive")).toBe("invertive");
  });

  it("resolves mesh style by priority: disabled > celebrated > selected > highlighted > hovered > neutral", () => {
    expect(resolveMeshStyle({})).toBe("neutral");
    expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
    expect(resolveMeshStyle({ hovered: true, highlighted: true })).toBe("highlighted");
    expect(resolveMeshStyle({ highlighted: true, selected: true })).toBe("selected");
    expect(resolveMeshStyle({ selected: true, celebrating: true })).toBe("celebrated");
    expect(resolveMeshStyle({ celebrating: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ selected: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ disabled: true, selected: true, highlighted: true, hovered: true, celebrating: true })).toBe("disabled");
  });

  it("celebrateWorldInstances stamps ids and cancel clears them so paint prefers celebrated over selected", () => {
    const cancel = celebrateWorldInstances(["drop-1"], 60_000);
    expect(isWorldInstanceCelebrating("drop-1")).toBe(true);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("celebrated");
    cancel();
    expect(isWorldInstanceCelebrating("drop-1")).toBe(false);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("selected");
  });

  it("maps edge hover to line paint so coplanar edges stay distinct from face hover fill", () => {
    const palette = {
      neutral: { meshColor: "#111111", lineColor: "#222222", emissiveIntensity: 0, opacity: 1 },
      hovered: { meshColor: "#aaaaaa", lineColor: "#333333", emissiveIntensity: 0.08, opacity: 1 },
      selected: { meshColor: "#0000ff", lineColor: "#0000ff", emissiveIntensity: 0.35, opacity: 1 },
      highlighted: { meshColor: "#00ff00", lineColor: "#00ff00", emissiveIntensity: 0.2, opacity: 1 },
      celebrated: { meshColor: "#ff00ff", lineColor: "#ff00ff", emissiveIntensity: 0.55, opacity: 1 },
      disabled: { meshColor: "#999999", lineColor: "#888888", emissiveIntensity: 0, opacity: 0.45 },
    } as Parameters<typeof semanticColorsFromPalette>[0];
    const colors = semanticColorsFromPalette(palette);
    expect(colors.hover).toBe("#aaaaaa");
    expect(colors.edgeHover).toBe("#333333");
    expect(colors.edgeHover).not.toBe(colors.hover);
    expect(colors.select).toBe("#0000ff");
  });

  it("treats centerline meshes without shaded triangles as curve-only instances", () => {
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(true);
    expect(isCurveOnlyWorldMesh({ indices: [0, 1, 2], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(false);
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [] })).toBe(false);
  });

  it("derives marquee bounds from edge samples when positions are empty", () => {
    const corners = meshBoundsCorners({
      positions: [],
      normals: [],
      indices: [],
      edgePositions: [0, 0, 0, 2, 4, 6],
    } as Parameters<typeof meshBoundsCorners>[0]);
    expect(corners).toContainEqual([0, 0, 0]);
    expect(corners).toContainEqual([2, 4, 6]);
  });

  it("renders the new group selection as active and only objects leaving the old selection as highlighted", () => {
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, false)).toBe("highlighted");
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, false)).toBe("neutral");
    expect(resolveMeshSelectionPreviewStyle({ selected: true, disabled: true }, false)).toBe("disabled");
  });

  it("builds addBrushObject args from a parsed brush preview, or null when there is nothing to place", () => {
    expect(brushObjectPlacementArgs(null)).toBeNull();
    const args = brushObjectPlacementArgs({
      targetVortexFullId: "seed-left-001:v0",
      objectKindId: "hex-concrete",
      sourceVortexIndex: 2,
      origin: [1, 2, 3],
      orientation: [0, 0, 0, 1],
      scale: 1,
    });
    expect(args).toMatchObject({
      targetVortexFullId: "seed-left-001:v0",
      objectKindId: "hex-concrete",
      sourceVortexIndex: 2,
      origin: [1, 2, 3],
      orientation: [0, 0, 0, 1],
      scale: 1,
    });
  });

  it("defaults sourceVortexIndex to 0 when the brush preview omits it", () => {
    const args = brushObjectPlacementArgs({ targetVortexFullId: "seed-left-001:v0", objectKindId: "hex-concrete" });
    expect(args).toMatchObject({ sourceVortexIndex: 0 });
  });

  it("resolves brush/suggestion ghost mesh URLs even when the kind is not yet among scene meshes", () => {
    // 👻️ One-shot suggestion ghosts must load the preview meshUrl directly (catalogue-drop parity) —
    // requiring a scene mesh match left suggested objects invisible in 3D.
    expect(brushPreviewGhostMeshUrl({ meshUrl: "/meshes/new-kind.glb" }, [])).toBe("/meshes/new-kind.glb");
    expect(brushPreviewGhostMeshUrl({ meshUrl: "/meshes/placed.glb" }, [{ url: "/meshes/placed.glb" }])).toBe("/meshes/placed.glb");
    expect(brushPreviewGhostMeshUrl({}, [{ url: "/meshes/placed.glb" }])).toBeUndefined();
  });

  it("resolves the right-click context menu target by priority: vortex, then object, then reference", () => {
    expect(resolveWorldContextMenuTarget({ hoveredVortexFullId: "seed-left-001:v0" }, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({
      kind: "vortex",
      id: "seed-left-001:v0",
    });
    expect(resolveWorldContextMenuTarget({}, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "reference:ref-1" })).toEqual({ kind: "reference", id: "ref-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "obj-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, {})).toBeNull();
  });

  it("titles context menus from the specific hit before falling back to the surface", () => {
    const request = (domain?: string, kind = "world3d") => ({
      menu: { id: kind },
      surface: { surfaceId: "surface", kind, hits: domain ? [{ domain, id: "target" }] : [] },
    });

    expect(surfaceContextMenuTitleKey(request("vortex"))).toBe("ui.surfaceContextMenu.vortex");
    expect(surfaceContextMenuTitleKey(request("object"))).toBe("ui.surfaceContextMenu.object");
    expect(surfaceContextMenuTitleKey(request("reference"))).toBe("ui.surfaceContextMenu.reference");
    expect(surfaceContextMenuTitleKey(request())).toBe("ui.surfaceContextMenu.scene");
    expect(surfaceContextMenuTitleKey(request(undefined, "board2d"))).toBe("ui.surfaceContextMenu.board");
  });

  it("covers every target domain emitted by current surface pickers", () => {
    const domains = ["architecture", "attraction", "block", "edge", "entry", "feature", "group", "handle", "layer", "node", "object", "part", "path", "pixel", "position", "reference", "route", "row", "slider", "vortex"];
    for (const domain of domains) {
      expect(surfaceContextMenuTitleKey({ menu: { id: "surface" }, surface: { surfaceId: "surface", kind: "unknown", hits: [{ domain, id: "target" }] } })).toBe(`ui.surfaceContextMenu.${domain}`);
    }
  });

  it("renders text editor host", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "hello",
            language: "jack",
            tokensJson: JSON.stringify([{ class: "ident", start: 0, end: 5 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
    expect(markup).toContain("hello");
  });

  it("renders text editor host with hover/newline/rename scene fields", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "MATCH (a:Piece) RETURN a.name",
            language: "jack",
            hoverJson: '{"start":0,"end":5}',
            newlineGatesJson: "[30]",
            renameJson: '{"name":"a","occurrences":[{"start":7,"end":8}]}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
  });



  it("multiSpanReplace renames every occurrence and remaps spans", () => {
    const result = multiSpanReplace(
      "MATCH (a:Piece) RETURN a.name",
      [
        { start: 7, end: 8 },
        { start: 23, end: 24 },
      ],
      "piece",
    );
    expect(result.text).toBe("MATCH (piece:Piece) RETURN piece.name");
    expect(result.occurrences).toEqual([
      { start: 7, end: 12 },
      { start: 23, end: 28 },
    ]);
  });

  it("lineRangeAt finds the line containing an offset", () => {
    const text = "MATCH (a)\nWHERE a.x = 1\nRETURN a";
    const range = lineRangeAt(text, 15);
    expect(text.slice(range.start, range.end)).toBe("WHERE a.x = 1");
  });

  it("renders table host with ui-react table", () => {
    const markup = renderToStaticMarkup(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.catalogue",
          controllerId: "s-play",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
            rowsJson: JSON.stringify([{ label: "Draw" }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-table-host");
    expect(markup).toContain("Draw");
  });

  // 🆔️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F: a plugin-authored
  // row's own `id` (now reachable from `TableWindowKit::render_rows`, `🔌️plugin/🦀️component.rs`) must
  // survive all the way to the DOM as `data-row-id` (contract §C0's `"space:<id>"`/`"artifact:<id>"`
  // grammar) — this was already true of `TableHost`/`Table` before lane 3-F; these two tests lock it in.
  it("stamps a row's own id onto the rendered row's data-row-id attribute", () => {
    const { container } = render(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.space.home",
          controllerId: "s-home",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "col0", label: "Name" }]),
            rowsJson: JSON.stringify([{ id: "space:abc", col0: { kind: "text", value: "Atelier" } }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(container.querySelector('[data-row-id="space:abc"]')).not.toBeNull();
  });

  it("dispatches a row action button's own ActionDescriptor, unmodified, on click", () => {
    const onAction = vi.fn();
    const { container } = render(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.space.home",
          controllerId: "s-home",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([
              { id: "col0", label: "Name" },
              { id: "actions", label: "" },
            ]),
            rowsJson: JSON.stringify([
              {
                id: "space:abc",
                col0: { kind: "text", value: "Atelier" },
                actions: { kind: "buttons", buttons: [{ iconId: "trash-2", label: "delete", action: { controllerId: "s-home", action: "deleteSpace", args: { spaceId: "abc" } } }] },
              },
            ]),
          },
        },
        onAction,
      }),
    );
    const button = container.querySelector('[data-row-id="space:abc"] button');
    if (!button) throw new Error("row action button not found");
    fireEvent.click(button);
    expect(onAction).toHaveBeenCalledWith({ controllerId: "s-home", action: "deleteSpace", args: { spaceId: "abc" } });
  });

  it("renders vcs history host with an ancestor graph fork", () => {
    const columns = [
      {
        checkpointId: "c3",
        timestamp: "3",
        labels: ["feature-b"],
        authors: [],
        parentCheckpointId: "c2",
        description: "branch b",
        lane: 2,
        alternativeIds: ["b"],
      },
      {
        checkpointId: "c2",
        timestamp: "2",
        labels: ["feature-a"],
        authors: [],
        parentCheckpointId: "c1",
        description: "branch a",
        lane: 1,
        alternativeIds: ["a"],
      },
      {
        checkpointId: "c1",
        timestamp: "1",
        labels: ["main"],
        authors: [],
        description: "root",
        lane: 0,
        alternativeIds: [],
      },
    ];
    const markup = renderToStaticMarkup(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: {
            columnsJson: JSON.stringify(columns),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-graph-timeline-host");
    expect(markup).toContain('id="vcs.play.history.table"');
    expect(markup).toContain('d="M ');
    expect(markup.match(/<circle /g)?.length).toBe(3);
    expect(markup).toContain("branch b");
    expect(markup).toContain("feature-b");
  });

  it("renders paint-2d host canvas surface from document sync scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-surface-id="raster.play.viewport"');
    expect(markup).toContain('data-view-mode="composite"');
  });

  it("renders paint-2d navigator host with the composite viewport overlay channel", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.navigator",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            viewMode: "navigator",
            compositeViewportJson: '{"width":640,"height":480}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-view-mode="navigator"');
  });

  it("renders paint-2d host empty fallback without a scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.composite",
          controllerId: "raster-play",
          componentKind: "paint-2d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-empty");
  });

  it("interprets virtual file system component scenes", async () => {
    // 🧬️ MIGRATION: `Component::Surface`'s single `SurfaceProps.doc` (a pack-encoded opaque payload)
    // replaces the old `componentScene`/`virtualFileSystem` field pair — `surfacePropsToComponentSceneNode`
    // (Interpreter/🟦️component.tsx) decodes `doc.bytes` back into the exact scene sub-field shape
    // `VirtualFileSystemHost` (unowned, unchanged) already reads.
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const doc = {
      schemaJson: JSON.stringify({
        fileNodeKinds: { instance: { id: "instance", name: "Instance", descriptors: [] } },
        descriptorKinds: {},
        descriptorColumnIds: [],
      }),
      rowsJson: JSON.stringify([{ id: "row-1", fileNodeKindId: "instance", name: "Draw", path: "/draw", level: 0 }]),
    };
    const markup = renderContractTree({
      key: "s.play.media-vfs",
      component: {
        type: "surface",
        surfaceId: "s.play.media-vfs",
        controllerId: "s-play",
        kind: "virtualFileSystem",
        paneId: null,
        bindingId: null,
        docSchema: "virtualFileSystem@1",
        doc: { bytes: Array.from(encodePackValue(doc)) },
        domainId: null,
        domainGranularityId: null,
      },
    });
    expect(markup).toContain("Draw");
  });
});

describe("dag marquee overlay", () => {
  it("computes a rect overlay with numeric bounds for the rectangle method", () => {
    const pointsJson = JSON.stringify([
      { x: 10, y: 20 },
      { x: 30, y: 50 },
    ]);
    const overlay = computeDagMarqueeOverlay(pointsJson, false, "rectangle");
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "full" });
  });

  // Regression: board rust publishes `[[x,y],…]` (not `{x,y}` objects). Object-only parsing yielded
  // NaN bounds so the live select marquee never painted even though selection itself worked.
  it("computes a rect overlay from the rust tuple-array wire format", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 20],
        [30, 50],
        [10, 50],
      ]),
      true,
      "rectangle",
    );
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "partial" });
  });

  it("computes a lasso overlay carrying the raw points for the lasso method", () => {
    const points = [
      { x: 10, y: 20 },
      { x: 30, y: 50 },
      { x: 15, y: 40 },
    ];
    const overlay = computeDagMarqueeOverlay(JSON.stringify(points), true, "lasso");
    expect(overlay).toEqual({ kind: "lasso", points, coverage: "partial" });
  });

  it("computes a lasso overlay from the rust tuple-array wire format", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 50],
        [15, 40],
      ]),
      false,
      "lasso",
    );
    expect(overlay).toEqual({
      kind: "lasso",
      points: [
        { x: 10, y: 20 },
        { x: 30, y: 50 },
        { x: 15, y: 40 },
      ],
      coverage: "full",
    });
  });

  it("infers lasso from a non-rectangular path when method is omitted", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 50],
        [15, 40],
      ]),
      false,
    );
    expect(overlay?.kind).toBe("lasso");
  });

  it("infers rectangle from four axis-aligned corner points when method is omitted", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 20],
        [30, 50],
        [10, 50],
      ]),
      true,
    );
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "partial" });
  });

  it("returns null for fewer than two points", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 0, y: 0 }]), false, "rectangle")).toBeNull();
    expect(computeDagMarqueeOverlay(JSON.stringify([[0, 0]]), false, "rectangle")).toBeNull();
  });

  it("returns null for malformed point entries", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([[10], [20, 30]]), false, "rectangle")).toBeNull();
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 10 }, { x: 20, y: 30 }]), false, "rectangle")).toBeNull();
  });

  // Regression: node-graph-host.tsx used to pass `shape={{ shape: "polygon", points }}` (a single
  // nested-object prop) instead of separate `shape`/`points` props, so `props.shape === "rect"` was
  // always false and the polygon branch read `props.points` as undefined — crashing on every marquee
  // drag and tripping the shell's render error boundary (visible as an interaction "reset").
  it("renders a rect overlay from a computeDagMarqueeOverlay rect result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [0, 0],
        [40, 25],
      ]),
      false,
      "rectangle",
    );
    if (!overlay || overlay.kind !== "rect") throw new Error("expected rect overlay");
    const markup = renderToStaticMarkup(
      createElement(SelectionMarquee, {
        coverage: overlay.coverage ?? "full",
        shape: "rect",
        rect: { x: overlay.x ?? 0, y: overlay.y ?? 0, width: overlay.width ?? 0, height: overlay.height ?? 0 },
      }),
    );
    expect(markup).toContain("<rect");
    expect(markup).toContain('width="40"');
    expect(markup).toContain('height="25"');
  });

  it("renders a polygon overlay from a computeDagMarqueeOverlay lasso result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [0, 0],
        [40, 25],
        [5, 30],
      ]),
      false,
      "lasso",
    );
    if (!overlay || overlay.kind !== "lasso") throw new Error("expected lasso overlay");
    const markup = renderToStaticMarkup(createElement(SelectionMarquee, { coverage: overlay.coverage ?? "full", shape: "polygon", points: overlay.points ?? [] }));
    expect(markup).toContain("<polygon");
    expect(markup).toContain("0,0 40,25 5,30");
  });
});

describe("ink canvas host", () => {
  const semioInkDocument: InkDocument = {
    schema: "ink.document",
    id: "semio",
    title: "Semio Note",
    camera: { x: 0, y: 0, zoom: 1 },
    activeUtility: "selectDirect",
    gridVisible: true,
    snapEnabled: false,
    pencilWidth: 3,
    eraserRadius: 12,
    blocks: [
      {
        kind: "text",
        id: "welcome-text",
        name: "Welcome",
        x: 80,
        y: 80,
        width: 360,
        height: 120,
        visible: true,
        locked: false,
        paragraphs: [{ runs: [{ text: "Welcome to Note — an infinite canvas for text, images, tables, math, and pencil ink." }] }],
        fontSize: 20,
        fontWeight: "normal",
        align: "left",
      },
      { kind: "math", id: "welcome-math", name: "Equation", x: 80, y: 240, width: 240, height: 80, visible: true, locked: false, tex: "E = mc^2", displayMode: true },
      {
        kind: "table",
        id: "welcome-table",
        name: "Blocks",
        x: 80,
        y: 360,
        width: 360,
        height: 140,
        visible: true,
        locked: false,
        columns: ["Block", "Description"],
        rows: [
          [{ content: "Text" }, { content: "Rich text blocks" }],
          [{ content: "Math" }, { content: "TeX equations" }],
          [{ content: "Ink" }, { content: "Freehand pencil strokes" }],
        ],
      },
    ],
  };

  it("renders the semio example composite scene with rich text, table, and math fallback", () => {
    const markup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: {
          type: "componentScene",
          surfaceId: "note.play.composite",
          controllerId: "note-play",
          componentKind: "ink-canvas",
          inkCanvas: {
            documentJson: JSON.stringify(semioInkDocument),
            selectionJson: "[]",
            activeUtility: "selectDirect",
            viewMode: "composite",
            interactive: true,
          },
        },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(markup).toContain("Welcome to Note");
    expect(markup).toContain("<table");
    expect(markup).toMatch(/\$\$E = mc\^2\$\$|annotation encoding="application\/x-tex">E = mc\^2</);
    expect(markup).toContain('data-surface-id="note.play.composite"');
  });

  it("shows the grid pattern in composite mode but not in navigator mode", () => {
    const baseNode = {
      type: "componentScene" as const,
      surfaceId: "note.play.composite",
      controllerId: "note-play",
      componentKind: "ink-canvas",
    };
    const compositeMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "composite", interactive: true } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(compositeMarkup).toContain("ink-viewport-grid");

    const navigatorMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "navigator", interactive: false } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(navigatorMarkup).not.toContain("ink-viewport-grid");
  });

  it("resizes with a minimum size and scales ink points when a group is resized", () => {
    const fromBounds = { x: 0, y: 0, width: 100, height: 100 };
    const shrunk = inkResizeBounds(fromBounds, "e", -1000, 0);
    expect(shrunk.width).toBe(8);

    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [100, 100],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const scaled = inkScaleItemWithinGroup(ink, { x: 0, y: 0, width: 100, height: 100 }, { x: 0, y: 0, width: 200, height: 50 });
    expect(scaled.kind).toBe("stroke");
    if (scaled.kind === "stroke")
      expect(scaled.points).toEqual([
        [0, 0],
        [200, 50],
      ]);
  });

  it("splits an ink stroke into fragments when erasing its middle point", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 80,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [40, 0],
        [80, 0],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const fragments = eraseInkStrokePointsInItem(ink, 40, 0, 5);
    expect(fragments).toHaveLength(0);
    const wideStroke: InkStrokeItem = {
      ...ink,
      points: [
        [0, 0],
        [10, 0],
        [40, 0],
        [70, 0],
        [80, 0],
      ],
    };
    const splitFragments = eraseInkStrokePointsInItem(wideStroke, 40, 0, 5);
    expect(splitFragments).toHaveLength(2);
  });

  it("round-trips bold and link marks between paragraphs and html", () => {
    const html = inkParagraphsToHtml([{ runs: [{ text: "hello", bold: true, link: "https://semio.tech" }] }]);
    expect(html).toContain("<strong>");
    expect(html).toContain('href="https://semio.tech"');
  });

  it("round-trips a clipboard payload of note blocks", () => {
    const payload = inkClipboardPayload([semioInkDocument.blocks[1]!]);
    const parsed = inkItemsFromClipboardPayload(payload);
    expect(parsed).toHaveLength(1);
    expect(parsed?.[0]?.kind).toBe("math");
  });

  it("computes ink block bounds from its local points", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 10,
      y: 10,
      width: 1,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [5, 5],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    expect(inkItemBounds(ink)).toEqual({ x: 10, y: 10, width: 5, height: 5 });
  });

  it("applies the canonical wheel-zoom camera formula symmetrically for screen<->world conversion", () => {
    const camera = { x: 50, y: 50, zoom: 2 };
    const world = screenToWorld(camera, 150, 150);
    expect(world).toEqual([50, 50]);
    expect(worldToScreen(camera, 50, 50)).toEqual({ x: 150, y: 150 });
  });
});

describe("spawned window chrome", () => {
  const app = {
    id: "cad-play",
    label: "CAD",
    breadcrumb: ["semio", "cad"],
    controllerId: "cad-play",
    defaultModeId: "edit",
    modes: [
      {
        id: "edit",
        label: "Edit",
        utilities: [{ id: "static-utility", kind: "button" as const, iconId: "save", controllerId: "cad-play", action: "save" }],
      },
    ],
    windowKinds: [
      {
        id: "cad-window-shape",
        label: "Shape",
        bodyKey: "shape",
        options: {
          engagement: {
            kind: "some" as const,
            value: {
              input: {
                id: "engagement-input",
                placeholder: "Action",
                onChange: { controllerId: "cad-play", action: "engagementInput" },
              },
              possibleEngagements: [{ id: "box", label: "Box", action: { controllerId: "cad-play", action: "startBox" } }],
            },
          },
          measures: [{ id: "render-mode", kind: "select" as const, label: "Render Mode", value: "shaded", items: [], onChange: { controllerId: "cad-play", action: "setRenderMode" } }],
        },
      },
    ],
    panelTabs: [],
    keybindings: [],
  };

  it("builds spawned engagement and measures chrome from program contributions", () => {
    const kind = app.windowKinds[0]!;
    const engagements = {
      [kind.id]: {
        input: {
          id: "engagement-input",
          value: "Box",
          placeholder: "Action",
          onChange: { controllerId: "cad-play", action: "engagementInput" },
        },
        possibleEngagements: [{ id: "box", label: "Box", detail: "b", action: { controllerId: "cad-play", action: "startBox" } }],
      },
    };
    const measures = { [kind.id]: kind.options.measures ?? [] };
    const chrome = spawnedWindowChromeForKind(kind, kind.id, engagements, measures, undefined, noopAction);
    expect(chrome.search?.input?.value).toBe("Box");
    expect(chrome.search?.possibles?.[0]?.label).toBe("Box");
    const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
    expect(measuresMarkup).toContain("Render Mode");
  });
});

describe("partitionWindowMeasures", () => {
  const utilityGroup = (id: string, activeUtilityId: string | undefined, children: WindowMeasure[] = []): WindowMeasure => ({ kind: "group", id, label: id, activeUtilityId, children });
  const slider = (id: string): WindowMeasure => ({ kind: "slider", id, value: 1, min: 0, max: 2, onChange: { controllerId: "c", action: "a" } });

  it("unwraps a tagged group's children into utilityOptions only when its utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush", [slider("size")]), slider("zoom")];
    const active = partitionWindowMeasures(measures, "brush");
    expect(active.utilityOptions.map((m) => m.id)).toEqual(["size"]);
    expect(active.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("drops a tagged group from both buckets when a different or no utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush", [slider("size")]), slider("zoom")];
    const other = partitionWindowMeasures(measures, "fill");
    expect(other.utilityOptions).toEqual([]);
    expect(other.general.map((m) => m.id)).toEqual(["zoom"]);
    const none = partitionWindowMeasures(measures, undefined);
    expect(none.utilityOptions).toEqual([]);
    expect(none.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("keeps untagged groups and non-group measures in general, unaffected by the active utility", () => {
    const measures = [utilityGroup("grid", undefined), slider("zoom")];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "brush");
    expect(general.map((m) => m.id)).toEqual(["grid", "zoom"]);
    expect(utilityOptions).toEqual([]);
  });

  it("wires a utility-scoped group into spawnedWindowChromeForKind's utilityOptions slot only when its utility is active", () => {
    const kind = { id: "w", label: "W", bodyKey: "b", surfaceKind: "paint-2d", options: { engagement: { kind: "none" as const }, measures: [] } } as unknown as AppWindowKindDefinition;
    const brushGroup: WindowMeasure = {
      kind: "group",
      id: "brush-params",
      label: "Brush",
      defaultOpen: true,
      activeUtilityId: "brush",
      children: [{ kind: "slider", id: "size", label: "Brush size", value: 4, min: 1, max: 10, onChange: { controllerId: "c", action: "setSize" } }],
    };
    const measures = { [kind.id]: [brushGroup] };
    const activeChrome = spawnedWindowChromeForKind(kind, kind.id, {}, measures, "brush", noopAction);
    const activeMarkup = renderToStaticMarkup(activeChrome.utilityOptions as ReactElement);
    expect(activeMarkup).toContain("Brush size");
    expect(activeMarkup).toContain('data-direction="up"');
    expect(activeChrome.measures).toBeUndefined();
    const idleChrome = spawnedWindowChromeForKind(kind, kind.id, {}, measures, "fill", noopAction);
    expect(idleChrome.utilityOptions).toBeUndefined();
    expect(idleChrome.measures).toBeUndefined();
  });

  it("parses the real ui_wgpu camelCase wire JSON and unwraps a utility-scoped fill group into flat utilityOptions (snake_case divergence regression guard)", () => {
    // Verbatim shape of `ui_wgpu::WindowMeasure`'s serde wire after the D-4 `rename_all_fields = "camelCase"`
    // fix: a fill-utility slider group tagged with `activeUtilityId`, plus an untagged toggle. This is the exact
    // class of payload whose snake_case↔camelCase divergence made the puzzle fill slider invisible in React.
    const wireJson =
      '[{"kind":"group","id":"fill-params","label":"Fill","activeUtilityId":"fill","children":[{"kind":"slider","id":"fillCount","label":"Count","value":3,"min":1,"max":9,"step":1,"onChange":{"controllerId":"puzzle","action":"setFillCount"}}]},{"kind":"toggle","id":"grid","iconId":"layout-grid","pressed":true,"onChange":{"controllerId":"puzzle","action":"toggleGrid"}}]';
    const measures = JSON.parse(wireJson) as WindowMeasure[];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "fill");
    expect(utilityOptions.map((m) => m.id)).toEqual(["fillCount"]);
    expect(utilityOptions[0]).toMatchObject({ kind: "slider", id: "fillCount", onChange: { action: "setFillCount" } });
    expect(general.map((m) => m.id)).toEqual(["grid"]);
    const gridToggle = general[0];
    expect(gridToggle.kind === "toggle" && gridToggle.iconId).toBe("layout-grid");

    // Regression guard for the fixed bug: the pre-fix snake_case wire leaves `activeUtilityId` undefined, so the
    // tagged group silently falls through to `general` and the fill slider never reaches the Utility Options rail.
    const legacyJson = wireJson
      .replace(/"activeUtilityId"/g, '"active_utility_id"')
      .replace(/"onChange"/g, '"on_change"')
      .replace(/"iconId"/g, '"icon_id"');
    const legacy = JSON.parse(legacyJson) as WindowMeasure[];
    const legacyPartition = partitionWindowMeasures(legacy, "fill");
    expect(legacyPartition.utilityOptions).toEqual([]);
    expect(legacyPartition.general.map((m) => m.id)).toEqual(["fill-params", "grid"]);
  });
});

describe("utility ribbon", () => {
  it("sorts utility nodes by order", () => {
    const sorted = sortUtilityNodes([
      { id: "b", kind: "button", iconId: "box", order: 2, controllerId: "x", action: "b" },
      { id: "a", kind: "button", iconId: "box", order: 1, controllerId: "x", action: "a" },
    ]);
    expect(sorted.map((node) => node.id)).toEqual(["a", "b"]);
  });

  it("recurses into a collection level only when the path names one of its collections", () => {
    const tree = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [
          {
            id: "view-tools",
            kind: "collection",
            iconId: "zoom-in",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
        ],
      },
      {
        id: "construct",
        kind: "collection",
        iconId: "box",
        children: [
          {
            id: "construct-tools",
            kind: "collection",
            iconId: "box",
            children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
          },
        ],
      },
    ];

    const noActive = buildUtilityRibbonSegments(tree, []);
    expect(noActive).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);

    const oneActive = buildUtilityRibbonSegments(tree, ["construct"]);
    expect(oneActive[0]).toMatchObject({ kind: "picker", depth: 0 });
    expect(oneActive[1]).toMatchObject({ kind: "picker", depth: 1, collections: tree[1].children });
    expect(oneActive).toHaveLength(2);

    const twoActive = buildUtilityRibbonSegments(tree, ["construct", "construct-tools"]);
    const utilitiesSegment = twoActive.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "box"));
    expect(utilitiesSegment).toMatchObject({ depth: 2 });
  });

  it("ignores a path entry that no longer names an enabled collection at that level", () => {
    const tree = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
      },
    ];
    expect(buildUtilityRibbonSegments(tree, ["nonexistent"])).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);
  });

  it("emits a picker segment alongside loose leaves at the same depth", () => {
    const segments = buildUtilityRibbonSegments(
      [
        { id: "undo", kind: "button", iconId: "undo", controllerId: "x", action: "undo" },
        {
          id: "view",
          kind: "collection",
          iconId: "eye",
          children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
        },
      ],
      [],
    );
    expect(segments).toEqual([
      { kind: "picker", collections: [expect.objectContaining({ id: "view" })], depth: 0 },
      { kind: "utilities", items: [expect.objectContaining({ id: "undo" })], depth: 0 },
    ]);
  });

  it("reconciles an active path by truncating at the first stale entry instead of substituting a default", () => {
    const tree = [
      {
        id: "a",
        kind: "collection",
        iconId: "box",
        children: [
          { id: "x", kind: "collection", iconId: "box", children: [{ id: "leaf", kind: "button", iconId: "box", controllerId: "c", action: "act" }] },
          { id: "y", kind: "collection", iconId: "box", children: [] },
        ],
      },
      { id: "b", kind: "collection", iconId: "box", children: [] },
    ];
    expect(reconcileUtilityPath(tree, ["a", "x"])).toEqual(["a", "x"]);
    expect(reconcileUtilityPath(tree, ["a", "gone"])).toEqual(["a"]);
    expect(reconcileUtilityPath(tree, ["gone"])).toEqual([]);
    expect(reconcileUtilityPath(tree, [])).toEqual([]);
  });

  it("buckets top-level utility nodes into ordered category collections (uncategorized nodes default to tools now that the Actions category is gone)", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "sel", kind: "toggle", iconId: "cursor", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
      { id: "act", kind: "button", iconId: "wand", controllerId: "x", action: "run" },
      { id: "tool", kind: "toggle", iconId: "pen", controllerId: "x", action: "pen" },
      { id: "sync", kind: "toggle", iconId: "cloud", controllerId: "x", action: "sync", category: "sync" },
    ]);
    expect(grouped.map((node) => node.id)).toEqual(["selection", "utilities", "history", "sync"]);
    expect(grouped.every((node) => node.kind === "collection")).toBe(true);
  });

  it("drops separator-only category buckets so an empty group never appears as a picker option", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "sep", kind: "separator" },
    ]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0].id).toBe("utilities");
  });

  it("reuses a category's single already-meaningful collection instead of re-wrapping it, avoiding a duplicate-looking picker level", () => {
    const selectionCollection = {
      id: "lowpoly-tools-selection",
      kind: "collection" as const,
      iconId: "mouse-pointer",
      label: "Selection",
      category: "selection" as const,
      children: [{ id: "mesh", kind: "toggle" as const, iconId: "box", controllerId: "x", action: "mesh" }],
    };
    const grouped = groupUtilityNodesByCategory([selectionCollection]);
    expect(grouped).toEqual([{ ...selectionCollection, order: 0 }]);
    const segments = buildUtilityRibbonSegments(grouped, ["lowpoly-tools-selection"]);
    const utilitiesSegment = segments.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "mesh"));
    expect(utilitiesSegment).toBeTruthy();
  });

  it("still wraps a category with multiple top-level nodes in a synthetic collection", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "b", kind: "button", iconId: "box", controllerId: "x", action: "b", category: "utilities" },
    ]);
    expect(grouped).toEqual([{ id: "utilities", kind: "collection", iconId: "wrench", text: "utilities", order: 0, category: "utilities", children: expect.any(Array) }]);
  });

  it("scopes grouping to the given categories only", () => {
    const nodes = [
      { id: "sel", kind: "toggle", iconId: "cursor", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
    ];
    expect(groupUtilityNodesByCategory(nodes, ["selection", "utilities"]).map((node) => node.id)).toEqual(["selection"]);
    expect(groupUtilityNodesByCategory(nodes, ["utilities", "history"]).map((node) => node.id)).toEqual(["history"]);
  });

  it("deduplicates utility nodes by id across window utility lists for a single shared footer entry", () => {
    const history = { id: "s-play.history", kind: "collection" as const, iconId: "clock", category: "history" as const, children: [] };
    const deduped = dedupeUtilityNodesById([[history, { id: "leaf-a", kind: "button" as const, iconId: "box", controllerId: "x", action: "a" }], [history], []]);
    expect(deduped).toEqual([history, { id: "leaf-a", kind: "button", iconId: "box", controllerId: "x", action: "a" }]);
  });

  it("renders utility ribbon with picker and batched toggles", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [
              {
                id: "view-tools",
                kind: "collection",
                iconId: "eye",
                children: [
                  { id: "show-edges", kind: "toggle", iconId: "box", pressed: true, controllerId: "x", action: "edges" },
                  { id: "show-faces", kind: "toggle", iconId: "square", pressed: false, controllerId: "x", action: "faces" },
                ],
              },
            ],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities"');
    expect(markup).toContain('data-slot="toggle-group"');
  });

  it("stacks the window utility bar ribbon upward, showing only the base picker row until a group is activated", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        direction: "up",
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-slot="ribbon"');
    expect(markup).toContain('data-direction="up"');
    expect(markup).toContain("flex-col-reverse");
    // No active path given, so neither group is expanded: exactly one ribbon row (the base picker).
    expect(markup.match(/data-slot="ribbon-row"/g)?.length).toBe(1);
    expect(markup).toContain('data-slot="toggle-group"');
    expect(markup).not.toContain('id="zoom-in"');
  });

  it("renders UtilityTree with a custom id for per-window namespacing", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.model",
        utilities: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities.model"');
    expect(markup).not.toContain('id="ui.utilities"');
  });

  it("renders utilityOptions as an extra ribbon row when direction is up", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.w",
        direction: "up",
        utilities: [{ id: "brush", kind: "toggle", iconId: "brush", pressed: true, controllerId: "x", action: "setActiveUtility" }],
        utilityOptions: createElement("span", { "data-testid": "brush-options" }, "Brush size"),
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-testid="brush-options"');
    expect(markup).toContain("Brush size");
    expect(markup).toContain('data-variable-height="true"');
    expect(markup).toContain("h-auto min-h-medium");
    expect(markup).toContain("items-start");
    const variableZoneClass = markup.match(/data-variable-height="true" class="([^"]*)"/)?.[1].split(" ") ?? [];
    expect(variableZoneClass).toContain("h-auto");
    expect(variableZoneClass).not.toContain("h-medium");
    expect(markup).toContain("h-auto items-start");
  });
});

describe("s workflow flow routing", () => {
  it("selects the flow engine for scenes with engine flow capabilities", () => {
    expect(isFlowGraphScene('{"engine":"flow","spotlight":false,"noteEdit":false}')).toBe(true);
    expect(isFlowGraphScene('{"spotlight":false,"noteEdit":false,"clusters":false}')).toBe(false);
    expect(isFlowGraphScene(undefined)).toBe(false);
  });

  it("renders presence peers from the scene payload", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodesJson: "[]",
            edgesJson: "[]",
            viewportJson: '{"x":0,"y":0,"zoom":1}',
            presencePeersJson: JSON.stringify([{ clientId: "client-b", name: "Ada", selectionCount: 2 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("Ada");
    expect(markup).toContain("2 selected");
  });

  it("parses a catalogue app drag payload, ignoring extra keys", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ pluginId: "s.system", appId: "draw", label: "Draw", extra: "x" }))).toEqual({
      pluginId: "s.system",
      appId: "draw",
      label: "Draw",
    });
  });

  it("rejects catalogue app drag payloads missing pluginId/appId, and garbage", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ appId: "draw" }))).toBeNull();
    expect(parseCatalogueAppDragPayload(JSON.stringify({ kind: "neuron" }))).toBeNull();
    expect(parseCatalogueAppDragPayload("not json")).toBeNull();
  });

  it("builds a ghost neuron descriptor, preferring label over appId", () => {
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw", label: "Draw" }))).toEqual({ kind: "neuron", neuronKind: "Draw" });
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw" }))).toEqual({ kind: "neuron", neuronKind: "draw" });
  });

  it("builds addWidget descriptors from catalogue items", () => {
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" }))).toEqual({
      kind: "neuron",
      neuronKind: "math.add",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "outputExport", format: "svg", name: "Export SVG", abbreviation: "SVG", icon: "emoji:📤️", summary: "" }))).toEqual({
      kind: "outputExport",
      format: "svg",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" }))).toEqual({ kind: "inputSlider" });
  });

  it("ranks catalogue suggestions by exact/prefix match with neurons first", () => {
    const sections = [
      {
        id: "inputs",
        title: "Inputs",
        items: [
          { kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" },
          { kind: "inputNote", name: "Note", abbreviation: "Note", icon: "emoji:📝️", summary: "" },
        ],
      },
      {
        id: "math",
        title: "Math",
        items: [
          { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" },
          { kind: "neuron", neuronKind: "math.subtract", name: "Subtract", abbreviation: "Sub", icon: "emoji:➖️", summary: "" },
        ],
      },
    ];
    expect(flowRankCatalogueSuggestions(sections, "add").map((item) => item.neuronKind ?? item.kind)).toEqual(["math.add"]);
    expect(flowRankCatalogueSuggestions(sections, "sl").map((item) => item.kind)).toEqual(["inputSlider"]);
    const brepSections = [
      {
        id: "brep",
        title: "Brep",
        items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Axis-aligned box" }],
      },
    ];
    expect(flowRankCatalogueSuggestions(brepSections, "brep").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    expect(flowRankCatalogueSuggestions(brepSections, "box").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    const empty = flowRankCatalogueSuggestions(sections, "");
    expect(empty[0]?.kind).toBe("neuron");
    expect(empty.some((item) => item.kind === "inputSlider")).toBe(true);
  });

  it("returns every catalogue item for empty query without a 20-item cap", () => {
    const items = Array.from({ length: 25 }, (_, index) => ({
      kind: "neuron",
      neuronKind: `math.operation${index}`,
      name: `Operation ${index}`,
      abbreviation: `Operation${index}`,
      icon: "emoji:➕️",
      summary: "",
    }));
    const sections = [{ id: "math", title: "Math", items }];
    expect(flowRankCatalogueSuggestions(sections, "")).toHaveLength(25);
  });

  it("enables overflow scrolling only when spotlight suggestions are expanded", () => {
    expect(flowSpotlightSuggestionListScrollClass(false)).toContain("overflow-hidden");
    expect(flowSpotlightSuggestionListScrollClass(false)).not.toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("max-h-[min(24rem,70vh)]");
  });

  it("attaches a drag-and-drop controller to tree panels whose items carry drag data", () => {
    const config = uiNodeToTreePanelConfig(
      {
        type: "tree",
        sections: [
          {
            id: "catalogue",
            label: "Catalogue",
            items: [{ id: "s-play-catalogue.document.draw", label: "Draw", draggable: true, dragData: { "application/x-semio-catalogue-item": '{"pluginId":"s.system","appId":"draw"}' } }],
          },
        ],
      },
      noopAction,
    );
    expect(config.dragAndDropController).toBeDefined();
  });

  it("omits the drag-and-drop controller for tree panels without drag data", () => {
    const config = uiNodeToTreePanelConfig({ type: "tree", sections: [{ id: "catalogue", label: "Catalogue", items: [{ id: "s-play-catalogue.document.draw", label: "Draw" }] }] }, noopAction);
    expect(config.dragAndDropController).toBeUndefined();
  });

  it("resolves a fixture widget id to its workflow instance id, independent of selection state", () => {
    const fixtureJson = JSON.stringify({
      widgets: [
        { id: "widget-1", params: { instanceId: "app-1" } },
        { id: "widget-2", params: {} },
      ],
    });
    expect(resolveFixtureWidgetInstanceId(fixtureJson, "widget-1")).toBe("app-1");
    expect(resolveFixtureWidgetInstanceId(fixtureJson, "widget-2")).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId(fixtureJson, "missing-widget")).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId(fixtureJson, undefined)).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId(undefined, "widget-1")).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId("not json", "widget-1")).toBeUndefined();
  });

  it("parses studio and studio+instance shell paths, and rejects non-studio routes", () => {
    expect(parseSpaceShellPath("/spaces/my-studio")).toEqual({ spaceId: "my-studio", instanceId: undefined });
    expect(parseSpaceShellPath("/spaces/my-studio/instances/inst-1")).toEqual({ spaceId: "my-studio", instanceId: "inst-1" });
    expect(parseSpaceShellPath("/")).toBeNull();
    expect(parseSpaceShellPath("/spaces/my-studio/instances/inst-1/extra")).toBeNull();
  });

  it("classifies shell routes into landing, space, and notFound", () => {
    expect(parseShellRoute("/")).toEqual({ kind: "landing" });
    expect(parseShellRoute("/spaces/my-studio")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: undefined });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: "inst-1" });
    expect(parseShellRoute("/unknown/path")).toEqual({ kind: "notFound", path: "/unknown/path" });
  });

  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3/§C6 — pure-function
  // coverage of the identity/directory helpers `ShellHost/🟦️component.tsx` exports for this. Full
  // end-to-end coverage (a real `openDocument` binding snapshot with/without a resolved identity, an
  // `os.open-artifact{documentId}` effect resulting in a worker `open` request with hub+folder
  // bindings) would need `fetch`/`Worker`/`DirectoryClient` mocking this already-huge shared suite has
  // no existing pattern for; not added this lane — see `📓️w2-c-report.md`.
  it("shellActorId mints user:{userId}#{sessionId} once identity resolves, else client-{sessionId}", () => {
    expect(shellActorId("sess-1", null)).toBe("client-sess-1");
    expect(
      shellActorId("sess-1", { userId: "u-1", email: "u1@semio.dev", displayName: "U1", hubBaseUrl: "http://127.0.0.1:8787", sessionToken: "tok", issuedAtMs: 0 }),
    ).toBe("user:u-1#sess-1");
  });

  it("canonicalSurfaceId formats <kind>@<standard>/<subset>#<role>", () => {
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "editor")).toBe("s.space.space@1/*#editor");
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "viewer")).toBe("s.space.space@1/*#viewer");
  });

  it("directoryCommandFromAction maps all 7 frozen os.directory.* ids, share-link sugaring to create-invite", () => {
    expect(directoryCommandFromAction("os.directory.create-space", { name: "Atelier", spaceKind: "atelier", visibility: "private" })).toEqual({
      kind: "create-space",
      name: "Atelier",
      spaceKind: "atelier",
      visibility: "private",
    });
    expect(directoryCommandFromAction("os.directory.delete-space", { spaceId: "sp-1" })).toEqual({ kind: "delete-space", spaceId: "sp-1" });
    expect(directoryCommandFromAction("os.directory.rename-space", { spaceId: "sp-1", name: "New" })).toEqual({ kind: "rename-space", spaceId: "sp-1", name: "New" });
    expect(directoryCommandFromAction("os.directory.set-visibility", { spaceId: "sp-1", visibility: "public" })).toEqual({ kind: "set-visibility", spaceId: "sp-1", visibility: "public" });
    expect(directoryCommandFromAction("os.directory.upsert-member", { spaceId: "sp-1", email: "a@b.com", role: "author" })).toEqual({
      kind: "upsert-member",
      spaceId: "sp-1",
      email: "a@b.com",
      role: "author",
    });
    expect(directoryCommandFromAction("os.directory.remove-member", { spaceId: "sp-1", userId: "u-1" })).toEqual({ kind: "remove-member", spaceId: "sp-1", userId: "u-1" });
    expect(directoryCommandFromAction("os.directory.share-link", { spaceId: "sp-1", role: "spectator", ttlSecs: 60 })).toEqual({
      kind: "create-invite",
      spaceId: "sp-1",
      role: "spectator",
      ttlSecs: 60,
    });
    expect(directoryCommandFromAction("os.unknownVerb", {})).toBeNull();
  });

  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C5 — save/check-in policy
  // (lane 3-A). `AutoCheckinScheduler` is deliberately framework-free (see its own doc) so its
  // debounce/storm-guard behaviour is verifiable with fake timers directly, without mounting
  // `ShellHost` — the same "no Worker/fetch/DirectoryClient mocking pattern exists yet" ceiling
  // `📓️w2-c-report.md` already documented applies here too; `ShellHost`'s own wiring of this
  // scheduler (and the pill's rendering into the sync tab) is reviewed, not click/mount-tested.
  describe("AutoCheckinScheduler (§C5 auto check-in)", () => {
    afterEach(() => {
      vi.useRealTimers();
    });

    it("3 edits then idle ⇒ exactly one commitCheckpoint", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(2);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(3);
      expect(onCheckpoint).not.toHaveBeenCalled();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      // 🎯️ "never a storm": more time passing without a fresh `notify` never fires a second time.
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 3);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("≥ 200 uncommitted edits ⇒ checkpoint without waiting for idle", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD - 1);
      expect(onCheckpoint).not.toHaveBeenCalled();
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      // 🎯️ "never a storm": a second `notify` at/above the threshold before the checkpoint's own
      // `notify(0)` lands must not fire again.
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD + 1);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("notify(0) (a landed checkpoint) clears the pending latch for a fresh idle window later", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      scheduler.notify(0);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(2);
    });

    it("cancel() stops a pending idle timer (unmount/session-switch)", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      scheduler.cancel();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 2);
      expect(onCheckpoint).not.toHaveBeenCalled();
    });
  });

  describe("sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...))", () => {
    it("persisted: a live remote with nothing pending", () => {
      const state = computeSyncPillState({ persisted: true, pendingMutations: 0, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "persisted" });
      expect(syncPillText(state, "en")).toBe("Persisted");
      expect(syncPillText(state, "de")).toBe("Gespeichert");
    });

    it("pending(n): a live remote with unacked mutations", () => {
      const state = computeSyncPillState({ persisted: false, pendingMutations: 3, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "pending", count: 3 });
      expect(syncPillText(state, "en")).toBe("Pending (3)");
      expect(syncPillText(state, "de")).toBe("Ausstehend (3)");
    });

    it("remote(connecting|backoff|detached): a non-live remote takes priority over a pending count", () => {
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "en")).toBe("Remote: connecting");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "de")).toBe("Remote: verbindet");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 9, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "en")).toBe("Remote: detached");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "de")).toBe("Remote: getrennt");
    });

    it("no status observed yet reads as remote(detached)", () => {
      expect(computeSyncPillState(null)).toEqual({ kind: "remote", remote: "detached" });
    });
  });

  // 🧪️ §C5 item 5: "viewers never checkpoint" — `canCheckIn` is the SAME predicate `ShellHost` gates
  // both the `#s-checkin`/checkpoint footer items (JSX presence, `!canCheckIn(session.app.role)`) and
  // the auto-checkin scheduler's arming (`isEditorSession`) with, so this one test covers both call
  // sites' logic without needing to mount `ShellHost` itself.
  it("canCheckIn is true only for an editor role — viewer gets no affordance and no auto timer", () => {
    expect(canCheckIn("editor")).toBe(true);
    expect(canCheckIn("viewer")).toBe(false);
    expect(canCheckIn(undefined)).toBe(false);
  });

  it("isolates render faults in ShellFaultBoundary", () => {
    function FaultyChild(): ReactElement {
      throw new Error("boom");
    }
    const { getByRole } = render(
      createElement(
        ShellFaultBoundary,
        { boundaryId: "test", fallbackLabel: "Fault" as never },
        createElement(FaultyChild),
      ),
    );
    expect(getByRole("alert")).toHaveTextContent("boom");
  });

  it("folds spawned focus into viewState so a subsequent host-effect session write keeps activeSpawnedId", async () => {
    const panel = {
      activePanelTab: "s-play-catalogue",
      programs: [{ pluginId: "draw", workflowStepId: "draw", appId: "draw", label: "Draw", breadcrumb: ["draw"], yields: "2d.drawing" }],
      spawnedApps: [] as const,
    };
    const spawned = { id: "app-draw-1", pluginId: "draw", instanceId: 1, appId: "draw", label: "Semio Emblem", breadcrumb: ["draw"] };
    const focused = studioPanelFocusingSpawned(panel, spawned);
    expect(focused.activeSpawnedId).toBe("app-draw-1");
    expect(focused.spawnedApps).toEqual([spawned]);
    // 🐚️ Simulate applyHostEffects: fold into nextViewState, then a final SET_SESSION commits that
    // viewState (the bug was committing the pre-spawn viewState and wiping activeSpawnedId).
    const baseViewState = { panelJson: JSON.stringify(panel) };
    const nextViewState = viewStateWithSpacePanel(baseViewState, focused);
    const { packValueFromBase64 } = await import("@semio-tech/framework-os");
    expect((packValueFromBase64(nextViewState.panelJson!) as { activeSpawnedId?: string }).activeSpawnedId).toBe("app-draw-1");
    const refocused = studioPanelFocusingSpawned(focused, { ...spawned, label: "Renamed" });
    expect(refocused.spawnedApps).toHaveLength(1);
    expect(refocused.spawnedApps[0]?.label).toBe("Renamed");
    expect(refocused.activeSpawnedId).toBe("app-draw-1");
  });
});

describe("ui search/find (fuse re-export from @semio-tech/ui-react)", () => {
  // Command dialogs render via a Radix Portal into `document.body`, not into the render() container, so assertions query `document.body`.
  // This package's vitest config has no shared setupFile, so tests here clean up their own portal-rendered DOM.
  afterEach(async () => {
    const { cleanup } = await import("@testing-library/react");
    cleanup();
  });

  it("UISearch renders all items and fuzzy-filters them via the shared Fuse re-export", async () => {
    const { render, fireEvent } = await import("@testing-library/react");
    const items: UISearchItem[] = [
      { id: "a", label: "Alpha", category: "Test", onSelect: noopAction },
      { id: "b", label: "Bravo", category: "Test", onSelect: noopAction },
    ];
    render(createElement(UIFindProvider, null, createElement(UISearch, { items, open: true, onOpenChange: noopAction })));
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).toContain("Bravo");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "alp" } });
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).not.toContain("Bravo");
  });

  it("UIFind renders and fuzzy-filters items registered on its context via the shared Fuse re-export", async () => {
    const { render, fireEvent, act } = await import("@testing-library/react");
    let contextValue: ReturnType<typeof useUIFind> | undefined;
    const Harness = () => {
      contextValue = useUIFind();
      return createElement(UIFind, { open: true, onOpenChange: noopAction });
    };
    render(createElement(UIFindProvider, null, createElement(Harness)));
    act(() => {
      contextValue!.setFindItems([
        { id: "1", label: "Chair", category: "Test" },
        { id: "2", label: "Table", category: "Test" },
      ]);
    });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).toContain("Table");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "cha" } });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).not.toContain("Table");
  });
});

// 🧰️ Window Actions & Utilities Contract (WS-2): staged argument forms (P1/P2), palette redirect (P3),
// keybinding rule (P4), and registry-derived utility activation (P5).
describe("window action panel — staging and single dispatch (P1/P2)", () => {
  afterEach(() => cleanup());

  const numberArg = (id: string, required: boolean, def?: number): ActionArgDef => ({ id, label: id[0]!.toUpperCase() + id.slice(1), control: { kind: "number" }, required, ...(def === undefined ? {} : { default: def }) });

  const twoArgAction: ActionDefinition = { id: "extrude", label: "Extrude", kind: "mutation", inPalette: true, args: [numberArg("depth", true), numberArg("segments", true)] };
  const zeroArgAction: ActionDefinition = { id: "flatten", label: "Flatten", kind: "mutation", inPalette: true, args: [] };
  const defaultedAction: ActionDefinition = { id: "bevel", label: "Bevel", kind: "mutation", inPalette: true, args: [numberArg("radius", true, 2)] };

  function Harness({ actions, onExecute, disabled }: { actions: readonly ActionDefinition[]; onExecute: (descriptor: unknown) => void; disabled?: boolean }): ReactElement {
    const [expanded, setExpanded] = useState<string | null>(null);
    const [staged, setStaged] = useState<Record<string, Record<string, unknown>>>({});
    return createElement(WindowActionPane, {
      windowId: "w1",
      controllerId: "c",
      actions,
      expandedActionId: expanded,
      stagedArgsByKey: staged,
      disabled: Boolean(disabled),
      onExpandedChange: setExpanded,
      onStageArg: (actionId, argId, value) => setStaged((prev) => ({ ...prev, [actionStageKey("w1", actionId)]: { ...(prev[actionStageKey("w1", actionId)] ?? {}), [argId]: value } })),
      onResetArgs: (actionId) =>
        setStaged((prev) => {
          const next = { ...prev };
          delete next[actionStageKey("w1", actionId)];
          return next;
        }),
      onExecute,
    });
  }

  const buttonByText = (container: HTMLElement, text: string): HTMLButtonElement => {
    const match = [...container.querySelectorAll("button")].find((button) => button.textContent?.includes(text));
    if (!match) throw new Error(`button "${text}" not found`);
    return match as HTMLButtonElement;
  };

  // 🌳️ Action rows render as Tree items (`role="treeitem"`), not `<button>`s — only the Execute/Reset
  // form actions render as real buttons. Row clicks (fire a zero-arg action, toggle an arg-carrying one)
  // go through this helper instead of `buttonByText`.
  const rowByText = (container: HTMLElement, text: string): HTMLElement => {
    const match = [...container.querySelectorAll('[role="treeitem"]')].find((row) => row.querySelector('[data-slot="tree-label"]')?.textContent?.trim() === text);
    if (!match) throw new Error(`tree row "${text}" not found`);
    return match as HTMLElement;
  };

  it("stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Extrude…"));
    const inputs = container.querySelectorAll('input[type="number"]');
    expect(inputs).toHaveLength(2);
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(onExecute).not.toHaveBeenCalled();
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "extrude", args: { depth: 3, segments: 2 } });
    // staged values survive Execute (tweak-and-repeat): the inputs still hold their values
    expect((container.querySelectorAll('input[type="number"]')[0] as HTMLInputElement).value).toBe("3");
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(2);
  });

  it("gates Execute on required args, but a default-satisfied required arg counts without staging", () => {
    const onExecute = vi.fn();
    const required = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(required.container, "Extrude…"));
    expect(buttonByText(required.container, "Execute").disabled).toBe(true);
    const inputs = required.container.querySelectorAll('input[type="number"]');
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    expect(buttonByText(required.container, "Execute").disabled).toBe(true);
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(buttonByText(required.container, "Execute").disabled).toBe(false);
    cleanup();

    const defaulted = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(defaulted.container, "Bevel…"));
    expect(buttonByText(defaulted.container, "Execute").disabled).toBe(false);
    fireEvent.click(buttonByText(defaulted.container, "Execute"));
    expect(onExecute).toHaveBeenLastCalledWith({ controllerId: "c", action: "bevel", args: { radius: 2 } });
  });

  it("Reset restores defaults while keeping the form expanded", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(container, "Bevel…"));
    const input = () => container.querySelector('input[type="number"]') as HTMLInputElement;
    expect(input().value).toBe("2");
    fireEvent.change(input(), { target: { value: "9" } });
    expect(input().value).toBe("9");
    fireEvent.click(buttonByText(container, "Reset"));
    // still expanded (Execute/Reset buttons present) and back to the default effective value
    expect(input().value).toBe("2");
    expect([...container.querySelectorAll("button")].some((b) => b.textContent?.includes("Execute"))).toBe(true);
  });

  it("a zero-arg action row fires immediately with no args object", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "flatten" });
  });

  it("renders every row disabled when an active utility gates actions", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute, disabled: true }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).not.toHaveBeenCalled();
  });

  it("groups actions into category sections like the command panel", () => {
    const createAction: ActionDefinition = { id: "box", label: "Box", kind: "mutation", inPalette: true, category: "create", args: [] };
    const transformAction: ActionDefinition = { id: "move", label: "Move", kind: "mutation", inPalette: true, category: "transform", args: [] };
    const historyAction: ActionDefinition = { id: "undo", label: "Undo", kind: "history", inPalette: true, args: [] };
    const uncategorizedAction: ActionDefinition = { id: "flatten2", label: "Flatten2", kind: "mutation", inPalette: true, args: [] };
    const { container } = render(createElement(Harness, { actions: [createAction, transformAction, historyAction, uncategorizedAction], onExecute: vi.fn() }));
    const textOf = (text: string) => [...container.querySelectorAll("*")].some((el) => el.textContent?.trim() === text && el.children.length === 0);
    expect(textOf("Create")).toBe(true);
    expect(textOf("Transform")).toBe(true);
    expect(textOf("History")).toBe(true);
    expect(textOf("Actions")).toBe(true);
    expect(rowByText(container, "Box")).toBeTruthy();
    expect(rowByText(container, "Move")).toBeTruthy();
    expect(rowByText(container, "Undo")).toBeTruthy();
    expect(rowByText(container, "Flatten2")).toBeTruthy();
  });
});

describe("palette redirect and keybinding rule (P3/P4)", () => {
  const argAction: ActionDefinition = { id: "extrude", label: "Extrude", kind: "mutation", inPalette: true, args: [{ id: "depth", label: "Depth", control: { kind: "number" }, required: true }] };
  const zeroAction: ActionDefinition = { id: "flatten", label: "Flatten", kind: "mutation", inPalette: true, args: [] };

  it("only arg-carrying actions redirect to a staged form (P3 decision)", () => {
    expect(actionRequiresStagedForm(argAction)).toBe(true);
    expect(actionRequiresStagedForm(zeroAction)).toBe(false);
  });

  it("keybinding intent: arg-less fires, arg-action opens unless already expanded and valid then executes (P4)", () => {
    expect(resolveKeybindingIntent(zeroAction, null, {})).toEqual({ kind: "fire" });
    expect(resolveKeybindingIntent(undefined, null, {})).toEqual({ kind: "fire" });
    // not expanded → open
    expect(resolveKeybindingIntent(argAction, null, {})).toEqual({ kind: "open", actionId: "extrude" });
    expect(resolveKeybindingIntent(argAction, "other", { depth: 3 })).toEqual({ kind: "open", actionId: "extrude" });
    // expanded but required arg missing → stays open, never silent-fires
    expect(resolveKeybindingIntent(argAction, "extrude", {})).toEqual({ kind: "open", actionId: "extrude" });
    // expanded and valid → execute with merged effective args
    expect(resolveKeybindingIntent(argAction, "extrude", { depth: 4 })).toEqual({ kind: "execute", actionId: "extrude", args: { depth: 4 } });
  });
});

describe("registry-derived utilities and activation (P5)", () => {
  const utilities: UtilityDefinition[] = [
    { id: "select", label: "Select", iconId: "mouse-pointer", category: "selection", allowsActionsWhileActive: true },
    { id: "brush", label: "Brush", iconId: "brush", group: "paint", category: "utilities", allowsActionsWhileActive: false },
    { id: "erase", label: "Erase", iconId: "eraser", group: "paint", category: "utilities", allowsActionsWhileActive: false },
  ];
  const app = { controllerId: "draw", utilities } satisfies Pick<AppDefinition, "controllerId" | "utilities">;

  it("resolveUtilities scopes to the window kind's refs, falling back to all app utilities when unset", () => {
    expect(resolveUtilities(app, { utilities: ["brush"] } as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["brush"]);
    expect(resolveUtilities(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["select", "brush", "erase"]);
  });

  it("derives grouped utility nodes with the active utility pressed and a setActiveUtility onChange tagged by window", () => {
    const nodes = resolveUtilityNodes(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">, "brush", "w1");
    const select = nodes.find((node) => node.id === "select");
    expect(select && select.kind === "toggle" ? select.pressed : undefined).toBe(false);
    const paint = nodes.find((node) => node.id === "group:paint");
    expect(paint?.kind).toBe("collection");
    const brush = paint && paint.kind === "collection" ? paint.children.find((child) => child.id === "brush") : undefined;
    expect(brush && brush.kind === "toggle" ? brush.pressed : undefined).toBe(true);
    expect(brush && brush.kind === "toggle" && "onChange" in brush ? brush.onChange : undefined).toEqual({ controllerId: "draw", action: "setActiveUtility", args: { utilityId: "brush", windowId: "w1" } });
  });

  it("deriveUtilityNodes twin marks exactly the active utility pressed", () => {
    const nodes = deriveUtilityNodes(
      "draw",
      [
        { id: "a", label: "A", iconId: "x" },
        { id: "b", label: "B", iconId: "y" },
      ],
      "b",
    );
    expect(nodes.map((node) => (node.kind === "toggle" ? node.pressed : undefined))).toEqual([false, true]);
  });

  it("deriveUtilityNodes hoists a single-child group to a top-level toggle", () => {
    const nodes = deriveUtilityNodes(
      "puzzle",
      [
        { id: "transform", label: "Transform", iconId: "move-3d", group: "transform" },
        { id: "brush", label: "Brush", iconId: "brush" },
      ],
      "transform",
    );
    expect(nodes.map((node) => node.id)).toEqual(["transform", "brush"]);
    expect(nodes[0]?.kind).toBe("toggle");
    expect(nodes[0] && nodes[0].kind === "toggle" ? nodes[0].pressed : undefined).toBe(true);
  });

  it("resolveUtilityActivation toggles: click activates, re-click or empty deactivates", () => {
    expect(resolveUtilityActivation(null, "brush")).toBe("brush");
    expect(resolveUtilityActivation("brush", "erase")).toBe("erase");
    expect(resolveUtilityActivation("brush", "brush")).toBeNull();
    expect(resolveUtilityActivation("brush", "")).toBeNull();
    expect(resolveUtilityActivation(undefined, "")).toBeNull();
  });

  it("findPressedUtilityLeafId walks nested collections", () => {
    expect(
      findPressedUtilityLeafId([
        {
          id: "group:transform",
          kind: "collection",
          iconId: "move",
          children: [
            { id: "move", kind: "toggle", iconId: "move", pressed: false, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "move" } } },
            { id: "rotate", kind: "toggle", iconId: "rotate-cw", pressed: true, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "rotate" } } },
          ],
        },
      ]),
    ).toBe("rotate");
    expect(findPressedUtilityLeafId([{ id: "brush", kind: "toggle", iconId: "brush", pressed: false, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "brush" } } }])).toBeUndefined();
  });

  it("isWorldTransformGumballMode requires an explicit move/rotate/scale/transform mode", () => {
    expect(isWorldTransformGumballMode("move")).toBe(true);
    expect(isWorldTransformGumballMode("rotate")).toBe(true);
    expect(isWorldTransformGumballMode("scale")).toBe(true);
    expect(isWorldTransformGumballMode("transform")).toBe(true);
    expect(isWorldTransformGumballMode(undefined)).toBe(false);
    expect(isWorldTransformGumballMode("brush")).toBe(false);
    expect(isWorldTransformGumballMode("")).toBe(false);
  });

  it("worldGumballConfigForProjection intersects transform mode with planar window projections", () => {
    expect(worldGumballConfigForProjection("move", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual({
      moveAxes: true,
      movePlanes: true,
      rotate: false,
      scaleAxes: false,
      scalePlanes: false,
      scaleUniform: false,
      plane: "xy",
    });
    expect(worldGumballConfigForProjection("rotate", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }).plane).toBe("xz");
    expect(worldGumballConfigForProjection("scale", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } }).plane).toBe("yz");
    expect(worldGumballConfigForProjection("move", { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }).plane).toBeUndefined();
    expect(worldGumballConfigForProjection("transform", undefined).plane).toBeUndefined();
  });

  it("gumballTransformDeltaBetweenPoses emits incremental translate/rotate/scale args", () => {
    const base = { mode: "mesh", ids: ["obj-1"] };
    expect(
      gumballTransformDeltaBetweenPoses(
        "move",
        { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        { position: [2, -1, 0.5], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        base,
      ),
    ).toEqual({ action: "translateSelection", args: { ...base, dx: 2, dy: -1, dz: 0.5 } });
    expect(
      gumballTransformDeltaBetweenPoses(
        "move",
        { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        base,
      ),
    ).toBeNull();
    const rotate = gumballTransformDeltaBetweenPoses(
      "rotate",
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] },
      base,
    );
    expect(rotate?.action).toBe("rotateSelection");
    expect(rotate?.args.angle).toBeCloseTo(Math.PI / 2, 5);
    const scale = gumballTransformDeltaBetweenPoses(
      "scale",
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 3, 4] },
      base,
    );
    expect(scale).toEqual({ action: "scaleSelection", args: { ...base, sx: 2, sy: 3, sz: 4 } });
    expect(
      gumballTransformDeltaBetweenPoses(
        "transform",
        { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        { position: [1, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        base,
        "moveX",
      ),
    ).toEqual({ action: "translateSelection", args: { ...base, dx: 1, dy: 0, dz: 0 } });
    const transformRotate = gumballTransformDeltaBetweenPoses(
      "transform",
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] },
      base,
      "rotateY",
    );
    expect(transformRotate?.action).toBe("rotateSelection");
    expect(transformRotate?.args.angle).toBeCloseTo(Math.PI / 2, 5);
  });

  it("gumballLivePreviewDeltaBetweenPoses applies local start→current deltas for instant mid-drag preview", () => {
    expect(
      gumballLivePreviewDeltaBetweenPoses(
        "move",
        { position: [1, 2, 3], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        { position: [4, 2, 5], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      ),
    ).toEqual({ kind: "translate", dx: 3, dy: 0, dz: 2 });
    const rotate = gumballLivePreviewDeltaBetweenPoses(
      "rotate",
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] },
    );
    expect(rotate?.kind).toBe("rotate");
    const scaled = applyGumballLivePreviewDeltaToPose(
      { position: [10, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 2, 2] },
      { kind: "scale", sx: 1.5, sy: 1, sz: 2 },
    );
    expect(scaled).toEqual({ position: [10, 0, 0], quaternion: [0, 0, 0, 1], scale: [3, 2, 4] });
    const translated = applyGumballLivePreviewDeltaToPose(
      { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { kind: "translate", dx: 2, dy: -3, dz: 0.5 },
    );
    expect(translated.position).toEqual([3, -2, 1.5]);
  });

  it("resolveWindowActions surfaces only panel-eligible definitions owned by the window", () => {
    const actionsApp = {
      controllerId: "draw",
      windowKinds: [{ actions: [
        { id: "extrude", label: "Extrude", kind: "mutation", inPalette: true, args: [] },
        { id: "undo", label: "Undo", kind: "history", iconId: "undo", inPalette: true, args: [] },
        { id: "setActiveUtility", label: "Set Active Utility", kind: "view", inPalette: false, args: [] },
      ] as ActionDefinition[] }],
    };
    const resolved = resolveWindowActions(actionsApp, actionsApp.windowKinds[0]!);
    expect(resolved.map((action) => action.id)).toEqual(["extrude"]);
  });

  it("panelTabDefinitionToNode maps the framework-injected History panel tab through its rendered body", () => {
    // 🕰️ id mirrors Rust `FRAMEWORK_PANEL_TAB_HISTORY_ID` — auto-injected into every app's panelTabs
    // by `AppBuilder::build_definition` (see `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`).
    const emptyAppLabelsOverlay = {
      windowKindLabels: {},
      panelTabLabels: {},
      modeLabels: {},
      actionLabels: {},
      utilityLabels: {},
      exampleLabels: {},
      actionArgLabels: {},
      dialogLabels: {},
      introductionLabels: {},
      groupLabels: {},
    };
    const historyTab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: "History", group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const historyUiNode = { type: "tree" as const, sections: [{ id: "framework.history.commands", label: undefined, defaultOpen: true, items: [{ id: "framework.history.entry.1", label: "Increment" }] }] };
    const node = panelTabDefinitionToNode(historyTab, "settings", { "framework.panel.history": historyUiNode }, () => {}, 1, emptyAppLabelsOverlay);
    expect(node.kind).toBe("leaf");
    if (node.kind !== "leaf") return;
    expect(node.id).toBe("framework.panel.history");
    const source = node.trees[0].tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    expect(config.sections[0]?.items?.[0]?.id).toBe("framework.history.entry.1");
  });
});

describe("resolveCommands / commandCategories (footer command panel registry)", () => {
  const command = (id: string, label: string, category: string): CommandDefinition => ({ id, label, category, iconId: "wrench", kind: "shell", inPalette: true, args: [], keybindings: [] });
  const osCommands: CommandDefinition[] = [command("os.setThemeId", "Set Theme", "appearance")];
  const pluginManifest = { pluginId: "fixture", commands: [command("export", "Export", "document")] };
  const app = {
    id: "canvas",
    commands: [command("resetGrid", "Reset Grid", "document")],
    modes: [
      { id: "edit", label: "Edit", commands: [command("focus", "Focus", "view")] },
      { id: "paint", label: "Paint", commands: [command("paintOnly", "Paint Only", "view")] },
    ] as AppModeDefinition[],
  };

  it("aggregates os + program + app-scope + active-mode's mode-scope commands, excluding other modes' mode-scope commands", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "export", "resetGrid", "focus"]);
    expect(resolved.find((entry) => entry.definition.id === "os.setThemeId")?.address).toEqual({ owner: "os", commandId: "os.setThemeId" });
    expect(resolved.find((entry) => entry.definition.id === "resetGrid")?.address).toEqual({ owner: { app: { pluginId: "fixture", appId: "canvas" } }, commandId: "resetGrid" });
    expect(resolved.find((entry) => entry.definition.id === "focus")?.address).toEqual({ owner: { mode: { pluginId: "fixture", appId: "canvas", modeId: "edit" } }, commandId: "focus" });
  });

  it("switching the active mode swaps which mode-scope commands resolve", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "paint");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "export", "resetGrid", "paintOnly"]);
  });

  it("resolves only os commands with no session (null program manifest / app)", () => {
    const resolved = resolveCommands(osCommands, null, null, "");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId"]);
  });

  it("owner-qualifies identical local command ids into collision-free UI keys", () => {
    const duplicateId = "refresh";
    const resolved = resolveCommands(
      [command(duplicateId, "Refresh Shell", "general")],
      { pluginId: "fixture", commands: [command(duplicateId, "Refresh Plugin", "general")] },
      {
        id: "canvas",
        commands: [command(duplicateId, "Refresh App", "general")],
        modes: [{ id: "edit", label: "Edit", commands: [command(duplicateId, "Refresh Mode", "general")] }] as AppModeDefinition[],
      },
      "edit",
    );
    const keys = resolved.map((entry) => commandAddressKey(entry.address));
    expect(new Set(keys).size).toBe(4);
    expect(keys).toEqual(["os:refresh", "plugin:fixture:refresh", "app:fixture:canvas:refresh", "mode:fixture:canvas:edit:refresh"]);
  });

  it("commandCategories orders and dedupes categories by first appearance", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(commandCategories(resolved)).toEqual([
      { id: "appearance", label: "Appearance" },
      { id: "document", label: "Artifact" },
      { id: "view", label: "View" },
    ]);
  });
});

describe("resolveModeTools / buildToolTabs (footer tool panel registry)", () => {
  const toolApp = {
    tools: [
      { id: "fill", label: "Fill", iconId: "fill" },
      { id: "brush", label: "Brush", iconId: "brush" },
    ] as ToolDefinition[],
    modes: [
      { id: "edit", label: "Edit", tools: ["fill", "brush"] },
      { id: "view", label: "View", tools: [] },
    ] as AppModeDefinition[],
  };

  it("resolves the active mode's tools in declared order", () => {
    expect(resolveModeTools(toolApp, "edit").map((tool) => tool.id)).toEqual(["fill", "brush"]);
  });

  it("tools are opt-in per mode — no orphan fallback for a mode that declares none", () => {
    expect(resolveModeTools(toolApp, "view")).toEqual([]);
  });

  it("resolves nothing for an app/mode that doesn't exist", () => {
    expect(resolveModeTools(undefined, "edit")).toEqual([]);
    expect(resolveModeTools(toolApp, "nonexistent")).toEqual([]);
  });

    it("buildToolTabs builds one leaf per resolved tool, whose lazily-resolved tree reflects the current active tool and its measures", () => {
    const activeToolIdRef = { current: "fill" as string | null };
    const toolMeasuresByToolIdRef = { current: { fill: [{ kind: "slider", id: "puzzle3d-fill-count", label: "Count", value: 3, min: 0, max: 100, onChange: { controllerId: "c", action: "setFillCount" } }] } as Readonly<Record<string, readonly WindowMeasure[]>> };
    const onAction = vi.fn();
    const tabs = buildToolTabs(toolApp.tools, "puzzle3d-play", activeToolIdRef, toolMeasuresByToolIdRef, onAction);
    expect(tabs.map((tab) => tab.id)).toEqual(["tool.fill", "tool.brush"]);
    const fillTab = tabs[0] as Extract<PanelTabNode, { kind: "leaf" }>;
    const fillTree = fillTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[]; sortableSections: false } };
    const fillResolved = fillTree.resolveTree();
    expect(fillResolved.sortableSections).toBe(false);
    expect(fillResolved.sections).toHaveLength(1);
    expect(fillResolved.sections[0]!.id).toBe("tool.fill.options");
    expect(fillResolved.sections[0]!.items).toHaveLength(1);
    expect(fillResolved.sections[0]!.items[0]!.label).toBe("Count");
    expect(fillResolved.sections[0]!.items[0]!.control).toBeTruthy();

    const brushTab = tabs[1] as Extract<PanelTabNode, { kind: "leaf" }>;
    const brushTree = brushTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
    // brush is not the active tool — only the flat activation toggle renders (no nested Fill-style label row).
    const brushResolved = brushTree.resolveTree();
    expect(brushResolved.sections).toHaveLength(1);
    expect(brushResolved.sections[0]!.id).toBe("tool.brush.activate");
    expect(brushResolved.sections[0]!.items[0]!.label).toBe("");
  });

  it("buildToolTabs' activation toggle dispatches setActiveTool with this tool's id", () => {
    const activeToolIdRef = { current: null as string | null };
    const toolMeasuresByToolIdRef = { current: {} as Readonly<Record<string, readonly WindowMeasure[]>> };
    const onAction = vi.fn();
    const tabs = buildToolTabs(toolApp.tools, "puzzle3d-play", activeToolIdRef, toolMeasuresByToolIdRef, onAction);
    const fillTab = tabs[0] as Extract<PanelTabNode, { kind: "leaf" }>;
    const fillTree = fillTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
    const activateControl = fillTree.resolveTree().sections[0]!.items[0]!.control as ReactElement<{ onPressedChange: (pressed: boolean) => void }>;
    activateControl.props.onPressedChange(true);
    expect(onAction).toHaveBeenCalledWith({ controllerId: "puzzle3d-play", action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "fill" } });
  });

  it("toolIdFromPanelTabId extracts the mode tool id from a tool leaf tab id", () => {
    expect(toolIdFromPanelTabId("tool.fill")).toBe("fill");
    expect(toolIdFromPanelTabId("tool.brush")).toBe("brush");
    expect(toolIdFromPanelTabId("framework.category.tool")).toBeNull();
    expect(toolIdFromPanelTabId("tool.")).toBeNull();
    expect(toolIdFromPanelTabId(undefined)).toBeNull();
  });
});

describe("Introduce App command", () => {
  it("is available only for apps with an introduction", () => {
    expect(buildOsCommands([], [], true).find((command) => command.id === "os.introduceApp")).toMatchObject({ label: "Introduce App", category: "app", args: [] });
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.introduceApp")).toBe(false);
  });

  it("starts the introduction at its first step", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.introduceApp", undefined, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).toHaveBeenCalledWith({ type: "SET_INTRODUCTION_STEP", value: 0 });
  });
});

describe("Play/Record Tutorial commands", () => {
  it("os.playTutorial appears only when at least one tutorial is declared, offering each as a Select option", () => {
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.playTutorial")).toBe(false);
    const withTutorials = buildOsCommands([], [], false, undefined, undefined, [{ id: "welcome-tour", title: "Welcome Tour" }]);
    const playTutorial = withTutorials.find((command) => command.id === "os.playTutorial");
    expect(playTutorial).toMatchObject({ label: "Play Tutorial", category: "app" });
    expect(playTutorial?.args[0]).toMatchObject({ id: "tutorialId", required: true, control: { kind: "select", options: [{ value: "welcome-tour", label: "Welcome Tour" }] } });
  });

  it("os.recordTutorial appears only when the recorder is available (dev/studio), independent of declared tutorials", () => {
    expect(buildOsCommands([], [], false, undefined, undefined, [], false).some((command) => command.id === "os.recordTutorial")).toBe(false);
    expect(buildOsCommands([], [], false, undefined, undefined, [], true).some((command) => command.id === "os.recordTutorial")).toBe(true);
  });

  it("os-scope Play/Record Tutorial commands are NOT handled by dispatchOsCommand (routed earlier, through the shell's own startTutorialRef/toggleTutorialRecordingRef bridge)", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.playTutorial", { tutorialId: "welcome-tour" }, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    dispatchOsCommand("os.recordTutorial", undefined, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).not.toHaveBeenCalled();
  });
});

describe("shell option locks (SEMIO_LOCKED_*)", () => {
  it("resolves valid locale/appearance and falls back with a warning on invalid values while staying locked", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    expect(resolveShellLocks({ locale: "de" })).toMatchObject({ locale: "de" });
    expect(resolveShellLocks({ locale: "fr" })).toMatchObject({ locale: "en" });
    expect(resolveShellLocks({ appearance: "dark" })).toMatchObject({ appearance: "dark" });
    expect(resolveShellLocks({ appearance: "bogus" })).toMatchObject({ appearance: "system" });
    expect(warn).toHaveBeenCalled();
    warn.mockRestore();
  });

  it("accepts any non-empty terminology id verbatim (app-declared ids can't be validated at boot)", () => {
    expect(resolveShellLocks({ terminology: "reuse" })).toMatchObject({ terminology: "reuse" });
    expect(resolveShellLocks({ terminology: "some-app-declared-id" })).toMatchObject({ terminology: "some-app-declared-id" });
    expect(resolveShellLocks({ terminology: "" })).toEqual({});
  });

  it("returns an empty object for undefined locks", () => {
    expect(resolveShellLocks(undefined)).toEqual({});
  });

  it("initialShellState applies locked values over stored/default prefs", () => {
    const state = initialShellState({ plugins: [], locks: { exampleId: "concrete-forest", locale: "de", terminology: "reuse", themeId: "semio", appearance: "dark" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    expect(state.uiPrefs.uiLocale).toBe("de");
    expect(state.uiPrefs.uiTerminology).toBe("reuse");
    expect(state.uiPrefs.uiThemeId).toBe("semio");
    expect(state.uiPrefs.uiAppearance).toBe("dark");
  });

  it("mergeShellLockSources keeps brand locks locked and lets a defined env lock win per key", () => {
    expect(mergeShellLockSources({ locale: "de", terminology: "reuse" }, { locale: "en", themeId: undefined })).toEqual({ locale: "en", terminology: "reuse" });
    expect(mergeShellLockSources({ locale: "de" }, undefined)).toEqual({ locale: "de" });
    expect(mergeShellLockSources(undefined, { locale: "en" })).toEqual({ locale: "en" });
    expect(mergeShellLockSources(undefined, undefined)).toBeUndefined();
  });

  it("resolveShellDefaults prefers env defaults over brand defaults and initialShellState seeds without locking", () => {
    const brand = { id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", defaults: { exampleId: "concrete-forest" } };
    expect(resolveShellDefaults(brand, { exampleId: "nakagin-capsule-tower" })).toEqual({ exampleId: "nakagin-capsule-tower" });
    expect(resolveShellDefaults(brand, undefined)).toEqual({ exampleId: "concrete-forest" });
    expect(resolveShellDefaults(undefined, undefined)).toEqual({ exampleId: undefined });
    const state = initialShellState({ plugins: [], defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    const locked = initialShellState({ plugins: [], locks: { exampleId: "nakagin-capsule-tower" }, defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(locked.layout.activeExampleId).toBe("nakagin-capsule-tower");
  });

  it("resolveBootExampleId seeds the first registered example when nothing is active or defaulted", () => {
    const options = [
      { id: "hexagonal-mushroom-column" },
      { id: "rectangle-extrude-volume" },
      { id: "sphere-cut-with-torus" },
    ];
    expect(resolveBootExampleId("", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", options, "sphere-cut-with-torus")).toBe("sphere-cut-with-torus");
    expect(resolveBootExampleId("rectangle-extrude-volume", options, "sphere-cut-with-torus")).toBe("rectangle-extrude-volume");
    expect(resolveBootExampleId("missing", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", [])).toBe("");
  });

  it("shouldReplayIntroductionOnLoad opts a brand into replaying its tour after every window refresh", () => {
    expect(shouldReplayIntroductionOnLoad(undefined)).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain", replayIntroductionOnLoad: false })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "plain", windowTitle: "Plain" })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(false);
    expect(ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND.replayIntroductionOnLoad).toBe(true);
  });

  it("isEphemeralShellBrand skips durable shell state so a refresh boots from brand defaults only", () => {
    expect(isEphemeralShellBrand(undefined)).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain", ephemeral: true })).toBe(true);
    expect(isEphemeralShellBrand(ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND)).toBe(true);
    expect(shouldReplayIntroductionOnLoad(ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND)).toBe(true);
    expect(shouldPersistIntroductionSeen(ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND)).toBe(false);
    const ephemeralState = initialShellState({
      plugins: [],
      locks: { locale: "de", terminology: "reuse", themeId: "semio" },
      defaults: { exampleId: "concrete-forest" },
      // 🐚️ An in-memory storage port is now the direct analogue of the old `ephemeral: true` flag —
      // nothing persists, so every unlocked pref reads back its own built-in default, same as before.
      storage: createMemoryStoragePort(),
    });
    expect(ephemeralState.layout.activeExampleId).toBe("concrete-forest");
    expect(ephemeralState.uiPrefs.uiLocale).toBe("de");
    expect(ephemeralState.uiPrefs.uiTerminology).toBe("reuse");
    expect(ephemeralState.uiPrefs.uiThemeId).toBe("semio");
    expect(ephemeralState.uiPrefs.uiAppearance).toBe("system");
    expect(ephemeralState.uiPrefs.uiLayout).toBe("desktop");
    expect(ephemeralState.uiPrefs.uiCustomThemes).toEqual({});
    expect(ephemeralState.layout.dockOverride).toBeNull();
    expect(ephemeralState.layout.shellLayout).toBeNull();
    localStorage.setItem("ui.chrome.appearance", "dark");
    localStorage.setItem("semio.os.dock", "{}");
    localStorage.setItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play", "true");
    clearDurableShellStorage();
    expect(localStorage.getItem("ui.chrome.appearance")).toBeNull();
    expect(localStorage.getItem("semio.os.dock")).toBeNull();
    expect(localStorage.getItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play")).toBeNull();
  });

  it("registers all six Entwerfen mit Bestand demonstrator shell brands", () => {
    expect(ENTWERFEN_MIT_BESTAND_BRAND_IDS).toEqual([
      "entwerfen-mit-bestand-aggregator",
      "entwerfen-mit-bestand-aussuchen",
      "entwerfen-mit-bestand-bearbeiten",
      "entwerfen-mit-bestand-generator",
      "entwerfen-mit-bestand-koordinator",
      "entwerfen-mit-bestand-verfolgen",
    ]);
    expect(ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND.id).toBe("entwerfen-mit-bestand-aggregator");
    expect(ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND.id).toBe("entwerfen-mit-bestand-aussuchen");
    expect(ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND.id).toBe("entwerfen-mit-bestand-bearbeiten");
    expect(ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND.id).toBe("entwerfen-mit-bestand-generator");
    expect(ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND.id).toBe("entwerfen-mit-bestand-koordinator");
    expect(ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND.id).toBe("entwerfen-mit-bestand-verfolgen");
    expect(ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION.steps.map((step) => step.id)).toEqual(["welcome", "prototype", "funding"]);
  });

  it("ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND introduction is app-specific only after the general landing tour was split out", () => {
    const steps = ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND.introduction!.steps;
    expect(steps.map((step) => step.id)).toEqual(["viewport", "panels", "catalogue-objects", "add-object", "transform-utility", "verbindungspunkte", "suggest-objects", "fill-tool", "fill-distribution"]);
    const viewport = steps.find((step) => step.id === "viewport")!;
    expect(viewport.ordered).toBe(false);
    expect(viewport.interactions.map((interaction) => interaction.on)).toEqual([
      { kind: "zoom", id: "puzzle3d-main" },
      { kind: "pan", id: "puzzle3d-main" },
      { kind: "orbit", id: "puzzle3d-main" },
    ]);
    expect(viewport.interactions.map((interaction) => interaction.label)).toEqual([
      "Zoomen (Mausrad)",
      "Verschieben (Mittelklick ziehen)",
      "Orbitieren (Alt + Rechtsklick ziehen)",
    ]);
    expect(viewport.body).toMatch(/Mausrad|Mittelklick|Alt \+ Rechtsklick/i);
    expect(steps.find((step) => step.id === "panels")).toMatchObject({
      introduce: "framework.panel.catalogue",
      interactions: [{ on: { kind: "panel", id: "framework.panel.catalogue" }, label: "Katalog-Reiter anklicken" }],
    });
    expect(steps.find((step) => step.id === "panels")?.body).toMatch(/linken Maustaste|Katalog-Reiter/i);
    expect(steps.find((step) => step.id === "catalogue-objects")).toMatchObject({
      introduce: "puzzle3d-play-kinds.objects",
      placement: "right",
      interactions: [{ on: { kind: "expand", id: "puzzle3d-play-kinds.objects" }, label: "»Baukomponenten« anklicken" }],
      show: ["framework.panelTab.framework.panel.catalogue"],
    });
    expect(steps.find((step) => step.id === "catalogue-objects")?.body).toMatch(/linken Maustaste|Baukomponenten/i);
    expect(steps.find((step) => step.id === "add-object")).toMatchObject({
      introduce: "framework.panelTab.framework.panel.catalogue.firstDraggable",
      placement: "right",
      interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: "Mit linker Maustaste in die Ansicht ziehen" }],
      show: ["framework.panelTab.framework.panel.catalogue", "framework.window.puzzle3dMain"],
    });
    expect(steps.find((step) => step.id === "add-object")?.body).toMatch(/linken Maustaste|Drag-and-Drop/i);
    expect(steps.find((step) => step.id === "transform-utility")).toMatchObject({
      introduce: "transform",
      interactions: [{ on: { kind: "utility", id: "transform" }, label: "Transformieren anklicken" }],
      show: ["framework.window.puzzle3dMain"],
    });
    expect(steps.find((step) => step.id === "transform-utility")?.body).toMatch(/linken Maustaste|Transformieren/i);
    expect(steps.find((step) => step.id === "verbindungspunkte")).toMatchObject({
      introduce: "puzzle3d-play-vortex-show",
      interactions: [{ on: { kind: "action", id: "setVortexShow" }, label: "»Verbindungspunkte anzeigen« auf »Immer« stellen" }],
      show: ["framework.window.puzzle3dMain"],
    });
    expect(steps.find((step) => step.id === "verbindungspunkte")?.body).toMatch(/Linksklick|Verbindungspunkte/i);
    expect(steps.find((step) => step.id === "suggest-objects")).toMatchObject({
      introduce: "framework.window.puzzle3dMain",
      interactions: [{ on: { kind: "action", id: "acceptSuggestion" }, label: "Vorschlag per Linksklick wählen" }],
    });
    expect(steps.find((step) => step.id === "suggest-objects")?.body).toMatch(/Linksklick|Rechtsklick|Aktionsmenü/i);
    expect(steps.find((step) => step.id === "fill-tool")).toMatchObject({
      introduce: "tool.fill",
      interactions: [{ on: { kind: "tool", id: "fill" }, label: "»Füllen« anklicken" }],
      show: [],
      placement: "top",
    });
    expect(steps.find((step) => step.id === "fill-tool")?.body).toMatch(/linken Maustaste|Füllen/i);
    expect(steps.find((step) => step.id === "fill-distribution")).toMatchObject({
      introduce: "puzzle3d-play-distribution",
      interactions: [],
      show: ["puzzle3d-fill-count", "framework.panelTab.tool.fill"],
      placement: "top",
    });
    expect(steps.find((step) => step.id === "fill-distribution")?.body).toMatch(/Schieberegler|Verteilung/i);

    const funding = ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION.steps.find((step) => step.id === "funding")!;
    expect(funding.logos).toHaveLength(3);
    for (const logo of funding.logos!) {
      expect(logo.src).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logo\//);
      expect(logo.darkSrc).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logo\//);
      expect(logo.alt).toBeTruthy();
    }
    const zukunftBauLogo = funding.logos!.find((logo) => logo.href === ZUKUNFT_BAU_PROJECT_URL);
    expect(zukunftBauLogo).toBeDefined();
  });

  it("mit-bestand/demonstrator footer credits render the funding/partner logos, links, and locale text", () => {
    const fundedByMarkup = renderToStaticMarkup(createElement(Footer, { items: [navbarFillItem("fillLeft"), fundedByZukunftBauFooterItem("fundedByEn", "en"), navbarFillItem("fillRight")] }));
    expect(fundedByMarkup).toContain("<button");
    expect(fundedByMarkup).toContain("Funded by");
    expect(fundedByMarkup).toContain("z-40");
    expect(ZUKUNFT_BAU_PROJECT_URL).toMatch(/^https:\/\/www\.zukunftbau\.de\//);
    const fundedByDeMarkup = renderToStaticMarkup(createElement(Footer, { items: [fundedByZukunftBauFooterItem("fundedByDe", "de")] }));
    expect(fundedByDeMarkup).toContain("Gefördert durch");
    const projectOfMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem()] }));
    expect(projectOfMarkup).toContain("Ein Projekt von");
    expect(projectOfMarkup).toContain("und");
    expect(projectOfMarkup).toContain(LUH_LOGO_URL);
    expect(projectOfMarkup).toContain(UDK_LOGO_URL);
    expect(projectOfMarkup).toContain(LUH_URL);
    expect(projectOfMarkup).toContain(UDK_URL);
    expect(projectOfMarkup).toContain("z-40");
    const projectOfEnMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfEn", "en")] }));
    expect(projectOfEnMarkup).toContain("A project of");
    expect(projectOfEnMarkup).toContain("and");
    // 📱️ iconOnly (mobile) drops the surrounding text but keeps both logos and their links.
    const fundedByIconOnlyMarkup = renderToStaticMarkup(createElement(Footer, { items: [fundedByZukunftBauFooterItem("fundedByIconOnly", "en", true)] }));
    expect(fundedByIconOnlyMarkup).not.toContain("Funded by");
    const projectOfIconOnlyMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfIconOnly", "de", true)] }));
    expect(projectOfIconOnlyMarkup).not.toContain("Ein Projekt von");
    expect(projectOfIconOnlyMarkup).not.toContain(">und<");
    expect(projectOfIconOnlyMarkup).toContain(LUH_LOGO_URL);
    expect(projectOfIconOnlyMarkup).toContain(UDK_LOGO_URL);
    expect(LUH_LOGO_URL).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logo\//);
    expect(UDK_LOGO_URL).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logo\//);
  });

  it("buildOsCommands omits only the commands for locked prefs", () => {
    const ids = buildOsCommands([], [], false, { locale: "de", appearance: "dark" }).map((c) => c.id);
    expect(ids).not.toContain("os.setLocale");
    expect(ids).not.toContain("os.setAppearance");
    expect(ids).toContain("os.setTerminology");
    expect(ids).toContain("os.setThemeId");
  });

  it("dispatchOsCommand is a no-operation for a locked pref even if invoked directly", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.setLocale", { locale: "de" }, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never, { locale: "en" });
    expect(dispatch).not.toHaveBeenCalled();
  });
});

describe("buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel)", () => {
  const definition = (id: string, label: string, category: string, args: CommandDefinition["args"] = []): CommandDefinition => ({ id, label, category, args, iconId: "wrench", kind: "shell", keybindings: [], inPalette: true });
  const zeroArgCommand: ResolvedCommand = { definition: definition("os.resetDock", "Reset Dock", "layout"), address: { owner: "os", commandId: "os.resetDock" } };
  const argCommand: ResolvedCommand = {
    definition: definition("os.setThemeId", "Set Theme", "appearance", [{ id: "themeId", label: "Theme", control: { kind: "text" }, required: true }]),
    address: { owner: "os", commandId: "os.setThemeId" },
  };
  const secondArgCommand: ResolvedCommand = {
    definition: definition("os.setAppearance", "Set Appearance", "appearance", [{ id: "appearance", label: "Appearance", control: { kind: "text" }, required: true }]),
    address: { owner: "os", commandId: "os.setAppearance" },
  };
  const singletonArgCommand: ResolvedCommand = {
    definition: definition("os.setDriver", "Set Driver", "general", [{ id: "driver", label: "Driver", control: { kind: "text" }, required: true }]),
    address: { owner: "os", commandId: "os.setDriver" },
  };

  it("a zero-arg command row fires onExecute directly on click; only one command-list section is present when nothing is expanded", () => {
    const onExecute = vi.fn();
    const tree = buildCommandCategoryTree([zeroArgCommand], null, {}, onExecute, vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    const row = tree.sections[0]!.items!.find((item) => item.id === "command.os.os.resetDock")!;
    expect(row.label).toBe("Reset Dock");
    row.onClick?.({} as never, {} as never);
    expect(onExecute).toHaveBeenCalledWith(zeroArgCommand);
  });

  it("auto-expands a singleton arg-carrying category into a flat form with section actions and no disclosure list", () => {
    const tree = buildCommandCategoryTree([singletonArgCommand], null, {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    expect(tree.sections[0]!.id).toBe("command.category.general.form");
    expect(tree.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.os.setDriver.arg.driver"]);
    expect(tree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.os.setDriver-execute", "command-os.os.setDriver-reset"]);
  });

  it("an arg-carrying command row toggles expansion instead of executing, and a synthetic arg-form section only appears while expanded", () => {
    const onToggleExpanded = vi.fn();
    const collapsedTree = buildCommandCategoryTree([argCommand, secondArgCommand], null, {}, vi.fn(), onToggleExpanded, vi.fn(), vi.fn());
    expect(collapsedTree.sections).toHaveLength(1);
    const collapsedRow = collapsedTree.sections[0]!.items!.find((item) => item.id === "command.os.os.setThemeId")!;
    expect(collapsedRow.label).toBe("Set Theme…");
    collapsedRow.onClick?.({} as never, {} as never);
    expect(onToggleExpanded).toHaveBeenCalledWith("os:os.setThemeId");

    const expandedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(expandedTree.sections).toHaveLength(2);
    const formItems = expandedTree.sections[0]!.items!;
    expect(formItems.find((item) => item.id === "command.os.os.setThemeId.arg.themeId")?.label).toBe("Theme");
    expect(expandedTree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.os.setThemeId-execute", "command-os.os.setThemeId-reset"]);
    expect(expandedTree.sections[1]!.items?.map((item) => item.id)).toEqual(["command.os.os.setAppearance"]);
  });

  it("Execute is disabled until the required arg is staged, and calling it passes the effective (staged) args; Reset dispatches onResetArgs", () => {
    const onExecute = vi.fn();
    const onStageArg = vi.fn();
    const onResetArgs = vi.fn();

    const missingTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", {}, onExecute, vi.fn(), onStageArg, onResetArgs);
    const missingExecute = missingTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-execute")!;
    expect(missingExecute.disabled).toBe(true);

    const stagedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", { "os:os.setThemeId": { themeId: "semio" } }, onExecute, vi.fn(), onStageArg, onResetArgs);
    const stagedExecute = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-execute")!;
    const stagedReset = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-reset")!;
    expect(stagedExecute.disabled).toBe(false);
    stagedExecute.onClick();
    expect(onExecute).toHaveBeenCalledWith(argCommand, { themeId: "semio" });
    stagedReset.onClick();
    expect(onResetArgs).toHaveBeenCalledWith("os:os.setThemeId");
  });

  it("buildCommandCategoryTabs builds one namespaced PanelTabLeaf per category, whose lazily-resolved tree only contains that category's commands", () => {
    const categories = [
      { id: "layout", label: "Layout" },
      { id: "appearance", label: "Appearance" },
    ];
    const expandedRef = { current: null as string | null };
    const stagedRef = { current: {} as Readonly<Record<string, Readonly<Record<string, unknown>>>> };
    const onCommand = vi.fn();
    const dispatch = vi.fn();
    const tabs = buildCommandCategoryTabs([zeroArgCommand, argCommand], categories, expandedRef, stagedRef, onCommand, dispatch);
    expect(tabs.map((tab) => tab.id)).toEqual(["command.category.layout", "command.category.appearance"]);
    expect(tabs.every((tab) => tab.kind === "leaf")).toBe(true);

    const layoutLeaf = tabs[0]!;
    expect(layoutLeaf.kind).toBe("leaf");
    const resolved = layoutLeaf.kind === "leaf" ? (layoutLeaf.trees[0]!.tree as { resolveTree: () => { sections: { items?: { id: string }[] }[] } }).resolveTree() : { sections: [] };
    expect(resolved.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.os.resetDock"]);

    // Executing routes through the injected onCommand with the command's own source.
    const executeRow = resolved.sections[0]!.items!.find((item: { id: string }) => item.id === "command.os.os.resetDock") as unknown as { onClick: (event: never, context: never) => void };
    executeRow.onClick({} as never, {} as never);
    expect(onCommand).toHaveBeenCalledWith({ kind: "os" }, "os.resetDock", undefined);
  });
});

describe("host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames)", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("encodes recursive host-effect actions as fully scoped JSON at the runtime boundary", () => {
    const baseSession = {
      pluginId: "demonstrator",
      app: { id: "cad-play", defaultModeId: "edit", modes: [{ id: "edit" }], windowKinds: [{ id: "shape" }] },
      viewState: { activeModeId: "edit", activeWindowKindId: "shape", windowId: "shape-2" },
    } as unknown as Parameters<typeof encodeEffectActionInvocation>[0];
    expect(JSON.parse(encodeEffectActionInvocation(baseSession, "dispatchNext", { jobId: "job-1" }))).toEqual({
      address: {
        pluginId: "demonstrator",
        appId: "cad-play",
        modeId: "edit",
        windowKindId: "shape",
        windowInstanceId: "shape-2",
        actionId: "dispatchNext",
      },
      arguments: { jobId: "job-1", windowId: "shape-2" },
    });
    expect(JSON.parse(encodeEffectCommandInvocation(baseSession, "flowEvalTick"))).toEqual({
      address: { owner: { app: { pluginId: "demonstrator", appId: "cad-play" } }, commandId: "flowEvalTick" },
      arguments: {},
    });
  });

  it("scheduleDispatchAction (D2): fires dispatchOne with action/args only after delayMs elapses", () => {
    vi.useFakeTimers();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction("advanceReconstruction", { jobId: "job-1" }, 250, dispatchOne);
    expect(dispatchOne).not.toHaveBeenCalled();
    vi.advanceTimersByTime(249);
    expect(dispatchOne).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("advanceReconstruction", { jobId: "job-1" });
  });

  it("scheduleDispatchAction (D2): delayMs 0 still defers to a scheduled tick, not a synchronous call", () => {
    vi.useFakeTimers();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction("tick", undefined, 0, dispatchOne);
    expect(dispatchOne).not.toHaveBeenCalled();
    vi.advanceTimersByTime(0);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("tick", undefined);
  });

  it("dispatchOpenedFiles (D3): single-file (multiple=false) makes exactly one call with {payload, name} and no index/total", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    await dispatchOpenedFiles([{ contents: "abc", name: "a.png" }], "importFramePayload", false, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("importFramePayload", { payload: "abc", name: "a.png" });
  });

  it("dispatchOpenedFiles (D3): multiple=true dispatches once per file, in order, each extended with {index, total}", async () => {
    const calls: unknown[][] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: unknown) => {
      calls.push([action, args]);
    });
    const opened = [
      { contents: "a", name: "a.png" },
      { contents: "b", name: "b.png" },
      { contents: "c", name: "c.png" },
    ];
    await dispatchOpenedFiles(opened, "importFramePayload", true, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledTimes(3);
    expect(calls).toEqual([
      ["importFramePayload", { payload: "a", name: "a.png", index: 0, total: 3 }],
      ["importFramePayload", { payload: "b", name: "b.png", index: 1, total: 3 }],
      ["importFramePayload", { payload: "c", name: "c.png", index: 2, total: 3 }],
    ]);
  });

  it("sampleMediaFrameTimestampsMs (D5): steps by sampleStride/fpsHint seconds, capped at maxFrames", () => {
    // Expected timestamps mirror the implementation's own `k * stepMs` computation exactly (bit-for-bit)
    // rather than independently-derived literals — plain float division/multiplication isn't perfectly
    // associative, so e.g. `k * (5 / 30 * 1000)` and `(k * 5000) / 30` can differ by a ULP.
    const stepAt5_30 = (5 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(1000, 5, 0, 30)).toEqual([0, 1, 2, 3, 4, 5].map((k) => k * stepAt5_30));
    expect(sampleMediaFrameTimestampsMs(1000, 5, 2, 30)).toEqual([0, stepAt5_30]);
    // sampleStride 0 floors to 1, fpsHint 0 falls back to 30 — never divides by zero.
    const stepAt1_30 = (1 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(100, 0, 0, 0)).toEqual([0, 1, 2].map((k) => k * stepAt1_30));
    expect(sampleMediaFrameTimestampsMs(0, 5, 10, 30)).toEqual([]);
  });

  //#region 🔌️jsdom media mocks
  /** 🎞️ jsdom has no real media decoder — `<video>`'s `duration`/`videoWidth`/`videoHeight` are
   * read-only getters that never change from a `src` assignment, and `currentTime` is a no-operation setter
   * that never fires `seeked`. This stubs both to the minimum needed for `runTier2VideoFrames`'s
   * seek-and-capture loop: `currentTime` synchronously (via microtask) fires `seeked`, mirroring how a
   * real browser resolves a seek asynchronously without needing fake timers in these tests. */
  function mockVideoElement(durationMs: number, width: number, height: number): HTMLVideoElement {
    const video = document.createElement("video");
    Object.defineProperty(video, "duration", { value: durationMs / 1000, configurable: true });
    Object.defineProperty(video, "videoWidth", { value: width, configurable: true });
    Object.defineProperty(video, "videoHeight", { value: height, configurable: true });
    Object.defineProperty(video, "readyState", { value: 1, configurable: true });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        queueMicrotask(() => video.dispatchEvent(new Event("seeked")));
      },
    });
    return video;
  }

  function mockCanvasCapture(): void {
    HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({ drawImage: vi.fn() }) as unknown as typeof HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.toDataURL = vi.fn().mockReturnValue("data:image/jpeg;base64,ZmFrZQ==");
  }
  //#endregion 🔌️jsdom media mocks

  it("runTier2VideoFrames (D5): dispatches frameAction once per sampled timestamp, in order, then doneAction exactly once", async () => {
    mockCanvasCapture();
    const video = mockVideoElement(200, 64, 48);
    const calls: { action: string; args: Record<string, unknown> }[] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: Record<string, unknown>) => {
      calls.push({ action, args });
    });
    await runTier2VideoFrames(video, { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 5, maxFrames: 0, maxLongEdgePx: 0, fpsHint: 30, args: { streamId: "s1" } }, "clip.mp4", dispatchOne);
    const frameCalls = calls.filter((call) => call.action === "frame");
    const doneCalls = calls.filter((call) => call.action === "done");
    expect(frameCalls.length).toBeGreaterThan(0);
    expect(doneCalls).toHaveLength(1);
    // frame/done ordering: every frame dispatch precedes the single done dispatch.
    expect(calls.at(-1)!.action).toBe("done");
    expect(frameCalls.map((call) => call.args.index)).toEqual(frameCalls.map((_, index) => index));
    expect(frameCalls[0]!.args).toMatchObject({ payload: "data:image/jpeg;base64,ZmFrZQ==", name: "clip.mp4", streamId: "s1" });
    expect(doneCalls[0]!.args).toMatchObject({ name: "clip.mp4", frameCount: frameCalls.length, sampledCount: frameCalls.length, streamId: "s1" });
  });

  it("runRequestMediaFrames (D5): Tier 2 failure (video element throws mid-seek) ⇒ dispatches fallbackAction exactly once with raw bytes as a data URL, no frame/done calls", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const throwingVideo = mockVideoElement(1000, 16, 16);
    Object.defineProperty(throwingVideo, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        throw new Error("decode failed");
      },
    });
    await runRequestMediaFrames(
      { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 },
      "video/mp4",
      payload,
      dispatchOne,
      () => throwingVideo,
    );
    expect(dispatchOne).toHaveBeenCalledTimes(1);
    const [action, args] = dispatchOne.mock.calls[0]! as [string, Record<string, unknown>];
    expect(action).toBe("fallback");
    expect(args.name).toBe("video");
    expect(String(args.payload)).toMatch(/^data:video\/mp4;base64,/);
  });

  it("runRequestMediaFrames (D5): payload bytes in hand ⇒ Tier 2 seek-capture runs, ending in doneAction (no picker needed)", async () => {
    mockCanvasCapture();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    await runRequestMediaFrames(
      { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 },
      "video/mp4",
      payload,
      dispatchOne,
      () => mockVideoElement(1000, 16, 16),
    );
    const actions = dispatchOne.mock.calls.map((call) => call[0] as string);
    expect(actions.at(-1)).toBe("done");
    expect(actions.filter((action) => action === "frame").length).toBeGreaterThan(0);
  });
});

describe("Display Windows tab — projection drag templates", () => {
  function windowsTreeSections(windowKinds: DisplayHostApi["windowKinds"]) {
    const host: DisplayHostApi = {
      windowKinds,
      namedLayouts: [],
      userLayouts: [],
      saveCurrentLayout: () => {},
      applyNamedLayout: () => {},
      deleteUserLayout: () => {},
      layoutSaveLabel: "",
      setLayoutSaveLabel: () => {},
    };
    const tabs = createFrameworkDisplayPanelTabs(() => host);
    type WindowsTreeSections = { readonly sections: readonly { readonly id: string; readonly items?: readonly unknown[] }[] };
    type LeafWithTrees = { readonly trees: readonly { readonly tree: { readonly resolveTree: () => WindowsTreeSections } }[] };
    const windowsTab = tabs.find((tab) => tab.id === "framework.display.windows") as unknown as LeafWithTrees;
    return windowsTab.trees[0]!.tree.resolveTree().sections;
  }

  type LabeledTreeItem = { readonly id: string; readonly label?: string; readonly icon?: unknown; readonly items?: readonly LabeledTreeItem[]; readonly dragData?: Record<string, string> };
  const byLabel = (items: readonly LabeledTreeItem[], label: string) => items.find((row) => row.label === label);

  it("shows window kind icons on section headers and kind rows", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", iconId: "layout-grid", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.icon).toBeTruthy();
    const items = sections[0]!.items as LabeledTreeItem[];
    expect(items[0]?.icon).toBeTruthy();
  });

  it("nests the full Parallel/Perspective projection taxonomy under a world-3d window kind", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    expect(sections).toHaveLength(1);
    const items = sections[0]!.items as LabeledTreeItem[];
    expect(items).toHaveLength(3);
    expect(items.some((row) => row.id === "framework.display.windows.puzzle3d-main.kind")).toBe(true);
    const parallel = byLabel(items, "Parallel")!;
    const perspective = byLabel(items, "Perspective")!;
    expect(perspective).toBeDefined();
    expect(parallel.items!.map((row) => row.id.split(".").pop()).sort()).toEqual(["axonometric", "oblique", "orthographic"]);
  });

  it("keeps a flat single drag entry for non-world-3d window kinds", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", iconId: "layout-grid", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.items).toHaveLength(1);
  });

  it("pre-reverses every level so the bottom-anchored (direction=\"up\") Tree's own sibling-reversal renders Parallel children top-to-bottom", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as LabeledTreeItem[];
    const parallel = byLabel(items, "Parallel")!;
    // Raw (pre-render) order is reversed once more so that after the Tree's own "up" reversal on render,
    // Parallel reads Orthographic, Axonometric, Oblique — top to bottom.
    expect([...parallel.items!].reverse().map((row) => row.label)).toEqual(["Orthographic", "Axonometric", "Oblique"]);
    const axonometric = byLabel(parallel.items!, "Axonometric")!;
    expect([...axonometric.items!].reverse().map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
  });

  it("each projection leaf's drag payload decodes back to its WorldProjectionSpec", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as LabeledTreeItem[];
    const parallel = byLabel(items, "Parallel")!;
    const orthographic = byLabel(parallel.items!, "Orthographic")!;
    const payload = JSON.parse(orthographic.dragData!["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(payload.windowKindId).toBe("puzzle3d-main");
    expect(decodeWorldProjectionTemplateId(payload.templateId)).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } });
    const axonometric = byLabel(parallel.items!, "Axonometric")!;
    const isometric = byLabel(axonometric.items!, "Isometric")!;
    const isoPayload = JSON.parse(isometric.dragData!["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(decodeWorldProjectionTemplateId(isoPayload.templateId)).toMatchObject({ mode: { kind: "axonometric", variant: "isometric" } });
  });
});

describe("createFrameworkSettingsPanelTab", () => {
  it("exposes one Settings toggle whose children are General, Theme, and Hotkeys tabs", () => {
    const settings = createFrameworkSettingsPanelTab(() => null);
    expect(settings.kind).toBe("branch");
    if (settings.kind !== "branch") throw new Error("Settings must be a branch");
    expect(settings.id).toBe("framework.settings");
    expect(settings.children.map((child) => child.id)).toEqual(["framework.settings.general", "framework.settings.theme", "framework.settings.keybindings"]);
    expect(settings.children.map((child) => child.name)).toEqual(["General", "Theme", "Hotkeys"]);
  });
});

describe("createFrameworkMarketplacePanelTab", () => {
  type LabeledTreeItem = { readonly id: string; readonly label?: string; readonly loading?: boolean; readonly items?: readonly LabeledTreeItem[]; readonly control?: ReactElement };
  type MarketplaceTreeSections = { readonly sections: readonly { readonly id: string; readonly label?: string; readonly items?: readonly LabeledTreeItem[] }[] };
  type LeafWithTrees = { readonly trees: readonly { readonly tree: { readonly resolveTree: () => MarketplaceTreeSections } }[] };

  function marketplaceTreeSections(host: MarketplaceHostApi | null) {
    const marketplaceTab = createFrameworkMarketplacePanelTab(() => host) as unknown as LeafWithTrees;
    return marketplaceTab.trees[0]!.tree.resolveTree().sections;
  }

  it("shows an unavailable placeholder when no host is mounted yet", () => {
    const sections = marketplaceTreeSections(null);
    expect(sections).toHaveLength(1);
    expect(sections[0]!.items?.[0]?.id).toBe("unavailable");
  });

  const plugin = (pluginId: string, status: PluginPanelStatus, canUninstall: boolean): MarketplacePluginEntry => ({ pluginId, label: pluginId, version: "1", status, sourceId: "dev", canUninstall });
  const extension = (extensionId: string, extendsHost: string, enabled = true): MarketplaceExtensionEntry => ({ extensionId, label: extensionId, version: "1", extendsHost, enabled, status: "loaded" });
  const host = (overrides: Partial<MarketplaceHostApi> = {}): MarketplaceHostApi => ({
    plugins: [],
    extensions: [],
    installPlugin: () => {},
    uninstallPlugin: () => {},
    reloadPlugin: () => {},
    installExtensionFromUrl: () => {},
    installExtensionFromFile: () => {},
    uninstallExtension: () => {},
    setExtensionEnabled: () => {},
    ...overrides,
  });

  it("groups plugins into one section per source, sorted by pluginId within a source", () => {
    const sections = marketplaceTreeSections(host({ plugins: [plugin("s", "loaded", false), plugin("note", "loaded", true)] }));
    expect(sections.map((section) => section.id)).toEqual(["framework.marketplace.extensions.install", "framework.marketplace.source.dev"]);
    expect(sections[1]!.items?.map((item) => item.id)).toEqual(["framework.marketplace.plugin.note", "framework.marketplace.plugin.s"]);
  });

  it("integrates extensions as children of their owning plugin", () => {
    const sections = marketplaceTreeSections(
      host({
        plugins: [plugin("flow", "loaded", true), plugin("s", "loaded", false)],
        extensions: [extension("flow.brep", "flow"), extension("flow.math", "flow", false)],
      }),
    );
    const flow = sections[1]!.items?.find((item) => item.id === "framework.marketplace.plugin.flow");
    expect(flow?.items?.map((item) => item.id)).toEqual(["framework.marketplace.plugin.flow.extension.flow.brep", "framework.marketplace.plugin.flow.extension.flow.math"]);
    expect(sections.some((section) => section.id.includes("extensions.host"))).toBe(false);
  });

  it("marks installing/reloading rows as loading, and every status is reflected in the row label", () => {
    const items = marketplaceTreeSections(
      host({ plugins: [plugin("a", "available", true), plugin("b", "installing", true), plugin("c", "loaded", true), plugin("d", "failed", true), plugin("e", "reloading", true)] }),
    )[1]!.items!;
    const byId = (pluginId: string) => items.find((item) => item.id === `framework.marketplace.plugin.${pluginId}`)!;
    expect(byId("a").loading).toBe(false);
    expect(byId("b").loading).toBe(true);
    expect(byId("c").loading).toBe(false);
    expect(byId("d").loading).toBe(false);
    expect(byId("e").loading).toBe(true);
    expect(byId("a").label).toContain("Available");
    expect(byId("b").label).toContain("Installing");
    expect(byId("c").label).toContain("Loaded");
    expect(byId("d").label).toContain("Failed");
    expect(byId("e").label).toContain("Reloading");
  });

  it("routes install/uninstall/reload clicks for one row back through the host without touching others", () => {
    const calls: string[] = [];
    const sections = marketplaceTreeSections(
      host({
        plugins: [plugin("note", "loaded", true)],
        uninstallPlugin: (pluginId) => calls.push(`uninstall:${pluginId}`),
        reloadPlugin: (pluginId) => calls.push(`reload:${pluginId}`),
      }),
    );
    const noteItem = sections[1]!.items![0]!;
    const { getByText } = render(createElement("div", null, noteItem.control));
    fireEvent.click(getByText("Reload"));
    fireEvent.click(getByText("Uninstall"));
    expect(calls).toEqual(["reload:note", "uninstall:note"]);
    cleanup();
  });

  it("disables uninstall for the host/primary plugin and the active session's plugin (canUninstall: false)", () => {
    const sections = marketplaceTreeSections(host({ plugins: [plugin("s", "loaded", false)] }));
    const sItem = sections[1]!.items![0]!;
    const { getByText } = render(createElement("div", null, sItem.control));
    expect((getByText("Uninstall").closest("button") as HTMLButtonElement).disabled).toBe(true);
    cleanup();
  });
});

describe("introductionTargetsWindow", () => {
  it("matches both the window kind and every open instance of that kind", () => {
    expect(introductionTargetsWindow("puzzle3d-main", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main-top", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main-perspective", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("other-window", "other-window", "puzzle3d-main")).toBe(false);
  });

  it("matches action-rail segments against the kind and its instances", () => {
    expect(introductionTargetsWindow("puzzle3d-main-top", "puzzle3d-main", null, "puzzle3dMain")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main", "puzzle3d-main", null, "puzzle3dMain")).toBe(true);
    expect(introductionTargetsWindow("other-window", "other-window", null, "puzzle3dMain")).toBe(false);
  });
});

describe("windowMeasureTreeContainsId", () => {
  it("finds nested measure ids used as introduction targets", () => {
    const measures = [
      { kind: "select" as const, id: "puzzle3d-play-vortex-show", value: "selected", items: [], onChange: { id: "setVortexShow" } },
      {
        kind: "group" as const,
        id: "group",
        label: "Group",
        children: [{ kind: "toggle" as const, id: "nested-toggle", pressed: false, iconId: "eye", onChange: { id: "noOperation" } }],
      },
    ];
    expect(windowMeasureTreeContainsId(measures, "puzzle3d-play-vortex-show")).toBe(true);
    expect(windowMeasureTreeContainsId(measures, "nested-toggle")).toBe(true);
    expect(windowMeasureTreeContainsId(measures, "missing")).toBe(false);
  });
});

describe("renderWindowMeasuresTree", () => {
  it("puts toggle icons before labels and uses checkboxes instead of icon toggles", () => {
    const measures: WindowMeasure[] = [
      {
        kind: "toggle",
        id: "grid-visible",
        iconId: "layout-grid",
        label: "Grid",
        pressed: true,
        onChange: { controllerId: "x", action: "setGridVisible" },
      },
    ];
    const markup = renderToStaticMarkup(renderWindowMeasuresTree(measures, () => undefined) as ReactElement);
    const iconIdx = markup.indexOf('data-slot="tree-icon"');
    const labelIdx = markup.indexOf('data-slot="tree-label"');
    const checkboxIdx = markup.indexOf('data-slot="tree-action-checkbox"');
    expect(iconIdx).toBeGreaterThan(-1);
    expect(labelIdx).toBeGreaterThan(iconIdx);
    expect(checkboxIdx).toBeGreaterThan(labelIdx);
    expect(markup).toContain('type="checkbox"');
    expect(markup).toContain('id="grid-visible"');
    expect(markup).toContain('data-icon="layout-grid"');
    expect(markup).toContain("Grid");
    expect(markup).toContain("checked");
    expect(markup).not.toContain('data-state="on"');
  });
});

describe("resolveFrameworkLayoutSeed — multi-pane default layouts", () => {
  const emptyLabels = {
    windowKindLabels: {},
    panelTabLabels: {},
    modeLabels: {},
    actionLabels: {},
    utilityLabels: {},
    exampleLabels: {},
    actionArgLabels: {},
    dialogLabels: {},
    introductionLabels: {},
    groupLabels: {},
  };

  it("does not infer focus when an app has no explicit layout", () => {
    const seed = resolveFrameworkLayoutSeed(undefined, [{ id: "main", label: "Main" }], emptyLabels);
    expect(seed.modeLayout).toEqual({ kind: "stack", children: [{ kind: "window", id: "main" }] });
    expect(seed).not.toHaveProperty("activeWindowId");
  });

  it("hydrates Top (1/3) + Perspective (2/3) instances and projection templates", () => {
    const topTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } });
    const perspectiveTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } });
    const seed = resolveFrameworkLayoutSeed(
      {
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 100 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Top", instanceId: "puzzle3d-main-top", templateId: topTemplate }],
            },
            {
              kind: "stack",
              size: 200 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Perspective", instanceId: "puzzle3d-main-perspective", templateId: perspectiveTemplate }],
            },
          ],
        },
      },
      [{ id: "puzzle3d-main", label: "Puzzle 3D" }],
      emptyLabels,
    );
    expect(seed.modeLayout).toEqual({
      kind: "row",
      children: [
        { kind: "stack", size: 100 / 3, children: [{ kind: "window", id: "puzzle3d-main-top", title: "Top" }] },
        { kind: "stack", size: 200 / 3, children: [{ kind: "window", id: "puzzle3d-main-perspective", title: "Perspective" }] },
      ],
    });
    expect(seed.extraInstances).toEqual([
      { id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" },
      { id: "puzzle3d-main-perspective", windowKindId: "puzzle3d-main", title: "Perspective" },
    ]);
    expect(seed).not.toHaveProperty("activeWindowId");
    expect(seed.pendingProjections).toEqual([
      { windowId: "puzzle3d-main-top", templateId: topTemplate },
      { windowId: "puzzle3d-main-perspective", templateId: perspectiveTemplate },
    ]);
  });

  it("treats instance-id panes as extras so the host fetches bodies keyed by instance id, not only by kind", () => {
    const topTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } });
    const perspectiveTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } });
    const seed = resolveFrameworkLayoutSeed(
      {
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 100 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Top", instanceId: "puzzle3d-main-top", templateId: topTemplate }],
            },
            {
              kind: "stack",
              size: 200 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Perspective", instanceId: "puzzle3d-main-perspective", templateId: perspectiveTemplate }],
            },
          ],
        },
      },
      [{ id: "puzzle3d-main", label: "Puzzle 3D" }],
      emptyLabels,
    );
    // 🪟️ A refresh that only knows the bare kind id would leave Top/Perspective as "Fehlendes Fenster".
    // Live extras must be in the fetch list: base kind + each default-layout instance.
    const windowInstances = [
      { id: "puzzle3d-main", bodyKey: "puzzle3d.play.composite" },
      ...seed.extraInstances.map((entry) => ({ id: entry.id, bodyKey: "puzzle3d.play.composite" })),
    ];
    const request = buildUiRefreshRequest({ kind: "full" }, windowInstances, [], {}, new Map());
    expect(request?.windows?.map((window) => window.key)).toEqual(["puzzle3d-main", "puzzle3d-main-top", "puzzle3d-main-perspective"]);
  });

  it("re-derives window titles from localized windowKind labels on locale/terminology switch", () => {
    const windowKinds = [
      {
        id: "main",
        label: {
          native: { en: "Main Window", de: "Hauptfenster" },
          reuse: { en: "Main Component", de: "Hauptkomponente" },
        },
      },
    ];
    const layout = {
      kind: "stack" as const,
      children: [{ kind: "window" as const, id: "main", title: uiDataLabel("Main Window") }],
    };

    const retitledDeNative = retitleWindowLayoutNode(layout, windowKinds, [], "native", "de");
    expect(retitledDeNative).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "main", title: "Hauptfenster" }],
    });

    const retitledDeReuse = retitleWindowLayoutNode(layout, windowKinds, [], "reuse", "de");
    expect(retitledDeReuse).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "main", title: "Hauptkomponente" }],
    });
  });

  it("re-derives titles for extra window instances based on their windowKindId", () => {
    const windowKinds = [
      {
        id: "puzzle3d-main",
        label: {
          native: { en: "3D Editor", de: "3D-Editor" },
          reuse: { en: "3D Component", de: "3D-Komponente" },
        },
      },
    ];
    const extraInstances = [
      { id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "3D Editor" },
    ];
    const layout = {
      kind: "stack" as const,
      children: [{ kind: "window" as const, id: "puzzle3d-main-top", title: uiDataLabel("3D Editor") }],
    };

    const retitled = retitleWindowLayoutNode(layout, windowKinds, extraInstances, "reuse", "de");
    expect(retitled).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "puzzle3d-main-top", title: "3D-Komponente" }],
    });
  });
});

describe("classifyWindowLayoutChange", () => {
  const twoStackRow = (leftSize: number, rightSize: number) => ({
    kind: "row" as const,
    size: 100,
    children: [
      { kind: "stack" as const, size: leftSize, children: [{ kind: "window" as const, id: "a" }] },
      { kind: "stack" as const, size: rightSize, children: [{ kind: "window" as const, id: "b" }] },
    ],
  });

  it("returns null when the layout is identical (deep-equal, not just same reference)", () => {
    expect(classifyWindowLayoutChange(twoStackRow(50, 50), twoStackRow(50, 50))).toBeNull();
    const same = twoStackRow(50, 50);
    expect(classifyWindowLayoutChange(same, same)).toBeNull();
  });

  it("returns null for a pure active-window-flag change (skeleton and sizes both unchanged)", () => {
    const previous = { kind: "stack" as const, size: 100, activeId: "a", children: [{ kind: "window" as const, id: "a" }, { kind: "window" as const, id: "b" }] };
    const next = { ...previous, activeId: "b" };
    expect(classifyWindowLayoutChange(previous, next)).toBeNull();
  });

  it("returns 'resize' when only pane sizes differ", () => {
    expect(classifyWindowLayoutChange(twoStackRow(50, 50), twoStackRow(30, 70))).toBe("resize");
  });

  it("returns 'rearrange' when window ids/nesting structure differ (drag-to-new-position, split, close)", () => {
    const previous = twoStackRow(50, 50);
    const swapped = { ...previous, children: [previous.children[1]!, previous.children[0]!] };
    expect(classifyWindowLayoutChange(previous, swapped)).toBe("rearrange");
    const closed = { kind: "stack" as const, children: [{ kind: "window" as const, id: "a" }] };
    expect(classifyWindowLayoutChange(previous, closed)).toBe("rearrange");
    expect(classifyWindowLayoutChange(null, previous)).toBe("rearrange");
    expect(classifyWindowLayoutChange(previous, null)).toBe("rearrange");
  });

  it("returns null when both are null", () => {
    expect(classifyWindowLayoutChange(null, null)).toBeNull();
  });
});

describe("noteShellCommand", () => {
  it("buildNoteShellCommandAction builds a noteShellCommand action descriptor targeting the given controller, carrying detail only when provided", () => {
    expect(buildNoteShellCommandAction("puzzle3d-play", "shell.windowClose", "Close Window", { windowId: "w1" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "shell.windowClose", label: "Close Window", detail: { windowId: "w1" } },
    });
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.resetDock", "Reset Panels")).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.resetDock", label: "Reset Panels" },
    });
  });

  it("is excluded from tutorial recording, alongside world-navigation/introduction/tutorial-control action ids", () => {
    expect(TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has("noteShellCommand")).toBe(true);
  });
});

describe("TutorialRecorder LocalizedLabel synthesis", () => {
  it("synthesizeLocalizedLabel broadcasts a string across all 4 cells (native/reuse x en/de)", () => {
    const label = synthesizeLocalizedLabel("Test Chapter");
    expect(label).toEqual({
      native: { en: "Test Chapter", de: "Test Chapter" },
      reuse: { en: "Test Chapter", de: "Test Chapter" },
    });
    expect(synthesizeLocalizedLabel(label)).toBe(label);
  });

  it("TutorialRecorder synthesizes LocalizedLabel for addChapter and build titles", () => {
    const recorder = new TutorialRecorder({ activeUtilityByWindowId: {}, activePanelTabByGroup: {}, expandedTreeIds: [], commandPanelOpen: false }, null);
    recorder.addChapter("Introduction");
    recorder.addChapter();
    const def = recorder.build("rec-1", "Recorded Tutorial");

    expect(def.title).toEqual({
      native: { en: "Recorded Tutorial", de: "Recorded Tutorial" },
      reuse: { en: "Recorded Tutorial", de: "Recorded Tutorial" },
    });
    expect(def.chapters[0].title).toEqual({
      native: { en: "Introduction", de: "Introduction" },
      reuse: { en: "Introduction", de: "Introduction" },
    });
    expect(def.chapters[1].title).toEqual({
      native: { en: "Chapter 2", de: "Chapter 2" },
      reuse: { en: "Chapter 2", de: "Chapter 2" },
    });

    expect(resolveManifestLabel(def.title, "native", "en")).toBe("Recorded Tutorial");
    expect(resolveManifestLabel(def.chapters[0].title, "reuse", "de")).toBe("Introduction");
  });

  it("FrameworkOsShell portal layer is unconstrained by z-tutorial so portaled elements sit above elevated windows", () => {
    if (!window.matchMedia) {
      window.matchMedia = (() => ({ matches: false, media: "", onchange: null, addListener: () => {}, removeListener: () => {}, addEventListener: () => {}, removeEventListener: () => {}, dispatchEvent: () => false })) as unknown as typeof window.matchMedia;
    }
    const { container } = render(
      createElement(FrameworkOsShell, { plugins: [], appId: "test" })
    );
    const portalLayer = container.querySelector("[data-semio-portal-layer]");
    expect(portalLayer).toBeTruthy();
    expect(portalLayer?.className).not.toContain("z-tutorial");
  });
});
