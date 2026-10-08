/** 🧬️ Raster diff schema — sparse field delta over the artifact. */
import {inverse} from "../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🟦️.ts";
import {
  parseRasterArtifact,
  parseSemioImageSnapshot,
  parseRasterLayerNode,
  parseRasterLayerMask,
  parseRasterTransform,
  rasterTransformNumbers,
  type RasterTransform,
  type RasterLayerMask,
  type RasterArtifact,
  type SemioImageSnapshot,
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
  /** @state artifact */
  pixels?: RasterPixelRegion[];
}

export interface RasterAssetsDelta {
  entries: Record<string, SemioImageSnapshot | null>;
}

export interface RasterLayersDelta {
  removed: RasterLayerRemoval[];
  inserted: RasterLayerInsertion[];
  moved: RasterLayerRelocation[];
  modified: RasterLayerModification[];
}

export interface RasterLayerAddress {
  parentId?: string;
  index: number;
}

export interface RasterLayerRemoval {
  id: string;
  parentId?: string;
  index: number;
}

export interface RasterLayerInsertion {
  parentId?: string;
  index: number;
  layer: RasterLayerNode;
}

export interface RasterLayerRelocation {
  id: string;
  from: RasterLayerAddress;
  to: RasterLayerAddress;
}

export interface RasterPixelRegion {
  layerId: string;
  target: string;
  x: number;
  y: number;
  width: number;
  height: number;
  samples: number[];
}

export interface RasterLayerModification {
  id: string;
  patch: RasterLayerPatch;
}

export interface RasterPixelContent { imageKey: string | null; width: number | null; height: number | null }

export interface RasterMaskContent { mask: RasterLayerMask | null }

export interface RasterAdjustmentParameter {parameter: "brightness" | "contrast"; value: number | null}

export interface RasterLayerPatch {
  adjustmentParameters?: RasterAdjustmentParameter[];
  maskContent?: RasterMaskContent;
  pixelContent?: RasterPixelContent;
  transform?: RasterTransform;
  name?: string;
  visible?: boolean;
  locked?: boolean;
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
    ...(Object.hasOwn(row, "pixels") ? { pixels: rasterRasterDiffGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => parseRasterPixelRegion(item, `${at}.pixels[${index}]`)) } : {}),
  };
}

export function parseRasterAssetsDelta(value: unknown, at = "$"): RasterAssetsDelta {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    entries: Object.fromEntries(
      Object.entries(rasterRasterDiffGuardObject(row["entries"], `${at}.entries`))
        .map(([key, item]) => [key, item == null ? null : parseSemioImageSnapshot(item, `${at}.entries.${key}`)]),
    ),
  };
}

export function parseRasterLayersDelta(value: unknown, at = "$"): RasterLayersDelta {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    removed: rasterRasterDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseRasterLayerRemoval(item, `${at}.removed[${index}]`)),
    inserted: rasterRasterDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseRasterLayerInsertion(item, `${at}.inserted[${index}]`)),
    moved: rasterRasterDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseRasterLayerRelocation(item, `${at}.moved[${index}]`)),
    modified: rasterRasterDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseRasterLayerModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseRasterLayerAddress(value: unknown, at = "$"): RasterLayerAddress {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    parentId: row["parentId"] == null ? undefined : rasterRasterDiffGuardString(row["parentId"], `${at}.parentId`),
    index: rasterRasterDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
  };
}

export function parseRasterLayerRemoval(value: unknown, at = "$"): RasterLayerRemoval {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    id: rasterRasterDiffGuardString(row["id"], `${at}.id`),
    parentId: row["parentId"] == null ? undefined : rasterRasterDiffGuardString(row["parentId"], `${at}.parentId`),
    index: rasterRasterDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
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

export function parseRasterLayerRelocation(value: unknown, at = "$"): RasterLayerRelocation {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    id: rasterRasterDiffGuardString(row["id"], `${at}.id`),
    from: parseRasterLayerAddress(row["from"], `${at}.from`),
    to: parseRasterLayerAddress(row["to"], `${at}.to`),
  };
}

export function parseRasterPixelRegion(value: unknown, at = "$"): RasterPixelRegion {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    layerId: rasterRasterDiffGuardString(row["layerId"], `${at}.layerId`),
    target: rasterRasterDiffGuardString(row["target"], `${at}.target`),
    x: rasterRasterDiffGuardInteger(row["x"], `${at}.x`, { minimum: 0 }),
    y: rasterRasterDiffGuardInteger(row["y"], `${at}.y`, { minimum: 0 }),
    width: rasterRasterDiffGuardInteger(row["width"], `${at}.width`, { minimum: 0 }),
    height: rasterRasterDiffGuardInteger(row["height"], `${at}.height`, { minimum: 0 }),
    samples: rasterRasterDiffGuardArray(row["samples"], `${at}.samples`).map((item, index) => rasterRasterDiffGuardInteger(item, `${at}.samples[${index}]`, { minimum: 0, maximum: 255 })),
  };
}

