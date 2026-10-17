/** 🗃️ Shell state shapes: every slice of the shell store (plugin runtime, windows, action panes, panels, layout, overlays, tutorial, preferences, sync, inference, merge, command panel) and the updatable field helpers. */
import type { ShellDialogV1 } from "../../🏛️ShellHost/🗨️dialog-origin/🟦️.ts";
import { type Anchor, type ElementsSurfaceAppearance, type IconName, type UiChromeLayout, type UiDriver, type UiLocale, type UiTheme, type WindowLayoutNode } from "@semio-tech/ui-react";
import { type AppRole, type Conflict, type ConflictId, type DockSkeleton, type MergePolicy, type AppCatalogue, type PluginAppLabelsOverlay, type Severity, type BuiltNode, type WindowEngagement, type WindowMeasure } from "@semio-tech/framework";
import { type ArtifactSyncStatus } from "@semio-tech/framework/backbone";
import { type InstalledServiceStatusV1 } from "@semio-tech/framework/inference/service";
import { DEFAULT_PANEL_WIDTH_PX } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import type { WindowFault } from "../../🏛️ShellHost/🩺️fault/🟦️.ts";
import type { UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import type { ActiveSession, LoadedProgramState, PluginPanelStatus, PluginSupervisorState } from "../📦️program-sessions/🟦️.ts";
import type { SyncCardKind } from "../👥️peer-interaction/🟦️.ts";

/** 🧮️ Single consolidated `useReducer` state tree for `FrameworkOsShell`, replacing what used to be ~38 independent `useState` calls with one dispatch-driven store, grouped by concern. */
//#region 🧮️ShellStore
//#region slice shapes
export type PluginRuntimeState = {
  readonly loadedPlugins: readonly LoadedProgramState[];
  readonly pluginStatusById: Readonly<Record<string, PluginPanelStatus>>;
  readonly pluginSupervisorById: Readonly<Record<string, PluginSupervisorState>>;
  readonly session: ActiveSession | null;
  readonly error: string | null;
  /** 🩺️ The classified cause behind {@link PluginRuntimeState.error}, moved in lockstep with it by
   * the one `SET_ERROR` case so a banner and its discriminator can never drift apart. */
  readonly sessionFault: WindowFault | null;
  /** 🩺️ A classified fault raised by a BACKGROUND command against the live instance (today the
   * conflicts roster read), which leaves the canvas mounted — so it stamps the window bodies rather
   * than replacing them, unlike {@link PluginRuntimeState.sessionFault}. */
  readonly instanceFault: WindowFault | null;
};

/** 🪟️ Window UI/engagements/measures are keyed by window INSTANCE id, never by window kind — two
 * instances of the same kind (e.g. split top/perspective panes) must never share chrome or options. */
export type WindowUiState = {
  readonly windowUiByWindowId: Readonly<Record<string, BuiltNode>>;
  readonly windowEngagementsByWindowId: Readonly<Record<string, WindowEngagement>>;
  readonly windowMeasuresByWindowId: Readonly<Record<string, readonly WindowMeasure[]>>;
  /** 🛠️ Mode-level tool measures, keyed by TOOL id (never a window id) — see `DocumentApp::tool_measures`. */
  readonly toolMeasuresByToolId: Readonly<Record<string, readonly WindowMeasure[]>>;
  /** 🗂️ One body store per panel key (a spawned program's under its prefixed key). The record moves only when a panel
   * gains its store; a body that changes is loaded into that store outside render and re-renders only the mounted
   * panel tree (ticket 26/09/23 F3: writer's Inspection body re-rendered the whole shell on every keystroke). */
  readonly panelBodyStoreByKey: Readonly<Record<string, UiDocumentStore>>;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
  /** 🛍️ The active app instance's APP-STATIC operator/palette catalogue, fetched once from the reserved
   * `framework.section.catalogue` retained surface and handed to every scene host through
   * `AppCatalogueContext`. Never carried on a scene payload — see that context's own docstring. */
  readonly appCatalogue: AppCatalogue;
};

export type SpawnedWindowState = {
  /** 🪟️ Which of the focused spawned program's windows have a body, and that body's activity, keyed by the SHELL
   * window instance id (`${spawnedId}::${windowKindId}`) — one entry per window kind the spawned app declares. The
   * bodies themselves live only in the shell's per-window `📃️UiDocumentStore`s (`BuiltNodeStoreCacheV1`), loaded
   * outside render, so a refresh that changes body CONTENT alone updates the mounted `UiNodeView`s and never
   * re-renders the shell (ticket 26/09/23 F3: a puzzle3d hover re-rendered the whole shell twice, 69 ms per
   * transition). */
  readonly spawnedWindowActivityByWindowId: Readonly<Record<string, BuiltNode["activity"]>>;
  /** 🩺️ Why `spawnedWindowActivityByWindowId` went empty, when it went empty because the guest faulted
   * rather than because the windows legitimately have nothing to draw — the difference an empty body
   * cannot show. */
  readonly spawnedWindowFault: WindowFault | null;
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
export type PanelState = {
  readonly visible: boolean;
  readonly size: number;
  readonly path: readonly string[];
};

export function initialPanels(): Record<Anchor, PanelState> {
  const panel = (): PanelState => ({ visible: false, size: DEFAULT_PANEL_WIDTH_PX, path: [] });
  return { "top-left": panel(), "top-middle": panel(), "top-right": panel(), "right-middle": panel(), "bottom-right": panel(), "bottom-middle": panel(), "bottom-left": panel(), "left-middle": panel() };
}

export type ShellLayoutState = {
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

export type OverlayState = {
  readonly searchOpen: boolean;
  readonly findOpen: boolean;
  /** 🎓️ Current step of the active app's introduction walkthrough, or `null` when none is playing. */
  readonly introductionStepIndex: number | null;
  /** 🚦️ Introduction launch keys already auto-started during this shell lifetime — keeps a dismissed
   * replay-on-load tour dismissed while still allowing a different app/brand introduction to launch. */
  readonly introductionAutoStartedKeys: readonly string[];
  /** ✅️ Indices into the active step's `interactions` that are done — reset whenever the step changes. */
  readonly introductionCompletedInteractions: readonly number[];
  /** 🗨️ The open declared dialog (id + `Effect`-seeded args), or `null` when none is open. */
  readonly dialog: ShellDialogV1 | null;
  /** 🧯️ A non-blocking, auto-dismissing notice — e.g. a `"viewer.read-only"` fault arriving from the
   * host (contract freeze §2.3/§5: surfaces as a notice, never a crash). `null` when nothing to show. */
  readonly transientNotice: TransientNotice | null;
  /** 👁️✏️ Which role group the Document panel's "Open with…" section should default-expand —
   * set by the `open-artifact-with-viewer`/`open-artifact-with-editor` palette commands (contract
   * freeze §5) right before focusing the panel; `null` leaves both groups at their own default. */
  readonly openWithFocusRole: AppRole | null;
};

/** 🧯️ One non-blocking shell notice — `code` carries the originating fault code (e.g.
 * `SURFACE_FAULT_CODES.ViewerReadOnly`, or `"app.command.rejected"` for a rejected local dispatch —
 * contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C8/§C9)
 * when the notice was raised from a decoded {@link Fault}, `undefined` for a locally-raised notice.
 * `kind` widened from the original `"info" | "error"` to the full four-level {@link Severity}
 * vocabulary so a rejected dispatch's own worst level (`DispatchReport.worst`) can render as
 * `"warning"`/`"fatal"` too, not just collapse onto `"error"`. */
export type TransientNotice = {
  readonly id: number;
  readonly message: string;
  readonly kind: Severity;
  readonly code?: string;
};

/** 🎥️ Playback/recording state of the active `TutorialDefinition` — mutually exclusive with `overlays.introductionStepIndex` (see `SET_TUTORIAL`/`SET_INTRODUCTION_STEP` reducer cases). `playing`/`rate`/`muted`/`captionsOn`/`recording`/`deviated` are all UI-only (never persisted into the tutorial data itself). */
export type TutorialState = {
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
export type TutorialShellUiSnapshot = {
  readonly activeWindowId: string | null;
  readonly shellLayout: WindowLayoutNode | null;
  readonly extraWindowInstances: readonly ExtraWindowInstance[];
  readonly panelPatches: Partial<Record<Anchor, { readonly visible: boolean; readonly path: readonly string[] }>>;
  readonly treeOpenStates: Readonly<Record<string, boolean>>;
  readonly activeUtilityByWindowId: Readonly<Record<string, string | null>>;
  readonly activeToolId: string | null;
  readonly dialog: ShellDialogV1 | null;
  readonly commandPanelOpen: boolean;
};

export type UiPrefsState = {
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

export type SyncState = {
  readonly syncBackboneUri: string | null;
  readonly syncCardKind: SyncCardKind | null;
  readonly syncDraftPath: string;
  /** 🚦️ Per-document sync health fed by `🟦️backbone-🟦️worker.ts`'s `DocumentEvent::Status` events, keyed by `documentId`. */
  readonly syncStatusByDocumentId: Readonly<Record<string, ArtifactSyncStatus>>;
};

/** 💡️ Host-owned ephemeral inference-port slice, keyed by the exact hub document runtime key. It
 * is fed solely by `🏪️store/👷️worker/🟦️.ts`'s
 * `inference-port-status` responses and by the shell's own operator intents; nothing in it is ever
 * persisted, and the proposal itself reaches a document only through the hub's server-stamped
 * approval command. */
export type InferenceState = {
  readonly portByRuntimeKey: Readonly<Record<string, InstalledServiceStatusV1>>;
  /** 🎫️ The operation epoch that currently owns the worker-side port, or `null` when none does. */
  readonly operationEpoch: number | null;
  /** 🗂️ The document runtime key the live operation belongs to, or `null` when none does. */
  readonly operationRuntimeKey: string | null;
};

/** ⚖️ Merge-outcome/first-class-conflict slice (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-
 * POLICIES-AND-FIRST-CLASS-CONFLICTS` §C3/§C5/§C9) — `mergePolicy` mirrors the persisted
 * `os.config.merge-policy` setting (`🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy`),
 * `conflicts` is this authority's `Conflict` roster as of the last `MergeReport`/`Conflicts` frame,
 * `selectedConflictId` drives the `📌️ChromePanels` Conflicts panel's `🔺️DiffViewHost` preview. Whether a
 * peer batch is currently quarantined is DERIVED (see {@link selectQuarantinedConflicts}), never a
 * separate stored flag — it is exactly "any `Open` `Quarantined` conflict in this list". */
export type MergeState = {
  readonly mergePolicy: MergePolicy;
  readonly conflicts: readonly Conflict[];
  readonly selectedConflictId: ConflictId | null;
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
  readonly inference: InferenceState;
  readonly merge: MergeState;
};

//#endregion slice shapes
//#region actions
/** 🌀️ A `useState`-style value-or-updater payload, kept so every migrated `setXxx` call-site can dispatch its existing `value` or `(prev) => next` argument unchanged. */
export type Updatable<T> = T | ((prev: T) => T);

export const resolveUpdatable = <T,>(next: Updatable<T>, prev: T): T => (typeof next === "function" ? (next as (prev: T) => T)(prev) : next);

/** 🪞️ Writes one field of a slice, keeping the slice's IDENTITY when the value did not change. Every
 * slice reducer used to spread a fresh object on every action, so `shellReducer` returned a new
 * `ShellState` for every dispatch — even the eight identity-preserving dispatches one guest refresh
 * fans out (`mergeRecordPreservingIdentity` kept the entries, the spread threw the bail-out away) —
 * and `useReducer` re-rendered the whole shell tree on every hover echo, measured at ~20 ms of chrome
 * reconcile per pointer move. A same-value write is now a no-op React bails out of. */
export function withField<S extends object, K extends keyof S>(state: S, key: K, value: S[K]): S {
  return Object.is(state[key], value) ? state : { ...state, [key]: value };
}
//#endregion actions
//#endregion 🧮️ShellStore
