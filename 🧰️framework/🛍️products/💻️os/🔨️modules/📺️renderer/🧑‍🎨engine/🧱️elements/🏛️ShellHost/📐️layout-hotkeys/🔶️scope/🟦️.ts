/** 🔶️ The scope phase of useDirectoryLaneAndLayout. */
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { latestWins } from "@semio-tech/framework";
import { type BackboneWorkerRequest, encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type DirectoryEvent } from "@semio-tech/framework/directory/schema";
import { collectLocalPresenceWindowViewsV1, collectLocalActiveToolV1, localPresenceTypingFieldsV1, subscribeLocalPresenceWindowViewsV1 } from "../../../👕️canvas-presence/🟦️.ts";
import { applyUiThemeToRoot, clearUiThemeFromRoot, setActiveUiTheme, useActionHotkey, useElementsSurfaceChrome, usePanelChromeHotkeys } from "@semio-tech/ui-react";
import { PRESENCE_HEARTBEAT_INTERVAL_MS, presenceLiveViewBeatDelayMsV1, presenceEphemeralSnapshotWithinBoundV1, presenceClientIdentity, presenceEphemeralPeerFieldsV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/👥️presence-identity/🟦️.ts";
import { shellLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { syncShellLabelLocale } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { sessionIsSpaceIndex } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterTutorialOrchestration } from "../../🎥️tutorial-orchestration/🟦️.ts";

/** 🔷 Builds scope, hostConfig, hostMode, mobile of useDirectoryLaneAndLayout. */
export function useScopeState({ core }: { readonly core: ShellCoreAfterTutorialOrchestration }) {
  const {
    scope, hostConfig, hostMode, mobile, ephemeral, dispatch, session, uiAppearance, uiLocale, uiTerminology,
    uiKeybindingOverrides, hostAppId, appLabelsOverlay, shellLayout, extraWindowInstances, uiNamedLayouts,
    commitUiPreferenceRef, extraWindowCounterRef, extraWindowInstancesRef, sessionRef, uiDevice, uiTheme, uiDriver,
    backboneWorkerRef, shellActorIdRef, identity, identityRef, uiLocaleRef, dispatchDirectoryEventsRef,
    showTransientNoticeRef, presenceConnectedAtMsRef, presenceBeatTriggersRef, openDocumentSessionsRef,
    spaceDirectoryHistoriesRef, foldSpaceDirectoryRef, sharedSpaceAccessRef, loadedPluginsRef, reportRefreshFault,
    canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, focusedApp, focusedSpawnedId, refreshUi,
    switchToPluginApp, noteShellCommand, onActionRef
  } = core;
  //#region FrameworkOsShell
  //#region 🔖️DirectoryLane
  /** 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C6 — relays one batch of
   * `DirectoryEvent`s to the mounted Space index session as its `foldDirectoryEvents` view action, the
   * only app that still folds raw events. Home never receives them: its projection has exactly one
   * writer, the receipt-sealed `applyDirectoryEventPage` lane, and a live event only WAKES the worker's
   * acknowledged stream into fetching the next sealed page. A raw fold there advanced the projection
   * cursor past its receipt, so the next page answered `s.home.directory-event-page-frontier-race`
   * (ticket 26/09/18 U5 §6b). The batch travels as the `eventsJson: string` field every Rust
   * `command_from_action` reads (w4-h root-cause fix #1). */
  const dispatchDirectoryEventBatch = useCallback(
    (events: readonly DirectoryEvent[]) => {
      if (events.length === 0 || !hostConfig) return;
      const current = sessionRef.current;
      if (!sessionIsSpaceIndex(current, loadedPluginsRef.current)) return;
      onActionRef.current({ controllerId: current.app.controllerId, action: "foldDirectoryEvents", args: { eventsJson: JSON.stringify(events) } });
    },
    [hostConfig],
  );
  dispatchDirectoryEventsRef.current = dispatchDirectoryEventBatch;
  foldSpaceDirectoryRef.current = () => {
    const current = sessionRef.current;
    if (current === null) return;
    for (const [runtimeKey, entry] of openDocumentSessionsRef.current) {
      if (entry.session.pluginId !== current.pluginId || entry.session.instanceId !== current.instanceId) continue;
      const history = spaceDirectoryHistoriesRef.current.get(runtimeKey);
      if (history !== undefined && history.events().length > 0) dispatchDirectoryEventBatch(history.events());
      return;
    }
  };
  //#endregion 🔖️DirectoryLane

  const hostSessionActive = hostMode && session?.app.id === hostAppId;
  const studioSessionControllerId = hostSessionActive ? session?.app.controllerId : undefined;
  useEffect(() => {
    if (!hostSessionActive || !studioSessionControllerId || typeof window === "undefined") return;
    const presenceIdentity = presenceClientIdentity(ephemeral, identityRef.current ? { clientId: shellActorIdRef.current, name: identityRef.current.displayName } : undefined);
    const beat = () => onActionRef.current({ controllerId: studioSessionControllerId, action: "presenceHeartbeat", args: presenceIdentity });
    const initial = window.setTimeout(beat, 1000);
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearTimeout(initial);
      window.clearInterval(timer);
    };
  }, [hostSessionActive, studioSessionControllerId, ephemeral, identity]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    const presenceIdentity = presenceClientIdentity(ephemeral, identityRef.current ? { clientId: shellActorIdRef.current, name: identityRef.current.displayName } : undefined);
    const beatOneDocument = (runtimeKey: string): Promise<void> => {
      let trigger = presenceBeatTriggersRef.current.get(runtimeKey);
      if (!trigger) {
        trigger = latestWins(async () => {
          const worker = backboneWorkerRef.current;
          const entry = openDocumentSessionsRef.current.get(runtimeKey);
          if (!worker || !entry) return;
          const snapshot = await presenceEphemeralSnapshotWithinBoundV1(entry.plugin, entry.session.instanceId);
          if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== entry.clientInstanceId) return;
          const request: BackboneWorkerRequest = {
            kind: "send",
            documentId: entry.documentId,
            clientInstanceId: entry.clientInstanceId,
            ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }),
            message: {
              kind: "presenceHeartbeat",
              peer: {
                actor: shellActorIdRef.current,
                label: presenceIdentity.name,
                presencePack: snapshot?.presence,
                connectedAtMs: presenceConnectedAtMsRef.current,
                ...presenceEphemeralPeerFieldsV1(snapshot),
                ...((snapshot?.activeTool ?? collectLocalActiveToolV1(runtimeKey))
                  ? { activeTool: snapshot?.activeTool ?? collectLocalActiveToolV1(runtimeKey) }
                  : {}),
                views: collectLocalPresenceWindowViewsV1(runtimeKey),
                ...localPresenceTypingFieldsV1(runtimeKey),
              },
            },
          };
          worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
        });
        presenceBeatTriggersRef.current.set(runtimeKey, trigger);
      }
      return trigger();
    };
    let lastBeatAtMs = 0;
    let liveViewBeat: ReturnType<typeof setTimeout> | null = null;
    const beat = () => {
      lastBeatAtMs = Date.now();
      for (const runtimeKey of presenceBeatTriggersRef.current.keys()) {
        if (!openDocumentSessionsRef.current.has(runtimeKey)) presenceBeatTriggersRef.current.delete(runtimeKey);
      }
      for (const runtimeKey of openDocumentSessionsRef.current.keys()) {
        void beatOneDocument(runtimeKey).catch((error) => console.error("[os-shell] presence heartbeat failed for document", runtimeKey, error));
      }
    };
    const unsubscribeLiveViews = subscribeLocalPresenceWindowViewsV1(() => {
      if (liveViewBeat !== null) return;
      liveViewBeat = setTimeout(() => {
        liveViewBeat = null;
        beat();
      }, presenceLiveViewBeatDelayMsV1(lastBeatAtMs, Date.now()));
    });
    beat();
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      unsubscribeLiveViews();
      if (liveViewBeat !== null) clearTimeout(liveViewBeat);
      window.clearInterval(timer);
      presenceBeatTriggersRef.current.clear();
    };
  }, [ephemeral, identity]);

  usePanelChromeHotkeys({
    onToggle: (anchor) => {
      if (mobile) dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: (visible) => !visible });
      else dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: (visible) => !visible });
      noteShellCommand("shell.panelToggle", shellLabelTextV1("ui.shellCommand.panelToggle"), { anchor: mobile ? undefined : anchor, hotkey: true });
    },
  });

  useElementsSurfaceChrome({ appearance: uiAppearance, device: uiDevice, driver: uiDriver }, scope.rootRef.current ?? undefined);

  //#region 🎨️ Ui preference projection effects
  useEffect(() => {
    void scope.i18n.changeLanguage(uiLocale);
    syncShellLabelLocale(uiLocale);
    if (scope.ownsPage) {
      if (typeof document !== "undefined") document.documentElement.lang = uiLocale;
    } else if (scope.rootRef.current) {
      scope.rootRef.current.lang = uiLocale;
    }
    if (scope.ownsPage) {
      setActiveUiTheme(uiTheme);
    } else if (scope.rootRef.current) {
      applyUiThemeToRoot(scope.rootRef.current, uiTheme);
    }
  }, [uiLocale, uiTheme, scope]);

  useEffect(() => {
    if (scope.ownsPage) return;
    return () => {
      if (scope.rootRef.current) clearUiThemeFromRoot(scope.rootRef.current);
    };
  }, [scope]);
  //#endregion

  useActionHotkey(
    "ui.nav.back",
    useCallback(() => {
      if (canGoBack) goBack();
    }, [canGoBack, goBack]),
    undefined,
    [canGoBack, goBack],
    { overrides: uiKeybindingOverrides },
  );
  return { canGoForward, goForward, uiKeybindingOverrides, canGoUp, goUp, dispatch, session, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, refreshUi, reportRefreshFault, sessionRef, loadedPluginsRef, openDocumentSessionsRef, sharedSpaceAccessRef, showTransientNoticeRef, uiLocaleRef, switchToPluginApp, focusedSpawnedId, noteShellCommand, focusedApp, uiNamedLayouts, shellLayout, extraWindowInstances, commitUiPreferenceRef };
}
