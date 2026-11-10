/** 🪝️ World3dHost handle drag session: presses on an axis-carrying handle marker, solves the drag parameter from the camera ray and fires the surface node's `delta`, `commit` and `abort` bindings.
 *
 * @see ../🟦️.ts — the pure gesture the session drives; the sequences in `../../🧪️tests/🧫️fixtures/🧲️handle-drag.json` pin it.
 */
import * as React from "../../../⚛️react-port/🟦️.ts";
import { useCallback, useEffect, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { axisDragParam } from "../../🪟️window-view-store/🟦️.tsx";
import { raycastGroundPoint } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import type { WorldCore5 } from "../../🔀️paint-and-projection/🟦️.ts";
import { world3dHandleDragStepV1, world3dHandlePickV1, type WorldHandleDragEvent, type WorldHandleDragState, type WorldHandleModifiers, type WorldHandleTrigger, type WorldVec3 } from "../🟦️.ts";

const HANDLE_PICK_RADIUS_PX = 12;

type HandleSession = { state: WorldHandleDragState; position: WorldVec3; axis: WorldVec3 };

const modifiersOf = (event: React.PointerEvent<HTMLDivElement>): WorldHandleModifiers => ({ shift: event.shiftKey, alt: event.altKey, ctrl: event.ctrlKey || event.metaKey });

export function useWorldHandleDrag(core: WorldCore5) {
  const { annotationLayer, annotationStore, cameraRef, hostRef, onTrigger } = core;
  const sessionRef = useRef<HandleSession | null>(null);
  const [handleDragActive, setHandleDragActive] = useState(false);
  const chainRef = useRef<Promise<unknown>>(Promise.resolve());

  const fire = useCallback(
    (step: { readonly dispatch: { readonly trigger: WorldHandleTrigger; readonly input: object } | null }): void => {
      const sent = step.dispatch;
      if (!sent || !onTrigger) return;
      chainRef.current = chainRef.current.then(() => onTrigger(sent.trigger, sent.input as Readonly<Record<string, unknown>>)).catch(() => undefined);
    },
    [onTrigger],
  );

  const parameterAt = useCallback(
    (event: React.PointerEvent<HTMLDivElement>, session: HandleSession): number | null => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      return host && camera ? axisDragParam(event.clientX, event.clientY, host.getBoundingClientRect(), camera, session.position, session.axis) : null;
    },
    [cameraRef, hostRef],
  );

  const stepSession = useCallback(
    (session: HandleSession, event: WorldHandleDragEvent): void => {
      const step = world3dHandleDragStepV1(session.state, event);
      session.state = step.state;
      fire(step);
    },
    [fire],
  );

  const beginHandleDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const host = hostRef.current;
      if (!onTrigger || !annotationLayer || !host) return false;
      const rect = host.getBoundingClientRect();
      const projected = new Map(annotationStore.get().map((entry) => [entry.id, entry]));
      const handles = annotationLayer.items.flatMap((item) => {
        const marker = item.kind === "marker" && item.axis ? projected.get(item.id)?.marker : undefined;
        return marker ? [{ id: item.id, x: marker.x, y: marker.y }] : [];
      });
      const id = world3dHandlePickV1(handles, event.clientX - rect.left, event.clientY - rect.top, HANDLE_PICK_RADIUS_PX);
      const item = annotationLayer.items.find((entry) => entry.id === id);
      if (!item || item.kind !== "marker" || !item.axis) return false;
      const session: HandleSession = { state: null, position: item.position, axis: item.axis };
      const s = parameterAt(event, session);
      if (s === null) return false;
      sessionRef.current = session;
      setHandleDragActive(true);
      stepSession(session, { kind: "press", handle: item.id, s });
      event.currentTarget.setPointerCapture?.(event.pointerId);
      return true;
    },
    [annotationLayer, annotationStore, hostRef, onTrigger, parameterAt, stepSession],
  );

  const pointerAt = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): WorldVec3 => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      return (host && camera && raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera)) || [0, 0, 0];
    },
    [cameraRef, hostRef],
  );

  const updateHandleDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const session = sessionRef.current;
      if (!session) return false;
      const s = parameterAt(event, session);
      if (s !== null) stepSession(session, { kind: "move", s, modifiers: modifiersOf(event), at: pointerAt(event) });
      return true;
    },
    [parameterAt, pointerAt, stepSession],
  );

  const endHandleDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement> | null): boolean => {
      const session = sessionRef.current;
      if (!session) return false;
      sessionRef.current = null;
      setHandleDragActive(false);
      const modifiers = event ? modifiersOf(event) : { shift: false, alt: false, ctrl: false };
      const at = event ? pointerAt(event) : session.position;
      const s = event && event.type !== "pointercancel" ? parameterAt(event, session) : null;
      stepSession(session, s === null ? { kind: "cancel", modifiers, at } : { kind: "release", s, modifiers, at });
      return true;
    },
    [parameterAt, pointerAt, stepSession],
  );

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key === "Escape" && endHandleDrag(null)) event.stopPropagation();
    };
    window.addEventListener("keydown", onKeyDown, true);
    return () => window.removeEventListener("keydown", onKeyDown, true);
  }, [endHandleDrag]);

  return { beginHandleDrag, endHandleDrag, handleDragActive, updateHandleDrag };
}
