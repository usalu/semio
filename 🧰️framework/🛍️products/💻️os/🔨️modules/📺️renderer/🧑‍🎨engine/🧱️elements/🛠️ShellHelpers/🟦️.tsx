// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHelpers/component.tsx
/** @emoji 🧰️ `ShellHelpers` — shared plumbing behind the framework OS shell orchestrator
 * ({@link ../ShellHost}): action-history/reserved-id bookkeeping, presence identity, UI history,
 * media-export download helpers, `requestMediaFrames`'s WebCodecs/`<video>` tiered decode pipeline,
 * window-layout-change classification, the utility-tree/command/tool registries, the tutorial UI
 * bridge, reveal-cutoff store, the window action pane, and the plugin UI-refresh cache. No single
 * exported component here — a grab bag of the functions/types `ShellHost` and sibling elements need.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { ShellDialogV1 } from "../🏛️ShellHost/🗨️dialog-origin/🟦️.ts";
import { segmentedDownloadSinkFactory, type SegmentedDownloadSinkFactory } from "../📤️SegmentedDownload/🟦️.ts";
import React, {
  type KeyboardEvent,
  type ReactElement,
  type ReactNode,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import {
  isIconName,
} from "@semio-tech/assets";
import {
  type ActionArgControl,
  type ActionArgDef,
  // 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6:
  // `ActionArgDef.control` is gone (derived, not stored) — every reader below now calls this instead.
  argControl,
  artifactKindChoices,
  encodeArtifactKindChoice,
  actionSemanticsForKind,
  type ActionDefinition,
  type ActionDescriptor,
  type ActionInvocation,
  type AppDefinition,
  type AppModeDefinition,
  type AppPanelTabDefinition,
  type AppRef,
  type AppRole,
  type AppRouter,
  type AppWindowKindDefinition,
  type ArtifactKindChoice,
  type ArtifactDialect,
  type BuiltNode,
  dialectCoordinate,
  type CommandAddress,
  type CommandDefinition,
  CONTEXT_MENU_GROUP_ID_PREFIX,
  CONTEXT_MENU_OVERFLOW_CATEGORY,
  type CommandInvocation,
  type DerivedUtilitySpec,
  deriveUtilityNodes,
  type DialogDefinition,
  DockLayoutStore,
  DockUiStateStore,
  effectiveActionArgs,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  FRAMEWORK_HISTORY_BODY_KEY,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  type Effect,
  type IntroductionDefinition,
  type IntroductionStepDefinition,
  type LocalizedLabel,
  type MergeMode,
  unresolvedActionArgs,
  type PanelTabKind,
  panelTabKindId,
  partitionWindowMeasures,
  pendingPanelUiNode,
  type AppCatalogue,
  type PluginAppLabelsOverlay,
  type PluginCatalog,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  type PluginViewState,
  type Platform,
  RECORD_TUTORIAL_ACTION_ID,
  resolvePluginHostConfig,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_UTILITY_ACTION_ID,
  SHELL_LOCALES,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  type ToolDefinition,
  type TutorialUiChange,
  type TutorialUiSnapshot,
  type UiDirtyScope,
  type UiIntent,
  type UtilityDefinition,
  type UtilityNode,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowLayout,
  type WindowLayoutAxisNode,
  type WindowLayoutStackNode,
  type WindowLayoutWindowNode,
  type WindowStackCorner,
  type WindowMeasure,
  blake3Hex,
} from "@semio-tech/framework";
import {
  type ArtifactSyncStatus,
  packValueFromBase64,
  packValueToBase64,
} from "@semio-tech/framework-os";
import { type UiPreferencesConfigMutation, setAppearance, setDriver, setLayout, setLocale, setTerminology, setTheme } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import type { DomainSelection, InteractionState } from "../../../../../../../🔨️modules/🕹️interaction/🟦️.ts";
import { hostContinuations, type ContinuationCancel, type ContinuationScheduler } from "../../../../../../../🔨️modules/⏳️async/🪃️continuation/🟦️.ts";
import { GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES } from "../../../../../../../🔨️modules/⏱️trace/🧮️memory/🟦️.ts";
import {
  decodeWorldProjectionTemplateId,
} from "@semio-tech/infinite-world-r3f";
import {
  type Anchor,
  ANCHORS,
  builtinUiDrivers,
  childElementId,
  ChromeAwareWindowScrollSurface,
  classifyIconSelectorMode,
  createEvenWindowLayout,
  elementIdSegment,
  type ElementsSurfaceAppearance,
  type EngagementControl,
  type EngagementSpec,
  Icon,
  type IconName,
  IconSelector,
  Input,
  type UIDialogFieldBinding,
  type PanelTabNode,
  resolveTranslationLabel,
  RibbonDivider,
  type SearchSpec,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  singleTreeLeaf,
  Slider,
  staticTreePanelDefinition,
  Toggle,
  ToggleGroup,
  Tree,
  TreeCheckbox,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  UI_RIBBON_PARENT_CATEGORIES,
  UI_TERMINOLOGY_NATIVE,
  type UiChromeLayout,
  type UiChromeTerminologyId,
  uiDataLabel,
  type UiDriver,
  uiI18n,
  type UiLabel,
  type UiLocale,
  type UiRibbonParentCategory,
  type UiTheme,
  type UiTranslationKey,
  useLabel,
  useShellScope,
  type WindowLayoutNode,
  WindowMeasuresTree,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
} from "@semio-tech/ui-react";
import {
  InterpretedUiNode,
  wireLabel,
} from "../🗣️Interpreter/🟦️.tsx";
import { builtNodeToSnapshot, UiDocumentStore } from "../📃️UiDocumentStore/🟦️.tsx";
import {
  type ActionPaneState,
  actionStageKey,
  type ActiveSession,
  EMPTY_SHELL_LOCKS,
  type ExtraWindowInstance,
  type LoadedProgramState,
  type PluginManifest,
  type ResolvedShellLocks,
  type ShellAction,
  ShellFaultBoundary,
  type ShellState,
  type SpacePanelState,
  type SpaceProgramEntry,
  type SpawnedAppEntry,
  type UIHistory,
  type ViewModel,
} from "../🐚️Shell/🟦️.tsx";
import {
  registerPendingWorldProjection,
  type WorldInstanceRecord,
} from "../🌐️World3dHost/🟦️.tsx";
import { groupUtilityNodesByCategory, UTILITY_CATEGORIES, UtilityTree } from "../🎛️UtilityTree/🟦️.tsx";
import { WindowMeasureSelect, WindowMeasureToggle } from "./🎚️measure-controls/🟦️.tsx";
import { loadPluginModule, pluginLoadProgressAt, SHARD_LIVENESS_POLICY, type PluginWasmHandle } from "../🔌️PluginRuntime/🟦️.tsx";
// #endregion 🔌️Adapters

//#region ShellHelpers
export function syncDocumentId(session: ActiveSession, panel: SpacePanelState | null, hostMode: boolean): string {
  if (hostMode && panel?.activeSpawnedId) {
    const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (spawned) return `${spawned.pluginId}-${spawned.instanceId}`;
  }
  return `${session.pluginId}-${session.instanceId}`;
}

/** @emoji ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel. */
export const DEFAULT_PANEL_WIDTH_PX = 300;

/** @emoji 🌳️ Root category id for the nested dock tab tree — the top row of {@link defaultDock}'s bottom-left (Display) anchor tabs; top-left (Workbench), top-right (Details) and bottom-right (Settings) render their tabs flat instead of under a category branch. */
export const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
/** @emoji 🎛️ Root category id bundling every command-category leaf under one expandable Command toggle on bottom-middle (mirrors Display on bottom-left). */
export const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";
/** @emoji 🛠️ Root category id bundling every mode-level tool leaf under one expandable Tool toggle on
 * bottom-middle, ordered left of the Command branch (mirrors Command's own bundling on the same anchor). */
export const FRAMEWORK_CATEGORY_TOOL_ID = "framework.category.tool";

/** @emoji 🎛️ Corner/top-middle/bottom-middle anchors park their *folded* root tab row in navbar/footer chrome (via {@link PanelChromeTabBar}); while open, the floating {@link Panel} hosts the full strip on its {@link WindowChrome}. The two side-middle anchors have no navbar/footer slot, so they're absent here and fall back to `"panel"` (see the `?..:"panel"` read site), carrying their own tab bar when folded too. */
export const PANEL_TAB_BAR_HOSTS: Partial<Record<Anchor, "navbar" | "footer">> = {
  "top-left": "navbar",
  "top-middle": "navbar",
  "top-right": "navbar",
  "bottom-left": "footer",
  "bottom-middle": "footer",
  "bottom-right": "footer",
};
const APP_BREADCRUMB_SEPARATOR = " · ";

/** 🧭️ Shell-only action id `World3dHost`'s `WorldOrbitGated.onNavigationGestures` dispatches through the
 * standard `onAction` funnel to report a completed pan/zoom/orbit gesture — intercepted in `onAction`
 * (never forwarded to the program), args `{ windowId: string, gestures: readonly string[] }`. */
export const NOTE_WORLD_NAVIGATION_ACTION_ID = "noteWorldNavigation";

/** 🧭️ Framework-injected action id, dispatched via `noteShellCommand` (see `onAction`'s central funnel) to
 * log a shell-chrome command (theme/appearance/locale/driver/layout change, dock drag, window
 * resize/rearrange/activate/close/split, panel toggle/tab) into the plugin's session-only command-history
 * panel — intercepted by the plugin BEFORE the app ever sees it, args `{ commandId: string, label: string,
 * detail?: unknown }`. Routed through the exact same `handleAction` dispatch path as every other action
 * (unlike {@link NOTE_WORLD_NAVIGATION_ACTION_ID}, which is fully shell-intercepted and never forwarded). */
const NOTE_SHELL_COMMAND_ACTION_ID = "noteShellCommand";

/** 🛡️ Action ids intercepted by `VcsDocumentApp::dispatch_action` before `command_from_action` — undeclared
 * surface verbs (e.g. VFS `selectRows` on Home) must not be forwarded or they hard-error the bridge. */
/** 🧾️ Equal-cursor patches still apply when they carry upserts; older cursors never clobber. */
export function historyPatchShouldApplyV1(
  currentCursor: number,
  patch: Readonly<{ cursor: number; upserts?: readonly unknown[] }>,
  replace = false,
): boolean {
  if (replace) return true;
  if (patch.cursor > currentCursor) return true;
  return patch.cursor === currentCursor && (patch.upserts?.length ?? 0) > 0;
}

/** 🔄 Catalog example switch does not carry a history_patch on the first Invocation; chrome must re-snapshot. */
export function historyRefreshNeededV1(actionId: string, patch: Readonly<{ upserts?: readonly unknown[] }> | undefined): boolean {
  return actionId === SET_ACTIVE_EXAMPLE_ACTION_ID && (patch?.upserts?.length ?? 0) === 0;
}

export const FRAMEWORK_RESERVED_ACTION_IDS: ReadonlySet<string> = new Set([
  "undo",
  "redo",
  "commitCheckpoint",
  "createAlternative",
  "switchAlternative",
  "checkoutCheckpoint",
  "copy",
  "cut",
  "paste",
  "revertToCommand",
  "setHistoryCommandFilter",
  NOTE_SHELL_COMMAND_ACTION_ID,
  "recordTutorial",
  "startIntroduction",
  "startTutorial",
  "setActiveUtility",
  "setActiveTool",
  "suggestionsTick",
  "fillBuildTick",
]);

/** 🪟️ The shape `undeclaredActionDiagnostic` reads a session app's window kinds through — the manifest's
 * own `WindowKindDefinition` structurally satisfies it, and nothing outside this codebase appears in it. */
export type WindowKindActionDeclaration = { readonly id: string; readonly actions?: readonly { readonly id: string }[] };

/** 🚨️ One rejected action dispatch, fully described. `onAction` drops any action no window kind of the
 * target app declares (`WindowKindDefinition.actions`, populated by `.window_kind_actions()` /
 * `.window_kind_action_refs()` or by `build_definition`'s unowned-action fallback) — a silent drop that
 * makes a wired button look inert with no fault anywhere. This is that drop, typed and named. */
export type UndeclaredActionDiagnostic = {
  readonly appId: string;
  readonly action: string;
  /** 🪟️ The window kind the dispatch came from, when the host could resolve one — `null` for a
   * chrome/navbar/keybinding dispatch that belongs to no window instance. */
  readonly windowKindId: string | null;
  /** 🪟️ Every window kind of the app, so the message says what WAS on offer, not just what was not. */
  readonly windowKindIds: readonly string[];
  readonly message: string;
};

/** 🚨️ Decides whether one dispatch is an undeclared-action drop, and describes it. Returns `null` when the
 * action is declared on ANY window kind of the app (the gate is app-wide on purpose: a context menu, a
 * palette entry or a keybinding may dispatch a window-owned action from anywhere) or when it is one of the
 * framework's own reserved verbs. Pure, so the message is testable without a session
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function undeclaredActionDiagnostic(appId: string, action: string, windowKinds: readonly WindowKindActionDeclaration[], windowKindId?: string | null): UndeclaredActionDiagnostic | null {
  if (FRAMEWORK_RESERVED_ACTION_IDS.has(action)) return null;
  if (windowKinds.some((kind) => (kind.actions ?? []).some((entry) => entry.id === action))) return null;
  const windowKindIds = windowKinds.map((kind) => kind.id);
  const from = windowKindId ? ` dispatched from window kind "${windowKindId}"` : "";
  return {
    appId,
    action,
    windowKindId: windowKindId ?? null,
    windowKindIds,
    message: `semio: app "${appId}" dropped action "${action}"${from}: no window kind declares it (window kinds: ${windowKindIds.join(", ") || "none"}). Declare it with .window_kind_actions()/.window_kind_action_refs() on the window that dispatches it.`,
  };
}

/** 🧭️ Builds the `noteShellCommand` action descriptor `noteShellCommand` (the component helper) dispatches
 * through the standard `onAction` funnel — pure so it's testable without a session/component. */
export function buildNoteShellCommandAction(controllerId: string, commandId: string, label: string, detail?: Record<string, unknown>): ActionDescriptor {
  return { controllerId, action: NOTE_SHELL_COMMAND_ACTION_ID, args: { commandId, label, inverseCommandId: commandId, ...(detail ? { detail, inverseArgs: detail } : {}) } };
}

/** 🎨️ The program action the navbar example picker dispatches. */
export const SET_ACTIVE_EXAMPLE_ACTION_ID = "setActiveExample";

/** 🎨️ Builds the `setActiveExample` descriptor the navbar example picker dispatches through the standard
 * `onAction` funnel — pure so the dispatched id is testable without a session/component. The picker's own
 * `SET_ACTIVE_EXAMPLE_ID` reducer write only moves the LABEL; this descriptor is the only thing that
 * reaches the program, and an empty selection means "the app's default document", never "no argument"
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function buildActiveExampleAction(controllerId: string, exampleId: string): ActionDescriptor {
  return { controllerId, action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: exampleId || "" } };
}

/** 🎨️ The example id the navbar must remember after one dispatch, given what it remembers now. Any
 * `setActiveExample` carrying a non-empty `exampleId` teaches it — not only `NavbarExampleSelect`'s own
 * `onValueChange`, because {@link navbarExampleIdFromHistoryUpserts} spends this memory on REDO, and a
 * `Set Active Example` row can be redone that this shell dispatched from the palette, a context menu, a
 * replayed shell command or the boot load. An empty `exampleId` means "the app's default document",
 * which needs no memory at all (the popped-row branch already answers with `bootExampleId`), so it
 * leaves the memory standing rather than erasing it. */
export function rememberedExampleIdFromDispatchV1(action: { readonly action: string; readonly args?: unknown }, remembered: string): string {
  if (action.action !== SET_ACTIVE_EXAMPLE_ACTION_ID) return remembered;
  const requested = typeof action.args === "object" && action.args !== null ? (action.args as { exampleId?: unknown }).exampleId : undefined;
  return typeof requested === "string" && requested ? requested : remembered;
}

/** 🎨️ Navbar example id implied by a history upsert batch. Chrome-only rows leave the label alone
 * (`undefined`). A live Set Active Example row restores `rememberedExampleId`; a popped row returns
 * `bootExampleId`. */
export function navbarExampleIdFromHistoryUpserts(
  upserts: readonly { readonly actionId?: string; readonly label?: string; readonly revertible?: boolean }[] | undefined,
  rememberedExampleId: string,
  bootExampleId: string,
): string | undefined {
  const rows = (upserts ?? []).filter((entry) => entry.actionId === SET_ACTIVE_EXAMPLE_ACTION_ID || (entry.label ?? "").includes("Set Active Example"));
  if (rows.length === 0) return undefined;
  if (rows.some((entry) => entry.revertible !== false)) return rememberedExampleId || undefined;
  return bootExampleId;
}

export type LeftoverInteractionViewV1 = {
  readonly selectedIds: readonly string[];
  readonly hoverTarget: { readonly domain: string; readonly channel: string; readonly id: string } | null;
  readonly locked: Readonly<Record<string, boolean>>;
  readonly gumballActive: boolean;
  readonly gumballAnchorId: string | null;
  readonly selection: Readonly<Record<string, { readonly granularity: string; readonly ids: readonly string[]; readonly anchorId?: string }>>;
  readonly hover: Readonly<Record<string, { readonly channel: string; readonly ids: readonly string[] }>>;
  readonly activeMode: Readonly<Record<string, string>>;
  readonly activeGranularity: Readonly<Record<string, string>>;
  readonly activeUtility?: string | null;
  readonly brushPreviewJson?: string | null;
  /** 🪟️ The window INSTANCE the action that produced this leftover addressed, straight from the
   * guest's own encode (`leftover_interaction_view_from`, `🔌️plugin/🦀️.rs`). `null` is a windowless,
   * document-scoped action — never a synthetic window surface (wave B56). */
  readonly windowId: string | null;
};

function leftoverStringRecord(value: unknown): Record<string, string> {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).filter((entry): entry is [string, string] => typeof entry[1] === "string"));
}

function leftoverLockedRecord(value: unknown): Record<string, boolean> {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).filter((entry): entry is [string, boolean] => typeof entry[1] === "boolean"));
}

function leftoverSelectionRecord(value: unknown): LeftoverInteractionViewV1["selection"] {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  const selection: Record<string, { granularity: string; ids: string[]; anchorId?: string }> = {};
  for (const [domain, raw] of Object.entries(value as Record<string, unknown>)) {
    if (!raw || typeof raw !== "object" || Array.isArray(raw)) continue;
    const record = raw as { granularity?: unknown; ids?: unknown; anchorId?: unknown };
    const ids = Array.isArray(record.ids) ? record.ids.filter((id): id is string => typeof id === "string") : [];
    selection[domain] = { granularity: typeof record.granularity === "string" ? record.granularity : "", ids, ...(typeof record.anchorId === "string" ? { anchorId: record.anchorId } : {}) };
  }
  return selection;
}

function leftoverHoverRecord(value: unknown): LeftoverInteractionViewV1["hover"] {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  const hover: Record<string, { channel: string; ids: string[] }> = {};
  for (const [domain, raw] of Object.entries(value as Record<string, unknown>)) {
    if (!raw || typeof raw !== "object" || Array.isArray(raw)) continue;
    const record = raw as { channel?: unknown; ids?: unknown };
    hover[domain] = { channel: typeof record.channel === "string" ? record.channel : "pointer", ids: Array.isArray(record.ids) ? record.ids.filter((id): id is string => typeof id === "string") : [] };
  }
  return hover;
}

/** 🕹️ Peels leftover `Invocation.output.interactionView` — same leftover lane as history_patch. */
export function interactionViewFromLeftoverOutput(output: unknown): LeftoverInteractionViewV1 | null {
  if (!output || typeof output !== "object" || Array.isArray(output)) return null;
  const envelope = output as { interactionView?: unknown; brushPreviewJson?: unknown };
  const raw = envelope.interactionView;
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return null;
  const view = raw as Record<string, unknown>;
  if (typeof envelope.brushPreviewJson === "string" && envelope.brushPreviewJson.length > 0) view.brushPreviewJson = envelope.brushPreviewJson;
  const selectedIds = Array.isArray(view.selectedIds) ? view.selectedIds.filter((id): id is string => typeof id === "string") : [];
  const hoverRaw = view.hoverTarget;
  const hoverTarget =
    hoverRaw && typeof hoverRaw === "object" && !Array.isArray(hoverRaw) && typeof (hoverRaw as { id?: unknown }).id === "string"
      ? {
          domain: typeof (hoverRaw as { domain?: unknown }).domain === "string" ? (hoverRaw as { domain: string }).domain : "",
          channel: typeof (hoverRaw as { channel?: unknown }).channel === "string" ? (hoverRaw as { channel: string }).channel : "pointer",
          id: (hoverRaw as { id: string }).id,
        }
      : null;
  const gumball = view.gumball && typeof view.gumball === "object" && !Array.isArray(view.gumball) ? (view.gumball as { active?: unknown; anchorId?: unknown }) : {};
  return {
    selectedIds,
    hoverTarget,
    locked: leftoverLockedRecord(view.locked),
    gumballActive: gumball.active === true || selectedIds.length > 0,
    gumballAnchorId: typeof gumball.anchorId === "string" ? gumball.anchorId : selectedIds[0] ?? null,
    windowId: typeof view.windowId === "string" && view.windowId.length > 0 ? view.windowId : null,
    selection: leftoverSelectionRecord(view.selection),
    hover: leftoverHoverRecord(view.hover),
    activeMode: leftoverStringRecord(view.activeMode),
    activeGranularity: leftoverStringRecord(view.activeGranularity),
    ...(typeof view.activeUtility === "string" ? { activeUtility: view.activeUtility } : {}),
    ...(typeof view.brushPreviewJson === "string" && view.brushPreviewJson.length > 0 ? { brushPreviewJson: view.brushPreviewJson } : {}),
  };
}

/** 🕹️ Host InteractionState from leftover publication — Inspection/clipboard/lock/gumball consumers. */
export function leftoverInteractionStateV1(view: LeftoverInteractionViewV1): {
  readonly selection: LeftoverInteractionViewV1["selection"];
  readonly hover: LeftoverInteractionViewV1["hover"];
  readonly activeMode: LeftoverInteractionViewV1["activeMode"];
  readonly activeGranularity: LeftoverInteractionViewV1["activeGranularity"];
} {
  const vortex = view.selection.vortex;
  const vortexEmpty = !vortex || vortex.ids.length === 0;
  const selection =
    !vortexEmpty || view.selectedIds.length === 0
      ? view.selection
      : { ...view.selection, vortex: { granularity: vortex?.granularity || view.activeGranularity.vortex || "object", ids: [...view.selectedIds], ...(vortex?.anchorId ? { anchorId: vortex.anchorId } : {}) } };
  return { selection, hover: view.hover, activeMode: view.activeMode, activeGranularity: view.activeGranularity };
}

/** 🕹️ Leftover gumball pose for World3d — transformMode + target from the selected instance. */
export function leftoverWorldGumballPoseV1(
  leftover: { readonly gumballActive: boolean; readonly gumballAnchorId: string | null; readonly ids: readonly string[] },
  instances: readonly { readonly id: string; readonly interactionId?: string; readonly position?: readonly [number, number, number]; readonly x?: number; readonly y?: number; readonly z?: number }[],
): { readonly transformMode: "move" | "transform" | undefined; readonly gumballTarget: readonly [number, number, number] | undefined } {
  const id = leftover.gumballAnchorId ?? leftover.ids[0];
  const inst = id ? instances.find((row) => row.id === id || row.interactionId === id) : undefined;
  const gumballTarget =
    inst?.position ??
    (inst && inst.x != null && inst.y != null && inst.z != null ? ([inst.x, inst.y, inst.z] as const) : undefined);
  return {
    transformMode: leftover.gumballActive || leftover.ids.length > 0 ? "transform" : undefined,
    gumballTarget,
  };
}

/** 🧭️ Action ids the tutorial recorder never captures (see `onAction`'s recorder tap) — telemetry/chrome
 * noise a tutorial replay should never literally reproduce, or actions the director/recorder itself just
 * dispatched. Exported so it's independently testable. */
export const TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS: ReadonlySet<string> = new Set([NOTE_WORLD_NAVIGATION_ACTION_ID, NOTE_SHELL_COMMAND_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, RECORD_TUTORIAL_ACTION_ID]);

export const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
export const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

function presenceIdentityPackBase64(identity: { readonly clientId: string; readonly name: string }): string {
  return packValueToBase64(identity);
}

function presenceIdentityFromPackBase64(encoded: string): { readonly clientId: string; readonly name: string } | null {
  try {
    const decoded = packValueFromBase64(encoded) as { readonly clientId?: string; readonly name?: string };
    if (decoded.clientId && decoded.name) return { clientId: decoded.clientId, name: decoded.name };
  } catch {
    return null;
  }
  return null;
}

/** 🪪️ `real`, when given, is the ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
 * shell's resolved `Identity` presence pair (`user:{userId}#{shellSessionId}` + `displayName`) —
 * ShellHost's only caller passes it once sign-in resolves, so presence/heartbeat labels show the real
 * signed-in user instead of a random per-tab `Guest ####`. Absent (no hub env, offline, or not yet
 * resolved) falls back to the pre-existing session-storage-cached guest identity unchanged. */
export function presenceClientIdentity(ephemeral = false, real?: { readonly clientId: string; readonly name: string }): { readonly clientId: string; readonly name: string } {
  if (real) return real;
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  if (!ephemeral) {
    const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
    if (stored) {
      const parsed = presenceIdentityFromPackBase64(stored);
      if (parsed) return parsed;
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  if (!ephemeral) window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, presenceIdentityPackBase64(identity));
  return identity;
}

function readBrowserUri(): string {
  if (typeof window === "undefined") return "/";
  return `${window.location.pathname}${window.location.search}` || "/";
}

export function useUIHistory(initialUri = "/", syncBrowser = false) {
  const [history, setHistory] = useState<UIHistory>(() => ({
    entries: [{ uri: syncBrowser ? readBrowserUri() : initialUri }],
    index: 0,
  }));
  const uri = history.entries[history.index]?.uri ?? initialUri;
  const canGoBack = history.index > 0;
  const canGoForward = history.index < history.entries.length - 1;
  const segments = uri.split("/").filter(Boolean);
  const canGoUp = segments.length > 0;
  const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

  const goBack = useCallback(() => {
    setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
  }, []);
  const goForward = useCallback(() => {
    setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
  }, []);
  const goUp = useCallback(() => {
    if (!canGoUp || parentUri === null) return;
    setHistory((prev) => {
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
    });
  }, [canGoUp, parentUri]);
  const navigate = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  const syncUri = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const current = `${window.location.pathname}${window.location.search}`;
    if (current !== uri) window.history.pushState(null, "", uri);
  }, [syncBrowser, uri]);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const onPopState = () => syncUri(readBrowserUri());
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [syncBrowser, syncUri]);

  return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}

export const DOWNLOAD_MEDIA_EXPORT_REVOKE_MS = 10_000;

/** @emoji 📥️ Guest `encoding` is only a segmented-download marker when it is a string prefix. */
export function mediaExportEncodingText(encoding: unknown): string | undefined {
  return typeof encoding === "string" ? encoding : undefined;
}

/** @emoji 📥️ Host download of guest `download-media-export` — the anchor must be in the document and the object URL must outlive the click or the browser drops the file. */
export function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  const payload = encoding === "base64" ? Uint8Array.from(atob(data), (char) => char.charCodeAt(0)) : data;
  const blob = new Blob([payload], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.style.display = "none";
  document.body.appendChild(anchor);
  anchor.click();
  window.setTimeout(() => {
    anchor.remove();
    URL.revokeObjectURL(url);
  }, DOWNLOAD_MEDIA_EXPORT_REVOKE_MS);
}

/** @emoji 📥️ Host delivery of already-assembled bytes — the blob-and-anchor half of {@link downloadMediaExport}, reached by the segmented lane once its chunks are drained. */
export function downloadMediaExportBytes(filename: string, mimeType: string, bytes: Uint8Array): void {
  if (typeof document === "undefined") return;
  const blob = new Blob([bytes], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.style.display = "none";
  document.body.appendChild(anchor);
  anchor.click();
  window.setTimeout(() => {
    anchor.remove();
    URL.revokeObjectURL(url);
  }, DOWNLOAD_MEDIA_EXPORT_REVOKE_MS);
}

/** @emoji 🌊 The sink factory every shell drain uses: assembled chunks delivered as one file through {@link downloadMediaExportBytes}. */
export const shellSegmentedDownloadSinkFactory: SegmentedDownloadSinkFactory = segmentedDownloadSinkFactory(downloadMediaExportBytes);

export {
  createBufferedDownloadSink,
  createSegmentedDownloadSink,
  drainSegmentedMediaExport,
  SEGMENTED_DOWNLOAD_CONTRACT,
  SEGMENTED_DOWNLOAD_REFUSAL,
  parseSegmentedDownloadMarker,
  parseSegmentedDownloadOperationId,
  SEGMENTED_DOWNLOAD_MARKER_PREFIX,
  type SegmentedDownloadEncoding,
  type SegmentedDownloadSink,
  type SegmentedDownloadSinkFactory,
} from "../📤️SegmentedDownload/🟦️.ts";

export function downloadDataUrl(filename: string, dataUrl: string): void {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}

/** 📤️ Opens the native file picker. Resolves with one entry per selected file, in selection order —
 * always an array (empty on cancel) so single-file callers just read `[0]` and `multiple` callers can
 * fan out over the whole list; single-file behavior (one `<input>`, one resolved entry) is unchanged
 * when `multiple` is false/absent. */
export function requestFileOpen(accept: string, readAs?: string, multiple?: boolean): Promise<readonly { contents: string; name: string }[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    if (multiple) input.multiple = true;
    input.onchange = async () => {
      const files = input.files ? Array.from(input.files) : [];
      if (files.length === 0) {
        resolve([]);
        return;
      }
      const opened: { contents: string; name: string }[] = [];
      for (const file of files) {
        if (readAs === "dataUrl") {
          const contents = await new Promise<string | null>((resolveFile) => {
            const reader = new FileReader();
            reader.onload = () => resolveFile(typeof reader.result === "string" ? reader.result : null);
            reader.onerror = () => resolveFile(null);
            reader.readAsDataURL(file);
          });
          if (contents !== null) opened.push({ contents, name: file.name });
          continue;
        }
        opened.push({ contents: await file.text(), name: file.name });
      }
      resolve(opened);
    };
    input.click();
  });
}

