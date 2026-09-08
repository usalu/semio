/** 📝️ Text representation for `raster.raster.diff`. */
export type RasterDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterDiffTextGuardReject = (at: string, why: string): never => {
  throw new rasterRasterDiffTextGuardRefusal(at, why);
};

type rasterRasterDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterDiffTextGuardReject(at, "value is not an object");
export const rasterRasterDiffTextGuardArray = (value: unknown, at: string, bounds: rasterRasterDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterDiffTextGuardString = (value: unknown, at: string, bounds: rasterRasterDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterDiffTextGuardReject(at, "value is not a boolean"));
export const rasterRasterDiffTextGuardNumber = (value: unknown, at: string, bounds: rasterRasterDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterDiffTextGuardInteger = (value: unknown, at: string, bounds: rasterRasterDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterDiffTextGuardNumber(value, at, bounds) : rasterRasterDiffTextGuardReject(at, "value is not an integer");
export const rasterRasterDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterDiffText(value: unknown, at = "$"): RasterDiffText {
  return rasterRasterDiffTextGuardObject(value, `${at}`);
}
