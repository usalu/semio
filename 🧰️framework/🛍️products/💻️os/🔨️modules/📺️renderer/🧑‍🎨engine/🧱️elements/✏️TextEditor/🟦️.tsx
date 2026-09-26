// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/✏️TextEditor/component.tsx
/** @emoji 📝️ `✏️TextEditor` — the text/code document scene host: wasm editor-engine canvas surface
 * (grammar highlighting, hover tokens, F2 rename with multi-span preview, alt-click completions),
 * its context menu and keyboard-editing wiring, and the SSR-safe `Textarea`-based fallback. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-canvas-react-renderer";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import { cn, ContextMenuController, glassClass, Textarea, useCanvasAppearanceSync, useLabel, useShellScopeOptional, type ContextMenuItem, type UiTranslationKey } from "@semio-tech/ui-react";
import { textEditorActions, type ActionDescriptor, type ComponentSceneHostProps, type ContextMenuItemSpec, type PluginContextMenuRequest, type TextEditorScene } from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework-os";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../🗣️Interpreter/🟦️.tsx";
import { mapContextMenuSpecs } from "../🌐️World3dHost/🟦️.tsx";
import { useClient } from "../🕸️NodeGraph/🟦️.tsx";
import { createEditorSession, type EditorWasmSession } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { createCoalescingActionDispatcher, shellLabel } from "../🛠️ShellHelpers/🟦️.tsx";
import { useAppKeybindingsByActionId } from "../🏛️ShellHost/🟦️.tsx";
import { CanvasPresenceOverlayV1, useLocalPresenceActorIdV1 } from "../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, publishLocalPresenceWindowViewV1, clearLocalPresenceWindowViewV1, publishLocalActiveToolV1 } from "../👕️canvas-presence/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️TextEditorHost
//#region Types
type GrammarToken = { readonly class: string; readonly start: number; readonly end: number };

type EditorDiagnostic = { readonly start: number; readonly end: number; readonly severity?: string; readonly message: string };

type FrameworkEditorSession = EditorWasmSession;

type SpanRange = { readonly start: number; readonly end: number };

type CompletionItem = { readonly label: string; readonly detail?: string; readonly insertText?: string };

type RenameInfo = { readonly name: string; readonly occurrences: readonly SpanRange[] };

type RenameDraft = { readonly occurrences: readonly SpanRange[]; readonly text: string };

type PickTarget = { readonly domain: string; readonly id: string; readonly generality?: number; readonly label: string };
//#endregion Types

const TOKEN_CLASS_COLORS: Record<string, string> = {
  keyword: "text-sky-400",
  string: "text-emerald-400",
  number: "text-amber-400",
  operator: "text-violet-400",
  ident: "text-foreground",
};

//#region HighlightedBuffer
function HighlightedBuffer({ buffer, tokens }: { readonly buffer: string; readonly tokens: readonly GrammarToken[] }) {
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

function identifierPrefixStart(text: string, caret: number): number {
  let start = caret;
  while (start > 0 && /[A-Za-z0-9_]/.test(text[start - 1] ?? "")) start -= 1;
  return start;
}

/** @emoji 🔁️ What one editor host knows about its own edits in flight: the texts it sent and has not seen echoed yet (oldest
 * first) and the last text an echo acknowledged. */
export type TextEditorEchoStateV1 = { readonly pending: readonly string[]; readonly acknowledged: string | null };

/** @emoji 🔁️ The host's optimistic local echo (fixture `🧫️fixtures/🔁️local-echo/🔣️.json`). While this editor types, its OWN text and
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

/** @emoji 🕳️ The hidden textarea is the editor's INPUT SINK (focus, IME, clipboard): the wasm session owns the text and the
 * caret, so the textarea's own value changes are never an edit — typing, compositions, pastes and text inserted without a
 * key of its own (a dead key, the emoji picker, dictation: `beforeinput` `insertText`) arrive as key, composition,
 * clipboard and input events and are applied at the session's caret. */
function ignoreInputSinkChange(): void {}

/** @emoji 🚫️ A refused edit never reaches the guest: it leaves the pending set, and once nothing is pending any more the host
 * re-applies the guest's own buffer (`resync`) — the typed text the guest did not take disappears together with the
 * shell's refusal notice instead of lingering as an edit that looks saved (ticket 26/09/23 F1: every keystroke after the
 * 64th of the trinity query was refused while the editor kept showing it). */
