/** 🧬️ Raster artifact schema — every field with its state class. */

export interface RasterArtifact {
  schema: string;
  id: string;
  title?: string;
  layers: RasterLayerNode[];
  assets: Record<string, RasterImageAsset>;
  selectedIds: string[];
  brushSize: number;
  brushOpacity: number;
  compositeViewport?: RasterViewportSize;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  hoveredId?: string;
}

export interface RasterLayerNode {
  kind: string;
  [key: string]: unknown;
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
    assets: rasterRasterArtifactGuardObject(row["assets"], `${at}.assets`),
    selectedIds: rasterRasterArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => rasterRasterArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
    brushSize: rasterRasterArtifactGuardNumber(row["brushSize"], `${at}.brushSize`),
    brushOpacity: rasterRasterArtifactGuardNumber(row["brushOpacity"], `${at}.brushOpacity`),
    compositeViewport: row["compositeViewport"] === undefined ? undefined : parseRasterViewportSize(row["compositeViewport"], `${at}.compositeViewport`),
    cameraX: rasterRasterArtifactGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: rasterRasterArtifactGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: rasterRasterArtifactGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    hoveredId: row["hoveredId"] === undefined ? undefined : rasterRasterArtifactGuardString(row["hoveredId"], `${at}.hoveredId`),
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
  return {
    kind: rasterRasterArtifactGuardString(row["kind"], `${at}.kind`),
  };
}

export function parseRasterViewportSize(value: unknown, at = "$"): RasterViewportSize {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    width: rasterRasterArtifactGuardNumber(row["width"], `${at}.width`),
    height: rasterRasterArtifactGuardNumber(row["height"], `${at}.height`),
  };
}
