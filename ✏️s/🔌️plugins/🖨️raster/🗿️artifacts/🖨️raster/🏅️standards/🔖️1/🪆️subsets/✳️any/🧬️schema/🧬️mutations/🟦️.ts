/** 🎭️ Canonical document types shared by mutation payloads. */
import type {RasterLayerNode, RasterLayerMask, RasterTransform, SemioImageSnapshot} from "../🟦️.ts";
export type {RasterLayerNode, RasterLayerMask, RasterTransform, SemioImageSnapshot} from "../🟦️.ts";
export interface RasterPixelContent { imageKey: string | null; width: number | null; height: number | null }

/** 🧬️ RasterMutation union — closed semantic mutation vocabulary for the raster document. */
export type RasterMutation =
  | {mutation:"changeLayerLocked";layerId:string;expected:boolean;locked:boolean}
  | {mutation:'changeLayerAdjustmentParameter'; layerId:string; parameter:'brightness'|'contrast'; expected:number|null; value:number|null}
  | { mutation: 'createLayer'; parentId?: string; index: number; layer: RasterLayerNode }
  | { mutation: 'deleteLayer'; layerId: string }
  | { mutation: 'reorderLayers'; layerId: string; parentId?: string; index: number }
  | { mutation: 'renameLayer'; layerId: string; newName: string }
  | { mutation: 'changeLayerVisible'; layerId: string; newVisible: boolean }
  | { mutation: 'changeLayerOpacity'; layerId: string; newOpacity: number }
  | { mutation: 'changeLayerBlendMode'; layerId: string; newBlendMode: string }
  | { mutation: 'moveLayer'; layerId: string; newX: number; newY: number }
  | { mutation: 'resizeLayer'; layerId: string; newWidth: number; newHeight: number }
  | { mutation: 'changeLayerAdjustmentKind'; layerId: string; newAdjustmentKind: string }
  | { mutation: 'addLayerAsset'; assetId: string; asset: SemioImageSnapshot }
  | { mutation: 'removeLayerAsset'; assetId: string }
  | { mutation: 'changeLayerPixels'; layerId: string; expectedImageKey: string | null; content: RasterPixelContent; transform?: RasterTransform | null }
  | { mutation: 'changeLayerMask'; layerId: string; expected: RasterLayerMask | null; mask: RasterLayerMask | null }
  | ({ mutation: 'paintStroke' } & PaintStroke)
  | ({ mutation: 'fillRegion' } & FillRegion)
  | ({ mutation: 'applyFilter' } & ApplyFilter)
  | ({ mutation: 'transformImage' } & TransformImage)
  | ({ mutation: 'fillSelection' } & FillSelection);

/** 📍️ One stroke point in the target image's pixels: pixel edges at whole numbers, centres at `+ 0.5`. */
export interface RasterStrokePoint { x: number; y: number }
/** 🖌️ A stroke's brush: diameter in pixels, hard-core fraction, opacity, straight-alpha sRGB colour (four channels 0..1). */
export interface RasterBrush { size: number; hardness: number; opacity: number; color: [number, number, number, number] }
/** ✂️ One selection run the stroke is clipped to: `length` pixels from row-major index `start`, covered `coverage`/255. */
export interface RasterSelectionSpan { start: number; length: number; coverage: number }
/** 🖌️ One brush or eraser stroke on a layer's pixels or on its mask, optionally clipped to a pixel selection. */
export interface PaintStroke { layerId: string; target: "pixels" | "mask"; tool: "brush" | "eraser"; brush: RasterBrush; points: RasterStrokePoint[]; selection: RasterSelectionSpan[] | null }
/** 🪣️ The whole pixel a region fill floods from, in the target image's pixels. */
export interface RasterSeed { x: number; y: number }
/** 🪣️ One bucket fill on a layer's pixels or on its mask: the 4-connected region around the seed within the channel
 * tolerance, filled with a straight-alpha sRGB colour (four channels 0..1), optionally clipped to a pixel selection. */
export interface FillRegion { layerId: string; target: "pixels" | "mask"; seed: RasterSeed; tolerance: number; color: [number, number, number, number]; selection: RasterSelectionSpan[] | null }
/** 🌈️ Every filter a layer image takes in place, with the closed range of its amount (`null`: it takes none, the amount is 0). */
export const RASTER_FILTERS = {
  invert: null, grayscale: null, clear: null, flipHorizontal: null, flipVertical: null,
  brightness: [-1, 1], contrast: [-1, 1], saturation: [-1, 1], gamma: [0.01, 10], threshold: [0, 255], posterize: [2, 256], blur: [0, 16], sharpen: [0, 5],
} as const satisfies Record<string, readonly [number, number] | null>;
/** 🌈️ The name of one in-place image filter. */
export type RasterFilter = keyof typeof RASTER_FILTERS;
/** 🌈️ One image filter on a layer's pixels, optionally clipped to a pixel selection (a flip takes none). */
export interface ApplyFilter { layerId: string; filter: RasterFilter; amount: number; selection: RasterSelectionSpan[] | null }
/** 🔄️ One change of a layer image's pixel grid: a quarter turn (no origin, extent or sampling), a resize to `width ×
 * height` (no origin) or a crop to the `width × height` window at `(x, y)` (no sampling). */
