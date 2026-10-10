import {parseDrawingFontFamily,type DrawingFontFamily} from "../📝️text/🔤️family/🟦️.ts";
import {SHAPE_COORDINATE_FIELDS,type ShapeCoordinateField} from "../🔷️shape/✏️coordinates/🟦️.ts";
import {parseFillRule,type FillRule} from "../🎨️fill/🌀️rule/🟦️.ts";
/** 🔺️ Mirrors Rust `DrawingDiff` (sparse field delta over the drawing artifact; sibling `🦀️.rs`,
 * `#[serde(rename_all = "camelCase", default)]`). Every top-level field is an optional patch slot;
 * a Rust `Option<Option<T>>` field (touched-but-cleared vs untouched) collapses to `T | null`
 * here, since JSON already conflates "absent key" with "not present" once serialized sparsely (same
 * `T | null` collapse as `manifestId`/`rootNodeId` on the trinity/jack artifact's own `JackDiff`).
 * Nested types
 * re-import the artifact's own root schema (`../🟦️.ts`) rather than re-declaring stubs, so
 * every facet of the drawing artifact agrees on the same `DrawingLayerNode`/`DrawingImageAsset`/
 * `DrawingArtboard`. */
import {
  parsePathGeometrySegment,
  parseDrawingTransform, parseDrawingFill, parseDrawingStroke, parseDrawingTraceParams,
  type DrawingTransform, type DrawingFill, type DrawingStroke, type DrawingTraceParams,
  parseBlendMode,
  type BlendMode,
  type PathGeometrySegment,
  parseDrawingArtboard,
  parseDrawingImageAsset,
  parseDrawingLayerNode,
  type DrawingArtboard,
  type DrawingImageAsset,
  type DrawingLayerNode,
} from "../🟦️.ts";

export interface DrawingDiff {
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  layers?: DrawingLayersDelta;
  /** @state artifact */
  assets?: DrawingAssetsDelta;
  /** @state artifact */
  artboard?: DrawingArtboard | null;
}

/** 🗂️ Mirrors Rust `DrawingAssetsDelta` — asset-map wrapper so optional map diffs stay scalar across
 * formats; `null` marks a removed asset. */
export interface DrawingAssetsDelta {
  entries: Record<string, DrawingImageAsset | null>;
}

/** 📋 Mirrors Rust `DrawingStringList` — string-list wrapper so optional list diffs stay scalar
 * across formats. */
export interface DrawingStringList {
  values: string[];
}

/** 🧩 Mirrors Rust `DrawingLayersDelta` — positional delta for the layer tree (per container, no order list, no anchor). */
export interface DrawingLayersDelta {
  removed: DrawingLayerRemoval[];
  inserted: DrawingLayerInsertion[];
  moved: DrawingLayerRelocation[];
  modified: DrawingLayerModification[];
}

/** 📍️ Mirrors Rust `DrawingLayerAddress` — the child list of `parentId` (root when absent) at `index`. */
export interface DrawingLayerAddress {
  parentId?: string;
  index: number;
}

/** ➖️ Mirrors Rust `DrawingLayerRemoval` — one removed layer subtree at its BASE address. */
export interface DrawingLayerRemoval {
  id: string;
  parentId?: string;
  index: number;
}

/** ➕️ Mirrors Rust `DrawingLayerInsertion` — one inserted layer subtree at its AFTER address. */
export interface DrawingLayerInsertion {
  parentId?: string;
  index: number;
  layer: DrawingLayerNode;
}

/** ↕️ Mirrors Rust `DrawingLayerRelocation` — one repositioned layer subtree: its BASE and its AFTER address. */
export interface DrawingLayerRelocation {
  id: string;
  from: DrawingLayerAddress;
  to: DrawingLayerAddress;
}

/** 🩹 Mirrors Rust `DrawingLayerModification` — one modified layer entry. */
export interface DrawingLayerModification {
  id: string;
  patch: DrawingLayerPatch;
}

