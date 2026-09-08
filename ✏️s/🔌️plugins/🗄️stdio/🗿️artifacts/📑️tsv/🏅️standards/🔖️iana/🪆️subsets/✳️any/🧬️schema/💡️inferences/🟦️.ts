/** 💡️ Tsv inference schema — document outline (record/column counts). */

export interface TsvOutline {
  recordCount: number;
  columnCount: number;
}

export interface TsvInference {
  /** @derived */
  outline: TsvOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnyInferenceGuardRefusal(at, why);
};

type stdioTsvIanaAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnyInferenceGuardReject(at, "value is not an object");
export const stdioTsvIanaAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnyInferenceGuardNumber(value, at, bounds) : stdioTsvIanaAnyInferenceGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvInference(value: unknown, at = "$"): TsvInference {
  const row = stdioTsvIanaAnyInferenceGuardObject(value, at);
  return {
    outline: parseTsvOutline(row["outline"], `${at}.outline`),
  };
}

export function parseTsvOutline(value: unknown, at = "$"): TsvOutline {
  const row = stdioTsvIanaAnyInferenceGuardObject(value, at);
  return {
    recordCount: stdioTsvIanaAnyInferenceGuardInteger(row["recordCount"], `${at}.recordCount`),
    columnCount: stdioTsvIanaAnyInferenceGuardInteger(row["columnCount"], `${at}.columnCount`),
  };
}
