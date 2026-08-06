// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Canvas2dHost/component.tsx
/** @emoji 🖌️ `Canvas2dHost` — canvas-2d `ComponentSceneHost`: interprets a `draw.document`-shaped
 * `layersJson` scene into an HTML canvas, owns pan/zoom camera math shared with `Paint2dHost`, and
 * dispatches pointer/drag/context-menu actions back to the plugin. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useMemo, useRef, useState, type DragEvent, type MouseEvent } from "react";
import { type GraphWasmSession, GraphWasmCanvas, type CanvasInputModifiers } from "@semio-tech/infinite-canvas-react-renderer";
import { ContextMenuController, CATALOGUE_DRAG_MIME, registerIntroductionSurfaceResolver, windowElementId, useLabel, type ContextMenuItem, type IntroductionResolvedGeometry } from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework-core";
import { currentStylingAppearanceName, STYLING_BOARD_PALETTES, STYLING_METRICS, STYLING_STROKES } from "@semio-tech/ui-styling";
import { WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
// 🐢️ Direct element-to-element import — `Interpreter` and `Canvas2dHost` landed in the same batch.
import { useShellContextMenuFallback, openSurfaceContextMenu } from "../Interpreter/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Canvas2dHost
//#region CanvasCameraMath
export const CANVAS_CAMERA_ZOOM_MIN = 0.05;
export const CANVAS_CAMERA_ZOOM_MAX = 32;
const WHEEL_ZOOM_IN_FACTOR = 1.1;
const WHEEL_ZOOM_OUT_FACTOR = 0.9;

export type CanvasCamera = {
  x: number;
  y: number;
  zoom: number;
};

export function clampCanvasZoom(zoom: number): number {
  return Math.min(CANVAS_CAMERA_ZOOM_MAX, Math.max(CANVAS_CAMERA_ZOOM_MIN, zoom));
}

/** 🧭️ Maps world coordinates to logical (CSS-pixel) screen space — matches `infinite_canvas::camera::world_to_screen`. */
export function worldToScreenLogical(worldX: number, worldY: number, camera: CanvasCamera, viewportWidth: number, viewportHeight: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom || 1;
  return {
    x: (worldX - camera.x) * zoom + viewportWidth * 0.5,
    y: (worldY - camera.y) * zoom + viewportHeight * 0.5,
  };
}

/** 🧭️ Maps logical (CSS-pixel) screen space to world coordinates — matches `infinite_canvas::camera::screen_to_world`. */
export function screenToWorldLogical(screenX: number, screenY: number, camera: CanvasCamera, viewportWidth: number, viewportHeight: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom || 1;
  return {
    x: (screenX - viewportWidth * 0.5) / zoom + camera.x,
    y: (screenY - viewportHeight * 0.5) / zoom + camera.y,
  };
}

/** 🔍️ Cursor-anchored wheel zoom — matches `infinite_canvas::camera::wheel_screen`. */
export function wheelCameraAtScreen(camera: CanvasCamera, screenX: number, screenY: number, deltaY: number, viewportWidth: number, viewportHeight: number): CanvasCamera {
  const zoomFactor = deltaY < 0 ? WHEEL_ZOOM_IN_FACTOR : WHEEL_ZOOM_OUT_FACTOR;
  const nextZoom = clampCanvasZoom((camera.zoom || 1) * zoomFactor);
  const worldBefore = screenToWorldLogical(screenX, screenY, camera, viewportWidth, viewportHeight);
  return {
    x: worldBefore.x - (screenX - viewportWidth * 0.5) / nextZoom,
    y: worldBefore.y - (screenY - viewportHeight * 0.5) / nextZoom,
    zoom: nextZoom,
  };
}
//#endregion CanvasCameraMath

//#region JsonLayersCanvasSession
type CanvasGradientStop = { readonly offset?: number; readonly color?: readonly number[] };

