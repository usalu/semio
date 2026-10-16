/** 🔩️ General shell plumbing: action-history and reserved-id bookkeeping, example switching, leftover interaction views, presence identity, UI history, media-export downloads, file requests and host-effect dispatch.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type ActionDescriptor,
  type HistoryEntry,
  historyEntryKey,
  historyEntryLabelText,
  type HistoryPatch,
  type HistoryReprojection,
  type HistoryTimeTravel,
  type Effect,
  type HostEffectDispatchScope,
  hostEffectInvocationV1,
  type LocalizedLabel,
  RECORD_TUTORIAL_ACTION_ID,
  resolveUiDirtyScope,
  SHELL_LOCALES,
  type ShellLocale,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  type UiDirtyScope,
  EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  HOST_EVENT_ACTION_ID,
  IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  OPEN_ARTIFACT_FILE_ACTION_ID,
  SAVE_ARTIFACT_FILE_ACTION_ID
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { packValueFromBase64, packValueToBase64 } from "@semio-tech/framework/packs";
import {
  decodePresenceHistoryEdit,
  decodePresenceInteraction,
  decodePresenceToolRun,
  type ArtifactPresenceHistoryEdit,
  type ArtifactPresenceInteraction,
  type ArtifactPresenceToolRun
} from "@semio-tech/framework/replication";
import { type Anchor, resolveTranslationLabel, uiI18n, type UiTranslationKey } from "@semio-tech/ui-react";
import { useCallback, useEffect, useState } from "react";
import type { SelectionMode } from "@semio-tech/framework";
import { mediaExportBytes } from "../../../../../../../../🔨️modules/🎠️kernel/🎬️media-export/🟦️.ts";
import { wireMediaExportEncoding } from "../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { causalOrderKeyV1, type InputProvenanceV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { type ActiveSession, type LoadedProgramState, type UIHistory } from "../../🐚️Shell/🟦️.tsx";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { segmentedDownloadSinkFactory, type SegmentedDownloadSinkFactory } from "../../📤️SegmentedDownload/🟦️.ts";
import { resolveManifestLabel } from "../🏷️labels/🟦️.tsx";

//#region ShellHelpers
/** ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel. */
export const DEFAULT_PANEL_WIDTH_PX = 300;

/** 🌳️ Root category id for the nested dock tab tree — the top row of {@link defaultDock}'s bottom-left (Display) anchor tabs; top-left (Workbench), top-right (Details) and bottom-right (Settings) render their tabs flat instead of under a category branch. */
export const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
/** 🎛️ Root category id bundling every command-category leaf under one expandable Command toggle on bottom-middle (mirrors Display on bottom-left). */
export const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";
/** 🛠️ Root category id bundling every mode-level tool leaf under one expandable Tool toggle on
 * bottom-middle, ordered left of the Command branch (mirrors Command's own bundling on the same anchor). */
export const FRAMEWORK_CATEGORY_TOOL_ID = "framework.category.tool";

/** 🎛️ Corner/top-middle/bottom-middle anchors park their *folded* root tab row in navbar/footer chrome (via {@link PanelChromeTabBar}); while open, the floating {@link Panel} hosts the full strip on its {@link WindowChrome}. The two side-middle anchors have no navbar/footer slot, so they're absent here and fall back to `"panel"` (see the `?..:"panel"` read site), carrying their own tab bar when folded too. */
export const PANEL_TAB_BAR_HOSTS: Partial<Record<Anchor, "navbar" | "footer">> = {
  "top-left": "navbar",
  "top-middle": "navbar",
  "top-right": "navbar",
  "bottom-left": "footer",
  "bottom-middle": "footer",
  "bottom-right": "footer",
};
export const APP_BREADCRUMB_SEPARATOR = " · ";

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

/** 🧾️ One program's history as the shell projects it: the cursor undo/redo act on, the rows folded under
 * {@link historyEntryKey}, the checkpoint the check-in lane watches, the live history-edit session the time-travel
 * band and window indicators show (`null` while none is open — every patch carries the session in full), the history change
 * replaying before adoption the shell's reprojection status announces (`HistoryPatch.reprojection`, `null` while none waits), and
 * how many edits the history holds (`HistoryPatch.editCount`, the `{n}` of the `history.full` notice). */
export type ShellHistoryProjectionV1 = {
  readonly cursor: number;
  readonly entries: Readonly<Record<string, HistoryEntry>>;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
  readonly currentCheckpointId: string | undefined;
  readonly timeTravel: HistoryTimeTravel | null;
  readonly reprojection: HistoryReprojection | null;
  readonly editCount: number;
};