export interface TransformImage { layerId: string; operation: "rotateClockwise" | "rotateCounterclockwise" | "resize" | "crop"; x: number; y: number; width: number; height: number; bilinear: boolean }
/** 🫗️ One fill of a layer's pixels or of its mask over a pixel selection (the whole image without one), with a
 * straight-alpha sRGB colour (four channels 0..1). */
export interface FillSelection { layerId: string; target: "pixels" | "mask"; color: [number, number, number, number]; selection: RasterSelectionSpan[] | null }
/** 🧮️ The longest side a transformed image may have, as the Rust leaf and the payload schema bound it. */
export const RASTER_IMAGE_MAXIMUM_SIDE = 16384;
/** 🧮️ The most points one stroke carries, as the Rust leaf and the payload schema bound it. */
export const RASTER_STROKE_MAXIMUM_POINTS = 2048;

/** 🖌️ Decodes one `paint-stroke` payload (with or without its `mutation` tag), refusing what its schema refuses: an unknown
 * target or tool, a brush outside its bounds, a colour that is not four unit channels, no or too many points, a non-finite
 * coordinate, selection runs out of order, or any field the schema does not name. */
export function parsePaintStroke(value: unknown): PaintStroke {
  const record = (input: unknown, keys: readonly string[], what: string): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input)) throw new TypeError(`Invalid ${what}`);
    const row = input as Record<string, unknown>;
    if (Object.keys(row).some((key) => !keys.includes(key))) throw new TypeError(`Invalid ${what}`);
    return row;
  };
  const number = (input: unknown, what: string, min = -Infinity, max = Infinity): number => {
    if (typeof input !== "number" || !Number.isFinite(input) || input < min || input > max) throw new TypeError(`Invalid ${what}`);
    return input;
  };
  const row = record(value, ["mutation", "layerId", "target", "tool", "brush", "points", "selection"], "paint-stroke payload");
  if (row.mutation !== undefined && row.mutation !== "paintStroke") throw new TypeError("Invalid paint-stroke payload");
  if (typeof row.layerId !== "string" || row.layerId.length === 0) throw new TypeError("Invalid stroke layer");
  if (row.target !== "pixels" && row.target !== "mask") throw new TypeError("Invalid paint target");
  if (row.tool !== "brush" && row.tool !== "eraser") throw new TypeError("Invalid paint tool");
  const brush = record(row.brush, ["size", "hardness", "opacity", "color"], "brush");
  if (!Array.isArray(brush.color) || brush.color.length !== 4) throw new TypeError("Invalid brush colour");
  const color = brush.color.map((channel) => number(channel, "brush colour", 0, 1)) as [number, number, number, number];
  if (!Array.isArray(row.points) || row.points.length < 1 || row.points.length > RASTER_STROKE_MAXIMUM_POINTS) throw new TypeError("A stroke carries 1 to 2048 points");
  const points = row.points.map((point): RasterStrokePoint => {
    const entry = record(point, ["x", "y"], "stroke point");
    return { x: number(entry.x, "stroke point"), y: number(entry.y, "stroke point") };
  });
  if (row.selection !== null && !Array.isArray(row.selection)) throw new TypeError("Invalid stroke selection");
  let next = 0;
  const selection = row.selection === null ? null : row.selection.map((span): RasterSelectionSpan => {
    const entry = record(span, ["start", "length", "coverage"], "selection run");
    const run = { start: number(entry.start, "selection run", 0, 16777215), length: number(entry.length, "selection run", 1, 16777216), coverage: number(entry.coverage, "selection run", 0, 255) };
    if (![run.start, run.length, run.coverage].every(Number.isInteger) || run.start < next) throw new TypeError("Invalid selection run");
    next = run.start + run.length;
    return run;
  });
  return {
    layerId: row.layerId,
    target: row.target,
    tool: row.tool,
    brush: { size: number(brush.size, "brush size", 0.1, 4096), hardness: number(brush.hardness, "brush hardness", 0, 1), opacity: number(brush.opacity, "brush opacity", 0, 1), color },
    points,
    selection,
  };
}

/** 🔄️ Decodes one `transform-image` payload (with or without its `mutation` tag), refusing what its schema refuses: an
 * unknown operation, a quarter turn with an origin, extent or sampling, a resize with an origin, a crop that samples, an
 * extent outside 1..16384 (0 for a quarter turn), or any field the schema does not name. */
