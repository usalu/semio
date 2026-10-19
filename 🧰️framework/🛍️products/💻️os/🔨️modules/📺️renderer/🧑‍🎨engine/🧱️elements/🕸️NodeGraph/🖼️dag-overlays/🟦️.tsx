/** 🖼️ The canvas overlays shared by both engines: overlay row and camera types, world and screen geometry, content framing, label and slider parsing, label painting, marquee, slider lanes and align chrome.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React, { useCallback, useEffect, useRef } from "react";
import { currentStylingAppearanceName, resolveColorHex } from "@semio-tech/ui-styling";
import {
  cn,
  glassClass,
  Slider,
  type IntroductionResolvedGeometry,
  type IntroductionSurfaceResolver
} from "@semio-tech/ui-react";
import {
  continuousPressIdentity,
  createContinuousGestureLane,
  nodeGraphActions,
  type ContinuousGestureAbortReason,
  type ContinuousGestureLane,
  type NodeGraphNodeRecord,
  type NodeGraphScene
} from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework/packs";
import { type FlowTask } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import { flowJsonText } from "../🧰️graph-core/🟦️.ts";

//#region 🔖️NodeGraphHost
//#region 🔖️graph-canvas-overlays
//#region DagOverlayTypes
export type DagLabelOverlayRow = {
  readonly id: string;
  readonly kind?: "port" | "node" | string;
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
  /** 📐️ The caption's own screen-width budget, published by the host: only it knows whether a
   * caption sits inside the node body or above it. */
  readonly maxScreenW?: number;
  readonly maxScreenH?: number;
  readonly ghost?: boolean;
};

export type DagPreselectSnapshot = {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
};

export type DagLabelOverlayInteraction = {
  readonly hoveredId: string | null;
  readonly selectedIds: readonly string[];
  readonly preselect: DagPreselectSnapshot;
  readonly dimmedIds?: readonly string[];
  /** ✨️ Extra ids — nodes, edges, or `"{nodeId}@{portId}"` ports — painted highlighted regardless
   * of preselection, e.g. `NodeGraphScene.highlighted` pushed in by a plugin. */
  readonly highlightIds?: readonly string[];
};

export type DagMarqueeOverlay = {
  readonly kind: "rect" | "lasso";
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly points?: readonly { readonly x: number; readonly y: number }[];
  readonly coverage?: "full" | "partial";
};

export type DagCameraState = { readonly x: number; readonly y: number; readonly zoom: number };

export type DagSliderOverlayRow = {
  readonly widgetId: string;
  readonly label: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
  /** 🔢 Readout typography the board publishes alongside the row, in SCREEN pixels — the overlay
   * divides by zoom because its wrapper is already scaled. Absent on a board that predates it. */
  readonly fontScreenPx?: number;
  readonly gapScreenPx?: number;
};

/** 🔢 The ONE text a published slider value is read as — the readout beside the track and the
 * `aria-valuenow` a screen reader announces are this same number, never two. */
export function dagSliderValueText(value: number): string {
  return value.toFixed(1);
}

/** 📐️ A screen-pixel metric the board published, or the overlay's own floor when the board is older
 * than the field — never `NaN`, which would silently drop the style it lands in. */
function screenMetric(published: number | undefined, fallback: number): number {
  return typeof published === "number" && Number.isFinite(published) && published > 0 ? published : fallback;
}

export type DagSelectionBounds = {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
};
//#endregion DagOverlayTypes

//#region DagOverlayGeometry
export function parseDagCameraState(json: string): DagCameraState {
  try {
    const parsed = JSON.parse(json) as Partial<DagCameraState>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

export function dagWorldToScreen(camera: DagCameraState, width: number, height: number, wx: number, wy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (wx - camera.x) * zoom + cx, y: (wy - camera.y) * zoom + cy };
}

export function dagScreenToWorld(camera: DagCameraState, width: number, height: number, sx: number, sy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (sx - cx) / zoom + camera.x, y: (sy - cy) / zoom + camera.y };
}
//#endregion DagOverlayGeometry

//#region 📷️ContentFraming
/** 🖼️ Screen margin a fit leaves around the graph it frames, per side. Twin of
 * `canvas::camera::CONTENT_FIT_PADDING_PX`. */
export const DAG_CONTENT_FIT_PADDING_PX = 24;
/** 🖼️ How much of the graph a STORED camera must already show to be adopted on a first attach. */
export const DAG_CONTENT_FRAMED_MIN_COVERAGE = 0.85;
/** 🖼️ How little of the graph has to be left on screen before a graph that CHANGED under a live
 * camera (an example switch) is re-fitted. Far below {@link DAG_CONTENT_FRAMED_MIN_COVERAGE}: a
 * camera the viewer set is never yanked back for an ordinary edit. */
