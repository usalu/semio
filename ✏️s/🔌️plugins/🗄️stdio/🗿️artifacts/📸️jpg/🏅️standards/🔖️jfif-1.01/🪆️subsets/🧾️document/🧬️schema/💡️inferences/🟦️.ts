/** 💡️ Jpg inference schema — canonical raster geometry. */

export interface JpgDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface JpgInference {
  /** @derived */
  dimensions: JpgDimensions;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJpgJfif101DocumentInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJpgJfif101DocumentInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioJpgJfif101DocumentInferenceGuardRefusal(at, why);
};

type stdioJpgJfif101DocumentInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJpgJfif101DocumentInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJpgJfif101DocumentInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJpgJfif101DocumentInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJpgJfif101DocumentInferenceGuardReject(at, "value is not an object");
export const stdioJpgJfif101DocumentInferenceGuardArray = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJpgJfif101DocumentInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJpgJfif101DocumentInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJpgJfif101DocumentInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJpgJfif101DocumentInferenceGuardString = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJpgJfif101DocumentInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJpgJfif101DocumentInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJpgJfif101DocumentInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJpgJfif101DocumentInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJpgJfif101DocumentInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJpgJfif101DocumentInferenceGuardReject(at, "value is not a boolean"));
export const stdioJpgJfif101DocumentInferenceGuardNumber = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJpgJfif101DocumentInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJpgJfif101DocumentInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJpgJfif101DocumentInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJpgJfif101DocumentInferenceGuardInteger = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJpgJfif101DocumentInferenceGuardNumber(value, at, bounds) : stdioJpgJfif101DocumentInferenceGuardReject(at, "value is not an integer");
export const stdioJpgJfif101DocumentInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJpgJfif101DocumentInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJpgJfif101DocumentInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJpgJfif101DocumentInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJpgInference(value: unknown, at = "$"): JpgInference {
  const row = stdioJpgJfif101DocumentInferenceGuardObject(value, at);
  return {
    dimensions: parseJpgDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parseJpgDimensions(value: unknown, at = "$"): JpgDimensions {
  const row = stdioJpgJfif101DocumentInferenceGuardObject(value, at);
  return {
    width: stdioJpgJfif101DocumentInferenceGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioJpgJfif101DocumentInferenceGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioJpgJfif101DocumentInferenceGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0}),
    hasAlpha: stdioJpgJfif101DocumentInferenceGuardBoolean(row["hasAlpha"], `${at}.hasAlpha`),
    pixelCount: stdioJpgJfif101DocumentInferenceGuardInteger(row["pixelCount"], `${at}.pixelCount`, {"minimum": 0}),
  };
}
