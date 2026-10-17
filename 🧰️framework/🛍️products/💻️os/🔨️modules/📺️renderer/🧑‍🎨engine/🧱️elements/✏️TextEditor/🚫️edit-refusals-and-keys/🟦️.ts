/** 🚫️ Text editor edit refusals and keys: the refusal of an edit, the applied and refusal outcomes, the read-only refusal reasons, the text editing key test and the document history chords. */
import type { TextEditorEchoStateV1 } from "../🖍️buffer-and-edit-helpers/🟦️.tsx";

//#region 🔖️TextEditorHost
//#region EditingHelpers
/** 🚫️ A refused edit never reaches the guest: it leaves the pending set, and once nothing is pending any more the host
 * re-applies the guest's own buffer (`resync`) — the typed text the guest did not take disappears together with the
 * shell's refusal notice instead of lingering as an edit that looks saved (ticket 26/09/23 F1: every keystroke after the
 * 64th of the trinity query was refused while the editor kept showing it). */
export function refuseTextEditorEditV1(state: TextEditorEchoStateV1, text: string): TextEditorEchoStateV1 & { readonly resync: boolean } {
  const index = state.pending.indexOf(text);
  if (index < 0) return { ...state, resync: false };
  const pending = [...state.pending.slice(0, index), ...state.pending.slice(index + 1)];
  return { pending, acknowledged: state.acknowledged, resync: pending.length === 0 };
}

/** 🧾️ The refusal reason of a dispatch outcome (the input ledger's `applied | refused | superseded`), or `null` when the
 * input was not refused. */
/** ✅️ Whether a dispatched action's typed outcome says the guest APPLIED it — its typed operation completed. */
export function outcomeApplied(outcome: unknown): boolean {
  return typeof outcome === "object" && outcome !== null && (outcome as { readonly kind?: unknown }).kind === "applied";
}

export function refusalReason(outcome: unknown): string | null {
  if (typeof outcome !== "object" || outcome === null) return null;
  const answer = outcome as { readonly kind?: unknown; readonly reason?: unknown };
  return answer.kind === "refused" ? String(answer.reason ?? "") : null;
}

/** 🔒️ Refusals that mean the window takes no text edits at all — a derived view (a compiled DSL, a script projection)
 * declares no `textEdit`, a viewer is read-only: the editor turns read-only instead of refusing every further keystroke
 * (ticket 26/09/23 F1: typing 158 keys into the dag/flow/sequence compiled windows raised 300 refusals). */
export const TEXT_EDITOR_READ_ONLY_REFUSALS: ReadonlySet<string> = new Set(["undeclared-action", "viewer-read-only"]);

/** ✍️ Whether a key edits the text (so a read-only editor swallows it): a printable key, Enter, Tab, Backspace, Delete. */
export function isTextEditingKey(event: { readonly key: string; readonly metaKey: boolean; readonly ctrlKey: boolean; readonly altKey: boolean }): boolean {
  if (event.key === "Enter" || event.key === "Tab" || event.key === "Backspace" || event.key === "Delete") return !event.metaKey && !event.ctrlKey;
  return event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey;
}

/** ⏪️ The document history verb a key chord asks for inside the editor (`mod+z` undo, `mod+shift+z` / `ctrl+y` redo). The
 * shell's own chords stand down while a text field has focus — right for a native field, which undoes itself — but the
 * editor's input sink is not a field of its own: its text lives in the document, so its undo is the document's
 * (ticket 26/09/23 F1: Cmd+Z in the writer and query editors did nothing). */
export function documentHistoryChord(event: { readonly key: string; readonly metaKey: boolean; readonly ctrlKey: boolean; readonly shiftKey: boolean; readonly altKey: boolean }): "undo" | "redo" | null {
  if (event.altKey || !(event.metaKey || event.ctrlKey)) return null;
  const key = event.key.toLowerCase();
  if (key === "z") return event.shiftKey ? "redo" : "undo";
  if (key === "y" && event.ctrlKey && !event.metaKey) return "redo";
  return null;
}
//#endregion EditingHelpers
//#endregion 🔖️TextEditorHost
