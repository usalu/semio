/** 🟠️ The mode windows phase of usePanelHosting. */
import { type CSSProperties, useMemo } from "react";
import { type AppDefinition, type LocalizedLabel } from "@semio-tech/framework";
import { spawnedGuestWindowInstancesV1, spawnedWindowInstanceIdV1 } from "../../🪟️spawned-program/🟦️.ts";
import { childElementId, ChromeAwareWindowScrollSurface, type ModeWindowDescriptor, WindowBodySkeleton } from "@semio-tech/ui-react";
import { InterpretedUiNode } from "../../../🗣️Interpreter/🟦️.tsx";
import { TreeWindowContext } from "../../../🗣️Interpreter/🪟️tree-windows/🟦️.ts";
import { MediaTransportOwnerContext, type MediaTransportOwner } from "../../../🎬️MediaTransportHost/🟦️.tsx";
import { ShellFaultBoundary } from "../../../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { World3dWindowViewStoreContext } from "../../../🌐️World3dHost/🪟️window-view-store/🟦️.tsx";
import { wireLabel, WindowInstanceIdContext } from "../../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { appBreadcrumb, appWindowLabel, resolveAppBreadcrumb, resolveFrameworkLayoutSeed, frameworkLayoutDeclaredInstances, resolveWindowEngagement, spawnedWindowChromeForKind, windowEngagementToSearchSpec, windowEngagementToSpec } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { introductionTargetsWindow } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { resolveAppLabel, resolveManifestLabel, shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { resolveUtilityNodes } from "../../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { utilityBarNode } from "../../../🛠️ShellHelpers/🪟️window-panes/🪛️utility-bar-options/🟦️.tsx";
import { windowActionPaneNode, type ActionPaneSlice } from "../../../🛠️ShellHelpers/🪟️window-panes/🟦️.tsx";
import { windowMeasuresChrome } from "../../../🛠️ShellHelpers/🪟️window-panes/🎚️window-measures/🟦️.tsx";
import { WindowFaultStatus } from "../../🎚️shell-model/🟦️.tsx";
import { windowHostEventHandlersV1 } from "../../📨️window-invocation/🟦️.ts";
import { PENDING_WINDOW_UI_NODE, withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useBrandState } from "../🔶️brand/🟦️.tsx";

/** 🟡 Builds modeWindows of usePanelHosting. */
export function useModeWindows({ session, loadedPlugins, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, activeUtilityByWindowId, introductionActionWindowSegment, actionPaneFoldedByWindowId, introductionUtilityWindowId, introductionMeasureWindowId, dispatch, hostMode, activeSpawnedEntry, activeSpawnedApp, focusedSpawnedId, localDocumentOwnerRegistry, world3dWindowViewRegistry, appLabelsOverlay, uiTerminology, uiLocale, spawnedWindowEngagements, spawnedWindowMeasures, onActionStable, utilityGenerationFor, spawnedWindowActivityByWindowId, builtNodeStoreCacheRef, windowIconsById, timeTravelIndicator, introductionUtilityId, localPrograms, selectedSpaceArtifactKinds, onIntentStable, spawnedWindowFault, currentBrowserActorUi, builtNodeStoreFor, windowMeasuresByWindowId, windowEngagementsByWindowId, windowTitlesById, windowUiByWindowId, primaryWorld3dOwner, world3dWindowViews, instanceFault, windowTreeContext, currentLocalDocumentOwner, currentDocumentRuntimeKey, onBrowserActorIntent, extraWindowInstances, registerRevision, browserActorUiVersion, panel }: Pick<ReturnType<typeof useBrandState>, "session" | "loadedPlugins" | "actionPaneExpandedByWindowId" | "actionPaneStagedArgsByKey" | "activeUtilityByWindowId" | "introductionActionWindowSegment" | "actionPaneFoldedByWindowId" | "introductionUtilityWindowId" | "introductionMeasureWindowId" | "dispatch" | "hostMode" | "activeSpawnedEntry" | "activeSpawnedApp" | "focusedSpawnedId" | "localDocumentOwnerRegistry" | "world3dWindowViewRegistry" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "spawnedWindowEngagements" | "spawnedWindowMeasures" | "onActionStable" | "utilityGenerationFor" | "spawnedWindowActivityByWindowId" | "builtNodeStoreCacheRef" | "windowIconsById" | "timeTravelIndicator" | "introductionUtilityId" | "localPrograms" | "selectedSpaceArtifactKinds" | "onIntentStable" | "spawnedWindowFault" | "currentBrowserActorUi" | "builtNodeStoreFor" | "windowMeasuresByWindowId" | "windowEngagementsByWindowId" | "windowTitlesById" | "windowUiByWindowId" | "primaryWorld3dOwner" | "world3dWindowViews" | "instanceFault" | "windowTreeContext" | "currentLocalDocumentOwner" | "currentDocumentRuntimeKey" | "onBrowserActorIntent" | "extraWindowInstances" | "registerRevision" | "browserActorUiVersion" | "panel">) {
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
  return { modeWindows };
}
