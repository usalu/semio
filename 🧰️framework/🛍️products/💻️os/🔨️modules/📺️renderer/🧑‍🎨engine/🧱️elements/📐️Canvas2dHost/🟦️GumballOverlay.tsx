// #region 🧭️Canvas2dGumballOverlay
/** 🧭️ Screen-space 2D transform gumball driven by the plugin `meta:gumball` layer (`🧬️schema/🔣️gumball-meta`). The
 * handle geometry, the hit test and the gesture algebra below are pure and shared with the wgpu twin
 * (`🎯️targets/🧊️wgpu`) through the corpus `🧫️fixtures/🧫️gumball-dispatch`: a live gumball streams its gesture
 * (`phase` stream/commit/abort) into the app's ONE open tool transaction, a non-live one previews locally and dispatches
 * ONE one-shot pose delta on release (design §5, decision 2026-09-30 21:36). */
import { useCallback, useEffect, useMemo, useRef, useState, type PointerEvent as ReactPointerEvent } from "react";
import { SPATIAL_AXIS_COLOR_REFS } from "@semio-tech/ui-styling";
import type { CanvasCamera } from "./🟦️.tsx";

/** 📏️ Screen pixels from the pivot to the move knobs; the scale knobs sit at three quarters of it on the diagonals. */
export const CANVAS2D_GUMBALL_HANDLE_LENGTH = 56;
/** ⭕️ Screen radius of the turn ring; the uniform-scale knob sits on it at 0.7 of the radius per axis. */
export const CANVAS2D_GUMBALL_ROTATE_RADIUS = 44;
/** 🎯️ Screen radius a knob is pressed within. */
export const CANVAS2D_GUMBALL_HIT_RADIUS = 10;
/** 💍️ Screen half-width of the turn ring's press band. */
export const CANVAS2D_GUMBALL_RING_HIT_BAND = 4;
const MOVE_EPSILON = 1e-9;
const TURN_EPSILON = 1e-6;

export type Canvas2dGumballConfig = {
  readonly moveAxes: boolean;
  readonly rotate: boolean;
  readonly scaleAxes: boolean;
  readonly scaleUniform: boolean;
};

/** 🗺️ The affine map from model to layer units per axis (`layer = model × scale + offset`); absent = identity. */
export type Canvas2dGumballModelToLayer = {
  readonly scale: readonly [number, number];
  readonly offset: readonly [number, number];
};

export type Canvas2dGumballMeta = {
  readonly active: boolean;
  readonly liveDispatch?: boolean;
  readonly pivotLayer: readonly [number, number];
  readonly modelToLayer?: Canvas2dGumballModelToLayer;
  readonly selectionIds: readonly string[];
  readonly config: Canvas2dGumballConfig;
};

export type Canvas2dGumballHandleKind = "moveX" | "moveY" | "rotate" | "scaleX" | "scaleY" | "scaleUniform";

/** 🔭️ The host view the gumball is painted and measured in: its camera and logical viewport size. */
export type Canvas2dGumballView = {
  readonly camera: CanvasCamera;
  readonly viewportWidth: number;
  readonly viewportHeight: number;
};

export type Canvas2dGumballTransformPayload = {
  readonly action: string;
  readonly args: Record<string, unknown>;
};

/** ❓️ Why a host cancels the gesture in flight — the tool-machine abort reasons a canvas overlay observes. */
export type Canvas2dGumballCancelReason = "blur" | "captureLost";

/** ✋️ One gumball gesture in flight: the handle, the press point, the pivot, the pinned ids, whether it streams, the last
 * cumulative pose delta (`null` until it moved) and whether a stream tick went out. */
export type Canvas2dGumballGesture = {
  readonly kind: Canvas2dGumballHandleKind;
  readonly start: { readonly x: number; readonly y: number };
  readonly pivotLayer: readonly [number, number];
  readonly modelToLayer: Canvas2dGumballModelToLayer;
  readonly ids: readonly string[];
  readonly live: boolean;
  readonly total: Canvas2dGumballTransformPayload | null;
  readonly streamed: boolean;
};

/** 🪜️ A pointer move's outcome: the gesture after it and the dispatch it sends (`null`: none). */
export type Canvas2dGumballDragStep = {
  readonly gesture: Canvas2dGumballGesture;
  readonly dispatch: Canvas2dGumballTransformPayload | null;
};

