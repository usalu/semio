/** 👥️ Text editor session, presence and caret: the session epoch and sync effect, the canvas appearance sync, the local and peer caret presence, the selection emission and the caret cadence effects. */
import { useCallback, useEffect, useRef, useState } from "react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import { useCanvasAppearanceSync, useShellScopeOptional } from "@semio-tech/ui-react";
import { TEXT_EDITOR_TYPING_HOST_SIGNALS } from "@semio-tech/framework";
import { publishLocalPresenceWindowViewV1, clearLocalPresenceWindowViewV1, publishLocalActiveToolV1 } from "../../👕️canvas-presence/🟦️.ts";
import { installTextEditorCaretCadenceV1, type TextEditorCaretCadenceV1 } from "../⏱️caret-cadence/🟦️.ts";
import type { EditorCore1 } from "../🧰️surface-state-and-delivery/🟦️.ts";

// #region WasmEditorSurface
/**
 * 👥️ Syncs the session, publishes the local caret and projects peer carets.
 *
 * ⌨️ The typing run ends as ONE edit when the editor loses focus or the page is hidden or left (a run pending for less than the
 * idle bound is never lost to a closed tab), and when the editor unmounts.
 */
export function useEditorSessionPresence(core: EditorCore1) {
  const { deliver, readOnlyRef, sessionRef, surfaceId, syncSession, typingRef , presenceRuntimeKey } = core;
  const [sessionEpoch, setSessionEpoch] = useState(0);

  useEffect(() => {
    syncSession();
  }, [syncSession, sessionEpoch]);

  const wasmEditorSurfaceShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      syncSessionCanvasTheme(sessionRef.current);
    },
    true,
    wasmEditorSurfaceShellScope?.rootRef.current ?? undefined,
  );

  const presenceWindowId = surfaceId || "text";
  const surfaceRef = useRef<HTMLDivElement>(null);
  const [presenceFrame, setPresenceFrame] = useState(0);
  /** ✏️ Publishes this text window's caret in the editor's WORLD coordinates (plus its camera and size) for the next
   * presence beat, so every peer paints it at the same text position through its own camera. */
  const publishCaretPresence = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const world = JSON.parse(session.caretWorldJson()) as { readonly x?: number; readonly y?: number } | null;
      const camera = JSON.parse(session.cameraJson()) as { readonly x?: number; readonly y?: number; readonly zoom?: number };
      const rect = surfaceRef.current?.getBoundingClientRect();
      if (!presenceRuntimeKey) return;
      publishLocalPresenceWindowViewV1(presenceRuntimeKey, presenceWindowId, {
        windowId: presenceWindowId,
        space: "text",
        kind: { kind: "canvas", x: camera.x ?? 0, y: camera.y ?? 0, zoom: camera.zoom ?? 1 },
        size: [rect?.width ?? 0, rect?.height ?? 0],
        ...(world?.x == null || world?.y == null ? {} : { pointer: [world.x, world.y, 0] as const }),
      });
      publishLocalActiveToolV1(presenceRuntimeKey, "edit");
    } catch {
      return;
    }
  }, [presenceWindowId, presenceRuntimeKey]);
  useEffect(() => presenceRuntimeKey ? () => clearLocalPresenceWindowViewV1(presenceWindowId, presenceRuntimeKey) : undefined, [presenceWindowId, presenceRuntimeKey]);
  const projectPeerCaret = useCallback((world: readonly [number, number]): { readonly x: number; readonly y: number } | null => {
    const session = sessionRef.current;
    if (!session) return null;
    try {
      const screen = JSON.parse(session.worldToScreenJson(world[0], world[1])) as { readonly x?: number; readonly y?: number } | null;
      return screen?.x == null || screen?.y == null ? null : { x: screen.x, y: screen.y };
    } catch {
      return null;
    }
  }, []);

  const emitSelection = useCallback(() => {
    deliver();
    publishCaretPresence();
  }, [deliver, publishCaretPresence]);

  const sinkRef = useRef<HTMLTextAreaElement>(null);
  const caretCadenceRef = useRef<TextEditorCaretCadenceV1 | null>(null);
  useEffect(() => {
    const sink = sinkRef.current;
    if (!sink) return;
    const cadence = installTextEditorCaretCadenceV1(sink, (visible) => {
      const session = sessionRef.current;
      if (!session) return;
      session.setCaretVisible(visible);
    });
    caretCadenceRef.current = cadence;
    return () => {
      cadence.dispose();
      if (caretCadenceRef.current === cadence) caretCadenceRef.current = null;
    };
  }, []);
  useEffect(() => {
    const sink = sinkRef.current;
    if (!sink) return;
    const document = sink.ownerDocument;
    const blur = () => typingRef.current?.commit(TEXT_EDITOR_TYPING_HOST_SIGNALS.blur);
    const hidden = () => typingRef.current?.commit(TEXT_EDITOR_TYPING_HOST_SIGNALS.hidden);
    const visibility = () => {
      if (document.visibilityState === "hidden") hidden();
    };
    sink.addEventListener("blur", blur);
    document.defaultView?.addEventListener("pagehide", hidden);
    document.addEventListener("visibilitychange", visibility);
    return () => {
      sink.removeEventListener("blur", blur);
      document.defaultView?.removeEventListener("pagehide", hidden);
      document.removeEventListener("visibilitychange", visibility);
      typingRef.current?.dispose();
    };
  }, []);
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
      emitSelection();
    };
    sink.addEventListener("beforeinput", onBeforeInput);
    return () => sink.removeEventListener("beforeinput", onBeforeInput);
  }, [emitSelection]);
  return { caretCadenceRef, emitSelection, presenceFrame, presenceWindowId, projectPeerCaret, publishCaretPresence, sessionEpoch, setPresenceFrame, setSessionEpoch, sinkRef, surfaceRef };
}

export type EditorCore2 = EditorCore1 & ReturnType<typeof useEditorSessionPresence>;
// #endregion WasmEditorSurface