/** 🔁️ The one-action-at-a-time callback shared by the `requestFileOpen`/`dispatchAction`/
 * `requestMediaFrames` `applyHostEffects` branches: dispatches `action` against the emitting program
 * instance and feeds its own `requestedEffects` back through `applyHostEffects` recursively. */
type EffectDispatchOne = (action: string, args?: Record<string, unknown>) => Promise<void>;

/** 🎯️ Encodes a host-effect callback as the same fully scoped JSON `ActionInvocation` every other
 * renderer action uses; pack-base64 belongs inside the worker channel, never at `handleAction`'s edge. */
export function encodeEffectActionInvocation(baseSession: ActiveSession, action: string, args?: Record<string, unknown>): string {
  const windowKindId = baseSession.viewState.activeWindowKindId ?? baseSession.app.windowKinds[0]?.id ?? "";
  const windowInstanceId = baseSession.viewState.windowId ?? windowKindId;
  const invocation: ActionInvocation = {
    address: {
      pluginId: baseSession.pluginId,
      appId: baseSession.app.id,
      modeId: baseSession.viewState.activeModeId ?? baseSession.app.defaultModeId ?? baseSession.app.modes[0]?.id ?? baseSession.app.id,
      windowKindId,
      windowInstanceId,
      actionId: action,
    },
    arguments: { ...args, windowId: windowInstanceId },
  };
  return JSON.stringify(invocation);
}

/** 🎯️ Encodes a recursively requested app command with its manifest owner. */
export function encodeEffectCommandInvocation(baseSession: ActiveSession, commandId: string, args?: Record<string, unknown>): string {
  const invocation: CommandInvocation = {
    address: { owner: { app: { pluginId: baseSession.pluginId, appId: baseSession.app.id } }, commandId },
    arguments: { ...args },
  };
  return JSON.stringify(invocation);
}

/** 🔁️ Builds an {@link EffectDispatchOne} bound to one plugin instance + `applyHostEffects` closure;
 * declared app commands re-enter the typed command channel and framework/window actions retain their
 * scoped action channel.
 *
 * 🗣️ `resolveViewState` is REQUIRED, and is the reason this takes a resolver instead of reading
 * `baseSession.viewState`: an `ActiveSession`'s stored view state is the shell's own partial
 * projection (`{ activeModeId }` at session establishment) and carries no `locale`/`terminology`,
 * while the guest's `ViewModel` declares both NON-optional. A recursively requested effect
 * (`dispatchAction`, `requestFileOpen`'s import, `requestMediaFrames`) dispatched with that raw
 * projection faulted in the guest with the anonymous `missing field \`locale\`` — and a
 * `scheduleDispatchAction` one is `void`-dispatched, so the fault surfaced only as an unhandled
 * `SemioFaultError` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, the deferred `flowEvalTick` re-arm).
 * Host owners pass `resolvedTargetViewState`, the single admission that stamps both preferences. */
export function makeEffectDispatchOne(
  pluginEntry: LoadedProgramState,
  baseSession: ActiveSession,
  applyEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope?: UiDirtyScope) => Promise<void>,
  isCurrent: () => boolean,
  resolveViewState: (session: ActiveSession) => ViewModel,
): EffectDispatchOne {
  return async (action, args) => {
    if (!isCurrent()) return;
    const viewState = resolveViewState(baseSession);
    const isAppCommand = (baseSession.app.commands ?? []).some((command) => command.id === action);
    const response = isAppCommand && pluginEntry.handle.handleCommand
      ? await pluginEntry.handle.handleCommand(baseSession.instanceId, encodeEffectCommandInvocation(baseSession, action, args), viewState)
      : await pluginEntry.handle.handleAction(baseSession.instanceId, encodeEffectActionInvocation(baseSession, action, args), viewState);
    if (isCurrent()) await applyEffects(response.requestedEffects ?? [], { ...baseSession, viewState }, resolveUiDirtyScope(response.uiScope));
  };
}

//#region 📥️ChunkedImport
/** 📏️ UTF-8 bytes ONE import chunk may carry — the INBOUND mirror of
 * `📤️SegmentedDownload`'s chunk contract, and the host half of `PUZZLE3D_IMPORT_CHUNK_BYTES`
 * (`✏️s/🔌️plugins/🧩️puzzle/…/🎮️commands/📥️import-fixture/🦀️.rs`), held equal to it by the engine
 * contract's own law.
 *
 * 🧊️ Derived from the guest's per-request contiguous ceiling, never a literal: an import's `payload`
 * crosses as ONE string, and every guest hop that carries it asks for one contiguous block — the channel's
 * `read_bounded_bytes` reserves the whole `AppCommand::Command` field exactly
 * (`📡️spr/🧵️channel/🦀️.rs`), and the retained tool job allocates its wire owner at the declared extent.
 * A 145 924-byte document sent as ONE command therefore asked the fixed guest heap for a 146 KB block,
 * 2.2× the ceiling. Half the ceiling leaves the other half for the JSON-escaped op envelope the chunk
 * rides in (measured 1.10× on the Nakagin export). */
export const IMPORT_CHUNK_BYTES = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2;

/** 📥️ One opened file's text as the chunk sequence the import lane carries: `chunk`/`chunkCount` name the
 * position, and a file that fits one chunk yields exactly one entry whose envelope a plugin may ignore.
 *
 * 🔤️ Sliced by UTF-8 EXTENT, not by code units: the guest measures `text.len()` in bytes, so a slice
 * counted in UTF-16 units would overrun the cap by up to 3× on non-ASCII labels (`·`, `ō`) — and the
 * Nakagin fixture has them. No slice ever splits a code point. */
export function importPayloadChunks(payload: string): readonly { readonly payload: string; readonly chunk: number; readonly chunkCount: number }[] {
  const pages: string[] = [];
  let page = "";
  let pageBytes = 0;
  for (const character of payload) {
    const code = character.codePointAt(0) ?? 0;
    const characterBytes = code < 0x80 ? 1 : code < 0x800 ? 2 : code < 0x10000 ? 3 : 4;
    if (pageBytes + characterBytes > IMPORT_CHUNK_BYTES) {
      pages.push(page);
      page = "";
      pageBytes = 0;
    }
    page += character;
    pageBytes += characterBytes;
  }
  if (page.length > 0 || pages.length === 0) pages.push(page);
  return pages.map((text, chunk) => ({ payload: text, chunk, chunkCount: pages.length }));
}
//#endregion 📥️ChunkedImport

/** 📤️ D3 fan-out: one {@link EffectDispatchOne} call per opened file, and — since wave B59 — one call per
 * {@link importPayloadChunks} CHUNK of each file, so no single import command asks the guest for a
 * contiguous block above its own per-request ceiling. A file that fits one chunk dispatches exactly one
 * call whose args are the pre-B59 `{payload, name}` plus the chunk envelope naming itself as `0` of `1`.
 *
 * 🧯 The chunks of one file are dispatched in ORDER and awaited one at a time: the guest's staging area
 * refuses a gap rather than resuming into bytes nobody can account for, so a concurrent fan-out would
 * cost the whole file. */
export async function dispatchOpenedFiles(
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
): Promise<void> {
  const total = opened.length;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index]!;
    for (const page of importPayloadChunks(file.contents)) {
      const envelope = { payload: page.payload, name: file.name, chunk: page.chunk, chunkCount: page.chunkCount };
      await dispatchOne(importAction, multiple ? { ...envelope, index, total } : envelope);
    }
  }
}

/** 🔁️ D2: schedules `action` onto `dispatchOne` after `delayMs` through the host's ONE
 * {@link ContinuationScheduler} — `delayMs <= 0` is an unthrottled macrotask, never a timer; a
 * positive `delayMs` is a real deadline. Tests pass {@link createContinuationScheduler} over
 * {@link createVirtualContinuationHost}'s ports instead of driving fake timers. Returns the cancel.
 *
 * 🛑️ Why this is not `setTimeout`: every `rearm()` in the flow/generation2d/generation3d command set
 * asks for `delay_ms: 0`, and the host answers each one by re-entering `applyHostEffects` from inside
 * the PREVIOUS dispatch's callback — a textbook nested zero-delay chain. Chrome clamps exactly that
 * shape to ~1 tick/s in a hidden, unfocused or headless renderer, which is where this ticket's
 * measured ~24 s per extension hop came from (two silent ~11-13 s gaps per hop against a 0.58 s
 * native baseline, `📓️audit-extension-hop-latency-2026-09-12.md`). Evaluation must keep converging
 * when nobody is looking at the tab; only paint may stop.
 *
 * 🩺 The scheduled dispatch has no awaiting caller, so its rejection is reported HERE and named — an
 * unnamed unhandled `SemioFaultError` is exactly how the deferred `flowEvalTick` re-arm's
 * `missing field \`locale\`` reached the page with nothing pointing at the dispatch that raised it
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function scheduleDispatchAction(
  action: string,
  args: Record<string, unknown> | undefined,
  delayMs: number,
  dispatchOne: EffectDispatchOne,
  scheduler: ContinuationScheduler = hostContinuations,
): ContinuationCancel {
  return scheduler.schedule(() => {
    void dispatchOne(action, args).catch((error: unknown) => console.error(`scheduled dispatch of "${action}" failed`, error));
  }, delayMs);
}

//#region RequestMediaFrames
//#region Bmff
/** 🧱️ One parsed ISO-BMFF box: `[type, payloadStart, payloadEnd)` — enough to recurse into containers
 * and slice leaf payloads without copying. */
type BmffBox = { readonly type: string; readonly start: number; readonly end: number };

/** 🧱️ Walks sibling boxes in `[start, end)` — handles 64-bit extended sizes (`size===1`) and to-end
 * boxes (`size===0`); malformed/truncated input just stops early rather than throwing, since MP4
 * probing here is best-effort — the Tier-2 `<video>` fallback covers anything this can't parse. */
function walkBmffBoxes(view: DataView, start: number, end: number): BmffBox[] {
  const boxes: BmffBox[] = [];
  let offset = start;
  while (offset + 8 <= end) {
    const size32 = view.getUint32(offset);
    const type = String.fromCharCode(view.getUint8(offset + 4), view.getUint8(offset + 5), view.getUint8(offset + 6), view.getUint8(offset + 7));
    let headerSize = 8;
    let boxSize = size32;
    if (size32 === 1) {
      if (offset + 16 > end) break;
      boxSize = Number(view.getBigUint64(offset + 8));
      headerSize = 16;
    } else if (size32 === 0) {
      boxSize = end - offset;
    }
    if (boxSize < headerSize || offset + boxSize > end) break;
    boxes.push({ type, start: offset + headerSize, end: offset + boxSize });
    offset += boxSize;
  }
  return boxes;
}

function findBmffBox(boxes: readonly BmffBox[], type: string): BmffBox | undefined {
  return boxes.find((box) => box.type === type);
}
//#endregion Bmff

//#region Tier1
type Mp4Sample = { readonly offset: number; readonly size: number; readonly timestampMs: number; readonly isSync: boolean };
type Mp4Track = {
  readonly width: number;
  readonly height: number;
  readonly codec: "avc1" | "hvc1";
  readonly description: Uint8Array;
  readonly samples: readonly Mp4Sample[];
};

/** 🎞️ Minimal MP4 sample-table extraction — `moov > trak[] > mdia > {mdhd, hdlr, minf > stbl}` for the
 * first video track (`hdlr`'s handler-type `"vide"`), enough to feed `VideoDecoder`: sample byte ranges
 * from `stsc` + `stco`/`co64` + `stsz`, decode timestamps from `stts`, sync flags from `stss` (absent
 * `stss` ⇒ every sample is sync per spec), and the AVC/HEVC decoder config from `stsd`'s `avcC`/`hvcC`.
 * Returns `null` for anything unrecognized (non-AVC/HEVC, missing boxes, malformed tables) so the
 * caller falls back to Tier 2 rather than guessing. */
function probeMp4VideoTrack(bytes: Uint8Array): Mp4Track | null {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const moov = findBmffBox(walkBmffBoxes(view, 0, bytes.byteLength), "moov");
  if (!moov) return null;
  for (const trak of walkBmffBoxes(view, moov.start, moov.end).filter((box) => box.type === "trak")) {
    const mdia = findBmffBox(walkBmffBoxes(view, trak.start, trak.end), "mdia");
    if (!mdia) continue;
    const mdiaBoxes = walkBmffBoxes(view, mdia.start, mdia.end);
    const hdlr = findBmffBox(mdiaBoxes, "hdlr");
    if (!hdlr || hdlr.end - hdlr.start < 12) continue;
    const handlerType = String.fromCharCode(view.getUint8(hdlr.start + 8), view.getUint8(hdlr.start + 9), view.getUint8(hdlr.start + 10), view.getUint8(hdlr.start + 11));
    if (handlerType !== "vide") continue;
    const mdhd = findBmffBox(mdiaBoxes, "mdhd");
    const minf = findBmffBox(mdiaBoxes, "minf");
    if (!mdhd || !minf) continue;
    const timescale = view.getUint8(mdhd.start) === 1 ? view.getUint32(mdhd.start + 20) : view.getUint32(mdhd.start + 12);
    if (timescale <= 0) continue;
    const stbl = findBmffBox(walkBmffBoxes(view, minf.start, minf.end), "stbl");
    if (!stbl) continue;
    const track = probeSampleTable(view, walkBmffBoxes(view, stbl.start, stbl.end), timescale);
    if (track) return track;
  }
  return null;
}

function parseStsd(view: DataView, stsd: BmffBox): { width: number; height: number; codec: "avc1" | "hvc1"; description: Uint8Array } | null {
  if (view.getUint32(stsd.start + 4) < 1) return null;
  const entryOffset = stsd.start + 8;
  const entrySize = view.getUint32(entryOffset);
  const format = String.fromCharCode(
    view.getUint8(entryOffset + 4),
    view.getUint8(entryOffset + 5),
    view.getUint8(entryOffset + 6),
    view.getUint8(entryOffset + 7),
  );
  if (format !== "avc1" && format !== "hvc1" && format !== "hev1") return null;
  const codec = format === "avc1" ? "avc1" : "hvc1";
  const visualEntryStart = entryOffset + 8;
  const width = view.getUint16(visualEntryStart + 24);
  const height = view.getUint16(visualEntryStart + 26);
  const inner = walkBmffBoxes(view, visualEntryStart + 78, entryOffset + entrySize);
  const config = findBmffBox(inner, codec === "avc1" ? "avcC" : "hvcC");
  if (!config) return null;
  return { width, height, codec, description: new Uint8Array(view.buffer.slice(config.start, config.end)) };
}

function parseStsz(view: DataView, box: BmffBox): number[] {
  const uniformSize = view.getUint32(box.start + 4);
  const sampleCount = view.getUint32(box.start + 8);
  if (uniformSize !== 0) return new Array(sampleCount).fill(uniformSize) as number[];
  const sizes: number[] = [];
  for (let i = 0; i < sampleCount; i += 1) sizes.push(view.getUint32(box.start + 12 + i * 4));
  return sizes;
}

function parseChunkOffsets(view: DataView, box: BmffBox, is64: boolean): number[] {
  const count = view.getUint32(box.start + 4);
  const offsets: number[] = [];
  for (let i = 0; i < count; i += 1) {
    offsets.push(is64 ? Number(view.getBigUint64(box.start + 8 + i * 8)) : view.getUint32(box.start + 8 + i * 4));
  }
  return offsets;
}

function parseChunkOfSample(view: DataView, box: BmffBox, sampleCount: number, chunkCount: number): number[] | null {
  const entryCount = view.getUint32(box.start + 4);
  const entries: { firstChunk: number; samplesPerChunk: number }[] = [];
  for (let i = 0; i < entryCount; i += 1) {
    entries.push({ firstChunk: view.getUint32(box.start + 8 + i * 12), samplesPerChunk: view.getUint32(box.start + 12 + i * 12) });
  }
  const chunkOfSample: number[] = [];
  for (let entryIndex = 0; entryIndex < entries.length; entryIndex += 1) {
    const entry = entries[entryIndex]!;
    const nextFirstChunk = entries[entryIndex + 1]?.firstChunk ?? chunkCount + 1;
    for (let chunk = entry.firstChunk; chunk < nextFirstChunk; chunk += 1) {
      for (let inChunk = 0; inChunk < entry.samplesPerChunk; inChunk += 1) chunkOfSample.push(chunk);
    }
  }
  return chunkOfSample.length >= sampleCount ? chunkOfSample : null;
}

function computeSampleOffsets(chunkOfSample: readonly number[], chunkOffsets: readonly number[], sizes: readonly number[]): number[] {
  const offsets: number[] = [];
  const cursorByChunk = new Map<number, number>();
  for (let i = 0; i < sizes.length; i += 1) {
    const chunk = chunkOfSample[i]!;
    const base = cursorByChunk.get(chunk) ?? chunkOffsets[chunk - 1] ?? 0;
    offsets.push(base);
    cursorByChunk.set(chunk, base + sizes[i]!);
  }
  return offsets;
}

function accumulateTimestampsMs(view: DataView, stts: BmffBox, sampleCount: number, timescale: number): number[] {
  const entryCount = view.getUint32(stts.start + 4);
  const timestamps: number[] = [];
  let ticks = 0;
  for (let entryIndex = 0; entryIndex < entryCount && timestamps.length < sampleCount; entryIndex += 1) {
    const count = view.getUint32(stts.start + 8 + entryIndex * 8);
    const delta = view.getUint32(stts.start + 12 + entryIndex * 8);
    for (let i = 0; i < count && timestamps.length < sampleCount; i += 1) {
      timestamps.push((ticks / timescale) * 1000);
      ticks += delta;
    }
  }
  return timestamps;
}

function parseSyncSamples(view: DataView, box: BmffBox): Set<number> {
  const count = view.getUint32(box.start + 4);
  const sync = new Set<number>();
  for (let i = 0; i < count; i += 1) sync.add(view.getUint32(box.start + 8 + i * 4));
  return sync;
}

function probeSampleTable(view: DataView, stblBoxes: readonly BmffBox[], timescale: number): Mp4Track | null {
  const stsd = findBmffBox(stblBoxes, "stsd");
  const stts = findBmffBox(stblBoxes, "stts");
  const stsc = findBmffBox(stblBoxes, "stsc");
  const stsz = findBmffBox(stblBoxes, "stsz");
  const stco = findBmffBox(stblBoxes, "stco") ?? findBmffBox(stblBoxes, "co64");
  if (!stsd || !stts || !stsc || !stsz || !stco) return null;
  const entry = parseStsd(view, stsd);
  if (!entry) return null;
  const sizes = parseStsz(view, stsz);
  const offsets = parseChunkOffsets(view, stco, stco.type === "co64");
  const chunkOfSample = parseChunkOfSample(view, stsc, sizes.length, offsets.length);
  if (!chunkOfSample) return null;
  const sampleOffsets = computeSampleOffsets(chunkOfSample, offsets, sizes);
  const timestampsMs = accumulateTimestampsMs(view, stts, sizes.length, timescale);
  const stss = findBmffBox(stblBoxes, "stss");
  const syncSamples = stss ? parseSyncSamples(view, stss) : null;
  const samples: Mp4Sample[] = sizes.map((size, index) => ({
    offset: sampleOffsets[index]!,
    size,
    timestampMs: timestampsMs[index] ?? 0,
    isSync: syncSamples ? syncSamples.has(index + 1) : true,
  }));
  return { width: entry.width, height: entry.height, codec: entry.codec, description: entry.description, samples };
}

/** 🌐️ Feature-detects the WebCodecs `VideoDecoder`/`EncodedVideoChunk` globals (Tier 1's prerequisite;
 * absent in most JS test environments and in browsers that only support WebM/VP9 without an AVC path). */
function webCodecsAvailable(): boolean {
  const scope = window as unknown as { VideoDecoder?: unknown; EncodedVideoChunk?: unknown };
  return typeof scope.VideoDecoder === "function" && typeof scope.EncodedVideoChunk === "function";
}

/** 🔢️ Derives a WebCodecs `avc1.PPCCLL` codec string from an `avcC` box's profile/compat/level bytes
 * (offsets 1/2/3 — version is byte 0). */
function avcCodecString(description: Uint8Array): string {
  const hex = (byte: number | undefined) => (byte ?? 0).toString(16).padStart(2, "0");
  return `avc1.${hex(description[1])}${hex(description[2])}${hex(description[3])}`;
}

type WebCodecsVideoFrame = { readonly codedWidth: number; readonly codedHeight: number; close: () => void };
type WebCodecsVideoDecoderCtor = new (init: { output: (frame: WebCodecsVideoFrame) => void; error: (error: unknown) => void }) => {
  configure: (config: { codec: string; codedWidth: number; codedHeight: number; description: Uint8Array }) => void;
  decode: (chunk: unknown) => void;
  flush: () => Promise<void>;
  close: () => void;
};
type WebCodecsEncodedVideoChunkCtor = new (init: { type: "key" | "delta"; timestamp: number; data: Uint8Array }) => unknown;

function jpegDataUrlFromFrame(frame: WebCodecsVideoFrame): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const canvas = document.createElement("canvas");
  canvas.width = frame.codedWidth;
  canvas.height = frame.codedHeight;
  canvas.getContext("2d")?.drawImage(frame as unknown as CanvasImageSource, 0, 0);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width: frame.codedWidth, height: frame.codedHeight };
}

/** 🎞️ Decodes exactly the samples needed for one target frame — from its nearest preceding sync sample
 * through the target — via a fresh `VideoDecoder`, capturing only the last output frame. Simplification:
 * each target frame re-decodes its GOP prefix from scratch instead of streaming continuously across
 * targets and demuxing outputs by timestamp; acceptable because sampled ingestion (`sampleStride`/
 * `maxFrames`) keeps GOP prefixes short between targets, and Tier 2's `<video>` element is always the
 * correctness fallback if Tier 1 fails or the codec isn't baseline-friendly. */
async function decodeOneMp4Frame(track: Mp4Track, bytes: Uint8Array, targetIndex: number): Promise<{ dataUrl: string; width: number; height: number } | null> {
  const scope = window as unknown as { VideoDecoder: WebCodecsVideoDecoderCtor; EncodedVideoChunk: WebCodecsEncodedVideoChunkCtor };
  let syncIndex = targetIndex;
  while (syncIndex > 0 && !track.samples[syncIndex]!.isSync) syncIndex -= 1;
  let captured: { dataUrl: string; width: number; height: number } | null = null;
  await new Promise<void>((resolve, reject) => {
    const decoder = new scope.VideoDecoder({
      output: (frame) => {
        captured = jpegDataUrlFromFrame(frame);
        frame.close();
      },
      error: reject,
    });
    decoder.configure({ codec: avcCodecString(track.description), codedWidth: track.width, codedHeight: track.height, description: track.description });
    for (let i = syncIndex; i <= targetIndex; i += 1) {
      const sample = track.samples[i]!;
      decoder.decode(
        new scope.EncodedVideoChunk({ type: sample.isSync ? "key" : "delta", timestamp: sample.timestampMs * 1000, data: bytes.subarray(sample.offset, sample.offset + sample.size) }),
      );
    }
    decoder.flush().then(() => {
      decoder.close();
      resolve();
    }, reject);
  });
  return captured;
}

/** 🎞️ Tier 1 orchestration: demuxes `bytes` as MP4/AVC, decodes one frame per sampled timestamp, and
 * dispatches `frameAction` per frame + `doneAction` once. Returns `false` (no dispatch performed at
 * all) when the demux can't find a usable AVC video track, so the caller falls through to Tier 2. */
async function runTier1VideoFrames(bytes: Uint8Array, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne): Promise<boolean> {
  const track = probeMp4VideoTrack(bytes);
  if (!track || track.samples.length === 0) return false;
  const durationMs = track.samples[track.samples.length - 1]!.timestampMs;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  let sampledCount = 0;
  for (let index = 0; index < timestamps.length; index += 1) {
    const targetMs = timestamps[index]!;
    let targetSampleIndex = 0;
    for (let i = 0; i < track.samples.length; i += 1) if (track.samples[i]!.timestampMs <= targetMs) targetSampleIndex = i;
    const frame = await decodeOneMp4Frame(track, bytes, targetSampleIndex);
    if (!frame) continue;
    sampledCount += 1;
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs: targetMs,
      index,
      total: timestamps.length,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
  }
  await dispatchOne(effect.doneAction, {
    name,
    durationMs,
    frameCount: track.samples.length,
    sampledCount,
    width: track.width,
    height: track.height,
    codec: track.codec,
    ...effect.args,
  });
  return true;
}
//#endregion Tier1

//#region Tier2
/** ⏱️ Tier-2 (`<video>` seek-and-capture) target timestamps, ms — one every `sampleStride /
 * (fpsHint || 30)` seconds starting at 0, capped at `maxFrames` (0 ⇒ unlimited, bounded only by
 * `durationMs`). Pure/deterministic so it's unit-testable without any DOM or media APIs. Computes each
 * timestamp as `k * stepMs` rather than an accumulating `ts += stepMs` loop — repeated float addition
 * drifts enough over dozens of steps to occasionally land just under an exact multiple of `durationMs`,
 * sneaking in one extra timestamp; multiplying from the loop index is exact per-step and deterministic. */
export function sampleMediaFrameTimestampsMs(durationMs: number, sampleStride: number, maxFrames: number, fpsHint: number): number[] {
  const stride = sampleStride > 0 ? sampleStride : 1;
  const fps = fpsHint > 0 ? fpsHint : 30;
  const stepMs = (stride / fps) * 1000;
  const timestamps: number[] = [];
  if (durationMs <= 0 || stepMs <= 0) return timestamps;
  for (let k = 0; ; k += 1) {
    if (maxFrames > 0 && timestamps.length >= maxFrames) break;
    const ts = k * stepMs;
    if (ts >= durationMs) break;
    timestamps.push(ts);
  }
  return timestamps;
}

function captureCanvasFrame(video: HTMLVideoElement, maxLongEdgePx: number): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const sourceWidth = video.videoWidth || 0;
  const sourceHeight = video.videoHeight || 0;
  const scale = maxLongEdgePx > 0 ? Math.min(1, maxLongEdgePx / Math.max(sourceWidth, sourceHeight, 1)) : 1;
  const width = Math.max(1, Math.round(sourceWidth * scale));
  const height = Math.max(1, Math.round(sourceHeight * scale));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  canvas.getContext("2d")?.drawImage(video, 0, 0, width, height);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width, height };
}

function waitForVideoEvent(video: HTMLVideoElement, type: string): Promise<void> {
  return new Promise((resolve) => {
    const handler = () => {
      video.removeEventListener(type, handler);
      resolve();
    };
    video.addEventListener(type, handler);
  });
}

/** 🎞️ Tier 2 orchestration: waits for `loadedmetadata` (if not already available), seeks `video`
 * through {@link sampleMediaFrameTimestampsMs}'s schedule, captures each landed frame to a scaled JPEG
 * data URL, dispatches `frameAction` per frame, then `doneAction` once. Used both as the WebM/no-
 * WebCodecs fallback and directly by tests (which inject a real `<video>` element with overridden
 * `duration`/`videoWidth`/`videoHeight`/`readyState` and manually dispatch `loadedmetadata`/`seeked`,
 * since headless test environments have no real media decoder). */
export async function runTier2VideoFrames(video: HTMLVideoElement, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne): Promise<void> {
  if (video.readyState < 1) await waitForVideoEvent(video, "loadedmetadata");
  const durationMs = Number.isFinite(video.duration) ? video.duration * 1000 : 0;
  const width = video.videoWidth || 0;
  const height = video.videoHeight || 0;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  const total = timestamps.length;
  for (let index = 0; index < total; index += 1) {
    const timestampMs = timestamps[index]!;
    video.currentTime = timestampMs / 1000;
    await waitForVideoEvent(video, "seeked");
    const frame = captureCanvasFrame(video, effect.maxLongEdgePx);
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs,
      index,
      total,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
  }
  await dispatchOne(effect.doneAction, { name, durationMs, frameCount: total, sampledCount: total, width, height, codec: "unknown", ...effect.args });
}
//#endregion Tier2

/** 🎞️ D5 `RequestMediaFrames` fields the two decode tiers need, decoupled from the raw `Effect`
 * union member shape so orchestration functions above take a plain, easily-constructed-in-tests object. */
export type RequestMediaFramesArgs = {
  readonly frameAction: string;
  readonly doneAction: string;
  readonly fallbackAction: string;
  readonly sampleStride: number;
  readonly maxFrames: number;
  readonly maxLongEdgePx: number;
  readonly fpsHint: number;
  readonly args?: Record<string, unknown>;
};

function bytesFromDataUrl(dataUrl: string): Uint8Array {
  const binary = atob(dataUrl.slice(dataUrl.indexOf(",") + 1));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

function bytesToDataUrl(bytes: Uint8Array, mime: string): string {
  let binary = "";
  for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]!);
  return `data:${mime};base64,${btoa(binary)}`;
}

/** 🎞️ D5 top-level: sources video bytes (`payload` data URL, or the native file picker when unset),
 * tries Tier 1 when WebCodecs is available and the demux finds a usable AVC track, otherwise Tier 2's
 * `<video>` seek-and-capture; on total failure (can't demux AND Tier 2 also throws, e.g. a corrupt
 * file) dispatches `fallbackAction` once with the raw original bytes as a data URL. */
export async function runRequestMediaFrames(
  effect: RequestMediaFramesArgs,
  accept: string,
  payload: string | undefined,
  dispatchOne: EffectDispatchOne,
  createVideoElement: () => HTMLVideoElement = () => document.createElement("video"),
): Promise<void> {
  let bytes: Uint8Array;
  let name = "video";
  if (payload) {
    bytes = bytesFromDataUrl(payload);
  } else {
    const opened = await requestFileOpen(accept || "video/*", "dataUrl", false);
    if (opened.length === 0) return;
    bytes = bytesFromDataUrl(opened[0]!.contents);
    name = opened[0]!.name;
  }
  try {
    if (webCodecsAvailable() && (await runTier1VideoFrames(bytes, effect, name, dispatchOne))) return;
    const url = URL.createObjectURL(new Blob([new Uint8Array(bytes)], { type: "video/mp4" }));
    const video = createVideoElement();
    video.muted = true;
    video.playsInline = true;
    video.src = url;
    try {
      await runTier2VideoFrames(video, effect, name, dispatchOne);
    } finally {
      URL.revokeObjectURL(url);
    }
  } catch (error) {
    console.error("[os-shell] requestMediaFrames: decode failed, falling back to raw bytes", error);
    await dispatchOne(effect.fallbackAction, { payload: bytesToDataUrl(bytes, "video/mp4"), name, ...effect.args });
  }
}
//#endregion RequestMediaFrames

