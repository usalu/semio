/** 🧱️ The dock model assembly, the persisted dock skeleton and ui state and the tab and tree unit drop handlers. */
import { useCallback, useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { type DockUiPanelState } from "@semio-tech/framework";
import { type Anchor, ANCHORS, applyDockSkeleton, dockSkeletonOf, dockSkeletonsEqual, findPanelTabPath, moveTabInDock, moveTreeUnitInDock, type PanelDock, panelTabChildren, type PanelTabDockMove, type PanelTabNode, type PanelTreeUnitDockMove, reconcileActivePath, singleTreeLeaf } from "@semio-tech/ui-react";
import { DEFAULT_PANEL_WIDTH_PX, FRAMEWORK_CATEGORY_COMMAND_ID, FRAMEWORK_CATEGORY_DISPLAY_ID, FRAMEWORK_CATEGORY_TOOL_ID } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { categoryTabIcon } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { shellLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { shellLabel, shellTabIcon } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { ShellCoreAfterCommandBar } from "../../🔎️command-bar/🟦️.tsx";

/** 🏗 Assembles the default dock, the host dock and the mobile panel tabs from the shell core. */
export function useDockModel({ core }: { readonly core: ShellCoreAfterCommandBar }) {
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
    const topLeft: PanelTabNode[] = [...workbenchLeftTabs];
    const bottomLeft: PanelTabNode[] = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    bottomLeft.push(...displayBottomLeftTabs);
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight: PanelTabNode[] = [...detailsRightTabs, frameworkChatTab];
    const bottomRight: PanelTabNode[] = [settingsBottomRightDockTab, frameworkMarketplaceTab, frameworkTaskManagerTab];
    if (frameworkHistoryTab) bottomRight.push(frameworkHistoryTab);
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

  const mobilePanelTabs = useMemo(() => {
    const anchorTabs = ANCHORS.flatMap((anchor) => defaultDock.anchors[anchor]);
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
  return { dock, defaultDock, dockLayoutStore, dispatch, dockUiStateStore, panels, panelPathMemory, treeOpenStates, noteShellCommand, hostMode, session, hostAppId, panel, hostCatalogueTabId, activeIntroduction, introductionStepIndex, appLabelsOverlay, uiTerminology, uiLocale, windowMeasuresByWindowId, extraWindowInstances, focusedToolMeasuresByToolId, activeToolIdRef, onActionStable, mobile, mobilePanelTabs, completeIntroductionInteraction, refreshHistorySnapshot, focusedProgram, mobilePanelPath, toolRunPanelNode, focusedTimeTravel, scope, applyAgentShellCommandRef, agentDeferredPlansRef, agentArtifactRouteRef, sessionRef, spawnedAppsRef, loadedPluginsRef, onActionRef, activeWindowIdRef, leftoverInspectionIdsRef, leftoverInspectionEpoch, uiRefreshCacheRef, refreshUi, reportRefreshFault, activeWindowId, spawnedIdsRef, activeToolId, toolGeneration };
}

/** 💾 Persists the dock skeleton and ui state once and on every change and builds the tab and tree unit drop handlers. */
export function useDockPersistence({ dock, defaultDock, dockLayoutStore, dispatch, dockUiStateStore, panels, panelPathMemory, treeOpenStates, noteShellCommand }: Pick<ReturnType<typeof useDockModel>, "dock" | "defaultDock" | "dockLayoutStore" | "dispatch" | "dockUiStateStore" | "panels" | "panelPathMemory" | "treeOpenStates" | "noteShellCommand">) {
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
  return { handleTabDockDrop, handleTreeUnitDockDrop };
}