/** 🧾️ What a program whose history has not been read yet projects: no rows, no session. */
export const EMPTY_SHELL_HISTORY_PROJECTION_V1: ShellHistoryProjectionV1 = { cursor: 0, entries: {}, canUndo: false, canRedo: false, currentCheckpointId: undefined, timeTravel: null, reprojection: null, editCount: 0 };

/** 🧾️ `current` after one admitted patch ({@link historyPatchShouldApplyV1}): a snapshot replaces the rows, a delta
 * upserts them, and the session is the patch's own — a patch without one closes it. */
export function shellHistoryProjectionAfterPatchV1(current: ShellHistoryProjectionV1, patch: HistoryPatch, replace: boolean): ShellHistoryProjectionV1 {
  const entries: Record<string, HistoryEntry> = replace ? {} : { ...current.entries };
  for (const entry of patch.upserts ?? []) entries[historyEntryKey(entry)] = entry;
  return { cursor: patch.cursor, entries, canUndo: patch.canUndo ?? false, canRedo: patch.canRedo ?? false, currentCheckpointId: replace ? patch.currentCheckpointId : (patch.currentCheckpointId ?? current.currentCheckpointId), timeTravel: patch.timeTravel ?? null, reprojection: patch.reprojection ?? null, editCount: patch.editCount ?? 0 };
}

/** 🚛️ One program's projection while its document loads in steps: the host's own polled progress as the `load`
 * reprojection, every other field kept — the shell status outside the History panel then shows a load like any other
 * replay (`HistoryReprojectionStatus`). `null` ends it: a settled, failed or cancelled load leaves no `load` reprojection
 * behind, and a replay of another kind is never touched. The same object when nothing changes. */
export function shellHistoryProjectionWithLoadV1(current: ShellHistoryProjectionV1, load: { readonly completed: number; readonly total: number } | null): ShellHistoryProjectionV1 {
  const shown = current.reprojection;
  if (load === null) return shown?.kind === "load" ? { ...current, reprojection: null } : current;
  if (shown?.kind === "load" && shown.done === load.completed && shown.total === load.total) return current;
  return { ...current, reprojection: { kind: "load", done: load.completed, total: load.total } };
}

/** 🧳️ Every program's projection after one polled step of `key`'s stepped document load
 * ({@link shellHistoryProjectionWithLoadV1}) — the same map when nothing changes, so the history store wakes no reader. */
export function programHistoryProjectionsWithLoadV1(projections: Readonly<Record<string, ShellHistoryProjectionV1>>, key: string, load: { readonly completed: number; readonly total: number } | null): Readonly<Record<string, ShellHistoryProjectionV1>> {
  if (key === "") return projections;
  const current = projections[key] ?? EMPTY_SHELL_HISTORY_PROJECTION_V1;
  const next = shellHistoryProjectionWithLoadV1(current, load);
  return next === current ? projections : { ...projections, [key]: next };
}

/** 🎞️ One unsolicited mid-operation frame (`AppFrame::Invocation`, `in_reply_to` 0): the dirty scope a running
 * operation asks the shell to refresh and the history patch a running replay publishes. */
export type OperationProgressV1 = { readonly uiScope?: UiDirtyScope; readonly historyPatch?: HistoryPatch };

/** 🎞️ What the shell does with one progress frame: refresh its scope (nothing when it carried none — a bare frame is
 * never read as a full refresh) and fold its history patch into the program's projection. */
export function operationProgressPartsV1(progress: OperationProgressV1): { readonly scope: UiDirtyScope; readonly historyPatch: HistoryPatch | undefined } {
  return { scope: progress.uiScope === undefined ? { kind: "none" } : resolveUiDirtyScope(progress.uiScope), historyPatch: progress.historyPatch };
}

/** 🧾️ The framework history cursor as the shell publishes it on `data-history-json` — the ONE readable
 * statement of what undo and redo would actually do right now.
 *
 * `historyProjection` is the shell's own reduction of every `HistoryPatch` the guest sent, and it is what
 * gates the `framework.history.undo`/`.redo` controls. Until this existed nothing carried it into the
 * document, so an undo/redo assertion could only watch WHICH ACTION a chord minted and never whether the
 * history it acted on had anything to undo — measured on generation3d 6018, `historyJsonPublished: false`
 * with zero `[data-history-json]` carriers in the page (`📓️react-current-tree-battery-2026-09-14.md` §6).
 * The newest few labels ride along in cursor order so a reader can name the entry a press would revert
 * without a second round trip; the list is bounded because this is chrome state on a DOM attribute, not
 * the History panel. */