const IDENTITY_MAP: Canvas2dGumballModelToLayer = { scale: [1, 1], offset: [0, 0] };

/** 🧩️ The active `meta:gumball` layer of a scene's `layersJson`, or `null`. */
export function parseCanvas2dGumballMeta(layersJson: string | undefined): Canvas2dGumballMeta | null {
  if (!layersJson) return null;
  try {
    const layers = JSON.parse(layersJson) as readonly { readonly role?: string; readonly gumball?: Canvas2dGumballMeta }[];
    if (!Array.isArray(layers)) return null;
    return layers.find((layer) => layer.role === "meta" && layer.gumball?.active)?.gumball ?? null;
  } catch {
    return null;
  }
}

/** 📷️ The host camera of `Canvas2dHost` (`worldToScreenLogical`): screen = (world − camera) · zoom + viewport / 2 —
 * restated here so the gumball algebra stays free of the host module. */
function worldToScreen(x: number, y: number, view: Canvas2dGumballView): { readonly x: number; readonly y: number } {
  const zoom = view.camera.zoom || 1;
  return { x: (x - view.camera.x) * zoom + view.viewportWidth * 0.5, y: (y - view.camera.y) * zoom + view.viewportHeight * 0.5 };
}

/** 🪞️ The inverse of `worldToScreen` (`screenToWorldLogical`). */
function screenToWorld(x: number, y: number, view: Canvas2dGumballView): { readonly x: number; readonly y: number } {
  const zoom = view.camera.zoom || 1;
  return { x: (x - view.viewportWidth * 0.5) / zoom + view.camera.x, y: (y - view.viewportHeight * 0.5) / zoom + view.camera.y };
}

/** 📍️ The pivot in screen-logical pixels under `view`. */
export function canvas2dGumballPivotScreen(pivotLayer: readonly [number, number], view: Canvas2dGumballView): { readonly x: number; readonly y: number } {
  return worldToScreen(pivotLayer[0], pivotLayer[1], view);
}

/** 🔘️ Every knob's screen centre about `pivot`, topmost first — the order a press resolves them in. */
export function canvas2dGumballKnobs(pivot: { readonly x: number; readonly y: number }): readonly { readonly kind: Exclude<Canvas2dGumballHandleKind, "rotate">; readonly x: number; readonly y: number }[] {
  const diagonal = CANVAS2D_GUMBALL_HANDLE_LENGTH * 0.75;
  const uniform = CANVAS2D_GUMBALL_ROTATE_RADIUS * 0.7;
  return [
    { kind: "scaleUniform", x: pivot.x + uniform, y: pivot.y - uniform },
    { kind: "scaleY", x: pivot.x - diagonal, y: pivot.y + diagonal },
    { kind: "scaleX", x: pivot.x + diagonal, y: pivot.y + diagonal },
    { kind: "moveY", x: pivot.x, y: pivot.y - CANVAS2D_GUMBALL_HANDLE_LENGTH },
    { kind: "moveX", x: pivot.x + CANVAS2D_GUMBALL_HANDLE_LENGTH, y: pivot.y },
  ];
}

function handleEnabled(kind: Canvas2dGumballHandleKind, config: Canvas2dGumballConfig): boolean {
  if (kind === "moveX" || kind === "moveY") return config.moveAxes;
  if (kind === "scaleX" || kind === "scaleY") return config.scaleAxes;
  return kind === "scaleUniform" ? config.scaleUniform : config.rotate;
}

/** 👆️ The handle a press at screen `(x, y)` grabs: the topmost enabled knob within its hit radius, else the turn ring
 * within its band, else `null`. */
export function canvas2dGumballHandleAt(meta: Canvas2dGumballMeta, view: Canvas2dGumballView, x: number, y: number): Canvas2dGumballHandleKind | null {
  if (!meta.active) return null;
  const pivot = canvas2dGumballPivotScreen(meta.pivotLayer, view);
  const knob = canvas2dGumballKnobs(pivot).find((candidate) => handleEnabled(candidate.kind, meta.config) && Math.hypot(x - candidate.x, y - candidate.y) <= CANVAS2D_GUMBALL_HIT_RADIUS);
  if (knob) return knob.kind;
  return meta.config.rotate && Math.abs(Math.hypot(x - pivot.x, y - pivot.y) - CANVAS2D_GUMBALL_ROTATE_RADIUS) <= CANVAS2D_GUMBALL_RING_HIT_BAND ? "rotate" : null;
}