export function parseTransformImage(value: unknown): TransformImage {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new TypeError("Invalid transform-image payload");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !["mutation", "layerId", "operation", "x", "y", "width", "height", "bilinear"].includes(key))) throw new TypeError("Invalid transform-image payload");
  if (row.mutation !== undefined && row.mutation !== "transformImage") throw new TypeError("Invalid transform-image payload");
  if (typeof row.layerId !== "string" || row.layerId.length === 0) throw new TypeError("Invalid transform layer");
  const integer = (input: unknown, min: number, max: number): number => {
    if (typeof input !== "number" || !Number.isInteger(input) || input < min || input > max) throw new TypeError("Invalid transform extent");
    return input;
  };
  const x = integer(row.x, 0, RASTER_IMAGE_MAXIMUM_SIDE - 1), y = integer(row.y, 0, RASTER_IMAGE_MAXIMUM_SIDE - 1);
  const width = integer(row.width, 0, RASTER_IMAGE_MAXIMUM_SIDE), height = integer(row.height, 0, RASTER_IMAGE_MAXIMUM_SIDE);
  if (typeof row.bilinear !== "boolean") throw new TypeError("Invalid transform sampling");
  const bilinear = row.bilinear;
  const operation = row.operation;
  const turn = operation === "rotateClockwise" || operation === "rotateCounterclockwise";
  if (!turn && operation !== "resize" && operation !== "crop") throw new TypeError("Invalid transform operation");
  if (turn && (x !== 0 || y !== 0 || width !== 0 || height !== 0 || bilinear)) throw new TypeError("A quarter turn keeps the image's own extent");
  if (operation === "resize" && (x !== 0 || y !== 0)) throw new TypeError("Only a crop has an origin");
  if (!turn && (width === 0 || height === 0)) throw new TypeError("Invalid transform extent");
  if (operation === "crop" && bilinear) throw new TypeError("Only a resize samples");
  return { layerId: row.layerId, operation, x, y, width, height, bilinear };
}

/** 🫗️ Decodes one `fill-selection` payload (with or without its `mutation` tag), refusing what its schema refuses: an
 * unknown target, a colour that is not four unit channels, selection runs out of order, or any field the schema does not
 * name. */
export function parseFillSelection(value: unknown): FillSelection {
  const record = (input: unknown, keys: readonly string[], what: string): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input)) throw new TypeError(`Invalid ${what}`);
    const row = input as Record<string, unknown>;
    if (Object.keys(row).some((key) => !keys.includes(key))) throw new TypeError(`Invalid ${what}`);
    return row;
  };
  const integer = (input: unknown, what: string, min: number, max: number): number => {
    if (typeof input !== "number" || !Number.isInteger(input) || input < min || input > max) throw new TypeError(`Invalid ${what}`);
    return input;
  };
  const row = record(value, ["mutation", "layerId", "target", "color", "selection"], "fill-selection payload");
  if (row.mutation !== undefined && row.mutation !== "fillSelection") throw new TypeError("Invalid fill-selection payload");
  if (typeof row.layerId !== "string" || row.layerId.length === 0) throw new TypeError("Invalid fill layer");
  if (row.target !== "pixels" && row.target !== "mask") throw new TypeError("Invalid fill target");
  if (!Array.isArray(row.color) || row.color.length !== 4 || !row.color.every((channel) => typeof channel === "number" && Number.isFinite(channel) && channel >= 0 && channel <= 1)) throw new TypeError("Invalid fill colour");
  if (row.selection !== null && !Array.isArray(row.selection)) throw new TypeError("Invalid fill selection");
  let next = 0;
  const selection = row.selection === null ? null : row.selection.map((span): RasterSelectionSpan => {
    const entry = record(span, ["start", "length", "coverage"], "selection run");
    const run = { start: integer(entry.start, "selection run", 0, 16777215), length: integer(entry.length, "selection run", 1, 16777216), coverage: integer(entry.coverage, "selection run", 0, 255) };
    if (run.start < next) throw new TypeError("Invalid selection run");
    next = run.start + run.length;
    return run;
  });
  return { layerId: row.layerId, target: row.target, color: row.color as [number, number, number, number], selection };
}

/** 🌈️ Decodes one `apply-filter` payload (with or without its `mutation` tag), refusing what its schema refuses: an unknown
 * filter, an amount outside the filter's range (a whole number for posterize and blur, 0 for a filter without
 * one), a flip with a selection, selection runs out of order, or any field the schema does not name. */
