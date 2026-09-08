/** 🧬️ RasterPresence */
export interface RasterPresenceCamera {
  /** @state presence */
  x: number;
  /** @state presence */
  y: number;
  /** @state presence */
  zoom: number;
}

export interface RasterPresence {
  /** @state presence */
  brushSize: number;
  /** @state presence */
  brushOpacity: number;
  /** @state presence */
  camera: RasterPresenceCamera;
  /** @state presence */
  activeUtilityId: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterPresenceGuardReject = (at: string, why: string): never => {
  throw new rasterRasterPresenceGuardRefusal(at, why);
};

type rasterRasterPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterPresenceGuardReject(at, "value is not an object");
export const rasterRasterPresenceGuardArray = (value: unknown, at: string, bounds: rasterRasterPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterPresenceGuardString = (value: unknown, at: string, bounds: rasterRasterPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterPresenceGuardReject(at, "value is not a boolean"));
export const rasterRasterPresenceGuardNumber = (value: unknown, at: string, bounds: rasterRasterPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterPresenceGuardInteger = (value: unknown, at: string, bounds: rasterRasterPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterPresenceGuardNumber(value, at, bounds) : rasterRasterPresenceGuardReject(at, "value is not an integer");
export const rasterRasterPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterPresence(value: unknown, at = "$"): RasterPresence {
  const row = rasterRasterPresenceGuardObject(value, at);
  return {
    brushSize: rasterRasterPresenceGuardNumber(row["brushSize"], `${at}.brushSize`),
    brushOpacity: rasterRasterPresenceGuardNumber(row["brushOpacity"], `${at}.brushOpacity`),
    camera: parseRasterCamera(row["camera"], `${at}.camera`),
    activeUtilityId: rasterRasterPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
  };
}

export interface RasterCamera {
  readonly x: number;
  readonly y: number;
  readonly zoom: number;
}

export function parseRasterCamera(value: unknown, at = "$"): RasterCamera {
  const row = rasterRasterPresenceGuardObject(value, at);
  return {
    x: rasterRasterPresenceGuardNumber(row["x"], `${at}.x`),
    y: rasterRasterPresenceGuardNumber(row["y"], `${at}.y`),
    zoom: rasterRasterPresenceGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
