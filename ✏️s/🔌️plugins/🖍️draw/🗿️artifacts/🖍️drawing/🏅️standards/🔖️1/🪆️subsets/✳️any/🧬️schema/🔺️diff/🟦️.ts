/** 🔺️ Mirrors Rust `DrawingDiff` (sparse field delta over the drawing artifact; sibling `🦀️.rs`,
 * `#[serde(rename_all = "camelCase", default)]`). Every top-level field is an optional patch slot;
 * a Rust `Option<Option<T>>` field (touched-but-cleared vs untouched) collapses to `T | null`
 * here, since JSON already conflates "absent key" with "not present" once serialized sparsely (same
 * `T | null` collapse as `manifestId`/`rootNodeId` on the trinity/jack artifact's own `JackDiff`).
 * Nested types
 * re-import the artifact's own root schema (`../🟦️.ts`) rather than re-declaring stubs, so
 * every facet of the drawing artifact agrees on the same `DrawingLayerNode`/`DrawingImageAsset`/
 * `DrawingArtboard`/`DrawingArtifact`. */
import {
  parseDrawingArtifact,
  parseDrawingArtboard,
  parseDrawingImageAsset,
  parseDrawingLayerNode,
  type DrawingArtifact,
  type DrawingArtboard,
  type DrawingImageAsset,
  type DrawingLayerNode,
} from "../🟦️.ts";

export interface DrawingDiff {
  /** @state artifact */
  artifact?: DrawingArtifact;
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

/** 🧩 Mirrors Rust `DrawingLayersDelta` — identified-collection delta for `layers`. */
export interface DrawingLayersDelta {
  added: DrawingLayerAddition[];
  removed: string[];
  patched: DrawingLayerPatchEntry[];
  reordered?: string[];
}

/** ➕️ Mirrors Rust `DrawingLayerAddition` — one inserted layer with its real (parent, index)
 * target location. */
export interface DrawingLayerAddition {
  parentId?: string;
  index: number;
  layer: DrawingLayerNode;
}

/** 🩹 Mirrors Rust `DrawingLayerPatchEntry` — one patched layer entry. */
export interface DrawingLayerPatchEntry {
  id: string;
  patch: DrawingLayerPatch;
}

/** 🩹 Mirrors Rust `DrawingLayerPatch` — sparse layer field patch (JSON blobs for complex nested
 * values: transform/fill/stroke/trace params are re-serialized rather than typed directly, matching
 * the Rust struct's own `*_json: Option<String>` fields). */
export interface DrawingLayerPatch {
  visible?: boolean;
  locked?: boolean;
  name?: string;
  opacity?: number;
  blendMode?: string;
  transformJson?: string;
  fillJson?: string;
  strokeJson?: string;
  booleanOperation?: string;
  traceParamsJson?: string;
  layerJson?: string;
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
    ...(Object.hasOwn(row, "artifact") ? { artifact: row["artifact"] == null ? undefined : parseDrawingArtifact(row["artifact"], `${at}.artifact`) } : {}),
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
    added: drawingDrawingDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseDrawingLayerAddition(item, `${at}.added[${index}]`)),
    removed: drawingDrawingDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => drawingDrawingDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: drawingDrawingDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseDrawingLayerPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] == null ? undefined : drawingDrawingDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => drawingDrawingDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseDrawingLayerAddition(value: unknown, at = "$"): DrawingLayerAddition {
  const row = drawingDrawingDiffGuardObject(value, at);
  return {
    parentId: row["parentId"] === undefined ? undefined : drawingDrawingDiffGuardString(row["parentId"], `${at}.parentId`),
    index: drawingDrawingDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
    layer: parseDrawingLayerNode(row["layer"], `${at}.layer`),
  };
}

export function parseDrawingLayerPatchEntry(value: unknown, at = "$"): DrawingLayerPatchEntry {
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
    blendMode: text("blendMode"),
    transformJson: text("transformJson"),
    fillJson: text("fillJson"),
    strokeJson: text("strokeJson"),
    booleanOperation: text("booleanOperation"),
    traceParamsJson: text("traceParamsJson"),
    layerJson: text("layerJson"),
  };
}
