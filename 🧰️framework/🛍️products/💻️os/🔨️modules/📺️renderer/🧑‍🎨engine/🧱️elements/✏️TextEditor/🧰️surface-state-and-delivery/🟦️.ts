/** 🧰️ Text editor surface state and delivery: the session, rename, hover, echo and draft refs, the dispatcher, the read-only and splice typing state, the typing run, the edit delivery, the explicit draft history stepping and the session sync. */
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { createTextEditorTypingRunV1, receiveTextEditorSceneV1, refuseTextEditorSpliceV1, sendTextEditorSpliceV1, settleTextEditorSpliceV1, TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_HOST_SIGNALS, textEditorActions, textEditorAppliedSpliceV1, textEditorSpliceHostV1, textEditorTypingV1, type TextEditorScene, type TextEditorSpliceHostV1, type TextEditorSpliceViewV1, type TextEditorTypingRunV1 } from "@semio-tech/framework";
import { useShellContextMenuFallback, useAppKeybindingsByActionId } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { publishLocalPresenceTypingV1 } from "../../👕️canvas-presence/🟦️.ts";
import type { FrameworkEditorSession, SpanRange, WasmEditorSurfaceProps } from "../🏷️editor-types/🟦️.ts";
import { reconcileTextEditorEchoV1, type TextEditorEchoStateV1 } from "../🖍️buffer-and-edit-helpers/🟦️.tsx";
import { outcomeApplied, refusalReason, refuseTextEditorEditV1, TEXT_EDITOR_READ_ONLY_REFUSALS } from "../🚫️edit-refusals-and-keys/🟦️.ts";
import { createTextEditorOutboxV1, sameScenePackV1, TEXT_EDITOR_PENDING_EDIT_LIMIT, type TextEditorOutboxV1, textEditorScenePacket } from "../📤️outbox-and-scene-packets/🟦️.ts";
import { showTextEditorSpliceViewV1, textEditorSessionSelectionV1 } from "../🪞️session-selection-and-splice-view/🟦️.ts";

