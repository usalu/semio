// #region 🧲️Header
/** @emoji 🧩️ `ToolRunTrace2dLayer` — the canvas-2d twin of `🌐️World3dHost/⏯️tool-run-trace`: the same keyed
 * record store fed by `Canvas2dScene.toolRunTrace`, painted as one batched path fill per
 * `(shape, verdict)` on an overlay canvas, with the same verdict tokens, age fade and newest-`testing`
 * highlight. Placement subjects draw; instance and entity subjects are counted only.
 * @see ../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useMemo, useRef } from "react";
import { resolveColorHex } from "@semio-tech/ui-styling";
import {
  TOOL_RUN_TRACE_HIGHLIGHT_PAINT,
  TOOL_RUN_TRACE_METRICS,
  TOOL_RUN_TRACE_VERDICT_PAINT,
  TOOL_RUN_TRACE_VISIBLE_ALL,
  toolRunTraceDataAttributes,
  toolRunTraceFade,
  toolRunTraceShows,
  usePrefersReducedMotion,
  useToolRunTraceStore,
  type ToolRunTraceRecordStore,
  type ToolRunTraceVisibility,
} from "../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import type { ToolRunTraceCursor } from "../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🖌️Paint
/** 🧭️ The canvas camera the host's own `worldToScreenLogical` uses: screen = (world − camera) · zoom + viewport / 2. */
export type ToolRunTrace2dCamera = { readonly x: number; readonly y: number; readonly zoom: number };

/** 🎨️ Resolved `#rrggbb` per verdict plus the highlight — resolve once per theme, not per frame. */
export type ToolRunTrace2dPalette = { readonly fill: Readonly<Record<keyof typeof TOOL_RUN_TRACE_VERDICT_PAINT, string>>; readonly highlight: string };

export function resolveToolRunTrace2dPalette(): ToolRunTrace2dPalette {
  return {
    fill: { testing: resolveColorHex(TOOL_RUN_TRACE_VERDICT_PAINT.testing.fill), success: resolveColorHex(TOOL_RUN_TRACE_VERDICT_PAINT.success.fill), warning: resolveColorHex(TOOL_RUN_TRACE_VERDICT_PAINT.warning.fill), danger: resolveColorHex(TOOL_RUN_TRACE_VERDICT_PAINT.danger.fill) },
    highlight: resolveColorHex(TOOL_RUN_TRACE_HIGHLIGHT_PAINT),
  };
}

/** 🖌️ The 2d paint surface this layer needs — a `CanvasRenderingContext2D` subset, so probes can record it. */
export type ToolRunTrace2dContext = Pick<CanvasRenderingContext2D, "save" | "restore" | "setTransform" | "fill" | "stroke" | "fillStyle" | "strokeStyle" | "globalAlpha" | "lineWidth">;

/** 📊️ What one paint call drew: fills per batch id, and whether the newest `testing` outline was stroked. */
export type ToolRunTrace2dPaintReport = { readonly fills: ReadonlyMap<string, number>; readonly highlighted: boolean };

/** 🖌️ Paints every visible placement batch with one fill style per `(shape, verdict)`. Each record is
 * one path fill at its placement transform, faded by age down to the floor token; the newest `testing`
 * record is stroked in the highlight token, pulsing unless `still`. */
