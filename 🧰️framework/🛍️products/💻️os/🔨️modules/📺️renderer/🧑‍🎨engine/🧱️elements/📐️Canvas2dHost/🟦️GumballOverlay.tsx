// #region 🧭️Canvas2dGumballOverlay
/** @emoji 🧭️ Screen-space 2D transform gumball driven by the plugin `meta:gumball` layer. */
import { useCallback, useMemo, useRef, useState, type PointerEvent as ReactPointerEvent } from "react";
import { SPATIAL_AXIS_COLOR_REFS } from "@semio-tech/ui-styling";
import { type CanvasCamera, screenToWorldLogical, worldToScreenLogical } from "./🟦️.tsx";

const FEM2D_SCALE = 20;
const FEM2D_ORIGIN = 40;
const HANDLE_LENGTH = 56;
const ROTATE_RADIUS = 44;
const HIT_RADIUS = 10;

export type Canvas2dGumballConfig = {
  readonly moveAxes: boolean;
  readonly rotate: boolean;
  readonly scaleAxes: boolean;
  readonly scaleUniform: boolean;
};

export type Canvas2dGumballSpace = "fem2d" | "world";

export type Canvas2dGumballMeta = {
  readonly active: boolean;
  readonly pivotLayer: readonly [number, number];
  readonly pivotModel: readonly [number, number];
  readonly selectionIds: readonly string[];
  readonly config: Canvas2dGumballConfig;
  readonly space?: Canvas2dGumballSpace;
};

type GumballHandleKind = "moveX" | "moveY" | "rotate" | "scaleX" | "scaleY" | "scaleUniform";

type DragState = {
  readonly kind: GumballHandleKind;
  readonly startScreen: { readonly x: number; readonly y: number };
  readonly startModelPivot: readonly [number, number];
};

export type Canvas2dGumballTransformPayload = {
  readonly action: string;
  readonly args: Record<string, unknown>;
};

function layerToModel(layerX: number, layerY: number, space: Canvas2dGumballSpace): { readonly x: number; readonly y: number } {
  if (space === "world") return { x: layerX, y: layerY };
  return { x: (layerX - FEM2D_ORIGIN) / FEM2D_SCALE, y: -(layerY - FEM2D_ORIGIN) / FEM2D_SCALE };
}

function modelToLayer(modelX: number, modelY: number, space: Canvas2dGumballSpace): { readonly x: number; readonly y: number } {
  if (space === "world") return { x: modelX, y: modelY };
  return { x: modelX * FEM2D_SCALE + FEM2D_ORIGIN, y: -modelY * FEM2D_SCALE + FEM2D_ORIGIN };
}

export function parseCanvas2dGumballMeta(layersJson: string | undefined): Canvas2dGumballMeta | null {
  if (!layersJson) return null;
  try {
    const layers = JSON.parse(layersJson) as readonly { readonly role?: string; readonly gumball?: Canvas2dGumballMeta }[];
    if (!Array.isArray(layers)) return null;
    const meta = layers.find((layer) => layer.role === "meta" && layer.gumball?.active)?.gumball ?? null;
    return meta?.active ? meta : null;
  } catch {
    return null;
  }
}

export function canvas2dGumballTransformDelta(
  kind: GumballHandleKind,
  drag: DragState,
  screenX: number,
  screenY: number,
  camera: CanvasCamera,
  viewportWidth: number,
  viewportHeight: number,
  selectionIds: readonly string[],
  space: Canvas2dGumballSpace = "fem2d",
): Canvas2dGumballTransformPayload | null {
  const base = { ids: [...selectionIds] };
  const pivotLayer = modelToLayer(drag.startModelPivot[0], drag.startModelPivot[1], space);
  const pivotScreen = worldToScreenLogical(pivotLayer.x, pivotLayer.y, camera, viewportWidth, viewportHeight);
  if (kind === "moveX" || kind === "moveY") {
    const before = screenToWorldLogical(drag.startScreen.x, drag.startScreen.y, camera, viewportWidth, viewportHeight);
    const after = screenToWorldLogical(screenX, screenY, camera, viewportWidth, viewportHeight);
    const beforeModel = layerToModel(before.x, before.y, space);
    const afterModel = layerToModel(after.x, after.y, space);
    const dx = kind === "moveX" ? afterModel.x - beforeModel.x : 0;
    const dy = kind === "moveY" ? afterModel.y - beforeModel.y : 0;
    if (Math.abs(dx) < 1e-9 && Math.abs(dy) < 1e-9) return null;
    return { action: "translateSelection", args: { ...base, dx, dy, dz: 0 } };
  }
  if (kind === "rotate") {
    const a0 = Math.atan2(drag.startScreen.y - pivotScreen.y, drag.startScreen.x - pivotScreen.x);
    const a1 = Math.atan2(screenY - pivotScreen.y, screenX - pivotScreen.x);
    const angle = a1 - a0;
    if (Math.abs(angle) < 1e-6) return null;
    return { action: "rotateSelection", args: { ...base, ax: 0, ay: 0, az: 1, angle } };
  }
  const dist0 = Math.hypot(drag.startScreen.x - pivotScreen.x, drag.startScreen.y - pivotScreen.y);
  const dist1 = Math.hypot(screenX - pivotScreen.x, screenY - pivotScreen.y);
  const ratio = dist0 > 1e-6 ? dist1 / dist0 : 1;
  if (Math.abs(ratio - 1) < 1e-6) return null;
  if (kind === "scaleUniform") {
    return { action: "scaleSelection", args: { ...base, sx: ratio, sy: ratio, sz: 1 } };
  }
  if (kind === "scaleX") {
    return { action: "scaleSelection", args: { ...base, sx: ratio, sy: 1, sz: 1 } };
  }
  return { action: "scaleSelection", args: { ...base, sx: 1, sy: ratio, sz: 1 } };
}

