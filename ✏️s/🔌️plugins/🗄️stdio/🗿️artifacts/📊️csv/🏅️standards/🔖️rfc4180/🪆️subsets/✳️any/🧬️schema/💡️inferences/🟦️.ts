/** 💡️ Csv inference schema — document outline (record/column counts + hasHeader). */

export interface CsvOutline {
  recordCount: number;
  columnCount: number;
  hasHeader: boolean;
}

export interface CsvInference {
  /** @derived */
  outline: CsvOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioCsvRfc4180AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioCsvRfc4180AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioCsvRfc4180AnyInferenceGuardRefusal(at, why);
};

type stdioCsvRfc4180AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioCsvRfc4180AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioCsvRfc4180AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioCsvRfc4180AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioCsvRfc4180AnyInferenceGuardReject(at, "value is not an object");
export const stdioCsvRfc4180AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioCsvRfc4180AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioCsvRfc4180AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioCsvRfc4180AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioCsvRfc4180AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioCsvRfc4180AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioCsvRfc4180AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioCsvRfc4180AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioCsvRfc4180AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioCsvRfc4180AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioCsvRfc4180AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioCsvRfc4180AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioCsvRfc4180AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioCsvRfc4180AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioCsvRfc4180AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioCsvRfc4180AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioCsvRfc4180AnyInferenceGuardNumber(value, at, bounds) : stdioCsvRfc4180AnyInferenceGuardReject(at, "value is not an integer");
export const stdioCsvRfc4180AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioCsvRfc4180AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioCsvRfc4180AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioCsvRfc4180AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCsvInference(value: unknown, at = "$"): CsvInference {
  const row = stdioCsvRfc4180AnyInferenceGuardObject(value, at);
  return {
    outline: parseCsvOutline(row["outline"], `${at}.outline`),
  };
}

export function parseCsvOutline(value: unknown, at = "$"): CsvOutline {
  const row = stdioCsvRfc4180AnyInferenceGuardObject(value, at);
  return {
    recordCount: stdioCsvRfc4180AnyInferenceGuardInteger(row["recordCount"], `${at}.recordCount`),
    columnCount: stdioCsvRfc4180AnyInferenceGuardInteger(row["columnCount"], `${at}.columnCount`),
    hasHeader: stdioCsvRfc4180AnyInferenceGuardBoolean(row["hasHeader"], `${at}.hasHeader`),
  };
}
