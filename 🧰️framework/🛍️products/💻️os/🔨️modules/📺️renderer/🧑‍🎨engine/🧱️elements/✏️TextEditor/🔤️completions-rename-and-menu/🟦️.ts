/** 🔤️ Text editor completions, rename and menu: the wasm session and rename state, the completions and rename info, the session factory, the caret screen position, the completion, rename and context menu handlers and the session ready callback. */
import { useCallback, useEffect, useMemo, useState } from "react";
import { type WasmCanvasSession } from "@semio-tech/framework/canvas/react-renderer";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import { useLabel } from "@semio-tech/ui-react";
import { textEditorActions } from "@semio-tech/framework";
import { createEditorSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { type SurfaceContextMenuResult } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import type { CompletionItem, FrameworkEditorSession, RenameDraft, RenameInfo } from "../🏷️editor-types/🟦️.ts";
import { identifierPrefixStart, multiSpanReplace } from "../🖍️buffer-and-edit-helpers/🟦️.tsx";
import { parseJsonOr } from "../📤️outbox-and-scene-packets/🟦️.ts";
import type { EditorCore2 } from "../👥️session-presence-and-caret/🟦️.ts";

// #region WasmEditorSurface
/** 🔤️ Provides the completion list, the rename preview and commit and the context menu of the editor. */
export function useEditorCompletionsRenameAndMenu(core: EditorCore2) {
  const { caretCadenceRef, dispatch, emitSelection, renameActiveRef, scene, sessionRef, setSessionEpoch } = core;
  const [wasmSession, setWasmSession] = useState<FrameworkEditorSession | null>(null);
  const [renameDraft, setRenameDraft] = useState<RenameDraft | null>(null);
  const [renamePosition, setRenamePosition] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const [completionsOpen, setCompletionsOpen] = useState(false);
  const [completionIndex, setCompletionIndex] = useState(0);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly position: { readonly x: number; readonly y: number } }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.editor");
  const editorLabel = useLabel(scene.language ? "ui.host.languageDocument" : "ui.host.documentPlaceholder", { language: scene.language });

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
    if (wasmSession && typeof wasmSession.attachCanvas === "function") return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      synchronizeScene: () => {},
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
      emitSelection();
      setCompletionsOpen(false);
    },
    [emitSelection],
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
    }
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [scene.buffer]);

  const dismissContextMenu = useCallback(() => setContextMenu(null), []);

  const onSessionReady = useCallback((session: WasmCanvasSession) => {
    sessionRef.current = session as FrameworkEditorSession;
    sessionRef.current.setCaretVisible(caretCadenceRef.current?.visible() ?? false);
    syncSessionCanvasTheme(sessionRef.current);
    setSessionEpoch((epoch) => epoch + 1);
  }, []);
  return { applyCompletion, cancelRename, caretScreenPosition, commitRename, completionIndex, completions, completionsOpen, contextMenu, contextMenuTitleLabel, dismissContextMenu, editorLabel, newlineGates, onSessionReady, openCompletions, renameDraft, renameInfo, renamePosition, sessionFactory, setCompletionIndex, setCompletionsOpen, setContextMenu, startRename, updateRenamePreview };
}

export type EditorCore3 = EditorCore2 & ReturnType<typeof useEditorCompletionsRenameAndMenu>;
// #endregion WasmEditorSurface
