// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Shell/component.tsx
/** @emoji 🐚️ `Shell` — the core shell state/types module: boot option/lock/default types, the
 * consolidated `ShellState` reducer (`useReducer` store replacing ~38 independent `useState` calls,
 * grouped by concern into slices), `bootFrameworkOs` (the pre-mount entry point), and the
 * `ShellFaultBoundary` scoped React error boundary. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { Component, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
  ANCHORS,
  Button,
  bootstrapElementsSurfaceChromeDocument,
  builtinUiThemes,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiChromeLocale,
  readStoredUiChromeTerminology,
  readStoredUiChromeThemeId,
  readStoredUiCustomDrivers,
  readStoredUiCustomThemes,
  readStoredUiDriverId,
  readStoredUiKeybindingOverrides,
  uiI18n,
  type Anchor,
  type ElementsSurfaceAppearance,
  type ElementsSurfaceDevice,
  type IconName,
  type UiChromeLayout,
  type UiDriver,
  type UiLabel,
  type UiLocale,
  type UiTheme,
  type WindowLayoutNode,
} from "@semio-tech/ui-react";
import {
  createBrowserStoragePort,
  createMemoryStoragePort,
  SemioFaultError,
  SHELL_LOCALES,
  type AppDefinition,
  type CommandDefinition,
  type DockSkeleton,
  type DockUiState,
  type Fault,
  type PluginAppLabelsOverlay,
  type PluginViewState,
  type ShellBrand,
  type ShellLocale,
  type ShellTerminology,
  type StoragePort,
  type UiNode,
  type WindowEngagement,
  type WindowMeasure,
} from "@semio-tech/framework";
import { type ArtifactSyncStatus } from "@semio-tech/framework-os";
// 🧱️core: shellLabel imported directly from ShellHelpers (its real implementation, not via the barrel) —
// this module calls shellLabel(...) at module top level (UI_INSPECTOR_MIXED_PLACEHOLDER), which requires
// a non-circular import; routing through the barrel indirection (cleared) hit the same
// module-top-level circular-import initialization-order bug documented in ui-react's
// 🧱️elements/🫀️core/Ports/🟦️component.tsx header comment (see 📋️w0-status.md's "W3 follow-up" section).
import { shellLabel } from "../ShellHelpers/🟦️component.tsx";
import { DEFAULT_PANEL_WIDTH_PX } from "../ShellHelpers/🟦️component.tsx";
import { FrameworkOsShell } from "../ShellHost/🟦️component.tsx";
import { type PluginWasmHandle } from "../PluginRuntime/🟦️component.tsx";
import { PRESENCE_CLIENT_STORAGE_KEY, EMPTY_APP_LABELS_OVERLAY } from "../ShellHelpers/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️types
/** 🌐️ Locale-resolved mixed-value placeholder for this renderer layer; framework/core/js/index.ts keeps its own non-reactive low-level default. */
export const UI_INSPECTOR_MIXED_PLACEHOLDER = shellLabel("ui.common.mixedValues");

/** 🎭️ Renderer-side view state passed to program wasm calls — structurally mirrors `@semio-tech/framework`'s {@link PluginViewState}, kept as a distinct local alias since `ViewModel` is the established name used throughout this file. */
export type ViewModel = PluginViewState;