function isStudioMode(catalog: PluginCatalog, pluginFilter?: string): boolean {
  return pluginFilter !== undefined && resolvePluginHostConfig(catalog, pluginFilter) !== undefined;
}

export interface SpaceShellPath {
  readonly spaceId: string;
  readonly instanceId?: string;
}

export type ShellRoute = { readonly kind: "landing" } | { readonly kind: "space"; readonly spaceId: string; readonly instanceId?: string } | { readonly kind: "notFound"; readonly path: string };

/** @emoji 🧭️ Classifies shell history paths into landing, studio space, or unknown routes. */
export function parseShellRoute(path: string): ShellRoute {
  const normalized = (path.split("?")[0] ?? "/").trim() || "/";
  if (normalized === "/") return { kind: "landing" };
  const match = /^\/spaces\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(normalized);
  if (match) return { kind: "space", spaceId: match[1]!, instanceId: match[2] };
  return { kind: "notFound", path: normalized };
}

/** @deprecated Use {@link parseShellRoute} instead. */
export function parseSpaceShellPath(path: string): SpaceShellPath | null {
  const route = parseShellRoute(path);
  if (route.kind !== "space") return null;
  return { spaceId: route.spaceId, instanceId: route.instanceId };
}

/**
 * 🗺️ Joins a breadcrumb for display.
 *
 * ⚠️ Accepts `undefined`: `AppDefinition.breadcrumb` is DECLARED OPTIONAL in the manifest, so every
 * unguarded `breadcrumb.join(…)` was a latent crash — and not a cosmetic one. This runs inside
 * `FrameworkOsShellInner`'s render, so a single app whose manifest omits the field took down the
 * WHOLE shell, and in a multi-pane host (the demonstrator) every pane died with it. An app with no
 * breadcrumb should render a nameless title, never destroy its host.
 */
export function appBreadcrumb(breadcrumb: readonly string[] | undefined): string {
  return (breadcrumb ?? []).join(APP_BREADCRUMB_SEPARATOR);
}

/** 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to `app.breadcrumb`, and an app that declares none at all to the empty breadcrumb (the field is optional — see {@link appBreadcrumb}). */
export function resolveAppBreadcrumb(app: Pick<AppDefinition, "breadcrumb" | "terminologyBreadcrumbs">, terminology: string): readonly string[] {
  return app.terminologyBreadcrumbs?.[terminology] ?? app.breadcrumb ?? [];
}

/** 🗺️ Resolves the breadcrumb for a non-active app (studio spawn palette/spawned entries) by looking up its `AppDefinition` across loaded plugins; falls back to the raw breadcrumb when the app can't be found. */
export function resolveArtifactByAppId(loadedPlugins: readonly LoadedProgramState[], appId: string, breadcrumb: readonly string[], terminology: string): readonly string[] {
  for (const program of loadedPlugins) {
    const app = program.manifest.apps.find((candidate) => candidate.id === appId);
    if (app) return resolveAppBreadcrumb(app, terminology);
  }
  return breadcrumb;
}

export function appWindowLabel(app: Pick<AppDefinition, "label" | "breadcrumb" | "terminologyBreadcrumbs">, terminology: string, windowLabel: string, locale: string = SHELL_LOCALES[0]): string {
  const trimmed = windowLabel.trim();
  if (trimmed) return trimmed;
  const override = app.terminologyBreadcrumbs?.[terminology];
  return override?.[override.length - 1]?.trim() || resolveManifestLabel(app.label, terminology, locale).trim();
}

// 📌️ The panel carriage moved to its own cycle-free module (`📌️panel/🟦️.ts`) so a law can drive
// it — importing this file from a test hits the `ShellHelpers → Shell → ShellHost` cycle. Re-exported
// here so every existing call site keeps one import.
export { buildSpacePanelState, isSpacePanelState, panelJsonFromState, parsePanelState, studioPanelFocusingSpawned, viewStateWithSpacePanel } from "./📌️panel/🟦️.ts";

/** @emoji 🧭️ Default anchor a plugin-declared panel-tab `group` docks into — groups only ever map to the four corners; the four edge-middle anchors start empty and are user-populated via drag-and-drop or a dock skeleton override. */
export function panelAnchorForGroup(group: string): Anchor {
  if (group === "workbench" || group === "document") return "top-left";
  if (group === "details") return "top-right";
  if (group === "display") return "bottom-left";
  if (group === "settings") return "bottom-right";
  return "top-right";
}

/**
 * @emoji 🕰️ Panel tab ids the shell mounts as its own chrome, so an app-declared tab carrying one of them
 * must never be mounted a second time out of `AppDefinition.panelTabs`. `framework.panel.history` is
 * injected into EVERY app (`🔌️plugin/🦀️.rs` `AppBuilder::build_definition`) so the guest renders the
 * history body, while the shell also builds that tab host-side; mounting both puts two identically-named
 * tab buttons and two nodes with one DOM id in the same anchor, and the guest-rendered twin namespaces
 * every child element as `panel:<key>/<id>` ({@link uiNodeToTreePanelConfig}), so `framework.history.entry.<seq>`
 * stops resolving by its declared id.
 */
export const SHELL_OWNED_PANEL_TAB_IDS: readonly string[] = [FRAMEWORK_PANEL_TAB_HISTORY_ID];

/** @emoji 🕰️ True when {@link SHELL_OWNED_PANEL_TAB_IDS} already covers this panel tab id — the one gate the dock's app-declared anchors filter on. */
export function shellRendersPanelTabItself(panelTabId: string): boolean {
  return SHELL_OWNED_PANEL_TAB_IDS.includes(panelTabId);
}

/** @emoji 🪟️ One leaf in a framework layout tree, with optional instance/template binding for multi-pane world views. */
type FrameworkLayoutWindowSeed = {
  readonly windowId: string;
  readonly windowKindId: string;
  readonly title?: string;
  readonly templateId?: string;
  readonly size: number;
  readonly corner?: WindowStackCorner;
};

/** @emoji 🪟️ Walks a framework layout and collects every window leaf, preferring `instanceId` as the live pane id. */
function collectFrameworkLayoutWindowSeeds(node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode, parentSize = 100): FrameworkLayoutWindowSeed[] {
  if (node.kind === "window") {
    return [
      {
        windowId: node.instanceId ?? node.windowKindId,
        windowKindId: node.windowKindId,
        title: node.title,
        templateId: node.templateId,
        size: parentSize,
        corner: node.corner,
      },
    ];
  }
  if (node.kind === "stack") {
    const size = node.size ?? parentSize;
    return node.children.map((child) => ({
      windowId: child.instanceId ?? child.windowKindId,
      windowKindId: child.windowKindId,
      title: child.title,
      templateId: child.templateId,
      size,
      corner: child.corner,
    }));
  }
  const childSizes = node.children.map((child) => ("size" in child ? child.size : undefined));
  const explicitTotal = childSizes.reduce<number>((sum, size) => sum + (size ?? 0), 0);
  const unsetCount = childSizes.filter((size) => size === undefined).length;
  const defaultEach = unsetCount > 0 ? Math.max(0, 100 - explicitTotal) / unsetCount : 0;
  return node.children.flatMap((child, index) => {
    const fraction = childSizes[index] ?? defaultEach;
    return collectFrameworkLayoutWindowSeeds(child, parentSize * (fraction / 100));
  });
}

/** 🗣️ For a single, non-instanced window (`instanceId` unset — the common case, one window per kind),
 * the `windowKind`'s own label is the single source of truth: a manifest-baked `WindowLayoutWindowNode.title`
 * (from a plugin's `create_default_layout(..., titles)` call) is a plain, locale-invariant string that
 * predates locale/terminology resolution entirely, so it must never win over a real `windowKinds` lookup —
 * it only survives as a last-resort fallback for a window kind id that isn't declared in the manifest
 * (mirrors {@link retitleWindowLayoutNode}'s already-correct precedence). For a multi-instance window
 * (`instanceId` set — several views sharing one `windowKind`, e.g. "Top"/"Perspective" both backed by a
 * single 3D-viewport kind), the shared kind label can't distinguish instances, so the baked per-instance
 * title is the real title and must win instead. */
function resolveFrameworkWindowTitle(
  windowKindId: string,
  instanceId: string | undefined,
  bakedTitle: string | undefined,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  terminology: string,
  locale: string,
): string {
  if (instanceId) return bakedTitle ?? windowKindId;
  const kind = windowKinds.find((entry) => entry.id === windowKindId);
  return kind ? resolveManifestLabel(kind.label, terminology, locale) : (bakedTitle ?? windowKindId);
}

function convertFrameworkLayoutNodeToModeLayout(
  node: WindowLayoutAxisNode | WindowLayoutStackNode,
  appLabelsOverlay: PluginAppLabelsOverlay,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  terminology: string,
  locale: string,
): Exclude<WindowLayoutNode, { kind: "window" }> {
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => {
        const id = child.instanceId ?? child.windowKindId;
        const title = resolveFrameworkWindowTitle(child.windowKindId, child.instanceId, child.title, windowKinds, terminology, locale);
        return {
          kind: "window" as const,
          id,
          title: wireLabel(resolveAppLabel(appLabelsOverlay, "windowKind", id, title)),
          corner: child.corner,
        };
      }),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay, windowKinds, terminology, locale)),
  };
}

/** @emoji 🗣️ Re-resolves every window's title from the app manifest's windowKinds via resolveManifestLabel in place, preserving the tree's structure/sizes/arrangement — used to react to a locale/terminology switch without discarding the user's live layout. */
export function retitleWindowLayoutNode(
  node: WindowLayoutNode,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  extraInstances: readonly ExtraWindowInstance[],
  terminology: string,
  locale: string,
): WindowLayoutNode {
  if (node.kind === "window") {
    const extra = extraInstances.find((entry) => entry.id === node.id);
    const windowKindId = extra ? extra.windowKindId : node.id;
    const kind = windowKinds.find((entry) => entry.id === windowKindId);
    const title = kind ? wireLabel(resolveManifestLabel(kind.label, terminology, locale)) : (node.title ?? uiDataLabel(node.id));
    return { ...node, title };
  }
  return {
    ...node,
    children: node.children.map((child) => retitleWindowLayoutNode(child, windowKinds, extraInstances, terminology, locale)),
  } as WindowLayoutNode;
}

/** @emoji 🪟️ Resolves a framework layout into the live mode tree, extra instances, and pending projection templates without inferring window focus (no side effects). */
export function resolveFrameworkLayoutSeed(
  layout: WindowLayout | undefined,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string,
  locale: string,
): {
  readonly modeLayout: WindowLayoutNode;
  readonly extraInstances: readonly ExtraWindowInstance[];
  readonly pendingProjections: readonly { readonly windowId: string; readonly templateId: string }[];
} {
  const windowIds = windowKinds.map((kind) => kind.id);
  if (!layout?.root) {
    return {
      modeLayout: createEvenWindowLayout(windowIds.length ? windowIds : ["main"]),
      extraInstances: [],
      pendingProjections: [],
    };
  }
  const seeds = collectFrameworkLayoutWindowSeeds(layout.root);
  const kindById = new Map(windowKinds.map((kind) => [kind.id, kind] as const));
  const extraInstances: ExtraWindowInstance[] = [];
  const pendingProjections: { readonly windowId: string; readonly templateId: string }[] = [];
  for (const seed of seeds) {
    const kind = kindById.get(seed.windowKindId);
    if (!kind) continue;
    if (seed.windowId !== seed.windowKindId) {
      extraInstances.push({
        id: seed.windowId,
        windowKindId: seed.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", seed.windowId, seed.title ?? resolveManifestLabel(kind.label, terminology, locale)),
      });
    }
    if (seed.templateId) pendingProjections.push({ windowId: seed.windowId, templateId: seed.templateId });
  }
  return {
    modeLayout: convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay, windowKinds, terminology, locale),
    extraInstances,
    pendingProjections,
  };
}

/** @emoji 🪟️ Applies a resolved framework layout seed: registers one-shot world projections, then returns the live layout payload. */
export function applyFrameworkLayoutSeed(
  layout: WindowLayout | undefined,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string,
  locale: string,
): {
  readonly modeLayout: WindowLayoutNode;
  readonly extraInstances: readonly ExtraWindowInstance[];
} {
  const seed = resolveFrameworkLayoutSeed(layout, windowKinds, appLabelsOverlay, terminology, locale);
  for (const pending of seed.pendingProjections) {
    const projectionSpec = decodeWorldProjectionTemplateId(pending.templateId);
    if (projectionSpec) registerPendingWorldProjection(pending.windowId, projectionSpec);
  }
  return { modeLayout: seed.modeLayout, extraInstances: seed.extraInstances };
}

function modeLayoutNodeToFramework(node: WindowLayoutNode, kindByInstanceId: ReadonlyMap<string, string>): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
  if (node.kind === "window") {
    const windowKindId = kindByInstanceId.get(node.id) ?? node.id;
    const instanceId = kindByInstanceId.has(node.id) ? node.id : undefined;
    return {
      kind: "window",
      windowKindId,
      ...(node.title ? { title: node.title } : {}),
      ...(instanceId ? { instanceId } : {}),
    };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => {
        const windowKindId = kindByInstanceId.get(child.id) ?? child.id;
        const instanceId = kindByInstanceId.has(child.id) ? child.id : undefined;
        return {
          kind: "window" as const,
          windowKindId,
          ...(child.title ? { title: child.title } : {}),
          ...(instanceId ? { instanceId } : {}),
        };
      }),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => modeLayoutNodeToFramework(child, kindByInstanceId) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

export function captureCurrentFrameworkLayout(shellLayout: WindowLayoutNode | null, extraWindowInstances: readonly ExtraWindowInstance[], fallback?: WindowLayout): WindowLayout | undefined {
  if (!shellLayout) return fallback;
  const kindByInstanceId = new Map(extraWindowInstances.map((entry) => [entry.id, entry.windowKindId] as const));
  const root = modeLayoutNodeToFramework(shellLayout, kindByInstanceId);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}

//#region WindowLayoutChangeClassify
/** 🪟️ Trailing settle delay for `Mode.onLayoutChange` (fires continuously during a live drag/resize) before
 * noting one `shell.windowResize`/`shell.windowMove` command for the whole gesture — matches Board2dHost's
 * own camera-sync settle debounce (`beginCameraInteraction`), the only precedent for this kind of
 * drag-settle pattern already in this file. */
export const LAYOUT_CHANGE_SETTLE_MS = 350;

/** 🪟️ Recursive skeleton of a {@link WindowLayoutNode} — kind/id/nesting only, stripping `size` (resize) and
 * a stack's `activeId` (mere focus echo) — so two trees compare equal here iff neither differs. */
type WindowLayoutSkeletonNode = { readonly kind: string; readonly id?: string; readonly corner?: string; readonly children?: readonly WindowLayoutSkeletonNode[] };
function windowLayoutSkeleton(node: WindowLayoutNode): WindowLayoutSkeletonNode {
  if (node.kind === "window") return { kind: node.kind, id: node.id, corner: node.corner };
  return { kind: node.kind, children: node.children.map((child) => windowLayoutSkeleton(child as WindowLayoutNode)) };
}

/** 🪟️ Like {@link windowLayoutSkeleton} but keeps each node's `size` (still ignores a stack's `activeId`) —
 * comparing two of these (after their plain skeletons already matched) is how {@link classifyWindowLayoutChange}
 * tells a pure resize apart from no change at all. */
type WindowLayoutSizedSkeletonNode = { readonly kind: string; readonly id?: string; readonly size?: number; readonly children?: readonly WindowLayoutSizedSkeletonNode[] };
function windowLayoutSizedSkeleton(node: WindowLayoutNode): WindowLayoutSizedSkeletonNode {
  if (node.kind === "window") return { kind: node.kind, id: node.id, size: node.size };
  return { kind: node.kind, size: node.size, children: node.children.map((child) => windowLayoutSizedSkeleton(child as WindowLayoutNode)) };
}

/** 🪟️ Classifies a `Mode.onLayoutChange` delta by comparing the previous and next layout tree — `"rearrange"`
 * when window ids/nesting structure differ (drag-to-new-position, split, close), `"resize"` when only pane
 * sizes differ, `null` when neither differs (a pure active-window-flag echo, handled by the dedicated
 * active-window seam instead — never worth its own shell command). */
export function classifyWindowLayoutChange(previous: WindowLayoutNode | null, next: WindowLayoutNode | null): "resize" | "rearrange" | null {
  if (previous === next) return null;
  if (!previous || !next) return "rearrange";
  if (JSON.stringify(windowLayoutSkeleton(previous)) !== JSON.stringify(windowLayoutSkeleton(next))) return "rearrange";
  if (JSON.stringify(windowLayoutSizedSkeleton(previous)) !== JSON.stringify(windowLayoutSizedSkeleton(next))) return "resize";
  return null;
}
//#endregion WindowLayoutChangeClassify

function windowEngagementControlToSpec(control: WindowEngagementControl | undefined, onAction: (action: ActionDescriptor) => void): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label === undefined ? undefined : uiDataLabel(control.label),
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id: string) => onAction({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } }) : undefined,
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label === undefined ? undefined : uiDataLabel(control.label),
      value: control.value,
      placeholder: control.placeholder === undefined ? undefined : uiDataLabel(control.placeholder),
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? (value: string) => onAction({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } }) : undefined,
    };
  }
  const dispatchNumeric = (action: ActionDescriptor | undefined, value: number) => {
    if (!action) return;
    onAction({ ...action, args: { ...(action.args as object | undefined), value } });
  };
  const numeric = {
    id: control.id,
    label: control.label === undefined ? undefined : uiDataLabel(control.label),
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value: number) => dispatchNumeric(control.onChange, value) : undefined,
    onCommit: control.onCommit ? (value: number) => dispatchNumeric(control.onCommit, value) : undefined,
  };
  if (control.kind === "slider") {
    if (control.min === undefined || control.max === undefined) throw new Error("engagement slider requires explicit minimum and maximum");
    return { ...numeric, kind: "slider", min: control.min, max: control.max };
  }
  return { ...numeric, kind: "stepper" };
}

/** 🫀️ Both numbers come from the ONE schema-owned liveness policy (`https://semio.tech/schema/framework/actor/shard-client/schema.json#/$defs/ShardClient`,
 * re-exported through `🔌️PluginRuntime`) — never a literal here. `pluginLoadIdleTimeoutMs` is an IDLE
 * budget, not a total one: the deadline is pushed forward every time this plugin's own load reports
 * progress, so a multi-MB wasm component fetching, compiling and instantiating for two minutes on a
 * loaded machine survives while a plugin whose module 404s or whose worker died still fails within one
 * idle window. `pluginLoadCeilingMs` bounds the whole attempt regardless of progress, so a plugin that
 * reports progress forever can never wedge the boot. */
const PLUGIN_LOAD_IDLE_TIMEOUT_MS = SHARD_LIVENESS_POLICY.pluginLoadIdleTimeoutMs;
const PLUGIN_LOAD_CEILING_MS = SHARD_LIVENESS_POLICY.pluginLoadCeilingMs;

/** ⏱️ Pure deadline rule, split out so it can be tested without a clock or a `Worker`: given when the
 * attempt started, when this plugin last reported progress and what time it is now, say how much
 * longer to wait (`0` means give up now). */
export function pluginLoadRemainingMs(startedAtMs: number, lastProgressAtMs: number | undefined, nowMs: number, idleTimeoutMs: number = PLUGIN_LOAD_IDLE_TIMEOUT_MS, ceilingMs: number = PLUGIN_LOAD_CEILING_MS): number {
  const idleRemaining = Math.max(lastProgressAtMs ?? startedAtMs, startedAtMs) + idleTimeoutMs - nowMs;
  const ceilingRemaining = startedAtMs + ceilingMs - nowMs;
  return Math.max(0, Math.min(idleRemaining, ceilingRemaining));
}

/** @emoji 🔌️ Result of {@link installPlugin} — the boot effect must not infer success from
 * `loadedPluginsRef`, which only updates after the next React commit. */
export type PluginInstallOutcome = "loaded" | "already-loaded" | "in-flight" | "missing-registry" | "failed";

/** 🔐️ Keeps app-session ownership deterministic while dependency plugins stream concurrently. */
export function pluginShouldEstablishSession(pluginId: string, primaryPluginId: string | undefined, hasSession: boolean): boolean {
  return !hasSession && pluginId === primaryPluginId;
}

/** 🔁️ What one `PluginSource` availability event is worth: a first `install`, a `hot-swap` of the
 * artifact already loaded, or a `drop` because the event names nothing this shell does not already run. */
export type PluginAvailabilityRouteV1 = "install" | "hot-swap" | "drop";

/** 🔁️ The routing rule the availability pump obeys — the ONE place that decides whether an event may
 * destroy a live instance.
 *
 * A `PluginSource` streams AVAILABILITY, never commands: `subscribe` opens a fresh stream and the dev
 * source's SSE endpoint answers every connect with a full `snapshot` of what is already built, so the
 * same `rebuiltAt` arrives again on every reconnect. Routing a replay to `hot-swap` destroys the
 * session-owning plugin's live instance and its document (`actor-activation.revoked`, then `no channel
 * for instance N`) — measured in ticket 26/09/02 wave B38 §1.7 and diagnosed in wave B40, where a
 * language switch was re-running the subscription effect and thereby replaying the whole snapshot.
 *
 * `loadedRebuiltAt === undefined` is the first load's unbusted artifact (see `PluginSource.moduleUrl`):
 * it names no build, so any event that DOES name one is newer. An event carrying no `rebuiltAt` names no
 * newer artifact than whatever is loaded, so it can only ever be a first `install`. */
export function pluginAvailabilityRouteV1(alreadyLoaded: boolean, loadedRebuiltAt: number | undefined, eventRebuiltAt: number | undefined): PluginAvailabilityRouteV1 {
  if (!alreadyLoaded) return "install";
  if (eventRebuiltAt === undefined) return "drop";
  return loadedRebuiltAt === undefined || eventRebuiltAt > loadedRebuiltAt ? "hot-swap" : "drop";
}

/** 🧭️ Studio configures its catalogue; a focused shell configures only its active aggregate. */
export function pluginShouldReceiveContributions(pluginId: string, sessionPluginId: string, hostMode: boolean): boolean {
  return hostMode || pluginId === sessionPluginId;
}


export async function loadPluginModuleResilient(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle | null> {
  const startedAtMs = Date.now();
  let timer = 0;
  try {
    return await Promise.race([
      loadPluginModule(pluginId, moduleUrl),
      new Promise<never>((_, reject) => {
        // ⏱️ Re-arms itself for whatever the idle rule still allows instead of firing once at a fixed
        // wall-clock offset, so an attempt that keeps reporting progress keeps its deadline moving.
        const arm = () => {
          const remaining = pluginLoadRemainingMs(startedAtMs, pluginLoadProgressAt(pluginId), Date.now());
          if (remaining > 0) {
            timer = window.setTimeout(arm, remaining);
            return;
          }
          reject(new Error(`timeout loading ${pluginId} after ${Date.now() - startedAtMs} ms with no progress for ${PLUGIN_LOAD_IDLE_TIMEOUT_MS} ms`));
        };
        arm();
      }),
    ]);
  } catch (error) {
    console.error("program load failed", pluginId, error);
    return null;
  } finally {
    if (timer) window.clearTimeout(timer);
  }
}

function isViewportSurface(surfaceKind: string | undefined): boolean {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}

function defaultViewportEngagement(): WindowEngagement {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }],
  };
}

export function resolveWindowEngagement(kind: Pick<AppDefinition["windowKinds"][number], "options">, windowId: string, byWindowId: Readonly<Record<string, WindowEngagement>>): WindowEngagement | undefined {
  const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
  const declaredEngagement = kind.options.engagement.kind === "some" ? kind.options.engagement.value : undefined;
  return byWindowId[windowId] ?? declaredEngagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

export function windowEngagementToSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label === undefined ? undefined : uiDataLabel(option.label),
    icon: option.iconId ? <Icon icon={option.iconId} size="small" /> : { kind: "text" as const, text: option.label ?? option.id },
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action!) : undefined,
  }));
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, control, controls, status };
}

/** @emoji 🔎️ Builds the top-middle window {@link SearchSpec} from the same Rust engagement payload: typed action input and autocomplete possibles. */
export function windowEngagementToSearchSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): SearchSpec | undefined {
  if (!engagement) return undefined;
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder === undefined ? undefined : uiDataLabel(engagement.input.placeholder),
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => onAction({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => onAction({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input!.onRepeatLast!) : undefined,
        onAbort: engagement.input.onAbort ? () => onAction(engagement.input!.onAbort!) : undefined,
      }
    : undefined;
  const possibles = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action!) : undefined,
  }));
  const hasContent = Boolean(input) || (possibles?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, input, possibles };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
  // 🌱️ `group === "workbench"` already covers every host-app catalogue tab (each such app declares its
  // catalogue tab under `PanelGroup::Workbench` — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
  // so no separate app-specific tab-id literal is needed here.
  if (group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  if (tabId === FRAMEWORK_PANEL_TAB_HISTORY_ID) return shellTabIcon("undo");
  return shellTabIcon(tabId);
}

/** @emoji 🌳️ Category-row icon: the first child's icon, or `fallback` when the category has no tabs yet. */
export function categoryTabIcon(tabs: readonly PanelTabNode[], fallback: IconName): React.FC<{ size?: number }> {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }: { size?: number }) {
    return FirstIcon ? <FirstIcon size={size} /> : <Icon icon={fallback} size="small" />;
  };
}

/** @emoji 🌳️ Depth-first leaves of a recursive panel-tab tree — the nodes that actually carry a `bodyKey`
 * to render. `PanelTabDefinition` (required `children: T[]`, no leaf variant) satisfies this
 * constraint directly; `PanelTabNode` (`PanelTabLeaf | PanelTabBranch`, `PanelTabLeaf` carrying no
 * `children` key at all) is a TS "weak type" mismatch against a constraint of only-optional
 * properties — callers with that shape use `flattenPanelTabNodeLeaves` (`ShellHost`'s own, built on
 * `panelTabChildren`'s union-aware accessor) instead of fighting the weak-type check here. */
export function flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>(tabs: readonly T[]): T[] {
  return tabs.flatMap((tab) => (tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]));
}

/** @emoji 🌳️ Converts one plugin-declared {@link AppPanelTabDefinition} (recursively) into a {@link PanelTabNode}. */
export function panelTabDefinitionToNode(
  tab: AppPanelTabDefinition,
  group: string,
  panelUiByKey: Readonly<Record<string, BuiltNode>>,
  onAction: (action: ActionDescriptor) => void,
  order: number,
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
): PanelTabNode {
  const tabId = panelTabKindId(tab.kind);
  const label = resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label, terminology, locale));
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tabId,
      icon: panelTabIcon(tabId, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay, terminology, locale)),
    };
  }
  return singleTreeLeaf({
    id: tabId,
    icon: panelTabIcon(tabId, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tabId] ?? pendingPanelUiNode(), onAction)),
  });
}

export function resolveCanvasBodyKey(app: AppDefinition): string {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const workflow = app.windowKinds.find((kind) => kind.bodyKey.includes("workflow"));
    return workflow?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}

//#region 🧰️UtilityRegistry
/**
 * 🧰️ Resolves the `UtilityDefinition`s in scope for one window kind against the app's utility registry:
 * the window kind's own `utilities` refs when non-empty, otherwise every utility the app declares (the
 * scoping fallback, mirroring `resolveWindowActions`' intent for utilities). Unresolvable refs are dropped.
 */
export function resolveUtilities(app: Pick<AppDefinition, "utilities">, windowKind: Pick<AppWindowKindDefinition, "utilities">): UtilityDefinition[] {
  const registry = app.utilities ?? [];
  const refs = windowKind.utilities ?? [];
  if (refs.length === 0) return [...registry];
  const resolved: UtilityDefinition[] = [];
  for (const ref of refs) {
    const utility = registry.find((entry) => entry.id === ref);
    if (utility) resolved.push(utility);
  }
  return resolved;
}

/** 🧰️ Resolves a `UtilityDefinition.group` id's display label: the app's own `groupLabels` overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids ({@link ribbonParentLabel}), else the raw id. */
function resolveUtilityGroupLabel(group: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  return resolveAppLabel(appLabelsOverlay, "group", group, ribbonParentLabel(group) ?? group);
}

/** 🧰️ One `UtilityDefinition` → the lean `DerivedUtilitySpec` consumed by {@link deriveUtilityNodes}, resolving the label (and, for grouped utilities, the group label) through the app's locale/terminology overlay. `UtilityDefinition.label` is a manifest `LocalizedLabel` field. */
function utilityDefinitionToSpec(utility: UtilityDefinition, appLabelsOverlay: PluginAppLabelsOverlay, terminology: string, locale: string): DerivedUtilitySpec {
  return {
    id: utility.id,
    label: resolveAppLabel(appLabelsOverlay, "utility", utility.id, resolveManifestLabel(utility.label, terminology, locale)),
    iconId: utility.iconId,
    group: utility.group ?? undefined,
    groupLabel: utility.group ? resolveUtilityGroupLabel(utility.group, appLabelsOverlay) : undefined,
    category: utility.category ?? "utilities",
  };
}

/** 🧰️ Stamps the owning `windowId` onto every `setActiveUtility` descriptor in a derived utility tree so the shell's `onAction` interceptor targets the right window regardless of which window is globally active. */
function tagSetActiveUtilityWindow(nodes: readonly UtilityNode[], windowId: string): UtilityNode[] {
  return nodes.map((node) => {
    if (node.kind === "collection") return { ...node, children: tagSetActiveUtilityWindow(node.children, windowId) };
    if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return { ...node, onChange: { ...node.onChange, args: { ...(node.onChange.args as object | undefined), windowId } } };
    }
    return node;
  });
}

