/** 💡️ Tiff inference schema — IFD 0 baseline-tag-derived raster geometry. */

export interface TiffDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface TiffInference {
  /** @derived */
  dimensions: TiffDimensions;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentInferenceGuardRefusal(at, why);
};

type stdioTiff60DocumentInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentInferenceGuardReject(at, "value is not an object");
export const stdioTiff60DocumentInferenceGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentInferenceGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentInferenceGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentInferenceGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentInferenceGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentInferenceGuardNumber(value, at, bounds) : stdioTiff60DocumentInferenceGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffInference(value: unknown, at = "$"): TiffInference {
  const row = stdioTiff60DocumentInferenceGuardObject(value, at);
  return {
    dimensions: parseTiffDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parseTiffDimensions(value: unknown, at = "$"): TiffDimensions {
  const row = stdioTiff60DocumentInferenceGuardObject(value, at);
  return {
    width: stdioTiff60DocumentInferenceGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioTiff60DocumentInferenceGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioTiff60DocumentInferenceGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0}),
    hasAlpha: stdioTiff60DocumentInferenceGuardBoolean(row["hasAlpha"], `${at}.hasAlpha`),
    pixelCount: stdioTiff60DocumentInferenceGuardInteger(row["pixelCount"], `${at}.pixelCount`, {"minimum": 0}),
  };
}
