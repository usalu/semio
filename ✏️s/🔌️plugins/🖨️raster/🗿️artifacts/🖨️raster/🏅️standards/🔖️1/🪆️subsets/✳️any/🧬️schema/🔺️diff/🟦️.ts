/** 🧬️ Raster diff schema — sparse field delta over the artifact. */
import {
  parseRasterArtifact,
  parseRasterImageAsset,
  parseRasterLayerNode,
  type RasterArtifact,
  type RasterImageAsset,
  type RasterLayerNode,
} from "../🟦️.ts";

export interface RasterDiff {
  /** @state artifact */
  artifact?: RasterArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  layers?: RasterLayersDelta;
  /** @state artifact */
  assets?: RasterAssetsDelta;
}

export interface RasterAssetsDelta {
  entries: Record<string, RasterImageAsset | null>;
}

export interface RasterLayersDelta {
  added: RasterLayerInsertion[];
  removed: string[];
  patched: RasterLayerPatchEntry[];
  moved: RasterLayerMove[];
}

export interface RasterLayerInsertion {
  parentId?: string;
  index: number;
  layer: RasterLayerNode;
}

export interface RasterLayerMove {
  id: string;
  parentId?: string;
  index: number;
}

export interface RasterLayerPatchEntry {
  id: string;
  patch: RasterLayerPatch;
}

export interface RasterLayerPatch {
  name?: string;
  visible?: boolean;
  opacity?: number;
  blendMode?: string;
  transformX?: number;
  transformY?: number;
  width?: number;
  height?: number;
  adjustmentKind?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterDiffGuardReject = (at: string, why: string): never => {
  throw new rasterRasterDiffGuardRefusal(at, why);
};

type rasterRasterDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterDiffGuardReject(at, "value is not an object");
export const rasterRasterDiffGuardArray = (value: unknown, at: string, bounds: rasterRasterDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterDiffGuardString = (value: unknown, at: string, bounds: rasterRasterDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterDiffGuardReject(at, "value is not a boolean"));
export const rasterRasterDiffGuardNumber = (value: unknown, at: string, bounds: rasterRasterDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterDiffGuardInteger = (value: unknown, at: string, bounds: rasterRasterDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterDiffGuardNumber(value, at, bounds) : rasterRasterDiffGuardReject(at, "value is not an integer");
export const rasterRasterDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterDiff(value: unknown, at = "$"): RasterDiff {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    ...(Object.hasOwn(row, "artifact") ? { artifact: row["artifact"] == null ? undefined : parseRasterArtifact(row["artifact"], `${at}.artifact`) } : {}),
    ...(Object.hasOwn(row, "schema") ? { schema: row["schema"] == null ? undefined : rasterRasterDiffGuardString(row["schema"], `${at}.schema`) } : {}),
    ...(Object.hasOwn(row, "id") ? { id: row["id"] == null ? undefined : rasterRasterDiffGuardString(row["id"], `${at}.id`) } : {}),
    ...(Object.hasOwn(row, "title") ? { title: row["title"] == null ? null : rasterRasterDiffGuardString(row["title"], `${at}.title`) } : {}),
    ...(Object.hasOwn(row, "layers") ? { layers: row["layers"] == null ? undefined : parseRasterLayersDelta(row["layers"], `${at}.layers`) } : {}),
    ...(Object.hasOwn(row, "assets") ? { assets: row["assets"] == null ? undefined : parseRasterAssetsDelta(row["assets"], `${at}.assets`) } : {}),
  };
}

export function parseRasterAssetsDelta(value: unknown, at = "$"): RasterAssetsDelta {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    entries: Object.fromEntries(
      Object.entries(rasterRasterDiffGuardObject(row["entries"], `${at}.entries`))
        .map(([key, item]) => [key, item == null ? null : parseRasterImageAsset(item, `${at}.entries.${key}`)]),
    ),
  };
}

export function parseRasterLayersDelta(value: unknown, at = "$"): RasterLayersDelta {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    added: rasterRasterDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseRasterLayerInsertion(item, `${at}.added[${index}]`)),
    removed: rasterRasterDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => rasterRasterDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: rasterRasterDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseRasterLayerPatchEntry(item, `${at}.patched[${index}]`)),
    moved: rasterRasterDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseRasterLayerMove(item, `${at}.moved[${index}]`)),
  };
}

export function parseRasterLayerInsertion(value: unknown, at = "$"): RasterLayerInsertion {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    parentId: row["parentId"] == null ? undefined : rasterRasterDiffGuardString(row["parentId"], `${at}.parentId`),
    index: rasterRasterDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
    layer: parseRasterLayerNode(row["layer"], `${at}.layer`),
  };
}

export function parseRasterLayerMove(value: unknown, at = "$"): RasterLayerMove {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    id: rasterRasterDiffGuardString(row["id"], `${at}.id`),
    parentId: row["parentId"] == null ? undefined : rasterRasterDiffGuardString(row["parentId"], `${at}.parentId`),
    index: rasterRasterDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
  };
}

export function parseRasterLayerPatchEntry(value: unknown, at = "$"): RasterLayerPatchEntry {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    id: rasterRasterDiffGuardString(row["id"], `${at}.id`),
    patch: parseRasterLayerPatch(row["patch"], `${at}.patch`),
  };
}

export function parseRasterLayerPatch(value: unknown, at = "$"): RasterLayerPatch {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    name: row["name"] == null ? undefined : rasterRasterDiffGuardString(row["name"], `${at}.name`),
    visible: row["visible"] == null ? undefined : rasterRasterDiffGuardBoolean(row["visible"], `${at}.visible`),
    opacity: row["opacity"] == null ? undefined : rasterRasterDiffGuardNumber(row["opacity"], `${at}.opacity`),
    blendMode: row["blendMode"] == null ? undefined : rasterRasterDiffGuardString(row["blendMode"], `${at}.blendMode`),
    transformX: row["transformX"] == null ? undefined : rasterRasterDiffGuardNumber(row["transformX"], `${at}.transformX`),
    transformY: row["transformY"] == null ? undefined : rasterRasterDiffGuardNumber(row["transformY"], `${at}.transformY`),
    width: row["width"] == null ? undefined : rasterRasterDiffGuardInteger(row["width"], `${at}.width`, { minimum: 0 }),
    height: row["height"] == null ? undefined : rasterRasterDiffGuardInteger(row["height"], `${at}.height`, { minimum: 0 }),
    adjustmentKind: row["adjustmentKind"] == null ? undefined : rasterRasterDiffGuardString(row["adjustmentKind"], `${at}.adjustmentKind`),
  };
}