/** ⚠️ Not folded into `@semio-tech/framework`'s `PluginManifest`: this shell-local shape types `apps` richly (`AppDefinition[]`) where core intentionally keeps the wasm-boundary shape loose (`Record<string, unknown>[]`) for other consumers (e.g. compose, coda). Left for a human to decide whether to widen core's `PluginManifest` itself. */
export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly AppDefinition[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly {
    readonly kind: "playbookBlockKind";
    readonly appId: string;
    readonly blockKind: string;
    readonly label: string;
    readonly iconId: IconName;
    readonly defaultValueJson?: string;
    readonly paramsBodyKey: string;
    readonly previewBodyKey: string;
  }[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

export type LoadedProgramState = {
  readonly handle: PluginWasmHandle;
  readonly manifest: PluginManifest;
};

/** 🔌️ Lifecycle status of one registry entry for the plugin panel (bottom-right dock): "available" —
 * registered but not (yet) loaded, including a plugin whose first build hasn't landed. Driven by
 * `installPlugin`/`reloadPlugin`/`uninstallPlugin` and the `PluginSource` subscription, not by
 * `loadedPlugins` membership alone — a `"reloading"` entry is still present in `loadedPlugins` (the old
 * handle keeps serving until the swap completes). */
export type PluginPanelStatus = "available" | "installing" | "loaded" | "failed" | "reloading";

export type PluginSupervisorState = "loaded" | "running" | "restarting" | "crashed" | "quarantined";

export type ActiveSession = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly app: AppDefinition;
  readonly viewState: ViewModel;
};

export type SpaceProgramEntry = {
  readonly pluginId: string;
  readonly workflowStepId: string;
  readonly appId: string;
  readonly label: string;
  readonly label: readonly string[];
  readonly yields: string;
};

export type SpawnedAppEntry = {
  readonly id: string;
  readonly pluginId: string;
  readonly instanceId: number;
  readonly appId: string;
  readonly label: string;
  readonly label: readonly string[];
};

export type SpacePanelState = {
  readonly activePanelTab: string;
  readonly programs: readonly SpaceProgramEntry[];
  readonly spawnedApps: readonly SpawnedAppEntry[];
  readonly activeSpawnedId?: string;
};

export type FrameworkOsBootOptions = {
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  readonly locks?: FrameworkOsLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
};

//#region 🔒️FrameworkOsLocks
/** 🔒️ Raw boot-time lock values from env, before validation (any of the five may be unset). */
export type FrameworkOsLocks = {
  readonly exampleId?: string;
  readonly locale?: string;
  readonly terminology?: string;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🔒️ Validated locks: unknown values warn and fall back to a safe default while staying locked. */
export type ResolvedShellLocks = {
  readonly exampleId?: string;
  readonly locale?: UiLocale;
  readonly terminology?: ShellTerminology;
  readonly themeId?: string;
  readonly appearance?: ElementsSurfaceAppearance;
};

/**
 * 🔒️ Validates raw `FrameworkOsLocks` against what the shell can actually apply at boot. A locked
 * session stays locked even on an invalid value (falls back to a default) rather than silently
 * degrading to switchable — the CLI asked for no in-app switching, so a typo must not remove that.
 */
export function resolveShellLocks(locks: FrameworkOsLocks | undefined): ResolvedShellLocks {
  if (!locks) return {};
  const resolved: { -readonly [K in keyof ResolvedShellLocks]?: ResolvedShellLocks[K] } = {};
  if (locks.exampleId) resolved.exampleId = locks.exampleId;
  if (locks.locale !== undefined) {
    if ((SHELL_LOCALES as readonly string[]).includes(locks.locale)) {
      resolved.locale = locks.locale as ShellLocale;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_LOCALE ${JSON.stringify(locks.locale)}, falling back to "en"`);
      resolved.locale = "en";
    }
  }
  if (locks.terminology !== undefined) {
    if (locks.terminology !== "") {
      resolved.terminology = locks.terminology as ShellTerminology;
    }
  }
  if (locks.themeId !== undefined) {
    const known = new Set([...builtinUiThemes().map((t) => t.id), ...Object.keys(readStoredUiCustomThemes(createBrowserStoragePort()))]);
    if (known.has(locks.themeId)) {
      resolved.themeId = locks.themeId;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_THEME ${JSON.stringify(locks.themeId)}, falling back to "semio"`);
      resolved.themeId = "semio";
    }
  }
  if (locks.appearance !== undefined) {
    if (locks.appearance === "light" || locks.appearance === "dark") {
      resolved.appearance = locks.appearance;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_APPEARANCE ${JSON.stringify(locks.appearance)}, falling back to "system"`);
      resolved.appearance = "system";
    }
  }
  return resolved;
}

/** 🔒️ Stable empty-locks reference so an omitted `locks` prop never busts memo dependency arrays. */
export const EMPTY_SHELL_LOCKS: ResolvedShellLocks = {};

/** 🔒️ Overlays env locks onto a brand's locks per key — a lock set by either source stays locked, an env value wins over the brand value. */
export function mergeShellLockSources(brandLocks: FrameworkOsLocks | undefined, envLocks: FrameworkOsLocks | undefined): FrameworkOsLocks | undefined {
  if (!brandLocks || !envLocks) return envLocks ?? brandLocks;
  return { ...brandLocks, ...Object.fromEntries(Object.entries(envLocks).filter(([, value]) => value !== undefined)) };
}

/** 🎛️ Boot-time default values that seed shell state without locking it — the matching in-app switcher stays visible, unlike locks. */
export type FrameworkOsDefaults = {
  readonly exampleId?: string;
};

/** 🎛️ Resolves boot defaults: an env-provided default wins over the brand's. */
export function resolveShellDefaults(brand: ShellBrand | undefined, defaults: FrameworkOsDefaults | undefined): FrameworkOsDefaults {
  return { exampleId: defaults?.exampleId ?? brand?.defaults?.exampleId };
}

/**
 * 🧪️ Picks the example id announced on a fresh session: keep a still-valid active/default id,
 * otherwise the first registered example (matches wgpu `sync_session_chrome`) so the navbar never
 * boots on “No example” while the plugin’s default document is already that first fixture.
 */
export function resolveBootExampleId(
  activeExampleId: string,
  exampleOptions: readonly { readonly id: string }[],
  defaultsExampleId?: string,
): string {
  if (activeExampleId && exampleOptions.some((option) => option.id === activeExampleId)) return activeExampleId;
  if (defaultsExampleId && exampleOptions.some((option) => option.id === defaultsExampleId)) return defaultsExampleId;
  return exampleOptions[0]?.id ?? "";
}

/** 🎓️ Whether a brand's introduction should auto-start on every window load, ignoring any device-local seen flag. */
export function shouldReplayIntroductionOnLoad(brand: ShellBrand | undefined): boolean {
  return isEphemeralShellBrand(brand) || brand?.replayIntroductionOnLoad === true;
}

/** 🎓️ Whether completing or dismissing an introduction should persist a device-local seen flag for this brand. */
export function shouldPersistIntroductionSeen(brand: ShellBrand | undefined): boolean {
  return !shouldReplayIntroductionOnLoad(brand);
}

/** 🧊️ Whether a brand boots with no durable shell state — every refresh starts from locks/defaults only. */
export function isEphemeralShellBrand(brand: ShellBrand | undefined): boolean {
  return brand?.ephemeral === true;
}

/** 🧊️ Removes known durable shell keys so an ephemeral brand leaves no localStorage/sessionStorage residue across refresh. */
export function clearDurableShellStorage(): void {
  if (typeof window === "undefined") return;
  const prefixes = ["ui.chrome.", "ui.introduction.seen.", "ui.themes.", "ui.compute.", "semio.os.", "compose.display.layouts."];
  try {
    const keys = Array.from({ length: localStorage.length }, (_, index) => localStorage.key(index)).filter((key): key is string => typeof key === "string");
    for (const key of keys) {
      if (prefixes.some((prefix) => key === prefix || key.startsWith(prefix))) localStorage.removeItem(key);
    }
  } catch {
    /* ignore */
  }
  try {
    window.sessionStorage.removeItem(PRESENCE_CLIENT_STORAGE_KEY);
  } catch {
    /* ignore */
  }
}

/** 🎛️ Stable empty-defaults reference so an omitted `defaults` prop never busts memo dependency arrays. */
export const EMPTY_SHELL_DEFAULTS: FrameworkOsDefaults = {};
//#endregion 🔒️FrameworkOsLocks

export type SyncCardKind = "file" | "folder" | "remote";

type UIHistoryEntry = { readonly uri: string };
export type UIHistory = { readonly entries: readonly UIHistoryEntry[]; readonly index: number };
//#endregion 🔖️types

//#region 🧮️ShellStore
/** 🧮️ Single consolidated `useReducer` state tree for `FrameworkOsShell`, replacing what used to be ~38 independent `useState` calls with one dispatch-driven store, grouped by concern. */

//#region slice shapes
type PluginRuntimeState = {
  readonly loadedPlugins: readonly LoadedProgramState[];
  readonly pluginStatusById: Readonly<Record<string, PluginPanelStatus>>;
  readonly pluginSupervisorById: Readonly<Record<string, PluginSupervisorState>>;
  readonly session: ActiveSession | null;
  readonly error: string | null;
};

/** 🪟️ Window UI/engagements/measures are keyed by window INSTANCE id, never by window kind — two
 * instances of the same kind (e.g. split top/perspective panes) must never share chrome or options. */
type WindowUiState = {
  readonly windowUiByWindowId: Readonly<Record<string, UiNode>>;
  readonly windowEngagementsByWindowId: Readonly<Record<string, WindowEngagement>>;
  readonly windowMeasuresByWindowId: Readonly<Record<string, readonly WindowMeasure[]>>;
  /** 🛠️ Mode-level tool measures, keyed by TOOL id (never a window id) — see `DocumentApp::tool_measures`. */
  readonly toolMeasuresByToolId: Readonly<Record<string, readonly WindowMeasure[]>>;
  readonly panelUiByKey: Readonly<Record<string, UiNode>>;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
};

type SpawnedWindowState = {
  readonly spawnedWindowUi: UiNode | null;
  readonly spawnedWindowEngagements: Readonly<Record<string, WindowEngagement>>;
  readonly spawnedWindowMeasures: Readonly<Record<string, readonly WindowMeasure[]>>;
};

/**
 * 🧰️ Per-window Action rail (P1–P5) state: fold/expand chrome, locally-buffered staged arg values
 * (keyed `${windowId}:${actionId}`, never dispatched until Execute), and the host-owned active utility per
 * window (never a document field, never a VCS operation). See {@link WindowActionPane}.
 */
export type ActionPaneState = {
  readonly foldedByWindowId: Readonly<Record<string, boolean>>;
  readonly expandedByWindowId: Readonly<Record<string, string | null>>;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly activeUtilityByWindowId: Readonly<Record<string, string | null>>;
  /** 🛠️ Host-owned active tool of the active MODE (never per-window, never a document field/VCS operation) —
   * mutually exclusive with any window's active utility. See {@link ToolRegistry}. */
  readonly activeToolId: string | null;
};

/** 🧰️ Composite key into {@link ActionPaneState.stagedArgsByKey}. */
export function actionStageKey(windowId: string, actionId: string): string {
  return `${windowId}:${actionId}`;
}

export type ExtraWindowInstance = { readonly id: string; readonly windowKindId: string; readonly title: string };

/** 🧭️ Per-anchor fold/size/active-tab-path state for one of the six {@link Panel}s. */
type PanelState = {
  readonly visible: boolean;
  readonly size: number;
  readonly path: readonly string[];
};

type ShellLayoutState = {
  readonly panels: Record<Anchor, PanelState>;
  /** 🗄️ User-rearranged dock diff against `defaultDock`, persisted via `DockLayoutStore`; `null` means "use the computed default arrangement". */
  readonly dockOverride: DockSkeleton | null;
  /** 🌱️ Per-branch drill-down memory across every anchor + mobile (see `progressPanelTabSelection`), persisted via `DockUiStateStore`. */
  readonly panelPathMemory: Readonly<Record<string, string>>;
  /** 🌱️ Persisted tree section/group expansion, namespaced per {@link PanelTreeUnit}, persisted via `DockUiStateStore`. */
  readonly treeOpenStates: Readonly<Record<string, boolean>>;
  readonly activeWindowId: string | null;
  readonly shellLayout: WindowLayoutNode | null;
  readonly activeExampleId: string;
  readonly mobilePanelPath: readonly string[];
  /** 📱️ Whether the merged mobile panel is open — dedicated state (not derived from `panels[anchor].visible`) so desktop-persisted anchor visibility never auto-opens the mobile strip on hydrate. Never persisted; mobile always boots canvas-first. */
  readonly mobilePanelVisible: boolean;
  readonly extraWindowInstances: readonly ExtraWindowInstance[];
  /** 🏷️ Live window-title overrides (projection labels, etc.) keyed by window instance id — base kinds and extras. */
  readonly windowTitlesById: Readonly<Record<string, string>>;
  /** 🖼️ Live window-icon overrides (projection icons, etc.) keyed by window instance id — base kinds and extras. */
  readonly windowIconsById: Readonly<Record<string, IconName>>;
};

type OverlayState = {
  readonly searchOpen: boolean;
  readonly findOpen: boolean;
  /** 🎓️ Current step of the active app's introduction walkthrough, or `null` when none is playing. */
  readonly introductionStepIndex: number | null;
  /** ✅️ Indices into the active step's `interactions` that are done — reset whenever the step changes. */
  readonly introductionCompletedInteractions: readonly number[];
  /** 🗨️ The open declared dialog (id + `HostEffect`-seeded args), or `null` when none is open. */
  readonly dialog: { readonly dialogId: string; readonly seedArgs?: Readonly<Record<string, unknown>> } | null;
};

/** 🎥️ Playback/recording state of the active `TutorialDefinition` — mutually exclusive with `overlays.introductionStepIndex` (see `SET_TUTORIAL`/`SET_INTRODUCTION_STEP` reducer cases). `playing`/`rate`/`muted`/`captionsOn`/`recording`/`deviated` are all UI-only (never persisted into the tutorial data itself). */
type TutorialState = {
  readonly activeTutorialId: string | null;
  readonly playing: boolean;
  readonly rate: number;
  readonly muted: boolean;
  readonly captionsOn: boolean;
  readonly recording: boolean;
  /** 🧲️ True while the user has diverged from the recorded state during playback (auto-pauses); pressing play again converges the camera over `TUTORIAL_CONVERGE_MS` before resuming. */
  readonly deviated: boolean;
};

/** 🎥️ Precomputed, shell-native restore point for `APPLY_TUTORIAL_UI_SNAPSHOT` — every cross-slice lookup (window-kind labels, layout conversion) is resolved by the caller (`applyTutorialUiSnapshotToShell`, which has `session`/`appLabelsOverlay` in scope) so every slice reducer stays pure/local. */
type TutorialShellUiSnapshot = {
  readonly activeWindowId: string | null;
  readonly shellLayout: WindowLayoutNode | null;
  readonly extraWindowInstances: readonly ExtraWindowInstance[];
  readonly panelPatches: Partial<Record<Anchor, { readonly visible: boolean; readonly path: readonly string[] }>>;
  readonly treeOpenStates: Readonly<Record<string, boolean>>;
  readonly activeUtilityByWindowId: Readonly<Record<string, string | null>>;
  readonly activeToolId: string | null;
  readonly openDialogId: string | null;
  readonly commandPanelOpen: boolean;
};

type UiPrefsState = {
  readonly uiAppearance: ElementsSurfaceAppearance;
  readonly uiLayout: UiChromeLayout;
  readonly uiDriverId: string;
  readonly uiCustomDrivers: Record<string, UiDriver>;
  readonly uiDriverDraft: UiDriver | null;
  readonly uiLocale: UiLocale;
  readonly uiTerminology: string;
  readonly uiThemeId: string;
  readonly uiCustomThemes: Record<string, UiTheme>;
  readonly uiThemeDraft: UiTheme | null;
  readonly uiKeybindingOverrides: Record<string, string>;
};

type SyncState = {
  readonly syncBackboneUri: string | null;
  readonly syncCardKind: SyncCardKind | null;
  readonly syncDraftPath: string;
  /** 🚦️ Per-document sync health fed by `🟦️backbone-🟦️worker.ts`'s `DocumentEvent::Status` events, keyed by `documentId`. */
  readonly syncStatusByDocumentId: Readonly<Record<string, ArtifactSyncStatus>>;
};

/**
 * 🎛️ Command palette state not already covered by the generic per-anchor `Panel` state: the command
 * whose arg form is expanded one level above the command list (exclusive — only one at a time), and
 * locally-buffered staged arg values (never dispatched until Execute). Which category is active/folded
 * lives in `layout.panels["bottom-middle"]` like any other anchor — see `buildCommandCategoryTabs`.
 */
export type CommandPanelState = {
  readonly expandedCommandId: string | null;
  readonly stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
};

export type ShellState = {
  readonly pluginRuntime: PluginRuntimeState;
  readonly windowUi: WindowUiState;
  readonly spawnedWindow: SpawnedWindowState;
  readonly actionPane: ActionPaneState;
  readonly commandPanel: CommandPanelState;
  readonly layout: ShellLayoutState;
  readonly overlays: OverlayState;
  readonly tutorial: TutorialState;
  readonly uiPrefs: UiPrefsState;
  readonly sync: SyncState;
};
//#endregion slice shapes

//#region actions
/** 🌀️ A `useState`-style value-or-updater payload, kept so every migrated `setXxx` call-site can dispatch its existing `value` or `(prev) => next` argument unchanged. */
type Updatable<T> = T | ((prev: T) => T);

const resolveUpdatable = <T,>(next: Updatable<T>, prev: T): T => (typeof next === "function" ? (next as (prev: T) => T)(prev) : next);

export type ShellAction =
  | { readonly type: "UPSERT_LOADED_PLUGIN"; readonly value: LoadedProgramState }
  | { readonly type: "REMOVE_LOADED_PLUGIN"; readonly pluginId: string }
  | { readonly type: "SET_PLUGIN_STATUS"; readonly pluginId: string; readonly value: PluginPanelStatus }
  | { readonly type: "SET_PLUGIN_SUPERVISOR"; readonly pluginId: string; readonly value: PluginSupervisorState }
  | { readonly type: "SET_SESSION"; readonly value: Updatable<ActiveSession | null> }
  | { readonly type: "SET_ERROR"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_WINDOW_UI_BY_WINDOW_ID"; readonly value: Updatable<Readonly<Record<string, UiNode>>> }
  | { readonly type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_WINDOW_MEASURES_BY_WINDOW_ID"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_PANEL_UI_BY_KEY"; readonly value: Updatable<Readonly<Record<string, UiNode>>> }
  | { readonly type: "SET_APP_LABELS_OVERLAY"; readonly value: Updatable<PluginAppLabelsOverlay> }
  | { readonly type: "SET_SPAWNED_WINDOW_UI"; readonly value: Updatable<UiNode | null> }
  | { readonly type: "SET_SPAWNED_WINDOW_ENGAGEMENTS"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_SPAWNED_WINDOW_MEASURES"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_ACTION_PANE_FOLDED"; readonly windowId: string; readonly value: boolean }
  | { readonly type: "SET_ACTION_PANE_EXPANDED"; readonly windowId: string; readonly value: string | null }
  | { readonly type: "STAGE_ACTION_ARG"; readonly windowId: string; readonly actionId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_ACTION_ARGS"; readonly windowId: string; readonly actionId: string }
  | { readonly type: "SET_ACTIVE_UTILITY"; readonly windowId: string; readonly utilityId: string | null }
  | { readonly type: "SET_ACTIVE_TOOL"; readonly toolId: string | null }
  | { readonly type: "SET_TOOL_MEASURES_BY_TOOL_ID"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_COMMAND_EXPANDED"; readonly value: string | null }
  | { readonly type: "STAGE_COMMAND_ARG"; readonly commandId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_COMMAND_ARGS"; readonly commandId: string }
  | { readonly type: "SET_PANEL_VISIBLE"; readonly anchor: Anchor; readonly value: Updatable<boolean> }
  | { readonly type: "SET_PANEL_SIZE"; readonly anchor: Anchor; readonly value: Updatable<number> }
  | { readonly type: "SET_PANEL_PATH"; readonly anchor: Anchor; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_DOCK_OVERRIDE"; readonly value: DockSkeleton | null }
  | { readonly type: "SET_PANEL_PATH_MEMORY"; readonly value: Updatable<Readonly<Record<string, string>>> }
  | { readonly type: "SET_TREE_OPEN_STATE"; readonly id: string; readonly open: boolean }
  | { readonly type: "HYDRATE_DOCK_UI"; readonly value: DockUiState | null }
  | { readonly type: "RESET_DOCK" }
  | { readonly type: "SET_ACTIVE_WINDOW_ID"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SHELL_LAYOUT"; readonly value: Updatable<WindowLayoutNode | null> }
  | { readonly type: "SET_ACTIVE_EXAMPLE_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_MOBILE_PANEL_PATH"; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_MOBILE_PANEL_VISIBLE"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_EXTRA_WINDOW_INSTANCES"; readonly value: Updatable<readonly ExtraWindowInstance[]> }
  | { readonly type: "SET_WINDOW_TITLE"; readonly windowId: string; readonly title: string }
  | { readonly type: "SET_WINDOW_ICON"; readonly windowId: string; readonly iconId: IconName }
  | { readonly type: "SET_SEARCH_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_FIND_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_INTRODUCTION_STEP"; readonly value: Updatable<number | null> }
  | { readonly type: "COMPLETE_INTRODUCTION_INTERACTION"; readonly index: number }
  | { readonly type: "SET_DIALOG"; readonly value: OverlayState["dialog"] }
  | { readonly type: "SET_TUTORIAL"; readonly value: string | null }
  | { readonly type: "SET_TUTORIAL_PLAYING"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_TUTORIAL_RATE"; readonly value: number }
  | { readonly type: "SET_TUTORIAL_MUTED"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_TUTORIAL_CAPTIONS"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_TUTORIAL_RECORDING"; readonly value: boolean }
  | { readonly type: "SET_TUTORIAL_DEVIATED"; readonly value: boolean }
  | { readonly type: "APPLY_TUTORIAL_UI_SNAPSHOT"; readonly snapshot: TutorialShellUiSnapshot }
  | { readonly type: "SET_UI_APPEARANCE"; readonly value: Updatable<ElementsSurfaceAppearance> }
  | { readonly type: "SET_UI_LAYOUT"; readonly value: Updatable<UiChromeLayout> }
  | { readonly type: "SET_UI_DRIVER_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_CUSTOM_DRIVERS"; readonly value: Updatable<Record<string, UiDriver>> }
  | { readonly type: "SET_UI_DRIVER_DRAFT"; readonly value: Updatable<UiDriver | null> }
  | { readonly type: "SET_UI_LOCALE"; readonly value: Updatable<UiLocale> }
  | { readonly type: "SET_UI_TERMINOLOGY"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_THEME_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_CUSTOM_THEMES"; readonly value: Updatable<Record<string, UiTheme>> }
  | { readonly type: "SET_UI_THEME_DRAFT"; readonly value: Updatable<UiTheme | null> }
  | { readonly type: "SET_UI_KEYBINDING_OVERRIDES"; readonly value: Updatable<Record<string, string>> }
  | { readonly type: "SET_SYNC_BACKBONE_URI"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SYNC_CARD_KIND"; readonly value: Updatable<SyncCardKind | null> }
  | { readonly type: "SET_SYNC_DRAFT_PATH"; readonly value: Updatable<string> }
  | { readonly type: "SET_SYNC_STATUS_FOR_DOCUMENT"; readonly documentId: string; readonly status: ArtifactSyncStatus };
//#endregion actions

//#region slice reducers
function pluginRuntimeReducer(state: PluginRuntimeState, action: ShellAction): PluginRuntimeState {
  switch (action.type) {
    case "UPSERT_LOADED_PLUGIN": {
      const index = state.loadedPlugins.findIndex((entry) => entry.handle.pluginId === action.value.handle.pluginId);
      const loadedPlugins = index === -1 ? [...state.loadedPlugins, action.value] : state.loadedPlugins.map((entry, i) => (i === index ? action.value : entry));
      return { ...state, loadedPlugins };
    }
    case "REMOVE_LOADED_PLUGIN":
      return { ...state, loadedPlugins: state.loadedPlugins.filter((entry) => entry.handle.pluginId !== action.pluginId) };
    case "SET_PLUGIN_STATUS":
      return { ...state, pluginStatusById: { ...state.pluginStatusById, [action.pluginId]: action.value } };
    case "SET_PLUGIN_SUPERVISOR":
      return { ...state, pluginSupervisorById: { ...state.pluginSupervisorById, [action.pluginId]: action.value } };
    case "SET_SESSION":
      return { ...state, session: resolveUpdatable(action.value, state.session) };
    case "SET_ERROR":
      return { ...state, error: resolveUpdatable(action.value, state.error) };
    default:
      return state;
  }
}

function windowUiReducer(state: WindowUiState, action: ShellAction): WindowUiState {
  switch (action.type) {
    case "SET_WINDOW_UI_BY_WINDOW_ID":
      return { ...state, windowUiByWindowId: resolveUpdatable(action.value, state.windowUiByWindowId) };
    case "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID":
      return { ...state, windowEngagementsByWindowId: resolveUpdatable(action.value, state.windowEngagementsByWindowId) };
    case "SET_WINDOW_MEASURES_BY_WINDOW_ID":
      return { ...state, windowMeasuresByWindowId: resolveUpdatable(action.value, state.windowMeasuresByWindowId) };
    case "SET_TOOL_MEASURES_BY_TOOL_ID":
      return { ...state, toolMeasuresByToolId: resolveUpdatable(action.value, state.toolMeasuresByToolId) };
    case "SET_PANEL_UI_BY_KEY":
      return { ...state, panelUiByKey: resolveUpdatable(action.value, state.panelUiByKey) };
    case "SET_APP_LABELS_OVERLAY":
      return { ...state, appLabelsOverlay: resolveUpdatable(action.value, state.appLabelsOverlay) };
    default:
      return state;
  }
}

function spawnedWindowReducer(state: SpawnedWindowState, action: ShellAction): SpawnedWindowState {
  switch (action.type) {
    case "SET_SPAWNED_WINDOW_UI":
      return { ...state, spawnedWindowUi: resolveUpdatable(action.value, state.spawnedWindowUi) };
    case "SET_SPAWNED_WINDOW_ENGAGEMENTS":
      return { ...state, spawnedWindowEngagements: resolveUpdatable(action.value, state.spawnedWindowEngagements) };
    case "SET_SPAWNED_WINDOW_MEASURES":
      return { ...state, spawnedWindowMeasures: resolveUpdatable(action.value, state.spawnedWindowMeasures) };
    default:
      return state;
  }
}

/** 🧰️ Reducer for the per-window Action rail slice (P1–P5). Every case preserves referential identity when nothing actually changes so downstream memos can bail. */
function actionPaneReducer(state: ActionPaneState, action: ShellAction): ActionPaneState {
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
      if ((state.activeUtilityByWindowId[action.windowId] ?? null) === action.utilityId) return state;
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
function commandPanelReducer(state: CommandPanelState, action: ShellAction): CommandPanelState {
  switch (action.type) {
    case "SET_COMMAND_EXPANDED": {
      if (state.expandedCommandId === action.value) return state;
      return { ...state, expandedCommandId: action.value };
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

function shellLayoutReducer(state: ShellLayoutState, action: ShellAction): ShellLayoutState {
  switch (action.type) {
    case "SET_PANEL_VISIBLE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], visible: resolveUpdatable(action.value, state.panels[action.anchor].visible) } } };
    case "SET_PANEL_SIZE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], size: resolveUpdatable(action.value, state.panels[action.anchor].size) } } };
    case "SET_PANEL_PATH":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], path: resolveUpdatable(action.value, state.panels[action.anchor].path) } } };
    case "SET_DOCK_OVERRIDE":
      return { ...state, dockOverride: action.value };
    case "SET_PANEL_PATH_MEMORY":
      return { ...state, panelPathMemory: resolveUpdatable(action.value, state.panelPathMemory) };
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
      return { ...state, activeWindowId: resolveUpdatable(action.value, state.activeWindowId) };
    case "SET_SHELL_LAYOUT":
      return { ...state, shellLayout: resolveUpdatable(action.value, state.shellLayout) };
    case "SET_ACTIVE_EXAMPLE_ID":
      return { ...state, activeExampleId: resolveUpdatable(action.value, state.activeExampleId) };
    case "SET_MOBILE_PANEL_PATH":
      return { ...state, mobilePanelPath: resolveUpdatable(action.value, state.mobilePanelPath) };
    case "SET_MOBILE_PANEL_VISIBLE":
      return { ...state, mobilePanelVisible: resolveUpdatable(action.value, state.mobilePanelVisible) };
    case "SET_EXTRA_WINDOW_INSTANCES":
      return { ...state, extraWindowInstances: resolveUpdatable(action.value, state.extraWindowInstances) };
    case "SET_WINDOW_TITLE": {
      const windowTitlesById = { ...state.windowTitlesById, [action.windowId]: action.title };
      const extraWindowInstances = state.extraWindowInstances.map((entry) => (entry.id === action.windowId ? { ...entry, title: action.title } : entry));
      return { ...state, windowTitlesById, extraWindowInstances };
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

function overlayReducer(state: OverlayState, action: ShellAction): OverlayState {
  switch (action.type) {
    case "SET_SEARCH_OPEN":
      return { ...state, searchOpen: resolveUpdatable(action.value, state.searchOpen) };
    case "SET_FIND_OPEN":
      return { ...state, findOpen: resolveUpdatable(action.value, state.findOpen) };
    case "SET_INTRODUCTION_STEP": {
      const nextIndex = resolveUpdatable(action.value, state.introductionStepIndex);
      return { ...state, introductionStepIndex: nextIndex, introductionCompletedInteractions: [] };
    }
    case "COMPLETE_INTRODUCTION_INTERACTION":
      return state.introductionCompletedInteractions.includes(action.index)
        ? state
        : { ...state, introductionCompletedInteractions: [...state.introductionCompletedInteractions, action.index] };
    // 🎥️ Starting a tutorial (a non-null id) is mutually exclusive with an active introduction — mirrors
    // the `SET_INTRODUCTION_STEP` case below clearing the tutorial slice.
    case "SET_TUTORIAL":
      return action.value != null && state.introductionStepIndex != null ? { ...state, introductionStepIndex: null, introductionCompletedInteractions: [] } : state;
    case "SET_DIALOG":
      return { ...state, dialog: action.value };
    // 🎥️ `commandPanelOpen`/`openDialogId` restore onto the existing `searchOpen`/`dialog` fields — a
    // tutorial snapshot's "command panel" IS the shell's command palette (`UISearch`), and a dialog
    // restore only ever carries the id (seed args are not part of `TutorialUiSnapshot`).
    case "APPLY_TUTORIAL_UI_SNAPSHOT":
      return { ...state, dialog: action.snapshot.openDialogId ? { dialogId: action.snapshot.openDialogId } : null, searchOpen: action.snapshot.commandPanelOpen };
    default:
      return state;
  }
}

function uiPrefsReducer(state: UiPrefsState, action: ShellAction): UiPrefsState {
  switch (action.type) {
    case "SET_UI_APPEARANCE":
      return { ...state, uiAppearance: resolveUpdatable(action.value, state.uiAppearance) };
    case "SET_UI_LAYOUT":
      return { ...state, uiLayout: resolveUpdatable(action.value, state.uiLayout) };
    case "SET_UI_DRIVER_ID":
      return { ...state, uiDriverId: resolveUpdatable(action.value, state.uiDriverId) };
    case "SET_UI_CUSTOM_DRIVERS":
      return { ...state, uiCustomDrivers: resolveUpdatable(action.value, state.uiCustomDrivers) };
    case "SET_UI_DRIVER_DRAFT":
      return { ...state, uiDriverDraft: resolveUpdatable(action.value, state.uiDriverDraft) };
    case "SET_UI_LOCALE":
      return { ...state, uiLocale: resolveUpdatable(action.value, state.uiLocale) };
    case "SET_UI_TERMINOLOGY":
      return { ...state, uiTerminology: resolveUpdatable(action.value, state.uiTerminology) };
    case "SET_UI_THEME_ID":
      return { ...state, uiThemeId: resolveUpdatable(action.value, state.uiThemeId) };
    case "SET_UI_CUSTOM_THEMES":
      return { ...state, uiCustomThemes: resolveUpdatable(action.value, state.uiCustomThemes) };
    case "SET_UI_THEME_DRAFT":
      return { ...state, uiThemeDraft: resolveUpdatable(action.value, state.uiThemeDraft) };
    case "SET_UI_KEYBINDING_OVERRIDES":
      return { ...state, uiKeybindingOverrides: resolveUpdatable(action.value, state.uiKeybindingOverrides) };
    default:
      return state;
  }
}

function syncReducer(state: SyncState, action: ShellAction): SyncState {
  switch (action.type) {
    case "SET_SYNC_BACKBONE_URI":
      return { ...state, syncBackboneUri: resolveUpdatable(action.value, state.syncBackboneUri) };
    case "SET_SYNC_CARD_KIND":
      return { ...state, syncCardKind: resolveUpdatable(action.value, state.syncCardKind) };
    case "SET_SYNC_DRAFT_PATH":
      return { ...state, syncDraftPath: resolveUpdatable(action.value, state.syncDraftPath) };
    case "SET_SYNC_STATUS_FOR_DOCUMENT":
      return { ...state, syncStatusByDocumentId: { ...state.syncStatusByDocumentId, [action.documentId]: action.status } };
    default:
      return state;
  }
}

/** 🎥️ Reducer for the tutorial playback/recording slice — see `TutorialState`. `SET_TUTORIAL` resets every playback-rate/deviation flag (a fresh tutorial never inherits the previous one's rate/deviation); starting an introduction (`SET_INTRODUCTION_STEP` with a non-null value) clears the active tutorial, mirroring `overlayReducer`'s reverse case. */
function tutorialReducer(state: TutorialState, action: ShellAction): TutorialState {
  switch (action.type) {
    case "SET_TUTORIAL":
      return { activeTutorialId: action.value, playing: false, rate: 1, muted: state.muted, captionsOn: state.captionsOn, recording: false, deviated: false };
    case "SET_TUTORIAL_PLAYING": {
      const nextPlaying = resolveUpdatable(action.value, state.playing);
      return { ...state, playing: nextPlaying, deviated: nextPlaying ? false : state.deviated };
    }
    case "SET_TUTORIAL_RATE":
      return { ...state, rate: action.value };
    case "SET_TUTORIAL_MUTED":
      return { ...state, muted: resolveUpdatable(action.value, state.muted) };
    case "SET_TUTORIAL_CAPTIONS":
      return { ...state, captionsOn: resolveUpdatable(action.value, state.captionsOn) };
    case "SET_TUTORIAL_RECORDING":
      return { ...state, recording: action.value };
    case "SET_TUTORIAL_DEVIATED":
      return { ...state, deviated: action.value };
    // 🎓️ Only literal (non-updater) values can be checked here without this slice's own prior value —
    // every real call site dispatches a literal step index or `null` (never a functional updater), so
    // this conservatively no-ops on the (currently unused) updater form rather than guessing.
    case "SET_INTRODUCTION_STEP":
      return typeof action.value !== "function" && action.value != null && state.activeTutorialId != null ? { ...state, activeTutorialId: null, playing: false, deviated: false } : state;
    default:
      return state;
  }
}
//#endregion slice reducers

/** 🧵️ Root reducer for `FrameworkOsShell` — fans every action out to its owning slice reducer; slices that ignore an action's type return their input unchanged, so unrelated slices keep referential identity. */
export function shellReducer(state: ShellState, action: ShellAction): ShellState {
  return {
    pluginRuntime: pluginRuntimeReducer(state.pluginRuntime, action),
    windowUi: windowUiReducer(state.windowUi, action),
    spawnedWindow: spawnedWindowReducer(state.spawnedWindow, action),
    actionPane: actionPaneReducer(state.actionPane, action),
    commandPanel: commandPanelReducer(state.commandPanel, action),
    layout: shellLayoutReducer(state.layout, action),
    overlays: overlayReducer(state.overlays, action),
    tutorial: tutorialReducer(state.tutorial, action),
    uiPrefs: uiPrefsReducer(state.uiPrefs, action),
    sync: syncReducer(state.sync, action),
  };
}

//#region selectors
export const selectUiDevice = (state: ShellState, mobile: boolean): ElementsSurfaceDevice => (mobile ? "mobile" : state.uiPrefs.uiLayout);
//#endregion selectors

/** 🌱️ Builds the starting `ShellState` for `FrameworkOsShell`, mirroring exactly what each migrated `useState` used to initialize to (including reads from the shell's own storage for UI prefs). */
export function initialShellState(_props: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly storage: StoragePort;
}): ShellState {
  const locks = _props.locks ?? {};
  const defaults = _props.defaults ?? {};
  const storage = _props.storage;
  return {
    pluginRuntime: { loadedPlugins: [], pluginStatusById: {}, pluginSupervisorById: {}, session: null, error: null },
    windowUi: { windowUiByWindowId: {}, windowEngagementsByWindowId: {}, windowMeasuresByWindowId: {}, toolMeasuresByToolId: {}, panelUiByKey: {}, appLabelsOverlay: EMPTY_APP_LABELS_OVERLAY },
    spawnedWindow: { spawnedWindowUi: null, spawnedWindowEngagements: {}, spawnedWindowMeasures: {} },
    actionPane: { foldedByWindowId: {}, expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: {}, activeToolId: null },
    commandPanel: { expandedCommandId: null, stagedArgsByCommandId: {} },
    layout: {
      panels: Object.fromEntries(ANCHORS.map((anchor) => [anchor, { visible: false, size: DEFAULT_PANEL_WIDTH_PX, path: [] }])) as Record<Anchor, PanelState>,
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
    overlays: { searchOpen: false, findOpen: false, introductionStepIndex: null, introductionCompletedInteractions: [], dialog: null },
    tutorial: { activeTutorialId: null, playing: false, rate: 1, muted: false, captionsOn: true, recording: false, deviated: false },
    uiPrefs: {
      // 🐚️ No more `ephemeral ? default : readStored...()` branching here — `storage` already resolves
      // to an empty, this-shell-only memory store for an ephemeral shell (see `resolveShellScopeStorage`),
      // so a fresh read naturally falls through to each reader's own built-in default.
      uiAppearance: locks.appearance ?? readStoredUiChromeAppearance(storage),
      uiLayout: readStoredUiChromeLayout(storage),
      uiDriverId: readStoredUiDriverId(storage),
      uiCustomDrivers: readStoredUiCustomDrivers(storage),
      uiDriverDraft: null,
      uiLocale: locks.locale ?? readStoredUiChromeLocale(storage) ?? (uiI18n.resolvedLanguage?.toLowerCase().startsWith("de") ? "de" : "en"),
      uiTerminology: locks.terminology ?? readStoredUiChromeTerminology(storage),
      uiThemeId: locks.themeId ?? readStoredUiChromeThemeId(storage) ?? "semio",
      uiCustomThemes: readStoredUiCustomThemes(storage),
      uiThemeDraft: null,
      uiKeybindingOverrides: readStoredUiKeybindingOverrides(storage),
    },
    sync: { syncBackboneUri: null, syncCardKind: null, syncDraftPath: "", syncStatusByDocumentId: {} },
  };
}
//#endregion 🧮️ShellStore

//#region Boot
export async function bootFrameworkOs(options: FrameworkOsBootOptions = {}): Promise<void> {
  const root = document.getElementById(options.rootId ?? "root");
  if (!root) throw new Error("missing #root");
  const locks = resolveShellLocks(mergeShellLockSources(options.brand?.locks, options.locks));
  const defaults = resolveShellDefaults(options.brand, options.defaults);
  const ephemeral = isEphemeralShellBrand(options.brand);
  if (ephemeral) clearDurableShellStorage();
  if (options.brand) document.title = options.brand.windowTitle;
  // 🐚️ This pre-paint bootstrap runs before any `ShellScope` exists (React hasn't mounted yet), so it
  // resolves storage the same way `resolveShellScopeStorage` will once the shell below actually mounts.
  bootstrapElementsSurfaceChromeDocument(locks.appearance ?? readStoredUiChromeAppearance(ephemeral ? createMemoryStoragePort() : createBrowserStoragePort()));
  // 🐢️ No hardcoded fallback app — an omitted `plugins` list boots the shell with an explicit
  // "no plugins available" state rather than silently picking one app.
  createRoot(root).render(<FrameworkOsShell pluginFilter={options.plugin} plugins={options.plugins ?? []} appId={options.appId} locks={locks} defaults={defaults} brand={options.brand} ownsPage />);
}
//#endregion Boot

//#region ErrorBoundary
type ShellFaultBoundaryProps = {
  readonly boundaryId: string;
  readonly fallbackLabel: UiLabel;
  readonly children: ReactNode;
  readonly onFault?: (fault: Fault | null, error: Error, boundaryId: string) => void;
  readonly retry?: () => void;
};

type ShellFaultBoundaryState = { readonly hasFault: boolean; readonly message: string; readonly fault: Fault | null };

/** @emoji 🧯️ Scoped React fault boundary — logs `[DEBUG] shell fault` and renders a localized fallback with optional retry. */
export class ShellFaultBoundary extends Component<ShellFaultBoundaryProps, ShellFaultBoundaryState> {
  constructor(props: ShellFaultBoundaryProps) {
    super(props);
    this.state = { hasFault: false, message: "", fault: null };
  }

  static getDerivedStateFromError(error: Error): ShellFaultBoundaryState {
    const fault = error instanceof SemioFaultError ? error.fault : null;
    return { hasFault: true, message: fault?.message ?? error.message, fault };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error("[DEBUG] shell fault", this.props.boundaryId, error, info.componentStack);
    this.props.onFault?.(error instanceof SemioFaultError ? error.fault : null, error, this.props.boundaryId);
  }

  private readonly retry = () => {
    this.setState({ hasFault: false, message: "", fault: null });
    this.props.retry?.();
  };

  render() {
    if (this.state.hasFault) {
      return (
        <div className="p-double" role="alert" data-shell-fault-boundary={this.props.boundaryId}>
          <p className="text-sm text-destructive">
            {this.props.fallbackLabel}: {this.state.message}
          </p>
          {this.props.retry ? <Button className="mt-single" size="sm" text={shellLabel("ui.common.retry")} onClick={this.retry} /> : null}
        </div>
      );
    }
    return this.props.children;
  }
}
//#endregion ErrorBoundary