type CanvasLayerRecord = {
  readonly id?: string;
  readonly kind?: string;
  readonly role?: string;
  readonly utility?: string;
  readonly name?: string;
  readonly color?: string;
  readonly selected?: boolean;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly x0?: number;
  readonly y0?: number;
  readonly y1?: number;
  readonly x1?: number;
  readonly dataUrl?: string;
  readonly points?: readonly (readonly [number, number])[];
  readonly seams?: readonly number[];
  readonly base?: { readonly name?: string; readonly x?: number; readonly y?: number; readonly width?: number; readonly height?: number };
  readonly transform?: readonly number[];
  readonly segments?: readonly {
    readonly kind?: string;
    readonly to?: readonly [number, number];
    readonly ctrl?: readonly [number, number];
    readonly ctrl1?: readonly [number, number];
    readonly ctrl2?: readonly [number, number];
    readonly rx?: number;
    readonly ry?: number;
    readonly rotation?: number;
    readonly largeArc?: boolean;
    readonly sweep?: boolean;
  }[];
  readonly fill?: {
    readonly kind?: string;
    readonly color?: readonly number[];
    readonly x1?: number;
    readonly y1?: number;
    readonly x2?: number;
    readonly y2?: number;
    readonly cx?: number;
    readonly cy?: number;
    readonly r?: number;
    readonly stops?: readonly CanvasGradientStop[];
  };
  readonly stroke?: { readonly color?: readonly number[]; readonly width?: number; readonly dash?: readonly number[]; readonly cap?: string; readonly join?: string };
  readonly opacity?: number;
  readonly blendMode?: string;
  readonly fillRule?: string;
  readonly visible?: boolean;
  readonly text?: { readonly content?: string; readonly size?: number };
  readonly image?: { readonly src?: string; readonly width?: number; readonly height?: number };
};

function rgbaToCss(color: readonly number[] | undefined, opacity = 1): string {
  if (!color || color.length < 3) return `rgba(148, 163, 184, ${opacity})`;
  const alpha = (color[3] ?? 1) * opacity;
  return `rgba(${color[0]! * 255}, ${color[1]! * 255}, ${color[2]! * 255}, ${alpha})`;
}

/** 🎨️ Maps a `draw.document` blend mode to its `GlobalCompositeOperation` equivalent (16 modes, matches `DRAW_BLEND_MODES`). */
const BLEND_MODE_TO_COMPOSITE: Readonly<Record<string, GlobalCompositeOperation>> = {
  normal: "source-over",
  multiply: "multiply",
  screen: "screen",
  overlay: "overlay",
  darken: "darken",
  lighten: "lighten",
  colorDodge: "color-dodge",
  colorBurn: "color-burn",
  hardLight: "hard-light",
  softLight: "soft-light",
  difference: "difference",
  exclusion: "exclusion",
  hue: "hue",
  saturation: "saturation",
  color: "color",
  luminosity: "luminosity",
};

function blendModeToComposite(mode: string | undefined): GlobalCompositeOperation {
  return BLEND_MODE_TO_COMPOSITE[mode ?? "normal"] ?? "source-over";
}

/** 🪣️ Resolves a fill record into a canvas paint — solid color or gradient (linear/radial, in local layer coordinates). */
function fillStyleToPaint(ctx: CanvasRenderingContext2D, fill: CanvasLayerRecord["fill"], opacity: number): string | CanvasGradient | null {
  if (!fill) return null;
  if (fill.kind === "linearGradient" && fill.stops?.length) {
    const gradient = ctx.createLinearGradient(fill.x1 ?? 0, fill.y1 ?? 0, fill.x2 ?? 0, fill.y2 ?? 0);
    for (const stop of fill.stops) gradient.addColorStop(Math.min(1, Math.max(0, stop.offset ?? 0)), rgbaToCss(stop.color, opacity));
    return gradient;
  }
  if (fill.kind === "radialGradient" && fill.stops?.length) {
    const gradient = ctx.createRadialGradient(fill.cx ?? 0, fill.cy ?? 0, 0, fill.cx ?? 0, fill.cy ?? 0, Math.max(fill.r ?? 0, 0));
    for (const stop of fill.stops) gradient.addColorStop(Math.min(1, Math.max(0, stop.offset ?? 0)), rgbaToCss(stop.color, opacity));
    return gradient;
  }
  if (fill.color) return rgbaToCss(fill.color, opacity);
  return null;
}

