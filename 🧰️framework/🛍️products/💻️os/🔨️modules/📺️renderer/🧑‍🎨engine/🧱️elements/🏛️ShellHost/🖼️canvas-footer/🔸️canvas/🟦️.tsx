/** 🔸️ The canvas phase of useCanvasAndFooter. */
import type { ProgramRetirementLeaseV1 } from "../../🚪️program-retirement/🟦️.ts";
import { useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { type LocalizedLabel } from "@semio-tech/framework";
import { modeLayoutStacksV1, type WindowScopeLayoutNodeV1 } from "../../⌨️window-scope/🟦️.ts";
import { spawnedIdOfWindowInstanceV1, spawnedProgramWindowInstancesV1 } from "../../🪟️spawned-program/🟦️.ts";
import { App, borderNormalBottomClass, CanvasSkeleton, cn, insertWindowAtDropZone, resolveStackPathForWindowId, splitWithWindow, loadingBorderClass, Mode, uiDataLabel } from "@semio-tech/ui-react";
import { ShellFaultBoundary } from "../../../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { clearPendingWorldProjection } from "../../../🌐️World3dHost/🪟️window-view-store/🟦️.tsx";
import { wireLabel } from "../../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { appBreadcrumb, resolveAppBreadcrumb, resolveArtifactByAppId, resolveFrameworkLayoutSeed, frameworkLayoutDeclaredInstances } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildSpacePanelState, parsePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { resolveAppLabel, resolveManifestLabel, shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { shellLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { PluginRecoveryPanel, ShellRouteNotFoundPage } from "../../../📌️ChromePanels/🟦️.tsx";
import { withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { buildPluginFilterState } from "../🔶️plugin-filter/🟦️.tsx";
import { LocalFilesV1 } from "../../🗂️local-catalog/📂️files/🟦️.tsx";
import { deriveSavedStateV1 } from "../../../🛠️ShellHelpers/💾️saved-state/🟦️.ts";

/** 🔹 Builds canvas of useCanvasAndFooter. */
export function useCanvas({ hostMode, notFoundPath, navigateShellUri, primaryPluginId, pluginSupervisorById, dispatch, reloadPlugin, uninstallPlugin, error, sessionFault, session, initialExampleReady, uiTerminology, hostAppId, panel, onAction, openSpaceIdRef, loadedPlugins, appLabelsOverlay, uiLocale, applyModeChange, mobile, modeWindows, effectiveModeLayout, activeWindowId, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, noteShellCommand, effectiveModeLayoutRef, updateSpacePanel, world3dWindowViewRegistry, writeUtilityRegister, extraWindowInstancesRef, spawnedIds, extraWindowCounterRef, refreshUi, reportRefreshFault, localCatalog, shellStateRef, currentDocumentRuntimeKey, openKeptFile, sessionRef, admitProgramRetirementRef, requestCloseDocument, openDocumentSessionsRef, updateSpacePanelFrom }: Pick<ReturnType<typeof buildPluginFilterState>, "hostMode" | "notFoundPath" | "navigateShellUri" | "primaryPluginId" | "pluginSupervisorById" | "dispatch" | "reloadPlugin" | "uninstallPlugin" | "error" | "sessionFault" | "session" | "initialExampleReady" | "uiTerminology" | "hostAppId" | "panel" | "onAction" | "openSpaceIdRef" | "loadedPlugins" | "appLabelsOverlay" | "uiLocale" | "applyModeChange" | "mobile" | "modeWindows" | "effectiveModeLayout" | "activeWindowId" | "handleActiveWindowChange" | "handleModeLayoutChange" | "handleTemplateDrop" | "noteShellCommand" | "effectiveModeLayoutRef" | "updateSpacePanel" | "world3dWindowViewRegistry" | "writeUtilityRegister" | "extraWindowInstancesRef" | "spawnedIds" | "extraWindowCounterRef" | "refreshUi" | "reportRefreshFault" | "localCatalog" | "shellStateRef" | "currentDocumentRuntimeKey" | "openKeptFile" | "sessionRef" | "admitProgramRetirementRef" | "requestCloseDocument" | "openDocumentSessionsRef" | "updateSpacePanelFrom">) {
  const closingWindowIdsRef = useRef(new Set<string>());
  const closeOrderRef = useRef<Promise<void>>(Promise.resolve());
  const statuses = shellStateRef.current.sync.syncStatusByDocumentId;
  useEffect(() => {
    const mounted = new Set(modeLayoutStacksV1(effectiveModeLayout as WindowScopeLayoutNodeV1 | null).flatMap((stack) => [...stack.windowIds]));
    for (const id of closingWindowIdsRef.current) if (!mounted.has(id)) closingWindowIdsRef.current.delete(id);
  }, [effectiveModeLayout]);
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
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology, uiLocale)) }];
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
        <span>{appBreadcrumb(resolveArtifactByAppId(loadedPlugins, focusedSpawned.appId, focusedSpawned.breadcrumb, uiTerminology, uiLocale))}</span>
      </div>
    ) : null;
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        {studioHomeBar}
        {focusedBar}
        {hostMode && <LocalFilesV1 documents={localCatalog.documents} locale={uiLocale} activeDocumentId={currentDocumentRuntimeKey === null ? null : openDocumentSessionsRef.current.get(currentDocumentRuntimeKey)?.documentId ?? null} persisted={new Set(Object.entries(statuses).filter(([, status]) => deriveSavedStateV1(status).kind === "saved").map(([id]) => openDocumentSessionsRef.current.get(id)?.documentId ?? id))} onOpen={openKeptFile} />}
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
              onWindowClose={async (windowId) => {
                const previousClose = closeOrderRef.current;
                let release!: () => void;
                closeOrderRef.current = new Promise<void>((resolve) => { release = resolve; });
                await previousClose;
                closingWindowIdsRef.current.add(windowId);
                let accepted = false, retirement: ProgramRetirementLeaseV1 | null = null;
                try {
                const closedSpawnedId = hostMode && panel ? spawnedIdOfWindowInstanceV1(windowId, panel.spawnedApps.map((entry) => entry.id)) : null;
                const closedSpawned = panel?.spawnedApps.find((entry) => entry.id === closedSpawnedId);
                  const closedApp = closedSpawned ? loadedPlugins.find((entry) => entry.handle.pluginId === closedSpawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === closedSpawned.appId) : undefined;
                  const mountedWindowIds = modeLayoutStacksV1(effectiveModeLayoutRef.current as WindowScopeLayoutNodeV1 | null).flatMap((stack) => [...stack.windowIds]);
                  const closedInstances = closedApp && closedSpawnedId !== null ? spawnedProgramWindowInstancesV1(closedSpawnedId, closedApp.windowKinds, frameworkLayoutDeclaredInstances(closedApp.defaultLayout, closedApp.windowKinds)) : [];
                  const remaining = closedInstances.map((instance) => instance.id).filter((id) => !closingWindowIdsRef.current.has(id) && mountedWindowIds.includes(id));
                  const ownsProgram = (): boolean => {
                    const current = sessionRef.current === null ? null : parsePanelState(sessionRef.current.viewState);
                    return current?.spawnedApps.some((entry) => entry.id === closedSpawnedId && entry.pluginId === closedSpawned?.pluginId && entry.appId === closedSpawned?.appId && entry.instanceId === closedSpawned?.instanceId) === true;
                  };
                  if (closedSpawned && !ownsProgram()) return false;
                  if (closedSpawned && remaining.length === 0) {
                    const handle = loadedPlugins.find(entry => entry.handle.pluginId === closedSpawned.pluginId)?.handle;
                    if (!handle || !closedApp) return false;
                    retirement = await admitProgramRetirementRef.current([{ handle, name: closedSpawned.label, session: { pluginId: closedSpawned.pluginId, instanceId: closedSpawned.instanceId, app: closedApp, viewState: sessionRef.current?.viewState ?? session.viewState } }], ownsProgram);
                    if (retirement === null || !retirement.current()) return false;
                  }
                  await noteShellCommand("shell.windowClose", shellLabelTextV1("ui.shellCommand.windowClose"), { windowId });
                  if (closedSpawned && remaining.length === 0) {
                    if (!ownsProgram() || retirement === null || !await retirement.retire()) return false;
                    updateSpacePanelFrom((latest) => {
                      if (latest === null) return buildSpacePanelState([], panel!.activePanelTab);
                      const nextSpawned = latest.spawnedApps.filter(entry => entry.id !== closedSpawnedId || entry.pluginId !== closedSpawned.pluginId || entry.appId !== closedSpawned.appId || entry.instanceId !== closedSpawned.instanceId);
                      return buildSpacePanelState(nextSpawned, latest.activePanelTab, latest.activeSpawnedId === closedSpawnedId ? nextSpawned[0]?.id : latest.activeSpawnedId);
                    });
                      world3dWindowViewRegistry.retireOwner(`${closedSpawned.pluginId}:${closedSpawned.appId}:${closedSpawned.instanceId}`);
                    for (const instance of closedInstances) writeUtilityRegister(instance.id, null, null);
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
                accepted = true;
                return true;
                } finally {
                  retirement?.release();
                  if (!accepted) closingWindowIdsRef.current.delete(windowId);
                  release();
                }
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
  }, [initialExampleReady, activeWindowId, appLabelsOverlay, effectiveModeLayout, error, sessionFault, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, loadedPlugins, mobile, modeWindows, navigateShellUri, noteShellCommand, onAction, panel, pluginSupervisorById, primaryPluginId, refreshUi, reloadPlugin, session, notFoundPath, hostMode, uiLocale, uiTerminology, updateSpacePanel, updateSpacePanelFrom, admitProgramRetirementRef, requestCloseDocument, dispatch, uninstallPlugin, localCatalog, currentDocumentRuntimeKey, openKeptFile, statuses]);
  return { canvas };
}