export const DAG_CONTENT_REFIT_MAX_COVERAGE = 0.05;
const DAG_CAMERA_ZOOM_MIN = 0.05;
const DAG_CAMERA_ZOOM_MAX = 32;

export type DagContentBounds = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

/** 🖼️ World bounds of every node a scene carries — what a fit frames. `null` for an empty graph,
 * which has nothing to frame. Twin of `DagHost::content_world_bounds`. */
export function dagContentBounds(nodes: readonly NodeGraphNodeRecord[] | undefined): DagContentBounds | null {
  let bounds: { minX: number; minY: number; maxX: number; maxY: number } | null = null;
  for (const node of nodes ?? []) {
    const halfW = (Number(node.width) || 0) * 0.5;
    const halfH = (Number(node.height) || 0) * 0.5;
    const x = Number(node.x) || 0;
    const y = Number(node.y) || 0;
    if (!Number.isFinite(x) || !Number.isFinite(y)) continue;
    bounds = bounds
      ? { minX: Math.min(bounds.minX, x - halfW), minY: Math.min(bounds.minY, y - halfH), maxX: Math.max(bounds.maxX, x + halfW), maxY: Math.max(bounds.maxY, y + halfH) }
      : { minX: x - halfW, minY: y - halfH, maxX: x + halfW, maxY: y + halfH };
  }
  return bounds;
}

/** 🔖️ Identity of the LAYOUT a camera was framed against: the node ids and their boxes, nothing
 * else. An example switch changes it; hovering, evaluating or selecting never does, which is what
 * keeps a re-fit from firing on an ordinary edit. */
export function nodeGraphContentSignature(nodes: readonly NodeGraphNodeRecord[] | undefined): string {
  return (nodes ?? []).map((node) => `${node.id}:${node.x},${node.y},${node.width},${node.height}`).join("|");
}

/** 🖼️ Fraction (`0..1`) of the graph's own area a camera currently shows. Degenerate content (a
 * single node, a row of nodes at one `y`) is measured on whichever axes have extent, so a
 * zero-height graph never reads as invisible. Twin of `canvas::camera::content_coverage`. */
export function dagContentCoverage(content: DagContentBounds, camera: DagCameraState, width: number, height: number): number {
  const zoom = Math.max(camera.zoom, 1e-9);
  const halfW = Math.max(width, 1) / (2 * zoom);
  const halfH = Math.max(height, 1) / (2 * zoom);
  const view = { minX: camera.x - halfW, minY: camera.y - halfH, maxX: camera.x + halfW, maxY: camera.y + halfH };
  const contentW = Math.max(content.maxX - content.minX, 0);
  const contentH = Math.max(content.maxY - content.minY, 0);
  const overlapX = Math.max(Math.min(content.maxX, view.maxX) - Math.max(content.minX, view.minX), 0);
  const overlapY = Math.max(Math.min(content.maxY, view.maxY) - Math.max(content.minY, view.minY), 0);
  const insideX = content.minX >= view.minX && content.maxX <= view.maxX;
  const insideY = content.minY >= view.minY && content.maxY <= view.maxY;
  if (contentW > 0 && contentH > 0) return (overlapX * overlapY) / (contentW * contentH);
  if (contentW > 0) return (insideY ? 1 : 0) * (overlapX / contentW);
  if (contentH > 0) return (insideX ? 1 : 0) * (overlapY / contentH);
  return insideX && insideY ? 1 : 0;
}

/** 🖼️ The camera that frames the whole graph with `paddingPx` of screen margin per side, zoom
 * clamped to the canvas camera range. Twin of `canvas::camera::fit_camera`. */
export function dagFitCamera(content: DagContentBounds, width: number, height: number, paddingPx = DAG_CONTENT_FIT_PADDING_PX): DagCameraState {
  const usableW = Math.max(Math.max(width, 1) - paddingPx * 2, 1);
  const usableH = Math.max(Math.max(height, 1) - paddingPx * 2, 1);
  const contentW = Math.max(content.maxX - content.minX, 0);
  const contentH = Math.max(content.maxY - content.minY, 0);
  const zoomX = contentW > 0 ? usableW / contentW : Number.POSITIVE_INFINITY;
  const zoomY = contentH > 0 ? usableH / contentH : Number.POSITIVE_INFINITY;
  const zoom = Math.min(zoomX, zoomY);
  return {
    x: (content.minX + content.maxX) * 0.5,
    y: (content.minY + content.maxY) * 0.5,
    zoom: Math.min(Math.max(Number.isFinite(zoom) ? zoom : 1, DAG_CAMERA_ZOOM_MIN), DAG_CAMERA_ZOOM_MAX),
  };
}

/** 🖼️ The camera a node-graph surface OPENS on — the law this window used to lack. A stored camera
 * is honoured only when it already frames the graph it was stored for; otherwise (and whenever
 * there is no stored camera) the graph is fitted, and `fitted` tells the caller to persist that the
 * same way it persists a pan or a zoom gesture. Twin of `canvas::camera::startup_camera`. */