/** 🔁️ Turns a cumulative drag total into the next incremental plugin dispatch (for live coalesced amends). */
export function canvas2dGumballTransformStep(
  kind: GumballHandleKind,
  drag: DragState,
  screenX: number,
  screenY: number,
  camera: CanvasCamera,
  viewportWidth: number,
  viewportHeight: number,
  selectionIds: readonly string[],
  previousTotal: Canvas2dGumballTransformPayload | null,
  space: Canvas2dGumballSpace = "fem2d",
): { readonly dispatch: Canvas2dGumballTransformPayload; readonly total: Canvas2dGumballTransformPayload } | null {
  const total = canvas2dGumballTransformDelta(kind, drag, screenX, screenY, camera, viewportWidth, viewportHeight, selectionIds, space);
  if (!total) return null;
  if (!previousTotal || previousTotal.action !== total.action) {
    return { dispatch: total, total };
  }
  if (total.action === "translateSelection") {
    const dx = (total.args.dx as number) - (previousTotal.args.dx as number);
    const dy = (total.args.dy as number) - (previousTotal.args.dy as number);
    if (Math.abs(dx) < 1e-9 && Math.abs(dy) < 1e-9) return null;
    return { dispatch: { action: total.action, args: { ...total.args, dx, dy } }, total };
  }
  if (total.action === "rotateSelection") {
    const angle = (total.args.angle as number) - (previousTotal.args.angle as number);
    if (Math.abs(angle) < 1e-6) return null;
    return { dispatch: { action: total.action, args: { ...total.args, angle } }, total };
  }
  const sx = (total.args.sx as number) / (previousTotal.args.sx as number);
  const sy = (total.args.sy as number) / (previousTotal.args.sy as number);
  const sz = (total.args.sz as number) / (previousTotal.args.sz as number);
  if (Math.abs(sx - 1) < 1e-6 && Math.abs(sy - 1) < 1e-6 && Math.abs(sz - 1) < 1e-6) return null;
  return { dispatch: { action: total.action, args: { ...total.args, sx, sy, sz } }, total };
}

type Canvas2dGumballOverlayProps = {
  readonly layersJson: string | undefined;
  readonly activeUtility: string | undefined;
  readonly camera: CanvasCamera;
  readonly viewportWidth: number;
  readonly viewportHeight: number;
  readonly onDispatch: (action: string, args: Record<string, unknown>) => void;
};

