/** 💡️ Gif (87a) inference schema — logical screen geometry (never alpha: 87a has none). */

export interface GifDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface GifInference {
  /** @derived */
  dimensions: GifDimensions;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif87aAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif87aAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioGif87aAnyInferenceGuardRefusal(at, why);
};

type stdioGif87aAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif87aAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif87aAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif87aAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif87aAnyInferenceGuardReject(at, "value is not an object");
export const stdioGif87aAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif87aAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif87aAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif87aAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif87aAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif87aAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif87aAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif87aAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif87aAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif87aAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif87aAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioGif87aAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif87aAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif87aAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif87aAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif87aAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif87aAnyInferenceGuardNumber(value, at, bounds) : stdioGif87aAnyInferenceGuardReject(at, "value is not an integer");
export const stdioGif87aAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif87aAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif87aAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif87aAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifInference(value: unknown, at = "$"): GifInference {
  const row = stdioGif87aAnyInferenceGuardObject(value, at);
  return {
    dimensions: parseGifDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parseGifDimensions(value: unknown, at = "$"): GifDimensions {
  const row = stdioGif87aAnyInferenceGuardObject(value, at);
  return {
    width: stdioGif87aAnyInferenceGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioGif87aAnyInferenceGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioGif87aAnyInferenceGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0}),
    hasAlpha: stdioGif87aAnyInferenceGuardBoolean(row["hasAlpha"], `${at}.hasAlpha`),
    pixelCount: stdioGif87aAnyInferenceGuardInteger(row["pixelCount"], `${at}.pixelCount`, {"minimum": 0}),
  };
}
