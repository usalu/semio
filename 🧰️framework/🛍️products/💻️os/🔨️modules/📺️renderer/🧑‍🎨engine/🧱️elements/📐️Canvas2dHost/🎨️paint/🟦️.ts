/** 🎨️ Platform canvas painting for typed scene geometry and authored appearance. */
import { pathSegmentsToSvgD, drawingTextLines, DRAWING_TEXT_LINE_HEIGHT, constantGradientColor, type PathSegment } from "../../../../../../../../🔨️modules/◻️2d/🟦️.ts";

type CanvasGradientStop = { readonly offset?: number; readonly color?: readonly number[] };

export type CanvasCompositingGroup = {readonly id:string;readonly opacity:number;readonly blendMode:string};

export type CanvasSceneNode = {
  readonly groups?:readonly CanvasCompositingGroup[];
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
  if (fill.kind === "linearGradient" || fill.kind === "radialGradient") {
    const degenerate = fill.kind === "linearGradient" ? (fill.x1 ?? 0) === (fill.x2 ?? 0) && (fill.y1 ?? 0) === (fill.y2 ?? 0) : (fill.r ?? 0) <= 0;
    const constant = constantGradientColor(fill.stops ?? [], degenerate);
    if (constant) return rgbaToCss(constant, opacity);
  }
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


/** 🧩️ Paints isolated group and leaf scopes with viewport-aligned reusable surfaces. */
export function paintCompositedLayers<T extends CanvasSceneNode>(root:CanvasRenderingContext2D,layers:readonly T[],paint:(ctx:CanvasRenderingContext2D,layer:T,index:number)=>void,allocate:(depth:number,width:number,height:number)=>CanvasRenderingContext2D):void {
  let previous:readonly CanvasCompositingGroup[]=[];
  const opened=new Set<string>();
  const commonDepth=(a:readonly CanvasCompositingGroup[],b:readonly CanvasCompositingGroup[])=>{
    let depth=0;
    while(depth<a.length && depth<b.length && a[depth]!.id===b[depth]!.id && a[depth]!.opacity===b[depth]!.opacity && a[depth]!.blendMode===b[depth]!.blendMode) depth++;
    return depth;
  };
  for(const layer of layers) {
    if(layer.visible===false) continue;
    const groups=layer.groups??[],common=commonDepth(previous,groups);
    for(const group of groups.slice(common)) {
      if(typeof group.id!=="string" || !group.id || opened.has(group.id) || !Number.isFinite(group.opacity) || group.opacity<0 || group.opacity>1 || !Object.hasOwn(BLEND_MODE_TO_COMPOSITE,group.blendMode)) throw new Error("Invalid canvas compositing hierarchy");
      opened.add(group.id);
    }
    if(!Number.isFinite(layer.opacity??1)) throw new Error("Canvas opacity must be finite");
    previous=groups;
  }
  const camera=root.getTransform(),width=root.canvas.width,height=root.canvas.height;
  const scopes:{group:CanvasCompositingGroup;ctx:CanvasRenderingContext2D}[]=[];
  const surface=(depth:number)=>{
    const ctx=allocate(depth,width,height);
    ctx.setTransform(1,0,0,1,0,0);ctx.globalAlpha=1;ctx.globalCompositeOperation="source-over";ctx.clearRect(0,0,width,height);
    ctx.setTransform(camera.a,camera.b,camera.c,camera.d,camera.e,camera.f);
    return ctx;
  };
  const composite=(target:CanvasRenderingContext2D,source:CanvasRenderingContext2D,opacity:number,blendMode:string)=>{
    target.save();target.setTransform(1,0,0,1,0,0);target.globalAlpha=Math.min(1,Math.max(0,opacity));target.globalCompositeOperation=blendModeToComposite(blendMode);target.drawImage(source.canvas,0,0);target.restore();
  };
  const close=()=>{
    const scope=scopes.pop()!;
    composite(scopes.at(-1)?.ctx??root,scope.ctx,scope.group.opacity,scope.group.blendMode);
  };
  previous=[];
  for(const [index,layer] of layers.entries()) {
    if(layer.visible===false) continue;
    const groups=layer.groups??[],common=commonDepth(previous,groups);
    while(scopes.length>common) close();
    for(const group of groups.slice(common)) scopes.push({group,ctx:surface(scopes.length)});
    const target=scopes.at(-1)?.ctx??root,opacity=layer.opacity??1,blendMode=layer.blendMode??"normal";
    if(opacity!==1 || blendMode!=="normal") {
      const ctx=surface(scopes.length);
      paint(ctx,{...layer,opacity:1,blendMode:"normal"},index);
      composite(target,ctx,opacity,blendMode);
    }else paint(target,layer,index);
    previous=groups;
  }
  while(scopes.length) close();
}
