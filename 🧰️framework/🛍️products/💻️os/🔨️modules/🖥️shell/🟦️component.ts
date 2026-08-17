/** 🖥️ TypeScript twin of the Rust shell-state SSOT (`🦀️component.rs`): the same `ShellState` +
 * `ShellCommand` + `ShellEvent` + `ShellError` vocabulary (re-exported from the ts-rs mirror at
 * `./🤖️generated/🟦️shell.js`) plus an independent `reduce()` implementation with the same
 * semantics. This file is NOT generated — it is hand-written to the same contract the Rust
 * `reduce` implements, and the parity fixtures under `../🧫️fixtures/*.json` are what keeps the two
 * honest: `component.rs`'s `fixtures_produce_expected_output` test re-derives every fixture
 * against the Rust reducer, and this file's own vitest suite (below) re-derives the same fixtures
 * against THIS reducer. A divergence between the two implementations shows up as a fixture
 * mismatch on exactly one side.
 *
 * Regenerate the type mirror via `bun nx run @semio-tech/framework-os-shell-rs:typegen`.
 */
export * from "./🤖️generated/🟦️shell.js";
import type { Anchor, ByAnchor, Conflict, DialogState, ExtraWindowInstance, LoadedPlugin, ShellCommand, ShellError, ShellEvent, ShellState } from "./🤖️generated/🟦️shell.js";

//#region 🛰️CapabilityIds
/** 🛰️ `ShellCommand["type"]` → `ShellCapability.id`. Must stay in lockstep with
 * `SHELL_COMMAND_CATALOG` in `🦀️component.rs` — the shared fixtures catch drift (every fixture's
 * `expected.events` includes an `applied` event stamped with the id this map produces). */
const CAPABILITY_ID_BY_COMMAND_TYPE: Record<ShellCommand["type"], string> = {
  registerLoadedPlugin: "plugin.register",
  unregisterLoadedPlugin: "plugin.unregister",
  setPluginStatus: "plugin.setStatus",
  setPluginSupervisorState: "plugin.setSupervisorState",
  setActiveSession: "plugin.setActiveSession",
  setSessionError: "plugin.setSessionError",
  setAppLabelOverride: "ui.app.setLabelOverride",
  setActionPaneFolded: "shell.action.fold",
  setActionPaneExpanded: "shell.action.expand",
  stageActionArg: "shell.action.stageArg",
  resetActionArgs: "shell.action.reset",
  setActiveUtility: "ui.window.setActiveUtility",
  setActiveTool: "ui.tool.setActive",
  setCommandExpanded: "ui.command.expand",
  stageCommandArg: "ui.command.stageArg",
  resetCommandArgs: "ui.command.reset",
  setPanelVisible: "shell.panelToggle",
  setPanelSize: "ui.panel.setSize",
  setPanelPath: "shell.panel.tab",
  setDockOverride: "ui.dock.setOverride",
  setPanelPathMemory: "ui.panel.setPathMemory",
  setTreeOpenState: "ui.tree.setOpenState",
  hydrateDockUi: "ui.dock.hydrate",
  resetDock: "ui.dock.reset",
  focusWindow: "ui.window.focus",
  setShellLayout: "ui.shell.setLayout",
  setActiveExample: "ui.example.setActive",
  setMobilePanelPath: "ui.mobile.setPanelPath",
  setMobilePanelVisible: "ui.mobile.setPanelVisible",
  setExtraWindows: "ui.window.setExtraWindows",
  setWindowTitle: "ui.window.setTitle",
  setWindowIcon: "ui.window.setIcon",
  setSearchOpen: "ui.search.setOpen",
  setFindOpen: "ui.find.setOpen",
  autoStartIntroduction: "ui.introduction.autoStart",
  setIntroductionStep: "ui.introduction.setStep",
  completeIntroductionInteraction: "ui.introduction.completeInteraction",
  openDialog: "ui.dialog.open",
  closeDialog: "ui.dialog.close",
  showTransientNotice: "ui.notice.show",
  dismissTransientNotice: "ui.notice.dismiss",
  setOpenWithFocusRole: "ui.open.setFocusRole",
  setActiveTutorial: "ui.tutorial.setActive",
  setUiAppearance: "os.setAppearance",
  setUiLayout: "os.setDriver",
  setUiDriver: "ui.driver.setActive",
  setUiCustomDriver: "ui.driver.setCustom",
  setUiDriverDraft: "ui.driver.setDraft",
  setUiLocale: "os.setLocale",
  setUiTerminology: "os.setTerminology",
  setUiTheme: "os.setThemeId",
  setUiCustomTheme: "ui.theme.setCustom",
  setUiThemeDraft: "ui.theme.setDraft",
  setUiKeybindingOverride: "ui.keybinding.setOverride",
  setSyncBackboneUri: "sync.setBackboneUri",
  setSyncCardKind: "sync.setCardKind",
  setSyncDraftPath: "sync.setDraftPath",
  setDocumentSyncStatus: "sync.setDocumentStatus",
  setMergePolicy: "merge.setPolicy",
  setConflicts: "merge.setConflicts",
  selectConflict: "merge.selectConflict",
  setStorageScope: "host.setStorageScope",
  setOpeningPreference: "host.setOpeningPreference",
};
//#endregion 🛰️CapabilityIds

