/** 🧬️ Raster artifact schema — every field with its state class. */
import type { SemioImageSnapshot } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/🟦️.ts";
export type { SemioImageSnapshot } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/🟦️.ts";
import { parseIntrinsicValue, type IntrinsicValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

import { type Binary64, type Binary32, binary64Value } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseBinary64, parseBinary32 } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

export interface RasterArtifact {
  schema: string;
  id: string;
  title?: string;
  layers: RasterLayerNode[];
  assets: Record<string, ArtifactChild>;
}

export type RasterLayerNode = RasterLayerPixel | RasterLayerGroup | RasterLayerAdjustment;

export interface RasterLayerPixel {
  kind: "pixel";
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: Binary32;
  blendMode: string;
  transform: RasterTransform;
  mask?: RasterLayerMask;
  width?: number;
  height?: number;
  imageKey?: string;
}

export interface RasterLayerGroup {
  kind: "group";
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: Binary32;
  blendMode: string;
  transform: RasterTransform;
  mask?: RasterLayerMask;
  children: RasterLayerNode[];
}

export interface RasterLayerAdjustment {
  kind: "adjustment";
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: Binary32;
  blendMode: string;
  transform: RasterTransform;
  adjustmentKind: string;
  params: Record<string, IntrinsicValue>;
}

export interface RasterTransform {
  x: Binary64;
  y: Binary64;
  a: Binary64;
  b: Binary64;
  c: Binary64;
  d: Binary64;
}

export interface RasterLayerMask {
  enabled: boolean;
  linked: boolean;
  invert: boolean;
  width?: number;
  height?: number;
  imageKey?: string;
  transform: RasterTransform;
}


export interface RasterViewportSize {
  width: number;
  height: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterArtifactGuardReject = (at: string, why: string): never => {
  throw new rasterRasterArtifactGuardRefusal(at, why);
};

type rasterRasterArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterArtifactGuardReject(at, "value is not an object");
export const rasterRasterArtifactGuardArray = (value: unknown, at: string, bounds: rasterRasterArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterArtifactGuardString = (value: unknown, at: string, bounds: rasterRasterArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterArtifactGuardReject(at, "value is not a boolean"));
export const rasterRasterArtifactGuardNumber = (value: unknown, at: string, bounds: rasterRasterArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterArtifactGuardInteger = (value: unknown, at: string, bounds: rasterRasterArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterArtifactGuardNumber(value, at, bounds) : rasterRasterArtifactGuardReject(at, "value is not an integer");
export const rasterRasterArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterArtifact(value: unknown, at = "$"): RasterArtifact {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    schema: rasterRasterArtifactGuardString(row["schema"], `${at}.schema`),
    id: rasterRasterArtifactGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : rasterRasterArtifactGuardString(row["title"], `${at}.title`),
    layers: rasterRasterArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseRasterLayerNode(item, `${at}.layers[${index}]`)),
    assets: Object.fromEntries(
      Object.entries(rasterRasterArtifactGuardObject(row["assets"], `${at}.assets`))
        .map(([key, item]) => [key, parseArtifactChild(item)]),
    ),
  };
}

/** 🖼️ Validates decoded image state without opening any carrier. */
export function parseSemioImageSnapshot(value: unknown, at = "$"): SemioImageSnapshot {
  const row = rasterRasterArtifactGuardObject(value, at);
  const bytes = (value: unknown, path: string) => rasterRasterArtifactGuardArray(value, path).map((sample, index) => rasterRasterArtifactGuardInteger(sample, `${path}[${index}]`, { minimum: 0, maximum: 255 }));
  return {
    schema: rasterRasterArtifactGuardString(row["schema"], `${at}.schema`),
    width: rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 0, maximum: 4294967295 }),
    height: rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 0, maximum: 4294967295 }),
    colorspace: rasterRasterArtifactGuardMember(row["colorspace"] ?? "rgb", `${at}.colorspace`, ["rgb", "rgba", "grayscale", "grayscaleAlpha", "indexed"] as const),
    bitDepth: rasterRasterArtifactGuardInteger(row["bitDepth"] ?? 0, `${at}.bitDepth`, { minimum: 0, maximum: 255 }),
    frames: rasterRasterArtifactGuardArray(row["frames"] ?? [], `${at}.frames`).map((item, index) => {
      const frame = rasterRasterArtifactGuardObject(item, `${at}.frames[${index}]`);
      return { delayMs: rasterRasterArtifactGuardInteger(frame["delayMs"], `${at}.frames[${index}].delayMs`, { minimum: 0, maximum: 4294967295 }), rgba8: bytes(frame["rgba8"] ?? [], `${at}.frames[${index}].rgba8`) };
    }),
    icc: row["icc"] == null ? null : bytes(row["icc"], `${at}.icc`),
    metadata: rasterRasterArtifactGuardArray(row["metadata"] ?? [], `${at}.metadata`).map((item, index) => {
      const entry = rasterRasterArtifactGuardObject(item, `${at}.metadata[${index}]`);
      return { key: rasterRasterArtifactGuardString(entry["key"], `${at}.metadata[${index}].key`), value: rasterRasterArtifactGuardString(entry["value"] ?? "", `${at}.metadata[${index}].value`) };
    }),
  };
}

