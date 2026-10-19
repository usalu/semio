/** ♻️ Shell slice reducers: the loaded program upsert and the reducer of every store slice. */
import { ANCHORS, uiDataLabel, type Anchor, type WindowLayoutNode } from "@semio-tech/ui-react";
import { DEFAULT_PANEL_WIDTH_PX } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import type { LoadedProgramState } from "../📦️program-sessions/🟦️.ts";
import { type ActionPaneState, actionStageKey, type CommandPanelState, type InferenceState, type MergeState, type OverlayState, type PanelState, type PluginRuntimeState, resolveUpdatable, type ShellLayoutState, type SpawnedWindowState, type SyncState, type TutorialState, type UiPrefsState, type WindowUiState, withField } from "../🗃️state-shapes/🟦️.ts";
import type { ShellAction } from "../🎛️shell-actions/🟦️.ts";

//#region 🧮️ShellStore
//#region slice reducers
/** 🔌️ The loaded set with `entry` in its plugin's place (appended when new) — the one upsert the reducer applies,
 * shared with callers that must read the set before the next render commits it. */
export function upsertLoadedProgramV1(loaded: readonly LoadedProgramState[], entry: LoadedProgramState): readonly LoadedProgramState[] {
  const index = loaded.findIndex((candidate) => candidate.handle.pluginId === entry.handle.pluginId);
  return index === -1 ? [...loaded, entry] : loaded.map((candidate, i) => (i === index ? entry : candidate));
}

export function pluginRuntimeReducer(state: PluginRuntimeState, action: ShellAction): PluginRuntimeState {
  switch (action.type) {
    case "UPSERT_LOADED_PLUGIN":
      return { ...state, loadedPlugins: upsertLoadedProgramV1(state.loadedPlugins, action.value) };
    case "REMOVE_LOADED_PLUGIN":
      return { ...state, loadedPlugins: state.loadedPlugins.filter((entry) => entry.handle.pluginId !== action.pluginId) };
    case "SET_PLUGIN_STATUS":
      return { ...state, pluginStatusById: { ...state.pluginStatusById, [action.pluginId]: action.value } };
    case "SET_PLUGIN_SUPERVISOR":
      return { ...state, pluginSupervisorById: { ...state.pluginSupervisorById, [action.pluginId]: action.value } };
    case "SET_SESSION":
      return withField(state, "session", resolveUpdatable(action.value, state.session));
    case "SET_ERROR":
      return { ...state, error: resolveUpdatable(action.value, state.error), sessionFault: action.fault ?? null };
    case "SET_INSTANCE_FAULT":
      return state.instanceFault === action.value ? state : { ...state, instanceFault: action.value };
    default:
      return state;
  }
}

export function windowUiReducer(state: WindowUiState, action: ShellAction): WindowUiState {
  switch (action.type) {
    case "SET_WINDOW_UI_BY_WINDOW_ID":
      return withField(state, "windowUiByWindowId", resolveUpdatable(action.value, state.windowUiByWindowId));
    case "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID":
      return withField(state, "windowEngagementsByWindowId", resolveUpdatable(action.value, state.windowEngagementsByWindowId));
    case "SET_WINDOW_MEASURES_BY_WINDOW_ID":
      return withField(state, "windowMeasuresByWindowId", resolveUpdatable(action.value, state.windowMeasuresByWindowId));
    case "SET_TOOL_MEASURES_BY_TOOL_ID":
      return withField(state, "toolMeasuresByToolId", resolveUpdatable(action.value, state.toolMeasuresByToolId));
    case "SET_PANEL_BODY_STORE_BY_KEY":
      return withField(state, "panelBodyStoreByKey", resolveUpdatable(action.value, state.panelBodyStoreByKey));
    case "SET_APP_LABELS_OVERLAY":
      return withField(state, "appLabelsOverlay", resolveUpdatable(action.value, state.appLabelsOverlay));
    case "SET_APP_CATALOGUE":
      return withField(state, "appCatalogue", resolveUpdatable(action.value, state.appCatalogue));
    default:
      return state;
  }
}