export function dagStartupCamera(
  stored: DagCameraState | null | undefined,
  content: DagContentBounds | null,
  width: number,
  height: number,
  minCoverage = DAG_CONTENT_FRAMED_MIN_COVERAGE,
): { readonly camera: DagCameraState; readonly fitted: boolean } {
  if (!content) return { camera: stored ?? { x: 0, y: 0, zoom: 1 }, fitted: false };
  if (stored && stored.zoom > 0 && dagContentCoverage(content, stored, width, height) >= minCoverage) return { camera: stored, fitted: false };
  return { camera: dagFitCamera(content, width, height), fitted: true };
}
//#endregion 📷️ContentFraming

//#region DagOverlayPaint
const DAG_LABEL_SCREEN_PX = 11;
const DAG_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

export function parseDagNodeIdArray(json: string): string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

export function parseDagPreselectJson(json: string): DagPreselectSnapshot {
  try {
    const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
    const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
    const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
    return { ids, removedIds };
  } catch {
    return { ids: [], removedIds: [] };
  }
}

export function dagElementInteractionChrome(
  selectionIds: Iterable<string>,
  preselection: DagPreselectSnapshot,
  extraHighlightIds: Iterable<string> = [],
): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  const base = !preselection.ids.length && !preselection.removedIds.length
    ? { selectedIds: new Set(selectionIds), highlightedIds: new Set<string>() }
    : { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
  for (const id of extraHighlightIds) base.highlightedIds.add(id);
  return base;
}

export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as {
      readonly labels?: readonly Record<string, unknown>[];
      readonly rows?: readonly Record<string, unknown>[];
    };
    const raw = parsed.labels ?? parsed.rows ?? [];
    return raw
      .map((row): DagLabelOverlayRow | null => {
        const text = typeof row.text === "string" ? row.text.trim() : "";
        if (!text) return null;
        const align = row.align === "left" || row.align === "right" || row.align === "center" ? row.align : undefined;
        return {
          id: String(row.id ?? ""),
          kind: typeof row.kind === "string" ? row.kind : undefined,
          text,
          layout: row.layout === "vertical" ? "vertical" : "horizontal",
          align,
          x: Number(row.x ?? 0),
          y: Number(row.y ?? 0),
          nodeW: Number(row.nodeW ?? row.width ?? 0),
          nodeH: Number(row.nodeH ?? row.height ?? 0),
          fontScreenPx: typeof row.fontScreenPx === "number" ? row.fontScreenPx : undefined,
          maxScreenH: typeof row.maxScreenH === "number" ? row.maxScreenH : undefined,
          ghost: row.ghost === true,
        } satisfies DagLabelOverlayRow;
      })
      .filter((row): row is DagLabelOverlayRow => row !== null);
  } catch {
    return [];
  }
}

/** ✂️ The one glyph a clipped caption ends on — the same character every presentation of this repo
 * appends (`canvas::text::LABEL_ELLIPSIS`). */
export const DAG_LABEL_ELLIPSIS = "…";
/** 🔠️ Smallest font an overlay caption may shrink to. Below this a caption is a smudge, not a word,
 * which is why width is answered by {@link dagEllipsizeByMeasure} and never by shrinking further. */
const DAG_LABEL_LEGIBLE_MIN_PX = 8;

/** ✂️ Longest prefix of `text` that still fits `maxWidth` once {@link DAG_LABEL_ELLIPSIS} is
 * appended, measured by the CALLER's own measure. The JavaScript twin of
 * `canvas::text::ellipsize_by_measure`; both are pinned to the rows of
 * `🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json`.
 *
 * Empty text and a non-positive budget draw nothing; a budget too narrow for even one glyph plus
 * the ellipsis draws the bare ellipsis, so a clipped caption is always visibly clipped. */
export function dagEllipsizeByMeasure(text: string, maxWidth: number, measure: (candidate: string) => number): string {
  const trimmed = text.trim();
  if (!trimmed || maxWidth <= 0) return "";
  if (measure(trimmed) <= maxWidth) return trimmed;
  const glyphs = [...trimmed];
  let best = "";
  for (let index = 1; index <= glyphs.length; index += 1) {
    const candidate = glyphs.slice(0, index).join("") + DAG_LABEL_ELLIPSIS;
    if (measure(candidate) > maxWidth) break;
    best = candidate;
  }
  return best || DAG_LABEL_ELLIPSIS;
}

/** ✂️ {@link dagEllipsizeByMeasure} driven by the overlay canvas's own `measureText` at `fontPx`. */
function dagEllipsizeOverlayLabel(ctx: CanvasRenderingContext2D, text: string, fontPx: number, maxW: number): string {
  ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
  return dagEllipsizeByMeasure(text, maxW, (candidate) => ctx.measureText(candidate).width);
}