/** 🖊️ Builds a `Path2D` from the full (possibly multi-contour) segment list — evenodd fill handles holes correctly across contours. */
function buildScenePath(segments: CanvasLayerRecord["segments"]): Path2D | null {
  if (!segments?.length) return null;
  const path = new Path2D();
  for (const segment of segments) {
    const kind = segment.kind ?? "line";
    if (kind === "move" && segment.to) {
      path.moveTo(segment.to[0]!, segment.to[1]!);
    } else if (kind === "line" && segment.to) {
      path.lineTo(segment.to[0]!, segment.to[1]!);
    } else if (kind === "quad" && segment.ctrl && segment.to) {
      path.quadraticCurveTo(segment.ctrl[0]!, segment.ctrl[1]!, segment.to[0]!, segment.to[1]!);
    } else if (kind === "cubic" && segment.ctrl1 && segment.ctrl2 && segment.to) {
      path.bezierCurveTo(segment.ctrl1[0]!, segment.ctrl1[1]!, segment.ctrl2[0]!, segment.ctrl2[1]!, segment.to[0]!, segment.to[1]!);
    } else if (kind === "arc" && segment.to) {
      path.lineTo(segment.to[0]!, segment.to[1]!);
    } else if (kind === "close") {
      path.closePath();
    }
  }
  return path;
}

function layerColorCss(layer: CanvasLayerRecord, fallbackHue: number, opacity = 1): string {
  if (layer.color) {
    if (layer.color.startsWith("#") || layer.color.startsWith("hsl")) {
      if (layer.color.startsWith("hsl") && opacity < 1) {
        return layer.color.replace(")", ` / ${opacity})`).replace("hsl(", "hsla(");
      }
      return layer.color;
    }
  }
  return `hsla(${fallbackHue}, 70%, 55%, ${opacity})`;
}

function applySceneTransform(ctx: CanvasRenderingContext2D, transform: readonly number[] | undefined): void {
  if (!transform || transform.length < 6) return;
  const [a, b, c, d, e, f] = transform;
  ctx.transform(a ?? 1, b ?? 0, c ?? 0, d ?? 1, e ?? 0, f ?? 0);
}

function drawSceneNode(ctx: CanvasRenderingContext2D, layer: CanvasLayerRecord, zoom: number, imageCache: ReadonlyMap<string, HTMLImageElement>): void {
  if (layer.visible === false) return;
  const opacity = layer.opacity ?? 1;
  ctx.save();
  ctx.globalCompositeOperation = blendModeToComposite(layer.blendMode);
  applySceneTransform(ctx, layer.transform);
  const path = buildScenePath(layer.segments);
  if (path) {
    const fillRule = layer.fillRule === "nonzero" ? "nonzero" : "evenodd";
    const fillPaint = fillStyleToPaint(ctx, layer.fill, opacity);
    if (fillPaint) {
      ctx.fillStyle = fillPaint;
      ctx.fill(path, fillRule);
    }
    if (layer.stroke) {
      ctx.strokeStyle = rgbaToCss(layer.stroke.color, opacity);
      ctx.lineWidth = Math.max((layer.stroke.width ?? 1) / zoom, 1 / zoom);
      ctx.lineCap = (layer.stroke.cap as CanvasLineCap) ?? "butt";
      ctx.lineJoin = (layer.stroke.join as CanvasLineJoin) ?? "miter";
      ctx.setLineDash(layer.stroke.dash?.map((value) => value / zoom) ?? []);
      ctx.stroke(path);
      ctx.setLineDash([]);
    } else if (!fillPaint) {
      ctx.strokeStyle = rgbaToCss([0.58, 0.64, 0.72, 0.95], opacity);
      ctx.lineWidth = Math.max(1 / zoom, 1);
      ctx.stroke(path);
    }
  }
  if (layer.text?.content) {
    ctx.fillStyle = layer.fill?.color ? rgbaToCss(layer.fill.color, opacity) : rgbaToCss([0.89, 0.91, 0.94, 1], opacity);
    ctx.font = `${layer.text.size ?? 14}px ui-monospace, monospace`;
    ctx.fillText(layer.text.content, 0, layer.text.size ?? 14);
  }
  if (layer.image?.src) {
    const width = layer.image.width ?? layer.width ?? 64;
    const height = layer.image.height ?? layer.height ?? 64;
    const image = imageCache.get(layer.image.src);
    if (image?.complete) {
      ctx.globalAlpha = opacity;
      ctx.drawImage(image, 0, 0, width, height);
      ctx.globalAlpha = 1;
    }
  }
  ctx.restore();
}

function layerBounds(layer: CanvasLayerRecord): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null {
  const x = layer.x ?? layer.base?.x;
  const y = layer.y ?? layer.base?.y;
  const width = layer.width ?? layer.base?.width;
  const height = layer.height ?? layer.base?.height;
  if (x == null || y == null || width == null || height == null) return null;
  return { x, y, width, height };
}