export function spawnedWindowReducer(state: SpawnedWindowState, action: ShellAction): SpawnedWindowState {
  switch (action.type) {
    case "SET_SPAWNED_WINDOW_ACTIVITY": {
      const spawnedWindowActivityByWindowId = resolveUpdatable(action.value, state.spawnedWindowActivityByWindowId);
      const spawnedWindowFault = action.fault ?? null;
      return Object.is(spawnedWindowActivityByWindowId, state.spawnedWindowActivityByWindowId) && Object.is(spawnedWindowFault, state.spawnedWindowFault) ? state : { ...state, spawnedWindowActivityByWindowId, spawnedWindowFault };
    }
    case "SET_SPAWNED_WINDOW_ENGAGEMENTS":
      return withField(state, "spawnedWindowEngagements", resolveUpdatable(action.value, state.spawnedWindowEngagements));
    case "SET_SPAWNED_WINDOW_MEASURES":
      return withField(state, "spawnedWindowMeasures", resolveUpdatable(action.value, state.spawnedWindowMeasures));
    default:
      return state;
  }
}

/** 🧰️ Reducer for the per-window Action rail slice (P1–P5). Every case preserves referential identity when nothing actually changes so downstream memos can bail. */
export function actionPaneReducer(state: ActionPaneState, action: ShellAction): ActionPaneState {
  switch (action.type) {
    case "SET_ACTION_PANE_FOLDED": {
      if (state.foldedByWindowId[action.windowId] === action.value) return state;
      return { ...state, foldedByWindowId: { ...state.foldedByWindowId, [action.windowId]: action.value } };
    }
    case "SET_ACTION_PANE_EXPANDED": {
      if ((state.expandedByWindowId[action.windowId] ?? null) === action.value) return state;
      return { ...state, expandedByWindowId: { ...state.expandedByWindowId, [action.windowId]: action.value } };
    }
    case "STAGE_ACTION_ARG": {
      const key = actionStageKey(action.windowId, action.actionId);
      const current = state.stagedArgsByKey[key] ?? {};
      if (Object.prototype.hasOwnProperty.call(current, action.argId) && current[action.argId] === action.value) return state;
      return { ...state, stagedArgsByKey: { ...state.stagedArgsByKey, [key]: { ...current, [action.argId]: action.value } } };
    }
    case "RESET_ACTION_ARGS": {
      const key = actionStageKey(action.windowId, action.actionId);
      if (!Object.prototype.hasOwnProperty.call(state.stagedArgsByKey, key)) return state;
      const next = { ...state.stagedArgsByKey };
      delete next[key];
      return { ...state, stagedArgsByKey: next };
    }
    case "SET_ACTIVE_UTILITY": {
      if (Object.hasOwn(state.activeUtilityByWindowId, action.windowId) && state.activeUtilityByWindowId[action.windowId] === action.utilityId) return state;
      return { ...state, activeUtilityByWindowId: { ...state.activeUtilityByWindowId, [action.windowId]: action.utilityId } };
    }
    case "SET_ACTIVE_TOOL":
      if (state.activeToolId === action.toolId) return state;
      return { ...state, activeToolId: action.toolId };
    case "APPLY_TUTORIAL_UI_SNAPSHOT":
      return { ...state, activeUtilityByWindowId: action.snapshot.activeUtilityByWindowId, activeToolId: action.snapshot.activeToolId };
    default:
      return state;
  }
}

/** 🎛️ Reducer for the command palette's arg-expansion/staging slice — category active/fold state is the `bottom-middle` anchor's own generic `SET_PANEL_VISIBLE`/`SET_PANEL_PATH` (see `shellLayoutReducer`), not handled here. */
export function commandPanelReducer(state: CommandPanelState, action: ShellAction): CommandPanelState {
  switch (action.type) {
    case "SET_COMMAND_EXPANDED": {
      if (state.expandedCommandId === action.value) return state;
      return withField(state, "expandedCommandId", action.value);
    }
    case "STAGE_COMMAND_ARG": {
      const current = state.stagedArgsByCommandId[action.commandId] ?? {};
      if (Object.prototype.hasOwnProperty.call(current, action.argId) && current[action.argId] === action.value) return state;
      return { ...state, stagedArgsByCommandId: { ...state.stagedArgsByCommandId, [action.commandId]: { ...current, [action.argId]: action.value } } };
    }
    case "RESET_COMMAND_ARGS": {
      if (!Object.prototype.hasOwnProperty.call(state.stagedArgsByCommandId, action.commandId)) return state;
      const next = { ...state.stagedArgsByCommandId };
      delete next[action.commandId];
      return { ...state, stagedArgsByCommandId: next };
    }
    default:
      return state;
  }
}