/**
 * 🧰️ Builds the window utility bar `UtilityNode[]` for one window purely from the static utility registry plus
 * the host-owned active utility id — the replacement for the deleted program `list-tools` sourcing. Each
 * `setActiveUtility` descriptor is tagged with `windowId` so activation is scoped to this exact window.
 */
export function resolveUtilityNodes(
  app: Pick<AppDefinition, "utilities" | "controllerId">,
  windowKind: Pick<AppWindowKindDefinition, "utilities">,
  activeUtilityId: string | null | undefined,
  windowId: string,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
): UtilityNode[] {
  const utilities = resolveUtilities(app, windowKind);
  if (utilities.length === 0) return [];
  return tagSetActiveUtilityWindow(
    deriveUtilityNodes(
      app.controllerId,
      utilities.map((utility) => utilityDefinitionToSpec(utility, appLabelsOverlay, terminology, locale)),
      activeUtilityId ?? undefined,
    ),
    windowId,
  );
}
//#endregion 🧰️UtilityRegistry

/** @emoji 💬️ Builds spawned-window engagement, search, measures, and utility-options chrome for one window instance. */
export function spawnedWindowChromeForKind(
  kind: Pick<AppDefinition["windowKinds"][number], "options">,
  windowId: string,
  engagementsByWindowId: Readonly<Record<string, WindowEngagement>>,
  measuresByWindowId: Readonly<Record<string, readonly WindowMeasure[]>>,
  activeUtilityId: string | undefined,
  onAction: (action: ActionDescriptor) => void,
): { readonly engagement?: EngagementSpec; readonly search?: SearchSpec; readonly measures: ReactNode; readonly utilityOptions: ReactNode } {
  const { measures, utilityOptions } = windowMeasuresChrome(measuresByWindowId[windowId] ?? kind.options.measures, activeUtilityId, windowId, onAction);
  const resolvedEngagement = resolveWindowEngagement(kind, windowId, engagementsByWindowId);
  return {
    engagement: windowEngagementToSpec(resolvedEngagement, onAction),
    search: windowEngagementToSearchSpec(resolvedEngagement, onAction),
    measures,
    utilityOptions,
  };
}

/** 🏷️ The argument FIELD a trigger's scalar payload travels under. An action reads named arguments —
 * `value` for an absolute edit (`engagement_input`, `engagement_control_select`,
 * `puzzle3d_absolute_or_delta`'s absolute half) and `delta` for a relative bump (its relative half) —
 * so the name belongs to the TRIGGER, not to the control that fired it. */
function uiInputField(trigger: UiIntent["trigger"]): string {
  return trigger === "delta" ? "delta" : "value";
}

/** 🎬️ The arguments one intent dispatches: the node's AUTHORED args with the gesture's own payload
 * merged over them.
 *
 * 🧯️ A scalar payload used to REPLACE the authored args wholesale — a `NumberStepper`'s `onChange`
 * reports the number `10.5`, so the action received the bare scalar `10.5` and every guest, which reads
 * `args.get("value")`, saw nothing at all while the authored `{windowId}` was thrown away with it. The
 * whole Settings panel was mute for exactly that reason: browser-measured on `:6013`, `setGridSpacing`
 * reaching the guest and settling with `historyUpserts: 0, effects: 0` while the stepper rendered its
 * optimistic 10.5 and both windows' rails stayed at 10 for 30 s (ticket 26/09/02/PUZZLE-3D-END-TO-END
 * wave B47 §2). A scalar is NAMED by its trigger and merged; a map payload merges as it always did. */
function uiIntentPayload(intent: UiIntent): unknown {
  if (intent.input === null) return intent.args ?? undefined;
  const scalar = typeof intent.input === "string" || typeof intent.input === "number" || typeof intent.input === "boolean" || typeof intent.input === "bigint";
  const named = scalar ? { [uiInputField(intent.trigger)]: intent.input } : intent.input;
  if (intent.args === null) return named;
  if (typeof intent.args === "object" && !Array.isArray(intent.args) && typeof named === "object" && !Array.isArray(named)) return { ...intent.args, ...named };
  return named;
}

/** @emoji 🌉️ Bridges one semantic UI intent onto the existing plugin action address. Version one is the direct `ActionFactory` mapping; later versions stay explicit in the action name until the host wire owns a version field. */
export function uiIntentToActionDescriptor(intent: UiIntent): ActionDescriptor {
  const payload = uiIntentPayload(intent);
  return {
    controllerId: intent.action.scope,
    action: intent.action.version === 1 ? intent.action.name : `${intent.action.name}@${intent.action.version}`,
    ...(payload === undefined ? {} : { args: payload }),
  };
}

/** @emoji 🌲️ Hosts an authored semantic {@link BuiltNode} in the shell's panel-tree leaf without reviving the removed recursive `UiNode` compatibility model. */
export function uiNodeToTreePanelConfig(node: BuiltNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  const store = new UiDocumentStore(`panel:${node.key}`);
  store.loadSnapshot(builtNodeToSnapshot(`panel:${node.key}`, node));
  // 🧭️ Never park the interpreted body on an empty-label `TreeDataItem.control` — property-layout rows
  // split every row into a wide label column plus a fixed value column, which parked the lone default
  // `file-text` icon in the left column and squeezed the whole inspector/document/catalogue tree into
  // the value column (ticket 26/08/01 FIX-INSPECTOR-TREES-TO-MATCH-DOCUMENT, remeasured 2026-09-13).
  // Host the body as the tree's full-width `emptyState` instead; the inner document still renders its
  // own `Tree` when the snapshot root is `component.type === "tree"`.
  return {
    sections: [],
    emptyState: (
      <ShellFaultBoundary boundaryId={`panel-${node.key}`} fallbackLabel={shellLabel("ui.common.renderError")}>
        <div className="min-h-0 min-w-0 w-full flex-1">
          <InterpretedUiNode store={store} onAction={onAction} onIntent={(intent) => onAction(uiIntentToActionDescriptor(intent))} />
        </div>
      </ShellFaultBoundary>
    ),
    className: "min-w-0 w-full",
    sortableSections: false,
  };
}

export function shellTabIcon(iconId: IconName | string): React.FC<{ size?: number }> {
  return function ShellTabIcon({ size = 16 }: { size?: number }) {
    const iconName: IconName =
      iconId === FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID
        ? "file-text"
        : iconId === FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID
          ? "panel-catalogue"
          : iconId === FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID
            ? "panel-inspection"
            : iconId === FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID
              ? "panel-parameters"
              : isIconName(iconId)
                ? iconId
                : "circle-dot";
    return <Icon icon={iconName} size={size} />;
  };
}

/** @emoji 🌐️ Resolves a chrome translation key outside hook context (tree builders run there). Both bundles
 * are guaranteed complete for every key via `satisfies UiTranslationSchema`, so `?? key` is unreachable in
 * practice — kept only as a last-resort literal rather than a thrown error. `options` supports i18next
 * interpolation for keys with `{{placeholders}}`. */
export function shellLabel(key: UiTranslationKey, options?: Record<string, unknown>): UiLabel {
  return wireLabel(resolveTranslationLabel(uiI18n.t(key, options)) ?? key);
}

/**
 * @emoji 🌐️ Points the shared port {@link shellLabel} reads at `locale`. A shell has TWO i18n ports: its own
 * `ShellScope` instance, which `useUiTranslation`/`useLabel` resolve through, and this shared module port,
 * which is the only thing a tree builder running outside hook context can read. Moving the scope instance
 * alone relabels the hook-rendered chrome (fullscreen, the sync pill, the fold buttons) and pins every
 * builder-produced label — the panel tab names, the Settings branch with its General/Theme/Hotkeys children,
 * the Display/Tool/Command category names — at whatever language this module booted in, which renders one
 * shell in two languages at once. Both ports move together or neither does.
 */
export function syncShellLabelLocale(locale: Parameters<typeof uiI18n.changeLanguage>[0]): void {
  void uiI18n.changeLanguage(locale);
}

/** 🗂️ EN/DE label for a `UI_RIBBON_PARENT_CATEGORIES` id, resolved off the SAME `ui.ribbon.parent.*`
 * chrome bundle the ribbon itself reads (`uiRibbonParentEn`/`uiRibbonParentDe`) — the TypeScript twin
 * of `ui_wgpu::wgpu::ribbon_parent_label`. `undefined` for any id outside the closed 20-id taxonomy;
 * callers fall back to whatever raw id or overlay label they hold. The one resolver behind every
 * ribbon-parent lookup in this shell ({@link resolveUtilityGroupLabel}, `actionCategoryLabel`,
 * {@link contextMenuGroupLabel}) so the taxonomy never grows a second string table. */
export function ribbonParentLabel(category: string): UiLabel | undefined {
  const known = UI_RIBBON_PARENT_CATEGORIES.find((value) => value === category);
  return known === undefined ? undefined : shellLabel(`ui.ribbon.parent.${known}`);
}

/** 🗂️ Display label for a `menu.group.<category>` row, which the guest emits with `label: None` on
 * purpose (the host owns the chrome vocabulary — see `plugin/🦀️.rs`'s `Menu::group` doc and the wgpu
 * twin `shell_context_menu_item_from_spec`). Taxonomy categories resolve through
 * {@link ribbonParentLabel}; the overflow row `organizeContextMenu` synthesizes (`menu.group.more`)
 * is the one id outside the taxonomy and resolves through `ui.contextMenu.more`. `undefined` for any
 * row that is not a group row, or a group row whose category is unknown. */
export function contextMenuGroupLabel(id: string): UiLabel | undefined {
  if (!id.startsWith(CONTEXT_MENU_GROUP_ID_PREFIX)) return undefined;
  const category = id.slice(CONTEXT_MENU_GROUP_ID_PREFIX.length);
  return category === CONTEXT_MENU_OVERFLOW_CATEGORY ? shellLabel("ui.contextMenu.more") : ribbonParentLabel(category);
}

//#region 👁️✏️SurfaceRoleLabels
/** 👁️✏️ Frozen bilingual text pair (contract freeze §5) — English first, no default language. These
 * strings are net-new chrome vocabulary and the domain-neutral `uiChromeTranslationBundles` dictionary
 * they'd normally register into lives outside this lease (`🖱️ui/📦️packages/🟦️typescript/🎯️targets/
 * ⚛️react/📦️component.tsx`), so they resolve directly off `uiLocale` here — the same
 * `{ native: { en, de }, reuse: { en, de } }`-shaped idiom `resolveManifestLabel` already gives a
 * plugin's own `LocalizedLabel`, just constructed locally instead of decoded off the wire. */
type FrozenLabel = { readonly en: string; readonly de: string };

function frozenLabelText(pair: FrozenLabel, locale: string): string {
  return locale === "de" ? pair.de : pair.en;
}

/** 👁️✏️ Window title chip / read-only badge text — contract freeze §5. */
const SURFACE_ROLE_CHIP_LABEL: Readonly<Record<AppRole, FrozenLabel>> = {
  viewer: { en: "Viewer", de: "Betrachter" },
  editor: { en: "Editor", de: "Editor" },
};
export function surfaceRoleChipText(role: AppRole, locale: string): string {
  return frozenLabelText(SURFACE_ROLE_CHIP_LABEL[role], locale);
}

/** 👁️✏️ Accessible name of the navbar role `ButtonGroup` (`playground.navbar.roles`) — the group needs
 * its OWN name because each button is named by the target app's own `AppDefinition.label`
 * ("Editor"/"Viewer"), which says what you switch TO but not what axis the group controls. */
const SURFACE_ROLE_GROUP_LABEL: FrozenLabel = { en: "Surface role", de: "Oberflächenrolle" };
export function surfaceRoleGroupText(locale: string): string {
  return frozenLabelText(SURFACE_ROLE_GROUP_LABEL, locale);
}

/** 🎛️ Accessible name of the navbar mode `ButtonGroup` (`playground.navbar.modes`) — same reasoning as
 * {@link surfaceRoleGroupText}: the items are named by the plugin's own mode labels. */
const APP_MODE_GROUP_LABEL: FrozenLabel = { en: "Mode", de: "Modus" };
export function appModeGroupText(locale: string): string {
  return frozenLabelText(APP_MODE_GROUP_LABEL, locale);
}


const OPEN_ARTIFACT_WITH_LABEL: FrozenLabel = { en: "Open with…", de: "Öffnen mit…" };
export function openArtifactWithText(locale: string): string {
  return frozenLabelText(OPEN_ARTIFACT_WITH_LABEL, locale);
}

const SET_AS_DEFAULT_LABEL: FrozenLabel = { en: "Set as default", de: "Als Standard festlegen" };
export function setAsDefaultText(locale: string): string {
  return frozenLabelText(SET_AS_DEFAULT_LABEL, locale);
}

const DEFAULT_APPS_SETTINGS_TAB_LABEL: FrozenLabel = { en: "Default apps", de: "Standard-Apps" };
export function defaultAppsSettingsTabText(locale: string): string {
  return frozenLabelText(DEFAULT_APPS_SETTINGS_TAB_LABEL, locale);
}
/** 👁️✏️ {@link defaultAppsSettingsTabText} as a {@link UiLabel} — for `PanelTabNode.name`, which
 * (unlike a plain tree-item `label`) always takes the `wireLabel`-wrapped shape `shellLabel` returns. */
export function defaultAppsSettingsTabLabel(locale: string): UiLabel {
  return wireLabel(defaultAppsSettingsTabText(locale));
}

/** 👁️✏️ Not itself a contract-frozen string (the freeze pins the chip/"Open with…"/"Set as
 * default"/"Default apps" vocabulary, not this one) — the text for the non-blocking notice contract
 * freeze §2.3/§5 requires when a `"viewer.read-only"` fault surfaces, or a viewer-role dispatch is
 * blocked client-side before it ever reaches the host. */
const VIEWER_READ_ONLY_NOTICE_LABEL: FrozenLabel = { en: "This is a read-only viewer — editing is disabled.", de: "Dies ist ein schreibgeschützter Betrachter – Bearbeiten ist deaktiviert." };
export function viewerReadOnlyNoticeText(locale: string): string {
  return frozenLabelText(VIEWER_READ_ONLY_NOTICE_LABEL, locale);
}

/** 👁️✏️ The `SettingsDefaultApps` table's "no pin" option — no existing `ui.common.*` chrome key
 * covers a bare "None" (checked: `ui.common.close`/`.none` are not registered), so this follows the
 * same local-resolution idiom as the rest of this region rather than adding one to the out-of-lease
 * chrome dictionary for a single call site. */
const NONE_OPTION_LABEL: FrozenLabel = { en: "None", de: "Keine" };
export function noneOptionText(locale: string): string {
  return frozenLabelText(NONE_OPTION_LABEL, locale);
}

/** 👁️✏️ Palette command ids, frozen (contract freeze §5) — `owner: "os"`, no `os.` prefix (unlike the
 * wire `AppCommand`s in `💻️os/🎮️commands/`, which these are NOT the same thing as: selecting either
 * one opens the shell's own "Open with…" picker scoped to that role, it doesn't itself send
 * `os.open-artifact-with` over the app channel — the picker's own row click does that). */
export const OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID = "open-artifact-with-viewer";
export const OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID = "open-artifact-with-editor";

/** 👁️✏️ `true` for a `mutation`-kind action/command — the one predicate every viewer-chrome hiding
 * rule in this lease (context menu, command palette, dispatch guard) shares, so "what counts as an
 * editing verb" has exactly one definition. */
export function isMutationKindDefinition(definition: Pick<ActionDefinition, "kind"> | Pick<CommandDefinition, "kind">): boolean {
  return definition.kind === "mutation";
}

/** 👁️✏️ Filters `Mutation`-kind entries out of an action/command list for a `"viewer"` role — a no-op
 * for `"editor"` (and for `undefined`, since an app with no resolved session has no role to gate on
 * yet). Shared by the shell context menu and the command palette so both hide the exact same set. */
export function filterDefinitionsForRole<T extends Pick<ActionDefinition, "kind"> | Pick<CommandDefinition, "kind">>(definitions: readonly T[], role: AppRole | undefined): readonly T[] {
  if (role !== "viewer") return definitions;
  return definitions.filter((definition) => !isMutationKindDefinition(definition));
}

/** 👁️✏️ One `AppRouter` entry resolved for display in the "Open with…" surfaces (Document panel,
 * context menu, command palette) — plugin-labelled, flagged as the current session's own app and/or
 * the pinned default so the picker can render both without a second lookup. */
export type OpenWithEntry = {
  readonly app: AppRef;
  readonly pluginLabel: string;
  readonly current: boolean;
  readonly isDefault: boolean;
};

/** 👁️✏️ Groups one dialect's `AppRouter` entries by role for the "Open with…" surfaces — owner-plugin
 * first within each role group (the router's own build-time ordering, contract freeze §3), annotated
 * with `current`/`isDefault` against the live session and pinned prefs. `pluginLabel` falls back to
 * the raw `pluginId` when the plugin's own manifest label isn't available (e.g. not loaded yet). */
export function groupOpenWithEntries(
  router: AppRouter,
  dialect: ArtifactDialect,
  currentApp: AppRef | undefined,
  pinnedApp: (role: AppRole) => AppRef | undefined,
  pluginLabelById: ReadonlyMap<string, string>,
): Readonly<Record<AppRole, readonly OpenWithEntry[]>> {
  const forRole = (role: AppRole): readonly OpenWithEntry[] => {
    const pinned = pinnedApp(role);
    return router.entriesFor(dialect, role).map((app) => ({
      app,
      pluginLabel: pluginLabelById.get(app.pluginId) ?? app.pluginId,
      current: currentApp?.pluginId === app.pluginId && currentApp?.appId === app.appId,
      isDefault: pinned?.pluginId === app.pluginId && pinned?.appId === app.appId,
    }));
  };
  return { viewer: forRole("viewer"), editor: forRole("editor") };
}
//#endregion 👁️✏️SurfaceRoleLabels

//#region 🔖️CheckInAndSyncStatus
/** 📌️ ticket `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` §C5 — auto check-in
 * policy constants: an open editor session with uncommitted edits checkpoints once it has been idle
 * (no new edit) for this long, or immediately once this many uncommitted edits have piled up (never
 * waiting for an idle period once the threshold is crossed). */
export const AUTO_CHECKIN_IDLE_MS = 20_000;
export const AUTO_CHECKIN_EDIT_THRESHOLD = 200;

/** 📌️ A tiny, framework-free debounce scheduler for the auto check-in policy: call {@link notify}
 * every time the uncommitted-edit count changes for the currently open editor session; it fires
 * `onCheckpoint` at most once per idle period — immediately once `threshold` is reached (never
 * waiting out the idle period once crossed), otherwise `idleMs` after the LAST `notify` call, and
 * never again until `notify(0)` (a landed checkpoint) clears the `pending` latch — the "never a
 * storm" guard: a `notify` that arrives while a checkpoint is already pending is a no-op. Deliberately
 * framework-free (`setTimeout`/`clearTimeout` only, no React) so it is unit-testable with vitest's
 * fake timers without mounting a component tree — `ShellHost`'s own effect is a thin wrapper around
 * one instance per open editor session, `cancel`ed on unmount/session-switch. */
export class AutoCheckinScheduler {
  private timer: ReturnType<typeof setTimeout> | null = null;
  private pending = false;

  constructor(
    private readonly onCheckpoint: () => void,
    private readonly idleMs: number = AUTO_CHECKIN_IDLE_MS,
    private readonly threshold: number = AUTO_CHECKIN_EDIT_THRESHOLD,
  ) {}

  notify(uncommittedEditCount: number): void {
    if (uncommittedEditCount === 0) {
      this.cancel();
      this.pending = false;
      return;
    }
    if (this.pending) return;
    if (uncommittedEditCount >= this.threshold) {
      this.cancel();
      this.pending = true;
      this.onCheckpoint();
      return;
    }
    this.cancel();
    this.timer = setTimeout(() => {
      this.timer = null;
      this.pending = true;
      this.onCheckpoint();
    }, this.idleMs);
  }

  cancel(): void {
    if (this.timer != null) {
      clearTimeout(this.timer);
      this.timer = null;
    }
  }
}

const CHECKIN_ACTION_LABEL: FrozenLabel = { en: "Check In", de: "Einchecken" };
/** 📌️ `#s-checkin`'s own label — mirrors `⚛️react/🟦️.tsx`'s `ui.checkin.action` bilingual pair
 * (that barrel sits downstream of `ShellHost`, so it cannot be imported here without a cycle — see
 * `SurfaceRoleLabels`'s header doc for the identical constraint). */
export function checkinActionText(locale: string): string {
  return frozenLabelText(CHECKIN_ACTION_LABEL, locale);
}

const CHECKIN_MESSAGE_PLACEHOLDER_LABEL: FrozenLabel = { en: "Check-in message", de: "Check-in-Nachricht" };
export function checkinMessagePlaceholderText(locale: string): string {
  return frozenLabelText(CHECKIN_MESSAGE_PLACEHOLDER_LABEL, locale);
}

const CHECKIN_SUBMIT_LABEL: FrozenLabel = { en: "Commit", de: "Übernehmen" };
export function checkinSubmitText(locale: string): string {
  return frozenLabelText(CHECKIN_SUBMIT_LABEL, locale);
}

const CHECKIN_CANCEL_LABEL: FrozenLabel = { en: "Cancel", de: "Abbrechen" };
export function checkinCancelText(locale: string): string {
  return frozenLabelText(CHECKIN_CANCEL_LABEL, locale);
}

/** 👁️✏️ ticket §C5 item 5 — "viewers never checkpoint": the one predicate gating BOTH the
 * `#s-checkin` affordance's presence and the auto check-in timer's arming, mirroring
 * `isMutationKindDefinition`'s "one definition of what counts as an editing verb" precedent —
 * checkpoint is exactly such a verb. `VcsArtifactApp`'s own host-side guard already rejects a
 * viewer's `CommitCheckpoint` dispatch regardless; this is the client-side mirror that keeps the
 * affordance from ever reaching the wire in the first place. */
export function canCheckIn(role: AppRole | undefined): boolean {
  return role === "editor";
}

/** 🚦️ ticket §C5 — the sync status pill's three-way vocabulary, derived from `ArtifactSyncStatus`. A
 * non-live remote takes priority over a pending-mutation count (the user needs to know the
 * connection itself is degraded before anything about local pending edits); `null` (no status event
 * observed yet, e.g. before the document's first `open`) reads as `remote: "detached"`. */
export type SyncPillState = { readonly kind: "persisted" } | { readonly kind: "pending"; readonly count: number } | { readonly kind: "remote"; readonly remote: "connected" | "connecting" | "backoff" | "detached" };

export function computeSyncPillState(status: ArtifactSyncStatus | null): SyncPillState {
  if (!status || status.remote.kind !== "live") {
    return { kind: "remote", remote: !status ? "detached" : status.remote.kind === "live" ? "connected" : status.remote.kind };
  }
  if (status.pendingMutations > 0) return { kind: "pending", count: status.pendingMutations };
  return { kind: "persisted" };
}

const SYNC_STATUS_PERSISTED_LABEL: FrozenLabel = { en: "Persisted", de: "Gespeichert" };
const SYNC_STATUS_PENDING_LABEL: FrozenLabel = { en: "Pending", de: "Ausstehend" };
const SYNC_STATUS_REMOTE_LABEL: FrozenLabel = { en: "Remote", de: "Remote" };
const SYNC_STATUS_REMOTE_STATE_LABEL: Readonly<Record<"connected" | "connecting" | "backoff" | "detached", FrozenLabel>> = {
  connected: { en: "connected", de: "verbunden" },
  connecting: { en: "connecting", de: "verbindet" },
  backoff: { en: "backoff", de: "erneuter Versuch" },
  detached: { en: "detached", de: "getrennt" },
};

/** 🚦️ Localized pill text for `#s-sync-status` — matches contract §C5's own vocabulary (`persisted |
 * pending(n) | remote(connected|connecting|backoff|detached)`) closely enough that a state can be
 * read back off the rendered string in either locale, not just English. */
export function syncPillText(state: SyncPillState, locale: string): string {
  if (state.kind === "persisted") return frozenLabelText(SYNC_STATUS_PERSISTED_LABEL, locale);
  if (state.kind === "pending") return `${frozenLabelText(SYNC_STATUS_PENDING_LABEL, locale)} (${state.count})`;
  return `${frozenLabelText(SYNC_STATUS_REMOTE_LABEL, locale)}: ${frozenLabelText(SYNC_STATUS_REMOTE_STATE_LABEL[state.remote], locale)}`;
}
//#endregion 🔖️CheckInAndSyncStatus

/** @emoji 🧭️ The five panel tabs the framework itself owns (never app-supplied) — routed through the typed chrome schema instead of the plugin overlay so a locale-locked shell can never show their English manifest label. */
const FRAMEWORK_PANEL_TAB_LABEL_KEYS: Readonly<Record<string, UiTranslationKey>> = {
  [FRAMEWORK_PANEL_TAB_ARTIFACT_ID]: "ui.panel.artifact",
  [FRAMEWORK_PANEL_TAB_CATALOGUE_ID]: "ui.panel.catalogue",
  [FRAMEWORK_PANEL_TAB_INSPECTION_ID]: "ui.panel.inspection",
  [FRAMEWORK_PANEL_TAB_PARAMETERS_ID]: "ui.panel.parameters",
  [FRAMEWORK_PANEL_TAB_HISTORY_ID]: "ui.panel.history",
};

/** @emoji 🧭️ Framework-owned panel tabs resolve through the chrome schema (`shellLabel`); every other app-declared tab still resolves through the plugin overlay (`resolveAppLabel`). */
export function resolvePanelTabLabel(overlay: PluginAppLabelsOverlay, tabId: string, fallback: string): string {
  const chromeKey = FRAMEWORK_PANEL_TAB_LABEL_KEYS[tabId];
  return chromeKey ? shellLabel(chromeKey) : resolveAppLabel(overlay, "panelTab", tabId, fallback);
}

/** @emoji 🗣️ Stable empty overlay reference so components depending on it don't re-render before the first `appLabels` fetch resolves. */
export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** @emoji 🛍️ Stable empty catalogue reference so scene hosts depending on `AppCatalogueContext` don't
 * re-render before the first `catalogue` section fetch resolves. */
export const EMPTY_APP_CATALOGUE: AppCatalogue = Object.freeze({});

/** 🗺️ Synthesizes a full `LocalizedLabel` matrix from a user-authored string by broadcasting it across all cells (native/reuse × en/de), matching Rust's `LocalizedLabel::data(...)`. Also accepts an existing `LocalizedLabel` idempotently. */
export function synthesizeLocalizedLabel(label: string | LocalizedLabel): LocalizedLabel {
  if (typeof label !== "string") return label;
  return {
    native: { en: label, de: label },
    reuse: { en: label, de: label },
  };
}

/** 🗺️ Resolves an exact manifest terminology/locale cell or already-resolved/user-authored text. Malformed cells remain unresolved; no language is selected implicitly. */
export function resolveManifestLabel(label: unknown, terminology: string, locale: string): string {
  if (typeof label === "string") return label;
  if (!label || typeof label !== "object" || !("native" in label) || !("reuse" in label)) return "";
  if (terminology !== "native" && terminology !== "reuse") return "";
  if (locale !== "en" && locale !== "de") return "";
  const row = label[terminology];
  if (!row || typeof row !== "object" || !("en" in row) || !("de" in row)) return "";
  const value = row[locale];
  return typeof value === "string" ? value : "";
}

/** @emoji 🗣️ Resolves a window-kind/panel-tab/mode/action/utility/example/actionArg/dialog/introduction/group id's locale-aware label from the active app's overlay, falling back to the static manifest label. */
export function resolveAppLabel(overlay: PluginAppLabelsOverlay, kind: "windowKind" | "panelTab" | "mode" | "action" | "utility" | "example" | "actionArg" | "dialog" | "introduction" | "group", id: string, fallback: string): string {
  const map =
    kind === "windowKind"
      ? overlay.windowKindLabels
      : kind === "panelTab"
        ? overlay.panelTabLabels
        : kind === "mode"
          ? overlay.modeLabels
          : kind === "action"
            ? overlay.actionLabels
            : kind === "utility"
              ? overlay.utilityLabels
              : kind === "example"
                ? overlay.exampleLabels
                : kind === "actionArg"
                  ? overlay.actionArgLabels
                  : kind === "dialog"
                    ? overlay.dialogLabels
                    : kind === "introduction"
                      ? overlay.introductionLabels
                      : overlay.groupLabels;
  return map[id] ?? fallback;
}

/** @emoji 🗣️ Resolves one action-arg's label + (for `select` controls) its options' labels from the overlay's `actionArgLabels` map, keyed `"{scopeId}.{argId}"` / `"{scopeId}.{argId}.option.{value}"`. `scopeId` is an action id for staged/palette forms, a dialog id for dialog args, or a command id for command args. `ActionArgDef.label`/`ActionArgOption.label` are manifest `LocalizedLabel` fields, resolved for `terminology`/`locale` before the overlay's (always-empty, see the `AppLabelsOverlay` deletion note) fallback lookup even applies. */
export type ResolvedActionArgDef = Omit<ActionArgDef, "label" | "schema"> & {
  readonly label: string;
  readonly schema: Exclude<ActionArgDef["schema"], { kind: "string" }> | (Omit<Extract<ActionArgDef["schema"], { kind: "string" }>, "options"> & { readonly options: { value: string; label: string }[] });
};

export type ResolvedActionDefinition = Omit<ActionDefinition, "label" | "args"> & { readonly label: string; readonly args: ResolvedActionArgDef[] };
export type ResolvedToolDefinition = Omit<ToolDefinition, "label"> & { readonly label: string };

/** 🎚️ A free-text string argument carries no choice list at all: the manifest wire form declares
 * `options` `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, so an absent field is exactly
 * an empty option set rather than a malformed schema. */
type ActionArgStringSchema = Extract<ActionArgDef["schema"], { kind: "string" }>;
function actionArgStringOptions<T>(schema: { readonly options?: readonly T[] }): readonly T[] {
  return schema.options ?? [];
}

function resolveArtifactKindChoiceLabel(choice: { readonly kindId: string; readonly label: { readonly en: string; readonly de: string } }, locale: string): string {
  if (locale === "en") return choice.label.en;
  if (locale === "de") return choice.label.de;
  return choice.kindId;
}