export const SHELL_HISTORY_DOM_LABELS = 8;

export function shellHistoryCursorDomV1(
  projection: {
    readonly cursor: number;
    readonly entries: Readonly<Record<string, { readonly seq: number; readonly label: LocalizedLabel; readonly actionId: string }>>;
    readonly canUndo: boolean;
    readonly canRedo: boolean;
    readonly currentCheckpointId?: string;
  },
  axes: { readonly terminology: string; readonly locale: string },
): Record<string, unknown> {
  const ordered = Object.values(projection.entries ?? {}).sort((left, right) => left.seq - right.seq);
  const recent = ordered.slice(-SHELL_HISTORY_DOM_LABELS);
  const text = (entry: { readonly label: LocalizedLabel } | undefined): string | null => (entry ? historyEntryLabelText(entry.label, axes.terminology, axes.locale) : null);
  return {
    cursor: projection.cursor,
    canUndo: Boolean(projection.canUndo),
    canRedo: Boolean(projection.canRedo),
    entries: ordered.length,
    currentCheckpointId: projection.currentCheckpointId ?? null,
    labels: recent.map((entry) => historyEntryLabelText(entry.label, axes.terminology, axes.locale)),
    actionIds: recent.map((entry) => entry.actionId),
    undoLabel: projection.canUndo ? text(ordered.findLast((entry) => entry.seq <= projection.cursor)) : null,
    redoLabel: projection.canRedo ? text(ordered.find((entry) => entry.seq > projection.cursor)) : null,
  };
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
  EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  OPEN_ARTIFACT_FILE_ACTION_ID,
  SAVE_ARTIFACT_FILE_ACTION_ID,
  HOST_EVENT_ACTION_ID,
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
export function undeclaredActionDiagnostic(
  appId: string,
  action: string,
  windowKinds: readonly WindowKindActionDeclaration[],
  windowKindId?: string | null,
  appActions: readonly { readonly id: string }[] = [],
): UndeclaredActionDiagnostic | null {
  if (FRAMEWORK_RESERVED_ACTION_IDS.has(action)) return null;
  if (windowKinds.some((kind) => (kind.actions ?? []).some((entry) => entry.id === action))) return null;
  if (appActions.some((entry) => entry.id === action)) return null;
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

/** ⏪️ A chrome command the shell can really put back, as {@link buildNoteShellCommandAction} declares it:
 * the command id that restores the state this note replaced, plus the arguments that restore it. */
export interface ShellCommandInverse {
  readonly commandId: string;
  readonly args?: Record<string, unknown>;
}

/** 🧭️ Builds the `noteShellCommand` action descriptor `noteShellCommand` (the component helper) dispatches
 * through the standard `onAction` funnel — pure so it's testable without a session/component.
 *
 * ⏪️ `inverse` is what makes the row an UNDO TARGET, and only a caller holding the state this command
 * replaced can declare one: `detail` describes where the chrome went, never where it came from, so
 * inverting a note by re-dispatching its own id and its own arguments (what this builder used to do
 * unconditionally) is an identity, not an inverse. Without it the guest records the row with no
 * inverse and `undo` steps over it straight onto the user's document edit — see
 * `🔌️plugin/🦀️.rs`'s `noteShellCommand` branch and `dispatch_chrome_history_action`. */
export function buildNoteShellCommandAction(controllerId: string, commandId: string, label: ShellLabelTextV1, detail?: Record<string, unknown>, inverse?: ShellCommandInverse): ActionDescriptor {
  return {
    controllerId,
    action: NOTE_SHELL_COMMAND_ACTION_ID,
    args: { commandId, label, ...(detail ? { detail } : {}), ...(inverse ? { inverseCommandId: inverse.commandId, ...(inverse.args ? { inverseArgs: inverse.args } : {}) } : {}) },
  };
}

/** 🐚️ Whether a replayed command id names SHELL-owned state (dock, panels, windows, appearance) rather
 * than a plugin action. `Effect::ReplayShellCommand` carries both: a `View`-kind row replays a plugin
 * action id the guest itself declared, while a `🐚️Shell`-kind row replays chrome the guest has no
 * window kind for. Dispatching the latter into the guest is what produced the framework-wide
 * `undeclared-action (guest window=… causedBy=…)` error after every undo (ticket 26/09/18 §3.2). */
/** 🌐️ One text in every shell locale — what a journalled `noteShellCommand` row carries (`{en, de}`), so the guest stores
 * a real `LocalizedLabel::native` and the Rust history body re-renders the row on a locale switch instead of freezing it
 * in the locale it was dispatched in (ticket 26/09/18 S10 §2.10 item 2).
 * @see ../../../../🔌️plugin/🦀️.rs */
export type ShellLabelTextV1 = Readonly<Record<ShellLocale, string>>;

/** 🌐️ A chrome translation key's text in every shell locale, at the active driver's label tier. */
export function shellLabelTextV1(key: UiTranslationKey, options?: Record<string, unknown>): ShellLabelTextV1 {
  return Object.fromEntries(SHELL_LOCALES.map((locale) => [locale, resolveTranslationLabel(uiI18n.tIn(locale, key, options)) ?? key])) as ShellLabelTextV1;
}

/** 🌐️ A manifest label's text in every shell locale on one terminology; a plain string reads the same in each. */
export function manifestLabelTextV1(label: LocalizedLabel | string, terminology: string): ShellLabelTextV1 {
  return Object.fromEntries(SHELL_LOCALES.map((locale) => [locale, resolveManifestLabel(label, terminology, locale)])) as ShellLabelTextV1;
}

export function isShellOwnedCommandId(commandId: string): boolean {
  return commandId.startsWith("shell.") || commandId.startsWith("os.");
}

/** 🎨️ The program action the navbar example picker dispatches. */
export const SET_ACTIVE_EXAMPLE_ACTION_ID = "setActiveExample";

/** 📚️ Whether the app itself declares `setActiveExample`. The navbar no longer uses this as the
 * visibility gate: an editor loads its registered catalogue even when the action was never declared. */
export function appSwitchesExamples(appId: string, windowKinds: readonly WindowKindActionDeclaration[], appActions: readonly { readonly id: string }[] = []): boolean {
  return undeclaredActionDiagnostic(appId, SET_ACTIVE_EXAMPLE_ACTION_ID, windowKinds, null, appActions) === null;
}

/** 📚️ Whether the navbar lists registered examples. An editor always does when the catalogue is
 * non-empty — the framework loads the body. A viewer lists them only when it declares the action,
 * because it does not own the editor's example documents. */
export function appOffersRegisteredExamples(
  app: { readonly id: string; readonly role?: string; readonly windowKinds: readonly WindowKindActionDeclaration[]; readonly actions?: readonly { readonly id: string }[] },
  exampleCount: number,
): boolean {
  if (exampleCount <= 0) return false;
  if (app.role === "editor") return true;
  return appSwitchesExamples(app.id, app.windowKinds, app.actions ?? []);
}

/** 🎨️ An editor may dispatch `setActiveExample` without declaring it. The guest loads the registered catalogue. */
export function frameworkOwnsExampleSwitch(role: string | undefined, action: string): boolean {
  return role === "editor" && action === SET_ACTIVE_EXAMPLE_ACTION_ID;
}

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
  upserts: readonly { readonly actionId?: string; readonly revertible?: boolean }[] | undefined,
  rememberedExampleId: string,
  bootExampleId: string,
): string | undefined {
  // 🪪️ Keyed on the action id alone. The former `label.includes("Set Active Example")` arm matched
  // the English prose of a row's label, which no longer exists as a single string and would have
  // silently stopped matching for a German shell — the id is the locale-independent key.
  const rows = (upserts ?? []).filter((entry) => entry.actionId === SET_ACTIVE_EXAMPLE_ACTION_ID);
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
  /** 🧭️ `SelectionMode`, not `string`: this view is built from — and handed straight back to — the
   * shell's own `InteractionState`, whose `activeMode` the interaction module declares as that union. */
  readonly activeMode: Readonly<Record<string, SelectionMode>>;
  readonly activeGranularity: Readonly<Record<string, string>>;
  readonly activeUtility?: string | null;
  /** 🪟️ The window INSTANCE the action that produced this leftover addressed, straight from the
   * guest's own encode (`leftover_interaction_view_from`, `🔌️plugin/🦀️.rs`). `null` is a windowless,
   * document-scoped action — never a synthetic window surface (wave B56). */
  readonly windowId: string | null;
  readonly selectionCleared?: boolean;
};

/** 🧭️ The guest encodes `activeMode` as free text; only the two the interaction module declares are a
 * `SelectionMode`, and anything else is dropped rather than smuggled into the shell's own state. */
function leftoverSelectionModeRecord(value: unknown): Record<string, SelectionMode> {
  const isSelectionMode = (mode: unknown): mode is SelectionMode => mode === "single" || mode === "multiple";
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).filter((entry): entry is [string, SelectionMode] => isSelectionMode(entry[1])));
}

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