/** 🏷️ Updates the named leaf while retaining every unaffected branch and its layout geometry. */
function withWindowLayoutTitle(node: WindowLayoutNode, windowId: string, title: string): WindowLayoutNode {
  if (node.kind === "window") return node.id === windowId && node.title !== title ? { ...node, title: uiDataLabel(title) } : node;
  const children = node.children.map(child => withWindowLayoutTitle(child, windowId, title));
  return children.some((child, index) => child !== node.children[index]) ? { ...node, children } as WindowLayoutNode : node;
}

export function shellLayoutReducer(state: ShellLayoutState, action: ShellAction): ShellLayoutState {
  switch (action.type) {
    case "SET_PANEL_VISIBLE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], visible: resolveUpdatable(action.value, state.panels[action.anchor].visible) } } };
    case "SET_PANEL_SIZE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], size: resolveUpdatable(action.value, state.panels[action.anchor].size) } } };
    case "SET_PANEL_PATH":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], path: resolveUpdatable(action.value, state.panels[action.anchor].path) } } };
    case "SET_DOCK_OVERRIDE":
      return withField(state, "dockOverride", action.value);
    case "SET_PANEL_PATH_MEMORY":
      return withField(state, "panelPathMemory", resolveUpdatable(action.value, state.panelPathMemory));
    case "SET_TREE_OPEN_STATE":
      return { ...state, treeOpenStates: { ...state.treeOpenStates, [action.id]: action.open } };
    case "HYDRATE_DOCK_UI": {
      if (!action.value) return state;
      const panels = { ...state.panels };
      for (const anchor of ANCHORS) {
        const saved = action.value.anchors[anchor];
        if (!saved) continue;
        panels[anchor] = {
          visible: saved.visible ?? panels[anchor].visible,
          size: saved.size ?? panels[anchor].size,
          path: saved.path ?? panels[anchor].path,
        };
      }
      return { ...state, panels, panelPathMemory: action.value.pathMemory ?? state.panelPathMemory, treeOpenStates: action.value.treeOpen ?? state.treeOpenStates };
    }
    case "RESET_DOCK": {
      const panels = {} as Record<Anchor, PanelState>;
      for (const anchor of ANCHORS) panels[anchor] = { visible: false, size: DEFAULT_PANEL_WIDTH_PX, path: [] };
      return { ...state, dockOverride: null, panels, panelPathMemory: {}, treeOpenStates: {} };
    }
    case "SET_ACTIVE_WINDOW_ID":
      return withField(state, "activeWindowId", resolveUpdatable(action.value, state.activeWindowId));
    case "SET_SHELL_LAYOUT":
      return withField(state, "shellLayout", resolveUpdatable(action.value, state.shellLayout));
    case "SET_ACTIVE_EXAMPLE_ID":
      return withField(state, "activeExampleId", resolveUpdatable(action.value, state.activeExampleId));
    case "SET_MOBILE_PANEL_PATH":
      return withField(state, "mobilePanelPath", resolveUpdatable(action.value, state.mobilePanelPath));
    case "SET_MOBILE_PANEL_VISIBLE":
      return withField(state, "mobilePanelVisible", resolveUpdatable(action.value, state.mobilePanelVisible));
    case "SET_EXTRA_WINDOW_INSTANCES":
      return withField(state, "extraWindowInstances", resolveUpdatable(action.value, state.extraWindowInstances));
    case "SET_WINDOW_TITLE": {
      const windowTitlesById = { ...state.windowTitlesById, [action.windowId]: action.title };
      const extraWindowInstances = state.extraWindowInstances.map((entry) => (entry.id === action.windowId ? { ...entry, title: action.title } : entry));
      const shellLayout = state.shellLayout ? withWindowLayoutTitle(state.shellLayout, action.windowId, action.title) : null;
      return { ...state, windowTitlesById, extraWindowInstances, shellLayout };
    }
    case "SET_WINDOW_ICON": {
      const windowIconsById = { ...state.windowIconsById, [action.windowId]: action.iconId };
      return { ...state, windowIconsById };
    }
    case "APPLY_TUTORIAL_UI_SNAPSHOT": {
      const { snapshot } = action;
      const panels = { ...state.panels };
      for (const anchor of ANCHORS) {
        const patch = snapshot.panelPatches[anchor];
        if (!patch) continue;
        panels[anchor] = { ...panels[anchor], visible: patch.visible, path: patch.path };
      }
      return { ...state, activeWindowId: snapshot.activeWindowId, shellLayout: snapshot.shellLayout, extraWindowInstances: snapshot.extraWindowInstances, treeOpenStates: snapshot.treeOpenStates, panels };
    }
    default:
      return state;
  }
}