function resolveActionArgDef(def: ActionArgDef, scopeId: string, overlay: PluginAppLabelsOverlay, terminology: string, locale: string, manifests: readonly { readonly apps: readonly unknown[] }[], selectedArtifactKinds?: readonly ArtifactKindChoice[]): ResolvedActionArgDef {
  const label = resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}`, resolveManifestLabel(def.label, terminology, locale));
  if (def.schema.kind !== "string") return { ...def, label, schema: def.schema };
  if (def.schema.format?.kind === "artifactKind") {
    const choices = selectedArtifactKinds ?? artifactKindChoices(manifests, def.schema.format.roles);
    const options = choices.map((choice) => ({ value: encodeArtifactKindChoice(choice), label: resolveArtifactKindChoiceLabel(choice, locale) }));
    return { ...def, label, schema: { ...def.schema, options } };
  }
  const options = actionArgStringOptions(def.schema).map((option) => ({ ...option, label: resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}.option.${option.value}`, resolveManifestLabel(option.label, terminology, locale)) }));
  return { ...def, label, schema: { ...def.schema, options } };
}

/** @emoji 🗣️ Resolves a `DialogDefinition`'s title/body/submitLabel/cancelLabel/args from the overlay's `dialogLabels`/`actionArgLabels` maps, keyed by the dialog's own id. `title`/`body`/`submitLabel`/`cancelLabel` are all manifest `LocalizedLabel` fields. */
export function resolveDialogDefinition(dialog: DialogDefinition, overlay: PluginAppLabelsOverlay, terminology: string, locale: string, manifests: readonly { readonly apps: readonly unknown[] }[] = [], selectedArtifactKinds?: readonly ArtifactKindChoice[]): Omit<DialogDefinition, "args"> & { readonly args: ResolvedActionArgDef[] } {
  return {
    ...dialog,
    title: resolveAppLabel(overlay, "dialog", `${dialog.id}.title`, resolveManifestLabel(dialog.title, terminology, locale)),
    body: dialog.body ? resolveAppLabel(overlay, "dialog", `${dialog.id}.body`, resolveManifestLabel(dialog.body, terminology, locale)) : dialog.body,
    submitLabel: resolveAppLabel(overlay, "dialog", `${dialog.id}.submit`, resolveManifestLabel(dialog.submitLabel, terminology, locale)),
    cancelLabel: dialog.cancelLabel ? resolveAppLabel(overlay, "dialog", `${dialog.id}.cancel`, resolveManifestLabel(dialog.cancelLabel, terminology, locale)) : dialog.cancelLabel,
    args: dialog.args.map((def) => resolveActionArgDef(def, dialog.id, overlay, terminology, locale, manifests, selectedArtifactKinds)),
  };
}

/** @emoji 🗣️ Resolves an `IntroductionDefinition`'s title and every step's title/body labels from the
 * overlay's `introductionLabels` map. `title`/`body` are manifest `LocalizedLabel` fields;
 * `IntroductionInteraction.label` is a short checklist caption that is still a plain `String` on the Rust
 * side (not part of the `LocalizedLabel` migration), so it is left as-is. */
export function resolveIntroductionDefinition(introduction: IntroductionDefinition, overlay: PluginAppLabelsOverlay, terminology: string, locale: string): IntroductionDefinition {
  return {
    title: resolveAppLabel(overlay, "introduction", "intro.title", resolveManifestLabel(introduction.title, terminology, locale)),
    steps: introduction.steps.map(
      (step): IntroductionStepDefinition => ({
        ...step,
        title: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.title`, resolveManifestLabel(step.title, terminology, locale)),
        body: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.body`, resolveManifestLabel(step.body, terminology, locale)),
        interactions: (step.interactions ?? []).map((interaction, index) => ({
          ...interaction,
          label: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.interaction.${index}.label`, interaction.label),
        })),
        ordered: step.ordered ?? false,
      }),
    ),
  };
}

//#region 🎥️TutorialUiBridge
/** @emoji 🕹️ Copies the exact typed selection projection without retaining mutable manifest arrays or invoking special object keys. */
function captureInteractionSelection(state: ShellState): TutorialUiSnapshot["interactionSelection"] {
  const selection: Record<string, { granularity: string; ids: string[]; anchorId?: string }> = {};
  for (const [domainId, current] of Object.entries(state.interaction.selection)) {
    const captured = { granularity: current.granularity, ids: [...current.ids], ...(current.anchorId === undefined ? {} : { anchorId: current.anchorId }) };
    Object.defineProperty(selection, domainId, { value: captured, enumerable: true, writable: true, configurable: true });
  }
  return selection;
}

/** @emoji 🛡️ Converts the generated optional-value map into the total runtime selection projection. */
function tutorialInteractionSelection(selection: TutorialUiSnapshot["interactionSelection"]): InteractionState["selection"] {
  const result: Record<string, DomainSelection> = {};
  for (const [domainId, current] of Object.entries(selection)) {
    if (!current) continue;
    const captured = { granularity: current.granularity, ids: [...current.ids], ...(current.anchorId === undefined ? {} : { anchorId: current.anchorId }) };
    Object.defineProperty(result, domainId, { value: captured, enumerable: true, writable: true, configurable: true });
  }
  return result;
}

/** @emoji 🎥️ Captures the shell's current `ShellState` (+ active session) as a renderer-neutral `TutorialUiSnapshot` — the recorder's periodic full-snapshot keyframes and the `TutorialBar`'s "record" path both call this. See the Rust doc comment on `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState` itself. */
export function captureTutorialUiSnapshot(state: ShellState, session: ActiveSession | null): TutorialUiSnapshot {
  const activeUtilityByWindowId: Record<string, string> = {};
  for (const [windowId, utilityId] of Object.entries(state.actionPane.activeUtilityByWindowId)) {
    if (utilityId) activeUtilityByWindowId[windowId] = utilityId;
  }
  const activePanelTabByGroup: Record<string, string> = {};
  for (const anchor of ANCHORS) {
    const panelState = state.layout.panels[anchor];
    const tabId = panelState.path[panelState.path.length - 1];
    if (panelState.visible && tabId) activePanelTabByGroup[anchor] = tabId;
  }
  return {
    activeModeId: session?.viewState.activeModeId,
    focusedWindowId: state.layout.activeWindowId ?? undefined,
    activeUtilityByWindowId,
    activeToolId: state.actionPane.activeToolId ?? undefined,
    layout: captureCurrentFrameworkLayout(state.layout.shellLayout, state.layout.extraWindowInstances),
    activePanelTabByGroup,
    panelJson: session?.viewState.panelJson,
    interactionSelection: captureInteractionSelection(state),
    openDialogId: state.overlays.dialog?.dialogId,
    expandedTreeIds: Object.entries(state.layout.treeOpenStates).filter(([, open]) => open).map(([id]) => id),
    commandPanelOpen: state.overlays.searchOpen,
  };
}

/** @emoji 🎥️ Context every `applyTutorialUiSnapshotToShell`/`applyTutorialUiChangeToShell` call needs beyond `dispatch` itself — resolved once per render by the caller (the director/seek/deviation-converge paths all share it). */
export type TutorialUiBridgeContext = {
  readonly session: ActiveSession | null;
  readonly restoreDialog: (dialogId: string, seedArgs?: Readonly<Record<string, unknown>>) => ShellDialogV1 | null;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
  readonly terminology: string;
  readonly locale: string;
  readonly interactionSelection: () => InteractionState["selection"];
  readonly publishInteractionSelection: (selection: InteractionState["selection"]) => void;
};

/** @emoji 🎥️ Applies a full `TutorialUiSnapshot` (a `TutorialUiSample::Snapshot`, or the composed target of a seek/deviation-converge) onto the live `ShellState` — snaps every field instantly (camera is the only interpolated track, applied separately by the director). Dispatches the atomic `APPLY_TUTORIAL_UI_SNAPSHOT` for shell-owned fields, including typed interaction selection, plus one `SET_SESSION` for `ActiveSession.viewState`'s `activeModeId`/`panelJson`. */
export function applyTutorialUiSnapshotToShell(dispatch: (action: ShellAction) => void, snapshot: TutorialUiSnapshot, ctx: TutorialUiBridgeContext): void {
  const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
  const seed = applyFrameworkLayoutSeed(snapshot.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
  const panelPatches: Partial<Record<Anchor, { readonly visible: boolean; readonly path: readonly string[] }>> = {};
  for (const anchor of ANCHORS) {
    const tabId = snapshot.activePanelTabByGroup[anchor];
    panelPatches[anchor] = tabId ? { visible: true, path: [tabId] } : { visible: false, path: [] };
  }
  const treeOpenStates: Record<string, boolean> = {};
  for (const id of snapshot.expandedTreeIds) treeOpenStates[id] = true;
  const activeUtilityByWindowId: Record<string, string | null> = {};
  for (const [windowId, utilityId] of Object.entries(snapshot.activeUtilityByWindowId)) {
    if (utilityId !== undefined) activeUtilityByWindowId[windowId] = utilityId;
  }
  dispatch({
    type: "APPLY_TUTORIAL_UI_SNAPSHOT",
    snapshot: {
      activeWindowId: snapshot.focusedWindowId ?? null,
      shellLayout: seed.modeLayout,
      extraWindowInstances: seed.extraInstances,
      panelPatches,
      treeOpenStates,
      activeUtilityByWindowId,
      activeToolId: snapshot.activeToolId ?? null,
      dialog: snapshot.openDialogId === undefined ? null : ctx.restoreDialog(snapshot.openDialogId),
      commandPanelOpen: snapshot.commandPanelOpen,
    },
  });
  ctx.publishInteractionSelection(tutorialInteractionSelection(snapshot.interactionSelection));
  if (ctx.session) {
    dispatch({
      type: "SET_SESSION",
      value: (current) =>
        current
          ? {
              ...current,
              viewState: {
                ...current.viewState,
                activeModeId: snapshot.activeModeId ?? current.viewState.activeModeId,
                panelJson: snapshot.panelJson ?? current.viewState.panelJson,
              },
            }
          : current,
    });
  }
}

/** @emoji 🎥️ Applies one sparse `TutorialUiChange` (a `TutorialUiSample::Delta` entry, replayed by the director's per-tick `tutorialSlice`) onto the live `ShellState` by dispatching the SAME existing, targeted `ShellAction`s the real UI's own interactions use — never a bespoke tutorial-only mutation channel. */
export function applyTutorialUiChangeToShell(dispatch: (action: ShellAction) => void, change: TutorialUiChange, ctx: TutorialUiBridgeContext): void {
  switch (change.kind) {
    case "activeMode":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, activeModeId: change.id } } : current) });
      return;
    case "focusedWindow":
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: change.id ?? null });
      return;
    case "activeUtility":
      dispatch({ type: "SET_ACTIVE_UTILITY", windowId: change.windowId, utilityId: change.utilityId ?? null });
      return;
    case "activeTool":
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: change.id ?? null });
      return;
    case "layout": {
      const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
      const seed = applyFrameworkLayoutSeed(change.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
      dispatch({ type: "SET_SHELL_LAYOUT", value: seed.modeLayout });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seed.extraInstances });
      return;
    }
    case "panelTab": {
      const anchor = change.group as Anchor;
      if (!(ANCHORS as readonly string[]).includes(anchor)) return;
      dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: change.tabId != null });
      dispatch({ type: "SET_PANEL_PATH", anchor, value: change.tabId ? [change.tabId] : [] });
      return;
    }
    case "panelState":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, panelJson: change.panelJson } } : current) });
      return;
    case "selection": {
      const selection = { ...ctx.interactionSelection() };
      if (change.ids.length === 0) delete selection[change.domainId];
      else Object.defineProperty(selection, change.domainId, { value: { granularity: change.granularity, ids: [...change.ids] }, enumerable: true, writable: true, configurable: true });
      ctx.publishInteractionSelection(selection);
      return;
    }
    case "dialog":
      dispatch({ type: "SET_DIALOG", value: change.id ? ctx.restoreDialog(change.id, change.args as Record<string, unknown> | undefined) : null });
      return;
    case "treeExpansion":
      dispatch({ type: "SET_TREE_OPEN_STATE", id: change.id, open: change.expanded });
      return;
    case "commandPanel":
      dispatch({ type: "SET_SEARCH_OPEN", value: change.open });
      return;
    default:
      return;
  }
}
//#endregion 🎥️TutorialUiBridge

/** @emoji 🗣️ Resolves a terminology id's display name; chrome-known ids get a translated label, app-declared ids fall back to their raw id. */
export function shellTerminologyLabel(id: string): string {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id as UiChromeTerminologyId}`) : id;
}

/** @emoji 🎚️ Serializes async updates while retaining only the newest value requested during an in-flight update. */
export function createLatestAsyncDispatcher<T>(dispatchValue: (value: T) => unknown): (value: T) => void {
  let running = false;
  let queued: T | undefined;
  let hasQueued = false;
  const dispatchLatest = (value: T) => {
    if (running) {
      queued = value;
      hasQueued = true;
      return;
    }
    running = true;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      running = false;
      if (!hasQueued) return;
      const next = queued as T;
      queued = undefined;
      hasQueued = false;
      dispatchLatest(next);
    });
  };
  return dispatchLatest;
}

/** @emoji ↕️ Serializes numeric slider updates while retaining every direction change and coalescing movement within one direction. */
export function createDirectionalAsyncDispatcher(dispatchValue: (value: number) => unknown): (value: number) => void {
  let running = false;
  let active = 0;
  const queued: number[] = [];
  const dispatchNext = (value: number) => {
    running = true;
    active = value;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      const next = queued.shift();
      if (next === undefined) {
        running = false;
        return;
      }
      dispatchNext(next);
    });
  };
  return (value) => {
    if (!running) {
      dispatchNext(value);
      return;
    }
    const previous = queued.at(-1);
    if (previous === undefined) {
      if (value !== active) queued.push(value);
      return;
    }
    const anchor = queued.at(-2) ?? active;
    const direction = Math.sign(previous - anchor);
    const nextDirection = Math.sign(value - previous);
    if (nextDirection === 0) return;
    if (direction === 0 || nextDirection === direction) queued[queued.length - 1] = value;
    else queued.push(value);
    // 🔁️ A jittery drag (rapid direction reversals while a round trip is in flight) would otherwise grow
    // `queued` by one entry per reversal; only the last two are ever needed (the pending value and the
    // anchor used to detect the next reversal), so cap it there.
    if (queued.length > 2) queued.splice(0, queued.length - 2);
  };
}

//#region RevealCutoffStore
/**
 * @emoji 🪣️ Live per-gesture visibility cutoff for reveal-tagged instances (`WorldInstanceRecord.revealIndex`,
 * set by a `WindowMeasure.Slider.reveal` group). Main-thread-only and never dispatched: a slider drag writes
 * here directly, `WorldInstancesLayer` subscribes and imperatively toggles `Object3D.visible` — zero React
 * re-render, zero WASM round trip. Reconciled from the plugin's committed `WorldInteractionRecord.revealCutoffs`
 * whenever that value changes (a no-operation during a live drag, since the committed value only changes on commit).
 */
export type RevealCutoffStore = {
  get(groupId: string): number | undefined;
  set(groupId: string, value: number): void;
  subscribe(groupId: string, listener: (value: number | undefined) => void): () => void;
};

export function createRevealCutoffStore(): RevealCutoffStore {
  const values = new Map<string, number>();
  const listeners = new Map<string, Set<(value: number | undefined) => void>>();
  return {
    get: (groupId) => values.get(groupId),
    set: (groupId, value) => {
      values.set(groupId, value);
      for (const listener of listeners.get(groupId) ?? []) listener(value);
    },
    subscribe: (groupId, listener) => {
      let group = listeners.get(groupId);
      if (!group) {
        group = new Set();
        listeners.set(groupId, group);
      }
      group.add(listener);
      return () => {
        group!.delete(listener);
      };
    },
  };
}

/** Shared instance — a reveal group id is app-instance-global in v1; namespace by app instance id if a second concurrent document instance ever needs independent cutoffs. */
export const worldRevealCutoffStore = createRevealCutoffStore();

/** The only reveal group that exists today — puzzle3d's fill-plan slider. */
export const PUZZLE3D_FILL_REVEAL_GROUP_ID = "puzzle3d-fill";

/**
 * @emoji 🪣️ Writes committed reveal cutoffs into `store` only when the numeric value for a group changes.
 * Ignores object-identity churn from `fillBuildTick` refreshes so a live slider drag is not reset mid-gesture.
 */
export function reconcileCommittedRevealCutoffs(
  store: RevealCutoffStore,
  committedRef: { current: Readonly<Record<string, number>> },
  revealCutoffs: Readonly<Record<string, number>>,
): void {
  for (const [groupId, value] of Object.entries(revealCutoffs)) {
    if (committedRef.current[groupId] === value) continue;
    committedRef.current = { ...committedRef.current, [groupId]: value };
    store.set(groupId, value);
  }
}

/** @emoji 🙈️ True for a reveal-tagged instance beyond the live cutoff — `WorldInstancesLayer` already
 * hides its root imperatively, but pure functions that read `instances` data directly (marquee hit
 * testing) don't see three.js `Object3D.visible` and need this check instead. Untagged instances are
 * never cutoff-hidden: the nullish guard also rejects a JSON `null`, which would otherwise compare as `0`
 * and hide every ordinary object while the cutoff sits at its boot value. */
export function isRevealCutoffHidden(instance: Pick<WorldInstanceRecord, "revealIndex">): boolean {
  if (instance.revealIndex == null) return false;
  const cutoff = worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID);
  return cutoff !== undefined && instance.revealIndex >= cutoff;
}

/** @emoji 🖱️ Overlay chrome for a world-3d marquee method. `rectangle` draws a box; `lasso` draws a
 * polygon; `pick` has no drag chrome. */
export function world3dMarqueeOverlayShape(method: string): "rect" | "polygon" | null {
  if (method === "lasso") return "polygon";
  if (method === "rectangle") return "rect";
  return null;
}
//#endregion RevealCutoffStore

//#region 🛑️ExtensionRequestCancellation
/**
 * @emoji 🛑️ Every extension request currently in flight, keyed by the REQUESTING actor
 * (`<pluginId>:<instanceId>`) — the same key `serializePerActor` already serializes those requests
 * under, because they are the same set of calls.
 *
 * 🚪️ Why the host owns this and not the guest: all of one instance's extension invocations are
 * serialized under that one key, so a cancel the guest emitted could not overtake the request it
 * means to stop — it would queue BEHIND it. Only the host, outside that queue, can abort a call
 * already handed to the door. `driveInboundRequest` reads the signal at turn boundaries, so a parked
 * multi-turn request is retired and a turn already handed to the worker is never half-abandoned.
 */
const inFlightExtensionRequestsByActor = new Map<string, Set<AbortController>>();

/**
 * @emoji 🛑️ Action ids a mounted surface has DECLARED as its cancel affordance — the surface reads
 * the id off its own status contract (`World3dScene.statusJson`'s `cancelAction`) and registers it
 * here while that status says `cancellable`. This is what keeps the shell domain-neutral: it never
 * learns a plugin's verb from code, only from the surface that is currently offering it.
 */
const declaredSurfaceCancelActions = new Map<string, number>();

/** 🛑️ Registers one in-flight extension request and hands back its signal plus a retirement. */
export function beginCancellableExtensionRequest(actorKey: string): { readonly signal: AbortSignal; readonly finish: () => void } {
  const controller = new AbortController();
  let live = inFlightExtensionRequestsByActor.get(actorKey);
  if (!live) {
    live = new Set();
    inFlightExtensionRequestsByActor.set(actorKey, live);
  }
  live.add(controller);
  return {
    signal: controller.signal,
    finish: () => {
      const set = inFlightExtensionRequestsByActor.get(actorKey);
      if (!set) return;
      set.delete(controller);
      if (set.size === 0) inFlightExtensionRequestsByActor.delete(actorKey);
    },
  };
}

/** 🛑️ Aborts every extension request `actorKey` has in flight. Returns how many were aborted. */
export function abortExtensionRequestsForActor(actorKey: string, reason: string): number {
  const live = inFlightExtensionRequestsByActor.get(actorKey);
  if (!live || live.size === 0) return 0;
  const aborted = live.size;
  for (const controller of [...live]) controller.abort(new Error(reason));
  inFlightExtensionRequestsByActor.delete(actorKey);
  return aborted;
}

/** 🔎️ How many extension requests `actorKey` has in flight — readable so a law can state it. */
export function inFlightExtensionRequestCount(actorKey: string): number {
  return inFlightExtensionRequestsByActor.get(actorKey)?.size ?? 0;
}

/** 🛑️ Declares `actionId` as a mounted surface's live cancel affordance; call the returned
 * retirement when the surface stops offering it (it unmounted, or its status stopped being
 * cancellable). Reference-counted, because two preview windows may offer the same verb. */
export function declareSurfaceCancelAction(actionId: string): () => void {
  if (!actionId) return () => undefined;
  declaredSurfaceCancelActions.set(actionId, (declaredSurfaceCancelActions.get(actionId) ?? 0) + 1);
  let retired = false;
  return () => {
    if (retired) return;
    retired = true;
    const count = (declaredSurfaceCancelActions.get(actionId) ?? 1) - 1;
    if (count <= 0) declaredSurfaceCancelActions.delete(actionId);
    else declaredSurfaceCancelActions.set(actionId, count);
  };
}

/** 🔎️ Whether `actionId` is a cancel affordance some mounted surface is currently offering. */
export function isDeclaredSurfaceCancelAction(actionId: string): boolean {
  return declaredSurfaceCancelActions.has(actionId);
}
//#endregion 🛑️ExtensionRequestCancellation

/**
 * @emoji 🚦️ Fires `run` at most once at a time — interval ticks that arrive while a previous run is still
 * in flight are dropped (not queued). Used by World3dHost's `suggestionsTick`/`fillBuildTick` loops so a
 * slow program tick cannot unbounded-queue into the serialized WASM handle and starve the fill utility.
 */
export function createInFlightSkippingInterval<Timer>(run: () => unknown, delayMs: number, setIntervalFn?: (callback: () => void, delayMs: number) => Timer, clearIntervalFn?: (timer: Timer) => void): () => void {
  let cancelled = false;
  let inFlight = false;
  const tick = () => {
    if (cancelled || inFlight) return;
    inFlight = true;
    void Promise.resolve(run()).finally(() => {
      inFlight = false;
    });
  };
  let disposeTimer: () => void;
  if (setIntervalFn && clearIntervalFn) {
    const timer = setIntervalFn(tick, delayMs);
    disposeTimer = () => clearIntervalFn(timer);
  } else {
    if (setIntervalFn || clearIntervalFn) throw new Error("interval scheduling and cancellation ports must be supplied together");
    const timer = setInterval(tick, delayMs);
    disposeTimer = () => clearInterval(timer);
  }
  return () => {
    cancelled = true;
    disposeTimer();
  };
}

export { beginIsolatedJobDrive, endIsolatedJobDrive, requestIsolatedJobUiPoll, takeIsolatedJobUiPoll, isolatedJobDriveIsActive, subscribeIsolatedJobDrive, isolatedJobDriveSnapshot } from "../🔌️PluginRuntime/🟦️.tsx";

/**
 * @emoji 🎯️ Coalesces rapid dispatches to the latest value — skips when unchanged and keeps at most one
 * in-flight round trip (used by World3dHost hover so pointermove cannot flood the WASM handle).
 */
export function createCoalescingActionDispatcher<T>(dispatch: (value: T) => unknown, isEqual: (a: T, b: T) => boolean = (a, b) => Object.is(a, b)): (value: T) => void {
  let inFlight = false;
  let pending: T | undefined;
  let lastSent: T | undefined;
  const flush = () => {
    if (inFlight || pending === undefined) return;
    const next = pending;
    pending = undefined;
    if (lastSent !== undefined && isEqual(lastSent, next)) return;
    lastSent = next;
    inFlight = true;
    // 🩹️ A REFUSED round trip frees the gate exactly like a settled one, and its rejection is already
    // reported through the runtime's own fault channel — one `then(release, release)` handles both in the
    // SAME microtask hop a bare `finally` took, so a caller that now hands over a real awaitable
    // (wave B33 §4) cannot turn a refused hover into an unhandled rejection or a wedged lane.
    const release = () => {
      inFlight = false;
      flush();
    };
    void Promise.resolve(dispatch(next)).then(release, release);
  };
  return (value: T) => {
    if (pending === undefined && lastSent !== undefined && isEqual(lastSent, value)) return;
    pending = value;
    flush();
  };
}

//#region 🥽️Puzzle3dBrushMeshUpload
/** 📏️ Raw JSON bytes one retained puzzle command admits — `PUZZLE_COMMAND_RAW_BYTES`
 * (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`). A whole GLB never fitted it: the Nakagin
 * capsule `🧊️placeholder.glb` is 25 344 positions plus 48 384 indices, 64 KB as JSON number arrays,
 * so real collision geometry reaches the plugin only as a page run. */
export const PUZZLE3D_MESH_COMMAND_RAW_BYTES = 8_192;

/** 📏️ Values — positions and indices counted together — one page carries at most. Mirrors
 * `PUZZLE3D_MESH_PAGE_VALUES` (`✏️editor/⏳️precompute/🦀️.rs`); a mesh id long enough to crowd the
 * envelope shrinks its own pages below this instead of overrunning the wire. */
export const PUZZLE3D_MESH_PAGE_VALUES = 1_024;

/** 🔤️ Base64 padding characters reserved per page — one group per payload string. */
const PUZZLE3D_MESH_PAGE_PADDING_CHARS = 8;

/** 📦️ Partial uploads the plugin stages at once — `PUZZLE3D_MESH_UPLOAD_SLOTS`
 * (`✏️editor/⏳️precompute/🦀️.rs`). The host drains its queue one page at a time, so it only ever holds
 * one run open; the count bounds how many runs may wait behind it. */
export const PUZZLE3D_MESH_UPLOAD_SLOTS = 4;

/** 🧮️ Longest page run one mesh identity may claim — `PUZZLE3D_MESH_UPLOAD_MAX_PAGES`
 * (`✏️editor/⏳️precompute/🦀️.rs`). A mesh needing more pages than this is beyond what the collision
 * engine admits at all, so it is never paged onto the wire. */
export const PUZZLE3D_MESH_UPLOAD_MAX_PAGES = 384;

/** 📦️ Pages the host may hold undispatched: every staging slot the plugin owns, each at its own run
 * ceiling. A run that would overrun this is refused here rather than queued into unbounded memory. */
export const PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES = PUZZLE3D_MESH_UPLOAD_SLOTS * PUZZLE3D_MESH_UPLOAD_MAX_PAGES;

/** 🚚️ How many times one mesh identity's guest-side re-upload request may be claimed under a single
 * guest instantiation — see {@link Puzzle3dBrushMeshRegistry.claimReupload}. Two: the page run the
 * request asks for, plus exactly one retry for a run that failed part-way (a `Gap`, a refused digest, a
 * world host unmounted mid-drain). A third claim can only ever repeat an outcome the first two already
 * produced, and each one costs a full `PUZZLE3D_MESH_UPLOAD_MAX_PAGES`-bounded run on the serialized
 * per-actor command lane, which is what every user interaction then queues behind. */
export const PUZZLE3D_MESH_REUPLOAD_CLAIMS = 2;

/** 🥽️ One page of a `registerBrushMesh` upload run. Positions fill a page first; the indices stream
 * continues in whatever of the page's value budget is left, so the run is dense and its last page is
 * the only partial one. */
export type Puzzle3dBrushMeshPage = {
  readonly url: string;
  readonly digest: string;
  readonly page: number;
  readonly pageCount: number;
  readonly positionsB64?: string;
  readonly indicesB64?: string;
};

/** 🥽️ What this page believes ONE guest instantiation holds: mesh identities it paged, by id, with the
 * digest that was uploaded — so a later window adopts the same geometry by `{url, digest}` alone
 * instead of paging it again, and a mesh whose bytes changed uploads afresh.
 *
 * The claim is bounded by the guest's lifetime, never by the tab's. A plain `Map` outlived every fact
 * it recorded: the plugin's own mesh store is a `static OnceLock` in the wasm instantiation
 * (`✏️editor/⏳️precompute/🦀️.rs`), so a restored actor holds nothing, and the seven id-only
 * re-announcements that every later window activation then sent were each refused and each dropped on
 * the floor — the brush utility silently kept no collision geometry until a full page reload. Two
 * guest-published facts close that hole, both on `interactionJson`:
 *
 * - `meshResidency` — the guest's monotone install counter. It only ever climbs inside one
 *   instantiation and starts at zero in a fresh one, so a value below the high-water mark this page
 *   saw is proof of a restart and voids every entry ({@link observeResidency}).
 * - `meshReuploadUrls` — identities a refused id-only announcement is waiting on bytes for
 *   ({@link claimReupload}), claimed at most {@link PUZZLE3D_MESH_REUPLOAD_CLAIMS} times per guest
 *   instantiation so a republished stale scene cannot re-drive an upload that already ran. */
export class Puzzle3dBrushMeshRegistry {
  #residency = -1;
  readonly #entries = new Map<string, { readonly digest: string; readonly paged: boolean }>();
  readonly #claims = new Map<string, number>();
  readonly #refusedAlias = new Set<string>();

  /** 🔄️ Folds one published `meshResidency` in. Answers `true` exactly when the count went backwards —
   * the guest was re-instantiated and holds nothing this page uploaded — having dropped every claim. */
  observeResidency(installs: number): boolean {
    if (!Number.isFinite(installs) || installs < 0) return false;
    const restarted = installs < this.#residency;
    this.#residency = installs;
    if (restarted) {
      this.#entries.clear();
      this.#claims.clear();
      this.#refusedAlias.clear();
    }
    return restarted;
  }

  /** 🔢️ The last residency this registry observed, `-1` before the guest published one. */
  get residency(): number {
    return this.#residency;
  }

  holds(url: string, digest: string): boolean {
    return this.#entries.get(url)?.digest === digest;
  }

  /** 🪢️ True when this guest holds that GEOMETRY, PAGED, under some id. Mesh ids are distinct identities
   * to the collision engine, but the bytes behind them are content-addressed — every `dist/mesh/*.glb` in
   * this repo is the same capsule — so a second id whose digest is already resident is announced by
   * `{url, digest}` and aliased guest-side (`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`)
   * instead of being paged again. Only a PAGED entry answers: an entry that is itself an alias proves
   * nothing about what the guest holds, so a chain of aliases can never stand in for the bytes. */
  holdsDigest(digest: string): boolean {
    if (digest.length === 0) return false;
    for (const held of this.#entries.values()) {
      if (held.paged && held.digest === digest) return true;
    }
    return false;
  }