//#region 🚨️Rejected
/** 🚨️ Internal control-flow-only exception carrying a [`ShellError`] — never escapes `reduce()`. */
class Rejected extends Error {
  constructor(readonly shellError: ShellError) {
    super(shellError.kind);
  }
}

function requireNonEmpty(value: string, field: string): void {
  if (value.trim().length === 0) throw new Rejected({ kind: "emptyIdentifier", field });
}
//#endregion 🚨️Rejected

//#region 🧮️reduce
export type ReduceOk = { readonly ok: true; readonly state: ShellState; readonly events: readonly ShellEvent[] };
export type ReduceErr = { readonly ok: false; readonly error: ShellError };
export type ReduceResult = ReduceOk | ReduceErr;

/** 🧮️ Total, pure state transition — the TypeScript twin of `🦀️component.rs`'s `reduce`. Same
 * inputs → same outputs, never throws to the caller (rejections come back as `{ok: false}`).
 * `nowMs` is accepted for interface parity with the Rust signature; like the Rust side, this
 * function never reads a clock itself. */
export function reduce(state: ShellState, command: ShellCommand, nowMs: number): ReduceResult {
  void nowMs;
  const next: ShellState = structuredClone(state);
  const original: ShellState = structuredClone(state);
  const cmd: ShellCommand = structuredClone(command);
  const events: ShellEvent[] = [];
  try {
    applyCommand(next, original, cmd, events);
  } catch (error) {
    if (error instanceof Rejected) return { ok: false, error: error.shellError };
    throw error;
  }
  next.revision = state.revision + 1;
  events.push({ type: "applied", capabilityId: CAPABILITY_ID_BY_COMMAND_TYPE[cmd.type], revision: next.revision });
  return { ok: true, state: next, events };
}

function byAnchorSet<T>(record: ByAnchor<T>, anchor: Anchor, value: T): void {
  record[anchor] = value;
}