export function paintToolRunTrace2d(
  ctx: ToolRunTrace2dContext,
  store: ToolRunTraceRecordStore,
  camera: ToolRunTrace2dCamera,
  viewport: { readonly width: number; readonly height: number; readonly pixelRatio: number },
  pathForShape: (shape: number) => Path2D | null,
  palette: ToolRunTrace2dPalette,
  visibility: ToolRunTraceVisibility = TOOL_RUN_TRACE_VISIBLE_ALL,
  pulse = 1,
): ToolRunTrace2dPaintReport {
  const fills = new Map<string, number>();
  const newestKey = store.newestTesting;
  const newestStamp = store.newestStamp;
  const zoom = camera.zoom || 1;
  const ratio = viewport.pixelRatio;
  let highlighted = false;
  ctx.save();
  for (const batch of store.batches()) {
    if (batch.family !== "placement2d" || !toolRunTraceShows(visibility, batch.verdict)) continue;
    const path = pathForShape(batch.index);
    if (!path) continue;
    const paint = TOOL_RUN_TRACE_VERDICT_PAINT[batch.verdict];
    ctx.fillStyle = palette.fill[batch.verdict];
    let drawn = 0;
    for (let at = 0; at < batch.count; at += 1) {
      const m = batch.matrices;
      const o = at * 16;
      ctx.setTransform(ratio * zoom * m[o]!, ratio * zoom * m[o + 1]!, ratio * zoom * m[o + 4]!, ratio * zoom * m[o + 5]!, ratio * ((m[o + 12]! - camera.x) * zoom + viewport.width / 2), ratio * ((m[o + 13]! - camera.y) * zoom + viewport.height / 2));
      const newest = batch.keys[at] === newestKey;
      ctx.globalAlpha = paint.opacity * (newest ? 1 : toolRunTraceFade(newestStamp - batch.stamps[at]!));
      ctx.fill(path);
      drawn += 1;
      if (newest) {
        ctx.globalAlpha = pulse;
        ctx.strokeStyle = palette.highlight;
        ctx.lineWidth = TOOL_RUN_TRACE_METRICS.testingOutlineWidth / zoom;
        ctx.stroke(path);
        highlighted = true;
      }
    }
    fills.set(batch.id, drawn);
  }
  ctx.restore();
  return { fills, highlighted };
}
//#endregion 🖌️Paint

//#region 🧩️Layer
export type ToolRunTrace2dLayerProps = {
  /** 🚚️ `Canvas2dScene.toolRunTrace`. */
  readonly lane: string | null | undefined;
  readonly camera: ToolRunTrace2dCamera;
  /** 🔷️ Outline of the plugin's `shape` index in world units around its origin. */
  readonly pathForShape: (shape: number) => Path2D | null;
  readonly visibility?: ToolRunTraceVisibility;
  readonly reducedMotion?: boolean;
  readonly onCursor?: (cursor: ToolRunTraceCursor) => void;
};

/** 🧩️ An overlay canvas over the host's canvas-2d surface. Carries the `data-tool-run-*` probe counters. */
export function ToolRunTrace2dLayer({ lane, camera, pathForShape, visibility = TOOL_RUN_TRACE_VISIBLE_ALL, reducedMotion, onCursor }: ToolRunTrace2dLayerProps) {
  const { store, version } = useToolRunTraceStore(lane, onCursor);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const prefersReduced = usePrefersReducedMotion();
  const still = reducedMotion ?? prefersReduced;
  const palette = useMemo(resolveToolRunTrace2dPalette, []);
  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext("2d");
    if (!canvas || !ctx) return;
    let frame = 0;
    const paint = (time: number) => {
      const pixelRatio = window.devicePixelRatio || 1;
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;
      if (canvas.width !== Math.round(width * pixelRatio) || canvas.height !== Math.round(height * pixelRatio)) {
        canvas.width = Math.round(width * pixelRatio);
        canvas.height = Math.round(height * pixelRatio);
      }
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const pulse = still ? 1 : 0.6 + 0.4 * Math.abs(Math.sin((time * Math.PI) / TOOL_RUN_TRACE_METRICS.testingPulseMs));
      paintToolRunTrace2d(ctx, store, camera, { width, height, pixelRatio }, pathForShape, palette, visibility, pulse);
      if (!still && store.newestTesting !== null) frame = requestAnimationFrame(paint);
    };
    frame = requestAnimationFrame(paint);
    return () => cancelAnimationFrame(frame);
  }, [store, version, camera.x, camera.y, camera.zoom, pathForShape, palette, visibility, still]);
  return <canvas ref={canvasRef} className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden="true" data-slot="tool-run-trace-2d" {...toolRunTraceDataAttributes(store)} />;
}
//#endregion 🧩️Layer
