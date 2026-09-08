/** 📝️ Text representation for `raster.raster.inference`. */
export type RasterInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterInferenceTextGuardReject = (at: string, why: string): never => {
  throw new rasterRasterInferenceTextGuardRefusal(at, why);
};

type rasterRasterInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterInferenceTextGuardReject(at, "value is not an object");
export const rasterRasterInferenceTextGuardArray = (value: unknown, at: string, bounds: rasterRasterInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterInferenceTextGuardString = (value: unknown, at: string, bounds: rasterRasterInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterInferenceTextGuardReject(at, "value is not a boolean"));
export const rasterRasterInferenceTextGuardNumber = (value: unknown, at: string, bounds: rasterRasterInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterInferenceTextGuardInteger = (value: unknown, at: string, bounds: rasterRasterInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterInferenceTextGuardNumber(value, at, bounds) : rasterRasterInferenceTextGuardReject(at, "value is not an integer");
export const rasterRasterInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterInferenceText(value: unknown, at = "$"): RasterInferenceText {
  return rasterRasterInferenceTextGuardObject(value, `${at}`);
}
