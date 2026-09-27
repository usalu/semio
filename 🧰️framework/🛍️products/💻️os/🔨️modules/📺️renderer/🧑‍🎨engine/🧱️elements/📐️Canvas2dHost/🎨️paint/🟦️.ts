/** 🎨️ Platform canvas painting for typed scene geometry and authored appearance. */
import { pathSegmentsToSvgD, drawingTextLines, DRAWING_TEXT_LINE_HEIGHT, type PathSegment } from "../../../../../../../../🔨️modules/◻️2d/🟦️.ts";

type CanvasGradientStop = { readonly offset?: number; readonly color?: readonly number[] };

export type CanvasSceneNode = {
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly transform?: readonly number[];
  readonly segments?: readonly PathSegment[];
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
function fillStyleToPaint(ctx: CanvasRenderingContext2D, fill: CanvasSceneNode["fill"], opacity: number): string | CanvasGradient | null {
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
export function buildScenePath(segments: CanvasSceneNode["segments"]): Path2D | null {
  return segments?.length ? new Path2D(pathSegmentsToSvgD(segments)) : null;
}


function applySceneTransform(ctx: CanvasRenderingContext2D, transform: readonly number[] | undefined): void {
  if (!transform || transform.length < 6) return;
  const [a, b, c, d, e, f] = transform;
  ctx.transform(a ?? 1, b ?? 0, c ?? 0, d ?? 1, e ?? 0, f ?? 0);
}

/** 🖼️ `complete` alone is NOT "this image can be drawn": it turns true for a FAILED load too, and
 * `drawImage` on a broken element throws an uncaught `InvalidStateError` that surfaces as a page
 * error (measured on 🎞️animate, whose demo deck names a source the dev server does not serve — five
 * of its six remaining console faults). A decoded raster always reports a non-zero `naturalWidth`. */
export function isDecodedImage(image: HTMLImageElement | undefined): image is HTMLImageElement {
  return Boolean(image?.complete) && (image?.naturalWidth ?? 0) > 0;
}

export function drawSceneNode(ctx: CanvasRenderingContext2D, layer: CanvasSceneNode, imageCache: ReadonlyMap<string, HTMLImageElement>): void {
  if (layer.visible === false) return;
  const opacity = layer.opacity ?? 1;
  ctx.save();
  ctx.globalCompositeOperation = blendModeToComposite(layer.blendMode);
  applySceneTransform(ctx, layer.transform);
  const path = buildScenePath(layer.segments);
  const fillPaint = fillStyleToPaint(ctx, layer.fill, opacity);
  const strokeWidth = layer.stroke?.width ?? 1;
  const hasStroke = Boolean(layer.stroke && Number.isFinite(strokeWidth) && strokeWidth > 0);
  if (fillPaint) ctx.fillStyle = fillPaint;
  if (hasStroke) {
    ctx.strokeStyle = rgbaToCss(layer.stroke!.color, opacity);
    ctx.lineWidth = strokeWidth;
    ctx.lineCap = (layer.stroke!.cap as CanvasLineCap) ?? "butt";
    ctx.lineJoin = (layer.stroke!.join as CanvasLineJoin) ?? "miter";
    ctx.setLineDash(layer.stroke!.dash ? [...layer.stroke!.dash] : []);
  }
  if (path) {
    if (fillPaint) ctx.fill(path, layer.fillRule === "nonzero" ? "nonzero" : "evenodd");
    if (hasStroke) ctx.stroke(path);
  }
  if (layer.text?.content) {
    const size = layer.text.size ?? 14;
    ctx.font = `${size}px ui-sans-serif, system-ui, sans-serif`;
    ctx.textBaseline = "alphabetic";
    ctx.textAlign = "left";
    let index = 0;
    for (const line of drawingTextLines(layer.text.content)) {
      const baseline = (layer.y ?? 0) + size + index++ * size * DRAWING_TEXT_LINE_HEIGHT;
      if (!line) continue;
      if (fillPaint) ctx.fillText(line, layer.x ?? 0, baseline);
      if (hasStroke) ctx.strokeText(line, layer.x ?? 0, baseline);
    }
  }
  if (layer.image?.src) {
    const width = layer.image.width ?? layer.width ?? 64;
    const height = layer.image.height ?? layer.height ?? 64;
    const image = imageCache.get(layer.image.src);
    if (isDecodedImage(image)) {
      ctx.globalAlpha = opacity;
      ctx.drawImage(image, 0, 0, width, height);
      ctx.globalAlpha = 1;
    }
  }
  ctx.restore();
}

