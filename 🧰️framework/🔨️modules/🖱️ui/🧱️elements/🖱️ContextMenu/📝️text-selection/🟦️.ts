import { type UiLabel, uiDataLabel } from "../../🎗️UiLabel/🟦️.tsx";
import { formatKeybindingShortcut } from "../../../🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts";
import type { ContextMenuItem } from "../🎨️item-render/🟦️.tsx";

/** 📋️ Non-collapsed DOM text selection string, or empty. */
export function readDomTextSelection(): string {
  if (typeof window === "undefined") return "";
  const selection = window.getSelection();
  if (!selection || selection.isCollapsed || selection.rangeCount < 1) return "";
  return selection.toString();
}

/** 📋️ True when `target` intersects the current non-empty DOM text selection. */
export function isPointerEventOnDomTextSelection(target: EventTarget | null): boolean {
  if (typeof window === "undefined") return false;
  if (!(target instanceof Node)) return false;
  const selection = window.getSelection();
  if (!selection || selection.isCollapsed || selection.rangeCount < 1) return false;
  if (!selection.toString()) return false;
  try {
    return selection.getRangeAt(0).intersectsNode(target);
  } catch {
    return false;
  }
}

/** ✏️ True when the event target (or its editable ancestor) accepts cut/paste. */
export function isDomTextEditableTarget(target: EventTarget | null): boolean {
  if (target instanceof Text) return isDomTextEditableTarget(target.parentElement);
  if (!(target instanceof Element)) return false;
  if (target instanceof HTMLTextAreaElement) return !target.readOnly && !target.disabled;
  if (target instanceof HTMLInputElement) {
    const kind = (target.type || "text").toLowerCase();
    if (kind === "button" || kind === "checkbox" || kind === "radio" || kind === "file" || kind === "range" || kind === "color" || kind === "hidden") return false;
    return !target.readOnly && !target.disabled;
  }
  if ((target as HTMLElement).isContentEditable) return true;
  return Boolean(target.closest('[contenteditable="true"], [role="textbox"]'));
}

export interface TextSelectionContextMenuLabels {
  readonly cut: UiLabel;
  readonly copy: UiLabel;
  readonly paste: UiLabel;
  readonly selectAll: UiLabel;
}

export interface TextSelectionContextMenuActions {
  readonly cut: () => void;
  readonly copy: () => void;
  readonly paste: () => void;
  readonly selectAll: () => void;
}

/** 📋️ Builds Cut/Copy/Paste/Select All rows for a DOM text selection. */
export function buildTextSelectionContextMenuItems(input: { readonly editable: boolean; readonly hasSelection: boolean }, labels: TextSelectionContextMenuLabels, actions: TextSelectionContextMenuActions): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  if (input.editable) {
    items.push({ id: "text-cut", label: labels.cut, icon: "scissors", shortcut: formatKeybindingShortcut("mod+x"), disabled: !input.hasSelection, onSelect: () => actions.cut() });
  }
  items.push({ id: "text-copy", label: labels.copy, icon: "copy", shortcut: formatKeybindingShortcut("mod+c"), disabled: !input.hasSelection, onSelect: () => actions.copy() });
  if (input.editable) {
    items.push({ id: "text-paste", label: labels.paste, icon: "clipboard", shortcut: formatKeybindingShortcut("mod+v"), onSelect: () => actions.paste() });
  }
  items.push({ id: "text-sep", label: uiDataLabel(""), separator: true });
  items.push({ id: "text-select-all", label: labels.selectAll, icon: "select-all", shortcut: formatKeybindingShortcut("mod+a"), onSelect: () => actions.selectAll() });
  return items;
}

/**
 * 📋️ Copies a DOM text selection to the clipboard — `captured` when the caller took a reading,
 * otherwise whatever is selected right now.
 *
 * 🎯️ `captured` is not a convenience. An open menu HOLDS FOCUS (the active row is the focused
 * element, which is what lets a screen reader follow an arrow walk), and taking focus collapses the
 * document text selection the menu was opened on — so by the time `Copy` is activated,
 * {@link readDomTextSelection} answers the empty string and nothing reaches the clipboard. The
 * selection is therefore read when the menu OPENS and handed to the action.
 **/
export async function copyDomTextSelection(captured?: string): Promise<void> {
  const text = captured ?? readDomTextSelection();
  if (!text || typeof navigator === "undefined" || !navigator.clipboard?.writeText) return;
  await navigator.clipboard.writeText(text);
}

/** ✂️ Cuts a DOM text selection when the target is editable; `captured` is the reading taken when the menu opened (see {@link copyDomTextSelection}). */
export async function cutDomTextSelection(target: EventTarget | null, captured?: string): Promise<void> {
  await copyDomTextSelection(captured);
  if (!isDomTextEditableTarget(target)) return;
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
    const start = target.selectionStart ?? 0;
    const end = target.selectionEnd ?? 0;
    if (start === end) return;
    const value = target.value;
    target.value = `${value.slice(0, start)}${value.slice(end)}`;
    target.setSelectionRange(start, start);
    target.dispatchEvent(new Event("input", { bubbles: true }));
    return;
  }
  if (typeof document !== "undefined" && typeof document.execCommand === "function") {
    document.execCommand("delete");
    return;
  }
  window.getSelection()?.deleteFromDocument();
}

/** 📋️ Pastes clipboard text into the editable target (or active element). */
export async function pasteDomTextSelection(target: EventTarget | null): Promise<void> {
  if (typeof navigator === "undefined" || !navigator.clipboard?.readText) return;
  const text = await navigator.clipboard.readText();
  const focus = (target instanceof Element ? target : null) ?? (typeof document !== "undefined" ? document.activeElement : null);
  if (focus instanceof HTMLInputElement || focus instanceof HTMLTextAreaElement) {
    if (focus.readOnly || focus.disabled) return;
    const start = focus.selectionStart ?? focus.value.length;
    const end = focus.selectionEnd ?? focus.value.length;
    focus.value = `${focus.value.slice(0, start)}${text}${focus.value.slice(end)}`;
    const caret = start + text.length;
    focus.setSelectionRange(caret, caret);
    focus.dispatchEvent(new Event("input", { bubbles: true }));
    return;
  }
  if (typeof document !== "undefined" && typeof document.execCommand === "function") {
    document.execCommand("insertText", false, text);
  }
}

/** 🅰️ Selects all text in the editable focus target, or the current selection's root element. */
export function selectAllDomText(target: EventTarget | null): void {
  const focus = (target instanceof Element ? target : target instanceof Text ? target.parentElement : null) ?? (typeof document !== "undefined" ? document.activeElement : null);
  if (focus instanceof HTMLInputElement || focus instanceof HTMLTextAreaElement) {
    focus.select();
    return;
  }
  if (typeof window === "undefined") return;
  const selection = window.getSelection();
  if (!selection) return;
  if (focus instanceof HTMLElement && focus.isContentEditable) {
    const range = document.createRange();
    range.selectNodeContents(focus);
    selection.removeAllRanges();
    selection.addRange(range);
    return;
  }
  if (selection.rangeCount < 1) return;
  const ancestor = selection.getRangeAt(0).commonAncestorContainer;
  const element = ancestor instanceof Element ? ancestor : ancestor.parentElement;
  if (!element) return;
  const range = document.createRange();
  range.selectNodeContents(element);
  selection.removeAllRanges();
  selection.addRange(range);
}