export function parseRasterLayerNode(value: unknown, at = "$"): RasterLayerNode {
  const row = rasterRasterArtifactGuardObject(value, at);
  const kind = rasterRasterArtifactGuardMember(row["kind"], `${at}.kind`, ["pixel", "group", "adjustment"] as const);
  const common = {
    id: rasterRasterArtifactGuardString(row["id"], `${at}.id`),
    name: rasterRasterArtifactGuardString(row["name"], `${at}.name`),
    visible: rasterRasterArtifactGuardBoolean(row["visible"], `${at}.visible`),
    locked: rasterRasterArtifactGuardBoolean(row["locked"], `${at}.locked`),
    opacity: parseBinary32(row["opacity"]),
    blendMode: rasterRasterArtifactGuardString(row["blendMode"], `${at}.blendMode`),
    transform: parseRasterTransform(row["transform"], `${at}.transform`),
  };
  if (kind === "pixel") {
    return {
      kind,
      ...common,
      mask: row["mask"] == null ? undefined : parseRasterLayerMask(row["mask"], `${at}.mask`),
      width: row["width"] == null ? undefined : rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 0, maximum: 4294967295 }),
      height: row["height"] == null ? undefined : rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 0, maximum: 4294967295 }),
      imageKey: row["imageKey"] == null ? undefined : rasterRasterArtifactGuardString(row["imageKey"], `${at}.imageKey`),
    };
  }
  if (kind === "group") {
    return {
      kind,
      ...common,
      mask: row["mask"] == null ? undefined : parseRasterLayerMask(row["mask"], `${at}.mask`),
      children: rasterRasterArtifactGuardArray(row["children"], `${at}.children`).map((item, index) => parseRasterLayerNode(item, `${at}.children[${index}]`)),
    };
  }
  return {
    kind,
    ...common,
    adjustmentKind: rasterRasterArtifactGuardString(row["adjustmentKind"], `${at}.adjustmentKind`),
    params: Object.fromEntries(
      Object.entries(rasterRasterArtifactGuardObject(row["params"], `${at}.params`))
        .map(([key, item]) => [key, parseIntrinsicValue(item)]),
    ),
  };
}

export function parseRasterTransform(value: unknown, at = "$"): RasterTransform {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    x: parseBinary64(row["x"]),
    y: parseBinary64(row["y"]),
    a: parseBinary64(row["a"]),
    b: parseBinary64(row["b"]),
    c: parseBinary64(row["c"]),
    d: parseBinary64(row["d"]),
  };
}

export function parseRasterLayerMask(value: unknown, at = "$"): RasterLayerMask {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    enabled: rasterRasterArtifactGuardBoolean(row["enabled"], `${at}.enabled`),
    linked: rasterRasterArtifactGuardBoolean(row["linked"], `${at}.linked`),
    invert: rasterRasterArtifactGuardBoolean(row["invert"], `${at}.invert`),
    width: row["width"] == null ? undefined : rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 1, maximum: 16384 }),
    height: row["height"] == null ? undefined : rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 1, maximum: 16384 }),
    imageKey: row["imageKey"] == null ? undefined : rasterRasterArtifactGuardString(row["imageKey"], `${at}.imageKey`, { minLength: 1 }),
    transform: parseRasterTransform(row["transform"], `${at}.transform`),
  };
}

//#region 🔖️JsonProjection
/** 🔢️ The transform's derived numeric values — what compositing math and the JSON wire read; the exact identity stays the words. */
export function rasterTransformNumbers(transform: RasterTransform): { x: number; y: number; a: number; b: number; c: number; d: number } {
  return { x: binary64Value(transform.x), y: binary64Value(transform.y), a: binary64Value(transform.a), b: binary64Value(transform.b), c: binary64Value(transform.c), d: binary64Value(transform.d) };
}

//#endregion 🔖️JsonProjection

export function parseRasterViewportSize(value: unknown, at = "$"): RasterViewportSize {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    width: rasterRasterArtifactGuardNumber(row["width"], `${at}.width`),
    height: rasterRasterArtifactGuardNumber(row["height"], `${at}.height`),
  };
}

export type LayerProtectionNode={id:string;locked:boolean;children?:readonly LayerProtectionNode[]};
export type LayerProtection={locked:boolean;inherited:boolean;descendant:boolean;editable:boolean;structural:boolean;canChangeLock:boolean};
export function layerProtection(layers:readonly LayerProtectionNode[],id:string):LayerProtection|null {
  const lockedBelow=(nodes:readonly LayerProtectionNode[]):boolean=>nodes.some(node=>node.locked||lockedBelow(node.children??[]));
  const visit=(nodes:readonly LayerProtectionNode[],inherited:boolean):LayerProtection|null=>{
    for(const node of nodes) {
      if(node.id===id) {const descendant=lockedBelow(node.children??[]),editable=!node.locked&&!inherited;return {locked:node.locked,inherited,descendant,editable,structural:editable&&!descendant,canChangeLock:!inherited};}
      const found=visit(node.children??[],inherited||node.locked);if(found)return found;
    }
    return null;
  };
  return visit(layers,false);
}
