/** 💡️ Gif (89a) inference schema — logical screen geometry (`hasAlpha` from any frame's GCE). */

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
export class stdioGif89aBaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif89aBaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioGif89aBaseInferenceGuardRefusal(at, why);
};

type stdioGif89aBaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif89aBaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif89aBaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif89aBaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif89aBaseInferenceGuardReject(at, "value is not an object");
export const stdioGif89aBaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif89aBaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif89aBaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif89aBaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif89aBaseInferenceGuardString = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif89aBaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif89aBaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif89aBaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif89aBaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif89aBaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif89aBaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioGif89aBaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif89aBaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif89aBaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif89aBaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif89aBaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif89aBaseInferenceGuardNumber(value, at, bounds) : stdioGif89aBaseInferenceGuardReject(at, "value is not an integer");
export const stdioGif89aBaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif89aBaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif89aBaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif89aBaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifInference(value: unknown, at = "$"): GifInference {
  const row = stdioGif89aBaseInferenceGuardObject(value, at);
  return {
    dimensions: parseGifDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parseGifDimensions(value: unknown, at = "$"): GifDimensions {
  const row = stdioGif89aBaseInferenceGuardObject(value, at);
  return {
    width: stdioGif89aBaseInferenceGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioGif89aBaseInferenceGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioGif89aBaseInferenceGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0}),
    hasAlpha: stdioGif89aBaseInferenceGuardBoolean(row["hasAlpha"], `${at}.hasAlpha`),
    pixelCount: stdioGif89aBaseInferenceGuardInteger(row["pixelCount"], `${at}.pixelCount`, {"minimum": 0}),
  };
}