/** 📐️ Overlay caption font size: the row's own target, shrunk ONLY to fit the row's height, never
 * below {@link DAG_LABEL_LEGIBLE_MIN_PX}. Width is not a font decision — see
 * {@link dagEllipsizeOverlayLabel}. This used to binary-search the font down to 4px on width too,
 * which is how a caption wider than its node body became an unreadable smear. */
function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxH: number): number {
  const px = Math.max(DAG_LABEL_LEGIBLE_MIN_PX, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (px * 1.2 <= maxH) {
    return px;
  }
  let low = DAG_LABEL_LEGIBLE_MIN_PX;
  let high = px;
  let best = DAG_LABEL_LEGIBLE_MIN_PX;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    if (mid * 1.2 <= maxH) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

export function parseDagSliderOverlays(stateJson: string): readonly DagSliderOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly sliders?: unknown } | null;
    if (!Array.isArray(parsed?.sliders)) return [];
    const ids = new Set<string>();
    return parsed.sliders.filter((value): value is DagSliderOverlayRow => {
      if (!value || typeof value !== "object") return false;
      const row = value as Partial<DagSliderOverlayRow>;
      if (typeof row.widgetId !== "string" || !row.widgetId.trim() || ids.has(row.widgetId) || typeof row.label !== "string" || !row.label.trim()) return false;
      if (![row.value, row.min, row.max, row.step, row.x, row.y, row.w, row.h].every((number) => typeof number === "number" && Number.isFinite(number))) return false;
      if (row.min! > row.max! || row.step! <= 0 || row.w! <= 0 || row.h! <= 0) return false;
      ids.add(row.widgetId);
      return true;
    });
  } catch {
    return [];
  }
}

/** 🔬️ One graph surface's geometry read-back, published as `window.__semioFlowGraphProbe[surfaceId]`.
 * `entity` is {@link dagIntroductionResolver}'s own resolver — `"node"`, `"handle"` (a port), `"edge"`
 * and `"slider"` all resolve to viewport pixels — so a scripted caller can aim a pointer gesture at a
 * node or a port on a canvas that paints itself and has no per-entity DOM. */