  /** ✅️ Records an upload as this guest's, at the residency it was dispatched under. Called when the
   * run's LAST page goes out, never when it is queued: a queued run confirms nothing. */
  confirm(url: string, digest: string): void {
    this.#entries.set(url, { digest, paged: true });
    this.#refusedAlias.delete(url);
  }

  /** 🪢️ Records an identity carried by {@link holdsDigest} alone — the bytes crossed under a sibling id.
   * Never proof for a third id ({@link holdsDigest} ignores it), and withdrawn the moment the guest says
   * it cannot serve this identity ({@link claimReupload}), which is what stops a guest that refuses an
   * alias from being handed the same alias forever instead of the bytes. */
  alias(url: string, digest: string): void {
    this.#entries.set(url, { digest, paged: false });
  }

  /** 🪢️ Whether this identity may still be announced by digest alone. False once the guest refused an
   * alias for it — the next announcement pages the bytes. */
  mayAlias(url: string): boolean {
    return !this.#refusedAlias.has(url);
  }

  forget(url: string): void {
    this.#entries.delete(url);
  }

  /** 🚚️ Claims one guest-side re-upload request, dropping the stale claim. `false` once this identity
   * has been claimed {@link PUZZLE3D_MESH_REUPLOAD_CLAIMS} times under the live guest instantiation.
   *
   * 🐛️ The gate used to be `residency <= #repaged[url]` — "at most one claim per published residency
   * value" — and `meshResidency` is the guest's own install counter, which EVERY accepted announcement
   * increments (`derive_brush_mesh`/`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`). So the
   * brake was moved by the very traffic it existed to suppress: one standing request that outlived its
   * identity's install re-opened the gate on every unit of progress anywhere in the tab, each claim
   * deleted the paged entry and permanently set {@link mayAlias} false, and the next announcement was
   * therefore a full 72-command page run instead of a one-command adopt. Measured at wasm #58 on the
   * 180-object Nakagin document: `registerBrushMesh` still arriving 8 minutes after the example switch
   * (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5, wave B48 §1.2).
   *
   * A count is a fact the guest's accepted work cannot move, and only a RESTART
   * ({@link observeResidency}) or a deliberate {@link clear} resets it — so the announce is bounded per
   * document per guest, with exactly one retry left for a page run that failed mid-way. */
  claimReupload(url: string): boolean {
    const claimed = this.#claims.get(url) ?? 0;
    if (claimed >= PUZZLE3D_MESH_REUPLOAD_CLAIMS) return false;
    this.#claims.set(url, claimed + 1);
    if (this.#entries.get(url)?.paged === false) this.#refusedAlias.add(url);
    this.#entries.delete(url);
    return true;
  }

  clear(): void {
    this.#entries.clear();
    this.#claims.clear();
    this.#refusedAlias.clear();
    this.#residency = -1;
  }

  get size(): number {
    return this.#entries.size;
  }
}

/** 🥽️ The page's single {@link Puzzle3dBrushMeshRegistry} — one guest per tab, so one registry. */
export const puzzle3dBrushMeshRegistry = new Puzzle3dBrushMeshRegistry();

/** 📏️ Upper bound on the bytes a JSON string costs on the retained wire: ASCII exactly, every other
 * UTF-16 code unit charged as a six-character `\uXXXX` escape, which no JSON encoder exceeds. */
function puzzle3dWireBytes(text: string): number {
  let bytes = 0;
  for (let index = 0; index < text.length; index += 1) bytes += text.charCodeAt(index) < 0x80 ? 1 : 6;
  return bytes;
}

function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  for (let index = 0; index < bytes.length; index += 1) binary += String.fromCharCode(bytes[index]!);
  return btoa(binary);
}

/** 🔗️ The mesh's bytes on the wire and in the digest: little-endian `f32` positions followed by
 * little-endian `u32` indices — the exact layout `decode_brush_mesh_page_values` reassembles. */
function puzzle3dBrushMeshBytes(positions: readonly number[], indices: readonly number[]): Uint8Array {
  const bytes = new Uint8Array((positions.length + indices.length) * 4);
  new Float32Array(bytes.buffer, 0, positions.length).set(positions);
  new Uint32Array(bytes.buffer, positions.length * 4, indices.length).set(indices);
  return bytes;
}

/** #️⃣ The identity one uploaded mesh is keyed by: unkeyed BLAKE3 over {@link puzzle3dBrushMeshBytes},
 * byte-identical to the plugin's own `brush_mesh_digest` (`✏️editor/⏳️precompute/🦀️.rs`), which is what
 * lets the plugin refuse a run whose pages do not reassemble into the mesh the client announced. */
export function puzzle3dBrushMeshDigest(positions: readonly number[], indices: readonly number[]): string {
  return blake3Hex(puzzle3dBrushMeshBytes(positions, indices));
}

/** 📏️ Values this mesh id may put in one page without pushing the JSON envelope past
 * {@link PUZZLE3D_MESH_COMMAND_RAW_BYTES}. Zero means the id alone already exhausts the wire, and the
 * mesh cannot be uploaded at all. */
function puzzle3dBrushMeshPageCapacity(url: string, surfaceId: string, digest: string): number {
  const probe = { surfaceId, url, digest, page: 999_999, pageCount: 999_999, positionsB64: "", indicesB64: "" };
  const budget = PUZZLE3D_MESH_COMMAND_RAW_BYTES - puzzle3dWireBytes(JSON.stringify(["registerBrushMesh", probe])) - PUZZLE3D_MESH_PAGE_PADDING_CHARS;
  return Math.max(0, Math.min(PUZZLE3D_MESH_PAGE_VALUES, Math.floor((budget * 3) / 16)));
}

/** 🥽️ Splits one loaded GLB's collision geometry into the `registerBrushMesh` page run that carries it
 * inside the retained command contract. An empty result means the mesh cannot be uploaded at all — the
 * id leaves no room for a payload on the wire, or the run would be longer than
 * {@link PUZZLE3D_MESH_UPLOAD_MAX_PAGES} — and the caller uploads nothing rather than emitting a command
 * the framework will refuse. */
export function puzzle3dBrushMeshPages(url: string, surfaceId: string, positions: readonly number[], indices: readonly number[]): readonly Puzzle3dBrushMeshPage[] {
  const digest = puzzle3dBrushMeshDigest(positions, indices);
  const capacity = puzzle3dBrushMeshPageCapacity(url, surfaceId, digest);
  const total = positions.length + indices.length;
  if (capacity === 0 || total === 0) return [];
  const pageCount = Math.ceil(total / capacity);
  if (pageCount > PUZZLE3D_MESH_UPLOAD_MAX_PAGES) return [];
  const pages: Puzzle3dBrushMeshPage[] = [];
  for (let page = 0; page < pageCount; page += 1) {
    const start = page * capacity;
    const end = Math.min(total, start + capacity);
    const positionSlice = positions.slice(Math.min(start, positions.length), Math.min(end, positions.length));
    const indexSlice = indices.slice(Math.max(0, start - positions.length), Math.max(0, end - positions.length));
    pages.push({
      url,
      digest,
      page,
      pageCount,
      ...(positionSlice.length > 0 ? { positionsB64: bytesToBase64(new Uint8Array(Float32Array.from(positionSlice).buffer)) } : {}),
      ...(indexSlice.length > 0 ? { indicesB64: bytesToBase64(new Uint8Array(Uint32Array.from(indexSlice).buffer)) } : {}),
    });
  }
  return pages;
}
/** 🚚️ What the drain does with the page it just took off the queue. `adopt` means the run collapsed:
 * a sibling identity already put this exact geometry into the guest, so the remaining pages of THIS
 * identity were dropped and the announcement alone carries it. */
export type Puzzle3dBrushMeshQueueStep =
  | { readonly kind: "page"; readonly page: Puzzle3dBrushMeshPage }
  | { readonly kind: "adopt"; readonly url: string; readonly digest: string }
  | { readonly kind: "idle" };

/** 🚚️ Takes the next unit of work off a brush-mesh page queue, in place. A page whose digest the guest
 * already holds under ANY id collapses its whole remaining run into one `adopt` announcement — this is
 * what keeps a scene of several ids over byte-identical geometry from paging the same 294 912 bytes
 * once per id. Pure: the caller owns the dispatch and the registry. */
export function puzzle3dBrushMeshQueueStep(queue: Puzzle3dBrushMeshPage[], adoptable: (page: Puzzle3dBrushMeshPage) => boolean): Puzzle3dBrushMeshQueueStep {
  const page = queue.shift();
  if (!page) return { kind: "idle" };
  if (!adoptable(page)) return { kind: "page", page };
  for (let index = queue.length - 1; index >= 0; index -= 1) {
    if (queue[index]!.url === page.url) queue.splice(index, 1);
  }
  return { kind: "adopt", url: page.url, digest: page.digest };
}

/** ⏳️ Drains a brush-mesh page queue with BACK PRESSURE: exactly one command is ever outstanding, so a
 * user action that arrives mid-run queues behind one page instead of behind the whole run. `dispatch`
 * must settle on the dispatched command's own guest completion (`ComponentSceneHostProps.onAction`
 * does); `live` false retires the drain without dispatching anything further. A run's LAST page — and a
 * collapsed run's announcement — confirms the identity through `confirm`, never its enqueue. */
export async function drainPuzzle3dBrushMeshQueue(
  queue: Puzzle3dBrushMeshPage[],
  registry: {
    readonly holdsDigest: (digest: string) => boolean;
    readonly mayAlias: (url: string) => boolean;
    readonly alias: (url: string, digest: string) => void;
    readonly confirm: (url: string, digest: string) => void;
  },
  dispatch: (args: Record<string, unknown>) => Promise<void>,
  live: () => boolean,
): Promise<void> {
  while (live()) {
    const step = puzzle3dBrushMeshQueueStep(queue, (page) => registry.mayAlias(page.url) && registry.holdsDigest(page.digest));
    if (step.kind === "idle") return;
    if (step.kind === "adopt") {
      registry.alias(step.url, step.digest);
      await dispatch({ url: step.url, digest: step.digest });
      continue;
    }
    await dispatch(step.page);
    if (step.page.page === step.page.pageCount - 1) registry.confirm(step.page.url, step.page.digest);
  }
}
//#endregion 🥽️Puzzle3dBrushMeshUpload

/** @emoji 🎚️ Whether any measure (including nested group children) declares `id`. */
export function windowMeasureTreeContainsId(measures: readonly WindowMeasure[], id: string): boolean {
  for (const measure of measures) {
    if (measure.id === id) return true;
    if (measure.kind === "group" && windowMeasureTreeContainsId(measure.children, id)) return true;
  }
  return false;
}

/** @emoji 📊️ Probability weights (0–1 simplex sliders) read out as whole-percent labels, not raw fractions. */
function windowMeasureUsesProbabilityReadout(measure: Extract<WindowMeasure, { kind: "slider" }>): boolean {
  const step = measure.step ?? 1;
  return measure.min === 0 && measure.max <= 1 && step < 1;
}

function windowMeasureProbabilityReadout(value: number): string {
  return `${Math.round(value * 100)}%`;
}

/** @emoji 🎚️ Keeps a measure slider live without accumulating stale document actions behind the pointer. */
function WindowMeasureSlider({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "slider" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const dispatchValue = useMemo(
    () => createDirectionalAsyncDispatcher((value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })),
    [measure.onChange, onAction],
  );
  const formatDisplayValue = windowMeasureUsesProbabilityReadout(measure) ? windowMeasureProbabilityReadout : undefined;
  const disabled = measure.disabled === true;
  // 🪣️ A reveal-group measure (e.g. puzzle3d's fill-count slider) must not round-trip through WASM on
  // every drag value — the plugin already rendered every planned piece tagged with its reveal index, so
  // dragging only needs to move a main-thread visibility cutoff. Only the final value round-trips, once,
  // on gesture release.
  const revealGroupId = measure.reveal;

  return (
    <Slider
      id={measure.id}
      data-published-value={String(measure.value)}
      value={[measure.value]}
      min={measure.min}
      max={measure.max}
      ready={measure.ready}
      loading={measure.loading === true}
      waiting={measure.waiting === true}
      step={measure.step}
      disabled={disabled}
      clampToReady={Boolean(revealGroupId)}
      formatDisplayValue={formatDisplayValue}
      onValueChange={(values) => {
        if (disabled) return;
        const value = values[0] ?? measure.value;
        if (revealGroupId) {
          worldRevealCutoffStore.set(revealGroupId, value);
          return;
        }
        dispatchValue(value);
      }}
      onValueCommit={
        revealGroupId
          ? (values) => {
              if (disabled) return;
              const value = values[0] ?? measure.value;
              worldRevealCutoffStore.set(revealGroupId, value);
              onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } });
            }
          : undefined
      }
      onPointerCancel={revealGroupId ? () => worldRevealCutoffStore.set(revealGroupId, measure.value) : undefined}
    />
  );
}

function windowMeasureGroupHeaderSlider(measure: Extract<WindowMeasure, { kind: "group" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode | undefined {
  if (measure.value === undefined || measure.onChange === undefined) return undefined;
  const sliderMeasure: Extract<WindowMeasure, { kind: "slider" }> = {
    kind: "slider",
    id: `${measure.id}.header-slider`,
    label: undefined,
    value: measure.value,
    min: measure.min ?? 0,
    max: measure.max ?? 1,
    step: measure.step,
    ready: measure.ready,
    loading: measure.loading,
    waiting: measure.waiting,
    onChange: measure.onChange,
  };
  return <WindowMeasureSlider measure={sliderMeasure} onAction={onAction} />;
}

function windowMeasureSelectControl(measure: Extract<WindowMeasure, { kind: "select" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureSelect measure={measure} onAction={onAction} />;
}

function windowMeasureToggleControl(measure: Extract<WindowMeasure, { kind: "toggle" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureToggle measure={measure} onAction={onAction} />;
}

function windowMeasureToggleIcon(measure: Extract<WindowMeasure, { kind: "toggle" }>): ReactNode {
  return <Icon icon={measure.iconId as IconName} size={12} />;
}

/**
 * 🌲️ Maps window measures to native panel-tree rows — same chrome as left-corner trees (label left, control right, guide lines).
 * Pre-reverses top-level measures so bottom-anchored `direction="up"` panels read Count at the bottom, Distribution above.
 */
function windowMeasuresToTreeItems(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, reverseForUpPanel = true): TreeDataItem[] {
  const ordered = reverseForUpPanel ? [...measures].reverse() : [...measures];
  const mapMeasure = (measure: WindowMeasure): TreeDataItem => {
    if (measure.kind === "group") {
      return {
        id: measure.id,
        label: measure.label,
        defaultOpen: measure.defaultOpen,
        control: windowMeasureGroupHeaderSlider(measure, onAction),
        items: measure.children.length > 0 ? windowMeasuresToTreeItems(measure.children, onAction, false) : undefined,
      };
    }
    if (measure.kind === "slider") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: <WindowMeasureSlider measure={measure} onAction={onAction} />,
        loading: measure.loading,
        waiting: measure.waiting,
      };
    }
    if (measure.kind === "select") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureSelectControl(measure, onAction),
      };
    }
    return {
      id: measure.id,
      label: measure.label ?? measure.text ?? "",
      icon: windowMeasureToggleIcon(measure),
      control: windowMeasureToggleControl(measure, onAction),
    };
  };
  return ordered.map(mapMeasure);
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  if (measure.kind === "group") {
    const headerSlider = windowMeasureGroupHeaderSlider(measure, onAction);
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen} headerControl={headerSlider}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        {windowMeasureSelectControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        <WindowMeasureSlider measure={measure} onAction={onAction} />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label ?? measure.text ? uiDataLabel(measure.label ?? measure.text ?? "") : undefined} icon={windowMeasureToggleIcon(measure)}>
        {windowMeasureToggleControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree direction={direction}>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

/** @emoji 🪟️ Public window-options tree for measures rails and tests — icon before label, checkbox for toggles. */
export function renderWindowMeasuresTree(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  return windowMeasuresOverlay(measures, onAction, direction);
}

export function SelectionUtilityOptions({ activeUtilityId, windowId, onAction }: { readonly activeUtilityId: string | undefined; readonly windowId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const methodLabel = useLabel("ui.selection.method");
  const modeLabel = useLabel("ui.selection.mode");
  const rectangleLabel = useLabel("ui.selection.rectangle");
  const lassoLabel = useLabel("ui.selection.lasso");
  const selectiveLabel = useLabel("ui.selection.selective");
  const additiveLabel = useLabel("ui.selection.additive");
  const subtractiveLabel = useLabel("ui.selection.subtractive");
  const invertiveLabel = useLabel("ui.selection.invertive");
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";
  // 🐚️ Replaces the old `(globalThis).__selectionMode` + `window` `"semio:selectionOptionsChanged"`
  // broadcast — this shell's own store, so its selection-mode toggle never reconfigures another mounted
  // shell's marquee gestures.
  const selectionStore = useShellScope().selection;

  const [selectionMode, setSelectionMode] = useState<MergeMode>(() => selectionStore.get());

  const handleModeChange = (mode: MergeMode) => {
    selectionStore.set(mode);
    setSelectionMode(mode);
  };

  const handleMethodChange = (method: "rectangle" | "lasso") => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee" },
    });
  };

  return (
    <div className="flex items-center gap-double">
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{methodLabel}</span>
        <ToggleGroup
          kind="single"
          value={selectionMethod}
          onValueChange={(val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          }}
          items={[
            { value: "rectangle", icon: <Icon icon="square-dashed" size="small" />, text: rectangleLabel },
            { value: "lasso", icon: <Icon icon="lasso" size="small" />, text: lassoLabel },
          ]}
        />
      </div>
      <RibbonDivider />
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{modeLabel}</span>
        <ToggleGroup
          kind="single"
          value={selectionMode}
          onValueChange={(val) => {
            if (val === "replace" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          }}
          items={[
            { value: "replace", icon: "mouse-pointer", text: selectiveLabel },
            { value: "additive", icon: "plus", text: additiveLabel },
            { value: "subtractive", icon: "minus", text: subtractiveLabel },
            { value: "invertive", icon: { kind: "emoji", emoji: "🔄" }, text: invertiveLabel },
          ]}
        />
      </div>
    </div>
  );
}

/** @emoji 🪪️ THE DOM identity of one window-measure control: the WINDOW INSTANCE it is rendered for, then the
 * program-authored measure id. Same rule and same separator as `uiNodeDomId` — a window kind's measure tree is
 * authored ONCE for the kind (`world3d_projection_measures` takes a kind-level `id_prefix`, and puzzle3d passes
 * the literal `"puzzle3d"`) and then rendered once per OPEN INSTANCE of that kind, so the authored id alone puts
 * `puzzle3d-measure-projection-orthographic-view` in the document once per pane the moment more than one
 * measures rail is unfolded. Duplicate ids are invalid HTML, they make the control unaddressable through
 * `<label for>`/`aria-labelledby`/automation, and they also collapse the rail's own fold state
 * (`WindowMeasureTreeGroup` keys `useTreeOpenState` on the group id), so unfolding "Parallel" in one pane
 * unfolded it in the other. */
export function windowMeasureDomId(windowId: string, measureId: string): string {
  return `${windowId}/${measureId}`;
}

/** @emoji 🪪️ {@link windowMeasureDomId} applied to a whole authored measure subtree — ids only, every other field
 * (labels, values, `reveal` group, `onChange`) verbatim, so the authored id stays the one integration key the
 * program, `windowMeasureTreeContainsId` and the `activeUtilityId` routing all speak. */
export function qualifyWindowMeasureIds(measures: readonly WindowMeasure[], windowId: string): WindowMeasure[] {
  return measures.map((measure) =>
    measure.kind === "group"
      ? { ...measure, id: windowMeasureDomId(windowId, measure.id), children: qualifyWindowMeasureIds(measure.children, windowId) }
      : { ...measure, id: windowMeasureDomId(windowId, measure.id) },
  );
}

export function windowMeasuresChrome(
  measures: readonly WindowMeasure[] | undefined,
  activeUtilityId: string | undefined,
  windowId: string,
  onAction: (action: ActionDescriptor) => unknown,
): { readonly measures: ReactNode | undefined; readonly utilityOptions: ReactNode | undefined } {
  const partitioned = partitionWindowMeasures(measures ?? [], activeUtilityId);
  const general = qualifyWindowMeasureIds(partitioned.general, windowId);
  const utilityOptions = qualifyWindowMeasureIds(partitioned.utilityOptions, windowId);
  // 🪟️ Stamps this chrome's owning `windowId` onto every measure action, mirroring `tagSetActiveUtilityWindow`
  // for the utility bar — the generic `onAction` dispatch path reads it back out to target the plugin call's
  // `view_state.windowId`, so a grid/LOD/selection toggle only ever mutates ITS OWN window's options.
  const taggedOnAction = (action: ActionDescriptor) => onAction({ ...action, args: { ...(action.args as object | undefined), windowId } });
  return {
    measures: windowMeasuresOverlay(general, taggedOnAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, taggedOnAction, "up"),
  };
}

/** @emoji 🎓️ Whether a utility node tree has a node (leaf or group) with the given id anywhere in it — used
 * to decide if this window's utility bar is the one an introduction step's `Utility` anchor targets. */
export function utilityNodeTreeContainsId(nodes: readonly UtilityNode[], targetId: string): boolean {
  return nodes.some((node) => node.id === targetId || (node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId)));
}

export function utilityBarNode(utilities: readonly UtilityNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void, revealUtilityId?: string | null, utilityOptions?: ReactNode): ReactNode {
  if (!utilities?.length && !utilityOptions) return undefined;
  const categories = groupUtilityNodesByCategory(utilities ?? [], UTILITY_CATEGORIES);
  if (!categories.length && !utilityOptions) return undefined;
  const grouped: UtilityNode[] = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "utilities" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return <UtilityTree id={`ui.utilities.${windowId}`} utilities={grouped} onAction={onAction} direction="up" revealUtilityId={revealUtilityId} utilityOptions={utilityOptions} />;
}

//#region 🧰️WindowActionPane
/**
 * 🎛️ Renders one {@link ActionArgControl} into a STAGED form field — the crucial difference from
 * `renderUiControl` in `ui-interpreter.tsx` is that this dispatches NOTHING globally; `onChange` only
 * writes to the caller's local staged buffer. `value` is the already-resolved effective value
 * (staged ?? default ?? unset).
 */
export function renderStagedArgControl(def: ResolvedActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean, field?: UIDialogFieldBinding): ReactElement {
  const control: ActionArgControl = argControl(def);
  const fieldId = field?.id ?? def.id;
  const labelledBy = field?.labelledBy;
  switch (control.kind) {
    case "text":
      return <Input id={fieldId} aria-labelledby={labelledBy} required={field?.required} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} placeholder={control.placeholder} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "number":
      return (
        <Input
          id={fieldId}
          aria-labelledby={labelledBy}
          required={field?.required}
          type="number"
          className="h-medium w-full min-w-0"
          value={value === undefined || value === null || value === "" ? "" : String(value)}
          min={control.min}
          max={control.max}
          step={control.step}
          disabled={disabled}
          onChange={(event) => onChange(event.target.value === "" ? undefined : Number(event.target.value))}
        />
      );
    case "slider": {
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : control.min;
      const slider = <Slider id={fieldId} aria-labelledby={labelledBy} className="w-full min-w-0" min={control.min} max={control.max} step={control.step ?? 1} value={[numeric]} disabled={disabled} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
      if (!control.unit) return slider;
      return (
        <div className="flex w-full min-w-0 items-center gap-single">
          {slider}
          <span className="shrink-0 text-xs tabular-nums text-muted-foreground">
            {numeric} {control.unit}
          </span>
        </div>
      );
    }
    case "toggle":
      return <Toggle id={fieldId} aria-labelledby={labelledBy} icon="check" pressed={value === true} text={uiDataLabel(def.label)} disabled={disabled} onPressedChange={(pressed) => onChange(pressed)} />;
    case "artifactKind":
    case "surfaceApp":
    case "select": {
      if (def.schema.kind !== "string") throw new Error("Select control requires a string argument schema");
      return (
        <Select id={def.id} value={typeof value === "string" && actionArgStringOptions(def.schema).some(option => option.value === value) ? value : ""} disabled={disabled || actionArgStringOptions(def.schema).length === 0} onValueChange={(next) => onChange(next)}>
          <SelectTrigger id={fieldId} aria-labelledby={labelledBy} aria-required={field?.required} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={def.label} />
          </SelectTrigger>
          <SelectContent>
            {actionArgStringOptions(def.schema).map((option, index) => (
              <SelectItem key={`${def.id}:${index}:${option.value}`} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    }
    case "vec3": {
      const tuple = Array.isArray(value) && value.length >= 3 ? (value as readonly number[]) : null;
      const axes = ["x", "y", "z"] as const;
      return (
        <div className="grid grid-cols-3 gap-single">
          {axes.map((axis, index) => (
            <Input
              key={`${def.id}.${axis}`}
              id={`${fieldId}.${axis}`}
              aria-label={`${uiDataLabel(def.label)} ${axis}`}
              required={field?.required}
              type="number"
              className="h-medium w-full min-w-0"
              value={tuple ? String(tuple[index] ?? 0) : ""}
              placeholder={axis}
              disabled={disabled}
              onChange={(event) => {
                const parsed = Number(event.target.value);
                if (!Number.isFinite(parsed)) return;
                const next: [number, number, number] = tuple ? [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0] : [0, 0, 0];
                next[index] = parsed;
                onChange(next);
              }}
            />
          ))}
        </div>
      );
    }
    case "iconSelect":
      return <IconSelector id={fieldId} aria-labelledby={labelledBy} disabled={disabled} classifyIconSelectorMode={undefined} value={typeof value === "string" ? value : ""} uniform onChange={(next) => onChange(next)} />;
  }
}

/** 🧰️ True when an action carries arguments and therefore stages a form instead of firing immediately (P1–P4). */
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args?.length ?? 0) > 0;
}

/** 🧰️ The decision a bound hotkey makes for one action (P4). */
/** ⌨️ True when a keydown's target is a text-editing surface (input/textarea/select/contenteditable) — hotkeys never fire while the user is typing. */
export function isEditableEventTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

/** 🩹️ Structural, not `React.KeyboardEvent`/`globalThis.KeyboardEvent` specifically —
 * `keyboardEventMatchesChord` only ever reads the five fields below, which both shapes carry
 * identically, and its real callers legitimately hold either (a native `useShellKeydown` listener vs a
 * React `onKeyDown` handler). */
export type KeyboardEventLike = { readonly key: string; readonly ctrlKey: boolean; readonly metaKey: boolean; readonly shiftKey: boolean; readonly altKey: boolean };

/** ⌨️ True when a keydown event matches one `+`-joined chord (e.g. `"mod+shift+z"`), where `mod` accepts either ctrl or meta. */
export function keyboardEventMatchesChord(event: KeyboardEventLike, chord: string): boolean {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  const hasCtrl = event.ctrlKey || event.metaKey;
  if (needsCtrl !== hasCtrl) return false;
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key;
}

export type KeybindingIntent = { readonly kind: "fire" } | { readonly kind: "open"; readonly actionId: string } | { readonly kind: "execute"; readonly actionId: string; readonly args: Record<string, unknown> };

/**
 * ✍️ Pure P4 decision: an arg-less action fires directly; an arg-carrying action opens its staged form,
 * unless that form is already the expanded one in the active window AND validation passes, in which case
 * the hotkey executes with the merged effective args. An already-open-but-invalid form stays open.
 */
export function resolveKeybindingIntent(definition: Pick<ActionDefinition, "id" | "args"> | undefined, expandedActionId: string | null, stagedArgs: Readonly<Record<string, unknown>>): KeybindingIntent {
  if (!definition || !actionRequiresStagedForm(definition)) return { kind: "fire" };
  if (expandedActionId === definition.id) {
    const effective = effectiveActionArgs(definition.args, stagedArgs);
    if (unresolvedActionArgs(definition.args, effective).length === 0) return { kind: "execute", actionId: definition.id, args: effective };
  }
  return { kind: "open", actionId: definition.id };
}

/** 📋 Fragment carried by a guest `clipboardWrite` host effect, or undefined when the effect is not that variant. */
export function clipboardWriteFragmentFromEffect(effect: unknown): unknown | undefined {
  if (effect === null || typeof effect !== "object" || !("clipboardWrite" in effect)) return undefined;
  const write = (effect as { clipboardWrite?: { fragment?: unknown } }).clipboardWrite;
  return write?.fragment;
}

export type ClipboardActionDescriptor = { readonly action: string; readonly args?: unknown };

/** 📋 Named `fragment` on a paste descriptor, when the args object carries one. */
export function pasteArgsFragment(action: { readonly args?: unknown }): unknown | undefined {
  const args = action.args;
  if (args === undefined || args === null || typeof args !== "object" || Array.isArray(args)) return undefined;
  if (!("fragment" in args)) return undefined;
  return (args as { fragment?: unknown }).fragment;
}

/** 📋 Injects a host-retained copy fragment onto paste when `args.fragment` is missing. */
export function pasteActionWithRetainedFragment<T extends ClipboardActionDescriptor>(action: T, fragment: unknown | undefined): T {
  if (action.action !== "paste" || fragment === undefined) return action;
  if (pasteArgsFragment(action) !== undefined) return action;
  const args = action.args;
  const base = args !== undefined && typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
  return { ...action, args: { ...base, fragment } };
}

/** 🧰️ Pure P5 activation decision: an empty request, or re-requesting the already-active utility, deactivates (null); otherwise the requested utility becomes active. */
export function resolveUtilityActivation(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}

/** 🗂️ Category id for one action: declared category, else `"history"` for history actions, else `"actions"` (mirrors the command-palette fallback at {@link resolveCommands}'s sibling `searchItems` builder). */
export function actionCategoryId(action: Pick<ActionDefinition, "category" | "kind">): string {
  return action.category ?? (action.kind === "history" ? "history" : "actions");
}

/** 🗂️ Resolves an action category's display label: the app's own group-label overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids ({@link ribbonParentLabel}), else the raw id (mirrors {@link resolveUtilityGroupLabel}). */
function actionCategoryLabel(category: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  return resolveAppLabel(appLabelsOverlay, "group", category, ribbonParentLabel(category) ?? category);
}

/** 🗂️ Ordered, deduped categories from resolved actions (sibling of {@link commandCategories}). */
export function actionCategories(actions: readonly ActionDefinition[], appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const action of actions) {
    const id = actionCategoryId(action);
    if (seen.has(id)) continue;
    seen.add(id);
    categories.push({ id, label: actionCategoryLabel(id, appLabelsOverlay) });
  }
  return categories;
}

