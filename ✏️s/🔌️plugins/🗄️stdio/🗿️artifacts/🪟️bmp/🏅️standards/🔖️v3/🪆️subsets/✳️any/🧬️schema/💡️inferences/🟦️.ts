/** 💡️ Bmp inference schema — BITMAPINFOHEADER-derived raster geometry. */

export interface BmpDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface BmpInference {
  /** @derived */
  dimensions: BmpDimensions;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBmpV3AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBmpV3AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioBmpV3AnyInferenceGuardRefusal(at, why);
};

type stdioBmpV3AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBmpV3AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBmpV3AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBmpV3AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBmpV3AnyInferenceGuardReject(at, "value is not an object");
export const stdioBmpV3AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBmpV3AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBmpV3AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBmpV3AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBmpV3AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBmpV3AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBmpV3AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBmpV3AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBmpV3AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBmpV3AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBmpV3AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioBmpV3AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBmpV3AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBmpV3AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBmpV3AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBmpV3AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBmpV3AnyInferenceGuardNumber(value, at, bounds) : stdioBmpV3AnyInferenceGuardReject(at, "value is not an integer");
export const stdioBmpV3AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBmpV3AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBmpV3AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBmpV3AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBmpInference(value: unknown, at = "$"): BmpInference {
  const row = stdioBmpV3AnyInferenceGuardObject(value, at);
  return {
    dimensions: parseBmpDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parseBmpDimensions(value: unknown, at = "$"): BmpDimensions {
  const row = stdioBmpV3AnyInferenceGuardObject(value, at);
  return {
    width: stdioBmpV3AnyInferenceGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioBmpV3AnyInferenceGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioBmpV3AnyInferenceGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0}),
    hasAlpha: stdioBmpV3AnyInferenceGuardBoolean(row["hasAlpha"], `${at}.hasAlpha`),
    pixelCount: stdioBmpV3AnyInferenceGuardInteger(row["pixelCount"], `${at}.pixelCount`, {"minimum": 0}),
  };
}
