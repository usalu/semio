/** 🆔️ Shell plumbing, action ids: the note world-navigation and shell-command action ids, the framework-reserved action id set, the undeclared-action diagnostic, the note shell-command action builder, the shell and manifest label texts and the tutorial-recording exclusion set. */
import { type ActionDescriptor, type LocalizedLabel, RECORD_TUTORIAL_ACTION_ID, SHELL_LOCALES, type ShellLocale, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, HOST_EVENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID } from "@semio-tech/framework";
import { resolveTranslationLabel, uiI18n, type UiTranslationKey } from "@semio-tech/ui-react";
import { resolveManifestLabel } from "../../🏷️labels/🟦️.tsx";

//#region ShellHelpers
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

/** 🧭️ Action ids the tutorial recorder never captures (see `onAction`'s recorder tap) — telemetry/chrome
 * noise a tutorial replay should never literally reproduce, or actions the director/recorder itself just
 * dispatched. Exported so it's independently testable. */
export const TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS: ReadonlySet<string> = new Set([NOTE_WORLD_NAVIGATION_ACTION_ID, NOTE_SHELL_COMMAND_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, RECORD_TUTORIAL_ACTION_ID]);
//#endregion ShellHelpers