// eslint-disable-next-line complexity
function applyCommand(next: ShellState, original: ShellState, command: ShellCommand, events: ShellEvent[]): void {
  switch (command.type) {
    //#region 🔌️PluginRuntime
    case "registerLoadedPlugin": {
      requireNonEmpty(command.plugin.pluginId, "plugin.pluginId");
      next.loadedPlugins = next.loadedPlugins.filter((p: LoadedPlugin) => p.pluginId !== command.plugin.pluginId);
      next.loadedPlugins.push(command.plugin);
      break;
    }
    case "unregisterLoadedPlugin": {
      requireNonEmpty(command.pluginId, "plugin_id");
      const before = next.loadedPlugins.length;
      next.loadedPlugins = next.loadedPlugins.filter((p: LoadedPlugin) => p.pluginId !== command.pluginId);
      if (next.loadedPlugins.length === before) throw new Rejected({ kind: "unknownPlugin", pluginId: command.pluginId });
      delete next.pluginStatusById[command.pluginId];
      delete next.pluginSupervisorById[command.pluginId];
      break;
    }
    case "setPluginStatus": {
      requireNonEmpty(command.pluginId, "plugin_id");
      next.pluginStatusById[command.pluginId] = command.status;
      break;
    }
    case "setPluginSupervisorState": {
      requireNonEmpty(command.pluginId, "plugin_id");
      next.pluginSupervisorById[command.pluginId] = command.state;
      break;
    }
    case "setActiveSession": {
      next.activeSession = command.session;
      break;
    }
    case "setSessionError": {
      next.sessionError = command.error;
      break;
    }
    //#endregion 🔌️PluginRuntime

    //#region 🏷️AppLabels
    case "setAppLabelOverride": {
      requireNonEmpty(command.appId, "app_id");
      requireNonEmpty(command.labelKey, "label_key");
      const entry = (next.appLabelsOverlay[command.appId] ??= {});
      if (command.value !== null) {
        entry[command.labelKey] = command.value;
      } else {
        delete entry[command.labelKey];
        if (Object.keys(entry).length === 0) delete next.appLabelsOverlay[command.appId];
      }
      break;
    }
    //#endregion 🏷️AppLabels

    //#region 🎛️ActionRail
    case "setActionPaneFolded": {
      requireNonEmpty(command.windowId, "window_id");
      next.actionPaneFoldedByWindow[command.windowId] = command.folded;
      break;
    }
    case "setActionPaneExpanded": {
      requireNonEmpty(command.windowId, "window_id");
      next.actionPaneExpandedByWindow[command.windowId] = command.actionId;
      break;
    }
    case "stageActionArg": {
      requireNonEmpty(command.windowId, "window_id");
      requireNonEmpty(command.actionId, "action_id");
      requireNonEmpty(command.argId, "arg_id");
      const byAction = (next.stagedActionArgs[command.windowId] ??= {});
      const byArg = (byAction[command.actionId] ??= {});
      byArg[command.argId] = command.value;
      break;
    }
    case "resetActionArgs": {
      requireNonEmpty(command.windowId, "window_id");
      requireNonEmpty(command.actionId, "action_id");
      const byAction = next.stagedActionArgs[command.windowId];
      if (byAction) {
        delete byAction[command.actionId];
        if (Object.keys(byAction).length === 0) delete next.stagedActionArgs[command.windowId];
      }
      break;
    }
    case "setActiveUtility": {
      requireNonEmpty(command.windowId, "window_id");
      next.activeUtilityByWindow[command.windowId] = command.utilityId;
      if (command.utilityId !== null && next.activeWindowId === command.windowId && next.activeToolId !== null) {
        const previous = next.activeToolId;
        next.activeToolId = null;
        events.push({ type: "activeToolChanged", previous, current: null });
      }
      break;
    }
    case "setActiveTool": {
      next.activeToolId = command.toolId;
      if (command.toolId !== null && next.activeWindowId !== null) {
        const windowId = next.activeWindowId;
        if (Object.prototype.hasOwnProperty.call(next.activeUtilityByWindow, windowId)) {
          const previous = next.activeUtilityByWindow[windowId];
          if (previous !== null && previous !== undefined) {
            next.activeUtilityByWindow[windowId] = null;
            events.push({ type: "activeUtilityChanged", windowId, previous, current: null });
          }
        }
      }
      break;
    }
    //#endregion 🎛️ActionRail

    //#region 🎮️CommandPalette
    case "setCommandExpanded": {
      next.commandPanelExpanded = command.commandId;
      break;
    }
    case "stageCommandArg": {
      requireNonEmpty(command.commandId, "command_id");
      requireNonEmpty(command.argId, "arg_id");
      const byArg = (next.stagedCommandArgs[command.commandId] ??= {});
      byArg[command.argId] = command.value;
      break;
    }
    case "resetCommandArgs": {
      requireNonEmpty(command.commandId, "command_id");
      delete next.stagedCommandArgs[command.commandId];
      break;
    }
    //#endregion 🎮️CommandPalette

    //#region 🗂️PanelLayout
    case "setPanelVisible": {
      byAnchorSet(next.panelsVisible, command.anchor, command.visible);
      break;
    }
    case "setPanelSize": {
      if (!Number.isFinite(command.size) || command.size < 0) throw new Rejected({ kind: "invalidPanelSize", anchor: command.anchor, size: command.size });
      byAnchorSet(next.panelsSize, command.anchor, command.size);
      break;
    }
    case "setPanelPath": {
      byAnchorSet(next.panelsPath, command.anchor, command.path);
      break;
    }
    case "setDockOverride": {
      next.dockOverride = command.dock;
      break;
    }
    case "setPanelPathMemory": {
      requireNonEmpty(command.panelKey, "panel_key");
      if (command.path !== null) next.panelPathMemory[command.panelKey] = command.path;
      else delete next.panelPathMemory[command.panelKey];
      break;
    }
    case "setTreeOpenState": {
      requireNonEmpty(command.treeId, "tree_id");
      next.treeOpenStates[command.treeId] = command.open;
      break;
    }
    case "hydrateDockUi": {
      next.dockOverride = command.dock ? command.dock.layout : null;
      if (command.dock) next.panelsVisible = command.dock.panelsVisible;
      break;
    }
    case "resetDock": {
      next.dockOverride = null;
      events.push({ type: "dockReset" });
      break;
    }
    case "focusWindow": {
      const previous = next.activeWindowId;
      next.activeWindowId = command.windowId;
      if (previous !== next.activeWindowId) events.push({ type: "windowFocusChanged", previous, current: next.activeWindowId });
      break;
    }
    case "setShellLayout": {
      next.shellLayout = command.layout;
      break;
    }
    case "setActiveExample": {
      next.activeExampleId = command.exampleId;
      break;
    }
    case "setMobilePanelPath": {
      next.mobilePanelPath = command.path;
      break;
    }
    case "setMobilePanelVisible": {
      next.mobilePanelVisible = command.visible;
      break;
    }
    case "setExtraWindows": {
      next.extraWindows = command.windows;
      if (next.activeWindowId !== null) {
        const active = next.activeWindowId;
        const wasExtra = original.extraWindows.some((w: ExtraWindowInstance) => w.windowId === active);
        const stillPresent = next.extraWindows.some((w: ExtraWindowInstance) => w.windowId === active);
        if (wasExtra && !stillPresent) {
          const fallback = next.extraWindows.length > 0 ? next.extraWindows[next.extraWindows.length - 1].windowId : null;
          next.activeWindowId = fallback;
          events.push({ type: "windowFocusChanged", previous: active, current: fallback });
        }
      }
      break;
    }
    case "setWindowTitle": {
      requireNonEmpty(command.windowId, "window_id");
      next.windowTitlesById[command.windowId] = command.title;
      break;
    }
    case "setWindowIcon": {
      requireNonEmpty(command.windowId, "window_id");
      next.windowIconsById[command.windowId] = command.icon;
      break;
    }
    //#endregion 🗂️PanelLayout

    //#region 🔔️Overlays
    case "setSearchOpen": {
      next.searchOpen = command.open;
      break;
    }
    case "setFindOpen": {
      next.findOpen = command.open;
      break;
    }
    case "autoStartIntroduction": {
      requireNonEmpty(command.key, "key");
      if (!next.introductionAutoStartedKeys.includes(command.key)) next.introductionAutoStartedKeys.push(command.key);
      break;
    }
    case "setIntroductionStep": {
      next.introductionStepIndex = command.stepIndex;
      break;
    }
    case "completeIntroductionInteraction": {
      if (!next.introductionCompletedInteractions.includes(command.interactionIndex)) next.introductionCompletedInteractions.push(command.interactionIndex);
      break;
    }
    case "openDialog": {
      requireNonEmpty(command.dialogId, "dialog_id");
      next.dialogStack.push({ dialogId: command.dialogId, seedArgs: command.seedArgs ?? null });
      events.push({ type: "dialogOpened", dialogId: command.dialogId });
      break;
    }
    case "closeDialog": {
      let closedId: string;
      if (command.dialogId !== null) {
        requireNonEmpty(command.dialogId, "dialog_id");
        const index = next.dialogStack.findIndex((d: DialogState) => d.dialogId === command.dialogId);
        if (index === -1) throw new Rejected({ kind: "unknownDialog", dialogId: command.dialogId });
        closedId = next.dialogStack.splice(index, 1)[0]!.dialogId;
      } else {
        const top = next.dialogStack.pop();
        if (!top) throw new Rejected({ kind: "unknownDialog", dialogId: "" });
        closedId = top.dialogId;
      }
      events.push({ type: "dialogClosed", dialogId: closedId });
      break;
    }
    case "showTransientNotice": {
      next.transientNotice = command.notice;
      break;
    }
    case "dismissTransientNotice": {
      next.transientNotice = null;
      break;
    }
    case "setOpenWithFocusRole": {
      next.openWithFocusRole = command.role;
      break;
    }
    //#endregion 🔔️Overlays

    //#region 🎓️Tutorial
    case "setActiveTutorial": {
      next.activeTutorialId = command.tutorialId;
      break;
    }
    //#endregion 🎓️Tutorial

    //#region 🎨️UiPreferences
    case "setUiAppearance": {
      next.uiAppearance = command.appearance;
      break;
    }
    case "setUiLayout": {
      next.uiLayout = command.layout;
      break;
    }
    case "setUiDriver": {
      next.uiDriverId = command.driverId;
      break;
    }
    case "setUiCustomDriver": {
      requireNonEmpty(command.driverId, "driver_id");
      if (command.driver !== null) next.uiCustomDrivers[command.driverId] = command.driver;
      else delete next.uiCustomDrivers[command.driverId];
      break;
    }
    case "setUiDriverDraft": {
      next.uiDriverDraft = command.draft;
      break;
    }
    case "setUiLocale": {
      next.uiLocale = command.locale;
      break;
    }
    case "setUiTerminology": {
      next.uiTerminology = command.terminologyId;
      break;
    }
    case "setUiTheme": {
      next.uiThemeId = command.themeId;
      break;
    }
    case "setUiCustomTheme": {
      requireNonEmpty(command.themeId, "theme_id");
      if (command.theme !== null) next.uiCustomThemes[command.themeId] = command.theme;
      else delete next.uiCustomThemes[command.themeId];
      break;
    }
    case "setUiThemeDraft": {
      next.uiThemeDraft = command.draft;
      break;
    }
    case "setUiKeybindingOverride": {
      requireNonEmpty(command.controlId, "control_id");
      if (command.keys !== null) next.uiKeybindingOverrides[command.controlId] = command.keys;
      else delete next.uiKeybindingOverrides[command.controlId];
      break;
    }
    //#endregion 🎨️UiPreferences

    //#region 🔄️Sync
    case "setSyncBackboneUri": {
      next.syncBackboneUri = command.uri;
      break;
    }
    case "setSyncCardKind": {
      next.syncCardKind = command.kind;
      break;
    }
    case "setSyncDraftPath": {
      next.syncDraftPath = command.path;
      break;
    }
    case "setDocumentSyncStatus": {
      requireNonEmpty(command.documentId, "document_id");
      next.syncStatusByDocument[command.documentId] = command.status;
      break;
    }
    //#endregion 🔄️Sync

    //#region 🤝️Merge
    case "setMergePolicy": {
      next.mergePolicy = command.policy;
      break;
    }
    case "setConflicts": {
      next.conflicts = command.conflicts;
      if (next.selectedConflictId !== null && !next.conflicts.some((c: Conflict) => c.conflictId === next.selectedConflictId)) {
        next.selectedConflictId = null;
      }
      break;
    }
    case "selectConflict": {
      if (command.conflictId !== null) {
        requireNonEmpty(command.conflictId, "conflict_id");
        if (!next.conflicts.some((c: Conflict) => c.conflictId === command.conflictId)) throw new Rejected({ kind: "unknownConflict", conflictId: command.conflictId });
      }
      next.selectedConflictId = command.conflictId;
      break;
    }
    //#endregion 🤝️Merge

    //#region 💾️Host
    case "setStorageScope": {
      next.storageScope = command.scope;
      break;
    }
    case "setOpeningPreference": {
      requireNonEmpty(command.role, "role");
      if (command.dialectId !== null) next.openingPreferences[command.role] = command.dialectId;
      else delete next.openingPreferences[command.role];
      break;
    }
    //#endregion 💾️Host

    default: {
      const exhaustive: never = command;
      throw new Error(`shell: unhandled command ${JSON.stringify(exhaustive)}`);
    }
  }
}
//#endregion 🧮️reduce

