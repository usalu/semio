/** 🔶️ The brand phase of usePanelHosting. */
import { useInitialExampleReadiness } from "../../../🐚️Shell/🎬️initial-example/🟦️.ts";
import { type ReactNode, useCallback, useEffect, useMemo } from "react";
import { SET_ACTIVE_TOOL_ACTION_ID } from "@semio-tech/framework";
import { type Anchor, cn, findPanelTabNode, navbarFillItem, type NavbarItem, PanelChromeTabBar, type PanelTabSelectionOptions, SemioLogo, ShellBrandLogo, ResponsiveLabel, shellChromeTitleClassName, Toggle } from "@semio-tech/ui-react";
import { resolveBootExampleId } from "../../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { appBreadcrumb, resolveAppBreadcrumb } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { toolCategoryOpenPath, toolLeafInactiveRepress, toolPanelTreeContentRevision } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { shellLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { surfaceRoleChipText, viewerReadOnlyNoticeText } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { ShellCoreAfterDockAssembly } from "../../🏗️dock-assembly/🟦️.ts";

/** 🔷 Builds brand, localDocumentOwnerRegistry, hostMode, mobile of usePanelHosting. */
export function useBrandState({ core }: { readonly core: ShellCoreAfterDockAssembly }) {
  const {
    brand, localDocumentOwnerRegistry, hostMode, mobile, locks, defaults, dispatch, loadedPlugins, session,
    instanceFault, localPrograms, uiLocale, uiTerminology, landingAppId, hostAppId, hostControllerId, spacePrograms,
    windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, appLabelsOverlay,
    spawnedWindowActivityByWindowId, spawnedWindowFault, spawnedWindowEngagements, spawnedWindowMeasures,
    actionPaneFoldedByWindowId, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, activeUtilityByWindowId,
    activeToolId, panels, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath,
    mobilePanelVisible, extraWindowInstances, windowTitlesById, windowIconsById, world3dWindowViewRegistry,
    primaryWorld3dOwner, world3dWindowViews, dropForRetiredInstance, sessionRef, windowTreeContext,
    builtNodeStoreCacheRef, builtNodeStoreFor, browserActorUiVersion, panel, activeSpawnedEntry, activeSpawnedApp,
    focusedProgram, focusedApp, focusedSpawnedId, focusedToolMeasuresByToolId, timeTravelIndicator, registerRevision,
    utilityGenerationFor, toolGeneration, effectiveModeLayoutRef, mountedWindowIdsRef, unmountedSkippedWindowBodiesRef,
    refreshUi, spawnProgram, onAction, noteShellCommand, onActionStable, onIntentStable, onBrowserActorIntent,
    currentDocumentRuntimeKey, currentLocalDocumentOwner, currentBrowserActorUi, selectedSpaceArtifactKinds,
    focusedPluginManifest, exampleOptions, dispatchActiveExample, exampleSelectElement, modeSwitcherElement,
    roleSwitcherElement, resolvedCommands, onCommand, toolTabs, dock, mobilePanelTabs, introductionUtilityId,
    introductionActionWindowSegment, introductionUtilityWindowId, introductionMeasureWindowId, panelActivePaths
  } = core;
  const mobilePanel = useMemo(() => {
    if (mobilePanelTabs.length === 0) return undefined;
    return {
      visible: mobilePanelVisible,
      tabs: mobilePanelTabs,
      activeTabPath: mobilePanelPath,
      onActiveTabPathChange: (path: readonly string[]) => {
        dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
        const tabId = path[path.length - 1];
        if (tabId && hostMode && session && session.app.id === hostAppId && findPanelTabNode(mobilePanelTabs, path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      treeContentRevision: toolPanelTreeContentRevision(activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey),
    };
  }, [mobilePanelVisible, mobilePanelPath, mobilePanelTabs, onAction, panelPathMemory, session, hostMode, treeOpenStates, hostAppId, activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey]);

  useEffect(() => {
    if (exampleOptions.length === 0) return;
    dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: (current) => (!current || exampleOptions.some((option) => option.id === current) ? current : "") });
  }, [exampleOptions, session?.app.id, session?.pluginId]);

  const initialExampleReady = useInitialExampleReadiness(session ? `${session.pluginId}:${session.instanceId}` : null,!hostMode && !session?.documentInitialized,useCallback(() => {
    if (!session || !focusedPluginManifest || !focusedApp || !focusedProgram) return;
    if (exampleOptions.length === 0) return Promise.resolve();
    const exampleId = resolveBootExampleId(activeExampleId,exampleOptions,defaults.exampleId);
    if (exampleId !== activeExampleId) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID",value: exampleId });
    return dispatchActiveExample(exampleId);
  },[session,focusedPluginManifest,focusedApp,focusedProgram,exampleOptions,activeExampleId,defaults.exampleId,dispatchActiveExample]),useCallback(async () => {
    if (session) await refreshUi(session,{ kind: "full" });
  },[session,refreshUi]));

  //#region 🎛️PanelTabBarHosting — `buildPanelSelectionProps` is the single source of an anchor's tab
  const closePeerRightDockAnchors = useCallback((except: Anchor) => {
    for (const peer of ["top-right", "bottom-right", "right-middle"] as const) {
      if (peer !== except) dispatch({ type: "SET_PANEL_VISIBLE", anchor: peer, value: false });
    }
  }, []);
  const buildPanelSelectionProps = useCallback(
    (anchor: Anchor): PanelTabSelectionOptions => ({
      tabs: dock.anchors[anchor],
      visible: panels[anchor].visible,
      onVisibleChange: (value: boolean) => {
        if (value && (anchor === "top-right" || anchor === "bottom-right" || anchor === "right-middle")) closePeerRightDockAnchors(anchor);
        dispatch({ type: "SET_PANEL_VISIBLE", anchor, value });
        noteShellCommand("shell.panelToggle", shellLabelTextV1("ui.shellCommand.panelToggle"), { anchor, visible: value });
      },
      activeTabPath: panelActivePaths[anchor],
      onActiveTabPathChange: (path: readonly string[]) => {
        const previous = panelActivePaths[anchor] ?? [];
        const inactiveRepress = toolLeafInactiveRepress(previous, path, activeToolId ?? null);
        if (inactiveRepress && session) {
          path = inactiveRepress.path;
          void onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: inactiveRepress.toolId, expectedGeneration: toolGeneration() } });
        }
        const pathChanged = previous.join("/") !== path.join("/");
        dispatch({ type: "SET_PANEL_PATH", anchor, value: path });
        if (anchor === "bottom-middle" && panels[anchor].path[1] !== path[1]) {
          dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
        }
        const tabId = path[path.length - 1];
        if (tabId && hostMode && session && session.app.id === hostAppId && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
        if (pathChanged && tabId) noteShellCommand("shell.panelTab", shellLabelTextV1("ui.shellCommand.panelTab"), { anchor, tabId });
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      drillOnOpen: anchor === "bottom-middle" ? (path, memory) => toolCategoryOpenPath(path, memory, toolTabs.map((tab) => tab.id)) : undefined,
    }),
    [activeToolId, closePeerRightDockAnchors, dock, onAction, panelActivePaths, panelPathMemory, panels, session, hostMode, hostAppId, noteShellCommand, toolTabs, toolGeneration],
  );
  //#endregion 🎛️PanelTabBarHosting

  const navbarItems = useMemo((): NavbarItem[] => {
    if (!session) return [];
    const logoAndTitle = (
      <div key="logoAndTitle" className="flex min-w-0 shrink-0 items-center gap-single">
        {brand?.logoSvg ? <ShellBrandLogo svg={brand.logoSvg} className="size-workbench shrink-0" /> : <SemioLogo className="size-workbench shrink-0" />}
        <span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>
          <ResponsiveLabel full={brand?.windowTitle ?? appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))} short={brand?.shortWindowTitle} />
        </span>
        <span data-slot="surface-role-chip" data-role={session.app.role} data-app-id={session.app.id} title={session.app.role === "viewer" ? viewerReadOnlyNoticeText(uiLocale) : undefined} className="rounded-sm border border-border px-single text-xs text-muted-foreground">
          {surfaceRoleChipText(session.app.role, uiLocale)}
        </span>
      </div>
    );
    const showExampleSelect = exampleOptions.length > 0 && !locks.exampleId && (!hostMode || focusedApp?.id !== landingAppId);
    if (mobile) {
      return [
        { key: "logoAndTitle", content: logoAndTitle },
        navbarFillItem("navbarTrailingFill"),
        {
          key: "mobilePanelToggle",
          content: <Toggle id="ui.mobilePanel.toggle" pressed={mobilePanelVisible} onPressedChange={(value) => dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value })} icon="panel-left" />,
        },
      ];
    }
    const centerContent: ReactNode[] = [logoAndTitle];
    if (showExampleSelect && exampleSelectElement) centerContent.push(exampleSelectElement);
    if (modeSwitcherElement) centerContent.push(modeSwitcherElement);
    if (roleSwitcherElement) centerContent.push(roleSwitcherElement);
    return [
      { key: "topLeftPanelTabs", content: <PanelChromeTabBar anchor="top-left" {...buildPanelSelectionProps("top-left")} /> },
      navbarFillItem("navbarTrailingFill"),
      { key: "topRightPanelTabs", content: <PanelChromeTabBar anchor="top-right" {...buildPanelSelectionProps("top-right")} /> },
      {
        key: "center",
        centered: true,
        content: (
          <div className="flex min-w-0 items-center gap-double">
            {centerContent}
            <PanelChromeTabBar anchor="top-middle" {...buildPanelSelectionProps("top-middle")} />
          </div>
        ),
      },
    ];
  }, [brand, buildPanelSelectionProps, exampleOptions, exampleSelectElement, locks.exampleId, mobile, mobilePanelVisible, modeSwitcherElement, roleSwitcherElement, session, focusedApp?.id, uiLocale, uiTerminology, hostMode, landingAppId]);
  return { session, appLabelsOverlay, uiTerminology, uiLocale, onAction, dispatch, resolvedCommands, mobile, onCommand, hostMode, panel, spacePrograms, loadedPlugins, spawnProgram, hostControllerId, activeWindowId, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, activeUtilityByWindowId, introductionActionWindowSegment, actionPaneFoldedByWindowId, introductionUtilityWindowId, introductionMeasureWindowId, activeSpawnedEntry, activeSpawnedApp, focusedSpawnedId, localDocumentOwnerRegistry, world3dWindowViewRegistry, spawnedWindowEngagements, spawnedWindowMeasures, onActionStable, utilityGenerationFor, spawnedWindowActivityByWindowId, builtNodeStoreCacheRef, windowIconsById, timeTravelIndicator, introductionUtilityId, localPrograms, selectedSpaceArtifactKinds, onIntentStable, spawnedWindowFault, currentBrowserActorUi, builtNodeStoreFor, windowMeasuresByWindowId, windowEngagementsByWindowId, windowTitlesById, windowUiByWindowId, primaryWorld3dOwner, world3dWindowViews, instanceFault, windowTreeContext, currentLocalDocumentOwner, currentDocumentRuntimeKey, onBrowserActorIntent, extraWindowInstances, registerRevision, browserActorUiVersion, shellLayout, effectiveModeLayoutRef, mountedWindowIdsRef, sessionRef, unmountedSkippedWindowBodiesRef, refreshUi, dropForRetiredInstance, noteShellCommand, mobilePanel, initialExampleReady, buildPanelSelectionProps, navbarItems };
}