/** 🎚️ The verb one handle drags with. */
export function canvas2dGumballVerb(kind: Canvas2dGumballHandleKind): "translateSelection" | "rotateSelection" | "scaleSelection" {
  if (kind === "moveX" || kind === "moveY") return "translateSelection";
  return kind === "rotate" ? "rotateSelection" : "scaleSelection";
}

/** ✊️ The gesture a press of `kind` at screen `(x, y)` opens, pinning the meta's selection ids and liveness. */
export function canvas2dGumballBegin(meta: Canvas2dGumballMeta, kind: Canvas2dGumballHandleKind, x: number, y: number): Canvas2dGumballGesture {
  return { kind, start: { x, y }, pivotLayer: meta.pivotLayer, modelToLayer: meta.modelToLayer ?? IDENTITY_MAP, ids: [...meta.selectionIds], live: meta.liveDispatch ?? false, total: null, streamed: false };
}

function layerToModel(map: Canvas2dGumballModelToLayer, x: number, y: number): { readonly x: number; readonly y: number } {
  return { x: (x - map.offset[0]) / map.scale[0], y: (y - map.offset[1]) / map.scale[1] };
}

function wrapAngle(angle: number): number {
  return angle - 2 * Math.PI * Math.round(angle / (2 * Math.PI));
}

/** 🧮️ The cumulative pose delta from the press to screen `(x, y)` in model units — a turn's angle unwrapped against the
 * gesture's last total, so a drag past the ring's far side keeps turning instead of jumping a full turn; `null` when it
 * moves nothing. */
export function canvas2dGumballTotal(gesture: Canvas2dGumballGesture, view: Canvas2dGumballView, x: number, y: number): Canvas2dGumballTransformPayload | null {
  const ids = [...gesture.ids];
  const pivot = canvas2dGumballPivotScreen(gesture.pivotLayer, view);
  const { kind, start } = gesture;
  if (kind === "moveX" || kind === "moveY") {
    const before = screenToWorld(start.x, start.y, view);
    const after = screenToWorld(x, y, view);
    const beforeModel = layerToModel(gesture.modelToLayer, before.x, before.y);
    const afterModel = layerToModel(gesture.modelToLayer, after.x, after.y);
    const dx = kind === "moveX" ? afterModel.x - beforeModel.x : 0;
    const dy = kind === "moveY" ? afterModel.y - beforeModel.y : 0;
    if (Math.abs(dx) < MOVE_EPSILON && Math.abs(dy) < MOVE_EPSILON) return null;
    return { action: "translateSelection", args: { ids, dx, dy, dz: 0 } };
  }
  if (kind === "rotate") {
    const raw = Math.atan2(y - pivot.y, x - pivot.x) - Math.atan2(start.y - pivot.y, start.x - pivot.x);
    const previous = gesture.total ? (gesture.total.args.angle as number) : 0;
    const angle = previous + wrapAngle(raw - previous);
    if (Math.abs(angle) < TURN_EPSILON) return null;
    return { action: "rotateSelection", args: { ids, ax: 0, ay: 0, az: 1, angle } };
  }
  const reach0 = Math.hypot(start.x - pivot.x, start.y - pivot.y);
  const reach1 = Math.hypot(x - pivot.x, y - pivot.y);
  const ratio = reach0 > 1e-6 ? reach1 / reach0 : 1;
  if (Math.abs(ratio - 1) < TURN_EPSILON) return null;
  const [sx, sy] = kind === "scaleUniform" ? [ratio, ratio] : kind === "scaleX" ? [ratio, 1] : [1, ratio];
  return { action: "scaleSelection", args: { ids, sx, sy, sz: 1 } };
}

/** ➖️ The increment from `previous` to `current` (offsets subtract, angles subtract, factors divide); `null` when it
 * moves nothing. */
