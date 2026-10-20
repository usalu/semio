/** 🔸️ The apply named layout phase of useDirectoryLaneAndLayout. */
import { useCallback } from "react";
import { type AppRole, resolveLayoutForMode, type WindowLayout } from "@semio-tech/framework";
import { SURFACE_ROLE_CONTROL_IDS, roleSwitchTargetV1, stepModeIdV1 } from "../../🔀️surface-switch/🟦️.ts";
import { useActionHotkey } from "@semio-tech/ui-react";
import { sharedDocumentRoleSwitchRefusedV1 } from "../../🧭️opening/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { applyFrameworkLayoutSeed } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { replayRefusalCodeV1, replayRefusalNoticeTextV1 } from "../../📣️replay-refusal/🟦️.ts";
import { seededActiveWindowId } from "../../🎚️shell-model/🟦️.tsx";
import { withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🔹 Builds applyNamedLayout, applyModeChange, applyModeStep, switchToSessionRole of useDirectoryLaneAndLayout. */
export function useApplyNamedLayoutState({ canGoForward, goForward, uiKeybindingOverrides, canGoUp, goUp, dispatch, session, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, refreshUi, reportRefreshFault, sessionRef, loadedPluginsRef, openDocumentSessionsRef, sharedSpaceAccessRef, showTransientNoticeRef, uiLocaleRef, switchToPluginApp }: Pick<ReturnType<typeof useScopeState>, "canGoForward" | "goForward" | "uiKeybindingOverrides" | "canGoUp" | "goUp" | "dispatch" | "session" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "extraWindowInstancesRef" | "extraWindowCounterRef" | "refreshUi" | "reportRefreshFault" | "sessionRef" | "loadedPluginsRef" | "openDocumentSessionsRef" | "sharedSpaceAccessRef" | "showTransientNoticeRef" | "uiLocaleRef" | "switchToPluginApp">) {
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
      void refreshUi(session, { kind: "full" }, seeded.extraInstances).catch((error: unknown) => reportRefreshFault(session.pluginId, error));
    },
    [session, appLabelsOverlay, refreshUi, uiTerminology, uiLocale],
  );

  const applyModeChange = useCallback(
    (modeId: string) => {
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
  return { applyNamedLayout, applyModeChange, switchToSessionRole };
}
