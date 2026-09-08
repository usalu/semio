/** 💡️ Semio value inference schema — real variant census + max depth over the value graph. */

export interface SemioValueCensus {
  nullCount: number;
  boolCount: number;
  intCount: number;
  floatCount: number;
  strCount: number;
  bytesCount: number;
  listCount: number;
  mapCount: number;
  refCount: number;
  nodeCount: number;
  maxDepth: number;
}

export interface SemioValueInference {
  /** @derived */
  census: SemioValueCensus;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ValueInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ValueInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ValueInferenceGuardRefusal(at, why);
};

type stdioSemioV1ValueInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ValueInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ValueInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ValueInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ValueInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1ValueInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ValueInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ValueInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ValueInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ValueInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ValueInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ValueInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ValueInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ValueInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ValueInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ValueInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ValueInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ValueInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ValueInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ValueInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ValueInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ValueInferenceGuardNumber(value, at, bounds) : stdioSemioV1ValueInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1ValueInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ValueInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ValueInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ValueInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioValueInference(value: unknown, at = "$"): SemioValueInference {
  const row = stdioSemioV1ValueInferenceGuardObject(value, at);
  return {
    census: parseSemioValueCensus(row["census"], `${at}.census`),
  };
}

export function parseSemioValueCensus(value: unknown, at = "$"): SemioValueCensus {
  const row = stdioSemioV1ValueInferenceGuardObject(value, at);
  return {
    nullCount: stdioSemioV1ValueInferenceGuardInteger(row["nullCount"], `${at}.nullCount`, {"minimum": 0}),
    boolCount: stdioSemioV1ValueInferenceGuardInteger(row["boolCount"], `${at}.boolCount`, {"minimum": 0}),
    intCount: stdioSemioV1ValueInferenceGuardInteger(row["intCount"], `${at}.intCount`, {"minimum": 0}),
    floatCount: stdioSemioV1ValueInferenceGuardInteger(row["floatCount"], `${at}.floatCount`, {"minimum": 0}),
    strCount: stdioSemioV1ValueInferenceGuardInteger(row["strCount"], `${at}.strCount`, {"minimum": 0}),
    bytesCount: stdioSemioV1ValueInferenceGuardInteger(row["bytesCount"], `${at}.bytesCount`, {"minimum": 0}),
    listCount: stdioSemioV1ValueInferenceGuardInteger(row["listCount"], `${at}.listCount`, {"minimum": 0}),
    mapCount: stdioSemioV1ValueInferenceGuardInteger(row["mapCount"], `${at}.mapCount`, {"minimum": 0}),
    refCount: stdioSemioV1ValueInferenceGuardInteger(row["refCount"], `${at}.refCount`, {"minimum": 0}),
    nodeCount: stdioSemioV1ValueInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    maxDepth: stdioSemioV1ValueInferenceGuardInteger(row["maxDepth"], `${at}.maxDepth`, {"minimum": 0}),
  };
}
