/** 💡️ binary inference schema — a raw opaque byte blob's honest real extent (length, emptiness,
 * content digest); deliberately not a fabricated `entries` shape. */

export interface BinaryExtent {
  byteLength: number;
  isEmpty: boolean;
  contentDigest: string;
}

export interface BinaryInference {
  /** @derived */
  extent: BinaryExtent;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBinaryRawAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBinaryRawAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioBinaryRawAnyInferenceGuardRefusal(at, why);
};

type stdioBinaryRawAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBinaryRawAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBinaryRawAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBinaryRawAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBinaryRawAnyInferenceGuardReject(at, "value is not an object");
export const stdioBinaryRawAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBinaryRawAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBinaryRawAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBinaryRawAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBinaryRawAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBinaryRawAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBinaryRawAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBinaryRawAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBinaryRawAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBinaryRawAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBinaryRawAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioBinaryRawAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBinaryRawAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBinaryRawAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBinaryRawAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBinaryRawAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBinaryRawAnyInferenceGuardNumber(value, at, bounds) : stdioBinaryRawAnyInferenceGuardReject(at, "value is not an integer");
export const stdioBinaryRawAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBinaryRawAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBinaryRawAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBinaryRawAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBinaryInference(value: unknown, at = "$"): BinaryInference {
  const row = stdioBinaryRawAnyInferenceGuardObject(value, at);
  return {
    extent: parseBinaryExtent(row["extent"], `${at}.extent`),
  };
}

export function parseBinaryExtent(value: unknown, at = "$"): BinaryExtent {
  const row = stdioBinaryRawAnyInferenceGuardObject(value, at);
  return {
    byteLength: stdioBinaryRawAnyInferenceGuardInteger(row["byteLength"], `${at}.byteLength`, {"minimum": 0}),
    isEmpty: stdioBinaryRawAnyInferenceGuardBoolean(row["isEmpty"], `${at}.isEmpty`),
    contentDigest: stdioBinaryRawAnyInferenceGuardString(row["contentDigest"], `${at}.contentDigest`),
  };
}