/**
 * 🎛️ Category sections of one window's Actions rail (Tree twin of {@link buildCommandCategoryTree}):
 * one section per category, zero-arg actions fire directly, arg-carrying actions toggle a sibling form
 * section — exactly {@link buildCommandCategoryTree}'s list/form split, localized per category so
 * multiple categories can render side by side. Only one action (across all categories) is expanded at a
 * time, per `expandedActionId`.
 */
export function buildActionCategoryTree(
  windowId: string,
  controllerId: string,
  actions: readonly ResolvedActionDefinition[],
  expandedActionId: string | null,
  stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  disabled: boolean,
  onExpandedChange: (actionId: string | null) => void,
  onStageArg: (actionId: string, argId: string, value: unknown) => void,
  onResetArgs: (actionId: string) => void,
  onExecute: (descriptor: ActionDescriptor) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): TreeDataSection[] {
  const categories = actionCategories(actions, appLabelsOverlay);
  const expandedAction = expandedActionId ? actions.find((action) => action.id === expandedActionId) : undefined;
  const sections: TreeDataSection[] = [];
  for (const category of categories) {
    const categoryActions = actions.filter((action) => actionCategoryId(action) === category.id);
    sections.push({
      id: `action.category.${category.id}`,
      label: category.label,
      defaultOpen: true,
      items: categoryActions.map((action): TreeDataItem => {
        const icon = action.iconId ? <Icon icon={action.iconId as IconName} size="small" /> : undefined;
        const rowClassName = disabled ? "pointer-events-none opacity-50" : undefined;
        if (!actionRequiresStagedForm(action)) {
          return { id: `action.${action.id}`, label: action.label, icon, className: rowClassName, onClick: () => !disabled && onExecute({ controllerId, action: action.id }) };
        }
        const expanded = expandedActionId === action.id;
        return {
          id: `action.${action.id}`,
          label: `${action.label}…`,
          icon: icon ?? <Icon icon={expanded ? "chevron-down" : "chevron-right"} size="small" />,
          className: rowClassName,
          onClick: () => !disabled && onExpandedChange(expanded ? null : action.id),
        };
      }),
    });
    if (expandedAction && actionCategoryId(expandedAction) === category.id) {
      const staged = stagedArgsByKey[actionStageKey(windowId, expandedAction.id)] ?? {};
      const effective = effectiveActionArgs(expandedAction.args, staged);
      const missing = unresolvedActionArgs(expandedAction.args, effective);
      sections.push({
        id: `action.category.${category.id}.form`,
        defaultOpen: true,
        items: expandedAction.args.map(
          (def): TreeDataItem => ({
            id: `action.${expandedAction.id}.arg.${def.id}`,
            label: def.label,
            description: def.description,
            control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedAction.id, def.id, value), disabled),
          }),
        ),
        actions: [
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "execute"),
            icon: <Icon icon="check" size="small" />,
            text: shellLabel("ui.common.execute"),
            disabled: disabled || missing.length > 0,
            onClick: () => { if (!disabled && missing.length === 0) onExecute({ controllerId, action: expandedAction.id, args: effective }); },
          },
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "reset"),
            icon: <Icon icon="undo" size="small" />,
            text: shellLabel("ui.common.reset"),
            disabled,
            onClick: () => onResetArgs(expandedAction.id),
          },
        ],
      });
    }
  }
  return sections;
}

/** 🎛️ Props for the per-window Action rail body (P1/P2). */
export type WindowActionPaneProps = {
  readonly windowId: string;
  readonly controllerId: string;
  readonly actions: readonly ResolvedActionDefinition[];
  readonly expandedActionId: string | null;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly disabled: boolean;
  readonly onExpandedChange: (actionId: string | null) => void;
  readonly onStageArg: (actionId: string, argId: string, value: unknown) => void;
  readonly onResetArgs: (actionId: string) => void;
  readonly onExecute: (descriptor: ActionDescriptor) => void;
  readonly appLabelsOverlay?: PluginAppLabelsOverlay;
};

/**
 * 🎛️ The per-window Actions rail body (P1/P2), grouped into categories like the command panel. Zero-arg
 * actions fire directly; arg-carrying actions expand a locally-buffered staged form (same inline
 * property-row controls as utility measures) — nothing dispatches on edit, effective value is
 * `staged ?? default ?? unset`, Execute is enabled only when every required arg has an effective value,
 * fires exactly ONE `ActionDescriptor` with the merged args, and keeps the staged values afterward.
 * When `disabled` (an active utility with `allowsActionsWhileActive === false`), every row renders disabled.
 */
export function WindowActionPane(props: WindowActionPaneProps): ReactElement {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay } = props;
  const sections = buildActionCategoryTree(windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay);
  return (
    <div data-slot="window-action-pane" className="flex min-w-0 flex-col">
      <Tree sections={sections} showLines={false} sortableSections={false} />
    </div>
  );
}

/** 🧰️ Slice of the {@link ActionPaneState} the {@link windowActionPaneNode} builder reads. */
export type ActionPaneSlice = Pick<ActionPaneState, "expandedByWindowId" | "stagedArgsByKey" | "activeUtilityByWindowId">;

/**
 * 🧰️ Sibling of {@link utilityBarNode}: resolves a window kind's panel-eligible actions and returns a
 * bound {@link WindowActionPane}, or `undefined` when the window has no panel-eligible action (so the rail
 * chip never renders). Rows render disabled while an active utility gates actions
 * (`allowsActionsWhileActive === false`).
 *
 * 🧹️ Panel-eligible means `inPalette` — the SAME curation `resolveCommands` gives the palette and
 * `buildShellContextMenuItems` gives the shell fallback menu: an app declares its raw dispatch verbs
 * (`worldPointerDown`, `registerBrushMesh`, `suggestionsTick`, …) and the framework declares its reserved
 * ones (`interactionSelect`, `noteShellCommand`, `setActiveUtility`, …) as window actions purely so a
 * surface can dispatch them, and a rail that renders them buries the user's own verbs: the puzzle3d
 * perspective rail carried 96 rows and put `Export` at y=1990 inside an 807 px band, which is what
 * `export-only` could not press (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §3).
 */
export function windowActionPaneNode(
  app: AppDefinition,
  windowKind: AppWindowKindDefinition,
  windowId: string,
  actionPane: ActionPaneSlice,
  onAction: (action: ActionDescriptor) => void,
  dispatch: (action: ShellAction) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  manifests: readonly { readonly apps: readonly unknown[] }[] = [],
  selectedArtifactKinds?: readonly ArtifactKindChoice[],
): ReactNode {
  const resolvedActions = resolveWindowActions(app, windowKind).filter((action) => action.inPalette);
  if (resolvedActions.length === 0) return undefined;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, terminology, locale)),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay, terminology, locale, manifests, selectedArtifactKinds)),
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : undefined;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return (
    <WindowActionPane
      windowId={windowId}
      controllerId={app.controllerId}
      actions={actions}
      expandedActionId={actionPane.expandedByWindowId[windowId] ?? null}
      stagedArgsByKey={actionPane.stagedArgsByKey}
      disabled={disabled}
      onExpandedChange={(actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId })}
      onStageArg={(actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value })}
      onResetArgs={(actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId })}
      onExecute={onAction}
      appLabelsOverlay={appLabelsOverlay}
    />
  );
}
//#endregion 🧰️WindowActionPane

//#region 🎛️CommandRegistry
/** 🎛️ One command definition paired with its shared, fully qualified address. */
export type ResolvedCommand = {
  readonly definition: Omit<CommandDefinition, "label" | "args"> & { readonly label: string; readonly args: ResolvedActionArgDef[] };
  readonly address: CommandAddress;
};

/** 📍️ Stable key for maps whose command ids may overlap across owners. */
export function commandAddressKey(address: CommandAddress): string {
  const owner = address.owner;
  if (owner === "os") return `os:${address.commandId}`;
  if ("plugin" in owner) return `plugin:${owner.plugin.pluginId}:${address.commandId}`;
  if ("app" in owner) return `app:${owner.app.pluginId}:${owner.app.appId}:${address.commandId}`;
  return `mode:${owner.mode.pluginId}:${owner.mode.appId}:${owner.mode.modeId}:${address.commandId}`;
}

/** 🪪️ Element-id-safe projection of the complete command address. */
function commandElementKey(address: CommandAddress): string {
  return commandAddressKey(address).replaceAll(":", ".");
}

/** 📍️ Whether an addressed command belongs to the operating-system catalog. */
export function isOsCommandAddress(address: CommandAddress): boolean {
  return address.owner === "os";
}

/** 📍️ Plugin segment of a non-OS command owner. */
export function commandOwnerPluginId(owner: CommandAddress["owner"]): string | null {
  if (owner === "os") return null;
  if ("plugin" in owner) return owner.plugin.pluginId;
  if ("app" in owner) return owner.app.pluginId;
  return owner.mode.pluginId;
}

/** ⌨️ Detects the command-keybinding platform from a browser/host platform description. */
export function detectCommandPlatform(value: string): Platform | undefined {
  if (/mac|iphone|ipad/i.test(value)) return "macOs";
  if (/win/i.test(value)) return "windows";
  if (/linux|x11/i.test(value)) return "linux";
  return undefined;
}

/** ⌨️ Resolves a command's portable chords for one host platform. */
export function commandKeybindingChords(definition: Pick<CommandDefinition, "keybindings">, platform?: Platform): string[] {
  return definition.keybindings.filter((binding) => binding.platform === undefined || binding.platform === platform).map((binding) => binding.chord);
}

/**
 * 🎛️ Aggregates every command visible for the current session: os built-ins, the active session's
 * plugin-owned commands, app-owned commands, and commands owned by the active mode. Window-local
 * verbs are actions and therefore never enter this global command resolver — unlike
 * `resolveWindowActions`/`resolveUtilities`, this never takes a window kind.
 */
export function resolveCommands(
  osCommands: readonly CommandDefinition[],
  activePluginManifest: Pick<PluginManifest, "pluginId" | "commands"> | null | undefined,
  app: Pick<AppDefinition, "id" | "commands" | "modes"> | null | undefined,
  activeModeId: string,
  overlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  manifests: readonly { readonly apps: readonly unknown[] }[] = [],
  selectedArtifactKinds?: readonly ArtifactKindChoice[],
): ResolvedCommand[] {
  // 🗺️ `CommandDefinition.label`/`.args[].label` are manifest `LocalizedLabel` fields — there is no
  // "command"/"commandArg" overlay category (commands never went through `AppLabelsOverlay`), so this is
  // the single choke point that resolves them to plain strings for every downstream consumer (the
  // footer command panel, the command palette, `noteShellCommand`'s history label); `osCommands` are
  // already plain strings (built by `buildOsCommands` via `shellLabel`) and pass through unchanged.
  const resolveDefinition = (definition: CommandDefinition): ResolvedCommand["definition"] => ({
    ...definition,
    label: resolveManifestLabel(definition.label, terminology, locale),
    args: definition.args.map((def) => resolveActionArgDef(def, definition.id, overlay, terminology, locale, manifests, selectedArtifactKinds)),
  });
  const resolved: ResolvedCommand[] = osCommands.map((definition) => ({ definition: resolveDefinition(definition), address: { owner: "os", commandId: definition.id } }));
  for (const definition of activePluginManifest?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { plugin: { pluginId: activePluginManifest!.pluginId } }, commandId: definition.id } });
  }
  if (!app) return resolved;
  const pluginId = activePluginManifest?.pluginId ?? "";
  for (const definition of app.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { app: { pluginId, appId: app.id } }, commandId: definition.id } });
  }
  const activeMode = (app.modes as readonly AppModeDefinition[] | undefined)?.find((mode) => mode.id === activeModeId);
  for (const definition of activeMode?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { mode: { pluginId, appId: app.id, modeId: activeModeId } }, commandId: definition.id } });
  }
  return resolved;
}

/** 🎛️ Chrome-known command category ids that already have a `ui.settings.tab.*` translation key. */
const CHROME_KNOWN_COMMAND_CATEGORIES = new Set(["general", "driver", "app", "appearance", "layout", "language", "terminology", "theme"]);

/** 🎛️ Loose title-case for an open-set command category id (e.g. "appearance" -> "Appearance"). Falls back to this for app/plugin-invented categories that have no fixed framework vocabulary entry. */
function titleizeCommandCategory(category: string): string {
  return category.replace(/[-_]+/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}

/** 🎛️ Resolves a command category's display label, reusing the existing `ui.settings.tab.*` keys for chrome-known ids and falling back to a loose title-case for open-set app/plugin categories. */
export function commandCategoryLabel(category: string): string {
  return CHROME_KNOWN_COMMAND_CATEGORIES.has(category) ? shellLabel(`ui.settings.tab.${category as "general" | "driver" | "app" | "appearance" | "layout" | "language" | "terminology" | "theme"}`) : titleizeCommandCategory(category);
}

/** 🎛️ Ordered, deduped category tabs for the footer command panel, derived from whatever commands actually resolved. */
export function commandCategories(commands: readonly ResolvedCommand[]): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const { definition } of commands) {
    if (!definition.inPalette) continue;
    if (seen.has(definition.category)) continue;
    seen.add(definition.category);
    categories.push({ id: definition.category, label: commandCategoryLabel(definition.category) });
  }
  return categories;
}

function selectCommandArg(id: string, label: string, options: readonly { readonly value: string; readonly label: string }[]): ActionArgDef {
  return { id, label, schema: { kind: "string", options: options.map((option) => ({ ...option })) }, required: true };
}

/** @emoji 🚗️ Translated display name for a built-in driver id; a custom (user-authored) driver has no
 * translation key, so its own {@link UiDriver.label} (genuine runtime data) is the correct fallback. */
export function driverDisplayLabel(driver: UiDriver): string {
  if (driver.id === "default") return shellLabel("settings.driver.default");
  if (driver.id === "compact") return shellLabel("settings.driver.compact");
  return driver.label || driver.id;
}

/**
 * 🎛️ Os-owned built-in commands — app introduction/theme/layout/locale/appearance/driver,
 * handled locally by the shell (never routed to a program). Rebuilt via `useMemo` since the theme and
 * terminology option lists are live state.
 */
export function buildOsCommands(
  themeList: readonly UiTheme[],
  terminologies: readonly string[],
  hasIntroduction: boolean,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
  driverList: readonly UiDriver[] = builtinUiDrivers(),
  tutorials: readonly { readonly id: string; readonly title: unknown }[] = [],
  tutorialRecorderAvailable = false,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  /** 👁️✏️ Whether the active session's dialect has at least one `AppRouter` entry for either role —
   * gates `open-artifact-with-viewer`/`open-artifact-with-editor` (contract freeze §5) the same way
   * `hasIntroduction`/`tutorialRecorderAvailable` gate their own optional commands above. */
  hasOpenArtifactSurfaces = false,
): CommandDefinition[] {
  const lockedCommandIds = new Set<string>([...(locks.appearance ? ["os.setAppearance"] : []), ...(locks.themeId ? ["os.setThemeId"] : []), ...(locks.locale ? ["os.setLocale"] : []), ...(locks.terminology ? ["os.setTerminology"] : [])]);
  const commands: CommandDefinition[] = [
    {
      id: "os.toggleFullscreen",
      label: shellLabel("ui.fullscreen.toggle"),
      category: "layout",
      iconId: "maximize-2" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      inPalette: true,
      args: [],
      keybindings: [
        { chord: "f11", platform: "windows" },
        { chord: "f11", platform: "linux" },
        { chord: "control+meta+f", platform: "macOs" },
      ],
    },
    ...(hasIntroduction ? [{ id: "os.introduceApp", label: shellLabel("ui.command.introduceApp"), category: "app", iconId: "graduation-cap" as const, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] }] : []),
    // 🎥️ `os.playTutorial` only appears once at least one tutorial is declared (app-own or brand-own);
    // `os.recordTutorial` is dev/studio-only (see `isTutorialRecorderAvailable`) and needs no declared
    // tutorial at all — recording an app IS the authoring path for one.
    ...(tutorials.length > 0
      ? [{ id: "os.playTutorial", label: shellLabel("ui.command.playTutorial"), category: "app", iconId: "play" as const, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, keybindings: [], args: [selectCommandArg("tutorialId", shellLabel("tutorial.chapter"), tutorials.map((tutorial) => ({ value: tutorial.id, label: resolveManifestLabel(tutorial.title, terminology, locale) })))] }]
      : []),
    ...(tutorialRecorderAvailable ? [{ id: "os.recordTutorial", label: shellLabel("ui.command.recordTutorial"), category: "app", iconId: "circle" as const, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] }] : []),
    {
      id: "os.setAppearance",
      label: shellLabel("ui.command.setAppearance"),
      category: "appearance",
      iconId: "sun" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("appearance", shellLabel("ui.settings.tab.appearance"), [
          { value: "system", label: shellLabel("ui.settings.appearance.system") },
          { value: "light", label: shellLabel("ui.settings.appearance.light") },
          { value: "dark", label: shellLabel("ui.settings.appearance.dark") },
        ]),
      ],
    },
    {
      id: "os.setThemeId",
      label: shellLabel("ui.command.setTheme"),
      category: "appearance",
      iconId: "palette" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "themeId",
          shellLabel("ui.settings.tab.theme"),
          themeList.map((theme) => ({ value: theme.id, label: theme.label || theme.id })),
        ),
      ],
    },
    {
      id: "os.setLayout",
      label: shellLabel("ui.command.setLayout"),
      category: "layout",
      iconId: "layout" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("layout", shellLabel("ui.settings.tab.layout"), [
          { value: "desktop", label: shellLabel("settings.layout.desktop") },
          { value: "tablet", label: shellLabel("settings.layout.tablet") },
        ]),
      ],
    },
    { id: "os.resetDock", label: shellLabel("ui.settings.resetDock"), category: "layout", iconId: "undo" as const, semantics: actionSemanticsForKind("shell"), kind: "shell", inPalette: true, args: [], keybindings: [] },
    {
      id: "os.setLocale",
      label: shellLabel("ui.command.setLocale"),
      category: "language",
      iconId: "globe" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("locale", shellLabel("ui.settings.tab.language"), [
          { value: "en", label: shellLabel("ui.settings.language.en") },
          { value: "de", label: shellLabel("ui.settings.language.de") },
        ]),
      ],
    },
    {
      id: "os.setTerminology",
      label: shellLabel("ui.command.setTerminology"),
      category: "language",
      iconId: "type" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "terminology",
          shellLabel("ui.settings.tab.terminology"),
          terminologies.map((id) => ({ value: id, label: shellTerminologyLabel(id) })),
        ),
      ],
    },
    {
      id: "os.setDriver",
      label: shellLabel("ui.command.setDriver"),
      category: "layout",
      iconId: "settings" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "driver",
          shellLabel("ui.settings.tab.driver"),
          driverList.map((driver) => ({ value: driver.id, label: driverDisplayLabel(driver) })),
        ),
      ],
    },
    // 👁️✏️ Both share the frozen "Open with…" label (contract freeze §5) — the role is which picker
    // group they focus, not a different label; `dispatchOsCommand` opens the Document panel's
    // "Open with…" section pre-scoped to that role rather than sending a wire command itself.
    ...(hasOpenArtifactSurfaces
      ? [
          { id: OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID, label: openArtifactWithText(locale), category: "artifact", iconId: "eye" as IconName, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] },
          { id: OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID, label: openArtifactWithText(locale), category: "artifact", iconId: "pencil" as IconName, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] },
        ]
      : []),
  ];
  return commands.filter((command) => !lockedCommandIds.has(command.id));
}

/** 🎛️ Os-scope command ids that are handled locally by the shell — mirrors {@link buildOsCommands}. */
export function dispatchOsCommand(
  commandId: string,
  args: Record<string, unknown> | undefined,
  commitUiPreference: (mutation: UiPreferencesConfigMutation) => void,
  dispatch: (action: ShellAction) => void,
  dockLayoutStore: DockLayoutStore,
  dockUiStateStore: DockUiStateStore,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
): void {
  switch (commandId) {
    case "os.introduceApp":
      dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
      return;
    case "os.setAppearance":
      if (locks.appearance) return;
      commitUiPreference(setAppearance((args?.appearance as ElementsSurfaceAppearance) ?? "system"));
      return;
    case "os.setThemeId":
      if (locks.themeId) return;
      if (typeof args?.themeId === "string") commitUiPreference(setTheme(args.themeId));
      return;
    case "os.setLayout":
      commitUiPreference(setLayout((args?.layout as UiChromeLayout) ?? "desktop"));
      return;
    case "os.resetDock":
      dispatch({ type: "RESET_DOCK" });
      dockLayoutStore.reset();
      dockUiStateStore.reset();
      return;
    case "os.setLocale":
      if (locks.locale) return;
      if (typeof args?.locale === "string") commitUiPreference(setLocale(args.locale as UiLocale));
      return;
    case "os.setTerminology":
      if (locks.terminology) return;
      if (typeof args?.terminology === "string") commitUiPreference(setTerminology(args.terminology));
      return;
    case "os.setDriver":
      if (typeof args?.driver === "string") commitUiPreference(setDriver(args.driver));
      return;
    // 👁️✏️ Neither sends a wire command itself (contract freeze §5) — both just focus the Document
    // panel's "Open with…" section, pre-expanded to the role the palette entry named.
    case OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID:
    case OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID:
      dispatch({ type: "SET_OPEN_WITH_FOCUS_ROLE", value: commandId === OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID ? "viewer" : "editor" });
      dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [FRAMEWORK_PANEL_TAB_ARTIFACT_ID] });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
      return;
    default:
      return;
  }
}

/** @emoji 🎛️ Fallback icon for every command-category leaf — categories are open-set strings any plugin/app/mode author can invent, so there's no per-category icon metadata to key off (unlike the framework's own Workbench/Details/Display/Settings categories). */
const COMMAND_CATEGORY_ICON = shellTabIcon("wrench");

/**
 * 🎛️ One category's command list (and, if a command is expanded, its staged arg form) as a `TreePanelConfig`
 * — the content a category `PanelTabLeaf` resolves to. A zero-arg command's row fires immediately on click
 * (a plain fire-and-forget tree row, same pattern as {@link groupNamedLayoutsToTreeItems}'s layout rows —
 * no `selectedIds`/`onSelectionChange` override, so it takes `Tree`'s default single-select highlight after
 * firing, same as clicking a Display→Layout row does). An arg-carrying command's row toggles `expandedCommandId`
 * itself (kept as its own exclusive, bespoke state — not `Tree`'s per-row `openStates`, which isn't naturally
 * exclusive across sibling rows) and, when expanded, a synthetic form section (one row per arg, `control`
 * holding the staged input, replacing the old `Field` wrapper since `TreeDataItem` already renders label +
 * description + control in the same two-column layout) is prepended so it renders above the command list —
 * `Tree` reverses top-level `sections` for `direction="up"` (bottom anchors), threaded here via `flowFromAnchor`/
 * `FlowProvider`/`useFlow` down from the hosting `Panel`, not any manual reversal in this function.
 */
export function buildCommandCategoryTree(
  commands: readonly ResolvedCommand[],
  expandedCommandId: string | null,
  stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  onExecute: (entry: ResolvedCommand, executeArgs?: Record<string, unknown>) => void,
  onToggleExpanded: (commandId: string | null) => void,
  onStageArg: (commandId: string, argId: string, value: unknown) => void,
  onResetArgs: (commandId: string) => void,
): TreePanelConfig {
  const argCarryingCommands = commands.filter((entry) => entry.definition.args.length > 0);
  const autoExpandedSingleton = argCarryingCommands.length === 1 ? argCarryingCommands[0] : undefined;
  const expanded = (expandedCommandId ? commands.find((entry) => commandAddressKey(entry.address) === expandedCommandId) : undefined) ?? autoExpandedSingleton;
  const effectiveExpandedId = expanded ? commandAddressKey(expanded.address) : null;
  const sections: TreeDataSection[] = [];
  if (expanded && expanded.definition.args.length > 0) {
    const expandedKey = commandAddressKey(expanded.address);
    const expandedElementKey = commandElementKey(expanded.address);
    const staged = stagedArgsByCommandId[expandedKey] ?? {};
    const effective = effectiveActionArgs(expanded.definition.args, staged);
    const missing = unresolvedActionArgs(expanded.definition.args, effective);
    sections.push({
      id: `command.category.${expanded.definition.category}.form`,
      items: expanded.definition.args.map(
        (def): TreeDataItem => ({
          id: `command.${expandedElementKey}.arg.${def.id}`,
          label: def.label,
          description: def.description,
          control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedKey, def.id, value)),
        }),
      ),
      actions: [
        {
          id: `command-${expandedElementKey}-execute`,
          icon: <Icon icon="check" size="small" />,
          text: shellLabel("ui.common.execute"),
          disabled: missing.length > 0,
          onClick: () => { if (missing.length === 0) onExecute(expanded, effective); },
        },
        {
          id: `command-${expandedElementKey}-reset`,
          icon: <Icon icon="undo" size="small" />,
          text: shellLabel("ui.common.reset"),
          onClick: () => onResetArgs(expandedKey),
        },
      ],
    });
  }
  const listCommands = commands.filter((entry) => commandAddressKey(entry.address) !== effectiveExpandedId);
  if (listCommands.length > 0) {
    sections.push({
      id: "command.category.list",
      items: listCommands.map((entry): TreeDataItem => {
        const argCarrying = entry.definition.args.length > 0;
        const entryKey = commandAddressKey(entry.address);
        const elementKey = commandElementKey(entry.address);
        const icon = entry.definition.iconId ? <Icon icon={entry.definition.iconId as IconName} size="small" /> : undefined;
        if (!argCarrying) return { id: `command.${elementKey}`, label: entry.definition.label, icon, onClick: () => onExecute(entry) };
        return {
          id: `command.${elementKey}`,
          label: `${entry.definition.label}…`,
          icon: <Icon icon={expandedCommandId === entryKey ? "chevron-down" : "chevron-up"} size="small" />,
          onClick: () => onToggleExpanded(expandedCommandId === entryKey ? null : entryKey),
        };
      }),
    });
  }
  return { sections };
}

/**
 * 🎛️ One `PanelTabLeaf` per resolved command category — consumers wrap these under the Command branch
 * (`FRAMEWORK_CATEGORY_COMMAND_ID`) on `defaultDock.anchors["bottom-middle"]` so the folded chrome shows
 * a single expandable Command toggle. The command palette's fold/active-category/size/persistence is the
 * generic per-anchor `Panel` state (see `buildPanelProps`); this only builds the category tab leaves.
 * Content is a *lazy* `resolveTree` (mirrors {@link createFrameworkDisplayPanelTabs}'s windows tab) so
 * this array — and therefore `defaultDock`'s own memo — never depends on `expandedCommandId`/
 * `stagedArgsByCommandId`, which change on every keystroke while staging a command argument; `resolveTree`
 * reads those fresh off refs at render time instead.
 */
export function buildCommandCategoryTabs(
  resolvedCommands: readonly ResolvedCommand[],
  categories: readonly { readonly id: string; readonly label: string }[],
  expandedCommandIdRef: React.RefObject<string | null>,
  stagedArgsByCommandIdRef: React.RefObject<Readonly<Record<string, Readonly<Record<string, unknown>>>>>,
  onCommand: (address: CommandAddress, args?: Record<string, unknown>) => void,
  dispatch: (action: ShellAction) => void,
): PanelTabNode[] {
  return categories.map((category) => {
    const categoryCommands = resolvedCommands.filter((entry) => entry.definition.inPalette && entry.definition.category === category.id);
    return singleTreeLeaf({
      id: `command.category.${category.id}`,
      icon: COMMAND_CATEGORY_ICON,
      name: category.label,
      tree: {
        resolveTree: () =>
          buildCommandCategoryTree(
            categoryCommands,
            expandedCommandIdRef.current,
            stagedArgsByCommandIdRef.current,
            (entry, executeArgs) => onCommand(entry.address, executeArgs),
            (commandId) => dispatch({ type: "SET_COMMAND_EXPANDED", value: commandId }),
            (commandId, argId, value) => dispatch({ type: "STAGE_COMMAND_ARG", commandId, argId, value }),
            (commandId) => dispatch({ type: "RESET_COMMAND_ARGS", commandId }),
          ),
      },
    });
  });
}
//#endregion 🎛️CommandRegistry

//#region 🛠️ToolRegistry
/**
 * 🛠️ One tool's measure-tree content — a single headerless section mapped to native `TreeDataItem`s, so
 * Fill opens directly onto count + distribution with the same chrome as left-corner panel trees. The
 * `tool.<id>` leaf tab IS the activation control (see {@link reconcileToolTabSelection}): a selected tool
 * leaf always means that tool is active, so the tree never renders a second activation toggle of its own —
 * that toggle carried the leaf tab's own element id, so the tab and the toggle disagreed about the same
 * state and the first press on an already-selected leaf deactivated the tool instead of arming it. A tool
 * whose program publishes no measures yet resolves to an empty options section.
 */
function buildToolTree(tool: ResolvedToolDefinition, measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown): { readonly sections: TreeDataSection[]; readonly sortableSections: false } {
  return {
    sortableSections: false,
    sections: [
      {
        id: `tool.${tool.id}.options`,
        label: "",
        defaultOpen: true,
        items: measures && measures.length > 0 ? windowMeasuresToTreeItems(measures, onAction) : [],
      },
    ],
  };
}

/**
 * 🛠️ One `PanelTabLeaf` per resolved mode tool — consumers wrap these under the Tool branch
 * (`FRAMEWORK_CATEGORY_TOOL_ID`) on `defaultDock.anchors["bottom-middle"]`, ordered left of the Command
 * branch, so the folded chrome shows a single Tool toggle. Content is a *lazy* `resolveTree` (mirrors
 * `buildCommandCategoryTabs`'s windows tab) so this array — and therefore `defaultDock`'s own memo —
 * never depends on `toolMeasuresByToolId`, which changes on every activation/slider tick; `resolveTree`
 * reads those fresh off the ref at render time instead.
 */