/** 🕹️ Peels leftover `Invocation.output.interactionView` — same leftover lane as history_patch.
 * Gumball visibility follows `gumball.active` only (transform utility), not bare `selectedIds`. */
export function interactionViewFromLeftoverOutput(output: unknown): LeftoverInteractionViewV1 | null {
  if (!output || typeof output !== "object" || Array.isArray(output)) return null;
  const envelope = output as { interactionView?: unknown };
  const raw = envelope.interactionView;
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return null;
  const view = raw as Record<string, unknown>;
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
    gumballActive: gumball.active === true,
    gumballAnchorId: typeof gumball.anchorId === "string" ? gumball.anchorId : gumball.active === true ? selectedIds[0] ?? null : null,
    windowId: typeof view.windowId === "string" && view.windowId.length > 0 ? view.windowId : null,
    selection: leftoverSelectionRecord(view.selection),
    hover: leftoverHoverRecord(view.hover),
    activeMode: leftoverSelectionModeRecord(view.activeMode),
    activeGranularity: leftoverStringRecord(view.activeGranularity),
    ...(typeof view.activeUtility === "string" ? { activeUtility: view.activeUtility } : {}),
    ...(view.selectionCleared === true ? { selectionCleared: true } : {}),
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
    transformMode: leftover.gumballActive ? "transform" : undefined,
    gumballTarget,
  };
}

