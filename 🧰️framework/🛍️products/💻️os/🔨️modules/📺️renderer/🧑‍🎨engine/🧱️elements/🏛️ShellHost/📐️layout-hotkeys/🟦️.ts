/** 📐️ The directory lane, panel chrome hotkeys, ui preference projection, action hotkeys, named layouts, mode changes, role switching, template drops and the display host.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useDirectoryLaneAndLayout` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { useCallback, useEffect, useMemo, useRef } from "react";
// #region 🔌️Adapters
import {
  type AppRole,
  latestWins,
  type LocalizedLabel,
  createNamedLayout,
  resolveLayoutForMode,
  type WindowLayout
} from "@semio-tech/framework";
import { type BackboneWorkerRequest, encodeBackboneWorkerRequest, type DirectoryEvent } from "@semio-tech/framework-os";
import {
  collectLocalPresenceWindowViewsV1,
  collectLocalActiveToolV1,
  localPresenceTypingFieldsV1,
  subscribeLocalPresenceWindowViewsV1
} from "../../👕️canvas-presence/🟦️.ts";
import { SURFACE_ROLE_CONTROL_IDS, roleSwitchTargetV1, stepModeIdV1 } from "../🔀️surface-switch/🟦️.ts";
import { setNamedLayout } from "../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import { decodeWorldProjectionTemplateId, worldProjectionSpecIconId, worldProjectionSpecLabel } from "@semio-tech/infinite-world-r3f";
import {
  applyUiThemeToRoot,
  clearUiThemeFromRoot,
  type IconName,
  insertWindowAtDropZone,
  type ModeCanvasDropTarget,
  setActiveUiTheme,
  useActionHotkey,
  useElementsSurfaceChrome,
  usePanelChromeHotkeys,
  type WindowTemplateDropPayload
} from "@semio-tech/ui-react";
import { sharedDocumentRoleSwitchRefusedV1 } from "../🧭️opening/🟦️.ts";
import { type ActiveSession } from "../../🐚️Shell/🟦️.tsx";
import { registerPendingWorldProjection } from "../../🌐️World3dHost/🟦️.tsx";
import {
  PRESENCE_HEARTBEAT_INTERVAL_MS,
  presenceLiveViewBeatDelayMsV1,
  applyFrameworkLayoutSeed,
  captureCurrentFrameworkLayout,
  presenceEphemeralSnapshotWithinBoundV1,
  shellLabelTextV1,
  presenceClientIdentity,
  presenceEphemeralPeerFieldsV1,
  resolveAppLabel,
  resolveFrameworkLayoutSeed,
  resolveManifestLabel,
  syncShellLabelLocale
} from "../../🛠️ShellHelpers/🟦️.tsx";
// #endregion 🔌️Adapters
import { type DisplayHostApi, useNamedLayoutHost } from "../../📌️ChromePanels/🟦️.tsx";
import { replayRefusalCodeV1, replayRefusalNoticeTextV1 } from "../📣️replay-refusal/🟦️.ts";
import { seededActiveWindowId } from "../🎚️shell-model/🟦️.tsx";
import { sessionIsSpaceIndex } from "../🧾️space-artifact-creation/🟦️.ts";
import { withLocalizedWindowKindLabels } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterTutorialOrchestration } from "../🎥️tutorial-orchestration/🟦️.ts";

export type ShellCoreAfterDirectoryLaneAndLayout = ShellCoreAfterTutorialOrchestration & ReturnType<typeof useDirectoryLaneAndLayout>;

export function useDirectoryLaneAndLayout(core: ShellCoreAfterTutorialOrchestration) {
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
  /** 📇️ Folds the current session's space index over its space's FULL directory history (the guest's contract). */
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
  // 🏠️🧳️ Once `hostSessionActive` is true, `session.app` *is* the host app, so its own self-declared
  // `controllerId` is the right value — no separate app-identity lookup needed.
  const studioSessionControllerId = hostSessionActive ? session?.app.controllerId : undefined;
  useEffect(() => {
    if (!hostSessionActive || !studioSessionControllerId || typeof window === "undefined") return;
    // 🪪️ §C3 — a resolved sign-in shows the real user in presence chrome instead of a random Guest.
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
    // 🐚️ terra-web-shellhost (finding 5) — one `latestWins` trigger PER open document (lazily created,
    // cached in `presenceBeatTriggersRef`): a slow `ephemeralSnapshot` for one document no longer
    // delays the heartbeat for every OTHER open document — each document's beat now runs independently
    // and concurrently. `latestWins` still protects a SINGLE document against overlapping beats if its
    // own snapshot takes longer than one heartbeat interval (collapses to at most one trailing follow-
    // up), the same guarantee the old shell-WIDE `publishing` flag gave the whole loop, now scoped per
    // document instead of stalling every document behind the slowest one. `run` reads
    // `backboneWorkerRef`/`openDocumentSessionsRef` fresh on every actual invocation (never captured at
    // trigger-creation time), so a still-open document's beat never serves a stale plugin/session pair.
    const beatOneDocument = (runtimeKey: string): Promise<void> => {
      let trigger = presenceBeatTriggersRef.current.get(runtimeKey);
      if (!trigger) {
        trigger = latestWins(async () => {
          const worker = backboneWorkerRef.current;
          const entry = openDocumentSessionsRef.current.get(runtimeKey);
          if (!worker || !entry) return;
          // 💓️ The beat is the shell's liveness signal for the HUMAN, not for the document
          // component, so it can never be hostage to one. `ephemeralSnapshot` reaches into the
          // plugin — in the browser through the nested browser-actor child — and a document whose
          // component is wedged answers it never: awaiting it bare left `latestWins`' single flight
          // permanently in flight, so this document never beat again and the hub's
          // `PRESENCE_LEASE_TTL_MS` lease lapsed under a socket that was still open. That is the
          // decay C3 §3.4 read as an empty roster in both browsers, reproduced deterministically on
          // the hub's own sockets in ticket 26/09/18 slice PR1
          // (`🗑️generated/pr1-before-hub-socket.txt` run B). Past the bound the peer beats with no
          // app-owned pack — present, with stale ephemerals dropped, which is exactly what the hub
          // publishes for a lapsed lease anyway.
          const snapshot = await presenceEphemeralSnapshotWithinBoundV1(entry.plugin, entry.session.instanceId);
          if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== entry.clientInstanceId) return;
          const request: BackboneWorkerRequest = {
            kind: "send",
            documentId: entry.documentId,
            clientInstanceId: entry.clientInstanceId,
            ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }),
            message: {
              kind: "presenceHeartbeat",
              // 🚧️ `cursor`/`viewport` are gone from `ArtifactPresencePeer`'s current wire shape (see
              // that type's own field list, `@semio-tech/framework-replication`) — `presenceCursorRef`
              // is still tracked (pointermove listener above) but has no landing spot on the wire
              // anymore; left tracked rather than torn out, since removing the listener is a design
              // call about whether cursor-sharing comes back, not a type fix.
              peer: {
                actor: shellActorIdRef.current,
                label: presenceIdentity.name,
                presencePack: snapshot?.presence,
                connectedAtMs: presenceConnectedAtMsRef.current,
                ...presenceEphemeralPeerFieldsV1(snapshot),
                ...((snapshot?.activeTool ?? collectLocalActiveToolV1(runtimeKey))
                  ? { activeTool: snapshot?.activeTool ?? collectLocalActiveToolV1(runtimeKey) }
                  : {}),
                // 👕️ Per-window camera/pointer published by Board2d/World3d/TextEditor hosts (throttled).
                views: collectLocalPresenceWindowViewsV1(runtimeKey),
                // ⌨️ Pending typing runs of TextEditor hosts: the ephemeral shared preview peers render at this caret.
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
      // 🧹️ Drops triggers for documents closed since the last tick — bounded growth instead of one
      // entry per document ever opened this session.
      for (const runtimeKey of presenceBeatTriggersRef.current.keys()) {
        if (!openDocumentSessionsRef.current.has(runtimeKey)) presenceBeatTriggersRef.current.delete(runtimeKey);
      }
      for (const runtimeKey of openDocumentSessionsRef.current.keys()) {
        void beatOneDocument(runtimeKey).catch((error) => console.error("[os-shell] presence heartbeat failed for document", runtimeKey, error));
      }
    };
    // 🖱️ A local window view (pointer, camera, caret) that changed beats at once, spaced by the live bound, so a peer's
    // cursor follows the pointer instead of the next interval beat (`💓️presence-liveness-v1`, `a-moving-pointer-beats-live`).
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
    // 📱️ All eight anchor hotkeys collapse onto the single mobile panel toggle on mobile. Same `shell.panelToggle`
    // commandId as the mouse-driven toggle in `buildPanelSelectionProps` (so keyboard/mouse fold together),
    // flagged `hotkey: true` in detail.
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

  // 🐚️ Unmount cleanup for the embedded (non-page-owning) case — a shell that painted its own root's
  // theme tokens must remove them on unmount, or a later, unrelated element reused at the same DOM
  // position (React/vite HMR reuse, or another shell's canvas-clone assets in a dev harness) would
  // silently inherit a stale theme's inline overrides. The page-owning case is intentionally left alone:
  // `document.documentElement` outlives any single shell's lifetime.
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
  useActionHotkey(
    "ui.nav.forward",
    useCallback(() => {
      if (canGoForward) goForward();
    }, [canGoForward, goForward]),
    undefined,
    [canGoForward, goForward],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.nav.up",
    useCallback(() => {
      if (canGoUp) goUp();
    }, [canGoUp, goUp]),
    undefined,
    [canGoUp, goUp],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.search.toggle",
    useCallback(() => dispatch({ type: "SET_SEARCH_OPEN", value: (open) => !open }), []),
    undefined,
    [],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.find.toggle",
    useCallback(() => dispatch({ type: "SET_FIND_OPEN", value: (open) => !open }), []),
    undefined,
    [],
    { overrides: uiKeybindingOverrides },
  );

  const applyNamedLayout = useCallback(
    (layout: WindowLayout) => {
      if (!session) return;
      const seeded = applyFrameworkLayoutSeed(layout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
      // 🪟️ Hand the just-computed instance list straight to the fetch rather than reading `extraWindowInstances`
      // state (which wouldn't reflect this dispatch until the next render) — every newly-seeded pane's own
      // body/measures/engagement gets fetched immediately instead of showing "missing window" until later.
      void refreshUi(session, { kind: "full" }, seeded.extraInstances).catch((error: unknown) => reportRefreshFault(session.pluginId, error));
    },
    [session, appLabelsOverlay, refreshUi, uiTerminology, uiLocale],
  );

  const applyModeChange = useCallback(
    (modeId: string) => {
      // 🛠️ Tools are scoped to a mode — switching modes always clears the active tool (and every
      // window's active utility), mirroring how a fresh mode starts with no utility pressed either.
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return current;
          const layout = resolveLayoutForMode(current.app, modeId);
          const nextSession: ActiveSession = { ...current, viewState: { ...current.viewState, activeModeId: modeId, activeToolId: undefined } };
          if (layout) {
            const seeded = applyFrameworkLayoutSeed(layout, withLocalizedWindowKindLabels(current.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
            extraWindowInstancesRef.current = seeded.extraInstances;
            extraWindowCounterRef.current = seeded.extraInstances.length;
            dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
            dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
            dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
            void refreshUi(nextSession, { kind: "full" }, seeded.extraInstances).catch((error: unknown) => reportRefreshFault(nextSession.pluginId, error));
          }
          return nextSession;
        },
      });
    },
    [appLabelsOverlay, refreshUi, uiTerminology, uiLocale],
  );

  /** 🎛️⌨️ Keyboard half of the navbar mode group: cycles `session.app.modes` by one step, wrapping.
   * Positional rather than one binding per mode id because mode ids are plugin-authored while
   * `SHELL_KEYBINDINGS` is a static framework table — see its own doc comment. Reads `sessionRef` so
   * the bound callback survives every session change without re-arming the listener. */
  const applyModeStep = useCallback(
    (step: 1 | -1) => {
      const current = sessionRef.current;
      if (!current) return;
      const next = stepModeIdV1(current.app.modes.map((mode) => mode.id), current.viewState.activeModeId ?? current.app.defaultModeId, step);
      if (next) applyModeChange(next);
    },
    [applyModeChange],
  );

  /** 👁️✏️⌨️ Switches the mounted session to this plugin's OTHER surface for the open document's dialect.
   * Every `session.app.role` gate (VCS check-in, mutation-command filtering, the viewer mutation
   * refusal, `canonicalSurfaceId` scope validation) reads the live session, so swapping `session.app`
   * flips all of them in one step — nothing here reproduces a role rule. */
  const switchToSessionRole = useCallback(
    async (role: AppRole): Promise<void> => {
      const current = sessionRef.current;
      if (!current) return;
      const apps = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === current.pluginId)?.manifest.apps ?? [];
      const target = roleSwitchTargetV1(apps, current.app.dialect, current.app.role, role);
      if (!target) return;
      const scope = [...openDocumentSessionsRef.current.values()].find((entry) => entry.session.pluginId === current.pluginId && entry.session.instanceId === current.instanceId)?.scope;
      if (scope !== undefined && sharedDocumentRoleSwitchRefusedV1(sharedSpaceAccessRef.current.get(scope.spaceId) ?? null, role)) {
        showTransientNoticeRef.current(replayRefusalNoticeTextV1("view-only-access", uiLocaleRef.current), "warning", replayRefusalCodeV1("view-only-access"));
        return;
      }
      await switchToPluginApp(current.pluginId, target.id,undefined,current);
    },
    [switchToPluginApp],
  );

  useActionHotkey(
    "ui.shell.mode.next",
    useCallback(() => applyModeStep(1), [applyModeStep]),
    { preventDefault: true },
    [applyModeStep],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    "ui.shell.mode.previous",
    useCallback(() => applyModeStep(-1), [applyModeStep]),
    { preventDefault: true },
    [applyModeStep],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    SURFACE_ROLE_CONTROL_IDS.editor,
    useCallback(() => void switchToSessionRole("editor").catch((switchError) => console.error("role switch to editor failed", switchError)), [switchToSessionRole]),
    { preventDefault: true },
    [switchToSessionRole],
    { overrides: uiKeybindingOverrides },
  );
  useActionHotkey(
    SURFACE_ROLE_CONTROL_IDS.viewer,
    useCallback(() => void switchToSessionRole("viewer").catch((switchError) => console.error("role switch to viewer failed", switchError)), [switchToSessionRole]),
    { preventDefault: true },
    [switchToSessionRole],
    { overrides: uiKeybindingOverrides },
  );

  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      // 🪟️ Display ▸ Windows now offers the FOCUSED program's kinds, and a spawned program's extra
      // panes are not modelled yet (`refreshSpawnedUi` fetches its declared kinds, not splits of
      // them). Dropping one would mint a window instance no guest was asked to render, so the drop is
      // refused loudly instead. Named in `📓️s5-spawned-app-windows-in-s.md` as the next owner's step.
      if (focusedSpawnedId !== null) {
        console.warn(`[os-shell] window template drop refused: ${focusedSpawnedId} is a spawned program and has no split lane yet (kind ${payload.windowKindId})`);
        return;
      }
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      const projectionSpec = decodeWorldProjectionTemplateId(payload.templateId);
      if (projectionSpec) registerPendingWorldProjection(instanceId, projectionSpec);
      const title = projectionSpec ? worldProjectionSpecLabel(projectionSpec) : resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale));
      const nextExtraInstances = [...extraWindowInstancesRef.current, { id: instanceId, windowKindId: payload.windowKindId, title }];
      extraWindowInstancesRef.current = nextExtraInstances;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: nextExtraInstances });
      if (projectionSpec) {
        dispatch({ type: "SET_WINDOW_TITLE", windowId: instanceId, title });
        dispatch({ type: "SET_WINDOW_ICON", windowId: instanceId, iconId: worldProjectionSpecIconId(projectionSpec) as IconName });
      }
      // 🪟️ The new split pane is its own window instance — fetch its body/measures/engagement right away
      // (see `applyNamedLayout`'s comment) rather than waiting for an unrelated action to trigger a refresh.
      void refreshUi(session, { kind: "full" }, nextExtraInstances).catch((error: unknown) => reportRefreshFault(session.pluginId, error));
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: (current) => {
          const base =
            current ??
            resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
          return insertWindowAtDropZone(base, instanceId, target);
        },
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
      noteShellCommand("shell.windowSplit", shellLabelTextV1("ui.shellCommand.windowSplit"), { windowKindId: payload.windowKindId, instanceId });
    },
    [appLabelsOverlay, refreshUi, session, focusedSpawnedId, noteShellCommand, uiTerminology, uiLocale],
  );

  const displayHostRef = useRef<DisplayHostApi | null>(null);
  // 🪟️ Display ▸ Windows offers the windows of the program that OWNS the canvas. It read
  // `session.app` unconditionally, so a spawned foreign editor's Display menu listed the landing
  // app's window instead of its own — the symptom S4 measured as
  // "framework.display.windows.s-home-main (Studios)" after spawning `draw`.
  const displayLayoutAppId = focusedApp?.id ?? "framework-os";
  const displayUserLayouts = useMemo(
    () => Object.entries(uiNamedLayouts[displayLayoutAppId] ?? {}).sort(([left], [right]) => left.localeCompare(right)).map(([layoutId, saved]) => createNamedLayout(layoutId, saved.label, saved.layout, "user")),
    [displayLayoutAppId, uiNamedLayouts],
  );
  const displayHost = useNamedLayoutHost({
    appId: displayLayoutAppId,
    windowKinds: focusedApp?.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)) })) ?? [],
    builtinLayouts: focusedApp?.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, extraWindowInstances, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    userLayouts: displayUserLayouts,
    onSaveUserLayout: (layoutId, label, layout) => commitUiPreferenceRef.current(setNamedLayout(displayLayoutAppId, layoutId, { label, layout })),
    onDeleteUserLayout: (layoutId) => commitUiPreferenceRef.current(setNamedLayout(displayLayoutAppId, layoutId, null)),
  });
  displayHostRef.current = displayHost;

  return { applyModeChange, switchToSessionRole, handleTemplateDrop, displayHostRef, displayHost };
  //#endregion FrameworkOsShell
}