export type FlowGraphSurfaceProbe = {
  readonly entity: (domain: string, id: string) => IntroductionResolvedGeometry | null;
  /** 🪪️ The node ids this surface is actually painting — the input `entity("node", id)` needs, and the
   * only one a caller can get: `hostSnapshotJson` is a scene field the plugin may not carry at all. */
  readonly nodeIds: () => readonly string[];
  /** 🗺️ Where this surface is painting each node, in the graph's own world units — the read a caller
   * needs to answer "did a reorganize move anything". Same reason as {@link nodeIds}: it comes off the
   * scene the surface holds, not off a snapshot field the plugin may never send. */
  readonly nodeLayout: () => Readonly<Record<string, { readonly x: number; readonly y: number }>>;
  readonly viewport: () => { readonly x: number; readonly y: number; readonly zoom: number };
  readonly hostSnapshotJson: () => string | null;
  readonly rect: () => { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null;
};

/** 🎯️ The subset of `FlowWasmSession`/`FrameworkGraphSession` {@link dagIntroductionResolver} needs
 * — factored out because both session interfaces expose the same overlay/entity JSON shape and both host
 * components (`FlowGraphCanvasHost`, `WasmGraphSurface`) register the identical resolver logic. */
type DagIntroductionSession = {
  readonly labelOverlayPaintStateJson: () => string | FlowTask<unknown>;
  readonly sliderOverlayStateJson: () => string | FlowTask<unknown>;
  readonly entityScreenJson?: (domain: string, id: string) => string | FlowTask<unknown>;
};

/** 🎯️ Builds the `IntroductionSurfaceResolver` for a dag-engine-backed graph surface. Reads the
 * session and container via refs (not React state) every call — cheap, and lets registration skip
 * re-running whenever the surface re-renders. `entity`'s `"slider"` domain resolves entirely from the
 * already-fetched `sliderOverlayStateJson()` (no Rust round trip); every other domain (`"node"`,
 * `"handle"`, `"edge"`) goes through `entityScreenJson`, added to the dag engine specifically for this. */
export function dagIntroductionResolver(sessionRef: React.RefObject<DagIntroductionSession | null>, containerRef: React.RefObject<HTMLElement | null>): IntroductionSurfaceResolver {
  const cache = new Map<string, string>();
  const active = new Map<string, FlowTask<unknown>>();
  const read = (key: string, value: string | FlowTask<unknown>): string | undefined => {
    if (typeof value === "string") return value;
    active.get(key)?.cancel();
    active.set(key, value);
    const unsubscribe = value.subscribe(() => {});
    void value.result.then((result) => cache.set(key, flowJsonText(result))).catch(() => {}).finally(() => {
      unsubscribe();
      if (active.get(key) === value) active.delete(key);
    });
    return cache.get(key);
  };
  return {
    canvasPoint: (x, y) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return null;
      const rect = container.getBoundingClientRect();
      const cameraJson = read("labels", session.labelOverlayPaintStateJson());
      if (!cameraJson) return null;
      const camera = parseDagOverlayCamera(cameraJson);
      const screen = dagWorldToScreen(camera, rect.width, rect.height, x, y);
      return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
    },
    entity: (domain, entityId): IntroductionResolvedGeometry | null => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return null;
      const rect = container.getBoundingClientRect();
      if (domain === "slider") {
        const slidersJson = read("sliders", session.sliderOverlayStateJson());
        if (!slidersJson) return null;
        const sliders = parseDagSliderOverlays(slidersJson);
        const slider = entityId === "*" ? sliders[0] : sliders.find((row) => row.widgetId === entityId);
        if (!slider) return null;
        const cameraJson = read("labels", session.labelOverlayPaintStateJson());
        if (!cameraJson) return null;
        const camera = parseDagOverlayCamera(cameraJson);
        const anchor = dagWorldToScreen(camera, rect.width, rect.height, slider.x, slider.y);
        return {
          point: { x: rect.left + anchor.x, y: rect.top + anchor.y },
          rect: { x: rect.left + anchor.x - slider.w / 2, y: rect.top + anchor.y - slider.h / 2, width: slider.w, height: slider.h },
          domain: { min: slider.min, max: slider.max, axis: "x" },
          visible: true,
        };
      }
      if (!session.entityScreenJson) return null;
      try {
        const geometryJson = read(`entity:${domain}:${entityId}`, session.entityScreenJson(domain, entityId));
        if (!geometryJson) return null;
        const geometry = JSON.parse(geometryJson) as {
          readonly visible: boolean;
          readonly x?: number;
          readonly y?: number;
          readonly rect?: readonly [number, number, number, number];
          readonly polyline?: readonly (readonly [number, number])[];
        };
        if (!geometry.visible || geometry.x === undefined || geometry.y === undefined) return null;
        return {
          point: { x: rect.left + geometry.x, y: rect.top + geometry.y },
          rect: geometry.rect ? { x: rect.left + geometry.rect[0], y: rect.top + geometry.rect[1], width: geometry.rect[2], height: geometry.rect[3] } : undefined,
          polyline: geometry.polyline?.map(([px, py]) => ({ x: rect.left + px, y: rect.top + py })),
          visible: true,
        };
      } catch {
        return null;
      }
    },
  };
}

export function parseDagOverlayCamera(stateJson: string): DagCameraState {
  try {
    const parsed = JSON.parse(stateJson) as { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number };
    return parseDagCameraState(JSON.stringify(parsed.camera ?? {}));
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}
export function dagOverlayLabelFill(nodeId: string, ghost: boolean, hoveredId: string | null, chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> }, dimmedIds: readonly string[] = []): string {
  if (ghost) return "var(--color-secondary)";
  if (dimmedIds.includes(nodeId)) return "var(--color-border)";
  if (chrome.selectedIds.has(nodeId)) return "var(--color-foreground)";
  if (chrome.highlightedIds.has(nodeId)) return "var(--color-secondary)";
  if (hoveredId === nodeId) return "var(--color-foreground)";
  return "var(--color-muted-foreground)";
}

/** 🎨️ Resolves {@link dagOverlayLabelFill} to a Canvas2D-safe `#rrggbb` — CSS `var()` strings are not valid `fillStyle` values and silently paint as black. */
export function dagOverlayLabelFillHex(nodeId: string, ghost: boolean, hoveredId: string | null, chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> }, dimmedIds: readonly string[] = []): string {
  const expression = dagOverlayLabelFill(nodeId, ghost, hoveredId, chrome, dimmedIds);
  const appearanceFallback = currentStylingAppearanceName() === "dark" ? "light" : "dark";
  if (expression === "var(--color-secondary)") return resolveColorHex(expression, "secondary");
  if (expression === "var(--color-border)" || expression === "var(--color-muted-foreground)") return resolveColorHex(expression, "gray");
  return resolveColorHex(expression, appearanceFallback);
}

export function parseDagMinimapWidgetOccluder(stateJson: string): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null {
  try {
    const parsed = JSON.parse(stateJson) as { readonly minimapWidget?: { readonly x?: number; readonly y?: number; readonly width?: number; readonly height?: number } };
    const rect = parsed.minimapWidget;
    if (rect?.x == null || rect?.y == null || rect?.width == null || rect?.height == null) return null;
    return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  } catch {
    return null;
  }
}