/** 🧭️ Action ids the tutorial recorder never captures (see `onAction`'s recorder tap) — telemetry/chrome
 * noise a tutorial replay should never literally reproduce, or actions the director/recorder itself just
 * dispatched. Exported so it's independently testable. */
export const TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS: ReadonlySet<string> = new Set([NOTE_WORLD_NAVIGATION_ACTION_ID, NOTE_SHELL_COMMAND_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, RECORD_TUTORIAL_ACTION_ID]);

export const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
export const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

/** 🖱️ The least spacing between two presence beats a local window-view change (pointer, camera, caret) triggers, so a
 * peer's cursor follows within ~one frame budget instead of the next {@link PRESENCE_HEARTBEAT_INTERVAL_MS} beat. Pinned
 * by `💓️presence-liveness-v1` (`a-moving-pointer-beats-live`). */
export const PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS = 100;

/** ⏳️ How long a view-change beat waits: none once the last beat is {@link PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS} old,
 * else the rest of that spacing (a clock that ran backwards waits the whole spacing). */
export function presenceLiveViewBeatDelayMsV1(lastBeatAtMs: number, nowMs: number): number {
  const elapsed = nowMs - lastBeatAtMs;
  return elapsed < 0 ? PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS : Math.max(0, PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS - elapsed);
}

/** ⏱️ How long one presence beat may wait for the document component's ephemeral snapshot before it
 * goes out without one. Strictly below {@link PRESENCE_HEARTBEAT_INTERVAL_MS} so a slow document can
 * never push a beat into the next interval, and the pair sits far below the hub's own
 * `PRESENCE_LEASE_TTL_MS` (15 000) — three whole beats of headroom. The three numbers are pinned
 * together, in both languages, by `🧫️fixtures/💓️presence-liveness-v1/🔣️.json`. */
export const PRESENCE_EPHEMERAL_SNAPSHOT_DEADLINE_MS = 2000;

/** 💓️ One document's ephemeral snapshot, bounded. A wedged document component answers
 * `ephemeralSnapshot` never; awaiting it bare inside the beat's single flight silenced the shell's
 * heartbeat for that document for the rest of the session, and the hub then let the human's roster
 * row lapse under a socket that was still open (ticket 26/09/18 slice PR1). Past the bound the beat
 * carries no app-owned pack — the human stays present, only the app's ephemerals are dropped, which
 * is precisely what the hub publishes for a lapsed lease. A rejection is the same answer as a
 * timeout: presence is not the place to surface a document fault. */
export async function presenceEphemeralSnapshotWithinBoundV1(
  plugin: {
    readonly ephemeralSnapshot?: (
      instanceId: number,
    ) => Promise<{
      readonly presence: readonly number[];
      readonly interaction?: readonly number[];
      readonly toolRun?: readonly number[];
      readonly historyEdit?: readonly number[];
      readonly activeTool?: string;
      readonly presenceGeneration?: number;
      readonly transientGeneration?: number;
    } | null>;
  },
  instanceId: number,
  deadlineMs: number = PRESENCE_EPHEMERAL_SNAPSHOT_DEADLINE_MS,
): Promise<
  | {
      readonly presence: readonly number[];
      readonly interaction?: readonly number[];
      readonly toolRun?: readonly number[];
      readonly historyEdit?: readonly number[];
      readonly activeTool?: string;
      readonly presenceGeneration?: number;
      readonly transientGeneration?: number;
    }
  | undefined