export function canvas2dGumballIncrement(previous: Canvas2dGumballTransformPayload | null, current: Canvas2dGumballTransformPayload): Canvas2dGumballTransformPayload | null {
  if (!previous || previous.action !== current.action) return current;
  const before = previous.args;
  const after = current.args;
  if (current.action === "translateSelection") {
    const dx = (after.dx as number) - (before.dx as number);
    const dy = (after.dy as number) - (before.dy as number);
    return Math.abs(dx) < MOVE_EPSILON && Math.abs(dy) < MOVE_EPSILON ? null : { action: current.action, args: { ...after, dx, dy } };
  }
  if (current.action === "rotateSelection") {
    const angle = (after.angle as number) - (before.angle as number);
    return Math.abs(angle) < TURN_EPSILON ? null : { action: current.action, args: { ...after, angle } };
  }
  const sx = (after.sx as number) / (before.sx as number);
  const sy = (after.sy as number) / (before.sy as number);
  return Math.abs(sx - 1) < TURN_EPSILON && Math.abs(sy - 1) < TURN_EPSILON ? null : { action: current.action, args: { ...after, sx, sy } };
}

/** 🛑️ The pose delta that moves nothing for a handle — a commit whose tail moved nothing carries it. */
export function canvas2dGumballIdentity(kind: Canvas2dGumballHandleKind, ids: readonly string[]): Canvas2dGumballTransformPayload {
  const action = canvas2dGumballVerb(kind);
  if (action === "translateSelection") return { action, args: { ids: [...ids], dx: 0, dy: 0, dz: 0 } };
  if (action === "rotateSelection") return { action, args: { ids: [...ids], ax: 0, ay: 0, az: 1, angle: 0 } };
  return { action, args: { ids: [...ids], sx: 1, sy: 1, sz: 1 } };
}

/** 🧾️ `payload` stamped with its tool-transaction `phase`. */
function phased(payload: Canvas2dGumballTransformPayload, phase: "stream" | "commit"): Canvas2dGumballTransformPayload {
  return { action: payload.action, args: { ...payload.args, phase } };
}

/** 🌊️ A pointer move: a live gesture streams the increment since its last tick (`phase: stream`); a non-live one only
 * records its total for the local preview. */
export function canvas2dGumballDrag(gesture: Canvas2dGumballGesture, view: Canvas2dGumballView, x: number, y: number): Canvas2dGumballDragStep {
  const total = canvas2dGumballTotal(gesture, view, x, y);
  if (!total) return { gesture, dispatch: null };
  if (!gesture.live) return { gesture: { ...gesture, total }, dispatch: null };
  const step = canvas2dGumballIncrement(gesture.total, total);
  if (!step) return { gesture, dispatch: null };
  return { gesture: { ...gesture, total, streamed: true }, dispatch: phased(step, "stream") };
}

/** 💾️ The release at screen `(x, y)`: a live gesture commits its ONE transaction with the remaining tail (the identity
 * when nothing is left; nothing when it never moved); a non-live one dispatches its whole pose delta as ONE one-shot. */
export function canvas2dGumballRelease(gesture: Canvas2dGumballGesture, view: Canvas2dGumballView, x: number, y: number): Canvas2dGumballTransformPayload | null {
  const total = canvas2dGumballTotal(gesture, view, x, y);
  if (!gesture.live) return total;
  const tail = total ? canvas2dGumballIncrement(gesture.total, total) : null;
  if (!gesture.streamed && !tail) return null;
  return phased(tail ?? canvas2dGumballIdentity(gesture.kind, gesture.ids), "commit");
}

/** 🧯️ A host cancel: a live gesture that streamed drops its open transaction with zero trace (`phase: abort`); anything
 * else dispatches nothing. */
export function canvas2dGumballCancel(gesture: Canvas2dGumballGesture, reason: Canvas2dGumballCancelReason): Canvas2dGumballTransformPayload | null {
  if (!gesture.live || !gesture.streamed) return null;
  return { action: canvas2dGumballVerb(gesture.kind), args: { ids: [...gesture.ids], phase: "abort", reason } };
}

type Canvas2dGumballOverlayProps = {
  readonly layersJson: string | undefined;
  readonly activeUtility: string | undefined;
  readonly camera: CanvasCamera;
  readonly viewportWidth: number;
  readonly viewportHeight: number;
  readonly onDispatch: (action: string, args: Record<string, unknown>) => void;
};

/** 🫥️ What a non-live gesture paints while it previews locally: the gesture and the pointer it last saw. */
type LocalPreview = {
  readonly gesture: Canvas2dGumballGesture;
  readonly pointer: { readonly x: number; readonly y: number };
};