export function Canvas2dGumballOverlay({ layersJson, activeUtility, camera, viewportWidth, viewportHeight, onDispatch }: Canvas2dGumballOverlayProps) {
  const meta = useMemo(() => parseCanvas2dGumballMeta(layersJson), [layersJson]);
  const dragRef = useRef<DragState | null>(null);
  const totalRef = useRef<Canvas2dGumballTransformPayload | null>(null);
  const [preview, setPreview] = useState<DragState | null>(null);

  const visible = activeUtility === "transform" && meta != null && viewportWidth > 0 && viewportHeight > 0;
  const pivotScreen = useMemo(() => {
    if (!meta) return null;
    const [lx, ly] = meta.pivotLayer;
    return worldToScreenLogical(lx, ly, camera, viewportWidth, viewportHeight);
  }, [meta, camera, viewportWidth, viewportHeight]);

  const applyDragAt = useCallback(
    (screenX: number, screenY: number) => {
      const drag = dragRef.current;
      if (!drag || !meta) return;
      const step = canvas2dGumballTransformStep(drag.kind, drag, screenX, screenY, camera, viewportWidth, viewportHeight, meta.selectionIds, totalRef.current, meta.space ?? "fem2d");
      if (!step) return;
      totalRef.current = step.total;
      onDispatch(step.dispatch.action, step.dispatch.args);
    },
    [camera, meta, onDispatch, viewportHeight, viewportWidth],
  );

  const endDrag = useCallback(() => {
    dragRef.current = null;
    totalRef.current = null;
    setPreview(null);
  }, []);

  const localPoint = useCallback((event: ReactPointerEvent<Element>) => {
    const host = event.currentTarget.closest(".semio-canvas-2d-host") as HTMLElement | null;
    const rect = host?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  const onPointerDown = useCallback(
    (kind: GumballHandleKind, event: ReactPointerEvent<SVGElement>) => {
      if (!meta || !pivotScreen) return;
      event.preventDefault();
      event.stopPropagation();
      (event.currentTarget as SVGElement).setPointerCapture(event.pointerId);
      dragRef.current = { kind, startScreen: localPoint(event), startModelPivot: meta.pivotModel };
      totalRef.current = null;
      setPreview(dragRef.current);
    },
    [localPoint, meta, pivotScreen],
  );

  const onPointerMove = useCallback(
    (event: ReactPointerEvent<SVGSVGElement>) => {
      if (!dragRef.current) return;
      event.preventDefault();
      event.stopPropagation();
      const point = localPoint(event);
      applyDragAt(point.x, point.y);
    },
    [applyDragAt, localPoint],
  );

  const onPointerUp = useCallback(
    (event: ReactPointerEvent<SVGSVGElement>) => {
      if (!dragRef.current) return;
      event.preventDefault();
      event.stopPropagation();
      endDrag();
    },
    [endDrag],
  );

  if (!visible || !meta || !pivotScreen) return null;
  const { config } = meta;
  const cx = pivotScreen.x;
  const cy = pivotScreen.y;

  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden onPointerMove={onPointerMove} onPointerUp={onPointerUp} onPointerCancel={onPointerUp}>
      {config.rotate ? (
        <circle
          className="pointer-events-auto cursor-grab"
          cx={cx}
          cy={cy}
          r={ROTATE_RADIUS}
          fill="none"
          stroke={SPATIAL_AXIS_COLOR_REFS.y}
          strokeWidth={1.5}
          strokeDasharray="4 4"
          onPointerDown={(event) => onPointerDown("rotate", event)}
        />
      ) : null}
      {config.moveAxes ? (
        <>
          <line x1={cx} y1={cy} x2={cx + HANDLE_LENGTH} y2={cy} stroke={SPATIAL_AXIS_COLOR_REFS.x} strokeWidth={2} />
          <circle className="pointer-events-auto cursor-ew-resize" cx={cx + HANDLE_LENGTH} cy={cy} r={HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.x} onPointerDown={(event) => onPointerDown("moveX", event)} />
          <line x1={cx} y1={cy} x2={cx} y2={cy - HANDLE_LENGTH} stroke={SPATIAL_AXIS_COLOR_REFS.y} strokeWidth={2} />
          <circle className="pointer-events-auto cursor-ns-resize" cx={cx} cy={cy - HANDLE_LENGTH} r={HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.y} onPointerDown={(event) => onPointerDown("moveY", event)} />
        </>
      ) : null}
      {config.scaleAxes ? (
        <>
          <circle className="pointer-events-auto cursor-ew-resize" cx={cx + HANDLE_LENGTH * 0.75} cy={cy + HANDLE_LENGTH * 0.75} r={HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.x} opacity={0.85} onPointerDown={(event) => onPointerDown("scaleX", event)} />
          <circle className="pointer-events-auto cursor-ns-resize" cx={cx - HANDLE_LENGTH * 0.75} cy={cy + HANDLE_LENGTH * 0.75} r={HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.y} opacity={0.85} onPointerDown={(event) => onPointerDown("scaleY", event)} />
        </>
      ) : null}
      {config.scaleUniform ? (
        <circle className="pointer-events-auto cursor-nwse-resize" cx={cx + ROTATE_RADIUS * 0.7} cy={cy - ROTATE_RADIUS * 0.7} r={HIT_RADIUS} fill={SPATIAL_AXIS_COLOR_REFS.z} onPointerDown={(event) => onPointerDown("scaleUniform", event)} />
      ) : null}
      <circle cx={cx} cy={cy} r={4} fill="white" stroke="#94a3b8" strokeWidth={1} />
      {preview ? <circle cx={cx} cy={cy} r={3} fill="rgba(250,204,21,0.9)" /> : null}
    </svg>
  );
}
//#endregion 🧭️Canvas2dGumballOverlay
