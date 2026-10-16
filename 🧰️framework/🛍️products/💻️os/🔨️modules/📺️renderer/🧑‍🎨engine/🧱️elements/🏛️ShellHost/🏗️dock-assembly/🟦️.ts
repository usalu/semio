/** 🏗️ Default dock arrangement, dock persistence, tab and tree-unit drops, introduction overrides and the panel reveal effects.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useDockAssembly` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { useCallback, useEffect, useMemo, useRef } from "react";
// #region 🔌️Adapters
import {
  type DockUiPanelState,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_TOOL_RUN_ID,
  type IntroductionInteraction,
  SET_ACTIVE_TOOL_ACTION_ID
} from "@semio-tech/framework";
import { spawnedIdOfWindowInstanceV1 } from "../🪟️spawned-program/🟦️.ts";
import {
  type Anchor,
  ANCHORS,
  applyDockSkeleton,
  dockSkeletonOf,
  dockSkeletonsEqual,
  findPanelTabInDock,
  findPanelTabPath,
  moveTabInDock,
  moveTreeUnitInDock,
  type PanelDock,
  panelTabChildren,
  type PanelTabDockMove,
  type PanelTabNode,
  type PanelTreeUnitDockMove,
  reconcileActivePath,
  singleTreeLeaf
} from "@semio-tech/ui-react";
import { leftoverWorldSelectionOverlayV1 } from "../../🌐️World3dHost/🟦️.tsx";
import {
  DEFAULT_PANEL_WIDTH_PX,
  FRAMEWORK_CATEGORY_COMMAND_ID,
  FRAMEWORK_CATEGORY_DISPLAY_ID,
  FRAMEWORK_CATEGORY_TOOL_ID,
  categoryTabIcon,
  flattenPanelTabLeaves,
  shellLabelTextV1,
  programArmedToolRevealV1,
  reconcileToolTabSelection,
  resolveUtilityNodes,
  shellLabel,
  shellTabIcon,
  toolIdFromPanelTabId,
  utilityNodeTreeContainsId,
  windowMeasureTreeContainsId,
  type ToolTabSelection
} from "../../🛠️ShellHelpers/🟦️.tsx";
// #endregion 🔌️Adapters
import { toolRunPanelReveal } from "../../🛠️ShellHelpers/⏯️tool-run-panel/🟦️.ts";
import { useTimeTravelRevealV1 } from "../../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { FRAMEWORK_SETTINGS_PANEL_ID } from "../../📌️ChromePanels/🟦️.tsx";
import { leftoverInspectionRefreshScope, leftoverInspectionPanelHash } from "../../🔌️PluginRuntime/🟦️.tsx";
import { shellAppFault, type ShellAppFrameV1 } from "@semio-tech/framework/mcp/channel";
import type { ShellCommand } from "../../../../../🖥️shell/🟦️.ts";
import type { ShellCoreAfterCommandBar } from "../🔎️command-bar/🟦️.tsx";

export type ShellCoreAfterDockAssembly = ShellCoreAfterCommandBar & ReturnType<typeof useDockAssembly>;

export function useDockAssembly(core: ShellCoreAfterCommandBar) {
  const {
    scope, hostMode, mobile, dispatch, session, uiLocale, uiTerminology, leftoverInspectionEpoch,
    leftoverInspectionIdsRef, refreshHistorySnapshot, hostAppId, hostCatalogueTabId, windowMeasuresByWindowId,
    appLabelsOverlay, activeToolId, panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId,
    mobilePanelPath, extraWindowInstances, introductionStepIndex, uiRefreshCacheRef, sessionRef,
    applyAgentShellCommandRef, agentArtifactRouteRef, agentDeferredPlansRef, loadedPluginsRef, reportRefreshFault,
    dockLayoutStore, dockUiStateStore, panel, spawnedAppsRef, spawnedIdsRef, focusedProgram, toolRunPanelNode,
    focusedToolMeasuresByToolId, focusedTimeTravel, activeIntroduction, activeToolIdRef, toolGeneration,
    activeWindowIdRef, completeIntroductionInteraction, refreshUi, noteShellCommand, onActionRef, onActionStable,
    frameworkDisplayTabs, frameworkMarketplaceTab, frameworkChatTab, frameworkTaskManagerTab, workbenchLeftTabs,
    detailsRightTabs, displayBottomLeftTabs, settingsBottomRightDockTab, frameworkHistoryTab, frameworkSyncTab,
    exampleSelectElement, modeSwitcherElement, roleSwitcherElement, commandCategoryTabs, toolTabs
  } = core;
  //#region FrameworkOsShell
  //#region 🧭️DockAssembly — default four-corner arrangement (the two middle anchors start empty save the command palette in bottom-middle) + persisted-override reconciliation + drag-and-drop wiring.
  const defaultDock = useMemo((): PanelDock => {
    // 🧭️ Top-left (Workbench: Document/Catalogue) and top-right (Details: Inspection/Parameters) stay flat.
    // Bottom-right exposes one Settings branch whose children are internal tabs, plus one Marketplace leaf.
    const topLeft: PanelTabNode[] = [...workbenchLeftTabs];
    const bottomLeft: PanelTabNode[] = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    bottomLeft.push(...displayBottomLeftTabs);
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight: PanelTabNode[] = [...detailsRightTabs, frameworkChatTab];
    // 🧭️ App Settings-group tabs nest inside the framework Settings branch (document-first child order)
    // so the anchor carries one Settings toggle; Marketplace stays a sibling leaf beside that branch.
    // 🧵️ Task manager sits beside Marketplace on the same anchor: both are shell-owned windows about
    // what the runtime is doing, not about the open document.
    const bottomRight: PanelTabNode[] = [settingsBottomRightDockTab, frameworkMarketplaceTab, frameworkTaskManagerTab];
    if (frameworkHistoryTab) bottomRight.push(frameworkHistoryTab);
    // 🛠️ Tool categories stay nested under one expandable Tool branch, exactly like Command categories,
    // placed left of Command (order 0 vs 1) — like commands not being window-level, tools are not
    // window-level either; both live only on this shared mode-scoped anchor.
    // 🎛️ Command categories stay nested under one expandable Command branch so the folded bottom-middle
    // chrome shows a single Command toggle, not every category leaf inlined along the footer.
    const bottomMiddle: PanelTabNode[] = [
      ...(toolTabs.length > 0 ? [{ kind: "branch" as const, id: FRAMEWORK_CATEGORY_TOOL_ID, icon: categoryTabIcon(toolTabs, "hammer"), name: shellLabel("ui.panelToggle.tool"), order: 0, children: toolTabs }] : []),
      ...(commandCategoryTabs.length > 0 ? [{ kind: "branch" as const, id: FRAMEWORK_CATEGORY_COMMAND_ID, icon: categoryTabIcon(commandCategoryTabs, "wrench"), name: shellLabel("ui.panelToggle.command"), order: 1, children: commandCategoryTabs }] : []),
    ];
    return {
      anchors: {
        "top-left": topLeft,
        "top-middle": [],
        "top-right": topRight,
        "right-middle": [],
        "bottom-right": bottomRight,
        "bottom-middle": bottomMiddle,
        "bottom-left": bottomLeft,
        "left-middle": [],
      },
    };
  }, [commandCategoryTabs, detailsRightTabs, displayBottomLeftTabs, frameworkChatTab, frameworkDisplayTabs, frameworkMarketplaceTab, frameworkSyncTab, frameworkTaskManagerTab, frameworkHistoryTab, settingsBottomRightDockTab, toolTabs, uiLocale, workbenchLeftTabs]);

  useEffect(() => {
    dispatch({ type: "SET_DOCK_OVERRIDE", value: dockLayoutStore.getSnapshot() });
  }, [dockLayoutStore]);

  const dock = useMemo((): PanelDock => applyDockSkeleton(defaultDock, dockOverride), [defaultDock, dockOverride]);

  // 📱️ All eight anchors' tabs flattened into the single mobile panel's tab list — defined here (ahead of the
  // dock-assembly override effects below) so those effects can resolve a mobile-panel path alongside the
  // desktop per-anchor one.
  const mobilePanelTabs = useMemo(() => {
    const anchorTabs = ANCHORS.flatMap((anchor) => defaultDock.anchors[anchor]);
    // 📱️ The example selector and mode switcher have no navbar room on mobile (see `navbarItems`) — they
    // surface as one more tab in the merged mobile panel instead, sharing the exact same elements the
    // desktop navbar center cluster renders.
    if (!exampleSelectElement && !modeSwitcherElement && !roleSwitcherElement) return anchorTabs;
    const appTab = singleTreeLeaf({
      id: "framework.mobile.app",
      icon: shellTabIcon("smartphone"),
      name: shellLabel("ui.mobilePanel.app"),
      order: 99,
      tree: {
        sections: [
          {
            id: "framework.mobile.app.root",
            label: "",
            items: [
              ...(exampleSelectElement ? [{ id: "framework.mobile.app.example", label: "", control: exampleSelectElement }] : []),
              ...(modeSwitcherElement ? [{ id: "framework.mobile.app.modes", label: "", control: modeSwitcherElement }] : []),
              ...(roleSwitcherElement ? [{ id: "framework.mobile.app.roles", label: "", control: roleSwitcherElement }] : []),
            ],
          },
        ],
      },
    });
    return [...anchorTabs, appTab];
  }, [defaultDock, exampleSelectElement, modeSwitcherElement, roleSwitcherElement]);

  /** 🗄️ Skips the very first (pre-hydration) commit so a persisted skeleton isn't clobbered with `null` before the seeding effect above has a chance to read and apply it. */
  const dockPersistedOnceRef = useRef(false);
  useEffect(() => {
    if (!dockPersistedOnceRef.current) {
      dockPersistedOnceRef.current = true;
      return;
    }
    const nextSkeleton = dockSkeletonOf(dock);
    const defaultSkeleton = dockSkeletonOf(defaultDock);
    dockLayoutStore.save(dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton);
  }, [dock, defaultDock, dockLayoutStore]);

  useEffect(() => {
    dispatch({ type: "HYDRATE_DOCK_UI", value: dockUiStateStore.getSnapshot() });
  }, [dockUiStateStore]);

  /** 🗄️ Same first-commit-skip as the dock skeleton effect above, but also re-arms when the store identity itself changes (app switch) — otherwise the new app's pre-hydration state would be written into its own key on the first post-switch commit. */
  const dockUiPersistedOnceRef = useRef(false);
  const dockUiPersistedStoreRef = useRef(dockUiStateStore);
  useEffect(() => {
    if (dockUiPersistedStoreRef.current !== dockUiStateStore) {
      dockUiPersistedStoreRef.current = dockUiStateStore;
      dockUiPersistedOnceRef.current = false;
    }
    if (!dockUiPersistedOnceRef.current) {
      dockUiPersistedOnceRef.current = true;
      return;
    }
    const anchors: Partial<Record<Anchor, DockUiPanelState>> = {};
    for (const anchor of ANCHORS) {
      const panelState = panels[anchor];
      const entry: DockUiPanelState = {};
      if (panelState.visible) entry.visible = true;
      if (panelState.size !== DEFAULT_PANEL_WIDTH_PX) entry.size = panelState.size;
      if (panelState.path.length > 0) entry.path = panelState.path;
      if (Object.keys(entry).length > 0) anchors[anchor] = entry;
    }
    const hasPathMemory = Object.keys(panelPathMemory).length > 0;
    const hasTreeOpen = Object.keys(treeOpenStates).length > 0;
    const isDefault = Object.keys(anchors).length === 0 && !hasPathMemory && !hasTreeOpen;
    dockUiStateStore.save(isDefault ? null : { version: 3, anchors, pathMemory: hasPathMemory ? panelPathMemory : undefined, treeOpen: hasTreeOpen ? treeOpenStates : undefined });
  }, [panels, panelPathMemory, treeOpenStates, dockUiStateStore]);

  const handleTabDockDrop = useCallback(
    (move: PanelTabDockMove) => {
      const nextDock = moveTabInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      const targetPath = findPanelTabPath(nextDock.anchors[move.target.anchor], move.tabId);
      if (targetPath) dispatch({ type: "SET_PANEL_PATH", anchor: move.target.anchor, value: targetPath });
      if (move.fromAnchor !== move.target.anchor) {
        const sourceTabs = nextDock.anchors[move.fromAnchor];
        dispatch({ type: "SET_PANEL_PATH", anchor: move.fromAnchor, value: (prev) => reconcileActivePath(sourceTabs, prev, panelTabChildren) });
      }
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
      noteShellCommand("shell.dockMove", shellLabelTextV1("ui.shellCommand.dockMove"), { tabId: move.tabId, fromAnchor: move.fromAnchor, toAnchor: move.target.anchor });
    },
    [dock, defaultDock, noteShellCommand],
  );

  const handleTreeUnitDockDrop = useCallback(
    (move: PanelTreeUnitDockMove) => {
      const nextDock = moveTreeUnitInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
      noteShellCommand("shell.dockMove", shellLabelTextV1("ui.shellCommand.dockMove"), { toAnchor: move.target.anchor });
    },
    [dock, defaultDock, noteShellCommand],
  );

  const hostOverrideTabId = hostMode && session?.app.id === hostAppId ? (panel?.activePanelTab ?? hostCatalogueTabId) : undefined;
  const studioOverrideAnchor = hostOverrideTabId ? findPanelTabInDock(dock, hostOverrideTabId)?.anchor : undefined;
  const detailsOverrideTabId = panel?.activePanelTab;
  const detailsOverrideAnchor = detailsOverrideTabId ? findPanelTabInDock(dock, detailsOverrideTabId)?.anchor : undefined;

  /** 🎓️ The current introduction step's target element ids (`introduce` + `show`), classified by
   * shape — `null` unless that shape is present, so every reveal override below (here and in
   * `modeWindows`) is a plain truthiness check. A folded utility bar/Actions rail/dock panel would
   * otherwise hide the target from ever mounting (see `useIntroductionAnchorRect`), leaving the step
   * centered with no cutout and no way for the user to find what to do. Ids are matched, never
   * reconstructed: a `framework.window.{segment}` id's segment is `elementIdSegment(windowId)`, a lossy
   * camelCase normalization — comparing `elementIdSegment(windowId) === segment` OR the same for the
   * instance's window-kind id is the only safe check (Top/Perspective instances share a kind). */
  const activeIntroductionStep = activeIntroduction && introductionStepIndex != null ? (activeIntroduction.steps[introductionStepIndex] ?? null) : null;
  const introductionElementIds = useMemo(
    (): readonly string[] => (activeIntroductionStep ? [activeIntroductionStep.introduce, ...activeIntroductionStep.show].filter((id): id is string => Boolean(id)) : []),
    [activeIntroductionStep],
  );
  const introductionUtilityId = useMemo(() => {
    if (!session) return null;
    const utilities = session.app.utilities ?? [];
    return introductionElementIds.find((id) => utilities.some((utility) => utility.id === id)) ?? null;
  }, [introductionElementIds, session]);
  const introductionActionWindowSegment = useMemo(() => {
    for (const id of introductionElementIds) {
      const rest = id.startsWith("framework.window.") ? id.slice("framework.window.".length) : null;
      const actionIndex = rest?.indexOf(".action.") ?? -1;
      if (rest && actionIndex >= 0) return rest.slice(0, actionIndex);
    }
    return null;
  }, [introductionElementIds]);
  const introductionPanelTabId = useMemo(() => {
    for (const id of introductionElementIds) {
      if (id.startsWith("framework.panelTab.")) {
        const rest = id.slice("framework.panelTab.".length);
        return rest.endsWith(".firstDraggable") ? rest.slice(0, -".firstDraggable".length) : rest;
      }
    }
    return null;
  }, [introductionElementIds]);
  /** 🛠️ Tool ids the active step asks the user to activate (`interactions` of kind `tool`, or a bare
   * `tool.<id>` introduce/show). Reveals the Tool category chrome so the leaf tab can be pressed —
   * never drills into the leaf itself (that would open the inactive activate-toggle tree and, via tab
   * selection, auto-activate + celebrate before the user acts). */
  const introductionToolPickIds = useMemo((): readonly string[] => {
    const fromInteractions = (activeIntroductionStep?.interactions ?? [])
      .filter((interaction): interaction is IntroductionInteraction & { readonly on: { readonly kind: "tool"; readonly id: string } } => interaction.on.kind === "tool")
      .map((interaction) => interaction.on.id);
    if (fromInteractions.length > 0) return fromInteractions;
    return introductionElementIds.flatMap((id) => {
      const match = /^tool\.([a-z][a-zA-Z0-9]*)$/.exec(id);
      return match?.[1] ? [match[1]] : [];
    });
  }, [activeIntroductionStep, introductionElementIds]);
  const introductionPanelTabAnchor = introductionPanelTabId ? findPanelTabInDock(dock, introductionPanelTabId)?.anchor : undefined;
  const introductionUtilityWindowId = useMemo(() => {
    if (!introductionUtilityId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const utilities = resolveUtilityNodes(session.app, kind, null, kind.id, appLabelsOverlay, uiTerminology, uiLocale);
      if (utilityNodeTreeContainsId(utilities, introductionUtilityId)) return kind.id;
    }
    return null;
  }, [appLabelsOverlay, introductionUtilityId, session, uiTerminology, uiLocale]);
  /** 🎓️ Window-kind id whose measures tree owns an introduce/show measure id — force-unfolds the Window
   * Options rail so targets like `puzzle3d-play-vortex-show` can mount for the tour. */
  const introductionMeasureWindowId = useMemo(() => {
    if (!session || introductionElementIds.length === 0) return null;
    for (const kind of session.app.windowKinds) {
      const kindMeasures = kind.options.measures ?? [];
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(kindMeasures, id))) return kind.id;
      for (const [windowId, measures] of Object.entries(windowMeasuresByWindowId)) {
        if (!introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) continue;
        if (windowId === kind.id || extraWindowInstances.some((instance) => instance.id === windowId && instance.windowKindId === kind.id)) return kind.id;
      }
    }
    return null;
  }, [extraWindowInstances, introductionElementIds, session, windowMeasuresByWindowId]);

  /** 🛠️ Tool id whose measure tree owns an introduce/show id — keeps mode-level tools like fill
   * active so targets such as `puzzle3d-play-distribution` stay mounted for the tour. */
  const introductionToolId = useMemo(() => {
    if (introductionElementIds.length === 0) return null;
    for (const [toolId, measures] of Object.entries(focusedToolMeasuresByToolId)) {
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) return toolId;
    }
    return null;
  }, [introductionElementIds, focusedToolMeasuresByToolId]);

  const lastIntroductionToolIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!introductionToolId || !session) {
      lastIntroductionToolIdRef.current = null;
      return;
    }
    if (lastIntroductionToolIdRef.current === introductionToolId && activeToolIdRef.current === introductionToolId) return;
    lastIntroductionToolIdRef.current = introductionToolId;
    if (activeToolIdRef.current === introductionToolId) return;
    onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: introductionToolId } });
  }, [introductionToolId, onActionStable, session]);

  /** 🛠️ Tool-pick steps (e.g. Füllen): open the Tool category so `tool.<id>` leaf tabs mount in the
   * panel chrome, clear any already-active tool so the user must activate it, and never select the
   * leaf path (selecting auto-activates and would celebrate before they act). */
  const lastIntroductionToolPickStepIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!session || introductionToolPickIds.length === 0 || !activeIntroductionStep) {
      lastIntroductionToolPickStepIdRef.current = null;
      return;
    }
    // 🛠️ Measure-driven keep-alive (`introductionToolId`) owns activation for steps that introduce
    // tool measures (fill-distribution) — don't fight it by clearing the tool.
    if (introductionToolId) return;
    if (lastIntroductionToolPickStepIdRef.current === activeIntroductionStep.id) return;
    lastIntroductionToolPickStepIdRef.current = activeIntroductionStep.id;
    for (const toolId of introductionToolPickIds) {
      if (activeToolIdRef.current === toolId) {
        onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "" } });
      }
    }
    if (mobile) {
      const resolved = findPanelTabPath(mobilePanelTabs, FRAMEWORK_CATEGORY_TOOL_ID);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
      return;
    }
    const toolAnchor = findPanelTabInDock(dock, FRAMEWORK_CATEGORY_TOOL_ID)?.anchor ?? "bottom-middle";
    const resolved = findPanelTabPath(dock.anchors[toolAnchor], FRAMEWORK_CATEGORY_TOOL_ID);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: toolAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: toolAnchor, value: true });
  }, [activeIntroductionStep, dock, introductionToolId, introductionToolPickIds, mobile, mobilePanelTabs, onActionStable, session]);

  const lastIntroductionPanelTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!introductionPanelTabId || !introductionPanelTabAnchor) {
      lastIntroductionPanelTabIdRef.current = undefined;
      return;
    }
    if (lastIntroductionPanelTabIdRef.current === introductionPanelTabId) return;
    lastIntroductionPanelTabIdRef.current = introductionPanelTabId;
    if (mobile) {
      const resolved = findPanelTabPath(mobilePanelTabs, introductionPanelTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
      return;
    }
    const resolved = findPanelTabPath(dock.anchors[introductionPanelTabAnchor], introductionPanelTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: introductionPanelTabAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: introductionPanelTabAnchor, value: true });
  }, [introductionPanelTabId, introductionPanelTabAnchor, dock, mobile, mobilePanelTabs]);

  /** 🎓️ Panel interactions complete when their named panel tab is open and visible — checked for every
   * `panel` interaction of the active step, not just the first, so a step can require opening several. */
  useEffect(() => {
    if (!activeIntroductionStep) return;
    for (const interaction of activeIntroductionStep.interactions ?? []) {
      if (interaction.on.kind !== "panel") continue;
      const tabId = interaction.on.id;
      const located = findPanelTabInDock(dock, tabId);
      if (!located) continue;
      const panel = panels[located.anchor];
      if (!panel.visible || !panel.path.includes(tabId)) continue;
      completeIntroductionInteraction((candidate) => candidate.on.kind === "panel" && candidate.on.id === tabId);
    }
  }, [activeIntroductionStep, completeIntroductionInteraction, dock, panels]);

  /** 🎓️ Expand interactions start with every named tree section forced closed on step entry, then
   * complete individually as the user opens each one. */
  const lastIntroductionExpandStepIdRef = useRef<string | null>(null);
  useEffect(() => {
    const expandInteractions = (activeIntroductionStep?.interactions ?? []).filter((interaction) => interaction.on.kind === "expand");
    if (!activeIntroductionStep || expandInteractions.length === 0) {
      lastIntroductionExpandStepIdRef.current = null;
      return;
    }
    if (lastIntroductionExpandStepIdRef.current !== activeIntroductionStep.id) {
      lastIntroductionExpandStepIdRef.current = activeIntroductionStep.id;
      for (const interaction of expandInteractions) {
        const stateSuffix = `tree-section-${interaction.on.id}`;
        const catalogueKey = `${FRAMEWORK_PANEL_TAB_CATALOGUE_ID}.tree:${stateSuffix}`;
        dispatch({ type: "SET_TREE_OPEN_STATE", id: catalogueKey, open: false });
      }
      return;
    }
    for (const interaction of expandInteractions) {
      const sectionId = interaction.on.id;
      const stateSuffix = `tree-section-${sectionId}`;
      const expanded = Object.entries(treeOpenStates).some(([key, open]) => open && key.endsWith(stateSuffix));
      if (expanded) completeIntroductionInteraction((candidate) => candidate.on.kind === "expand" && candidate.on.id === sectionId);
    }
  }, [activeIntroductionStep, completeIntroductionInteraction, treeOpenStates]);

  /** 🧭️ Progressive reveal means a stored path can legitimately end at a branch (or be empty) — this is now a plain per-anchor truncation-validate, no override reassertion (see the write-through effects below). */
  const panelActivePaths = useMemo((): Record<Anchor, readonly string[]> => {
    const result = {} as Record<Anchor, readonly string[]>;
    for (const anchor of ANCHORS) result[anchor] = reconcileActivePath(dock.anchors[anchor], panels[anchor].path, panelTabChildren);
    return result;
  }, [panels, dock]);

  const historyTabOpenRef = useRef(false);
  useEffect(() => {
    if (!session) {
      historyTabOpenRef.current = false;
      return;
    }
    const historyOpen = Object.values(panelActivePaths).some((path) => path.includes(FRAMEWORK_PANEL_TAB_HISTORY_ID));
    const opened = historyOpen && !historyTabOpenRef.current;
    historyTabOpenRef.current = historyOpen;
    if (!opened) return;
    // 🧾️ The FOCUSED program's history, because that is whose ledger the panel renders.
    refreshHistorySnapshot(focusedProgram ?? { pluginId: session.pluginId, instanceId: session.instanceId });
  }, [focusedProgram, panelActivePaths, refreshHistorySnapshot, session]);

  /**
   * 🧭️ Generalizes the old `leftPanelActivePath`/`rightPanelActivePath` studio/plugin "snap to the active panel
   * tab" overrides across all eight anchors. Write-through rather than read-time: each override dispatches
   * `SET_PANEL_PATH` only when its target tab id actually changes, so a user's own collapse/navigation
   * afterward sticks instead of being reasserted on every render (progressive reveal made read-time reassertion
   * fight the user's own collapses). Studio wins over details when both would touch the same anchor.
   **/
  const lastStudioOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!hostOverrideTabId || !studioOverrideAnchor) {
      lastStudioOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastStudioOverrideTabIdRef.current === hostOverrideTabId) return;
    lastStudioOverrideTabIdRef.current = hostOverrideTabId;
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
      const resolved = findPanelTabPath(mobilePanelTabs, hostOverrideTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      return;
    }
    if (panels[studioOverrideAnchor].path[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
    const resolved = findPanelTabPath(dock.anchors[studioOverrideAnchor], hostOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: studioOverrideAnchor, value: resolved });
  }, [hostOverrideTabId, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);

  const toolRunPanelRunsRef = useRef<ReadonlySet<bigint>>(new Set());
  useEffect(() => {
    const { runs, added } = toolRunPanelReveal(toolRunPanelRunsRef.current, toolRunPanelNode);
    toolRunPanelRunsRef.current = runs;
    if (added.length === 0) return;
    const located = findPanelTabInDock(dock, FRAMEWORK_PANEL_TAB_TOOL_RUN_ID);
    if (!located) return;
    const resolved = findPanelTabPath(dock.anchors[located.anchor], FRAMEWORK_PANEL_TAB_TOOL_RUN_ID);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
  }, [toolRunPanelNode, dock]);

  /** ⏪️ Each change of the focused program's session reveals the History panel on the edge into a session and moves focus
   * where the session continues ({@link useTimeTravelRevealV1}). */
  useTimeTravelRevealV1(focusedTimeTravel, { mobile, dock, mobilePanelTabs, dispatch, root: scope.rootRef.current });

  /** 🤖️ Fills {@link applyAgentShellCommandRef}: the MCP gateway's `ui_reveal`/`ui_focus` reach this
   * host as SSOT `ShellCommand`s, and this is where they stop being a mirror update and become the
   * real chrome moving. `setPanelPath` carries the shell SSOT's four-anchor vocabulary while this
   * dock has six, so the TAB is the address — `findPanelTabInDock` answers where it actually lives,
   * exactly as the tool-run reveal above does, and the requested anchor is advisory. */
  useEffect(() => {
    applyAgentShellCommandRef.current = (command: ShellCommand): void => {
      if (command.type === "focusWindow") {
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: command.windowId });
        return;
      }
      if (command.type !== "setPanelPath") return;
      const tabId = command.path[command.path.length - 1];
      if (!tabId) return;
      const located = findPanelTabInDock(dock, tabId);
      if (!located) return;
      const resolved = findPanelTabPath(dock.anchors[located.anchor], tabId);
      if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
    };
  }, [dock]);

  /** 🗿️ Fills {@link agentArtifactRouteRef}: this is the LIVE artifact route — an MCP client's
   * `action_prepare`/`action_invoke`/`history_undo`/`transaction_*` executing against the plugin
   * instance the human is looking at, instead of against the gateway's own headless interpreter on
   * a different `--folder` workspace (`📓️r2…` §12 finding 1).
   *
   * Three laws make it the human's edit and not a side channel:
   * 1. **One dispatch lane.** A mutation leaves through `onActionRef.current`, the shell's ONE input
   *    funnel — the same call the palette, the keybindings and the app's own buttons make. It mints
   *    an input-ledger entry, obeys the active-window rules and settles with a typed outcome, so an
   *    agent can never bypass a gate the UI respects.
   * 2. **The agent is its own actor.** `provenance.origin = "agent"` rides along, so the edit is
   *    attributable in the human's history rather than looking like something they did.
   * 3. **The commit is where the mutation happens.** The gateway's protocol is
   *    `ReadHistory → PureCommand → TransactionPrepare → TransactionCommit`; the shell has no
   *    "compute the ops but do not apply them" seam, so `pureCommand` returns a DEFERRED plan and
   *    `transactionCommit` is the one leg that dispatches. A rollback therefore really does leave
   *    the document untouched, which is the property the two-phase shape exists to give. */
  useEffect(() => {
    const deferred = agentDeferredPlansRef.current;
    agentArtifactRouteRef.current = async (request): Promise<readonly ShellAppFrameV1[]> => {
      const session = sessionRef.current;
      if (!session) return [shellAppFault("plugin.unavailable", "no plugin session is established in this shell — open an artifact here before driving it from an agent")];
      /** 🗿️🪟️ The program this request addresses: the session, or any SPAWNED program of it.
       *
       * 🧯️ This route bound `sessionRef.current` and refused every other instance by name — the LAST
       * of the session-only decision sites S5 started on (five), S6 continued (the agent census) and
       * §3 finished (progress). With it, an MCP client could reach only the landing app of the shell
       * whose whole purpose is hosting every other plugin's artifacts: measured on the live gate
       * against `s` with a spawned `note` editor as
       * `this shell's live instance is \`1\`, not \`3\` — the agent addressed an instance this shell
       * has closed`, where instance 3 was exactly the editor the human was looking at. */
      const spawnedTarget = spawnedAppsRef.current.find((entry) => String(entry.instanceId) === request.instanceId);
      const spawnedTargetApp = spawnedTarget ? loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawnedTarget.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawnedTarget.appId) : undefined;
      const live =
        request.instanceId === String(session.instanceId)
          ? { pluginId: session.pluginId, instanceId: session.instanceId, app: session.app }
          : spawnedTarget && spawnedTargetApp
            ? { pluginId: spawnedTarget.pluginId, instanceId: spawnedTarget.instanceId, app: spawnedTargetApp }
            : null;
      if (!live) {
        const open = [`${session.pluginId}:${session.app.id}:${session.instanceId}`, ...spawnedAppsRef.current.map((entry) => `${entry.pluginId}:${entry.appId}:${entry.instanceId}`)].join(", ");
        return [shellAppFault("plugin.unavailable", `this shell has no open instance \`${request.instanceId}\` — the agent addressed an instance this shell has closed (open: ${open})`)];
      }
      const readHistory = async (): Promise<ShellAppFrameV1> => {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
        if (!plugin?.readHistory) return shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no history port in this shell`);
        const patch = await plugin.readHistory(live.instanceId);
        const head = [...(patch?.upserts ?? [])].sort((left, right) => right.seq - left.seq)[0];
        return { kind: "historySnapshot", artifactId: `${live.pluginId}:${live.app.id}:${live.instanceId}`, headEditId: head?.actionId ?? "", cursor: String(patch?.cursor ?? 0) };
      };
      const dispatchVerb = async (action: string, args: Record<string, unknown>): Promise<ShellAppFrameV1 | null> => {
        const outcome = await onActionRef.current({ controllerId: live.app.controllerId, action, args, provenance: { origin: "agent", windowId: activeWindowIdRef.current ?? null, causedBy: null } });
        if (outcome.kind === "refused") return shellAppFault(outcome.reason === "viewer-read-only" ? "viewer.read-only" : outcome.reason === "mutation-rejected" ? "app.command.rejected" : "channel.not-wired", `the shell refused \`${action}\`: ${outcome.reason}${outcome.detail ? ` (${outcome.detail})` : ""}`);
        return null;
      };
      switch (request.command.kind) {
        case "readHistory":
          return [await readHistory()];
        case "pureCommand": {
          const verb = request.command.capabilityId.split(".").pop() ?? "";
          if (!(live.app.actions ?? []).some((declared) => declared.id === verb)) {
            return [shellAppFault("capability.not-found", `app \`${live.app.id}\` declares no action \`${verb}\` (from capability \`${request.command.capabilityId}\`)`)];
          }
          const plan = new TextEncoder().encode(JSON.stringify({ deferredAction: verb, args: request.command.input ?? {} }));
          return [{ kind: "emit", ops: { document: [plan], config: [], draft: [] }, warnings: [] }];
        }
        case "transactionPrepare": {
          const lane = request.command.ops.document[0];
          if (!lane) return [shellAppFault("app.command.rejected", "the shell route prepares only its own deferred plan; this transaction carries no document-lane ops")];
          deferred.set(request.command.txnId, new TextDecoder().decode(lane));
          return [{ kind: "transactionPrepared", txnId: request.command.txnId }];
        }
        case "transactionRollback":
          deferred.delete(request.command.txnId);
          return [{ kind: "transactionRolledBack", txnId: request.command.txnId }];
        case "transactionCommit": {
          const plan = deferred.get(request.command.txnId);
          deferred.delete(request.command.txnId);
          if (plan === undefined) return [shellAppFault("transaction.generation-mismatch", `this shell holds no prepared plan for transaction \`${request.command.txnId}\``)];
          let parsed: { readonly deferredAction?: string; readonly args?: Record<string, unknown> };
          try {
            parsed = JSON.parse(plan) as { readonly deferredAction?: string; readonly args?: Record<string, unknown> };
          } catch {
            return [shellAppFault("app.command.rejected", "the prepared plan is not this shell route's own")];
          }
          if (!parsed.deferredAction) return [shellAppFault("app.command.rejected", "the prepared plan names no action")];
          const refusal = await dispatchVerb(parsed.deferredAction, parsed.args ?? {});
          if (refusal) return [refusal];
          const after = await readHistory();
          return [{ kind: "transactionCommitted", txnId: request.command.txnId, editId: after.kind === "historySnapshot" ? after.headEditId : "" }];
        }
        case "transactionUndo":
        case "transactionRedo": {
          const verb = request.command.kind === "transactionUndo" ? "undo" : "redo";
          const refusal = await dispatchVerb(verb, {});
          if (refusal) return [refusal];
          return request.command.kind === "transactionUndo" ? [{ kind: "transactionUndone", groupId: request.command.groupId }] : [{ kind: "transactionRedone", groupId: request.command.groupId }];
        }
        case "cancel":
          return [shellAppFault("budget.exceeded", `the gateway abandoned command ${request.command.cancelSeq}`)];
        // 🚧️ Honest gaps, named rather than faked: a genesis read and a media export both need the
        // live store's own pack bytes, which this host does not expose to the bridge yet. An agent
        // that needs either resolves a headless context (`--folder`/`--hub`) for it.
        // 🗿️ The genesis/snapshot read, answered from the LIVE store: `readAppDocumentPack` is the
        // handle's own document port (the one `🏪️store`'s persistence already round-trips through),
        // so an agent's `artifact_create`/`artifact_open` reads the very bytes the human is editing.
        // It is optional on the handle and may answer `null` for a plugin with no document — both
        // refuse BY NAME rather than going silent, the contract every other arm here keeps.
        case "readArtifact": {
          const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
          if (!plugin?.readAppDocumentPack) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no document port in this shell — an agent needing a genesis read must resolve a headless context (\`--folder\`/\`--hub\`)`)];
          const document = await plugin.readAppDocumentPack(live.instanceId);
          if (!document) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` instance ${live.instanceId} holds no document to read`)];
          return [{ kind: "artifact", pack: document.pack, spr: document.spr }];
        }
        // 📤️ The media OUT port, answered from the LIVE guest: `exportAppMedia` drives
        // `AppCommand::MediaOut` on the handle's own channel — the SAME command the headless
        // gateway sends — so the agent gets the guest's own bytes back over the bridge rather than
        // the human getting an unasked-for download off the segmented queue. Both refusal paths
        // (no port on the handle, no `Media` frame in the reply) name themselves, the contract
        // every other arm here keeps.
        case "exportMedia": {
          const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
          if (!plugin?.exportAppMedia) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no media OUT port in this shell — an agent needing an export must resolve a headless context (\`--folder\`/\`--hub\`)`)];
          const exported = await plugin.exportAppMedia(live.instanceId, request.command.port);
          if (!exported) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` produced no media on port \`${request.command.port}\``)];
          return [{ kind: "exported", port: exported.port, descriptor: exported.descriptor, data: exported.data }];
        }
      }
    };
  }, []);

  useEffect(() => {
    const leftoverIds = leftoverInspectionIdsRef.current.length > 0 ? leftoverInspectionIdsRef.current : leftoverWorldSelectionOverlayV1()?.ids ?? [];
    if (leftoverInspectionEpoch === 0 || leftoverIds.length === 0) return;
    const located = findPanelTabInDock(dock, FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    if (located) {
      const resolved = findPanelTabPath(dock.anchors[located.anchor], FRAMEWORK_PANEL_TAB_INSPECTION_ID);
      const current = panels[located.anchor]?.path ?? [];
      if (resolved && current.join("/") !== resolved.join("/")) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
      if (!panels[located.anchor]?.visible) dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
    }
    const inspectionCacheKey = `panel:${FRAMEWORK_PANEL_TAB_INSPECTION_ID}`;
    const cachedInspection = uiRefreshCacheRef.current.get(inspectionCacheKey);
    const inspectionHash = leftoverInspectionPanelHash(leftoverIds, cachedInspection?.hash);
    if (inspectionHash === undefined) {
      uiRefreshCacheRef.current.delete(inspectionCacheKey);
      for (const key of [...uiRefreshCacheRef.current.keys()]) {
        if (key.includes("inspection") || key.includes("inspector")) uiRefreshCacheRef.current.delete(key);
      }
    }
    const scope = leftoverInspectionRefreshScope(leftoverIds);
    const currentSession = sessionRef.current;
    if (scope && currentSession) {
      let cancelled = false;
      const timer = window.setTimeout(() => {
        if (cancelled) return;
        void refreshUi(currentSession, scope, undefined, true).catch((error: unknown) => reportRefreshFault(currentSession.pluginId, error));
      }, 250);
      return () => {
        cancelled = true;
        window.clearTimeout(timer);
      };
    }
  }, [leftoverInspectionEpoch, refreshUi]);

  // 🎯️ Moving focus between panes changes what every app-level panel body is addressed at
  // (`ViewModel::focused_window_id`), so the panel bodies are re-fetched for the newly focused pane.
  // Only the panel half: window bodies are re-projected per instance and cannot move on focus alone,
  // and the refresh is hash-conditional, so a panel that does not read the focused pane answers with
  // its unchanged hash and costs one compare (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B15).
  const lastFocusedPanelWindowIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (lastFocusedPanelWindowIdRef.current === activeWindowId) return;
    lastFocusedPanelWindowIdRef.current = activeWindowId;
    const currentSession = sessionRef.current;
    if (!currentSession) return;
    if (activeWindowId && spawnedIdOfWindowInstanceV1(activeWindowId, spawnedIdsRef.current) !== null) return;
    const panelBodies = flattenPanelTabLeaves(currentSession.app.panelTabs)
      .map((tab) => tab.bodyKey)
      .filter((bodyKey): bodyKey is string => Boolean(bodyKey));
    if (panelBodies.length === 0) return;
    void refreshUi(currentSession, { kind: "partial", panelBodies }).catch((error: unknown) => reportRefreshFault(currentSession.pluginId, error));
  }, [activeWindowId, refreshUi]);

  const lastDetailsOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!detailsOverrideTabId || !detailsOverrideAnchor) {
      lastDetailsOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastDetailsOverrideTabIdRef.current === detailsOverrideTabId) return;
    lastDetailsOverrideTabIdRef.current = detailsOverrideTabId;
    if (detailsOverrideAnchor === studioOverrideAnchor) return;
    // 🧭️ Skip the override while the Settings branch is active, so browsing Theme/Hotkeys is never stomped.
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_SETTINGS_PANEL_ID) return;
      const resolved = findPanelTabPath(mobilePanelTabs, detailsOverrideTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      return;
    }
    if (panels[detailsOverrideAnchor].path[0] === FRAMEWORK_SETTINGS_PANEL_ID) return;
    const resolved = findPanelTabPath(dock.anchors[detailsOverrideAnchor], detailsOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: detailsOverrideAnchor, value: resolved });
  }, [detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);

  /**
   * 🛠️ Single owner of "the selected `tool.<id>` leaf tab IS the active tool" (see
   * {@link reconcileToolTabSelection}). Every route into the Tool category's path lands here — a user
   * press, a `DockUiStateStore` arrangement restored on boot, an introduction step, a program
   * `setActiveTool` effect, a utility claiming the pointer — so the tab and the tool can never disagree,
   * and the very first press on a restored `tool.fill` leaf arms Fill instead of reading as a re-press.
   * Skipped wholesale (leaving `toolTabSelectionRef` untouched) while the Tool category is not the active
   * root, so an armed tool survives browsing the Command palette and is reconciled again on re-entry.
   */
  const toolTabSelectionRef = useRef<ToolTabSelection | null>(null);
  const pendingToolActivateRef = useRef<string | null | undefined>(undefined);
  const revealedToolIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!session) return;
    const toolAnchor = findPanelTabInDock(dock, FRAMEWORK_CATEGORY_TOOL_ID)?.anchor ?? "bottom-middle";
    const toolCategoryTabs = mobile ? mobilePanelTabs : dock.anchors[toolAnchor];
    const branchPath = findPanelTabPath(toolCategoryTabs, FRAMEWORK_CATEGORY_TOOL_ID);
    if (!branchPath) return;
    const path = mobile ? mobilePanelPath : panelActivePaths[toolAnchor];
    const toolCategoryActive = !branchPath.some((segment, index) => path[index] !== segment);
    if (programArmedToolRevealV1(revealedToolIdRef.current, activeToolId, toolCategoryActive)) {
      revealedToolIdRef.current = activeToolId;
      const value = findPanelTabPath(toolCategoryTabs, `tool.${activeToolId}`) ?? branchPath;
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_PATH", value } : { type: "SET_PANEL_PATH", anchor: toolAnchor, value });
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_VISIBLE", value: true } : { type: "SET_PANEL_VISIBLE", anchor: toolAnchor, value: true });
      return;
    }
    if (!toolCategoryActive) return;
    revealedToolIdRef.current = activeToolId;
    const selectedToolId = toolIdFromPanelTabId(path[path.length - 1]);
    if (pendingToolActivateRef.current !== undefined) {
      const pending = pendingToolActivateRef.current;
      if ((pending ?? "") === (activeToolId ?? "")) {
        pendingToolActivateRef.current = undefined;
      } else if (selectedToolId === pending) {
        return;
      } else {
        pendingToolActivateRef.current = undefined;
      }
    }
    const { next, effect } = reconcileToolTabSelection(toolTabSelectionRef.current, activeToolId, selectedToolId);
    toolTabSelectionRef.current = next;
    if (effect.kind === "activate") {
      pendingToolActivateRef.current = effect.toolId;
      void onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: effect.toolId ?? "", expectedGeneration: toolGeneration() } });
      return;
    }
    if (effect.kind === "select") {
      const value = (effect.toolId ? findPanelTabPath(toolCategoryTabs, `tool.${effect.toolId}`) : undefined) ?? branchPath;
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_PATH", value } : { type: "SET_PANEL_PATH", anchor: toolAnchor, value });
    }
  }, [activeToolId, dock, mobile, mobilePanelPath, mobilePanelTabs, onActionStable, panelActivePaths, session, toolGeneration]);
  //#endregion 🧭️DockAssembly
  //#endregion FrameworkOsShell

  return {
    dock, mobilePanelTabs, handleTabDockDrop, handleTreeUnitDockDrop, introductionUtilityId,
    introductionActionWindowSegment, introductionUtilityWindowId, introductionMeasureWindowId, panelActivePaths
  };
}
