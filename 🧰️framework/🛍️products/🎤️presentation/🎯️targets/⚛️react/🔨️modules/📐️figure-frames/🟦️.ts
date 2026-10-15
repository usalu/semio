/** 📐️ Figure crop and mosaic geometry, asset url resolution and the morph frame and figure crop css variable helpers.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { DispositionPosition, FigureEmbodiment, FigureMosaicGrid } from "@semio-tech/presentation";
import { type CSSProperties } from "@semio-tech/ui-react";

//#region 🔖️MorphView
/** 🔗️ Resolves deck-relative figure paths against the Vite base URL. */
export function resolvePresentationAssetUrl(src: string): string {
  if (/^(?:[a-z]+:)?\/\//i.test(src) || src.startsWith("data:") || src.startsWith("blob:")) {
    return src;
  }
  const trimmed = src.replace(/^\.\//, "");
  const base = import.meta.env.BASE_URL ?? "/";
  return `${base.endsWith("/") ? base : `${base}/`}${trimmed}`.replace(/\/{2,}/g, "/");
}

/** 📐️ Physical aspect of a normalized crop (width÷height in source pixels). */
export function figureCropPhysicalAspect(crop: DispositionPosition, sourceAspect = 1): number {
  return (crop.width / crop.height) * sourceAspect;
}

export const FIGURE_MOSAIC_ALIGN_EPSILON = 1e-4;

/** 🧩️ Grid column/row for a split crop, or null when the crop is not a mosaic cell. */
export function figureMosaicCellIndex(crop: DispositionPosition, mosaic: { readonly rows: number; readonly columns: number; readonly frame?: DispositionPosition }): { readonly column: number; readonly row: number } | null {
  const { rows, columns } = mosaic;
  if (rows < 1 || columns < 1) {
    return null;
  }
  const mosaicFrame = mosaic.frame ?? { x: 0, y: 0, width: 1, height: 1 };
  const colWidth = mosaicFrame.width / columns;
  const rowHeight = mosaicFrame.height / rows;
  if (Math.abs(crop.width - colWidth) > FIGURE_MOSAIC_ALIGN_EPSILON || Math.abs(crop.height - rowHeight) > FIGURE_MOSAIC_ALIGN_EPSILON) {
    return null;
  }
  const column = Math.round((crop.x - mosaicFrame.x) / colWidth);
  const row = Math.round((crop.y - mosaicFrame.y) / rowHeight);
  if (column < 0 || column >= columns || row < 0 || row >= rows) {
    return null;
  }
  if (Math.abs(crop.x - (mosaicFrame.x + column * colWidth)) > FIGURE_MOSAIC_ALIGN_EPSILON || Math.abs(crop.y - (mosaicFrame.y + row * rowHeight)) > FIGURE_MOSAIC_ALIGN_EPSILON) {
    return null;
  }
  return { column, row };
}

/** 🧩️ Edge-aligned background-position for one cell in a rows×columns sprite grid. */
export function figureMosaicBackgroundPosition(column: number, row: number, columns: number, rows: number): { readonly posX: number; readonly posY: number } {
  return {
    posX: columns <= 1 ? 50 : (column / (columns - 1)) * 100,
    posY: rows <= 1 ? 50 : (row / (rows - 1)) * 100,
  };
}

/** 📐️ Background-position along one axis when cover overflows (k≥1; k=1 → edge-aligned i/(n−1)). */
export function overflowAxisPosition(index: number, count: number, coverOverflowK: number): number {
  if (count <= 1) {
    return 50;
  }
  if (Math.abs(coverOverflowK - 1) < FIGURE_MOSAIC_ALIGN_EPSILON) {
    return (index / (count - 1)) * 100;
  }
  const cellSpan = 1 / count;
  const numerator = (1 - coverOverflowK) / 2 - index / count;
  const denominator = cellSpan - coverOverflowK;
  return (numerator / denominator) * 100;
}

/** 🪟️ One mosaic cell as a window onto a single cover render of `frame` (no per-crop sprite zoom). */
export function mosaicWindowedCoverVars(
  cell: { readonly column: number; readonly row: number },
  grid: { readonly rows: number; readonly columns: number },
  frame: DispositionPosition,
  sourceAspect: number,
  slideAspect?: number,
): { readonly size: string; readonly posX: number; readonly posY: number } {
  const { rows, columns } = grid;
  const { column, row } = cell;
  const slideAR = slideAspect ?? sourceAspect;
  const frameAspect = (frame.width / frame.height) * slideAR;
  if (frameAspect >= sourceAspect) {
    const k = frameAspect / sourceAspect;
    return {
      size: `${columns * 100}% auto`,
      posX: columns <= 1 ? 50 : (column / (columns - 1)) * 100,
      posY: overflowAxisPosition(row, rows, k),
    };
  }
  const k = sourceAspect / frameAspect;
  return {
    size: `auto ${rows * 100}%`,
    posY: rows <= 1 ? 50 : (row / (rows - 1)) * 100,
    posX: overflowAxisPosition(column, columns, k),
  };
}

/** 🖼️ True when the crop is the full source bitmap. */
function figureCropIsFullImage(crop: DispositionPosition): boolean {
  return crop.width >= 1 - FIGURE_MOSAIC_ALIGN_EPSILON && crop.height >= 1 - FIGURE_MOSAIC_ALIGN_EPSILON;
}

/** 🖼️ Positions a normalized source crop when background width is `(100/crop.width)%` (uniform, no distortion). */
export function figureCropBackgroundPosition(crop: DispositionPosition): {
  readonly posX: number;
  readonly posY: number;
} {
  if (figureCropIsFullImage(crop)) {
    return { posX: 50, posY: 50 };
  }
  const spanX = 1 - crop.width;
  const spanY = 1 - crop.height;
  return {
    posX: spanX > FIGURE_MOSAIC_ALIGN_EPSILON ? (crop.x / spanX) * 100 : 50,
    posY: spanY > FIGURE_MOSAIC_ALIGN_EPSILON ? (crop.y / spanY) * 100 : 50,
  };
}

/** 🖼️ Uniform background-size: `cover` for full image, else `N% auto` / `auto N%` zoomed to the crop (never dual-axis `%`). */
export function figureCropBackgroundSize(crop: DispositionPosition, frame: DispositionPosition, sourceAspect = 1): string {
  if (figureCropIsFullImage(crop)) {
    return "cover";
  }
  if (frame.width <= 0 || frame.height <= 0 || crop.width <= 0 || crop.height <= 0) {
    return "cover";
  }
  const zoomW = 100 / crop.width;
  const zoomH = 100 / crop.height;
  const cropSpanHAtZoomW = (crop.height * zoomW * frame.width) / sourceAspect;
  const cropAspect = figureCropPhysicalAspect(crop, sourceAspect);
  const frameAspect = frame.width / frame.height;
  const coverScale = Math.max(frameAspect / cropAspect, cropAspect / frameAspect);
  if (cropSpanHAtZoomW >= frame.height) {
    return `${zoomW * coverScale}% auto`;
  }
  return `auto ${zoomH * coverScale}%`;
}

/** 🖼️ Centered crop cover in a slide frame (non-mosaic / focus morph). */
function figureCropCoverVars(crop: DispositionPosition, frame: DispositionPosition, sourceAspect = 1): { readonly size: string; readonly posX: number; readonly posY: number } {
  const { posX, posY } = figureCropBackgroundPosition(crop);
  return {
    size: figureCropBackgroundSize(crop, frame, sourceAspect),
    posX,
    posY,
  };
}

/** 🪟️ Mosaic cell background vars (windowed cover of `mosaic.frame`). */
function figureMosaicCellCoverVars(crop: DispositionPosition, mosaic: FigureMosaicGrid, sourceAspect: number, slideAspect?: number): { readonly size: string; readonly posX: number; readonly posY: number } | null {
  const cell = figureMosaicCellIndex(crop, mosaic);
  if (!cell) {
    return null;
  }
  const mosaicFrame = mosaic.frame ?? { x: 0, y: 0, width: 1, height: 1 };
  return mosaicWindowedCoverVars(cell, mosaic, mosaicFrame, sourceAspect, slideAspect);
}

/** 📐️ Reads `left`/`top`/`width`/`height` percent inline styles as a normalized disposition frame. */
export function readPercentDispositionFrame(element: HTMLElement): DispositionPosition | null {
  const read = (property: "left" | "top" | "width" | "height"): number | null => {
    const raw = element.style.getPropertyValue(property);
    if (!raw.endsWith("%")) {
      return null;
    }
    const value = Number.parseFloat(raw);
    return Number.isFinite(value) ? value / 100 : null;
  };
  const x = read("left");
  const y = read("top");
  const width = read("width");
  const height = read("height");
  if (x === null || y === null || width === null || height === null) {
    return null;
  }
  return { x, y, width, height };
}

/** 📐️ Custom properties for morphing a canvas frame from `from` into `to` during reveal auto-animate. */
export function morphFrameCssVars(from: DispositionPosition, to: DispositionPosition): CSSProperties {
  return {
    ["--presentation-morph-frame-left" as string]: `${from.x * 100}%`,
    ["--presentation-morph-frame-top" as string]: `${from.y * 100}%`,
    ["--presentation-morph-frame-width" as string]: `${from.width * 100}%`,
    ["--presentation-morph-frame-height" as string]: `${from.height * 100}%`,
    ["--presentation-frame-left" as string]: `${to.x * 100}%`,
    ["--presentation-frame-top" as string]: `${to.y * 100}%`,
    ["--presentation-frame-width" as string]: `${to.width * 100}%`,
    ["--presentation-frame-height" as string]: `${to.height * 100}%`,
  };
}

/** 📐️ Writes {@link morphFrameCssVars} onto an element via `style.setProperty` (required for CSS variables). */
export function applyMorphFrameCssVars(element: HTMLElement, from: DispositionPosition, to: DispositionPosition): void {
  for (const [key, value] of Object.entries(morphFrameCssVars(from, to))) {
    if (typeof value === "string") {
      element.style.setProperty(key, value);
    }
  }
}

/** 🖼️ Applies figure crop CSS variables from a vars object via `setProperty`. */
export function applyFigureCropCssVars(element: HTMLElement, vars: CSSProperties): void {
  for (const [key, value] of Object.entries(vars)) {
    if (typeof value === "string" && key.startsWith("--presentation-figure-bg-")) {
      element.style.setProperty(key, value);
    }
  }
}

/** 🖼️ CSS vars for crop tiles: mosaic windowed cover at rest/grid; centered crop cover for focus morph. */
export function figureCropBackgroundVars(embodiment: FigureEmbodiment, crop: DispositionPosition, frame?: DispositionPosition, morphToFrame?: DispositionPosition, fromMorphToFrame?: DispositionPosition, slideAspect?: number): CSSProperties {
  const mosaic = embodiment.mosaic;
  const sourceAspect = embodiment.sourceAspect ?? 1;
  const mosaicRest = mosaic ? figureMosaicCellCoverVars(crop, mosaic, sourceAspect, slideAspect) : null;
  const restBasis = frame ?? morphToFrame;
  const rest = mosaicRest ? mosaicRest : restBasis ? figureCropCoverVars(crop, restBasis, sourceAspect) : figureCropCoverVars(crop, { x: 0, y: 0, width: 1, height: 1 }, sourceAspect);
  const morphBasis = morphToFrame ?? frame;
  const morph = morphBasis ? figureCropCoverVars(crop, morphBasis, sourceAspect) : rest;
  const gridFrom = fromMorphToFrame
    ? mosaic
      ? (figureMosaicCellCoverVars(crop, mosaic, sourceAspect, slideAspect) ?? figureCropCoverVars(crop, fromMorphToFrame, sourceAspect))
      : figureCropCoverVars(crop, fromMorphToFrame, sourceAspect)
    : undefined;
  const vars: CSSProperties & Record<string, string | number | undefined> = {
    backgroundImage: `url("${resolvePresentationAssetUrl(embodiment.src)}")`,
    ["--presentation-figure-bg-size" as string]: rest.size,
    ["--presentation-figure-bg-position" as string]: `${rest.posX}% ${rest.posY}%`,
    ["--presentation-figure-bg-size-morph" as string]: morph.size,
    ["--presentation-figure-bg-position-morph" as string]: `${morph.posX}% ${morph.posY}%`,
  };
  if (gridFrom) {
    vars["--presentation-figure-bg-grid-size" as string] = gridFrom.size;
    vars["--presentation-figure-bg-grid-position" as string] = `${gridFrom.posX}% ${gridFrom.posY}%`;
  }
  return vars;
}

/** 👻️ Target ghost crop vars: `--presentation-figure-bg-size` = source tile frame, `-morph` = label slot (many-to-one 8→9). */
export function figureCropBackgroundVarsTargetGhost(embodiment: FigureEmbodiment, crop: DispositionPosition, sourceFrame: DispositionPosition, labelFrame: DispositionPosition): CSSProperties {
  return figureCropBackgroundVars(embodiment, crop, sourceFrame, labelFrame);
}
//#endregion 🔖️MorphView
