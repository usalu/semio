/** 🖼️ The canvas, footer items, panel props, the readiness beacon, the catalog smoke probe and the shell context menu.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useCanvasAndFooter` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellFooterNavbarItem } from "../../🏛️ShellFooter/🟦️.tsx";
import { useCallback, useEffect, useMemo } from "react";
import {
  type ContextMenuItemSpec,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  type LocalizedLabel,
  organizeContextMenu,
  resolveWindowActions
} from "@semio-tech/framework";
import { documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import { modeLayoutStacksV1, type WindowScopeLayoutNodeV1 } from "../⌨️window-scope/🟦️.ts";
import { spawnedIdOfWindowInstanceV1, spawnedProgramWindowInstancesV1 } from "../🪟️spawned-program/🟦️.ts";
// #region 🔌️Adapters
import {
  type Anchor,
  App,
  borderNormalBottomClass,
  CanvasSkeleton,
  cn,
  type ContextMenuItem,
  insertWindowAtDropZone,
  resolveStackPathForWindowId,
  splitWithWindow,
  isContextMenuPointerTarget,
  loadingBorderClass,
  Mode,
  navbarFillItem,
  type NavbarItem,
  PanelChromeTabBar,
  PresenceBar,
  uiDataLabel
} from "@semio-tech/ui-react";
import { ShellFaultBoundary } from "../../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { clearPendingWorldProjection } from "../../🌐️World3dHost/🪟️window-view-store/🟦️.tsx";
import { wireLabel } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { mapContextMenuSpecs } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { PANEL_TAB_BAR_HOSTS } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { actionCategoryId } from "../../🛠️ShellHelpers/🪟️window-panes/🟦️.tsx";
import { actionRequiresStagedForm } from "../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import {
  appBreadcrumb,
  resolveAppBreadcrumb,
  resolveArtifactByAppId,
  resolveFrameworkLayoutSeed,
  frameworkLayoutDeclaredInstances
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildSpacePanelState } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { toolPanelTreeContentRevision } from "../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { filterDefinitionsForRole } from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { openArtifactWithText, resolveAppLabel, resolveManifestLabel, shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { shellLabelTextV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
// #endregion 🔌️Adapters
import { PluginRecoveryPanel, ShellRouteNotFoundPage } from "../../📌️ChromePanels/🟦️.tsx";
import { HubConnectionIndicator } from "../../🔄️ShellSync/🟦️.tsx";
import { AgentApprovalsNotice } from "../../🤖️AgentApprovals/🟦️.tsx";
import { AgentPresence } from "../../🚦️AgentPresence/🟦️.tsx";
import { type ShellCatalogProbe, shellCatalogProbePlugins } from "../🎚️shell-model/🟦️.tsx";
import { withLocalizedWindowKindLabels } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterPanelHosting } from "../📋️panel-hosting/🟦️.tsx";

export type ShellCoreAfterCanvasAndFooter = ShellCoreAfterPanelHosting & ReturnType<typeof useCanvasAndFooter>;

//#region FrameworkOsShell
export function useCanvasAndFooter(core: ShellCoreAfterPanelHosting) {
  const {
    pluginFilter, documentServices, brand, scope, hostMode, mobile, dispatch, loadedPlugins, pluginStatusById,
    pluginSupervisorById, session, error, sessionFault, uiLocale, uiTerminology, hostAppId, spacePrograms,
    appLabelsOverlay, actionPaneStagedArgsByKey, activeToolId, panels, treeOpenStates, activeWindowId,
    world3dWindowViewRegistry, extraWindowCounterRef, setShellContextMenu, extraWindowInstancesRef, openSpaceIdRef,
    uiDevice, agentBridge, browserActorUiByRuntimeKeyRef, reportRefreshFault, notFoundPath, navigateShellUri, registry,
    primaryPluginId, reloadPlugin, uninstallPlugin, panel, spawnedIds, focusedToolMeasuresByToolId, writeUtilityRegister,
    effectiveModeLayoutRef, contributionsPublisherRef, refreshUi, updateSpacePanel, onAction, noteShellCommand,
    applyModeChange, handleTemplateDrop, appRouter, routerFaultByPluginId, hasOpenArtifactSurfaces, keysByActionId,
    revealAgentApproval, presencePeers, timeTravelPeers, hubConnectionStatuses, hubSessionPresence, hubLink,
    openHubWorkspace, initialExampleReady, buildPanelSelectionProps, modeWindows, effectiveModeLayout,
    handleActiveWindowChange, handleModeLayoutChange
  } = core;
  const canvas = useMemo(() => {
    if (hostMode && notFoundPath !== null) {
      return <ShellRouteNotFoundPage path={notFoundPath} onHome={() => navigateShellUri("/")} />;
    }
    const supervisorPluginId = primaryPluginId;
    const supervisorState = supervisorPluginId ? pluginSupervisorById[supervisorPluginId] : undefined;
    if (supervisorState === "crashed" || supervisorState === "quarantined") {
      return (
        <PluginRecoveryPanel
          pluginId={supervisorPluginId!}
          quarantined={supervisorState === "quarantined"}
          onRestart={() => {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: supervisorPluginId!, value: "restarting" });
            void reloadPlugin(supervisorPluginId!);
          }}
          onDisable={() => {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: supervisorPluginId!, value: "quarantined" });
            if (supervisorPluginId !== primaryPluginId) void uninstallPlugin(supervisorPluginId!);
          }}
        />
      );
    }
    if (error)
      return (
        <p className="p-double text-sm text-destructive" role="alert" data-semio-os-shell-error="" data-semio-window-fault={sessionFault?.class ?? "unknown"} data-semio-window-fault-code={sessionFault?.code ?? ""}>
          {uiDataLabel(error)}
        </p>
      );
    if (!session || !initialExampleReady) return <CanvasSkeleton label={shellLabel("ui.common.loadingPlugins")} className={cn(loadingBorderClass, "h-full w-full")} />;
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology)) }];
    const studioHomeBar =
      hostMode && session.app.id === hostAppId && !panel?.activeSpawnedId ? (
        <button
          type="button"
          className={cn(borderNormalBottomClass, "px-single py-single text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground")}
          onClick={() => onAction({ controllerId: session.app.controllerId, action: "goHome" })}
        >
          ← {shellLabel("ui.common.home")}
        </button>
      ) : null;
    const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : undefined;
    const focusedBar = focusedSpawned ? (
      <div className={cn(borderNormalBottomClass, "flex items-center gap-single px-single py-single text-sm text-muted-foreground")}>
        <button type="button" className="hover:text-foreground" onClick={() => (openSpaceIdRef.current ? navigateShellUri(`/spaces/${openSpaceIdRef.current}`) : onAction({ controllerId: session.app.controllerId, action: "closeFocusedInstance" }))}>
          ← {shellLabel("ui.common.backToWorkflow")}
        </button>
        <span>·</span>
        <span>{appBreadcrumb(resolveArtifactByAppId(loadedPlugins, focusedSpawned.appId, focusedSpawned.breadcrumb, uiTerminology))}</span>
      </div>
    ) : null;
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        {studioHomeBar}
        {focusedBar}
        <div className="min-h-0 flex-1">
          <ShellFaultBoundary boundaryId="session-canvas" fallbackLabel={shellLabel("ui.common.renderError")}>
            <App
            modes={modes.map((mode) => ({ id: mode.id, label: wireLabel(resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label as LocalizedLabel | string, uiTerminology, uiLocale))), children: null }))}
            activeModeId={session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id}
            onActiveModeChange={applyModeChange}
            chrome={false}
          >
            <Mode
              className="h-full w-full"
              mobile={mobile}
              windows={modeWindows}
              layout={effectiveModeLayout}
              activeWindowId={activeWindowId}
              onActiveWindowChange={handleActiveWindowChange}
              onLayoutChange={handleModeLayoutChange}
              onTemplateDrop={mobile ? undefined : handleTemplateDrop}
              onWindowClose={(windowId) => {
                const closeNoted = noteShellCommand("shell.windowClose", shellLabelTextV1("ui.shellCommand.windowClose"), { windowId });
                const closedSpawnedId = hostMode && panel ? spawnedIdOfWindowInstanceV1(windowId, panel.spawnedApps.map((entry) => entry.id)) : null;
                if (closedSpawnedId !== null && panel) {
                  const closedSpawned = panel.spawnedApps.find((entry) => entry.id === closedSpawnedId);
                  const closedApp = closedSpawned ? loadedPlugins.find((entry) => entry.handle.pluginId === closedSpawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === closedSpawned.appId) : undefined;
                  const mountedWindowIds = modeLayoutStacksV1(effectiveModeLayoutRef.current as WindowScopeLayoutNodeV1 | null).flatMap((stack) => [...stack.windowIds]);
                  const closedInstances = closedApp ? spawnedProgramWindowInstancesV1(closedSpawnedId, closedApp.windowKinds, frameworkLayoutDeclaredInstances(closedApp.defaultLayout, closedApp.windowKinds)) : [];
                  const remaining = closedInstances.map((instance) => instance.id).filter((id) => id !== windowId && mountedWindowIds.includes(id));
                  if (remaining.length === 0) {
                    const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== closedSpawnedId);
                    updateSpacePanel(buildSpacePanelState(nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
                    if (closedSpawned) {
                      const closedPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === closedSpawned.pluginId)?.handle;
                      void closeNoted.then(() => closedPlugin?.destroyApp(closedSpawned.instanceId)).catch(() => {});
                      world3dWindowViewRegistry.retireOwner(`${closedSpawned.pluginId}:${closedSpawned.appId}:${closedSpawned.instanceId}`);
                    }
                    for (const instance of closedInstances) writeUtilityRegister(instance.id, null, null);
                  }
                }
                clearPendingWorldProjection(windowId);
                world3dWindowViewRegistry.retireWindow(windowId);
                dispatch({
                  type: "SET_EXTRA_WINDOW_INSTANCES",
                  value: (current) => {
                    const next = current.filter((entry) => entry.id !== windowId);
                    extraWindowInstancesRef.current = next;
                    return next;
                  },
                });
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) => current ?? resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout,
                });
              }}
              onWindowOpenInNewWindow={(windowId) => {
                if (!session) return;
                if (spawnedIdOfWindowInstanceV1(windowId, spawnedIds) !== null) return;
                const extra = extraWindowInstancesRef.current.find((entry) => entry.id === windowId);
                const windowKindId = extra?.windowKindId ?? session.app.windowKinds.find((kind) => kind.id === windowId)?.id;
                if (!windowKindId) return;
                const kind = session.app.windowKinds.find((entry) => entry.id === windowKindId);
                if (!kind) return;
                extraWindowCounterRef.current += 1;
                const instanceId = `${windowKindId}-${extraWindowCounterRef.current}`;
                const title = resolveAppLabel(
                  appLabelsOverlay,
                  "windowKind",
                  kind.id,
                  resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale),
                );
                const nextExtraInstances = [...extraWindowInstancesRef.current, { id: instanceId, windowKindId, title }];
                extraWindowInstancesRef.current = nextExtraInstances;
                dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: nextExtraInstances });
                void refreshUi(session, { kind: "full" }, nextExtraInstances).catch((error: unknown) => reportRefreshFault(session.pluginId, error));
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) => {
                    const base =
                      current ??
                      resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
                    const stackPath = resolveStackPathForWindowId(base, windowId);
                    if (stackPath === null) {
                      return insertWindowAtDropZone(base, instanceId, { kind: "root-split", side: "right" });
                    }
                    return splitWithWindow(base, stackPath, instanceId, "right");
                  },
                });
                dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
                noteShellCommand("shell.windowOpenInNewWindow", shellLabelTextV1("ui.shellCommand.windowOpenInNewWindow"), {
                  windowId,
                  windowKindId,
                  instanceId,
                });
              }}
            />
          </App>
          </ShellFaultBoundary>
        </div>
      </div>
    );
  }, [initialExampleReady, activeWindowId, appLabelsOverlay, effectiveModeLayout, error, sessionFault, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, loadedPlugins, mobile, modeWindows, navigateShellUri, noteShellCommand, onAction, panel, pluginSupervisorById, primaryPluginId, refreshUi, reloadPlugin, session, notFoundPath, hostMode, uiLocale, uiTerminology, updateSpacePanel, dispatch, uninstallPlugin]);

  const footerItems = useMemo((): NavbarItem[] => {
    const compactFooter = uiDevice === "tablet";
    const items: NavbarItem[] = mobile
      ? []
      : [
          { key: "bottomLeftPanelTabs", content: <PanelChromeTabBar anchor="bottom-left" compactLabels={compactFooter} {...buildPanelSelectionProps("bottom-left")} /> },
          { key: "bottomMiddlePanelTabs", centered: true, content: <PanelChromeTabBar anchor="bottom-middle" compactLabels={compactFooter} {...buildPanelSelectionProps("bottom-middle")} /> },
        ];
    const credits = brand?.footerItems ?? [];
    if (credits.some((item) => item.placement === "leading")) items.push({ key: "footerLeadingGap", className: "w-huge", content: null });
    for (const item of credits.filter((item) => item.placement === "leading")) items.push(shellFooterNavbarItem(item, uiLocale, mobile));
    items.push(navbarFillItem("footerLeadingFill"));
    for (const item of credits.filter((item) => item.placement === "trailing")) items.push(shellFooterNavbarItem(item, uiLocale, mobile));
    if (credits.some((item) => item.placement === "trailing")) items.push({ key: "footerTrailingGap", className: "w-huge", content: null });
    if (!mobile && !(compactFooter && presencePeers.length === 0)) items.push({ key: "presenceBar", content: <PresenceBar id="s-presence-peers" peers={timeTravelPeers.peers} /> });
    if (agentBridge.pendingApprovals.length > 0) items.push({ key: "agentApprovals", content: <AgentApprovalsNotice approvals={agentBridge.pendingApprovals} onReview={revealAgentApproval} /> });
    if (agentBridge.status === "unavailable" || agentBridge.status === "incompatible") items.push({ key: "agentBridgeNotice", content: <AgentPresence status={agentBridge.status} presence={agentBridge.presence} versionMismatch={agentBridge.versionMismatch} /> });
    items.push({ key: "hubConnection", content: <HubConnectionIndicator statuses={hubConnectionStatuses} session={hubSessionPresence} link={hubLink} onSignIn={openHubWorkspace} /> });
    if (!mobile) items.push({ key: "bottomRightPanelTabs", content: <PanelChromeTabBar anchor="bottom-right" compactLabels={compactFooter} {...buildPanelSelectionProps("bottom-right")} /> });
    return items;
  }, [agentBridge.pendingApprovals, agentBridge.presence, agentBridge.status, agentBridge.versionMismatch, brand?.footerItems, buildPanelSelectionProps, hubConnectionStatuses, hubLink, hubSessionPresence, mobile, openHubWorkspace, presencePeers, revealAgentApproval, timeTravelPeers.peers, uiDevice, uiLocale]);

  const buildPanelProps = useCallback(
    (anchor: Anchor) => ({
      ...buildPanelSelectionProps(anchor),
      size: panels[anchor].size,
      onSizeChange: (value: number) => dispatch({ type: "SET_PANEL_SIZE", anchor, value }),
      tabBarHost: (PANEL_TAB_BAR_HOSTS[anchor] ? "chrome" : "panel") as "panel" | "chrome",
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      treeContentRevision: toolPanelTreeContentRevision(activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey),
    }),
    [actionPaneStagedArgsByKey, activeToolId, buildPanelSelectionProps, panels, focusedToolMeasuresByToolId, treeOpenStates],
  );

  // #region 🔖️ReadinessBeacon
  useEffect(() => {
    const root = document.documentElement;
    const shellRoot = scope.rootRef.current;
    const beaconId = pluginFilter ?? "unknown";
    const notFound = hostMode && notFoundPath !== null;
    if (notFound) {
      root.dataset.semioOsNotFound = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      if (shellRoot) {
        shellRoot.dataset.shellNotFound = beaconId;
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellError;
      }
    } else if (error) {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        shellRoot.dataset.shellError = beaconId;
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellNotFound;
      }
    } else if ((session && initialExampleReady) || (!pluginFilter && registry.length === 0)) {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        shellRoot.dataset.shellReady = beaconId;
        delete shellRoot.dataset.shellError;
        delete shellRoot.dataset.shellNotFound;
      }
    }
    return () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellError;
        delete shellRoot.dataset.shellNotFound;
      }
    };
  }, [session, initialExampleReady, error, pluginFilter, registry.length, notFoundPath, hostMode, scope.rootRef]);
  // #endregion 🔖️ReadinessBeacon

  // #region 🔖️CatalogSmokeProbe
  useEffect(() => {
    let dev = false;
    try {
      dev = Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      dev = false;
    }
    if (!dev || typeof window === "undefined") return;
    const host = window as unknown as { __semioOsCatalogProbe?: ShellCatalogProbe };
    host.__semioOsCatalogProbe = {
      shellPluginId: pluginFilter ?? "unknown",
      ready: !!session && !error,
      plugins: shellCatalogProbePlugins(registry, pluginStatusById, appRouter),
      programs: spacePrograms.map((program) => ({ pluginId: program.pluginId, appId: program.appId, label: program.label })),
      spawned: (panel?.spawnedApps ?? []).map((entry) => ({ id: entry.id, pluginId: entry.pluginId, appId: entry.appId })),
    };
    const installed = window as unknown as { __semioOsInstalledContributions?: (instanceId: number) => string | null };
    installed.__semioOsInstalledContributions = (instanceId) => contributionsPublisherRef.current?.installedFor(instanceId) ?? null;
    return () => {
      delete host.__semioOsCatalogProbe;
      delete installed.__semioOsInstalledContributions;
    };
  }, [session, error, pluginFilter, registry, pluginStatusById, panel, spacePrograms, appRouter]);

  useEffect(() => {
    let dev = false;
    try {
      dev = Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      dev = false;
    }
    if (!dev || typeof window === "undefined") return;
    const read = (owner: string, serviceId: string, spaceId: string, documentId: string): unknown => {
      const presentation = documentServices.find((entry) => entry.owner === owner && entry.serviceId === serviceId);
      if (!presentation?.probe || typeof spaceId !== "string" || typeof documentId !== "string" || spaceId.length === 0 || documentId.length === 0) return null;
      const retained = browserActorUiByRuntimeKeyRef.current.get(documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId, documentId }));
      if (retained?.identity === null || retained?.identity === undefined) return null;
      const store = retained.windows.get(retained.identity.windowKindId);
      return store === undefined ? null : presentation.probe({ ...retained.identity, sessionInstanceId: retained.sessionInstanceId, store });
    };
    const host = window as unknown as { __semioMountedServiceProbe?: typeof read };
    host.__semioMountedServiceProbe = read;
    return () => { if (host.__semioMountedServiceProbe === read) delete host.__semioMountedServiceProbe; };
  }, [documentServices]);

  useEffect(() => {
    for (const fault of routerFaultByPluginId.values()) console.error(`AppRouter excluded plugin ${JSON.stringify(fault.scope.pluginId ?? "")}: ${fault.code}: ${fault.message}`);
  }, [routerFaultByPluginId]);
  // #endregion 🔖️CatalogSmokeProbe

  //#region 🖱️ShellContextMenu
  /** 🖱️ Dispatch sink for the shell fallback menu's `ContextMenuItemSpec`s (see
   * `buildShellContextMenuItems`) — intercepts the two reserved ids the builder emits in place of a
   * real dispatch (`"shell.openActionPane"`/`"shell.openPalette"`) and forwards everything else to
   * `onAction`, mirroring the command palette's own arg-carrying redirect. */
  const dispatchShellMenuAction = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!session) return;
      if (action === "shell.openActionPane") {
        const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
        const actionId = typeof args?.actionId === "string" ? args.actionId : undefined;
        if (!windowKind || !actionId) return;
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowKind.id });
        dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId: windowKind.id, value: false });
        dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId: windowKind.id, value: actionId });
        return;
      }
      if (action === "shell.openPalette") {
        dispatch({ type: "SET_SEARCH_OPEN", value: true });
        return;
      }
      if (action === "shell.openArtifactWith") {
        dispatch({ type: "SET_OPEN_WITH_FOCUS_ROLE", value: null });
        dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [FRAMEWORK_PANEL_TAB_ARTIFACT_ID] });
        dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
        return;
      }
      onAction({ controllerId: session.app.controllerId, action });
    },
    [session, activeWindowId, onAction, dispatch],
  );

  /** 🖱️ Builds the shell-level fallback menu: the active window's declared actions (undo/redo, view
   * actions, ...) plus a command-palette opener — shown for any right-click no inner surface claimed
   * (window background, empty panel/navbar/footer space, an app with no scene at all). Arg-carrying
   * actions route through the reserved `"shell.openActionPane"` id (parity with the wgpu shell's
   * `build_shell_context_menu_specs`), the whole spec list runs through `organizeContextMenu`, then
   * `mapContextMenuSpecs` binds it to `dispatchShellMenuAction`. */
  const buildShellContextMenuItems = useCallback((): ContextMenuItem[] => {
    if (!session) return [];
    const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
    const specs: ContextMenuItemSpec[] = [];
    const categoryByActionId = new Map<string, string>();
    if (windowKind) {
      for (const action of filterDefinitionsForRole(resolveWindowActions(session.app, windowKind), session.app.role)) {
        if (!action.inPalette) continue;
        const argCarrying = actionRequiresStagedForm(action);
        categoryByActionId.set(action.id, actionCategoryId(action));
        specs.push({
          id: `shell-menu.action.${action.id}`,
          label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label as LocalizedLabel | string, uiTerminology, uiLocale)) + (argCarrying ? "…" : ""),
          icon: action.iconId,
          shortcut: action.keys ?? keysByActionId.get(action.id),
          destructive: action.kind === "mutation" && action.id.toLowerCase().includes("delete"),
          action: argCarrying ? "shell.openActionPane" : action.id,
          args: argCarrying ? { actionId: action.id } : undefined,
        });
      }
    }
    if (specs.length > 0) specs.push({ id: "shell-menu.separator", separator: true });
    if (hasOpenArtifactSurfaces) {
      specs.push({ id: "shell.openArtifactWith", label: openArtifactWithText(uiLocale), icon: "app-window", action: "shell.openArtifactWith" });
    }
    specs.push({
      id: "shell.openPalette",
      label: shellLabel("ui.search.toggle"),
      icon: "search",
      action: "shell.openPalette",
    });
    const organized = organizeContextMenu(specs, (id) => categoryByActionId.get(id));
    return mapContextMenuSpecs(organized, dispatchShellMenuAction, keysByActionId);
  }, [session, activeWindowId, appLabelsOverlay, keysByActionId, dispatchShellMenuAction, uiTerminology, uiLocale, hasOpenArtifactSurfaces]);

  useEffect(() => {
    const handleContextMenu = (event: globalThis.MouseEvent) => {
      if (isContextMenuPointerTarget(event.target)) return;
      if (event.defaultPrevented) return;
      const items = buildShellContextMenuItems();
      if (items.length === 0) return;
      event.preventDefault();
      setShellContextMenu({ x: event.clientX, y: event.clientY, items });
    };
    window.addEventListener("contextmenu", handleContextMenu);
    return () => window.removeEventListener("contextmenu", handleContextMenu);
  }, [buildShellContextMenuItems]);
  //#endregion 🖱️ShellContextMenu

  return { canvas, footerItems, buildPanelProps, buildShellContextMenuItems };
}
//#endregion FrameworkOsShell
