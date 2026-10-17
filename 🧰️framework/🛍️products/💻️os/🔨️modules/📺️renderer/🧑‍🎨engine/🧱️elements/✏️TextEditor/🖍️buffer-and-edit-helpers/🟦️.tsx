/** 🖍️ Text editor buffer and edit helpers: the token class colors, the highlighted buffer, the host label, the multi-span replace, the line range and identifier prefix helpers and the local echo reconciliation. */
import { type ReactNode } from "react";
import { type UiTranslationKey } from "@semio-tech/ui-react";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { GrammarToken, SpanRange } from "../🏷️editor-types/🟦️.ts";

//#region 🔖️TextEditorHost
const TOKEN_CLASS_COLORS: Record<string, string> = {
  keyword: "text-info",
  string: "text-success",
  number: "text-warning",
  operator: "text-primary",
  ident: "text-foreground",
};

//#region HighlightedBuffer
export function HighlightedBuffer({ buffer, tokens }: { readonly buffer: string; readonly tokens: readonly GrammarToken[] }) {
  if (tokens.length === 0) {
    return <span className="whitespace-pre-wrap font-mono text-xs text-foreground">{buffer}</span>;
  }
  const parts: ReactNode[] = [];
  let cursor = 0;
  for (const token of tokens) {
    if (token.start > cursor) parts.push(<span key={`plain-${cursor}`}>{buffer.slice(cursor, token.start)}</span>);
    const color = TOKEN_CLASS_COLORS[token.class] ?? "text-foreground";
    parts.push(
      <span key={`token-${token.start}-${token.end}`} className={`font-mono text-xs ${color}`}>
        {buffer.slice(token.start, token.end)}
      </span>,
    );
    cursor = Math.max(cursor, token.end);
  }
  if (cursor < buffer.length) parts.push(<span key={`tail-${cursor}`}>{buffer.slice(cursor)}</span>);
  return <div className="pointer-events-none absolute inset-0 overflow-hidden whitespace-pre-wrap p-3">{parts}</div>;
}

//#endregion HighlightedBuffer
//#region EditingHelpers
/** 🌐️ Resolves a translation key outside of component render (e.g. text-editor context menu helpers) — an alias of {@link shellLabel} scoped to this region.
 * Exported (unlike the rest of this helper set) — a still-barrel-resident block-builder region references it unqualified. */
export function hostLabel(key: UiTranslationKey): string {
  return shellLabel(key);
}

/** ✂️ Language-agnostic multi-span rename preview: replaces every span with `nextName`, remapping spans left-to-right. */
export function multiSpanReplace(text: string, occurrences: readonly SpanRange[], nextName: string): { readonly text: string; readonly occurrences: readonly SpanRange[] } {
  const sorted = [...occurrences].sort((a, b) => b.start - a.start);
  let out = text;
  const nextOccurrences: SpanRange[] = [];
  for (const occ of sorted) {
    out = `${out.slice(0, occ.start)}${nextName}${out.slice(occ.end)}`;
    nextOccurrences.unshift({ start: occ.start, end: occ.start + nextName.length });
  }
  return { text: out, occurrences: nextOccurrences };
}

export function lineRangeAt(text: string, offset: number): SpanRange {
  const start = text.lastIndexOf("\n", Math.max(0, offset - 1)) + 1;
  const nextNewline = text.indexOf("\n", offset);
  const end = nextNewline === -1 ? text.length : nextNewline;
  return { start, end };
}

export function identifierPrefixStart(text: string, caret: number): number {
  let start = caret;
  while (start > 0 && /[A-Za-z0-9_]/.test(text[start - 1] ?? "")) start -= 1;
  return start;
}

/** 🔁️ What one editor host knows about its own edits in flight: the texts it sent and has not seen echoed yet (oldest
 * first) and the last text an echo acknowledged. */
export type TextEditorEchoStateV1 = { readonly pending: readonly string[]; readonly acknowledged: string | null };

/** 🔁️ The host's optimistic local echo (fixture `🧫️fixtures/🔁️local-echo/🔣️.json`). While this editor types, its OWN text and
 * caret are the newest state: a scene that only echoes what the host delivered — a buffer equal to a pending sent text (it
 * acknowledges that text and every older one) or to the last acknowledged text (the guest re-publishing a state it already
 * echoed, e.g. when a job finishes) — applies neither its buffer nor its selection, because both lag the editor (ticket
 * 26/09/23 F1: an older echo reverted typing still in flight, and an echoed selection threw the caret back mid-word, which
 * scrambled a fast run). Any other buffer is an external change (undo, a collaborator, a formatter): the host applies its
 * text and selection at once and drops every pending send. */
export function reconcileTextEditorEchoV1(state: TextEditorEchoStateV1, buffer: string): TextEditorEchoStateV1 & { readonly external: boolean } {
  const index = state.pending.indexOf(buffer);
  if (index >= 0) return { external: false, pending: state.pending.slice(index + 1), acknowledged: buffer };
  if (buffer === state.acknowledged) return { external: false, pending: state.pending, acknowledged: state.acknowledged };
  return { external: true, pending: [], acknowledged: buffer };
}

/** 🕳️ The hidden textarea is the editor's INPUT SINK (focus, IME, clipboard): the wasm session owns the text and the
 * caret, so the textarea's own value changes are never an edit — typing, compositions, pastes and text inserted without a
 * key of its own (a dead key, the emoji picker, dictation: `beforeinput` `insertText`) arrive as key, composition,
 * clipboard and input events and are applied at the session's caret. */
export function ignoreInputSinkChange(): void {}
//#endregion EditingHelpers
//#endregion 🔖️TextEditorHost