export function parseApplyFilter(value: unknown): ApplyFilter {
  const record = (input: unknown, keys: readonly string[], what: string): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input)) throw new TypeError(`Invalid ${what}`);
    const row = input as Record<string, unknown>;
    if (Object.keys(row).some((key) => !keys.includes(key))) throw new TypeError(`Invalid ${what}`);
    return row;
  };
  const integer = (input: unknown, what: string, min: number, max: number): number => {
    if (typeof input !== "number" || !Number.isInteger(input) || input < min || input > max) throw new TypeError(`Invalid ${what}`);
    return input;
  };
  const row = record(value, ["mutation", "layerId", "filter", "amount", "selection"], "apply-filter payload");
  if (row.mutation !== undefined && row.mutation !== "applyFilter") throw new TypeError("Invalid apply-filter payload");
  if (typeof row.layerId !== "string" || row.layerId.length === 0) throw new TypeError("Invalid filter layer");
  if (typeof row.filter !== "string" || !Object.hasOwn(RASTER_FILTERS, row.filter)) throw new TypeError("Invalid filter");
  const filter = row.filter as RasterFilter;
  const range: readonly [number, number] | null = RASTER_FILTERS[filter];
  const amount = row.amount;
  if (typeof amount !== "number" || !Number.isFinite(amount) || (range === null ? amount !== 0 : amount < range[0] || amount > range[1] || ((filter === "posterize" || filter === "blur") && !Number.isInteger(amount)))) throw new TypeError("Invalid filter amount");
  if (row.selection !== null && !Array.isArray(row.selection)) throw new TypeError("Invalid filter selection");
  if (row.selection !== null && (filter === "flipHorizontal" || filter === "flipVertical")) throw new TypeError("Invalid filter selection");
  let next = 0;
  const selection = row.selection === null ? null : row.selection.map((span): RasterSelectionSpan => {
    const entry = record(span, ["start", "length", "coverage"], "selection run");
    const run = { start: integer(entry.start, "selection run", 0, 16777215), length: integer(entry.length, "selection run", 1, 16777216), coverage: integer(entry.coverage, "selection run", 0, 255) };
    if (run.start < next) throw new TypeError("Invalid selection run");
    next = run.start + run.length;
    return run;
  });
  return { layerId: row.layerId, filter, amount, selection };
}

/** 🪣️ Decodes one `fill-region` payload (with or without its `mutation` tag), refusing what its schema refuses: an unknown
 * target, a seed off the 16384² pixel grid, a tolerance outside 0..255, a colour that is not four unit channels, selection
 * runs out of order, or any field the schema does not name. */
export function parseFillRegion(value: unknown): FillRegion {
  const record = (input: unknown, keys: readonly string[], what: string): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input)) throw new TypeError(`Invalid ${what}`);
    const row = input as Record<string, unknown>;
    if (Object.keys(row).some((key) => !keys.includes(key))) throw new TypeError(`Invalid ${what}`);
    return row;
  };
  const integer = (input: unknown, what: string, min: number, max: number): number => {
    if (typeof input !== "number" || !Number.isInteger(input) || input < min || input > max) throw new TypeError(`Invalid ${what}`);
    return input;
  };
  const row = record(value, ["mutation", "layerId", "target", "seed", "tolerance", "color", "selection"], "fill-region payload");
  if (row.mutation !== undefined && row.mutation !== "fillRegion") throw new TypeError("Invalid fill-region payload");
  if (typeof row.layerId !== "string" || row.layerId.length === 0) throw new TypeError("Invalid fill layer");
  if (row.target !== "pixels" && row.target !== "mask") throw new TypeError("Invalid fill target");
  const seed = record(row.seed, ["x", "y"], "fill seed");
  if (!Array.isArray(row.color) || row.color.length !== 4 || !row.color.every((channel) => typeof channel === "number" && Number.isFinite(channel) && channel >= 0 && channel <= 1)) throw new TypeError("Invalid fill colour");
  if (row.selection !== null && !Array.isArray(row.selection)) throw new TypeError("Invalid fill selection");
  let next = 0;
  const selection = row.selection === null ? null : row.selection.map((span): RasterSelectionSpan => {
    const entry = record(span, ["start", "length", "coverage"], "selection run");
    const run = { start: integer(entry.start, "selection run", 0, 16777215), length: integer(entry.length, "selection run", 1, 16777216), coverage: integer(entry.coverage, "selection run", 0, 255) };
    if (run.start < next) throw new TypeError("Invalid selection run");
    next = run.start + run.length;
    return run;
  });
  return {
    layerId: row.layerId,
    target: row.target,
    seed: { x: integer(seed.x, "fill seed", 0, 16383), y: integer(seed.y, "fill seed", 0, 16383) },
    tolerance: integer(row.tolerance, "fill tolerance", 0, 255),
    color: row.color as [number, number, number, number],
    selection,
  };
}