> {
  if (plugin.ephemeralSnapshot === undefined) return undefined;
  let timer: ReturnType<typeof setTimeout> | undefined;
  const bound = new Promise<undefined>((resolve) => {
    timer = setTimeout(() => resolve(undefined), deadlineMs);
  });
  try {
    return await Promise.race([Promise.resolve(plugin.ephemeralSnapshot(instanceId)).then((snapshot) => snapshot ?? undefined).catch(() => undefined), bound]);
  } finally {
    if (timer !== undefined) clearTimeout(timer);
  }
}

/** 👥️ The fields of this human's presence heartbeat an app's ephemeral snapshot owns, decoded and present only when the
 * snapshot carries them: the declared-broadcast interaction slice (bit 5), the tool-run summary (bit 10) and the open
 * history edit (bit 13) — the React twin of what the native host stamps, so a peer sees who edits history on which
 * mutation whichever renderer that human runs. */
export function presenceEphemeralPeerFieldsV1(snapshot: { readonly interaction?: readonly number[]; readonly toolRun?: readonly number[]; readonly historyEdit?: readonly number[] } | undefined): { readonly interaction?: ArtifactPresenceInteraction; readonly toolRun?: ArtifactPresenceToolRun; readonly historyEdit?: ArtifactPresenceHistoryEdit } {
  return {
    ...(snapshot?.interaction !== undefined && snapshot.interaction.length > 0 ? { interaction: decodePresenceInteraction(Uint8Array.from(snapshot.interaction), [0]) } : {}),
    ...(snapshot?.toolRun !== undefined && snapshot.toolRun.length > 0 ? { toolRun: decodePresenceToolRun(Uint8Array.from(snapshot.toolRun)) } : {}),
    ...(snapshot?.historyEdit !== undefined && snapshot.historyEdit.length > 0 ? { historyEdit: decodePresenceHistoryEdit(Uint8Array.from(snapshot.historyEdit)) } : {}),
  };
}

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

/** 📥️ Recovers the guest's `encoding` from the wire, whatever shape the WIT `option<string>`
 * arrived in — the SAME reader both renderer doors use (`🖼️wire-turn.ts`). Reading it flat here is
 * what dropped it for every binary export (`📓️io-surface-2026-09-13.md` §7.3). What the string MEANS
 * is `kernel::mediaExportBytes`; a `SEGMENTED_DOWNLOAD_MARKER_PREFIX` value is a handle, not an
 * encoding, and is consumed by the segmented lane before either reaches the bytes. */
export const mediaExportEncodingText = wireMediaExportEncoding;

/** 📥️ Host download of guest `download-media-export`. The bytes come from the kernel's own
 * `(data, encoding)` contract — not a local `atob` branch — so text stays text, base64 becomes bytes,
 * and an encoding no shell knows is a loud typed refusal rather than a corrupt file. The anchor must
 * be in the document and the object URL must outlive the click or the browser drops the file. */
export function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  downloadMediaExportBytes(filename, mimeType, mediaExportBytes(data, encoding));
}

