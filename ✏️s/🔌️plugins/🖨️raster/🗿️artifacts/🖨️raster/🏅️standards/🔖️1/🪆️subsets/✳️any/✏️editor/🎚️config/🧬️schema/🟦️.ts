/** 🧬️ RasterConfig */
export interface RasterCamera {
  /** @state config */
  x: number;
  /** @state config */
  y: number;
  /** @state config */
  zoom: number;
}

export interface RasterConfigViewportSize {
  /** @state config */
  width: number;
  /** @state config */
  height: number;
}

export interface RasterConfig {
  /** @state config */
  brushSize: number;
  /** @state config */
  brushOpacity: number;
  /** @state config */
  compositeViewport?: RasterConfigViewportSize;
  /** @state config */
  camera: RasterCamera;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterConfigGuardReject = (at: string, why: string): never => {
  throw new rasterRasterConfigGuardRefusal(at, why);
};

type rasterRasterConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterConfigGuardReject(at, "value is not an object");
export const rasterRasterConfigGuardArray = (value: unknown, at: string, bounds: rasterRasterConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterConfigGuardString = (value: unknown, at: string, bounds: rasterRasterConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterConfigGuardReject(at, "value is not a boolean"));
export const rasterRasterConfigGuardNumber = (value: unknown, at: string, bounds: rasterRasterConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterConfigGuardInteger = (value: unknown, at: string, bounds: rasterRasterConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterConfigGuardNumber(value, at, bounds) : rasterRasterConfigGuardReject(at, "value is not an integer");
export const rasterRasterConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterConfig(value: unknown, at = "$"): RasterConfig {
  const row = rasterRasterConfigGuardObject(value, at);
  return {
    brushSize: rasterRasterConfigGuardNumber(row["brushSize"], `${at}.brushSize`),
    brushOpacity: rasterRasterConfigGuardNumber(row["brushOpacity"], `${at}.brushOpacity`),
    compositeViewport: row["compositeViewport"] === undefined ? undefined : parseRasterConfigViewportSize(row["compositeViewport"], `${at}.compositeViewport`),
    camera: parseRasterCamera(row["camera"], `${at}.camera`),
  };
}

export function parseRasterCamera(value: unknown, at = "$"): RasterCamera {
  const row = rasterRasterConfigGuardObject(value, at);
  return {
    x: rasterRasterConfigGuardNumber(row["x"], `${at}.x`),
    y: rasterRasterConfigGuardNumber(row["y"], `${at}.y`),
    zoom: rasterRasterConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parseRasterConfigViewportSize(value: unknown, at = "$"): RasterConfigViewportSize {
  const row = rasterRasterConfigGuardObject(value, at);
  return {
    width: rasterRasterConfigGuardNumber(row["width"], `${at}.width`),
    height: rasterRasterConfigGuardNumber(row["height"], `${at}.height`),
  };
}
