/** 🖍️ Canvas2dHost layer paint: the JSON layer record, the layer colours and bounds, the rounded and bounds layer drawing, the board colour reader and the infinite grid. */
import { type CanvasSceneNode } from "../🎨️paint/🟦️.ts";
import { currentStylingAppearanceName, STYLING_BOARD_PALETTES, STYLING_METRICS, STYLING_STROKES } from "@semio-tech/ui-styling";
import type { CanvasCamera } from "../📷️camera-math/🟦️.ts";

//#region 🔖️Canvas2dHost
//#region JsonLayersCanvasSession
export type CanvasLayerRecord = CanvasSceneNode & {
  readonly id?: string;
  readonly kind?: string;
  readonly role?: string;
  readonly utility?: string;
  readonly name?: string;
  readonly color?: string;
  readonly selected?: boolean;
  readonly x0?: number;
  readonly y0?: number;
  readonly y1?: number;
  readonly x1?: number;
  readonly dataUrl?: string;
  readonly points?: readonly (readonly [number, number])[];
  readonly seams?: readonly number[];
  readonly base?: { readonly name?: string; readonly x?: number; readonly y?: number; readonly width?: number; readonly height?: number };
};

export function layerColorCss(layer: CanvasLayerRecord, fallbackHue: number, opacity = 1): string {
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

export function layerBounds(layer: CanvasLayerRecord): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null {
  const x = layer.x ?? layer.base?.x;
  const y = layer.y ?? layer.base?.y;
  const width = layer.width ?? layer.base?.width;
  const height = layer.height ?? layer.base?.height;
  if (x == null || y == null || width == null || height == null) return null;
  return { x, y, width, height };
}

export type CanvasLayerLabelInput = Pick<CanvasLayerRecord, "id" | "kind" | "name" | "base">;

/** 🏷️ Overlay text for bounds layers — only explicit `name` fields; geometry `kind` is not a label. */
export function canvasLayerDisplayLabel(layer: CanvasLayerLabelInput): string {
  return layer.name ?? layer.base?.name ?? "";
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

export function drawBoundsLayer(ctx: CanvasRenderingContext2D, layer: CanvasLayerRecord, bounds: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }, label: string, hue: number, zoom: number): void {
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
export function drawInfiniteCanvasGrid(ctx: CanvasRenderingContext2D, camera: CanvasCamera, logicalWidth: number, logicalHeight: number, zoom: number, stroke: string): void {
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
//#endregion JsonLayersCanvasSession
//#endregion 🔖️Canvas2dHost