export function Canvas2dGumballOverlay({ layersJson, activeUtility, camera, viewportWidth, viewportHeight, onDispatch }: Canvas2dGumballOverlayProps) {
  const meta = useMemo(() => parseCanvas2dGumballMeta(layersJson), [layersJson]);
  const view: Canvas2dGumballView = useMemo(() => ({ camera, viewportWidth, viewportHeight }), [camera, viewportHeight, viewportWidth]);
  const gestureRef = useRef<Canvas2dGumballGesture | null>(null);
  const [preview, setPreview] = useState<LocalPreview | null>(null);
  const visible = activeUtility === "transform" && meta != null && viewportWidth > 0 && viewportHeight > 0;
  const pivotScreen = useMemo(() => (meta ? canvas2dGumballPivotScreen(meta.pivotLayer, view) : null), [meta, view]);

  const send = useCallback(
    (payload: Canvas2dGumballTransformPayload | null) => {
      if (payload) onDispatch(payload.action, payload.args);
    },
    [onDispatch],
  );

  const end = useCallback(() => {
    gestureRef.current = null;
    setPreview(null);
  }, []);

  const cancel = useCallback(
    (reason: Canvas2dGumballCancelReason) => {
      const gesture = gestureRef.current;
      if (gesture) send(canvas2dGumballCancel(gesture, reason));
      end();
    },
    [end, send],
  );
  const cancelRef = useRef(cancel);
  cancelRef.current = cancel;
  useEffect(() => {
    const onBlur = () => {
      if (gestureRef.current) cancelRef.current("blur");
    };
    window.addEventListener("blur", onBlur);
    return () => {
      window.removeEventListener("blur", onBlur);
      if (gestureRef.current) cancelRef.current("captureLost");
    };
  }, []);

  const localPoint = useCallback((event: ReactPointerEvent<Element>) => {
    const host = event.currentTarget.closest(".semio-canvas-2d-host") as HTMLElement | null;
    const rect = host?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  const onPointerDown = useCallback(
    (kind: Canvas2dGumballHandleKind, event: ReactPointerEvent<SVGElement>) => {
      if (!meta) return;
      event.preventDefault();
      event.stopPropagation();
      (event.currentTarget as SVGElement).setPointerCapture(event.pointerId);
      const point = localPoint(event);
      const gesture = canvas2dGumballBegin(meta, kind, point.x, point.y);
      gestureRef.current = gesture;
      setPreview({ gesture, pointer: point });
    },
    [localPoint, meta],
  );

  const onPointerMove = useCallback(
    (event: ReactPointerEvent<SVGSVGElement>) => {
      const gesture = gestureRef.current;
      if (!gesture) return;
      event.preventDefault();
      event.stopPropagation();
      const point = localPoint(event);
      const step = canvas2dGumballDrag(gesture, view, point.x, point.y);
      gestureRef.current = step.gesture;
      send(step.dispatch);
      if (!gesture.live) setPreview({ gesture: step.gesture, pointer: point });
    },
    [localPoint, send, view],
  );

  const onPointerUp = useCallback(
    (event: ReactPointerEvent<SVGSVGElement>) => {
      const gesture = gestureRef.current;
      if (!gesture) return;
      event.preventDefault();
      event.stopPropagation();
      const point = localPoint(event);
      send(canvas2dGumballRelease(gesture, view, point.x, point.y));
      end();
    },
    [end, localPoint, send, view],
  );

  const onPointerCancel = useCallback(
    (event: ReactPointerEvent<SVGSVGElement>) => {
      if (!gestureRef.current) return;
      event.preventDefault();
      event.stopPropagation();
      cancel("captureLost");
    },
    [cancel],
  );

  if (!visible || !meta || !pivotScreen) return null;
  const { config } = meta;
  const cx = pivotScreen.x;
  const cy = pivotScreen.y;
  const knobs = canvas2dGumballKnobs(pivotScreen);
  const knob = (kind: Exclude<Canvas2dGumballHandleKind, "rotate">) => knobs.find((candidate) => candidate.kind === kind) ?? { x: cx, y: cy };
  const ghost = preview && !preview.gesture.live ? preview : null;

  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden onPointerMove={onPointerMove} onPointerUp={onPointerUp} onPointerCancel={onPointerCancel} onLostPointerCapture={onPointerCancel}>
      {config.rotate ? (
        <>
          <circle cx={cx} cy={cy} r={CANVAS2D_GUMBALL_ROTATE_RADIUS} fill="none" stroke={SPATIAL_AXIS_COLOR_REFS.y} strokeWidth={1.5} strokeDasharray="4 4" />
          <circle
            className="cursor-grab"
            style={{ pointerEvents: "stroke" }}
            cx={cx}
            cy={cy}
            r={CANVAS2D_GUMBALL_ROTATE_RADIUS}
            fill="none"
            stroke="transparent"
            strokeWidth={CANVAS2D_GUMBALL_RING_HIT_BAND * 2}
            onPointerDown={(event) => onPointerDown("rotate", event)}
          />
        </>
      ) : null}
      {config.moveAxes ? (
        <>
          <line x1={cx} y1={cy} x2={knob("moveX").x} y2={knob("moveX").y} stroke={SPATIAL_AXIS_COLOR_REFS.x} strokeWidth={2} />
          <circle className="pointer-events-auto cursor-ew-resize" cx={knob("moveX").x} cy={knob("moveX").y} r={CANVAS2D_GUMBALL_HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.x} onPointerDown={(event) => onPointerDown("moveX", event)} />
          <line x1={cx} y1={cy} x2={knob("moveY").x} y2={knob("moveY").y} stroke={SPATIAL_AXIS_COLOR_REFS.y} strokeWidth={2} />
          <circle className="pointer-events-auto cursor-ns-resize" cx={knob("moveY").x} cy={knob("moveY").y} r={CANVAS2D_GUMBALL_HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.y} onPointerDown={(event) => onPointerDown("moveY", event)} />
        </>
      ) : null}
      {config.scaleAxes ? (
        <>
          <circle className="pointer-events-auto cursor-ew-resize" cx={knob("scaleX").x} cy={knob("scaleX").y} r={CANVAS2D_GUMBALL_HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.x} opacity={0.85} onPointerDown={(event) => onPointerDown("scaleX", event)} />
          <circle className="pointer-events-auto cursor-ns-resize" cx={knob("scaleY").x} cy={knob("scaleY").y} r={CANVAS2D_GUMBALL_HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.y} opacity={0.85} onPointerDown={(event) => onPointerDown("scaleY", event)} />
        </>
      ) : null}
      {config.scaleUniform ? (
        <circle className="pointer-events-auto cursor-nwse-resize" cx={knob("scaleUniform").x} cy={knob("scaleUniform").y} r={CANVAS2D_GUMBALL_HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.z} onPointerDown={(event) => onPointerDown("scaleUniform", event)} />
      ) : null}
      <circle cx={cx} cy={cy} r={4} fill="white" stroke="#94a3b8" strokeWidth={1} />
      {ghost ? <Canvas2dGumballGhost preview={ghost} pivot={pivotScreen} /> : null}
      {preview ? <circle cx={cx} cy={cy} r={3} fill="rgba(250,204,21,0.9)" /> : null}
    </svg>
  );
}

/** 👻️ A non-live gesture's local preview in screen space: the dragged axis offset for a move, the arm to the pointer for
 * a turn, the reach ring for a scaling. The document only changes on release. */
function Canvas2dGumballGhost({ preview, pivot }: { readonly preview: LocalPreview; readonly pivot: { readonly x: number; readonly y: number } }) {
  const { gesture, pointer } = preview;
  const stroke = "rgba(250,204,21,0.9)";
  if (gesture.kind === "moveX" || gesture.kind === "moveY") {
    const x = gesture.kind === "moveX" ? pivot.x + pointer.x - gesture.start.x : pivot.x;
    const y = gesture.kind === "moveY" ? pivot.y + pointer.y - gesture.start.y : pivot.y;
    return (
      <>
        <line x1={pivot.x} y1={pivot.y} x2={x} y2={y} stroke={stroke} strokeWidth={1.5} strokeDasharray="3 3" />
        <circle cx={x} cy={y} r={5} fill="none" stroke={stroke} strokeWidth={2} />
      </>
    );
  }
  if (gesture.kind === "rotate") return <line x1={pivot.x} y1={pivot.y} x2={pointer.x} y2={pointer.y} stroke={stroke} strokeWidth={2} />;
  return <circle cx={pivot.x} cy={pivot.y} r={Math.hypot(pointer.x - pivot.x, pointer.y - pivot.y)} fill="none" stroke={stroke} strokeWidth={1.5} strokeDasharray="3 3" />;
}
//#endregion 🧭️Canvas2dGumballOverlay