// #region WasmEditorSurface
/** 🧰️ Holds the refs and state of the editor surface and delivers edits and scene syncs. */
export function useEditorSurfaceState({ scene, controllerId, surfaceId, onAction, explicitDraft, explicitDraftDirty, descriptionId, invalid, onDraftChange, requestContextMenu }: WasmEditorSurfaceProps) {
  const sessionRef = useRef<FrameworkEditorSession | null>(null);
  const renameActiveRef = useRef(false);
  const lastHoverRangeRef = useRef<SpanRange | null>(null);
  const echoStateRef = useRef<TextEditorEchoStateV1>({ pending: [], acknowledged: null });
  const reconciledRef = useRef<{ readonly scene: TextEditorScene; readonly pack: Uint8Array } | null>(null);
  const syncedRef = useRef<{ readonly session: FrameworkEditorSession; readonly pack: Uint8Array } | null>(null);
  const explicitHistoryRef = useRef<{ past: string[]; current: string; future: string[] }>({ past: [], current: scene.buffer, future: [] });

  useEffect(() => {
    if (!explicitDraft || explicitDraftDirty === false || explicitHistoryRef.current.current !== scene.buffer) {
      explicitHistoryRef.current = { past: [], current: scene.buffer, future: [] };
    }
  }, [explicitDraft, explicitDraftDirty, scene.buffer]);

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
  const spliceHostRef = useRef<TextEditorSpliceHostV1 | null>(null);
  const spliceTyping = useMemo(() => textEditorTypingV1(scene.settingsJson) !== null, [scene.settingsJson]);
  const spliceTypingRef = useRef(false);
  spliceTypingRef.current = spliceTyping && !explicitDraft;
  /** 📮️ ONE round trip in flight per editor, latest state wins: a typed run is delivered as the editor's text and selection AT
   * DELIVERY (`createTextEditorOutboxV1`) whenever the previous delivery settled, never one `textEdit` + one `textSelect` per key —
   * sustained typing at 40 keys/s filled the per-actor command queue (`queue-full`, > 256 pending turns) and dropped keys
   * (ticket 26/09/23 F1). The texts that do go out are the ones the echo reconciliation waits for. The dispatcher lives as long
   * as the surface and reads its owner through `deliveryOwnerRef`: a dispatcher rebuilt on every new `onAction` identity left
   * the previous one flushing its own stale pending text AFTER the newer one had gone out, so the guest took an older buffer
   * last and the author's final keys vanished (ticket 26/09/23 C11, probe `c11typing3`). */
  const deliveryOwnerRef = useRef({ controllerId, explicitDraft, onAction, onDraftChange, surfaceId });
  deliveryOwnerRef.current = { controllerId, explicitDraft, onAction, onDraftChange, surfaceId };
  /** ⌨️ Publishes the editor's pending run as the ephemeral shared preview peers render at its caret (`null` once it ended). */
  const publishTypingPreview = (surfaceId: string, local: string) => {
    const preview = typingRef.current?.preview(local) ?? null;
    publishLocalPresenceTypingV1("local", surfaceId || "text", preview === null ? null : { deleted: preview.deleted, insert: preview.insert });
  };
  /** ⌨️ The editor's live typing run (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every delivery the
   * window took names this editor's buffer (`typing`), so the window folds it into ONE run that commits as ONE document edit; the
   * host ends the run on idle, a caret move, blur and a hidden page — never one edit per keystroke, never a coalesce key. */
  const typingRef = useRef<TextEditorTypingRunV1 | null>(null);
  typingRef.current ??= createTextEditorTypingRunV1(
    (verb, reason) => {
      const { controllerId, onAction, surfaceId } = deliveryOwnerRef.current;
      publishLocalPresenceTypingV1("local", surfaceId || "text", null);
      void onAction({ controllerId, action: verb, args: { surfaceId, [TEXT_EDITOR_TYPING_BUFFER_ARG]: surfaceId, [TEXT_EDITOR_TYPING_COMMIT_ARG]: reason } });
    },
    (run, ms) => {
      const timer = setTimeout(run, ms);
      return () => clearTimeout(timer);
    },
  );
  const deliver = useMemo(
    () =>
      createTextEditorOutboxV1<TextEditorOutboxV1>(
        () => {
          const session = sessionRef.current;
          return session === null ? null : { text: session.text(), start: session.anchor(), end: session.caret() };
        },
        async (next) => {
          const { controllerId, explicitDraft, onAction, onDraftChange, surfaceId } = deliveryOwnerRef.current;
          if (explicitDraft) {
            const history = explicitHistoryRef.current;
            if (history.current !== next.text) explicitHistoryRef.current = { past: [...history.past, history.current].slice(-256), current: next.text, future: [] };
            echoStateRef.current = { pending: [], acknowledged: next.text };
            onDraftChange?.(next.text);
            return;
          }
          const spliceHost = spliceHostRef.current;
          if (spliceTypingRef.current && spliceHost !== null) {
            const sent = readOnlyRef.current ? null : sendTextEditorSpliceV1(spliceHost, next.text);
            if (sent !== null) {
              spliceHostRef.current = sent.host;
              const outcome = await onAction({ controllerId, action: textEditorActions.splice, args: { surfaceId, ...sent.splice, seq: sent.seq, anchor: next.start, caret: next.end, [TEXT_EDITOR_TYPING_BUFFER_ARG]: surfaceId } });
              const reason = refusalReason(outcome);
              if (reason === null) {
                if (outcomeApplied(outcome)) spliceHostRef.current = settleTextEditorSpliceV1(spliceHostRef.current ?? sent.host, sent.seq);
                typingRef.current?.typed(textEditorActions.splice, spliceHost.base);
                publishTypingPreview(surfaceId, next.text);
                return;
              }
              if (TEXT_EDITOR_READ_ONLY_REFUSALS.has(reason)) {
                readOnlyRef.current = true;
                setReadOnly(true);
              }
              const session = sessionRef.current;
              const local = session?.text() ?? next.text;
              const refused = refuseTextEditorSpliceV1(spliceHostRef.current ?? sent.host, sent.seq, local, textEditorSessionSelectionV1(session, local));
              spliceHostRef.current = refused.host;
              showTextEditorSpliceViewV1(session, refused.show);
              return;
            }
            typingRef.current?.commit(TEXT_EDITOR_TYPING_HOST_SIGNALS.caretMove);
            if (selectionUndeclaredRef.current) return;
            const reason = refusalReason(await onAction({ controllerId, action: textEditorActions.select, args: { surfaceId, start: next.start, end: next.end, splice: spliceHost.seq } }));
            if (reason === "undeclared-action") selectionUndeclaredRef.current = true;
            return;
          }
          const echo = echoStateRef.current;
          const guestHasText = echo.pending.length > 0 ? echo.pending.at(-1) === next.text : echo.acknowledged === next.text;
          if (guestHasText) typingRef.current?.commit(TEXT_EDITOR_TYPING_HOST_SIGNALS.caretMove);
          if (!guestHasText && !readOnlyRef.current) {
            const before = echo.pending.at(-1) ?? echo.acknowledged ?? next.text;
            echoStateRef.current = { ...echoStateRef.current, pending: [...echoStateRef.current.pending, next.text].slice(-TEXT_EDITOR_PENDING_EDIT_LIMIT) };
            const reason = refusalReason(await onAction({ controllerId, action: textEditorActions.edit, args: { surfaceId, text: next.text, [TEXT_EDITOR_TYPING_BUFFER_ARG]: surfaceId } }));
            if (reason === null) {
              typingRef.current?.typed(textEditorActions.edit, before);
              publishTypingPreview(surfaceId, next.text);
            }
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
    [],
  );
  const moveExplicitHistory = useCallback((direction: "undo" | "redo") => {
    if (!explicitDraft) return false;
    const history = explicitHistoryRef.current;
    const source = direction === "undo" ? history.past : history.future;
    const next = source.at(-1);
    if (next === undefined) return true;
    explicitHistoryRef.current = direction === "undo"
      ? { past: history.past.slice(0, -1), current: next, future: [...history.future, history.current] }
      : { past: [...history.past, history.current], current: next, future: history.future.slice(0, -1) };
    const session = sessionRef.current;
    session?.setText(next);
    session?.setSelectionRange(next.length, next.length);
    echoStateRef.current = { pending: [], acknowledged: next };
    onDraftChange?.(next);
    return true;
  }, [explicitDraft, onDraftChange]);

  const syncSession = useCallback(
    (resync = false) => {
      if (renameActiveRef.current) return;
      if (spliceTypingRef.current) {
        const session = sessionRef.current;
        let show: TextEditorSpliceViewV1 = null;
        if (resync || spliceHostRef.current === null) {
          spliceHostRef.current = textEditorSpliceHostV1(scene.buffer, textEditorAppliedSpliceV1(scene.selectionJson));
          reconciledRef.current = { scene, pack: textEditorScenePacket(scene, true) };
        } else if (reconciledRef.current?.scene !== scene) {
          const local = session?.text() ?? scene.buffer;
          const received = receiveTextEditorSceneV1(spliceHostRef.current, scene.buffer, textEditorAppliedSpliceV1(scene.selectionJson), local, textEditorSessionSelectionV1(session, local));
          spliceHostRef.current = received.host;
          reconciledRef.current = { scene, pack: textEditorScenePacket(scene, false) };
          show = received.show;
        }
        const pack = reconciledRef.current.pack;
        if (session === null) return;
        if (resync || syncedRef.current === null || syncedRef.current.session !== session || !sameScenePackV1(syncedRef.current.pack, pack)) {
          try {
            session.synchronizeScene(pack, 0, 16_777_216, 16_777_216, () => sessionRef.current === session && !renameActiveRef.current);
            syncedRef.current = { session, pack };
          } catch {
            syncedRef.current = null;
            return;
          }
        }
        showTextEditorSpliceViewV1(session, show);
        return;
      }
      if (resync) {
        echoStateRef.current = { pending: [], acknowledged: scene.buffer };
        reconciledRef.current = { scene, pack: textEditorScenePacket(scene, true) };
      } else if (reconciledRef.current?.scene !== scene) {
        const echo = reconcileTextEditorEchoV1(echoStateRef.current, scene.buffer);
        echoStateRef.current = { pending: echo.pending, acknowledged: echo.acknowledged };
        reconciledRef.current = { scene, pack: textEditorScenePacket(scene, echo.external) };
      }
      const session = sessionRef.current;
      const pack = reconciledRef.current.pack;
      if (session === null) return;
      if (!resync && syncedRef.current !== null && syncedRef.current.session === session && sameScenePackV1(syncedRef.current.pack, pack)) return;
      try {
        session.synchronizeScene(pack, 0, 16_777_216, 16_777_216, () => sessionRef.current === session && !renameActiveRef.current);
        syncedRef.current = { session, pack };
      } catch {
        syncedRef.current = null;
      }
    },
    [scene],
  );
  syncSessionRef.current = syncSession;
  return { deliver, descriptionId, dispatch, invalid, lastHoverRangeRef, moveExplicitHistory, readOnly, readOnlyRef, renameActiveRef, requestContextMenu, scene, sessionRef, shellContextMenuFallback, surfaceId, syncSession, textEditorMenuKeysByActionId, typingRef };
}

export type EditorCore1 = ReturnType<typeof useEditorSurfaceState>;
// #endregion WasmEditorSurface
