/** 🕹️ Shell store: the local interaction store, the root reducer, the selectors and the initial shell state. */
import { type ElementsSurfaceDevice, type UiLocale } from "@semio-tech/ui-react";
import { DEFAULT_MERGE_POLICY, type Conflict, type StoragePort } from "@semio-tech/framework";
import { EMPTY_INTERACTION_STATE, type InteractionState } from "@semio-tech/framework/interaction";
import { mergeRecordPreservingIdentity } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { EMPTY_APP_LABELS_OVERLAY, EMPTY_APP_CATALOGUE } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { readUiPreferences, resolveUiPreferences } from "../../🎚️UiPreferences/🟦️.ts";
import type { FrameworkOsDefaults, ResolvedShellLocks } from "../🔒️boot-and-locks/🟦️.ts";
import { initialPanels, type ShellState } from "../🗃️state-shapes/🟦️.ts";
import type { ShellAction } from "../🎛️shell-actions/🟦️.ts";
import { actionPaneReducer, commandPanelReducer, inferenceReducer, mergeReducer, overlayReducer, pluginRuntimeReducer, shellLayoutReducer, spawnedWindowReducer, syncReducer, tutorialReducer, uiPrefsReducer, windowUiReducer } from "../♻️slice-reducers/🟦️.ts";

//#region 🧮️ShellStore
//#region 🕹️LocalInteraction
/** 🕹️ The shell's own local interaction (selection, hover, mode and granularity per domain) — EPHEMERAL LOCAL state that
 * nothing the shell renders reads: the leftover inspection and the tutorial capture/replay read it through `get`, the
 * tutorial recorder through `subscribe`. It is no `ShellState` slice because a slice change re-renders the whole shell,
 * and every world hover publishes one (ticket 26/09/23 F3: the last whole-shell render per puzzle3d hover transition). */
export type LocalInteractionStoreV1 = {
  readonly get: () => InteractionState;
  readonly observe: (state: InteractionState) => void;
  readonly subscribe: (listener: () => void) => () => void;
};

/** 🧲️ An observation is a freshly decoded guest read, so it is structurally shared with the current state: an unchanged
 * one keeps the identity and notifies nobody, a hover-only one keeps every other field's identity. */
