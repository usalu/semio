/** ⌨️ Window panes, keyboard chords: the editable-target and keyboard-ownership guards, the canonical chord tokens, the chord matcher and the keybinding intent. */
import { type ActionDefinition, effectiveActionArgs, unresolvedActionArgs } from "@semio-tech/framework";
import { actionRequiresStagedForm } from "../📝️staged-args/🟦️.tsx";

//#region ShellHelpers
//#region 🧰️WindowActionPane
/** 🧰️ The decision a bound hotkey makes for one action (P4). */
/** ⌨️ True when a keydown's target is a text-editing surface (input/textarea/select/contenteditable) — hotkeys never fire while the user is typing. */
export function isEditableEventTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  if (target instanceof HTMLElement && target.isContentEditable) return true;
  return target.closest('input,textarea,select,[contenteditable]:not([contenteditable="false"]),[role="textbox"],[role="searchbox"]') !== null;
}

/** 🎹️ Focused widgets own ordinary keys; text editors also retain modified editing chords. */
export function controlOwnsKeyboardEvent(event: KeyboardEventLike & { readonly target: EventTarget | null; readonly isComposing?: boolean }): boolean {
  if (event.isComposing || isEditableEventTarget(event.target)) return true;
  if (event.ctrlKey || event.metaKey || event.altKey || !(event.target instanceof Element)) return false;
  return event.target.closest('button,a[href],summary,audio[controls],video[controls],[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="combobox"],[role="listbox"],[role="option"],[role="menu"],[role="menuitem"],[role="menuitemcheckbox"],[role="menuitemradio"],[role="slider"],[role="spinbutton"],[role="tab"],[role="tree"],[role="treeitem"],[role="gridcell"]') !== null;
}

/** 🩹️ Structural, not `React.KeyboardEvent`/`globalThis.KeyboardEvent` specifically —
 * `keyboardEventMatchesChord` only ever reads the five fields below, which both shapes carry
 * identically, and its real callers legitimately hold either (a native `useShellKeydown` listener vs a
 * React `onKeyDown` handler). */
export type KeyboardEventLike = { readonly key: string; readonly ctrlKey: boolean; readonly metaKey: boolean; readonly shiftKey: boolean; readonly altKey: boolean };

/** ⌨️ Punctuation keys whose DOM `event.key` is a single character — chord declarations MUST use the character, never these names. */
export const CHORD_KEY_NAME_ALIASES: Readonly<Record<string, string>> = {
  period: ".",
  comma: ",",
  slash: "/",
  minus: "-",
  equal: "=",
  semicolon: ";",
  quote: "'",
  backquote: "`",
  bracketleft: "[",
  bracketright: "]",
  backslash: "\\",
};

/** ⌨️ The key token `ShellHost` compares against `event.key` for one chord's last `+` segment. */
export function canonicalChordKeyToken(token: string): string {
  const lower = token.toLowerCase();
  return CHORD_KEY_NAME_ALIASES[lower] ?? token;
}

/** ⌨️ False when a chord's key segment spells punctuation as a name (`period`) instead of the character (`.`). */
export function chordKeyTokenIsCanonical(token: string): boolean {
  const lower = token.toLowerCase();
  if (lower in CHORD_KEY_NAME_ALIASES && token !== CHORD_KEY_NAME_ALIASES[lower]) return false;
  return true;
}

/** ⌨️ True when every non-modifier segment of a chord uses canonical key tokens (no `mod+period` dead strings). */
export function chordUsesCanonicalKeyTokens(chord: string): boolean {
  const modifierTokens = new Set(["mod", "ctrl", "meta", "shift", "alt"]);
  const parts = chord.split("+").map((part) => part.trim()).filter(Boolean);
  const key = parts[parts.length - 1] ?? "";
  if (modifierTokens.has(key.toLowerCase())) return true;
  return chordKeyTokenIsCanonical(key);
}

/** ⌨️ True when a keydown event matches one `+`-joined chord (e.g. `"mod+shift+z"`), where `mod` accepts either ctrl or meta. */
export function keyboardEventMatchesChord(event: KeyboardEventLike, chord: string): boolean {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsMod = parts.includes("mod");
  const needsCtrl = parts.includes("ctrl") || parts.includes("control");
  const needsMeta = parts.includes("meta");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  if (needsMod && !needsCtrl && !needsMeta) {
    if (event.ctrlKey === event.metaKey) return false;
  } else if (event.ctrlKey !== needsCtrl || event.metaKey !== needsMeta) {
    return false;
  }
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key.toLowerCase();
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
//#endregion 🧰️WindowActionPane
//#endregion ShellHelpers
