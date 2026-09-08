/** 💡️ Png inference schema — IHDR-derived raster geometry. */

export interface PngDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface PngInference {
  /** @derived */
  dimensions: PngDimensions;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPng12AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPng12AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioPng12AnyInferenceGuardRefusal(at, why);
};

type stdioPng12AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPng12AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPng12AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPng12AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPng12AnyInferenceGuardReject(at, "value is not an object");
export const stdioPng12AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioPng12AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPng12AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPng12AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPng12AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPng12AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioPng12AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPng12AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPng12AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPng12AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPng12AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPng12AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPng12AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioPng12AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioPng12AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPng12AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPng12AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPng12AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPng12AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioPng12AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPng12AnyInferenceGuardNumber(value, at, bounds) : stdioPng12AnyInferenceGuardReject(at, "value is not an integer");
export const stdioPng12AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPng12AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPng12AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPng12AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePngInference(value: unknown, at = "$"): PngInference {
  const row = stdioPng12AnyInferenceGuardObject(value, at);
  return {
    dimensions: parsePngDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parsePngDimensions(value: unknown, at = "$"): PngDimensions {
  const row = stdioPng12AnyInferenceGuardObject(value, at);
  return {
    width: stdioPng12AnyInferenceGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioPng12AnyInferenceGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioPng12AnyInferenceGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0}),
    hasAlpha: stdioPng12AnyInferenceGuardBoolean(row["hasAlpha"], `${at}.hasAlpha`),
    pixelCount: stdioPng12AnyInferenceGuardInteger(row["pixelCount"], `${at}.pixelCount`, {"minimum": 0}),
  };
}