export function createLocalInteractionStoreV1(initial: InteractionState = EMPTY_INTERACTION_STATE): LocalInteractionStoreV1 {
  let current = initial;
  const listeners = new Set<() => void>();
  return {
    get: () => current,
    observe: (state) => {
      const next = mergeRecordPreservingIdentity(current, Object.entries(state)) as InteractionState;
      if (next === current) return;
      current = next;
      for (const listener of [...listeners]) listener();
    },
    subscribe: (listener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}

//#endregion 🕹️LocalInteraction
/** 🧵️ Root reducer for `FrameworkOsShell` — fans every action out to its owning slice reducer; slices that ignore an action's type return their input unchanged, so unrelated slices keep referential identity. */
export function shellReducer(state: ShellState, action: ShellAction): ShellState {
  const pluginRuntime = pluginRuntimeReducer(state.pluginRuntime, action);
  const previousSession = state.pluginRuntime.session;
  const nextSession = pluginRuntime.session;
  const ownerChanged = previousSession?.pluginId !== nextSession?.pluginId || previousSession?.instanceId !== nextSession?.instanceId || previousSession?.app.id !== nextSession?.app.id;
  const layout = shellLayoutReducer(state.layout, action);
  const next: ShellState = {
    pluginRuntime,
    windowUi: windowUiReducer(state.windowUi, action),
    spawnedWindow: spawnedWindowReducer(state.spawnedWindow, action),
    actionPane: actionPaneReducer(state.actionPane, action),
    commandPanel: commandPanelReducer(state.commandPanel, action),
    layout: ownerChanged ? { ...layout, windowTitlesById: {}, windowIconsById: {} } : layout,
    overlays: overlayReducer(state.overlays, action),
    tutorial: tutorialReducer(state.tutorial, action),
    uiPrefs: uiPrefsReducer(state.uiPrefs, action),
    sync: syncReducer(state.sync, action),
    inference: inferenceReducer(state.inference, action),
    merge: mergeReducer(state.merge, action),
  };
  // 🪞️ An action no slice answered with a new value is a no-op: return the SAME state so
  // `useReducer` bails out instead of reconciling the whole shell (see `withField`).
  return shellStateUnchanged(state, next) ? state : next;
}

/** 🪞️ True when every slice kept its identity. */
export function shellStateUnchanged(previous: ShellState, next: ShellState): boolean {
  return (Object.keys(next) as (keyof ShellState)[]).every((key) => Object.is(previous[key], next[key]));
}

//#region selectors
/**
 * 📱️ The device this shell paints for: the MEASURED viewport device wins whenever it is narrower than a
 * desktop, and only a desktop-width viewport falls through to the user's stored `uiLayout` preference.
 *
 * `measured` comes from `useUiDevice()` — `🖱️ui/📱️device/🟦️.ts`'s three-way policy, the same thresholds
 * the wgpu dock compares its `screen_w` against. It used to be a bare `mobile: boolean`, so the 768-1023
 * band painted the DESKTOP layout and `"tablet"` was a value only a user who found the settings row ever
 * saw, contradicting AGENTS.md's explicit desktop → mobile → tablet priority. Pinning `"tablet"` in
 * settings still works, and still only affects a desktop-width viewport (pinning a wider layout on a
 * phone is exactly the case that has no room for it).
 */
export const selectUiDevice = (state: ShellState, measured: ElementsSurfaceDevice): ElementsSurfaceDevice => (measured === "desktop" ? state.uiPrefs.uiLayout : measured);

/** ⚖️ Open `Quarantined` conflicts — "a peer batch is being held" (contract freeze §C6/§C9's
 * `ShellSync` quarantine indicator) is exactly "this list is non-empty", never a separately stored
 * flag (see {@link MergeState}'s doc comment). */
export const selectQuarantinedConflicts = (state: ShellState): readonly Conflict[] => state.merge.conflicts.filter((conflict) => conflict.kind.kind === "quarantined" && conflict.status === "open");

/** ⚖️ Open conflicts of either kind, for the `📌️ChromePanels` Conflicts panel. */
export const selectOpenConflicts = (state: ShellState): readonly Conflict[] => state.merge.conflicts.filter((conflict) => conflict.status === "open");

//#endregion selectors
/** 🌱️ Builds the starting `ShellState` for `FrameworkOsShell`, mirroring exactly what each migrated `useState` used to initialize to (including reads from the shell's own storage for UI prefs). */
export function initialShellState(_props: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly storage: StoragePort;
  readonly locale: UiLocale;
}): ShellState {
  const locks = _props.locks ?? {};
  const defaults = _props.defaults ?? {};
  const storage = _props.storage;
  const preferences = resolveUiPreferences(readUiPreferences(storage), {
    appearance: "system",
    layout: "desktop",
    driverId: "default",
    locale: _props.locale,
    terminology: "native",
    themeId: "semio",
  });
  return {
    pluginRuntime: { loadedPlugins: [], pluginStatusById: {}, pluginSupervisorById: {}, session: null, error: null, sessionFault: null, instanceFault: null },
    windowUi: { windowUiByWindowId: {}, windowEngagementsByWindowId: {}, windowMeasuresByWindowId: {}, toolMeasuresByToolId: {}, panelBodyStoreByKey: {}, appLabelsOverlay: EMPTY_APP_LABELS_OVERLAY, appCatalogue: EMPTY_APP_CATALOGUE },
    spawnedWindow: { spawnedWindowActivityByWindowId: {}, spawnedWindowFault: null, spawnedWindowEngagements: {}, spawnedWindowMeasures: {} },
    actionPane: { foldedByWindowId: {}, expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: {}, activeToolId: null },
    commandPanel: { expandedCommandId: null, stagedArgsByCommandId: {} },
    layout: {
      panels: initialPanels(),
      dockOverride: null,
      panelPathMemory: {},
      treeOpenStates: {},
      activeWindowId: null,
      shellLayout: null,
      activeExampleId: locks.exampleId ?? defaults.exampleId ?? "",
      mobilePanelPath: [],
      mobilePanelVisible: false,
      extraWindowInstances: [],
      windowTitlesById: {},
      windowIconsById: {},
    },
    overlays: { searchOpen: false, findOpen: false, introductionStepIndex: null, introductionAutoStartedKeys: [], introductionCompletedInteractions: [], dialog: null, transientNotice: null, openWithFocusRole: null },
    tutorial: { activeTutorialId: null, playing: false, rate: 1, muted: false, captionsOn: true, recording: false, deviated: false },
    uiPrefs: {
      uiAppearance: locks.appearance ?? preferences.appearance,
      uiLayout: preferences.layout,
      uiDriverId: preferences.driverId,
      uiCustomDrivers: preferences.customDrivers,
      uiDriverDraft: null,
      uiLocale: locks.locale ?? preferences.locale,
      uiTerminology: locks.terminology ?? preferences.terminology,
      uiThemeId: locks.themeId ?? preferences.themeId,
      uiCustomThemes: preferences.customThemes,
      uiThemeDraft: null,
      uiKeybindingOverrides: preferences.keybindingOverrides,
    },
    sync: { syncBackboneUri: null, syncCardKind: null, syncDraftPath: "", syncStatusByDocumentId: {} },
    inference: { portByRuntimeKey: {}, operationEpoch: null, operationRuntimeKey: null },
    merge: { mergePolicy: DEFAULT_MERGE_POLICY, conflicts: [], selectedConflictId: null },
  };
}
//#endregion 🧮️ShellStore