function layerLabel(layer: CanvasLayerRecord): string {
  return layer.name ?? layer.base?.name ?? layer.kind ?? layer.id ?? "layer";
}

function drawRoundedRect(ctx: CanvasRenderingContext2D, x: number, y: number, width: number, height: number, radius: number): void {
  const r = Math.min(radius, width * 0.5, height * 0.5);
  ctx.beginPath();
  ctx.moveTo(x + r, y);
  ctx.lineTo(x + width - r, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + r);
  ctx.lineTo(x + width, y + height - r);
  ctx.quadraticCurveTo(x + width, y + height, x + width - r, y + height);
  ctx.lineTo(x + r, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - r);
  ctx.lineTo(x, y + r);
  ctx.quadraticCurveTo(x, y, x + r, y);
  ctx.closePath();
}

function drawBoundsLayer(ctx: CanvasRenderingContext2D, layer: CanvasLayerRecord, bounds: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }, label: string, hue: number, zoom: number): void {
  const isHandle = layer.role === "handle";
  const isSelected = layer.selected === true;
  const fillOpacity = isHandle ? 0.35 : isSelected ? 0.42 : 0.22;
  const strokeOpacity = isSelected ? 1 : isHandle ? 0.7 : 0.85;
  const fillColor = layerColorCss(layer, hue, fillOpacity);
  const strokeColor = isSelected ? "rgba(251, 191, 36, 0.95)" : layerColorCss(layer, hue, strokeOpacity);
  const lineWidth = Math.max((isSelected ? 2.5 : 1) / zoom, 1 / zoom);
  if (isSelected) {
    ctx.strokeStyle = "rgba(251, 191, 36, 0.28)";
    ctx.lineWidth = Math.max(5 / zoom, 2 / zoom);
    if (layer.kind === "circle") {
      const cx = bounds.x + bounds.width * 0.5;
      const cy = bounds.y + bounds.height * 0.5;
      const radius = Math.min(bounds.width, bounds.height) * 0.5 + 4 / zoom;
      ctx.beginPath();
      ctx.arc(cx, cy, radius, 0, Math.PI * 2);
      ctx.stroke();
    } else {
      drawRoundedRect(ctx, bounds.x - 4 / zoom, bounds.y - 4 / zoom, bounds.width + 8 / zoom, bounds.height + 8 / zoom, 6 / zoom);
      ctx.stroke();
    }
  }
  ctx.fillStyle = fillColor;
  ctx.strokeStyle = strokeColor;
  ctx.lineWidth = lineWidth;
  if (layer.kind === "circle") {
    const cx = bounds.x + bounds.width * 0.5;
    const cy = bounds.y + bounds.height * 0.5;
    const radius = Math.min(bounds.width, bounds.height) * 0.5;
    ctx.beginPath();
    ctx.arc(cx, cy, radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  } else {
    drawRoundedRect(ctx, bounds.x, bounds.y, bounds.width, bounds.height, 4 / zoom);
    ctx.fill();
    ctx.stroke();
  }
  if (!isHandle && label) {
    ctx.fillStyle = "rgba(226, 232, 240, 0.92)";
    ctx.font = `${12 / zoom}px ui-monospace, monospace`;
    ctx.fillText(label, bounds.x + 4, bounds.y + 14 / zoom);
  }
}

/** 🎨️ Board-palette rgba8888 → CSS, matching flow/infinite grid strokes. */
function boardChannelsToCss(channels: readonly number[]): string {
  const [r = 0, g = 0, b = 0, a = 255] = channels;
  return `rgba(${r}, ${g}, ${b}, ${a / 255})`;
}

/** 🌓️ Theme-aware canvas-2d surface paints — same board `rasterClear` / grid tokens as flow and infinite boards. */
export function readCanvas2dSurfaceColors(): { readonly clear: string; readonly grid: string } {
  const appearance = currentStylingAppearanceName();
  const board = STYLING_BOARD_PALETTES[appearance];
  return {
    clear: boardChannelsToCss(board.rasterClear),
    grid: boardChannelsToCss(board.gridMinorStroke),
  };
}

/** 📐️ World-space LOD grid (large/medium/small/micro), matching infinite board stroke steps. */
function drawInfiniteCanvasGrid(ctx: CanvasRenderingContext2D, camera: CanvasCamera, logicalWidth: number, logicalHeight: number, zoom: number, stroke: string): void {
  const halfW = logicalWidth / (2 * Math.max(zoom, 0.0001));
  const halfH = logicalHeight / (2 * Math.max(zoom, 0.0001));
  const minX = camera.x - halfW;
  const maxX = camera.x + halfW;
  const minY = camera.y - halfH;
  const maxY = camera.y + halfH;
  const steps: readonly { readonly world: number; readonly width: number; readonly minScreen: number }[] = [
    { world: STYLING_METRICS.board.gridWorldLarge, width: STYLING_STROKES.gridLarge, minScreen: 0 },
    { world: STYLING_METRICS.board.gridWorldMedium, width: STYLING_STROKES.gridMedium, minScreen: 8 },
    { world: STYLING_METRICS.board.gridWorldSmall, width: STYLING_STROKES.gridSmall, minScreen: 10 },
    { world: STYLING_METRICS.board.gridWorldMicro, width: STYLING_STROKES.gridMicro, minScreen: 12 },
  ];
  ctx.save();
  ctx.strokeStyle = stroke;
  for (const step of steps) {
    const screen = step.world * zoom;
    if (screen < step.minScreen) continue;
    ctx.lineWidth = Math.max(step.width / zoom, 0.5 / zoom);
    const startX = Math.floor(minX / step.world) * step.world;
    const startY = Math.floor(minY / step.world) * step.world;
    ctx.beginPath();
    for (let x = startX; x <= maxX; x += step.world) {
      ctx.moveTo(x, minY);
      ctx.lineTo(x, maxY);
    }
    for (let y = startY; y <= maxY; y += step.world) {
      ctx.moveTo(minX, y);
      ctx.lineTo(maxX, y);
    }
    ctx.stroke();
  }
  ctx.restore();
}

class JsonLayersCanvasSession implements GraphWasmSession {
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private logicalWidth = 1;
  private logicalHeight = 1;
  private dpr = 1;
  private readonly imageCache = new Map<string, HTMLImageElement>();
  private panning = false;
  private panStart = { x: 0, y: 0 };
  private panCameraStart = { x: 0, y: 0 };
  private activeUtility = "selectDirect";

  constructor(
    private readonly layersJson: string,
    private camera: CanvasCamera,
    private readonly onCameraChange: (camera: CanvasCamera) => void,
    private readonly onPointer?: (action: string, args?: Record<string, unknown>) => void,
  ) {}

  async attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown> {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.logicalWidth = logicalW;
    this.logicalHeight = logicalH;
    this.dpr = dpr;
    await this.preloadImages();
    this.renderFrame();
    return undefined;
  }

  setSize(width: number, height: number, dpr: number): void {
    this.logicalWidth = width;
    this.logicalHeight = height;
    this.dpr = dpr;
  }

  updateCamera(camera: CanvasCamera): void {
    this.camera = camera;
  }

  private parseLayers(): CanvasLayerRecord[] {
    try {
      return JSON.parse(this.layersJson) as CanvasLayerRecord[];
    } catch {
      return [];
    }
  }

  private async preloadImages(): Promise<void> {
    const layers = this.parseLayers();
    const urls = new Set<string>();
    for (const layer of layers) {
      if (layer.kind === "image" && layer.dataUrl) urls.add(layer.dataUrl);
      if (layer.image?.src) urls.add(layer.image.src);
    }
    await Promise.all(
      [...urls].map(async (key) => {
        if (this.imageCache.has(key)) return;
        const image = new Image();
        image.decoding = "async";
        image.src = key;
        await image.decode().catch(() => undefined);
        this.imageCache.set(key, image);
      }),
    );
  }

  renderFrame(): void {
    const ctx = this.ctx;
    const canvas = this.canvas;
    if (!ctx || !canvas) return;
    const deviceWidth = canvas.width;
    const deviceHeight = canvas.height;
    const logicalWidth = this.logicalWidth;
    const logicalHeight = this.logicalHeight;
    const dpr = this.dpr;
    const zoom = this.camera.zoom || 1;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, deviceWidth, deviceHeight);
    const surface = readCanvas2dSurfaceColors();
    ctx.fillStyle = surface.clear;
    ctx.fillRect(0, 0, deviceWidth, deviceHeight);
    const records = this.parseLayers();
    const meta = records.find((record) => record.role === "meta");
    if (meta?.utility) this.activeUtility = meta.utility;
    const layers = records.filter((record) => record.role !== "meta");
    ctx.save();
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.translate(logicalWidth * 0.5 - this.camera.x * zoom, logicalHeight * 0.5 - this.camera.y * zoom);
    ctx.scale(zoom, zoom);
    drawInfiniteCanvasGrid(ctx, this.camera, logicalWidth, logicalHeight, zoom, surface.grid);
    for (const [index, layer] of layers.entries()) {
      if (layer.segments?.length || layer.text || layer.image?.src) {
        drawSceneNode(ctx, layer, zoom, this.imageCache);
        continue;
      }
      if (layer.kind === "image" && layer.dataUrl) {
        const bounds = layerBounds(layer);
        const image = this.imageCache.get(layer.dataUrl);
        if (bounds && image && image.complete) {
          ctx.drawImage(image, bounds.x, bounds.y, bounds.width, bounds.height);
        }
        continue;
      }
      if (layer.kind === "polyline" && layer.points?.length) {
        const seams = layer.seams ?? [];
        for (let segment = 0; segment + 1 < layer.points.length; segment += 2) {
          const [x0, y0] = layer.points[segment]!;
          const [x1, y1] = layer.points[segment + 1]!;
          const seamIndex = segment / 2;
          ctx.strokeStyle = layerColorCss(layer, (index * 47) % 360, 0.95);
          ctx.lineWidth = Math.max(1 / zoom, 1);
          ctx.setLineDash(seams[seamIndex] ? [6 / zoom, 4 / zoom] : []);
          ctx.beginPath();
          ctx.moveTo(x0, y0);
          ctx.lineTo(x1, y1);
          ctx.stroke();
        }
        ctx.setLineDash([]);
        continue;
      }
      const bounds = layerBounds(layer);
      const label = layerLabel(layer);
      const hue = (index * 47) % 360;
      if (layer.kind === "line" || layer.x0 != null) {
        const x0 = layer.x0 ?? layer.x ?? 0;
        const y0 = layer.y0 ?? layer.y ?? 0;
        const x1 = layer.x1 ?? (layer.x ?? 0) + (layer.width ?? 0);
        const y1 = layer.y1 ?? (layer.y ?? 0) + (layer.height ?? 0);
        const isWire = layer.role === "wire";
        ctx.strokeStyle = layerColorCss(layer, hue, 0.9);
        ctx.lineWidth = Math.max((isWire ? 1.25 : 2) / zoom, 1 / zoom);
        ctx.setLineDash(isWire ? [6 / zoom, 4 / zoom] : []);
        ctx.beginPath();
        ctx.moveTo(x0, y0);
        ctx.lineTo(x1, y1);
        ctx.stroke();
        ctx.setLineDash([]);
        continue;
      }
      if (bounds) {
        drawBoundsLayer(ctx, layer, bounds, label, hue, zoom);
      } else {
        ctx.fillStyle = "rgba(226, 232, 240, 0.75)";
        ctx.font = `${12 / zoom}px ui-monospace, monospace`;
        ctx.fillText(label, -logicalWidth / 2 + 16, -logicalHeight / 2 + 20 + index * 18);
      }
    }
    if (layers.length === 0) {
      ctx.fillStyle = "rgba(148, 163, 184, 0.7)";
      ctx.font = `${12 / zoom}px ui-monospace, monospace`;
      ctx.fillText("Empty canvas", -36, 0);
    }
    ctx.restore();
  }

  pointerDown(x: number, y: number, button: number, _extend: boolean, modifiers?: CanvasInputModifiers): void {
    if (button === 1 || this.activeUtility === "transformMove") {
      this.panning = true;
      this.panStart = { x, y };
      this.panCameraStart = { x: this.camera.x, y: this.camera.y };
      return;
    }
    this.onPointer?.("canvasPointerDown", {
      x,
      y,
      button,
      shift: modifiers?.shift ?? false,
      ctrl: modifiers?.ctrl ?? false,
      meta: modifiers?.meta ?? false,
      alt: modifiers?.alt ?? false,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  pointerMove(x: number, y: number): void {
    if (this.panning) {
      const zoom = this.camera.zoom || 1;
      const next = {
        ...this.camera,
        x: this.panCameraStart.x - (x - this.panStart.x) / zoom,
        y: this.panCameraStart.y - (y - this.panStart.y) / zoom,
      };
      this.camera = next;
      this.onCameraChange(next);
      this.renderFrame();
      return;
    }
    this.onPointer?.("canvasPointerMove", {
      x,
      y,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  pointerUp(x: number, y: number, modifiers?: CanvasInputModifiers): void {
    if (this.panning) {
      this.panning = false;
      return;
    }
    this.onPointer?.("canvasPointerUp", {
      x,
      y,
      shift: modifiers?.shift ?? false,
      ctrl: modifiers?.ctrl ?? false,
      meta: modifiers?.meta ?? false,
      alt: modifiers?.alt ?? false,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  doubleClick(x: number, y: number): void {
    this.onPointer?.("canvasDoubleClick", { x, y, width: this.logicalWidth, height: this.logicalHeight });
  }

  wheel(x: number, y: number, deltaY: number): void {
    const next = wheelCameraAtScreen(this.camera, x, y, deltaY, this.logicalWidth, this.logicalHeight);
    this.camera = next;
    this.onCameraChange(next);
    this.renderFrame();
  }
}
//#endregion JsonLayersCanvasSession

//#region Canvas2dHost
// 🐢️ `CAMERA_SYNC_DEBOUNCE_MS` is also referenced (unqualified) by `World3dHost`'s still-barrel-resident
// camera-dispatch debounce — exported so that outside reference keeps resolving after this extraction.
export const CAMERA_SYNC_DEBOUNCE_MS = 120;
const DRAG_OVER_THROTTLE_MS = 50;
const DRAG_OVER_THROTTLE_DISTANCE = 4;

export function Canvas2dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.canvas2d;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const contextMenuTitleLabel = useLabel("ui.surfaceContextMenu.canvas");
  const initialCamera = useMemo(() => ({ x: scene?.cameraX ?? 0, y: scene?.cameraY ?? 0, zoom: scene?.zoom ?? 1 }), [scene?.cameraX, scene?.cameraY, scene?.zoom]);
  const cameraRef = useRef<CanvasCamera>(initialCamera);
  cameraRef.current = initialCamera;
  const sessionRef = useRef<JsonLayersCanvasSession | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const cameraSyncTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dragOverStateRef = useRef<{ x: number; y: number; time: number } | null>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const sessionFactory = useMemo(() => {
    return () => {
      const session = new JsonLayersCanvasSession(
        scene?.layersJson ?? "[]",
        cameraRef.current,
        (next) => {
          cameraRef.current = next;
          sessionRef.current?.updateCamera(next);
          if (cameraSyncTimeoutRef.current) clearTimeout(cameraSyncTimeoutRef.current);
          cameraSyncTimeoutRef.current = setTimeout(() => dispatch("setCamera", { camera: next }), CAMERA_SYNC_DEBOUNCE_MS);
        },
        (action, args) => {
          if (action === "canvasPointerDown" && args?.button === 0) {
            dispatch("paintStrokeBegin");
          }
          if (action === "canvasPointerUp") {
            dispatch("paintStrokeEnd");
          }
          dispatch(action, args);
        },
      );
      sessionRef.current = session;
      return session;
    };
  }, [dispatch, scene?.layersJson]);

  const layersJsonRef = useRef(scene?.layersJson);
  layersJsonRef.current = scene?.layersJson;
  const layersCacheRef = useRef<{ readonly json: string; readonly layers: readonly CanvasLayerRecord[] } | null>(null);

  /** 🐢️ `layersJson` can be sizeable and the resolver is called every animation frame while a
   * demonstration targets this surface — cache the parse, keyed by the source string's identity, so a
   * re-render with the same layers never re-parses. */
  const readLayers = useCallback((): readonly CanvasLayerRecord[] => {
    const json = layersJsonRef.current ?? "[]";
    const cached = layersCacheRef.current;
    if (cached && cached.json === json) return cached.layers;
    let layers: readonly CanvasLayerRecord[] = [];
    try {
      const parsed = JSON.parse(json) as unknown;
      layers = Array.isArray(parsed) ? (parsed as CanvasLayerRecord[]) : [];
    } catch {
      layers = [];
    }
    layersCacheRef.current = { json, layers };
    return layers;
  }, []);

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), {
      canvasPoint: (x, y) => {
        const container = containerRef.current;
        if (!container) return null;
        const rect = container.getBoundingClientRect();
        const screen = worldToScreenLogical(x, y, cameraRef.current, rect.width, rect.height);
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
      // 🏷️ `"layer"` is the only domain — a `CanvasLayerRecord.id` is optional and there is no id index,
      // so only layers that authored one are targetable; `"*"` picks the first that has an id.
      entity: (domain, entityId): IntroductionResolvedGeometry | null => {
        if (domain !== "layer") return null;
        const container = containerRef.current;
        if (!container) return null;
        const rect = container.getBoundingClientRect();
        const layer = entityId === "*" ? readLayers().find((candidate) => candidate.id) : readLayers().find((candidate) => candidate.id === entityId);
        if (!layer) return null;
        const toScreen = (wx: number, wy: number) => {
          const screen = worldToScreenLogical(wx, wy, cameraRef.current, rect.width, rect.height);
          return { x: rect.left + screen.x, y: rect.top + screen.y };
        };
        // 🐢️ Layer `transform` (a local affine matrix) is deliberately not applied here — composing it
        // correctly needs matrix math this resolver has no other use for; an explicitly transformed
        // layer's demonstration target will be off by that transform until this is extended.
        if (layer.segments && layer.segments.length > 0) {
          const polyline = sampleBezierSegments(layer.segments).map((point) => toScreen(point.x, point.y));
          if (polyline.length > 0) return { point: polyline[Math.floor(polyline.length / 2)], polyline, visible: true };
        }
        if (layer.points && layer.points.length > 0) {
          const polyline = layer.points.map(([px, py]) => toScreen(px, py));
          return { point: polyline[Math.floor(polyline.length / 2)], polyline, visible: true };
        }
        const bounds =
          layer.x !== undefined && layer.y !== undefined && layer.width !== undefined && layer.height !== undefined
            ? { x: layer.x, y: layer.y, width: layer.width, height: layer.height }
            : layer.x0 !== undefined && layer.y0 !== undefined && layer.x1 !== undefined && layer.y1 !== undefined
              ? { x: Math.min(layer.x0, layer.x1), y: Math.min(layer.y0, layer.y1), width: Math.abs(layer.x1 - layer.x0), height: Math.abs(layer.y1 - layer.y0) }
              : null;
        if (!bounds) return null;
        const topLeft = toScreen(bounds.x, bounds.y);
        const bottomRight = toScreen(bounds.x + bounds.width, bounds.y + bounds.height);
        return {
          point: { x: (topLeft.x + bottomRight.x) / 2, y: (topLeft.y + bottomRight.y) / 2 },
          rect: { x: topLeft.x, y: topLeft.y, width: bottomRight.x - topLeft.x, height: bottomRight.y - topLeft.y },
          visible: true,
        };
      },
    });
  }, [windowInstanceId, readLayers]);

  const handleDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;
      const last = dragOverStateRef.current;
      const now = Date.now();
      if (last && now - last.time < DRAG_OVER_THROTTLE_MS && Math.abs(x - last.x) < DRAG_OVER_THROTTLE_DISTANCE && Math.abs(y - last.y) < DRAG_OVER_THROTTLE_DISTANCE) return;
      dragOverStateRef.current = { x, y, time: now };
      dispatch("canvasDragOver", { x, y, width: rect.width, height: rect.height, types: [...event.dataTransfer.types] });
    },
    [dispatch],
  );

  const handleDragLeave = useCallback(() => {
    dragOverStateRef.current = null;
    dispatch("canvasDragLeave");
  }, [dispatch]);

  const handleDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
      if (!raw) return;
      event.preventDefault();
      dragOverStateRef.current = null;
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      dispatch("canvasDrop", { x: event.clientX - rect.left, y: event.clientY - rect.top, width: rect.width, height: rect.height, dragData: raw });
    },
    [dispatch],
  );

  //#region ContextMenu
  /** @emoji 🖱️ No layer pick/selection is tracked at this level (`Canvas2dScene` carries only camera + `layersJson`) — `hits`/`selection` stay empty per surface convention until layer picking lands here. */
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const items = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "canvas2d" },
            surface: { surfaceId: node.surfaceId, kind: "canvas2d", hits: [], selection: [] },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, items });
      })();
    },
    [mapContextMenu, node.surfaceId, requestContextMenu, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion ContextMenu

  if (!scene) return <div className="semio-canvas-2d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={containerRef}
      className="semio-canvas-2d-host h-full min-h-[24rem] w-full ui-surface"
      data-level="base"
      data-controller-id={node.controllerId}
      data-surface-id={node.surfaceId}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      onContextMenu={onContextMenu}
    >
      <GraphWasmCanvas className="h-full w-full" sessionFactory={sessionFactory} />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Canvas2dHost
//#endregion 🔖️Canvas2dHost
