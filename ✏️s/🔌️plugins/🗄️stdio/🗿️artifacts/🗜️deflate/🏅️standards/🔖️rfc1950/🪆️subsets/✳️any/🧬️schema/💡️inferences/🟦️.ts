/** 💡️ deflate inference schema — real RFC1950 zlib header semantics (CMF window size, FLG.FLEVEL,
 * FDICT), not a forced multi-entry shape (RFC1950 wraps exactly one deflate stream). */

export interface DeflateWindow {
  windowSize: number;
  compressionLevelHint: string;
  hasPresetDictionary: boolean;
  payloadSize: number;
  contentDigest: string;
}

export interface DeflateInference {
  /** @derived */
  window: DeflateWindow;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDeflateRfc1950AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDeflateRfc1950AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioDeflateRfc1950AnyInferenceGuardRefusal(at, why);
};

type stdioDeflateRfc1950AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDeflateRfc1950AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDeflateRfc1950AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDeflateRfc1950AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDeflateRfc1950AnyInferenceGuardReject(at, "value is not an object");
export const stdioDeflateRfc1950AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDeflateRfc1950AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDeflateRfc1950AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDeflateRfc1950AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDeflateRfc1950AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDeflateRfc1950AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDeflateRfc1950AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDeflateRfc1950AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDeflateRfc1950AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDeflateRfc1950AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDeflateRfc1950AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioDeflateRfc1950AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDeflateRfc1950AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDeflateRfc1950AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDeflateRfc1950AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDeflateRfc1950AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDeflateRfc1950AnyInferenceGuardNumber(value, at, bounds) : stdioDeflateRfc1950AnyInferenceGuardReject(at, "value is not an integer");
export const stdioDeflateRfc1950AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDeflateRfc1950AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDeflateRfc1950AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDeflateRfc1950AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDeflateInference(value: unknown, at = "$"): DeflateInference {
  const row = stdioDeflateRfc1950AnyInferenceGuardObject(value, at);
  return {
    window: parseDeflateWindow(row["window"], `${at}.window`),
  };
}

export function parseDeflateWindow(value: unknown, at = "$"): DeflateWindow {
  const row = stdioDeflateRfc1950AnyInferenceGuardObject(value, at);
  return {
    windowSize: stdioDeflateRfc1950AnyInferenceGuardInteger(row["windowSize"], `${at}.windowSize`, {"minimum": 0}),
    compressionLevelHint: stdioDeflateRfc1950AnyInferenceGuardMember(row["compressionLevelHint"], `${at}.compressionLevelHint`, ["Fastest", "Fast", "Default", "Maximum"] as const),
    hasPresetDictionary: stdioDeflateRfc1950AnyInferenceGuardBoolean(row["hasPresetDictionary"], `${at}.hasPresetDictionary`),
    payloadSize: stdioDeflateRfc1950AnyInferenceGuardInteger(row["payloadSize"], `${at}.payloadSize`, {"minimum": 0}),
    contentDigest: stdioDeflateRfc1950AnyInferenceGuardString(row["contentDigest"], `${at}.contentDigest`),
  };
}