/** 📥️ Host delivery of already-assembled bytes — the blob-and-anchor half of {@link downloadMediaExport}, reached by the segmented lane once its chunks are drained. */
export function downloadMediaExportBytes(filename: string, mimeType: string, bytes: Uint8Array): void {
  if (typeof document === "undefined") return;
  // 🧭️ `slice()` because a `Uint8Array` may be backed by a `SharedArrayBuffer`, which is not a
  // `BlobPart`; it answers a fresh `Uint8Array<ArrayBuffer>`, and `Blob` copies its parts anyway.
  const blob = new Blob([bytes.slice()], { type: mimeType });
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

/** 🌊 The sink factory every shell drain uses: assembled chunks delivered as one file through {@link downloadMediaExportBytes}. */
export const shellSegmentedDownloadSinkFactory: SegmentedDownloadSinkFactory = segmentedDownloadSinkFactory(downloadMediaExportBytes);

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
type FileWithNativePath = File & { readonly path?: string };

/** 📂 Opens a single-file picker for backbone `file://` attach — resolves an absolute path when the
 * host exposes one (`File.path` on native shells); otherwise `null` (cancel or path unavailable). */
export function requestBackboneFilePath(accept = ".json,application/json"): Promise<string | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.onchange = () => {
      const file = input.files?.[0] as FileWithNativePath | undefined;
      resolve(file?.path?.trim() ? file.path : null);
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

/** 📂 Opens a folder picker for backbone `folder://` attach — resolves an absolute directory path when
 * the host exposes one; otherwise `null`. */
export function requestBackboneFolderPath(): Promise<string | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.webkitdirectory = true;
    input.onchange = () => {
      const files = input.files;
      const first = files?.[0] as FileWithNativePath | undefined;
      if (!first) {
        resolve(null);
        return;
      }
      const native = first.path?.trim();
      if (native) {
        resolve(native.replace(/[/\\][^/\\]+$/, "") || native);
        return;
      }
      resolve(null);
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

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

/** 📏️ Maximum browser-owned natural-file buffer admitted before native codec admission. */
export const NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1 = 512 * 1024 * 1024;

/** 🧩️ Maximum Blob slice materialized by one cancellable natural-file read turn. */
export const NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1 = 256 * 1024;

/** 📂️ Opens a picker without reading the selected files. Cancel always settles with an empty list and
 * every terminal path detaches handlers and the transient input exactly once. */
export function requestFileSelectionV1(accept: string, multiple = false): Promise<readonly File[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.multiple = multiple;
    let settled = false;
    const settle = (files: readonly File[]): void => {
      if (settled) return;
      settled = true;
      input.onchange = null;
      input.oncancel = null;
      input.remove();
      resolve(files);
    };
    input.onchange = () => settle(input.files ? Array.from(input.files) : []);
    input.oncancel = () => settle([]);
    try {
      input.click();
    } catch {
      settle([]);
    }
  });
}

const readBlobSliceV1 = (blob: Blob, signal: AbortSignal): Promise<ArrayBuffer> => {
  if (typeof FileReader === "undefined") {
    let onAbort: (() => void) | undefined;
    const aborted = new Promise<never>((_, reject) => {
      onAbort = () => reject(signal.reason ?? new DOMException("The operation was aborted", "AbortError"));
      signal.addEventListener("abort", onAbort, { once: true });
    });
    return Promise.race([blob.arrayBuffer(), aborted]).finally(() => {
      if (onAbort !== undefined) signal.removeEventListener("abort", onAbort);
    });
  }
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    let settled = false;
    const finish = (result: { readonly ok: true; readonly value: ArrayBuffer } | { readonly ok: false; readonly error: unknown }): void => {
      if (settled) return;
      settled = true;
      signal.removeEventListener("abort", abort);
      reader.onload = null;
      reader.onerror = null;
      reader.onabort = null;
      if (result.ok) resolve(result.value); else reject(result.error);
    };
    const abort = (): void => {
      reader.abort();
      finish({ ok: false, error: signal.reason ?? new DOMException("The operation was aborted", "AbortError") });
    };
    reader.onload = () => reader.result instanceof ArrayBuffer
      ? finish({ ok: true, value: reader.result })
      : finish({ ok: false, error: new Error("natural-file.read-result-invalid") });
    reader.onerror = () => finish({ ok: false, error: reader.error ?? new Error("natural-file.read-failed") });
    reader.onabort = () => finish({ ok: false, error: signal.reason ?? new DOMException("The operation was aborted", "AbortError") });
    signal.addEventListener("abort", abort, { once: true });
    try {
      if (signal.aborted) abort(); else reader.readAsArrayBuffer(blob);
    } catch (error) {
      finish({ ok: false, error });
    }
  });
};

/** 🌊 Reads one Blob through bounded slices with exact byte progress and cancellation between every
 * slice. The final contiguous owner is admitted before any slice is read. */
export async function readBlobBytesBoundedV1(blob: Blob, control: Readonly<{
  signal: AbortSignal;
  maximumBytes: number;
  chunkBytes: number;
  progress: (completed: number, total: number) => void;
}>): Promise<Uint8Array> {
  if (!Number.isSafeInteger(control.maximumBytes) || control.maximumBytes < 0 || !Number.isSafeInteger(control.chunkBytes) || control.chunkBytes < 1)
    throw new Error("natural-file.read-control-invalid");
  if (!Number.isSafeInteger(blob.size) || blob.size < 0 || blob.size > control.maximumBytes) throw new Error("natural-file.read-limit");
  const cancelled = (): never => { throw control.signal.reason ?? new DOMException("The operation was aborted", "AbortError"); };
  if (control.signal.aborted) cancelled();
  const bytes = new Uint8Array(blob.size);
  control.progress(0, blob.size);
  for (let offset = 0; offset < blob.size; offset += control.chunkBytes) {
    if (control.signal.aborted) cancelled();
    const end = Math.min(blob.size, offset + control.chunkBytes);
    const slice = blob.slice(offset, end);
    try {
      const chunk = new Uint8Array(await readBlobSliceV1(slice, control.signal));
      if (chunk.byteLength !== end - offset) throw new Error("natural-file.read-truncated");
      bytes.set(chunk, offset);
    } catch (error) {
      if (control.signal.aborted) cancelled();
      throw new Error("natural-file.read-failed", { cause: error });
    }
    control.progress(end, blob.size);
  }
  return bytes;
}