//#region 🧪️tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  const defaultState = (): ShellState => ({
    revision: 0,
    loadedPlugins: [],
    pluginStatusById: {},
    pluginSupervisorById: {},
    activeSession: null,
    sessionError: null,
    appLabelsOverlay: {},
    actionPaneFoldedByWindow: {},
    actionPaneExpandedByWindow: {},
    stagedActionArgs: {},
    activeUtilityByWindow: {},
    activeToolId: null,
    commandPanelExpanded: null,
    stagedCommandArgs: {},
    panelsVisible: { left: false, right: false, top: false, bottom: false },
    panelsSize: { left: 280, right: 280, top: 280, bottom: 280 },
    panelsPath: { left: [], right: [], top: [], bottom: [] },
    dockOverride: null,
    panelPathMemory: {},
    treeOpenStates: {},
    activeWindowId: null,
    shellLayout: null,
    activeExampleId: "",
    mobilePanelPath: [],
    mobilePanelVisible: false,
    extraWindows: [],
    windowTitlesById: {},
    windowIconsById: {},
    searchOpen: false,
    findOpen: false,
    introductionStepIndex: null,
    introductionAutoStartedKeys: [],
    introductionCompletedInteractions: [],
    dialogStack: [],
    transientNotice: null,
    openWithFocusRole: null,
    activeTutorialId: null,
    uiAppearance: "system",
    uiLayout: "default",
    uiDriverId: "",
    uiCustomDrivers: {},
    uiDriverDraft: null,
    uiLocale: "en",
    uiTerminology: "",
    uiThemeId: "",
    uiCustomThemes: {},
    uiThemeDraft: null,
    uiKeybindingOverrides: {},
    syncBackboneUri: null,
    syncCardKind: null,
    syncDraftPath: "",
    syncStatusByDocument: {},
    mergePolicy: "manual",
    conflicts: [],
    selectedConflictId: null,
    storageScope: "memory",
    openingPreferences: {},
  });

  describe("@semio-tech/framework-os-shell reduce", () => {
    it("is pure and increments revision", () => {
      const state = defaultState();
      const result = reduce(state, { type: "setSearchOpen", open: true }, 1000);
      expect(result.ok).toBe(true);
      if (!result.ok) throw new Error("unreachable");
      expect(result.state.revision).toBe(state.revision + 1);
      expect(result.state.searchOpen).toBe(true);
      expect(state.searchOpen).toBe(false); // input untouched
    });

    it("rejects leave state untouched and report a typed error", () => {
      const state = defaultState();
      const result = reduce(state, { type: "selectConflict", conflictId: "missing" }, 1000);
      expect(result.ok).toBe(false);
      if (result.ok) throw new Error("unreachable");
      expect(result.error).toEqual({ kind: "unknownConflict", conflictId: "missing" });
    });

    it("re-derives every shared fixture (Rust/TS parity)", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const fixturesDir = join(here, "🧫️fixtures");
      const files = readdirSync(fixturesDir).filter((name) => name.endsWith(".json"));
      expect(files.length).toBeGreaterThanOrEqual(63);

      for (const file of files) {
        const fixture = JSON.parse(readFileSync(join(fixturesDir, file), "utf8"));
        const result = reduce(fixture.state, fixture.command, 1_700_000_000_000);
        if ("error" in fixture.expected) {
          expect(result.ok, `${fixture.name}: expected an error`).toBe(false);
          if (!result.ok) expect(result.error, `${fixture.name} error mismatch`).toEqual(fixture.expected.error);
        } else {
          expect(result.ok, `${fixture.name}: expected ok`).toBe(true);
          if (result.ok) {
            expect(result.state, `${fixture.name} state mismatch`).toEqual(fixture.expected.state);
            expect(result.events, `${fixture.name} events mismatch`).toEqual(fixture.expected.events);
          }
        }
      }
    });
  });
}
//#endregion 🧪️tests