export function buildToolTabs(tools: readonly ResolvedToolDefinition[], toolMeasuresByToolIdRef: React.RefObject<Readonly<Record<string, readonly WindowMeasure[]>>>, onAction: (action: ActionDescriptor) => unknown): PanelTabNode[] {
  return tools.map((tool) =>
    singleTreeLeaf({
      id: `tool.${tool.id}`,
      icon: shellTabIcon(tool.iconId),
      name: tool.label,
      tree: {
        resolveTree: () => {
          const measures = toolMeasuresByToolIdRef.current[tool.id];
          const tree = buildToolTree(tool, measures, onAction);
          return { sections: tree.sections, sortableSections: tree.sortableSections };
        },
      },
    }),
  );
}

/** 🛠️ Which of the two halves of the one tool state changed since the last reconciliation, and therefore what the shell owes the other half (see {@link reconcileToolTabSelection}). */
export type ToolTabSelectionEffect = { readonly kind: "idle" } | { readonly kind: "activate"; readonly toolId: string | null } | { readonly kind: "select"; readonly toolId: string | null };

/** 🛠️ The pair reconciled by {@link reconcileToolTabSelection}: the mode's active tool and the `tool.<id>` leaf selected under the Tool category (`null` = no tool / the category collapsed to its branch). */
export interface ToolTabSelection {
  readonly toolId: string | null;
  readonly selected: string | null;
}

/**
 * 🛠️ Tool activation and the selected `tool.<id>` leaf tab are ONE state with two representations, and this
 * is its single owner. Every route into the panel path — a user press, a restored `DockUiStateStore`
 * arrangement, an introduction step, a dock reset — has to end at the same place, so activation may not
 * hang off the press callback alone: a hydrated path that already selects `tool.fill` used to render a
 * selected tab over a tool that was never activated, after which the user's first press read as a re-press
 * and *collapsed* the leaf instead of arming the tool.
 *
 * Last change wins: when the active tool moved (program `setActiveTool` effect, utility mutual exclusion,
 * introduction keep-alive) the selection follows it; otherwise a moved selection drives activation. Both
 * outcomes record the value they are about to establish, so the follow-up pass sees no change and cannot
 * bounce — and a refused activation self-heals on the next pass by deselecting the leaf. Callers must skip
 * reconciliation entirely (without updating `previous`) while the Tool category is not the active root, so
 * a tool stays live while the user browses another category and is re-reconciled on re-entry.
 */
export function reconcileToolTabSelection(previous: ToolTabSelection | null, activeToolId: string | null, selectedToolId: string | null): { readonly next: ToolTabSelection; readonly effect: ToolTabSelectionEffect } {
  const last = previous ?? { toolId: null, selected: null };
  if (activeToolId !== last.toolId) return { next: { toolId: activeToolId, selected: activeToolId }, effect: selectedToolId === activeToolId ? { kind: "idle" } : { kind: "select", toolId: activeToolId } };
  if (selectedToolId !== last.selected) return { next: { toolId: selectedToolId, selected: selectedToolId }, effect: selectedToolId === activeToolId ? { kind: "idle" } : { kind: "activate", toolId: selectedToolId } };
  return { next: last, effect: { kind: "idle" } };
}

/**
 * 🛠️ Whether a tool the PROGRAM just armed has to reveal its own `tool.<id>` leaf. {@link
 * reconcileToolTabSelection} is skipped wholesale while the Tool category is not the active root — which
 * is right for a tool the user armed and then browsed away from, and wrong for one a program arms on its
 * own (typing `fill 3` on the window's engagement line answers `Effect::SetActiveTool { fill }`): the
 * tool went live with no leaf tab mounted anywhere, so its options were unreachable and nothing on screen
 * said it had armed. Only a MOVE to a real tool reveals — an idle category, a disarm and a re-render on
 * the same tool all owe nothing.
 */
export function programArmedToolRevealV1(previousToolId: string | null, activeToolId: string | null, toolCategoryActive: boolean): boolean {
  if (toolCategoryActive) return false;
  return activeToolId !== null && activeToolId !== previousToolId;
}

/** 🛠️ The mode tool a footer tab id (`tool.<id>`) names — the selected leaf and the active tool are reconciled by {@link reconcileToolTabSelection}. */
export function toolIdFromPanelTabId(tabId: string | undefined): string | null {
  if (!tabId?.startsWith("tool.")) return null;
  const toolId = tabId.slice("tool.".length);
  return toolId.length > 0 ? toolId : null;
}

/**
 * ♻️ Identity the desktop and mobile Tool panels share so a late `framework.section.tools` admission
 * re-resolves lazy `resolveTree` sources. `buildToolTabs` keeps tab object identity across measure
 * ticks (the tree reads a ref); {@link PanelTreeUnitsPane} is memoized, so without this revision the
 * Fill body stays the empty section from the first resolve after the guest's packed tools carrier lands.
 */
export function toolPanelTreeContentRevision<TStaged>(
  activeToolId: string | null,
  toolMeasuresByToolId: Readonly<Record<string, readonly WindowMeasure[]>>,
  actionPaneStagedArgsByKey: TStaged,
): { readonly activeToolId: string | null; readonly toolMeasuresByToolId: Readonly<Record<string, readonly WindowMeasure[]>>; readonly actionPaneStagedArgsByKey: TStaged } {
  return { activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey };
}

/**
 * 🛠️ One press that opens the Tool category must land on a tool leaf (remembered, else the first
 * declared tool). `progressPanelTabSelection` does not auto-descend a branch with no memory; the
 * closed-panel open path used to swallow that press entirely, so Fill never became the selected leaf.
 */
export function toolCategoryOpenPath(path: readonly string[], memory: Readonly<Record<string, string>>, toolTabIds: readonly string[]): readonly string[] {
  if (path.length !== 1 || path[0] !== FRAMEWORK_CATEGORY_TOOL_ID || toolTabIds.length === 0) return path;
  const remembered = memory[FRAMEWORK_CATEGORY_TOOL_ID];
  const child = remembered && toolTabIds.includes(remembered) ? remembered : toolTabIds[0]!;
  return [...path, child];
}

/**
 * 🛠️ A re-press of a selected `tool.<id>` leaf while that tool is inactive must arm it, not collapse
 * the leaf. `progressPanelTabSelection` treats the press as an active-segment re-press and walks up;
 * the host keeps the previous path and returns the tool id that `setActiveTool` must receive.
 */
export function toolLeafInactiveRepress(previousPath: readonly string[], nextPath: readonly string[], activeToolId: string | null): { readonly path: readonly string[]; readonly toolId: string } | null {
  const previousToolId = toolIdFromPanelTabId(previousPath[previousPath.length - 1]);
  const nextToolId = toolIdFromPanelTabId(nextPath[nextPath.length - 1]);
  if (!previousToolId || nextToolId || (activeToolId ?? null) === previousToolId) return null;
  return { path: previousPath, toolId: previousToolId };
}
//#endregion 🛠️ToolRegistry

/** @emoji 🐢️ Structural equality over plain JSON-shaped values (the shape every `UiNode`/`WindowEngagement`/`WindowMeasure` program payload takes) — no cycles, no non-JSON types. */
function uiJsonDeepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!uiJsonDeepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  const aRecord = a as Record<string, unknown>;
  const bRecord = b as Record<string, unknown>;
  const aKeys = Object.keys(aRecord);
  const bKeys = Object.keys(bRecord);
  if (aKeys.length !== bKeys.length) return false;
  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(bRecord, key)) return false;
    if (!uiJsonDeepEqual(aRecord[key], bRecord[key])) return false;
  }
  return true;
}

/**
 * @emoji 🐢️ Reuses `previous`'s object identity when it's structurally equal to `next` — every program
 * `render()`/`utilities()`/`windowEngagements()`/`windowMeasures()` call re-parses a fresh JSON payload
 * every time, even when nothing about that body actually changed (e.g. a camera-only or selection-only
 * action still returns byte-identical panel/utility JSON). Without this, every downstream `React.memo`
 * (see `InterpretedUiNode`) sees a new prop reference every render and can never bail.
 */
export function preserveJsonIdentity<T>(previous: T | undefined, next: T): T {
  return previous !== undefined && uiJsonDeepEqual(previous, next) ? previous : next;
}

/**
 * @emoji 🐢️ Builds a `Record<string, V>` from `entries`, reusing `prev`'s per-key value reference where
 * `preserveJsonIdentity` finds no structural change, and reusing `prev` itself (the whole record) when
 * no key actually changed — so a no-operation action's `dispatch` doesn't hand `windowUiByWindowId`/etc. a new
 * object reference and cascade an unmemoizable re-render through every downstream consumer.
 */
export function mergeRecordPreservingIdentity<V>(prev: Readonly<Record<string, V>>, entries: readonly (readonly [string, V])[]): Readonly<Record<string, V>> {
  const next: Record<string, V> = {};
  let changed = Object.keys(prev).length !== entries.length;
  for (const [key, value] of entries) {
    const preserved = preserveJsonIdentity(prev[key], value);
    next[key] = preserved;
    if (preserved !== prev[key]) changed = true;
  }
  return changed ? next : prev;
}

//#region UiRefresh
/** @emoji 🐢️ One cached section value keyed by `${section}:${key}` (e.g. `window:2d-overview`, `engagements`) — the hash is what gets sent back to the plugin next time so it can skip re-serializing unchanged content. */
export type UiRefreshCache = Map<string, { readonly hash: string; readonly value: unknown }>;

/** 🖼️ How much of the shell one direct browser-actor dispatch dirtied.
 *
 * The actor handoff answers with `{outcome, mutationCount}` and carries NO `UiDirtyScope` — the guest's
 * own scope never crosses it — and a framework-reserved verb that commits inline (`paste`, `undo`, a
 * gumball commit) publishes no `OperationCompleted` frame either, so this route is the one dispatch path
 * with no scope of its own at all. An applied mutation therefore dirties everything: the honest answer
 * when the only thing known is THAT the document changed. `refreshUi` is hash-conditional, so an
 * unchanged section still costs no payload; what this buys is the changed window body being re-taken in
 * the same turn instead of whenever some later, unrelated action happens to refresh (measured at +12 s
 * for `paste` — ticket 26/09/02/PUZZLE-3D-END-TO-END wave B6 §3, fixed in wave B9 lane 4).
 */
export function browserActorDispatchUiScopeV1(result: { readonly outcome: "guest-applied" | "rejected"; readonly mutationCount: number }): UiDirtyScope {
  return result.outcome === "guest-applied" && result.mutationCount > 0 ? { kind: "full" } : { kind: "none" };
}

/** 📏 Window-option verbs publish only the WindowConfig lane, so the actor handoff reports
 * `mutationCount: 0`. Without this list the rail never re-takes measures after a successful toggle. */
export const WINDOW_CONFIG_RAIL_ACTION_IDS: ReadonlySet<string> = new Set([
  "focusSelection",
  "setCamera",
  "setChunkSize",
  "setGridSnapEnabled",
  "setGridSpacing",
  "setGridVisible",
  "setLodAutomatic",
  "setLodDepthVariable",
  "setLodManual",
  "setPanelPage",
  "setProjection",
  "setProjectionParam",
  "setProximityRadius",
  "setSelectableKind",
  "setSunAzimuth",
  "setSunElevation",
  "setSunIntensity",
  "setTransformGumballFlag",
  "setVortexDirection",
  "setVortexShow",
  "setVoxelDims",
  "toggleSun",
]);

export function browserActorWindowConfigDispatchUiScopeV1(
  result: { readonly outcome: "guest-applied" | "rejected"; readonly mutationCount: number },
  actionId: string | undefined,
): UiDirtyScope {
  const dirty = browserActorDispatchUiScopeV1(result);
  if (dirty.kind !== "none") return dirty;
  if (result.outcome === "guest-applied" && actionId !== undefined && WINDOW_CONFIG_RAIL_ACTION_IDS.has(actionId)) return { kind: "full" };
  return dirty;
}

/** 🏁️ What ONE typed-operation completion owes the shell — `null` for "owes no pass at all".
 *
 * A typed operation's admitting reply is only an ADMISSION: its edit is staged, published and logged on
 * later continuations, so that reply reports `mutationCount: 0` and carries no scope worth reading. The
 * completion frame (`AppFrame::OperationCompleted`) is the only carrier of the mutation's own
 * `UiDirtyScope` and, since wave B21, of its history patch — so the refresh has to follow the COMPLETION,
 * and it must be the completion's own scope rather than a guess keyed on a verb name (compare
 * {@link WINDOW_CONFIG_RAIL_ACTION_IDS}, which only exists because the browser-actor route publishes no
 * completion at all).
 *
 * The other half of the rule is what a completion does NOT owe. Every retained operation completes,
 * including the View-kind turns a drain poll finishes while the document is untouched: measured on the
 * live puzzle 3d shell at **18 completions in an idle 10 s window, 12 of them `{kind:"none"}`**, each
 * running a host-effect pass and a `refreshUi` call for nothing. A completion whose scope is `none`,
 * which carries no history patch and requests no effect is bookkeeping, and gets no pass.
 *
 * A patch with an empty scope is the one case that still owes a refresh: the row it mints belongs to the
 * reserved history body ({@link FRAMEWORK_HISTORY_BODY_KEY}), and nothing else on the shell moved.
 */
export function typedOperationCompletionRefreshV1(completion: {
  readonly uiScope: UiDirtyScope | undefined;
  readonly historyPatch: unknown;
  readonly requestedEffects: readonly unknown[];
}): UiDirtyScope | null {
  const scope = resolveUiDirtyScope(completion.uiScope);
  if (scope.kind !== "none") return scope;
  if (completion.historyPatch !== undefined) return { kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] };
  return completion.requestedEffects.length > 0 ? scope : null;
}

/** 🧰️ Host effects that rewrite host-owned state the GUEST reads back on its next render. The host's
 * per-window utility map and active tool ride into every `refresh-ui` as `ViewModel
 * .activeUtilityByWindowId`/`activeToolId`, and `setPanel` rewrites `panelJson` — so applying one of
 * these changes what every subsequent render MUST produce, and the pass that applies it owes that render.
 *
 * Everything else an effect pass can do is host chrome or re-enters a route that owns its own scope: a
 * `notify` banner is host React state, `clipboardWrite`/`downloadMediaExport` never reach a projection,
 * `dispatchAction`/`replayShellCommand` come back through shell dispatch with their own `UiDirtyScope`,
 * and `navigate`/`spawnPluginInstance`/`openPluginInstance` switch sessions (a session switch forces a
 * full fetch of its own). Those earn NOTHING here — that is what keeps {@link
 * typedOperationCompletionRefreshV1}'s no-storm property intact. */
const HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS = ["setActiveUtility", "setActiveTool", "setPanel", "loadDocument"] as const;

/**
 * 🧰️ The scope a host-effect pass owes ON TOP of what its dispatch or completion declared.
 *
 * The measured defect (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B37): the typed `brush` engagement verb
 * arms `scene.active_utility`, its epilogue emits `Effect::SetActiveUtility`, and the operation completes
 * with no mutation, no history patch and `UiDirtyScope::None` — so {@link
 * typedOperationCompletionRefreshV1} answers `none` (correctly: the GUEST re-took nothing) and
 * `applyHostEffects` handed that `none` straight to `refreshUi`, which asks for nothing at all. Live
 * console, `engagementSubmit` at `effects:1`: the host armed `puzzle3d-main-perspective → brush` and the
 * NEXT `refresh-ui` crossing happened 30 s later on an unrelated keystroke, with both world panes
 * publishing `select` for the whole interval.
 *
 * The scope a completion declares is what the GUEST dirtied. What the HOST dirtied by applying the
 * completion's own effects is this — and the two are unioned, never substituted.
 */
/** 🖼️ Whether this effect pass rewrote a guest render input at all — the same question {@link
 * hostEffectRefreshScopeV1} answers with a scope, asked where a boolean is what is needed: the follow-up
 * must ALSO land in the live built-node stores the mounted surfaces read (`ShellHost`'s
 * `forceReloadLiveUiStoresV1`, gated on `replaceBodies`). Without that the armed body is fetched, cached
 * and dispatched while the mounted world pane keeps publishing the previous `data-interaction-json`, which
 * is the same red one hop further down. */
export function hostEffectsRewriteGuestRenderInputsV1(effects: readonly unknown[]): boolean {
  return effects.some((effect) => typeof effect === "object" && effect !== null && HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS.some((key) => key in effect));
}

export function hostEffectRefreshScopeV1(effects: readonly unknown[], declared: UiDirtyScope, windowBodyKeys: readonly string[]): UiDirtyScope {
  if (!hostEffectsRewriteGuestRenderInputsV1(effects)) return declared;
  const rewriting = effects.filter((effect) => typeof effect === "object" && effect !== null && HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS.some((key) => key in effect));
  // 🪟️ `setPanel`/`loadDocument` name no body of their own — `panelJson` feeds every section, so the only
  // honest answer is the widest one, which a hash-conditional fetch makes cheap for the unchanged parts.
  const widest = rewriting.some((effect) => "setPanel" in (effect as object) || "loadDocument" in (effect as object));
  const earned: UiDirtyScope = widest
    ? { kind: "full" }
    : { kind: "partial", windowBodies: [...windowBodyKeys], panelBodies: [], utilities: true, tools: true, engagements: false, measures: true, labels: false };
  return mergeUiDirtyScopeV1(declared, earned);
}

/** 🤝️ The scope one refresh pass must cover when a SECOND pass was asked for while the first was
 * still crossing into the guest — the union, never the newer alone.
 *
 * `ShellHost.refreshUi` used to abandon a superseded pass outright: it bumps a generation on every
 * call and drops its own response when the generation moved under an await. That is correct only
 * while a pass is faster than the cadence that triggers passes. It is not: a converging preview's
 * `flowEvalTick` completions each demand a full pass, and one guest crossing during a brep solve was
 * measured at 15-16 s on the served procedural editor while completions arrived every few seconds —
 * so EVERY response was superseded before it could be applied and the flow window kept the previous
 * example's graph for as long as the evaluation ran, with no fault anywhere
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Coalescing onto this union is what turns that livelock
 * into one in-flight pass plus at most one owed follow-up, exactly the way the guest's own
 * `FlowEvalSession` latch admits at most one pending tick per window.
 */
export function mergeUiDirtyScopeV1(first: UiDirtyScope, second: UiDirtyScope): UiDirtyScope {
  if (first.kind === "full" || second.kind === "full") return { kind: "full" };
  if (first.kind === "none") return second;
  if (second.kind === "none") return first;
  return {
    kind: "partial",
    windowBodies: [...new Set([...(first.windowBodies ?? []), ...(second.windowBodies ?? [])])],
    panelBodies: [...new Set([...(first.panelBodies ?? []), ...(second.panelBodies ?? [])])],
    utilities: Boolean(first.utilities || second.utilities),
    tools: Boolean(first.tools || second.tools),
    engagements: Boolean(first.engagements || second.engagements),
    measures: Boolean(first.measures || second.measures),
    labels: Boolean(first.labels || second.labels),
  };
}

/** 🤝️ One ui-refresh lane: at most ONE pass running, at most ONE owed follow-up carrying the union of
 * everything asked for while it ran, and a follow-up that is NEVER lost — whatever the pass did.
 *
 * The three properties, and the live defect each one answers:
 *
 * 1. **Every request is owed until a pass covers it.** The starter is not a special case: it registers
 *    into the same owed slot every joiner does, and the drain loop takes it from there. A caller's
 *    promise settles when the pass that covered ITS request settled, so `await refreshUi(…)` means "my
 *    scope has been re-taken", not "somebody else's pass finished".
 * 2. **A failed pass never ends the lane.** The rejection reaches exactly the requests that pass
 *    covered (the boot refresh turns one into the session's fault card), and the loop still runs
 *    whatever was owed behind it. The predecessor `while` loop read its owed slot AFTER `await pass`,
 *    so one rejected pass dropped the follow-up on the floor — invisibly, since the owed entry is then
 *    cleared by the next unrelated starter.
 * 3. **A pass may ASK for another pass; it may never WAIT for one.** A host effect applied from inside
 *    a pass (`ShellHost`'s `pendingRefreshEffects`) re-enters this lane, and its request is served by
 *    the NEXT iteration — so the pass itself must never await that re-entrant work, or it is waiting on
 *    itself. The predecessor made the re-entrant caller `await` the very promise the pass resolves: the
 *    lane wedged with its in-flight marker pinned forever and NOTHING repainted again, which is the
 *    shape measured on the served puzzle 3d editor as an armed `SetActiveUtility` whose world pane still
 *    published `select` 30 s later (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B37). `ShellHost` keeps
 *    its half by applying a pass's own owed effects OUTSIDE the pass.
 *
 * `merge` is injected ({@link mergeUiDirtyScopeV1} plus whatever else the caller's request record
 * carries) so this lane owns sequencing only and knows nothing about sessions or window instances.
 */
export interface UiRefreshCoalescerV1<TRequest> {
  /** 🤝️ Asks for a pass covering `next`; resolves once a pass that covered it has settled. A `none` scope asks for nothing and resolves at once. */
  readonly request: (next: TRequest) => Promise<void>;
  /** 🩺️ Whether the drain loop owns the lane right now. */
  readonly busy: () => boolean;
  /** 🩺️ The scope owed to the next iteration, `null` when nothing is. */
  readonly owedScope: () => UiDirtyScope | null;
  /** 🩺️ Passes this lane has run, ever — the no-storm counter every coalescing law measures. */
  readonly passes: () => number;
}

export function createUiRefreshCoalescerV1<TRequest extends { readonly scope: UiDirtyScope }>(
  run: (request: TRequest) => Promise<void>,
  merge: (owed: TRequest, next: TRequest) => TRequest,
  onDecision?: (decision: "owed" | "merged" | "pass" | "failed", scope: UiDirtyScope, passes: number) => void,
): UiRefreshCoalescerV1<TRequest> {
  let owed: TRequest | null = null;
  let waiters: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  let draining = false;
  let passes = 0;
  const drain = async (): Promise<void> => {
    while (owed) {
      const pending = owed;
      const covered = waiters;
      owed = null;
      waiters = [];
      passes += 1;
      onDecision?.("pass", pending.scope, passes);
      try {
        await run(pending);
        for (const waiter of covered) waiter.resolve();
      } catch (error) {
        onDecision?.("failed", pending.scope, passes);
        for (const waiter of covered) waiter.reject(error);
      }
    }
  };
  return {
    request: (next: TRequest): Promise<void> => {
      if (next.scope.kind === "none") return Promise.resolve();
      onDecision?.(owed ? "merged" : "owed", next.scope, passes);
      owed = owed ? merge(owed, next) : next;
      const joined = new Promise<void>((resolve, reject) => waiters.push({ resolve, reject }));
      // 🔁️ The flag is raised BEFORE the loop is entered, never after: `run`'s own synchronous prologue is
      // allowed to re-enter `request` (that is property 3), and a marker written after the call would let
      // that re-entrant request start a SECOND drain — two passes crossing into the same guest at once.
      if (!draining) {
        draining = true;
        void drain().finally(() => {
          draining = false;
        });
      }
      return joined;
    },
    busy: () => draining,
    owedScope: () => owed?.scope ?? null,
    passes: () => passes,
  };
}

function uiRefreshWantsWindow(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.windowBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsPanel(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.panelBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsFlag(scope: UiDirtyScope, flag: "engagements" | "measures" | "tools" | "labels"): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && scope[flag] === true);
}
/** 🛍️ The app-static operator/palette catalogue never goes stale within an app instance, so it has no
 * `UiDirtyScope` flag of its own: only a full scope (a session switch, or the first fetch) asks for it,
 * and even then the cached hash means an unchanged catalogue costs one hash compare instead of a
 * ~100 KB re-serialize. See `ArtifactApp::app_catalogue_json`. */
function uiRefreshWantsCatalogue(scope: UiDirtyScope): boolean {
  return scope.kind === "full";
}

/**
 * 🪟️ Every live window instance for a session — one per base `AppDefinition.windowKinds` entry (id ==
 * kind id) plus one per split/spawned extra — so `refreshUi` fetches and the plugin returns state for
 * every actual window, never collapsing two same-kind instances (e.g. split top/perspective panes) onto
 * one shared entry.
 */
export function sessionWindowInstances(
  app: { readonly windowKinds: readonly { readonly id: string; readonly bodyKey: string }[] },
  extraWindowInstances: readonly ExtraWindowInstance[],
): readonly { readonly id: string; readonly bodyKey: string; readonly windowKindId: string }[] {
  const kindById = new Map(app.windowKinds.map((kind) => [kind.id, kind] as const));
  const base = app.windowKinds.map((kind) => ({ id: kind.id, bodyKey: kind.bodyKey, windowKindId: kind.id }));
  const extra = extraWindowInstances.flatMap((instance) => {
    const kind = kindById.get(instance.windowKindId);
    return kind ? [{ id: instance.id, bodyKey: kind.bodyKey, windowKindId: instance.windowKindId }] : [];
  });
  return [...base, ...extra];
}

/** 🎓️ Kind-level introduction targets must also match live window *instances* of that kind
 * (`puzzle3d-main-top` / `puzzle3d-main-perspective` for kind `puzzle3d-main`) — otherwise force-unfold
 * of the utility bar / Actions rail never reaches the panes the user actually sees. `targetKindId` is a
 * raw window-kind id; `targetSegment` is an already-normalized `elementIdSegment` (e.g. from a
 * `framework.window.{segment}.action.*` introduce id). */
export function introductionTargetsWindow(
  windowId: string,
  windowKindId: string,
  targetKindId: string | null,
  targetSegment: string | null = null,
): boolean {
  if (targetKindId && (elementIdSegment(windowId) === elementIdSegment(targetKindId) || elementIdSegment(windowKindId) === elementIdSegment(targetKindId))) return true;
  if (targetSegment && (elementIdSegment(windowId) === targetSegment || elementIdSegment(windowKindId) === targetSegment)) return true;
  return false;
}

/** @emoji 🧰️ Materializes the shell's per-window utility map for batched `refresh-ui` — omits null entries. */
export function buildActiveUtilityByWindowId(activeUtilityByWindowId: Readonly<Record<string, string | null>>): Record<string, string> {
  return Object.fromEntries(Object.entries(activeUtilityByWindowId).flatMap(([windowId, utilityId]) => (utilityId ? [[windowId, utilityId]] : [])));
}

/**
 * @emoji 🐢️ Builds one batched `refresh-ui` request restricted to `scope` — `null` when the scope
 * resolves to nothing worth fetching (`none`, or a `partial` whose fields all miss this app's actual
 * bodies/instances). Every requested entry carries the host's cached hash so the plugin can omit payloads
 * for sections that didn't change. `windowInstances` is keyed by window INSTANCE id (base windows plus any
 * split/spawned extras) — never by window kind — so two instances of the same kind get independent
 * cache entries and independent rendered bodies.
 */
export function buildUiRefreshRequest(
  scope: UiDirtyScope,
  windowInstances: readonly { readonly id: string; readonly bodyKey: string }[],
  panelTabLeaves: readonly { readonly kind: PanelTabKind; readonly bodyKey?: string }[],
  viewState: PluginViewState,
  cache: UiRefreshCache,
): PluginUiRefreshRequest | null {
  if (scope.kind === "none") return null;
  const windows = windowInstances.filter((instance) => uiRefreshWantsWindow(scope, instance.bodyKey)).map((instance) => ({ key: instance.id, bodyKey: instance.bodyKey, hash: cache.get(`window:${instance.id}`)?.hash }));
  const panels = panelTabLeaves
    .filter((tab): tab is { readonly kind: PanelTabKind; readonly bodyKey: string } => Boolean(tab.bodyKey) && uiRefreshWantsPanel(scope, tab.bodyKey!))
    .map((tab) => ({ key: panelTabKindId(tab.kind), bodyKey: tab.bodyKey, hash: cache.get(`panel:${panelTabKindId(tab.kind)}`)?.hash }));
  const engagements = uiRefreshWantsFlag(scope, "engagements") ? { hash: cache.get("engagements")?.hash } : undefined;
  const measures = uiRefreshWantsFlag(scope, "measures") ? { hash: cache.get("measures")?.hash } : undefined;
  const tools = uiRefreshWantsFlag(scope, "tools") ? { hash: cache.get("tools")?.hash } : undefined;
  const labels = uiRefreshWantsFlag(scope, "labels") ? { hash: cache.get("labels")?.hash } : undefined;
  const catalogue = uiRefreshWantsCatalogue(scope) ? { hash: cache.get("catalogue")?.hash } : undefined;
  if (windows.length === 0 && panels.length === 0 && !engagements && !measures && !tools && !labels && !catalogue) return null;
  return { viewState, windows, panels, engagements, measures, tools, catalogue, labels };
}

/** @emoji 🐢️ Writes every changed section (`value !== undefined`) from a `refresh-ui` response into `cache`; unchanged sections are left as-is since the cached value is still current. */
function applyUiRefreshSectionsToCache(cache: UiRefreshCache, prefix: string, entries: readonly PluginUiRefreshSectionResponse[] | undefined): void {
  for (const entry of entries ?? []) {
    if (entry.value !== undefined) cache.set(`${prefix}:${entry.key}`, { hash: entry.hash, value: entry.value });
  }
}

export function applyUiRefreshResponseToCache(cache: UiRefreshCache, response: PluginUiRefreshResponse): void {
  applyUiRefreshSectionsToCache(cache, "window", response.windows);
  applyUiRefreshSectionsToCache(cache, "panel", response.panels);
  if (response.engagements?.value !== undefined) cache.set("engagements", { hash: response.engagements.hash, value: response.engagements.value });
  if (response.measures?.value !== undefined) cache.set("measures", { hash: response.measures.hash, value: response.measures.value });
  if (response.tools?.value !== undefined) {
    cache.set("tools", { hash: response.tools.hash, value: response.tools.value });
    const tools = response.tools.value as Record<string, unknown> | undefined;
  }
  if (response.catalogue?.value !== undefined) cache.set("catalogue", { hash: response.catalogue.hash, value: response.catalogue.value });
  if (response.labels?.value !== undefined) cache.set("labels", { hash: response.labels.hash, value: response.labels.value });
}
//#endregion UiRefresh
//#endregion ShellHelpers