/** 🔁️ The one-action-at-a-time callback shared by the `requestFileOpen`/`dispatchAction`/
 * `requestMediaFrames` `applyHostEffects` branches: dispatches `action` against the emitting program
 * instance and feeds its own `requestedEffects` back through `applyHostEffects` recursively. */
export type EffectDispatchOne = (action: string, args?: Record<string, unknown>) => Promise<void>;

/** 🎯️ Encodes a host-effect callback as the same fully scoped JSON `ActionInvocation` every other
 * renderer action uses; pack-base64 belongs inside the worker channel, never at `handleAction`'s edge. */
export function encodeEffectActionInvocation(baseSession: ActiveSession, action: string, args?: Record<string, unknown>): string {
  return JSON.stringify(hostEffectInvocationV1(effectDispatchScope(baseSession), [], action, args).invocation);
}

/** 🎯️ Encodes a recursively requested app command with its manifest owner. */
export function encodeEffectCommandInvocation(baseSession: ActiveSession, commandId: string, args?: Record<string, unknown>): string {
  return JSON.stringify(hostEffectInvocationV1(effectDispatchScope(baseSession), [commandId], commandId, args).invocation);
}

/** 🎯️ The `(plugin, app, mode, window kind, window instance)` a host effect re-dispatches in, read off
 * one live session — the shell's half of {@link hostEffectInvocationV1}. */
function effectDispatchScope(baseSession: ActiveSession): HostEffectDispatchScope {
  const windowKindId = baseSession.viewState.activeWindowKindId ?? baseSession.app.windowKinds[0]?.id ?? "";
  return {
    pluginId: baseSession.pluginId,
    appId: baseSession.app.id,
    modeId: baseSession.viewState.activeModeId ?? baseSession.app.defaultModeId ?? baseSession.app.modes[0]?.id ?? baseSession.app.id,
    windowKindId,
    windowInstanceId: baseSession.viewState.windowId ?? windowKindId,
  };
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
 * Host owners pass `resolvedTargetViewState`, the single admission that stamps both preferences.
 *
 * 🔗️ `provenance` is the ledger provenance of the input whose guest answer armed this follow-up
 * (`🎯️input-ledger` L2): its `causedBy` becomes the actor queue's causal `order` key, so a `dispatchAction`
 * re-arm dequeues BEFORE any input the user issued after its cause. Absent (`null` cause) the follow-up
 * is a root in its own right and carries no order. */
export function makeEffectDispatchOne(
  pluginEntry: LoadedProgramState,
  baseSession: ActiveSession,
  applyEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope?: UiDirtyScope) => Promise<void>,
  isCurrent: () => boolean,
  resolveViewState: (session: ActiveSession) => ViewModel,
  provenance?: Pick<InputProvenanceV1, "causedBy" | "windowId">,
): EffectDispatchOne {
  // 🔗️ A follow-up inherits its cause's key: `causalOrderKeyV1` over `{ inputSeq: cause, causedBy: cause }` IS the cause.
  const dispatch = provenance !== undefined && provenance.causedBy !== null ? { order: causalOrderKeyV1({ inputSeq: provenance.causedBy, causedBy: provenance.causedBy }) } : undefined;
  return async (action, args) => {
    if (!isCurrent()) return;
    const viewState = resolveViewState(baseSession);
    const isAppCommand = (baseSession.app.commands ?? []).some((command) => command.id === action);
    const response = isAppCommand && pluginEntry.handle.handleCommand
      ? await pluginEntry.handle.handleCommand(baseSession.instanceId, encodeEffectCommandInvocation(baseSession, action, args), viewState, dispatch)
      : await pluginEntry.handle.handleAction(baseSession.instanceId, encodeEffectActionInvocation(baseSession, action, args), viewState, dispatch);
    if (isCurrent()) await applyEffects(response.requestedEffects ?? [], { ...baseSession, viewState }, resolveUiDirtyScope(response.uiScope));
  };
}
//#endregion ShellHelpers