export function parseDagMinimapWidgetCursor(stateJson: string): string | undefined {
  try {
    const parsed = JSON.parse(stateJson) as { readonly minimapWidget?: { readonly cursor?: string | null } };
    const cursor = parsed.minimapWidget?.cursor;
    return typeof cursor === "string" && cursor.length > 0 ? cursor : undefined;
  } catch {
    return undefined;
  }
}

/** 🖼️ Sizes one canvas' backing store to `logicalW`×`logicalH` at `dpr` and returns whether the store
 * actually changed. Deliberately synchronous and free of any animation frame: a hidden/background tab
 * never fires `requestAnimationFrame`, so a canvas whose size is only reached from a rAF tick stays at
 * the HTML default 300×150 forever (`📓️runtime-verification-2026-09-09.md` boot #3). */
export function resizeCanvasBackingStore(canvas: HTMLCanvasElement | null | undefined, logicalW: number, logicalH: number, dpr: number): boolean {
  if (!canvas) return false;
  const pixelW = Math.max(1, Math.round(logicalW * dpr));
  const pixelH = Math.max(1, Math.round(logicalH * dpr));
  const changed = canvas.width !== pixelW || canvas.height !== pixelH;
  if (changed) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  canvas.style.width = `${Math.max(1, Math.round(logicalW))}px`;
  canvas.style.height = `${Math.max(1, Math.round(logicalH))}px`;
  return changed;
}

/** 🏷️ Paints the node captions over the engine canvas. Captions are centred on the MEASURED overlay
 * size, never on the session's reported `width`/`height`: the overlay is painted when the session is handed
 * over — before the engine canvas is attached and sized — and on later scene or interaction changes only,
 * so a static graph kept the 1×1 size the session reported at hand-over and drew every caption half a
 * canvas away from its node (dag, trinity-jack and mathematical play panes, 2026-09-23). */
export function paintDagLabelOverlays(stateJson: string, canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number, interaction: DagLabelOverlayInteraction): void {
  let state: { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number; readonly labels?: readonly DagLabelOverlayRow[] };
  try {
    state = JSON.parse(stateJson) as typeof state;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  resizeCanvasBackingStore(canvas, logicalW, logicalH, dpr);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, logicalW, logicalH);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = logicalW;
  const viewportH = logicalH;
  const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect, interaction.highlightIds ?? []);
  const dimmedIds = interaction.dimmedIds ?? [];
  const rows = state.labels ?? parseDagLabelRows(stateJson);
  const occluder = parseDagMinimapWidgetOccluder(stateJson);
  const inset = 0.88;
  for (const row of rows) {
    const anchor = dagWorldToScreen(camera, viewportW, viewportH, row.x, row.y);
    if (occluder && anchor.x >= occluder.x && anchor.x <= occluder.x + occluder.width && anchor.y >= occluder.y && anchor.y <= occluder.y + occluder.height) {
      continue;
    }
    const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
    const publishedW = Number(row.maxScreenW);
    const maxW = Number.isFinite(publishedW) && publishedW > 0 ? publishedW : Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(4, isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0 ? Number(row.maxScreenH) : Number(row.nodeH) * zoom * inset);
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
    const fontPx = dagClampLabelFontPx(ctx, row.text, targetPx, maxH);
    const textBudget = Number.isFinite(publishedW) && publishedW > 0 ? publishedW : row.layout === "vertical" ? Math.max(4, Number(row.nodeH) * zoom * inset) : maxW;
    const text = dagEllipsizeOverlayLabel(ctx, row.text, fontPx, textBudget);
    if (!text) continue;
    ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = dagOverlayLabelFillHex(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
    ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(text, anchor.x, anchor.y);
    }
    ctx.globalAlpha = 1;
  }
}

export function parseDagSelectionUnionBoundsScreen(json: string): DagSelectionBounds | null {
  try {
    const parsed = JSON.parse(json) as Partial<DagSelectionBounds>;
    if (parsed.x == null || parsed.y == null || parsed.width == null || parsed.height == null) return null;
    return { x: parsed.x, y: parsed.y, width: parsed.width, height: parsed.height };
  } catch {
    return null;
  }
}

/** 🧿️ Normalizes one selection-preview point from the rust `[[x,y],…]` wire format or `{x,y}` objects. */
function parseDagMarqueePoint(value: unknown): { readonly x: number; readonly y: number } | null {
  if (Array.isArray(value) && value.length >= 2 && typeof value[0] === "number" && typeof value[1] === "number" && Number.isFinite(value[0]) && Number.isFinite(value[1])) {
    return { x: value[0], y: value[1] };
  }
  if (value && typeof value === "object") {
    const x = (value as { readonly x?: unknown }).x;
    const y = (value as { readonly y?: unknown }).y;
    if (typeof x === "number" && typeof y === "number" && Number.isFinite(x) && Number.isFinite(y)) return { x, y };
  }
  return null;
}

