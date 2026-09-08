/** 📝️ Text representation for `raster.raster.mutations`. */
export type RasterMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterMutationsTextGuardReject = (at: string, why: string): never => {
  throw new rasterRasterMutationsTextGuardRefusal(at, why);
};

type rasterRasterMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterMutationsTextGuardReject(at, "value is not an object");
export const rasterRasterMutationsTextGuardArray = (value: unknown, at: string, bounds: rasterRasterMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterMutationsTextGuardString = (value: unknown, at: string, bounds: rasterRasterMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterMutationsTextGuardReject(at, "value is not a boolean"));
export const rasterRasterMutationsTextGuardNumber = (value: unknown, at: string, bounds: rasterRasterMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterMutationsTextGuardInteger = (value: unknown, at: string, bounds: rasterRasterMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterMutationsTextGuardNumber(value, at, bounds) : rasterRasterMutationsTextGuardReject(at, "value is not an integer");
export const rasterRasterMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterMutationsText(value: unknown, at = "$"): RasterMutationsText {
  return rasterRasterMutationsTextGuardObject(value, `${at}`);
}