export function overlayReducer(state: OverlayState, action: ShellAction): OverlayState {
  switch (action.type) {
    case "SET_SEARCH_OPEN":
      return withField(state, "searchOpen", resolveUpdatable(action.value, state.searchOpen));
    case "SET_FIND_OPEN":
      return withField(state, "findOpen", resolveUpdatable(action.value, state.findOpen));
    case "AUTO_START_INTRODUCTION":
      if (state.introductionAutoStartedKeys.includes(action.key)) return state;
      return {
        ...state,
        introductionStepIndex: 0,
        introductionAutoStartedKeys: [...state.introductionAutoStartedKeys, action.key],
        introductionCompletedInteractions: [],
      };
    case "SET_INTRODUCTION_STEP": {
      const nextIndex = resolveUpdatable(action.value, state.introductionStepIndex);
      return { ...state, introductionStepIndex: nextIndex, introductionCompletedInteractions: [] };
    }
    case "COMPLETE_INTRODUCTION_INTERACTION":
      return state.introductionCompletedInteractions.includes(action.index)
        ? state
        : { ...state, introductionCompletedInteractions: [...state.introductionCompletedInteractions, action.index] };
    case "SET_TUTORIAL":
      return action.value != null && state.introductionStepIndex != null ? { ...state, introductionStepIndex: null, introductionCompletedInteractions: [] } : state;
    case "SET_DIALOG":
      return withField(state, "dialog", action.value);
    case "CLOSE_DIALOG":
      return state.dialog?.openingId === action.openingId ? { ...state, dialog: null } : state;
    case "SET_TRANSIENT_NOTICE":
      return withField(state, "transientNotice", action.value);
    case "SET_OPEN_WITH_FOCUS_ROLE":
      return withField(state, "openWithFocusRole", action.value);
    case "APPLY_TUTORIAL_UI_SNAPSHOT":
      return { ...state, dialog: action.snapshot.dialog, searchOpen: action.snapshot.commandPanelOpen };
    default:
      return state;
  }
}

export function uiPrefsReducer(state: UiPrefsState, action: ShellAction): UiPrefsState {
  switch (action.type) {
    case "SET_UI_APPEARANCE":
      return withField(state, "uiAppearance", resolveUpdatable(action.value, state.uiAppearance));
    case "SET_UI_LAYOUT":
      return withField(state, "uiLayout", resolveUpdatable(action.value, state.uiLayout));
    case "SET_UI_DRIVER_ID":
      return withField(state, "uiDriverId", resolveUpdatable(action.value, state.uiDriverId));
    case "SET_UI_CUSTOM_DRIVERS":
      return withField(state, "uiCustomDrivers", resolveUpdatable(action.value, state.uiCustomDrivers));
    case "SET_UI_DRIVER_DRAFT":
      return withField(state, "uiDriverDraft", resolveUpdatable(action.value, state.uiDriverDraft));
    case "SET_UI_LOCALE":
      return withField(state, "uiLocale", resolveUpdatable(action.value, state.uiLocale));
    case "SET_UI_TERMINOLOGY":
      return withField(state, "uiTerminology", resolveUpdatable(action.value, state.uiTerminology));
    case "SET_UI_THEME_ID":
      return withField(state, "uiThemeId", resolveUpdatable(action.value, state.uiThemeId));
    case "SET_UI_CUSTOM_THEMES":
      return withField(state, "uiCustomThemes", resolveUpdatable(action.value, state.uiCustomThemes));
    case "SET_UI_THEME_DRAFT":
      return withField(state, "uiThemeDraft", resolveUpdatable(action.value, state.uiThemeDraft));
    case "SET_UI_KEYBINDING_OVERRIDES":
      return withField(state, "uiKeybindingOverrides", resolveUpdatable(action.value, state.uiKeybindingOverrides));
    default:
      return state;
  }
}