/** 🧿️ Rectangle wire format is always four axis-aligned corners; anything else is a lasso path. */
function inferDagMarqueeMethod(points: readonly { readonly x: number; readonly y: number }[]): "lasso" | "rectangle" {
  if (points.length !== 4) return points.length >= 3 ? "lasso" : "rectangle";
  const xs = new Set(points.map((point) => point.x));
  const ys = new Set(points.map((point) => point.y));
  return xs.size === 2 && ys.size === 2 ? "rectangle" : "lasso";
}

/** 🧿️ Builds the shared `SelectionMarquee` overlay from board preview points (`[[x,y],…]` from rust). */
export function computeDagMarqueeOverlay(pointsJson: string, crossing: boolean, method?: string): DagMarqueeOverlay | null {
  let raw: unknown;
  try {
    raw = JSON.parse(pointsJson);
  } catch {
    return null;
  }
  if (!Array.isArray(raw)) return null;
  const points: { readonly x: number; readonly y: number }[] = [];
  for (const entry of raw) {
    const point = parseDagMarqueePoint(entry);
    if (!point) return null;
    points.push(point);
  }
  if (points.length < 2) return null;
  const coverage = crossing ? "partial" : "full";
  const resolvedMethod = method === "lasso" || method === "rectangle" ? method : inferDagMarqueeMethod(points);
  if (resolvedMethod === "lasso") return { kind: "lasso", points, coverage };
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const x = Math.min(...xs);
  const y = Math.min(...ys);
  return { kind: "rect", x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y, coverage };
}

export function sceneToSyncPack(scene: NodeGraphScene): Uint8Array {
  return new Uint8Array(encodePackValue(scene));
}

export function sceneToSyncJson(scene: NodeGraphScene): string {
  return JSON.stringify(scene);
}

//#region 🎚️SliderGestureLanes
/** 🎚️ One coalescing lane per slider widget of a graph surface, plus the press identity its dispatches carry.
 *
 * A dragged inline slider produces ~60 values a second. Sending one `nodeGraphEdit` per value costs
 * one retained command, one document edit, one history entry and one preview re-evaluation EACH —
 * measured on 6018 as 24 `toolRunStart`s and 29 history entries for ONE one-second drag, with the
 * mesh arriving three seconds behind the thumb. The lane keeps only the value the user is on now and
 * sends it when the previous round trip has landed (`📓️slider-preview-update-2026-09-15.md`).
 *
 * The press id (`continuousPressIdentity`) is minted by the press's first send, closed by its release, and travels
 * as the dispatch's own top-level `gesture` (the release adds `commit: true`, a cancel `abort: <reason>` with no
 * value), so the framework scrub machine (design §13.1) keeps every tick provisional and commits the whole drag as
 * ONE edit while the guest only maps the value to its absolute leaf. A lost pointer capture or a blur mid-press
 * cancels it (`captureLost`, `blur`) and unmounting a surface mid-press retires it, each with zero trace.
 *
 * 🩸️ Before this, the overlay dispatched `setGraphParameter` — an action NO app declares. The shell
 * dropped every tick (`dropped action "setGraphParameter" … no window kind declares it`), so the knob
 * moved, the local flow session moved, and the document and the preview never did: the user's
 * "moving a slider doesn't update the preview" in one line.
 */
export function useGraphSliderLanes(surfaceId: string, dispatchRef: React.RefObject<(action: string, args?: Record<string, unknown>) => void | Promise<void>>) {
  const gestureIdsRef = useRef(new Map<string, string>());
  const lanesRef = useRef(new Map<string, ContinuousGestureLane<number>>());
  const sliderLane = useCallback(
    (widgetId: string) => {
      const existing = lanesRef.current.get(widgetId);
      if (existing) return existing;
      const lane = createContinuousGestureLane<number>({
        send: (value, phase) => {
          const gesture = gestureIdsRef.current.get(widgetId) ?? continuousPressIdentity(`${surfaceId}:${widgetId}`);
          if (phase === "commit") gestureIdsRef.current.delete(widgetId);
          else gestureIdsRef.current.set(widgetId, gesture);
          return dispatchRef.current?.(nodeGraphActions.edit, { operations: [{ operation: "setSlider", widgetId, value }], gesture, commit: phase === "commit" });
        },
        abort: (reason) => {
          const gesture = gestureIdsRef.current.get(widgetId);
          gestureIdsRef.current.delete(widgetId);
          return gesture === undefined ? undefined : dispatchRef.current?.(nodeGraphActions.edit, { operations: [], gesture, abort: reason });
        },
        onFault: (error) => undefined,
      });
      lanesRef.current.set(widgetId, lane);
      return lane;
    },
    [dispatchRef, surfaceId],
  );
  useEffect(
    () => () => {
      for (const lane of lanesRef.current.values()) if (lane.open()) lane.abort("retired");
    },
    [],
  );
  return { sliderLane };
}
//#endregion 🎚️SliderGestureLanes

