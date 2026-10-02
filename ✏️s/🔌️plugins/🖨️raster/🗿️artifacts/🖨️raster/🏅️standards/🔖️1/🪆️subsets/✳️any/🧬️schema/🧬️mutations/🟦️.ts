/** 🎭️ Canonical document types shared by mutation payloads. */
import type {RasterLayerNode, RasterLayerMask, RasterTransform, RasterImageAsset} from "../🟦️.ts";
export type {RasterLayerNode, RasterLayerMask, RasterTransform, RasterImageAsset} from "../🟦️.ts";
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
  | { mutation: 'addLayerAsset'; assetId: string; asset: RasterImageAsset }
  | { mutation: 'removeLayerAsset'; assetId: string }
  | { mutation: 'changeLayerPixels'; layerId: string; expectedImageKey: string | null; content: RasterPixelContent; transform?: RasterTransform | null }
  | { mutation: 'changeLayerMask'; layerId: string; expected: RasterLayerMask | null; mask: RasterLayerMask | null }
  | ({ mutation: 'paintStroke' } & PaintStroke)
  | ({ mutation: 'fillRegion' } & FillRegion);

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