export function syncReducer(state: SyncState, action: ShellAction): SyncState {
  switch (action.type) {
    case "SET_SYNC_BACKBONE_URI":
      return withField(state, "syncBackboneUri", resolveUpdatable(action.value, state.syncBackboneUri));
    case "SET_SYNC_CARD_KIND":
      return withField(state, "syncCardKind", resolveUpdatable(action.value, state.syncCardKind));
    case "SET_SYNC_DRAFT_PATH":
      return withField(state, "syncDraftPath", resolveUpdatable(action.value, state.syncDraftPath));
    case "SET_SYNC_STATUS_FOR_DOCUMENT":
      return { ...state, syncStatusByDocumentId: { ...state.syncStatusByDocumentId, [action.documentId]: action.status } };
    default:
      return state;
  }
}

/** 💡️ Retains pending operation identity while only actual owner replies populate service status. */
export function inferenceReducer(state: InferenceState, action: ShellAction): InferenceState {
  switch (action.type) {
    case "OPEN_INFERENCE_PORT":
      return { portByRuntimeKey: {}, operationEpoch: action.operationEpoch, operationRuntimeKey: action.runtimeKey };
    case "SET_INFERENCE_PORT_FOR_DOCUMENT":
      return state.operationRuntimeKey === action.runtimeKey ? { ...state, portByRuntimeKey: { [action.runtimeKey]: action.status } } : state;
    case "CLEAR_INFERENCE_PORT_FOR_DOCUMENT": {
      if (!(action.runtimeKey in state.portByRuntimeKey)) return state;
      const portByRuntimeKey = { ...state.portByRuntimeKey };
      delete portByRuntimeKey[action.runtimeKey];
      const owned = state.operationRuntimeKey === action.runtimeKey;
      return { portByRuntimeKey, operationEpoch: owned ? null : state.operationEpoch, operationRuntimeKey: owned ? null : state.operationRuntimeKey };
    }
    default:
      return state;
  }
}

/** ⚖️ Reducer for the merge-outcome/first-class-conflict slice — see {@link MergeState}. */
export function mergeReducer(state: MergeState, action: ShellAction): MergeState {
  switch (action.type) {
    case "SET_MERGE_POLICY":
      return state.mergePolicy === action.value ? state : { ...state, mergePolicy: action.value };
    case "SET_CONFLICTS": {
      const conflicts = resolveUpdatable(action.value, state.conflicts);
      const selectedConflictId = state.selectedConflictId && conflicts.some((entry) => entry.id === state.selectedConflictId) ? state.selectedConflictId : null;
      return { ...state, conflicts, selectedConflictId };
    }
    case "SET_SELECTED_CONFLICT_ID":
      return state.selectedConflictId === action.value ? state : { ...state, selectedConflictId: action.value };
    default:
      return state;
  }
}

/** 🎥️ Reducer for the tutorial playback/recording slice — see `TutorialState`. `SET_TUTORIAL` resets every playback-rate/deviation flag (a fresh tutorial never inherits the previous one's rate/deviation); starting an introduction (`SET_INTRODUCTION_STEP` with a non-null value) clears the active tutorial, mirroring `overlayReducer`'s reverse case. */
export function tutorialReducer(state: TutorialState, action: ShellAction): TutorialState {
  switch (action.type) {
    case "SET_TUTORIAL":
      return { activeTutorialId: action.value, playing: false, rate: 1, muted: state.muted, captionsOn: state.captionsOn, recording: false, deviated: false };
    case "SET_TUTORIAL_PLAYING": {
      const nextPlaying = resolveUpdatable(action.value, state.playing);
      return { ...state, playing: nextPlaying, deviated: nextPlaying ? false : state.deviated };
    }
    case "SET_TUTORIAL_RATE":
      return withField(state, "rate", action.value);
    case "SET_TUTORIAL_MUTED":
      return withField(state, "muted", resolveUpdatable(action.value, state.muted));
    case "SET_TUTORIAL_CAPTIONS":
      return withField(state, "captionsOn", resolveUpdatable(action.value, state.captionsOn));
    case "SET_TUTORIAL_RECORDING":
      return withField(state, "recording", action.value);
    case "SET_TUTORIAL_DEVIATED":
      return withField(state, "deviated", action.value);
    case "SET_INTRODUCTION_STEP":
      return typeof action.value !== "function" && action.value != null && state.activeTutorialId != null ? { ...state, activeTutorialId: null, playing: false, deviated: false } : state;
    default:
      return state;
  }
}
//#endregion slice reducers
//#endregion 🧮️ShellStore