/** 🩹 Sparse changes over decoded domain values. */
export interface DrawingLayerPatch {
  visible?: boolean;
  locked?: boolean;
  name?: string;
  opacity?: number;
  blendMode?: BlendMode;
  fillRule?:FillRule;
  isolation?:boolean;
  transform?: DrawingTransform;
  fill?: {value:DrawingFill|null};
  stroke?: {value:DrawingStroke|null};
  booleanOperation?: string;
  traceParams?: DrawingTraceParams;
  layer?: DrawingLayerNode;
  pathSegments?: PathGeometrySegment[];
  textContent?: string;
  textSize?: number;
  fontFamily?: DrawingFontFamily;
  imageKey?:string;
  imageWidth?:number;
  imageHeight?:number;
  shapeCoordinates?:{field:ShapeCoordinateField;index?:number|null;value:number}[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingDiffGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingDiffGuardRefusal(at, why);
};

type drawingDrawingDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingDiffGuardReject(at, "value is not an object");
export const drawingDrawingDiffGuardArray = (value: unknown, at: string, bounds: drawingDrawingDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingDiffGuardString = (value: unknown, at: string, bounds: drawingDrawingDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingDiffGuardReject(at, "value is not a boolean"));
export const drawingDrawingDiffGuardNumber = (value: unknown, at: string, bounds: drawingDrawingDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingDiffGuardInteger = (value: unknown, at: string, bounds: drawingDrawingDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingDiffGuardNumber(value, at, bounds) : drawingDrawingDiffGuardReject(at, "value is not an integer");
export const drawingDrawingDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingDiff(value: unknown, at = "$"): DrawingDiff {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    ...(Object.hasOwn(row, "schema") ? { schema: row["schema"] == null ? undefined : drawingDrawingDiffGuardString(row["schema"], `${at}.schema`) } : {}),
    ...(Object.hasOwn(row, "id") ? { id: row["id"] == null ? undefined : drawingDrawingDiffGuardString(row["id"], `${at}.id`) } : {}),
    ...(Object.hasOwn(row, "title") ? { title: row["title"] == null ? null : drawingDrawingDiffGuardString(row["title"], `${at}.title`) } : {}),
    ...(Object.hasOwn(row, "layers") ? { layers: row["layers"] == null ? undefined : parseDrawingLayersDelta(row["layers"], `${at}.layers`) } : {}),
    ...(Object.hasOwn(row, "assets") ? { assets: row["assets"] == null ? undefined : parseDrawingAssetsDelta(row["assets"], `${at}.assets`) } : {}),
    ...(Object.hasOwn(row, "artboard") ? { artboard: row["artboard"] == null ? null : parseDrawingArtboard(row["artboard"], `${at}.artboard`) } : {}),
  };
}

export function parseDrawingAssetsDelta(value: unknown, at = "$"): DrawingAssetsDelta {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    entries: Object.fromEntries(
      Object.entries(drawingDrawingDiffGuardObject(row["entries"], `${at}.entries`))
        .map(([key, item]) => [key, item == null ? null : parseDrawingImageAsset(item, `${at}.entries.${key}`)]),
    ),
  };
}

export function parseDrawingLayersDelta(value: unknown, at = "$"): DrawingLayersDelta {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    removed: drawingDrawingDiffGuardArray(row["removed"] ?? [], `${at}.removed`).map((item, index) => parseDrawingLayerRemoval(item, `${at}.removed[${index}]`)),
    inserted: drawingDrawingDiffGuardArray(row["inserted"] ?? [], `${at}.inserted`).map((item, index) => parseDrawingLayerInsertion(item, `${at}.inserted[${index}]`)),
    moved: drawingDrawingDiffGuardArray(row["moved"] ?? [], `${at}.moved`).map((item, index) => parseDrawingLayerRelocation(item, `${at}.moved[${index}]`)),
    modified: drawingDrawingDiffGuardArray(row["modified"] ?? [], `${at}.modified`).map((item, index) => parseDrawingLayerModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseDrawingLayerAddress(value: unknown, at = "$"): DrawingLayerAddress {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    parentId: row["parentId"] === undefined ? undefined : drawingDrawingDiffGuardString(row["parentId"], `${at}.parentId`),
    index: drawingDrawingDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
  };
}

export function parseDrawingLayerRemoval(value: unknown, at = "$"): DrawingLayerRemoval {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    id: drawingDrawingDiffGuardString(row["id"], `${at}.id`),
    parentId: row["parentId"] === undefined ? undefined : drawingDrawingDiffGuardString(row["parentId"], `${at}.parentId`),
    index: drawingDrawingDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
  };
}

export function parseDrawingLayerInsertion(value: unknown, at = "$"): DrawingLayerInsertion {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    parentId: row["parentId"] === undefined ? undefined : drawingDrawingDiffGuardString(row["parentId"], `${at}.parentId`),
    index: drawingDrawingDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
    layer: parseDrawingLayerNode(row["layer"], `${at}.layer`),
  };
}

export function parseDrawingLayerRelocation(value: unknown, at = "$"): DrawingLayerRelocation {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    id: drawingDrawingDiffGuardString(row["id"], `${at}.id`),
    from: parseDrawingLayerAddress(row["from"], `${at}.from`),
    to: parseDrawingLayerAddress(row["to"], `${at}.to`),
  };
}

