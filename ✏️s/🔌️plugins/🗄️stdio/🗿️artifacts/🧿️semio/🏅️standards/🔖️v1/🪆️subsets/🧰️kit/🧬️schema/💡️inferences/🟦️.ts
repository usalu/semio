/** 💡️ Semio kit inference schema — catalog census over the kit's own collections. */

export interface SemioKitEntries {
  typeCount: number;
  designCount: number;
  pieceCount: number;
  connectionCount: number;
  objectCount: number;
  modelCount: number;
  hasProperties: boolean;
  representationCount: number;
}

export interface SemioKitInference {
  /** @derived */
  entries: SemioKitEntries;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitInferenceGuardRefusal(at, why);
};

type stdioSemioV1KitInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1KitInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitInferenceGuardNumber(value, at, bounds) : stdioSemioV1KitInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1KitInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioKitInference(value: unknown, at = "$"): SemioKitInference {
  const row = stdioSemioV1KitInferenceGuardObject(value, at);
  return {
    entries: parseSemioKitEntries(row["entries"], `${at}.entries`),
  };
}

export function parseSemioKitEntries(value: unknown, at = "$"): SemioKitEntries {
  const row = stdioSemioV1KitInferenceGuardObject(value, at);
  return {
    typeCount: stdioSemioV1KitInferenceGuardInteger(row["typeCount"], `${at}.typeCount`, {"minimum": 0}),
    designCount: stdioSemioV1KitInferenceGuardInteger(row["designCount"], `${at}.designCount`, {"minimum": 0}),
    pieceCount: stdioSemioV1KitInferenceGuardInteger(row["pieceCount"], `${at}.pieceCount`, {"minimum": 0}),
    connectionCount: stdioSemioV1KitInferenceGuardInteger(row["connectionCount"], `${at}.connectionCount`, {"minimum": 0}),
    objectCount: stdioSemioV1KitInferenceGuardInteger(row["objectCount"], `${at}.objectCount`, {"minimum": 0}),
    modelCount: stdioSemioV1KitInferenceGuardInteger(row["modelCount"], `${at}.modelCount`, {"minimum": 0}),
    hasProperties: stdioSemioV1KitInferenceGuardBoolean(row["hasProperties"], `${at}.hasProperties`),
    representationCount: stdioSemioV1KitInferenceGuardInteger(row["representationCount"], `${at}.representationCount`, {"minimum": 0}),
  };
}