export function parseRasterLayerModification(value: unknown, at = "$"): RasterLayerModification {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    id: rasterRasterDiffGuardString(row["id"], `${at}.id`),
    patch: parseRasterLayerPatch(row["patch"], `${at}.patch`),
  };
}

export function parseRasterPixelContent(value: unknown, at = "$"): RasterPixelContent {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    imageKey: row.imageKey == null ? null : rasterRasterDiffGuardString(row.imageKey, `${at}.imageKey`, {minLength:1}),
    width: row.width == null ? null : rasterRasterDiffGuardInteger(row.width, `${at}.width`, {minimum:1,maximum:16384}),
    height: row.height == null ? null : rasterRasterDiffGuardInteger(row.height, `${at}.height`, {minimum:1,maximum:16384}),
  };
}

export function parseRasterMaskContent(value: unknown, at = "$"): RasterMaskContent {
  const row = rasterRasterDiffGuardObject(value, at);
  if (!Object.hasOwn(row, "mask")) return rasterRasterDiffGuardReject(`${at}.mask`, "mask replacement is missing");
  return {mask: row.mask === null ? null : parseRasterLayerMask(row.mask, `${at}.mask`)};
}

/** 🎚️ Parse a bounded tone parameter with an explicit nullable reset. */
export function parseRasterAdjustmentParameter(value: unknown, at = "$"): RasterAdjustmentParameter {
  const row = rasterRasterDiffGuardObject(value, at);
  if (row.parameter !== "brightness" && row.parameter !== "contrast") return rasterRasterDiffGuardReject(at, "unsupported adjustment parameter");
  if (!Object.hasOwn(row,"value")) return rasterRasterDiffGuardReject(at, "parameter value is missing");
  const number = row.value === null ? null : rasterRasterDiffGuardNumber(row.value, at);
  if (number !== null && (!Number.isFinite(number) || number < -1 || number > 1)) return rasterRasterDiffGuardReject(at, "adjustment is outside its range");
  return {parameter:row.parameter,value:number};
}

/** 🎛️ Compose at most two distinct tone parameters. */
export function parseRasterAdjustmentParameters(value: unknown, at = "$"): RasterAdjustmentParameter[] {
  if (!Array.isArray(value) || value.length > 2) return rasterRasterDiffGuardReject(at,"invalid adjustment parameter list");
  const result=value.map((row,index)=>parseRasterAdjustmentParameter(row,at+"["+index+"]"));
  if (new Set(result.map(row=>row.parameter)).size !== result.length) return rasterRasterDiffGuardReject(at,"duplicate adjustment parameter");
  return result;
}

export function parseRasterLayerPatch(value: unknown, at = "$"): RasterLayerPatch {
  const row = rasterRasterDiffGuardObject(value, at);
  if(row.transform!=null&&(row.transformX!=null||row.transformY!=null))return rasterRasterDiffGuardReject(at,"full and partial transforms cannot occur in one patch");
  const transform=row.transform==null?undefined:parseRasterTransform(row.transform,`${at}.transform`);
  if(transform){const t=rasterTransformNumbers(transform);inverse([t.a,t.b,t.c,t.d,t.x,t.y]);}
  return {
    name: row["name"] == null ? undefined : rasterRasterDiffGuardString(row["name"], `${at}.name`),
    visible: row["visible"] == null ? undefined : rasterRasterDiffGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] == null ? undefined : rasterRasterDiffGuardBoolean(row["locked"], `${at}.locked`),
    opacity: row["opacity"] == null ? undefined : rasterRasterDiffGuardNumber(row["opacity"], `${at}.opacity`),
    blendMode: row["blendMode"] == null ? undefined : rasterRasterDiffGuardString(row["blendMode"], `${at}.blendMode`),
    transformX: row["transformX"] == null ? undefined : rasterRasterDiffGuardNumber(row["transformX"], `${at}.transformX`),
    transformY: row["transformY"] == null ? undefined : rasterRasterDiffGuardNumber(row["transformY"], `${at}.transformY`),
    width: row["width"] == null ? undefined : rasterRasterDiffGuardInteger(row["width"], `${at}.width`, { minimum: 0 }),
    height: row["height"] == null ? undefined : rasterRasterDiffGuardInteger(row["height"], `${at}.height`, { minimum: 0 }),
    ...(row.pixelContent == null ? {} : {pixelContent: parseRasterPixelContent(row.pixelContent, `${at}.pixelContent`)}),
    ...(transform === undefined ? {} : {transform}),
    ...(row.adjustmentParameters == null ? {} : {adjustmentParameters: parseRasterAdjustmentParameters(row.adjustmentParameters, `${at}.adjustmentParameters`)}),
    ...(row.maskContent == null ? {} : {maskContent: parseRasterMaskContent(row.maskContent, `${at}.maskContent`)}),
    adjustmentKind: row["adjustmentKind"] == null ? undefined : rasterRasterDiffGuardString(row["adjustmentKind"], `${at}.adjustmentKind`),
  };
}