//#region DagDomOverlays
export function GraphSliderOverlays({
  scopeId,
  stateJson,
  logicalW,
  logicalH,
  editable,
  onSliderChange,
  onSliderCommit,
  onSliderPointerDown,
  onSliderPointerUp,
  onSliderAbort,
  occluderRect = null,
}: {
  readonly scopeId: string;
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onSliderChange: (widgetId: string, value: number) => void;
  readonly onSliderCommit?: (widgetId: string, value: number) => void;
  readonly onSliderPointerDown?: (widgetId: string) => void;
  readonly onSliderPointerUp?: (widgetId: string) => void;
  readonly onSliderAbort?: (widgetId: string, reason: ContinuousGestureAbortReason) => void;
  readonly occluderRect?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const sliders = parseDagSliderOverlays(stateJson);
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  if (sliders.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {sliders.map((slider) => {
        const screen = dagWorldToScreen(camera, logicalW, logicalH, slider.x, slider.y);
        if (occluderRect && screen.x >= occluderRect.x && screen.x <= occluderRect.x + occluderRect.width && screen.y >= occluderRect.y && screen.y <= occluderRect.y + occluderRect.height) {
          return null;
        }
        const w = slider.w;
        const h = Math.max(slider.h, 8 / zoom);
        return (
          <div
            key={slider.widgetId}
            className="pointer-events-auto absolute flex items-center"
            data-graph-slider-zoom={zoom}
            style={{ left: screen.x, top: screen.y, width: w, height: h, transform: `translate(-50%, -50%) scale(${zoom})`, transformOrigin: "center" }}
            onPointerDown={(event) => event.stopPropagation()}
          >
            <span
              aria-hidden="true"
              className="pointer-events-none absolute whitespace-nowrap tabular-nums"
              data-graph-slider-value={slider.widgetId}
              style={{ right: "100%", marginRight: screenMetric(slider.gapScreenPx, 4) / zoom, fontSize: screenMetric(slider.fontScreenPx, 10) / zoom, lineHeight: 1 }}
            >
              {dagSliderValueText(slider.value)}
            </span>
            <Slider
              id={`graph-slider-${encodeURIComponent(JSON.stringify([scopeId, slider.widgetId]))}`}
              aria-label={slider.label}
              className="h-full w-full min-w-0"
              thumbClassName="size-tiny"
              max={slider.max}
              min={slider.min}
              step={slider.step}
              value={[slider.value]}
              disabled={!editable}
              showValue={false}
              onValueChange={(values) => onSliderChange(slider.widgetId, values[0] ?? slider.value)}
              onPointerDown={() => onSliderPointerDown?.(slider.widgetId)}
              onPointerUp={() => onSliderPointerUp?.(slider.widgetId)}
              onPointerCancel={() => {
                onSliderAbort?.(slider.widgetId, "captureLost");
                onSliderPointerUp?.(slider.widgetId);
              }}
              onBlur={() => onSliderAbort?.(slider.widgetId, "blur")}
              onValueCommit={(values) => onSliderCommit?.(slider.widgetId, values[0] ?? slider.value)}
            />
          </div>
        );
      })}
    </div>
  );
}

const ALIGN_MODES = [
  { id: "left", label: "⬅️" },
  { id: "center-h", label: "↔" },
  { id: "right", label: "➡️" },
  { id: "top", label: "⬆️" },
  { id: "center-v", label: "↕️" },
  { id: "bottom", label: "⬇️" },
] as const;

export function alignModeToDag(mode: string): string {
  const map: Record<string, string> = {
    left: "alignLeft",
    right: "alignRight",
    top: "alignTop",
    bottom: "alignBottom",
    "center-h": "alignHorizontal",
    "center-v": "alignVertical",
  };
  return map[mode] ?? mode;
}

export function SelectionAlignChrome({ bounds, onAlign }: { readonly bounds: DagSelectionBounds; readonly onAlign: (mode: string) => void }) {
  return (
    <div className={cn("pointer-events-auto absolute z-50 flex gap-0.5 rounded border border-border p-0.5 shadow-sm", glassClass)} data-level="pane" style={{ left: bounds.x, top: Math.max(0, bounds.y - 28) }}>
      {ALIGN_MODES.map((mode) => (
        <button key={mode.id} type="button" className="size-5 rounded text-xs hover:bg-active-base" aria-label={mode.id} onPointerDown={(event) => event.stopPropagation()} onClick={() => onAlign(mode.id)}>
          {mode.label}
        </button>
      ))}
    </div>
  );
}
//#endregion DagDomOverlays
//#endregion DagOverlayPaint
//#endregion 🔖️graph-canvas-overlays
//#endregion 🔖️NodeGraphHost
