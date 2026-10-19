/** 📋️ The mobile panel, initial example readiness, panel tab bar hosting, navbar and search items, mode windows and the effective mode layout.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `usePanelHosting` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { useInitialExampleReadiness } from "../../🐚️Shell/🎬️initial-example/🟦️.ts";
// #region 🔌️Adapters
import {
  type CSSProperties,
  type ReactNode,
  useCallback,
  useEffect,
  useMemo,
  useRef
} from "react";
import {
  type AppDefinition,
  type LocalizedLabel,
  panelTabKindId,
  windowLayoutWindowIdsV1,
  SET_ACTIVE_TOOL_ACTION_ID
} from "@semio-tech/framework";
import { spawnedGuestWindowInstancesV1, spawnedWindowInstanceIdV1 } from "../🪟️spawned-program/🟦️.ts";
import {
  type Anchor,
  childElementId,
  ChromeAwareWindowScrollSurface,
  cn,
  findPanelTabNode,
  Icon,
  type ModeWindowDescriptor,
  navbarFillItem,
  type NavbarItem,
  PanelChromeTabBar,
  type PanelTabSelectionOptions,
  SemioLogo,
  ShellBrandLogo,
  ResponsiveLabel,
  shellChromeTitleClassName,
  Toggle,
  WindowBodySkeleton,
  type WindowLayoutNode
} from "@semio-tech/ui-react";
import { InterpretedUiNode } from "../../🗣️Interpreter/🟦️.tsx";
import { TreeWindowContext } from "../../🗣️Interpreter/🪟️tree-windows/🟦️.ts";
import { MediaTransportOwnerContext, type MediaTransportOwner } from "../../🎬️MediaTransportHost/🟦️.tsx";
import { resolveBootExampleId } from "../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { ShellFaultBoundary } from "../../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { World3dWindowViewStoreContext } from "../../🌐️World3dHost/🪟️window-view-store/🟦️.tsx";
import { wireLabel, WindowInstanceIdContext } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { FRAMEWORK_CATEGORY_COMMAND_ID } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import {
  LAYOUT_CHANGE_SETTLE_MS,
  appBreadcrumb,
  appWindowLabel,
  classifyWindowLayoutChange,
  flattenPanelTabLeaves,
  resolveAppBreadcrumb,
  resolveArtifactByAppId,
  resolveFrameworkLayoutSeed,
  frameworkLayoutDeclaredInstances,
  resolveWindowEngagement,
  spawnedWindowChromeForKind,
  windowEngagementToSearchSpec,
  windowEngagementToSpec
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import {
  toolCategoryOpenPath,
  toolLeafInactiveRepress,
  toolPanelTreeContentRevision,
  commandAddressKey,
  commandCategoryLabel,
  commandKeybindingChords,
  detectCommandPlatform
} from "../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { introductionTargetsWindow } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { shellLabelTextV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import {
  resolveAppLabel,
  resolveManifestLabel,
  resolvePanelTabLabel,
  shellLabel,
  surfaceRoleChipText,
  viewerReadOnlyNoticeText
} from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { resolveUtilityNodes } from "../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { utilityBarNode } from "../../🛠️ShellHelpers/🪟️window-panes/🪛️utility-bar-options/🟦️.tsx";
import { windowActionPaneNode, type ActionPaneSlice } from "../../🛠️ShellHelpers/🪟️window-panes/🟦️.tsx";
import { windowMeasuresChrome } from "../../🛠️ShellHelpers/🪟️window-panes/🎚️window-measures/🟦️.tsx";
// #endregion 🔌️Adapters
import { type UISearchItem } from "../../🔎️ShellSearch/🟦️.tsx";
import { WindowFaultStatus } from "../🎚️shell-model/🟦️.tsx";
import { windowHostEventHandlersV1 } from "../📨️window-invocation/🟦️.ts";
import { PENDING_WINDOW_UI_NODE, withLocalizedWindowKindLabels } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterDockAssembly } from "../🏗️dock-assembly/🟦️.ts";

export type ShellCoreAfterPanelHosting = ShellCoreAfterDockAssembly & ReturnType<typeof usePanelHosting>;

//#region FrameworkOsShell
export function usePanelHosting(core: ShellCoreAfterDockAssembly) {
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

  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label as LocalizedLabel | string, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.windows"),
        icon: <Icon icon="app-window" size="small" />,
        onSelect: () => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: kind.id }),
      });
    }
    for (const { definition, address } of resolvedCommands) {
      if (!definition.inPalette) continue;
      const argCarrying = (definition.args?.length ?? 0) > 0;
      items.push({
        id: `command.${commandAddressKey(address).replaceAll(":", ".")}`,
        label: (() => { const resolved = resolveManifestLabel(definition.label as LocalizedLabel | string, uiTerminology, uiLocale); return argCarrying ? `${resolved}…` : resolved; })(),
        description: commandKeybindingChords(definition, detectCommandPlatform(typeof navigator !== "undefined" ? `${navigator.platform} ${navigator.userAgent}` : "")).join(",") || undefined,
        category: commandCategoryLabel(definition.category),
        onSelect: () => {
          if (argCarrying) {
            const commandPath = [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${definition.category}`];
            if (mobile) {
              dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
              dispatch({ type: "SET_MOBILE_PANEL_PATH", value: commandPath });
            } else {
              dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
              dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: commandPath });
            }
            dispatch({ type: "SET_COMMAND_EXPANDED", value: commandAddressKey(address) });
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onCommand(address);
        },
      });
    }
    if (hostMode && panel) {
      const spawnEntryCounts = new Map<string, number>();
      for (const program of spacePrograms) {
        const seen = spawnEntryCounts.get(program.pluginId) ?? 0;
        spawnEntryCounts.set(program.pluginId, seen + 1);
        items.push({
          id: seen === 0 ? `spawn.${program.pluginId}` : `spawn.${program.pluginId}.${program.appId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appBreadcrumb(resolveArtifactByAppId(loadedPlugins, program.appId, program.breadcrumb, uiTerminology))}`,
          description: `${program.pluginId} · ${program.appId}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => void spawnProgram(program),
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.hostApp"),
          icon: <Icon icon="undo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "undo" }),
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.hostApp"),
          icon: <Icon icon="redo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "redo" }),
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "goHome" }),
        },
      );
    }
    return items;
  }, [activeWindowId, appLabelsOverlay, loadedPlugins, mobile, onAction, onCommand, panel, resolvedCommands, session, spacePrograms, hostMode, uiLocale, uiTerminology, hostControllerId]);

  const modeWindows = useMemo((): ModeWindowDescriptor[] => {
    if (!session) return [];
    const primaryMediaHandle = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    const primaryMediaOwner = (windowId: string): MediaTransportOwner | null => primaryMediaHandle ? { instanceId: session.instanceId, controllerId: session.app.controllerId, windowId, port: primaryMediaHandle } : null;
    const actionPaneSlice: ActionPaneSlice = { expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId };
    const actionsFoldedFor = (windowId: string, windowKindId: string = windowId) =>
      introductionTargetsWindow(windowId, windowKindId, null, introductionActionWindowSegment) ? false : (actionPaneFoldedByWindowId[windowId] ?? true);
    const utilityBarFoldedFor = (windowId: string, windowKindId: string = windowId): boolean | undefined =>
      introductionTargetsWindow(windowId, windowKindId, introductionUtilityWindowId) ? false : undefined;
    const measuresFoldedFor = (windowId: string, windowKindId: string = windowId): boolean | undefined =>
      introductionTargetsWindow(windowId, windowKindId, introductionMeasureWindowId) ? false : undefined;
    const onActionsFoldedFor = (windowId: string) => (folded: boolean) => dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: folded });
    const cursorFor = (app: AppDefinition, windowId: string): CSSProperties | undefined => {
      const utilityId = activeUtilityByWindowId[windowId];
      const cursor = utilityId ? (app.utilities ?? []).find((utility) => utility.id === utilityId)?.cursor : undefined;
      return cursor ? { cursor } : undefined;
    };
    if (hostMode && activeSpawnedEntry && activeSpawnedApp && focusedSpawnedId !== null) {
      const spawned = activeSpawnedEntry;
      const spawnedApp = activeSpawnedApp;
      const spawnedLocalDocumentOwner = localDocumentOwnerRegistry.acquire({ pluginId: spawned.pluginId, appId: spawned.appId, sessionInstanceId: spawned.instanceId, runtimeKey: `spawned:${spawned.id}`, clientInstanceId: spawned.id });
      const spawnedMediaHandle = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.handle;
      const spawnedMediaOwner = (windowId: string): MediaTransportOwner | null => spawnedMediaHandle ? { instanceId: spawned.instanceId, controllerId: spawnedApp.controllerId, windowId, port: spawnedMediaHandle } : null;
      const spawnedWorld3dViews = world3dWindowViewRegistry.scope(`${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`);
      const spawnedKindById = new Map(spawnedApp.windowKinds.map((kind) => [kind.id, kind] as const));
      const spawnedDeclaredTitles = new Map(resolveFrameworkLayoutSeed(spawnedApp.defaultLayout, withLocalizedWindowKindLabels(spawnedApp.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).extraInstances.map((extra) => [extra.id, extra.title] as const));
      return spawnedGuestWindowInstancesV1(spawnedApp.windowKinds, frameworkLayoutDeclaredInstances(spawnedApp.defaultLayout, spawnedApp.windowKinds)).flatMap((guest) => {
        const kind = spawnedKindById.get(guest.windowKindId);
        return kind ? [{ guestId: guest.id, windowKind: kind }] : [];
      }).map(({ guestId, windowKind }) => {
        const windowId = spawnedWindowInstanceIdV1(spawned.id, guestId);
        const declaredTitle = spawnedDeclaredTitles.get(guestId);
        const chrome = spawnedWindowChromeForKind(windowKind, windowId, spawnedWindowEngagements, spawnedWindowMeasures, activeUtilityByWindowId[windowId] ?? undefined, onActionStable);
        const spawnedUtilities = resolveUtilityNodes(spawnedApp, windowKind, activeUtilityByWindowId[windowId], windowId, appLabelsOverlay, uiTerminology, uiLocale, utilityGenerationFor(windowId));
        const bodyStore = Object.hasOwn(spawnedWindowActivityByWindowId, windowId) ? builtNodeStoreCacheRef.current.storeOf(`spawned:${windowId}`) : null;
        return {
          id: windowId,
          iconId: windowIconsById[windowId] ?? windowKind.iconId ?? "app-window",
          title: wireLabel(declaredTitle !== undefined ? appWindowLabel(spawnedApp, uiTerminology, declaredTitle, uiLocale) : spawnedApp.windowKinds.length > 1 ? appWindowLabel(spawnedApp, uiTerminology, resolveAppLabel(appLabelsOverlay, "windowKind", windowKind.id, resolveManifestLabel(windowKind.label as LocalizedLabel | string, uiTerminology, uiLocale)), uiLocale) : appBreadcrumb(resolveAppBreadcrumb(spawnedApp, uiTerminology))),
          fill: true,
          controls: timeTravelIndicator,
          measures: chrome.measures,
          measuresFolded: measuresFoldedFor(windowId, windowKind.id),
          engagement: chrome.engagement,
          search: chrome.search,
          utilityBar: utilityBarNode(spawnedUtilities, windowId, onActionStable, introductionUtilityId, chrome.utilityOptions),
          utilityBarFolded: utilityBarFoldedFor(windowId, windowKind.id),
          actionPane: windowActionPaneNode(spawnedApp, windowKind, windowId, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale, localPrograms.map((entry) => entry.manifest), selectedSpaceArtifactKinds),
          actionsFolded: actionsFoldedFor(windowId, windowKind.id),
          onActionsFoldedChange: onActionsFoldedFor(windowId),
          status: spawnedWindowActivityByWindowId[windowId],
          skeleton: <WindowBodySkeleton />,
          children: (
            <ChromeAwareWindowScrollSurface id={childElementId("framework.window", windowId)} className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={cursorFor(spawnedApp, windowId)} {...windowHostEventHandlersV1(spawnedApp.controllerId, windowId, onActionStable)}>
              <World3dWindowViewStoreContext.Provider key={`${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`} value={spawnedWorld3dViews}>
                <WindowInstanceIdContext.Provider value={windowId}>
                  <ShellFaultBoundary boundaryId={`window-${windowId}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                    {bodyStore ? <MediaTransportOwnerContext.Provider value={spawnedMediaOwner(windowId)}><InterpretedUiNode windowId={windowId} localDocumentOwner={spawnedLocalDocumentOwner} store={bodyStore} onAction={onActionStable} onIntent={onIntentStable} /></MediaTransportOwnerContext.Provider> : spawnedWindowFault ? <WindowFaultStatus fault={spawnedWindowFault} /> : <WindowBodySkeleton />}
                  </ShellFaultBoundary>
                </WindowInstanceIdContext.Provider>
              </World3dWindowViewStoreContext.Provider>
            </ChromeAwareWindowScrollSurface>
          ),
        };
      });
    }
    const browserActorUi = currentBrowserActorUi?.sessionInstanceId === session.instanceId ? currentBrowserActorUi : undefined;
    const baseWindows = session.app.windowKinds.map((kind) => {
      const browserActorStore = browserActorUi === undefined ? undefined : browserActorUi.windows.get(kind.id) ?? builtNodeStoreFor(`browser-actor-pending:${kind.id}`, PENDING_WINDOW_UI_NODE);
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[kind.id], kind.id, appLabelsOverlay, uiTerminology, uiLocale, utilityGenerationFor(kind.id));
      const chrome = windowMeasuresChrome(windowMeasuresByWindowId[kind.id] ?? kind.options.measures, activeUtilityByWindowId[kind.id] ?? undefined, kind.id, onActionStable);
      const resolvedEngagement = resolveWindowEngagement(kind, kind.id, windowEngagementsByWindowId);
      return {
        id: kind.id,
        iconId: windowIconsById[kind.id] ?? kind.iconId,
        title: wireLabel(windowTitlesById[kind.id] ?? appWindowLabel(session.app, uiTerminology, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)), uiLocale)),
        fill: true,
        controls: timeTravelIndicator,
        measures: chrome.measures,
        measuresFolded: measuresFoldedFor(kind.id, kind.id),
        engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
        search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
        utilityBar: utilityBarNode(utilities, kind.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
        utilityBarFolded: utilityBarFoldedFor(kind.id, kind.id),
        actionPane: windowActionPaneNode(session.app, kind, kind.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale, localPrograms.map((entry) => entry.manifest), selectedSpaceArtifactKinds),
        actionsFolded: actionsFoldedFor(kind.id, kind.id),
        onActionsFoldedChange: onActionsFoldedFor(kind.id),
        status: browserActorStore?.getState().root === null ? windowUiByWindowId[kind.id]?.activity : browserActorStore?.getState().nodes.get(browserActorStore.getState().root!)?.activity ?? windowUiByWindowId[kind.id]?.activity,
        skeleton: <WindowBodySkeleton />,
        children: (
          <ChromeAwareWindowScrollSurface id={childElementId("framework.window", kind.id)} className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={cursorFor(session.app, kind.id)} {...windowHostEventHandlersV1(session.app.controllerId, kind.id, onActionStable)}>
            <World3dWindowViewStoreContext.Provider key={primaryWorld3dOwner} value={world3dWindowViews}>
              <WindowInstanceIdContext.Provider value={kind.id}>
                <ShellFaultBoundary boundaryId={`window-${kind.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                  {instanceFault ? <WindowFaultStatus fault={instanceFault} /> : null}
                  <TreeWindowContext.Provider value={windowTreeContext(kind.bodyKey)}>
                  <MediaTransportOwnerContext.Provider value={browserActorStore === undefined ? primaryMediaOwner(kind.id) : null}><InterpretedUiNode windowId={kind.id} localDocumentOwner={currentLocalDocumentOwner} store={browserActorStore ?? builtNodeStoreFor(`window:${kind.id}`, windowUiByWindowId[kind.id] ?? PENDING_WINDOW_UI_NODE)} onAction={onActionStable} onIntent={browserActorStore === undefined ? onIntentStable : (intent) => currentDocumentRuntimeKey !== null && browserActorUi !== undefined ? onBrowserActorIntent(currentDocumentRuntimeKey, browserActorUi, kind.id, intent) : undefined} /></MediaTransportOwnerContext.Provider>
                  </TreeWindowContext.Provider>
                </ShellFaultBoundary>
              </WindowInstanceIdContext.Provider>
            </World3dWindowViewStoreContext.Provider>
          </ChromeAwareWindowScrollSurface>
        ),
      };
    });
    const extraWindows = extraWindowInstances.flatMap((instance) => {
      const kind = session.app.windowKinds.find((entry) => entry.id === instance.windowKindId);
      if (!kind) return [];
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, appLabelsOverlay, uiTerminology, uiLocale, utilityGenerationFor(instance.id));
      const chrome = windowMeasuresChrome(windowMeasuresByWindowId[instance.id] ?? kind.options.measures, activeUtilityByWindowId[instance.id] ?? undefined, instance.id, onActionStable);
      const resolvedEngagement = resolveWindowEngagement(kind, instance.id, windowEngagementsByWindowId);
      return [
        {
          id: instance.id,
          iconId: windowIconsById[instance.id] ?? kind.iconId,
          title: wireLabel(windowTitlesById[instance.id] ?? instance.title),
          fill: true,
          controls: timeTravelIndicator,
          measures: chrome.measures,
          measuresFolded: measuresFoldedFor(instance.id, instance.windowKindId),
          engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
          search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
          utilityBar: utilityBarNode(utilities, instance.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
          utilityBarFolded: utilityBarFoldedFor(instance.id, instance.windowKindId),
          actionPane: windowActionPaneNode(session.app, kind, instance.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale, localPrograms.map((entry) => entry.manifest), selectedSpaceArtifactKinds),
          actionsFolded: actionsFoldedFor(instance.id, instance.windowKindId),
          onActionsFoldedChange: onActionsFoldedFor(instance.id),
          status: windowUiByWindowId[instance.id]?.activity,
          skeleton: <WindowBodySkeleton />,
          children: (
            <ChromeAwareWindowScrollSurface
              id={childElementId("framework.window", instance.id)}
              data-element-alias={childElementId("framework.window", kind.id)}
              className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
              style={cursorFor(session.app, instance.id)}
              {...windowHostEventHandlersV1(session.app.controllerId, instance.id, onActionStable)}
            >
              <World3dWindowViewStoreContext.Provider key={primaryWorld3dOwner} value={world3dWindowViews}>
                <WindowInstanceIdContext.Provider value={instance.id}>
                  <ShellFaultBoundary boundaryId={`window-${instance.id}`} fallbackLabel={shellLabel("ui.common.renderError")}>
                    {instanceFault ? <WindowFaultStatus fault={instanceFault} /> : null}
                    <TreeWindowContext.Provider value={windowTreeContext(kind.bodyKey)}>
                      <MediaTransportOwnerContext.Provider value={primaryMediaOwner(instance.id)}><InterpretedUiNode windowId={instance.id} localDocumentOwner={currentLocalDocumentOwner} store={builtNodeStoreFor(`window:${instance.id}`, windowUiByWindowId[instance.id] ?? PENDING_WINDOW_UI_NODE)} onAction={onActionStable} onIntent={onIntentStable} /></MediaTransportOwnerContext.Provider>
                    </TreeWindowContext.Provider>
                  </ShellFaultBoundary>
                </WindowInstanceIdContext.Provider>
              </World3dWindowViewStoreContext.Provider>
            </ChromeAwareWindowScrollSurface>
          ),
        },
      ];
    });
    return [...baseWindows, ...extraWindows];
  }, [
    timeTravelIndicator,
    actionPaneExpandedByWindowId,
    actionPaneFoldedByWindowId,
    actionPaneStagedArgsByKey,
    activeUtilityByWindowId,
    registerRevision,
    utilityGenerationFor,
    appLabelsOverlay,
    browserActorUiVersion,
    builtNodeStoreFor,
    currentBrowserActorUi,
    currentDocumentRuntimeKey,
    currentLocalDocumentOwner,
    extraWindowInstances,
    introductionActionWindowSegment,
    introductionUtilityId,
    introductionUtilityWindowId,
    loadedPlugins,
    localDocumentOwnerRegistry,
    onActionStable,
    onBrowserActorIntent,
    onIntentStable,
    panel,
    session,
    spawnedWindowEngagements,
    spawnedWindowMeasures,
    spawnedWindowActivityByWindowId,
    spawnedWindowFault,
    activeSpawnedEntry,
    activeSpawnedApp,
    focusedSpawnedId,
    instanceFault,
    hostMode,
    uiLocale,
    uiTerminology,
    windowEngagementsByWindowId,
    windowMeasuresByWindowId,
    windowTitlesById,
    windowIconsById,
    windowUiByWindowId,
    world3dWindowViewRegistry,
    world3dWindowViews,
  ]);

  const effectiveModeLayout = useMemo(
    () =>
      shellLayout ??
      (session ? resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout : { kind: "stack" as const, children: [] }),
    [appLabelsOverlay, session, shellLayout, uiTerminology, uiLocale],
  );
  effectiveModeLayoutRef.current = effectiveModeLayout;
  mountedWindowIdsRef.current = useMemo(() => windowLayoutWindowIdsV1(effectiveModeLayout), [effectiveModeLayout]);
  useEffect(() => {
    const live = sessionRef.current;
    const skipped = unmountedSkippedWindowBodiesRef.current;
    if (!live || skipped.size === 0) return;
    const mounted = mountedWindowIdsRef.current;
    const windowBodies = [...skipped].filter(([windowId]) => mounted.has(windowId)).map(([, bodyKey]) => bodyKey);
    if (windowBodies.length === 0) return;
    unmountedSkippedWindowBodiesRef.current = new Map([...skipped].filter(([windowId]) => !mounted.has(windowId)));
    void refreshUi(live, { kind: "partial", windowBodies }).catch((error) => {
      if (!dropForRetiredInstance(live, "remounted window refresh", error)) console.error("[os-shell] remounted window refresh failed", error);
    });
  }, [dropForRetiredInstance, effectiveModeLayout, refreshUi]);

  const handleActiveWindowChange = useCallback(
    (value: string | null) => {
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value });
      if (value) noteShellCommand("shell.windowActivate", shellLabelTextV1("ui.shellCommand.windowActivate"), { windowId: value });
    },
    [noteShellCommand],
  );

  const layoutChangeSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const layoutChangeClassificationRef = useRef<"resize" | "rearrange" | null>(null);
  const layoutChangePreviousRef = useRef<WindowLayoutNode | null>(effectiveModeLayout);
  useEffect(() => {
    layoutChangePreviousRef.current = effectiveModeLayout;
  }, [effectiveModeLayout]);
  useEffect(
    () => () => {
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
    },
    [],
  );
  const handleModeLayoutChange = useCallback(
    (value: WindowLayoutNode) => {
      dispatch({ type: "SET_SHELL_LAYOUT", value });
      const classification = classifyWindowLayoutChange(layoutChangePreviousRef.current, value);
      layoutChangePreviousRef.current = value;
      if (classification) layoutChangeClassificationRef.current = classification;
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
      layoutChangeSettleTimeoutRef.current = setTimeout(() => {
        layoutChangeSettleTimeoutRef.current = null;
        const finalClassification = layoutChangeClassificationRef.current;
        layoutChangeClassificationRef.current = null;
        if (finalClassification === "resize") noteShellCommand("shell.windowResize", shellLabelTextV1("ui.shellCommand.windowResize"));
        else if (finalClassification === "rearrange") noteShellCommand("shell.windowMove", shellLabelTextV1("ui.shellCommand.windowMove"));
      }, LAYOUT_CHANGE_SETTLE_MS);
    },
    [noteShellCommand],
  );

  return {
    mobilePanel, initialExampleReady, buildPanelSelectionProps, navbarItems, searchItems, modeWindows,
    effectiveModeLayout, handleActiveWindowChange, handleModeLayoutChange
  };
}
//#endregion FrameworkOsShell