export function refuseTextEditorEditV1(state: TextEditorEchoStateV1, text: string): TextEditorEchoStateV1 & { readonly resync: boolean } {
  const index = state.pending.indexOf(text);
  if (index < 0) return { ...state, resync: false };
  const pending = [...state.pending.slice(0, index), ...state.pending.slice(index + 1)];
  return { pending, acknowledged: state.acknowledged, resync: pending.length === 0 };
}

/** @emoji 🧾️ The refusal reason of a dispatch outcome (the input ledger's `applied | refused | superseded`), or `null` when the
 * input was not refused. */
function refusalReason(outcome: unknown): string | null {
  if (typeof outcome !== "object" || outcome === null) return null;
  const answer = outcome as { readonly kind?: unknown; readonly reason?: unknown };
  return answer.kind === "refused" ? String(answer.reason ?? "") : null;
}

/** @emoji 🔒️ Refusals that mean the window takes no text edits at all — a derived view (a compiled DSL, a script projection)
 * declares no `textEdit`, a viewer is read-only: the editor turns read-only instead of refusing every further keystroke
 * (ticket 26/09/23 F1: typing 158 keys into the dag/flow/sequence compiled windows raised 300 refusals). */
export const TEXT_EDITOR_READ_ONLY_REFUSALS: ReadonlySet<string> = new Set(["undeclared-action", "viewer-read-only"]);

/** @emoji ✍️ Whether a key edits the text (so a read-only editor swallows it): a printable key, Enter, Tab, Backspace, Delete. */
function isTextEditingKey(event: { readonly key: string; readonly metaKey: boolean; readonly ctrlKey: boolean; readonly altKey: boolean }): boolean {
  if (event.key === "Enter" || event.key === "Tab" || event.key === "Backspace" || event.key === "Delete") return !event.metaKey && !event.ctrlKey;
  return event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey;
}

/** @emoji ⏪️ The document history verb a key chord asks for inside the editor (`mod+z` undo, `mod+shift+z` / `ctrl+y` redo). The
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

/** @emoji 📮️ What the editor owes the guest: its latest text and selection (byte offsets of the session). */
type TextEditorOutboxV1 = { readonly text: string; readonly start: number; readonly end: number };

/** @emoji 🧮️ Most edits one editor keeps in flight before an echo acknowledges them. */
const TEXT_EDITOR_PENDING_EDIT_LIMIT = 256;

/** @emoji ✂️ A scene without the fields an echo of the editor's own state must not apply — its buffer and its selection. */
function sceneWithoutEchoedText(scene: TextEditorScene): Record<string, unknown> {
  return Object.fromEntries(Object.entries(scene).filter(([key]) => key !== "buffer" && key !== "selectionJson"));
}

function parseJsonOr<T>(json: string | undefined, fallback: T): T {
  if (!json) return fallback;
  try {
    return parseSceneJsonField<T>(json);
  } catch {
    return fallback;
  }
}

//#endregion EditingHelpers