export function parseDrawingLayerModification(value: unknown, at = "$"): DrawingLayerModification {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    id: drawingDrawingDiffGuardString(row["id"], `${at}.id`),
    patch: parseDrawingLayerPatch(row["patch"], `${at}.patch`),
  };
}

export function parseDrawingLayerPatch(value: unknown, at = "$"): DrawingLayerPatch {
  const row = drawingDrawingDiffGuardObject(value, at);
  const text = (key: string): string | undefined => row[key] == null ? undefined : drawingDrawingDiffGuardString(row[key], `${at}.${key}`);
  return {
    visible: row["visible"] == null ? undefined : drawingDrawingDiffGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] == null ? undefined : drawingDrawingDiffGuardBoolean(row["locked"], `${at}.locked`),
    name: text("name"),
    opacity: row["opacity"] == null ? undefined : drawingDrawingDiffGuardNumber(row["opacity"], `${at}.opacity`),
    blendMode: row.blendMode == null ? undefined : parseBlendMode(row.blendMode,`${at}.blendMode`),
    fillRule:row.fillRule==null?undefined:parseFillRule(row.fillRule),
    isolation:row.isolation==null?undefined:drawingDrawingDiffGuardBoolean(row.isolation,`${at}.isolation`),
    transform: row["transform"] == null ? undefined : parseDrawingTransform(row["transform"], at+".transform"),
    fill: row["fill"] == null ? undefined : {value: drawingDrawingDiffGuardObject(row["fill"],at+".fill").value == null ? null : parseDrawingFill(drawingDrawingDiffGuardObject(row["fill"],at+".fill").value,at+".fill.value")},
    stroke: row["stroke"] == null ? undefined : {value: drawingDrawingDiffGuardObject(row["stroke"],at+".stroke").value == null ? null : parseDrawingStroke(drawingDrawingDiffGuardObject(row["stroke"],at+".stroke").value,at+".stroke.value")},
    booleanOperation: text("booleanOperation"),
    traceParams: row["traceParams"] == null ? undefined : parseDrawingTraceParams(row["traceParams"], at+".traceParams"),
    layer: row["layer"] == null ? undefined : parseDrawingLayerNode(row["layer"], at+".layer"),
    shapeCoordinates:row.shapeCoordinates==null?undefined:drawingDrawingDiffGuardArray(row.shapeCoordinates,`${at}.shapeCoordinates`).map((coordinate,index)=>{
      const point=drawingDrawingDiffGuardObject(coordinate,`${at}.shapeCoordinates[${index}]`);
      return {field:drawingDrawingDiffGuardMember(point.field,`${at}.shapeCoordinates[${index}].field`,SHAPE_COORDINATE_FIELDS),index:point.index==null?undefined:drawingDrawingDiffGuardNumber(point.index,`${at}.shapeCoordinates[${index}].index`),value:drawingDrawingDiffGuardNumber(point.value,`${at}.shapeCoordinates[${index}].value`)};
    }),
    imageKey:text("imageKey"),
    imageWidth:row.imageWidth==null?undefined:drawingDrawingDiffGuardNumber(row.imageWidth,`${at}.imageWidth`),
    imageHeight:row.imageHeight==null?undefined:drawingDrawingDiffGuardNumber(row.imageHeight,`${at}.imageHeight`),
    textContent: text("textContent"),
    fontFamily:row.fontFamily===undefined?undefined:parseDrawingFontFamily(row.fontFamily),
    textSize: row["textSize"] == null ? undefined : drawingDrawingDiffGuardNumber(row["textSize"], `${at}.textSize`),
    pathSegments: row["pathSegments"] == null ? undefined : drawingDrawingDiffGuardArray(row["pathSegments"], `${at}.pathSegments`).map((item, index) => parsePathGeometrySegment(item, `${at}.pathSegments[${index}]`)),
  };
}
