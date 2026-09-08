/** 💡️ zip inference schema — real central-directory-style census over decompressed `entries`. */

export interface ZipEntries {
  entryCount: number;
  totalUncompressedSize: number;
  contentDigest: string;
}

export interface ZipInference {
  /** @derived */
  entries: ZipEntries;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioZip20BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioZip20BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioZip20BaseInferenceGuardRefusal(at, why);
};

type stdioZip20BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioZip20BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioZip20BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioZip20BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioZip20BaseInferenceGuardReject(at, "value is not an object");
export const stdioZip20BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioZip20BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioZip20BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioZip20BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioZip20BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioZip20BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioZip20BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioZip20BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioZip20BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioZip20BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioZip20BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioZip20BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioZip20BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioZip20BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioZip20BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioZip20BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioZip20BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioZip20BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioZip20BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioZip20BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioZip20BaseInferenceGuardNumber(value, at, bounds) : stdioZip20BaseInferenceGuardReject(at, "value is not an integer");
export const stdioZip20BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioZip20BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioZip20BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioZip20BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseZipInference(value: unknown, at = "$"): ZipInference {
  const row = stdioZip20BaseInferenceGuardObject(value, at);
  return {
    entries: parseZipEntries(row["entries"], `${at}.entries`),
  };
}

export function parseZipEntries(value: unknown, at = "$"): ZipEntries {
  const row = stdioZip20BaseInferenceGuardObject(value, at);
  return {
    entryCount: stdioZip20BaseInferenceGuardInteger(row["entryCount"], `${at}.entryCount`, {"minimum": 0}),
    totalUncompressedSize: stdioZip20BaseInferenceGuardInteger(row["totalUncompressedSize"], `${at}.totalUncompressedSize`, {"minimum": 0}),
    contentDigest: stdioZip20BaseInferenceGuardString(row["contentDigest"], `${at}.contentDigest`),
  };
}