//#region WasmEditorSurface
function WasmEditorSurface({
  scene,
  controllerId,
  surfaceId,
  onAction,
  requestContextMenu,
}: {
  readonly scene: TextEditorScene;
  readonly controllerId: string;
  readonly surfaceId: string;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
}) {
  const sessionRef = useRef<FrameworkEditorSession | null>(null);
  const renameActiveRef = useRef(false);
  const lastHoverRangeRef = useRef<SpanRange | null>(null);
  const echoStateRef = useRef<TextEditorEchoStateV1>({ pending: [], acknowledged: null });
  const reconciledRef = useRef<{ readonly scene: TextEditorScene; readonly pack: Uint8Array } | null>(null);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );
  const textEditorMenuKeysByActionId = useAppKeybindingsByActionId();
  const shellContextMenuFallback = useShellContextMenuFallback();

  const syncSessionRef = useRef<(resync?: boolean) => void>(() => {});
  const readOnlyRef = useRef(false);
  const [readOnly, setReadOnly] = useState(false);
  const selectionUndeclaredRef = useRef(false);
  /** 📮️ ONE round trip in flight per editor, latest state wins: a typed run is delivered as the newest full text (plus the
   * selection that goes with it) whenever the previous delivery settled, never one `textEdit` + one `textSelect` per key —
   * sustained typing at 40 keys/s filled the per-actor command queue (`queue-full`, > 256 pending turns) and dropped keys
   * (ticket 26/09/23 F1). The texts that do go out are the ones the echo reconciliation waits for. */
  const deliver = useMemo(
    () =>
      createCoalescingActionDispatcher<TextEditorOutboxV1>(
        async (next) => {
          const echo = echoStateRef.current;
          const guestHasText = echo.pending.length > 0 ? echo.pending.at(-1) === next.text : echo.acknowledged === next.text;
          if (!guestHasText && !readOnlyRef.current) {
            echoStateRef.current = { ...echoStateRef.current, pending: [...echoStateRef.current.pending, next.text].slice(-TEXT_EDITOR_PENDING_EDIT_LIMIT) };
            const reason = refusalReason(await onAction({ controllerId, action: textEditorActions.edit, args: { surfaceId, text: next.text } }));
            if (reason !== null) {
              if (TEXT_EDITOR_READ_ONLY_REFUSALS.has(reason)) {
                readOnlyRef.current = true;
                setReadOnly(true);
              }
              const refused = refuseTextEditorEditV1(echoStateRef.current, next.text);
              echoStateRef.current = { pending: refused.pending, acknowledged: refused.acknowledged };
              if (refused.resync) syncSessionRef.current(true);
              return;
            }
          }
          if (selectionUndeclaredRef.current) return;
          const reason = refusalReason(await onAction({ controllerId, action: textEditorActions.select, args: { surfaceId, start: next.start, end: next.end } }));
          if (reason === "undeclared-action") selectionUndeclaredRef.current = true;
        },
        (a, b) => a.text === b.text && a.start === b.start && a.end === b.end,
      ),
    [controllerId, onAction, surfaceId],
  );
  const sendEdit = useCallback(
    (text: string) => {
      const session = sessionRef.current;
      deliver({ text, start: session?.anchor() ?? text.length, end: session?.caret() ?? text.length });
    },
    [deliver],
  );

  const syncSession = useCallback(
    (resync = false) => {
      if (renameActiveRef.current) return;
      if (resync) {
        echoStateRef.current = { pending: [], acknowledged: scene.buffer };
        reconciledRef.current = { scene, pack: new Uint8Array(encodePackValue(scene)) };
      } else if (reconciledRef.current?.scene !== scene) {
        const echo = reconcileTextEditorEchoV1(echoStateRef.current, scene.buffer);
        echoStateRef.current = { pending: echo.pending, acknowledged: echo.acknowledged };
        reconciledRef.current = { scene, pack: new Uint8Array(encodePackValue(echo.external ? scene : sceneWithoutEchoedText(scene))) };
      }
      try {
        sessionRef.current?.syncFromScenePack?.(reconciledRef.current.pack);
        sessionRef.current?.renderFrame();
      } catch (error) {
      }
    },
    [scene],
  );
  syncSessionRef.current = syncSession;

  const [sessionEpoch, setSessionEpoch] = useState(0);

  useEffect(() => {
    syncSession();
    // sessionEpoch: re-sync immediately after GraphWasmCanvas (re)attaches a session (e.g. the stub -> real wasm swap),
    // since the attach lifecycle is independent of scene changes and a ref update alone would not otherwise re-trigger this effect.
  }, [syncSession, sessionEpoch]);

  const wasmEditorSurfaceShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      syncSessionCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
    },
    true,
    wasmEditorSurfaceShellScope?.rootRef.current ?? undefined,
  );

  const emitSelection = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    deliver({ text: session.text(), start: session.anchor(), end: session.caret() });
  }, [deliver]);

  const sinkRef = useRef<HTMLTextAreaElement>(null);
  useEffect(() => {
    const sink = sinkRef.current;
    if (!sink) return;
    const onBeforeInput = (event: InputEvent) => {
      if (event.inputType !== "insertText" && event.inputType !== "insertFromDrop") return;
      const data = event.data ?? event.dataTransfer?.getData("text/plain") ?? "";
      event.preventDefault();
      const session = sessionRef.current;
      if (!session || readOnlyRef.current || data.length === 0) return;
      session.insertText(data);
      sendEdit(session.text());
      session.renderFrame();
      emitSelection();
    };
    sink.addEventListener("beforeinput", onBeforeInput);
    return () => sink.removeEventListener("beforeinput", onBeforeInput);
  }, [emitSelection, sendEdit]);

  const [wasmSession, setWasmSession] = useState<FrameworkEditorSession | null>(null);
  const [renameDraft, setRenameDraft] = useState<RenameDraft | null>(null);
  const [renamePosition, setRenamePosition] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const [completionsOpen, setCompletionsOpen] = useState(false);
  const [completionIndex, setCompletionIndex] = useState(0);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly position: { readonly x: number; readonly y: number } }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.editor");

  const completions = useMemo(() => parseJsonOr<readonly CompletionItem[]>(scene.completionsJson, []), [scene.completionsJson]);
  const renameInfo = useMemo(() => parseJsonOr<RenameInfo | null>(scene.renameJson, null), [scene.renameJson]);
  const newlineGates = useMemo(() => (scene.newlineGatesJson ? new Set(parseJsonOr<readonly number[]>(scene.newlineGatesJson, [])) : null), [scene.newlineGatesJson]);

  useEffect(() => {
    if (completions.length === 0 && completionsOpen) setCompletionsOpen(false);
    if (completionIndex >= completions.length) setCompletionIndex(0);
  }, [completions, completionsOpen, completionIndex]);

  useEffect(() => {
    let cancelled = false;
    void createEditorSession().then((session) => {
      if (!cancelled) setWasmSession(session);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const sessionFactory = useCallback(() => {
    // Empty pkg stub (`export class EditorSession {}`) must not replace the no-op host fallback.
    if (wasmSession && typeof wasmSession.attachCanvas === "function") return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      syncFromSceneJson: () => {},
      syncFromScenePack: () => {},
      setText: () => {},
      text: () => scene.buffer,
      caret: () => scene.buffer.length,
      anchor: () => 0,
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      wheelScrollScreen: () => {},
      insertText: () => {},
      backspace: () => {},
      deleteForward: () => {},
      selectAll: () => {},
      replaceSelection: () => {},
      selectionText: () => "",
      hoverTokenRangeJson: () => "null",
      setHoverRange: () => {},
      cameraJson: () => "{}",
      setCanvasThemeJson: () => {},
      moveLeft: () => {},
      moveRight: () => {},
      moveUp: () => {},
      moveDown: () => {},
      moveLineStart: () => {},
      moveLineEnd: () => {},
      tabInsertText: () => "  ",
      setSelectionRange: () => {},
      selectSpanAt: () => {},
      selectSpanAtScreen: () => {},
      pickTargetsAtScreenJson: () => "[]",
      caretWorldJson: () => "null",
      worldToScreenJson: () => "null",
      setSelectionOccurrencesJson: () => {},
      setExtraCaretsJson: () => {},
      setCaretVisible: () => {},
    } satisfies FrameworkEditorSession;
    // Deliberately omits scene.buffer: GraphWasmCanvas re-attaches the GPU canvas whenever sessionFactory's
    // identity changes, so this must stay stable across content edits — only the wasmSession load transition matters.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wasmSession]);

  const caretScreenPosition = useCallback((session: FrameworkEditorSession): { readonly x: number; readonly y: number } | null => {
    try {
      const world = JSON.parse(session.caretWorldJson()) as { readonly x?: number; readonly y?: number } | null;
      if (world?.x == null || world?.y == null) return null;
      const screen = JSON.parse(session.worldToScreenJson(world.x, world.y)) as { readonly x?: number; readonly y?: number } | null;
      if (screen?.x == null || screen?.y == null) return null;
      return { x: screen.x, y: screen.y };
    } catch {
      return null;
    }
  }, []);

  const openCompletions = useCallback(() => {
    if (completions.length === 0) return;
    setCompletionsOpen(true);
    setCompletionIndex(0);
  }, [completions.length]);

  const applyCompletion = useCallback(
    (item: CompletionItem) => {
      const session = sessionRef.current;
      if (!session) return;
      const text = session.text();
      const caret = session.caret();
      const prefixStart = identifierPrefixStart(text, caret);
      session.setSelectionRange(prefixStart, caret);
      session.replaceSelection(item.insertText ?? item.label);
      sendEdit(session.text());
      session.renderFrame();
      emitSelection();
      setCompletionsOpen(false);
    },
    [emitSelection, sendEdit],
  );

  const startRename = useCallback(() => {
    const session = sessionRef.current;
    if (!session || !renameInfo) return;
    renameActiveRef.current = true;
    setRenameDraft({ occurrences: renameInfo.occurrences, text: renameInfo.name });
    setRenamePosition(caretScreenPosition(session));
  }, [renameInfo, caretScreenPosition]);

  const updateRenamePreview = useCallback(
    (nextText: string) => {
      const session = sessionRef.current;
      if (!session || !renameDraft) return;
      const preview = multiSpanReplace(scene.buffer, renameDraft.occurrences, nextText);
      session.setText(preview.text);
      session.setSelectionOccurrencesJson(JSON.stringify(preview.occurrences));
      session.setExtraCaretsJson(JSON.stringify(preview.occurrences.map((occ) => occ.start)));
      session.renderFrame();
      setRenameDraft({ ...renameDraft, text: nextText });
    },
    [renameDraft, scene.buffer],
  );

  const commitRename = useCallback(() => {
    if (!renameDraft) return;
    dispatch(textEditorActions.commitRename, { occurrences: renameDraft.occurrences, text: renameDraft.text });
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [dispatch, renameDraft]);

  const cancelRename = useCallback(() => {
    const session = sessionRef.current;
    if (session) {
      session.setText(scene.buffer);
      session.renderFrame();
    }
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [scene.buffer]);

  const dismissContextMenu = useCallback(() => setContextMenu(null), []);

  // Stable identity: GraphWasmCanvas re-attaches the GPU canvas whenever this prop's identity changes,
  // so it must not close over anything that changes per scene update (see sessionEpoch above for re-sync).
  const onSessionReady = useCallback((session: GraphWasmSession) => {
    sessionRef.current = session as FrameworkEditorSession;
    syncSessionCanvasTheme(sessionRef.current);
    setSessionEpoch((epoch) => epoch + 1);
  }, []);

  return (
    <div className="relative min-h-0 flex-1">
      <GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <div
        className="absolute inset-0"
        onPointerDown={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          if (event.detail >= 2) {
            session.selectSpanAtScreen(sx, sy);
            session.renderFrame();
            emitSelection();
            return;
          }
          session.pointerDownScreen(sx, sy, event.button);
          session.renderFrame();
          emitSelection();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          try {
            const hover = JSON.parse(session.hoverTokenRangeJson()) as SpanRange | null;
            const changed = (hover?.start ?? null) !== (lastHoverRangeRef.current?.start ?? null) || (hover?.end ?? null) !== (lastHoverRangeRef.current?.end ?? null);
            if (changed) {
              lastHoverRangeRef.current = hover;
              if (hover) {
                session.setHoverRange(hover.start, hover.end);
                dispatch(textEditorActions.hover, { start: hover.start, end: hover.end });
              }
            }
          } catch {
            /* hover range unavailable */
          }
          session.renderFrame();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          session.renderFrame();
          emitSelection();
        }}
        onPointerCancel={() => {
          const session = sessionRef.current;
          if (!session) return;
          session.pointerCancelScreen();
          session.renderFrame();
        }}
        onWheel={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          event.preventDefault();
          session.wheelScrollScreen(event.deltaY);
          session.renderFrame();
          dispatch("setCamera", { camera: JSON.parse(session.cameraJson()) });
        }}
        onContextMenu={(event) => {
          event.preventDefault();
          event.stopPropagation();
          void (async () => {
          const session = sessionRef.current;
          if (!session) return;
          dismissContextMenu();
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          session.pointerDownScreen(sx, sy, 0);
          session.pointerUpScreen(sx, sy, 0);
          session.renderFrame();
          emitSelection();
          if (event.altKey && completions.length > 0) {
            openCompletions();
            return;
          }
          const pickTargets = parseJsonOr<readonly PickTarget[]>(session.pickTargetsAtScreenJson(sx, sy), []);
          const hasSelection = session.anchor() !== session.caret();
          const localActions: Record<string, () => void> = {
            requestCompletions: openCompletions,
            selectToken: () => {
              session.selectSpanAt(session.caret());
              session.renderFrame();
              emitSelection();
            },
            selectLine: () => {
              const caret = session.caret();
              const buffer = scene.buffer;
              const lines = buffer.split("\n");
              let offset = 0;
              let lineIndex = 0;
              for (let i = 0; i < lines.length; i++) {
                const next = offset + (lines[i]?.length ?? 0) + 1;
                if (caret < next || i === lines.length - 1) {
                  lineIndex = i;
                  break;
                }
                offset = next;
              }
              offset = 0;
              for (let i = 0; i < lineIndex; i++) offset += (lines[i]?.length ?? 0) + 1;
              const lineLength = lines[lineIndex]?.length ?? 0;
              session.setSelectionRange(offset, offset + lineLength);
              session.renderFrame();
              emitSelection();
            },
            selectAll: () => {
              session.setSelectionRange(0, scene.buffer.length);
              session.renderFrame();
              emitSelection();
            },
            commitRename: () => {
              if (renameInfo) startRename();
            },
            cut: () => {
              document.execCommand("cut");
            },
            copy: () => {
              document.execCommand("copy");
            },
            paste: () => {
              document.execCommand("paste");
            },
            formatDocument: () => dispatch(textEditorActions.formatDocument ?? "formatDocument"),
            lintDocument: () => dispatch("lintDocument"),
          };
          for (const target of pickTargets) {
            localActions[`pick:${target.domain}:${target.id}`] = () => {
              /* pick handled via menu row args if present */
            };
          }
          const dispatchTextEditorMenu = (action: string, args?: Record<string, unknown>) => {
            if (localActions[action]) {
              localActions[action]!();
              return;
            }
            dispatch(action, args);
          };
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "textEditor", args: null },
              surface: {
                surfaceId,
                kind: "textEditor",
                hits: pickTargets.map((target) => ({ domain: target.domain, id: target.id, label: target.label })),
                selection: [],
                text: {
                  caret: session.caret(),
                  hasSelection,
                  word: undefined,
                  canRename: renameInfo != null,
                  hasCompletions: completions.length > 0,
                },
              },
              point: { x: event.clientX, y: event.clientY },
            },
            (specs) => mapContextMenuSpecs(specs, dispatchTextEditorMenu, textEditorMenuKeysByActionId),
            shellContextMenuFallback,
          );
          setContextMenu({ position: { x: event.clientX, y: event.clientY }, ...menu });
          })();
        }}
      >
        {renameDraft ? (
          <input
            className={cn("pointer-events-auto absolute z-50 min-w-[12rem] rounded border border-border px-2 py-1 font-mono text-xs text-foreground shadow-md", glassClass)}
            data-level="pane"
            style={renamePosition ? { left: renamePosition.x, top: renamePosition.y - 4 } : { left: 12, top: 12 }}
            value={renameDraft.text}
            autoFocus
            onChange={(event) => updateRenamePreview(event.target.value)}
            onKeyDown={(event) => {
              event.stopPropagation();
              if (event.key === "Escape") {
                event.preventDefault();
                cancelRename();
                return;
              }
              if (event.key === "Enter") {
                event.preventDefault();
                commitRename();
              }
            }}
            onBlur={commitRename}
          />
        ) : null}
        {completionsOpen && completions.length > 0
          ? (() => {
              const session = sessionRef.current;
              const position = session ? caretScreenPosition(session) : null;
              return (
                <div className={cn("pointer-events-auto absolute z-50 max-h-48 min-w-40 overflow-auto rounded border border-border p-1 shadow-md", glassClass)} data-level="menu" style={position ? { left: position.x, top: position.y + 18 } : { left: 12, top: 12 }}>
                  {completions.map((item, index) => (
                    <button
                      key={`${item.label}-${index}`}
                      type="button"
                      className={`block w-full rounded px-2 py-1 text-left font-mono text-[11px] ${index === completionIndex ? "bg-accent text-accent-foreground" : "hover:bg-active-base"}`}
                      onPointerDown={(event) => event.stopPropagation()}
                      onClick={() => applyCompletion(item)}
                    >
                      <span>{item.label}</span>
                      {item.detail ? <span className="ml-2 text-muted-foreground">{item.detail}</span> : null}
                    </button>
                  ))}
                </div>
              );
            })()
          : null}
      </div>
      <ContextMenuController title={contextMenuTitleLabel} open={contextMenu != null} position={contextMenu?.position ?? null} items={contextMenu?.items ?? []} onOpenChange={(open) => !open && dismissContextMenu()} />
      <textarea
        ref={sinkRef}
        className="absolute inset-0 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground opacity-0"
        value={scene.buffer}
        onChange={ignoreInputSinkChange}
        onCompositionEnd={(event) => {
          const session = sessionRef.current;
          if (!session || readOnlyRef.current || event.data.length === 0) return;
          session.insertText(event.data);
          sendEdit(session.text());
          session.renderFrame();
          emitSelection();
        }}
        onPaste={(event) => {
          const session = sessionRef.current;
          const pasted = event.clipboardData.getData("text/plain");
          event.preventDefault();
          if (!session || readOnlyRef.current || pasted.length === 0) return;
          session.replaceSelection(pasted);
          sendEdit(session.text());
          session.renderFrame();
          emitSelection();
        }}
        onCopy={(event) => {
          const selected = sessionRef.current?.selectionText() ?? "";
          if (selected.length === 0) return;
          event.preventDefault();
          event.clipboardData.setData("text/plain", selected);
        }}
        onCut={(event) => {
          const session = sessionRef.current;
          const selected = session?.selectionText() ?? "";
          if (!session || selected.length === 0) return;
          event.preventDefault();
          event.clipboardData.setData("text/plain", selected);
          if (readOnlyRef.current) return;
          session.replaceSelection("");
          sendEdit(session.text());
          session.renderFrame();
          emitSelection();
        }}
        onKeyDown={(event) => {
          if (event.nativeEvent.isComposing || event.key === "Process") return;
          const history = documentHistoryChord(event);
          if (history) {
            event.preventDefault();
            dispatch(history, {});
            return;
          }
          if (readOnlyRef.current && isTextEditingKey(event)) {
            event.preventDefault();
            return;
          }
          const session = sessionRef.current;
          if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch("submit", {});
            return;
          }
          if ((event.key === " " || event.code === "Space") && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            openCompletions();
            return;
          }
          if (event.key === "F2" && renameInfo) {
            event.preventDefault();
            startRename();
            return;
          }
          if (event.key === "a" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            session?.selectAll();
            session?.renderFrame();
            emitSelection();
            return;
          }
          if (event.key.toLowerCase() === "f" && event.shiftKey && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch(textEditorActions.formatDocument, {});
            return;
          }
          if (!session) return;

          if (completionsOpen && completions.length > 0) {
            if (event.key === "ArrowDown") {
              event.preventDefault();
              setCompletionIndex((index) => (index + 1) % completions.length);
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              setCompletionIndex((index) => (index - 1 + completions.length) % completions.length);
              return;
            }
            if (event.key === "Tab" || event.key === "Enter") {
              event.preventDefault();
              applyCompletion(completions[completionIndex] ?? completions[0]!);
              return;
            }
            if (event.key === "Escape") {
              event.preventDefault();
              setCompletionsOpen(false);
              return;
            }
          }

          const extend = event.shiftKey;
          if (event.key === "ArrowLeft") {
            event.preventDefault();
            session.moveLeft(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowRight") {
            event.preventDefault();
            session.moveRight(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowUp") {
            event.preventDefault();
            session.moveUp(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowDown") {
            event.preventDefault();
            session.moveDown(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Home") {
            event.preventDefault();
            session.moveLineStart(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "End") {
            event.preventDefault();
            session.moveLineEnd(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Tab") {
            event.preventDefault();
            session.insertText(session.tabInsertText());
            sendEdit(session.text());
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Enter") {
            event.preventDefault();
            const allowed = newlineGates == null || newlineGates.has(session.caret());
            if (allowed) {
              session.insertText("\n");
              sendEdit(session.text());
              session.renderFrame();
              emitSelection();
            }
            return;
          }
          if (event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey) {
            event.preventDefault();
            session.insertText(event.key);
            sendEdit(session.text());
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Backspace") {
            event.preventDefault();
            session.backspace();
            sendEdit(session.text());
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Delete") {
            event.preventDefault();
            session.deleteForward();
            sendEdit(session.text());
            session.renderFrame();
            emitSelection();
          }
        }}
        spellCheck={false}
        aria-readonly={readOnly}
        data-read-only={readOnly ? "" : undefined}
        aria-label={scene.language ? `${scene.language} editor` : "Editor"}
      />
    </div>
  );
}
//#endregion WasmEditorSurface

//#region TextEditorHost
export function TextEditorHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.textEditor;
  const isClient = useClient();
  const presenceWindowId = node.surfaceId || "text";
  const localPresenceActor = useLocalPresenceActorIdV1("local");
  const presencePublishAtRef = useRef(0);
  useEffect(() => () => clearLocalPresenceWindowViewV1(presenceWindowId), [presenceWindowId]);
  useEffect(() => {
    if (!scene) return;
    const now = Date.now();
    if (now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS) return;
    presencePublishAtRef.current = now;
    const caret = scene.buffer.length;
    publishLocalPresenceWindowViewV1("local", presenceWindowId, {
      windowId: presenceWindowId,
      space: "canvas",
      kind: { kind: "canvas", x: 0, y: 0, zoom: 1 },
      size: [1, 1],
      pointer: [caret, 0, 0],
    });
    publishLocalActiveToolV1("local", "edit");
  }, [presenceWindowId, scene?.buffer]);
  const tokens = useMemo((): readonly GrammarToken[] => {
    if (!scene?.tokensJson) return [];
    try {
      return parseSceneJsonField<GrammarToken[]>(scene.tokensJson);
    } catch {
      return [];
    }
  }, [scene?.tokensJson]);
  const diagnostics = useMemo((): readonly EditorDiagnostic[] => {
    if (!scene?.diagnosticsJson) return [];
    try {
      return parseSceneJsonField<EditorDiagnostic[]>(scene.diagnosticsJson);
    } catch {
      return [];
    }
  }, [scene?.diagnosticsJson]);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const documentPlaceholderLabel = useLabel("ui.host.documentPlaceholder");
  const languageDocumentLabel = useLabel("ui.host.languageDocument", { language: scene?.language });

  if (!scene) return <div className="semio-text-editor-empty">{emptySceneLabel}</div>;

  return (
    <div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col ui-surface" data-level="base" data-surface-id={node.surfaceId}>
      {isClient ? (
        <WasmEditorSurface scene={scene} controllerId={node.controllerId} surfaceId={node.surfaceId} onAction={onAction} requestContextMenu={requestContextMenu} />
      ) : (
        <div className="relative min-h-0 flex-1">
          <HighlightedBuffer buffer={scene.buffer} tokens={tokens} />
          <Textarea
            className="relative min-h-0 flex-1 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground"
            id={`${node.surfaceId}.editor`}
            lazy
            rows={24}
            value={scene.buffer}
            placeholder={scene.language ? languageDocumentLabel : documentPlaceholderLabel}
            onLazyChange={(value) =>
              onAction({
                controllerId: node.controllerId,
                action: textEditorActions.edit,
                args: { surfaceId: node.surfaceId, text: value },
              })
            }
          />
        </div>
      )}
      {diagnostics.length > 0 ? (
        <div className="border-t border-border px-3 py-2 text-[11px] text-muted-foreground">
          {diagnostics.slice(0, 4).map((diag, index) => (
            <div key={`${diag.start}-${diag.end}-${index}`} className="truncate">
              {diag.severity ? `[${diag.severity}] ` : ""}
              {diag.message}
            </div>
          ))}
        </div>
      ) : null}
      <CanvasPresenceOverlayV1
        runtimeKey="local"
        windowId={presenceWindowId}
        space="canvas"
        myActor={localPresenceActor ?? ""}
        locale={typeof document !== "undefined" ? document.documentElement.lang : undefined}
        localCanvas={{ x: 0, y: 0, zoom: 1 }}
        localSizePx={[1, 1]}
        scenePath={`text/${presenceWindowId}`}
      />
    </div>
  );
}
//#endregion TextEditorHost
//#endregion 🔖️TextEditorHost
