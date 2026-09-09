/** 🧬️ Raster artifact schema — every field with its state class. */
import { parseDslValue, type DslValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

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
  opacity: number;
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
  opacity: number;
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
  opacity: number;
  blendMode: string;
  transform: RasterTransform;
  adjustmentKind: string;
  params: Record<string, DslValue>;
}

export interface RasterTransform {
  x: number;
  y: number;
  scaleX: number;
  scaleY: number;
  rotation: number;
}

export interface RasterLayerMask {
  enabled: boolean;
  linked: boolean;
  invert: boolean;
  width?: number;
  height?: number;
}

export interface RasterImageAsset {
  mime: string;
  data: string;
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

export function parseRasterImageAsset(value: unknown, at = "$"): RasterImageAsset {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    mime: rasterRasterArtifactGuardString(row["mime"], `${at}.mime`),
    data: rasterRasterArtifactGuardString(row["data"], `${at}.data`),
  };
}

export function parseRasterLayerNode(value: unknown, at = "$"): RasterLayerNode {
  const row = rasterRasterArtifactGuardObject(value, at);
  const kind = rasterRasterArtifactGuardMember(row["kind"], `${at}.kind`, ["pixel", "group", "adjustment"] as const);
  const common = {
    id: rasterRasterArtifactGuardString(row["id"], `${at}.id`),
    name: rasterRasterArtifactGuardString(row["name"], `${at}.name`),
    visible: rasterRasterArtifactGuardBoolean(row["visible"], `${at}.visible`),
    opacity: rasterRasterArtifactGuardNumber(row["opacity"], `${at}.opacity`),
    blendMode: rasterRasterArtifactGuardString(row["blendMode"], `${at}.blendMode`),
    transform: parseRasterTransform(row["transform"], `${at}.transform`),
  };
  if (kind === "pixel") {
    return {
      kind,
      ...common,
      mask: row["mask"] == null ? undefined : parseRasterLayerMask(row["mask"], `${at}.mask`),
      width: row["width"] == null ? undefined : rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 0 }),
      height: row["height"] == null ? undefined : rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 0 }),
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
        .map(([key, item]) => [key, parseDslValue(item)]),
    ),
  };
}

export function parseRasterTransform(value: unknown, at = "$"): RasterTransform {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    x: rasterRasterArtifactGuardNumber(row["x"], `${at}.x`),
    y: rasterRasterArtifactGuardNumber(row["y"], `${at}.y`),
    scaleX: rasterRasterArtifactGuardNumber(row["scaleX"], `${at}.scaleX`),
    scaleY: rasterRasterArtifactGuardNumber(row["scaleY"], `${at}.scaleY`),
    rotation: rasterRasterArtifactGuardNumber(row["rotation"], `${at}.rotation`),
  };
}

export function parseRasterLayerMask(value: unknown, at = "$"): RasterLayerMask {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    enabled: rasterRasterArtifactGuardBoolean(row["enabled"], `${at}.enabled`),
    linked: rasterRasterArtifactGuardBoolean(row["linked"], `${at}.linked`),
    invert: rasterRasterArtifactGuardBoolean(row["invert"], `${at}.invert`),
    width: row["width"] == null ? undefined : rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 0 }),
    height: row["height"] == null ? undefined : rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 0 }),
  };
}

export function parseRasterViewportSize(value: unknown, at = "$"): RasterViewportSize {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    width: rasterRasterArtifactGuardNumber(row["width"], `${at}.width`),
    height: rasterRasterArtifactGuardNumber(row["height"], `${at}.height`),
  };
}
