/** 💡️ Semio table inference schema — dimensions + declared column-kind census. */

export interface SemioTableShape {
  columnCount: number;
  rowCount: number;
  nullColumnCount: number;
  boolColumnCount: number;
  intColumnCount: number;
  floatColumnCount: number;
  strColumnCount: number;
  bytesColumnCount: number;
}

export interface SemioTableInference {
  /** @derived */
  shape: SemioTableShape;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TableInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TableInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TableInferenceGuardRefusal(at, why);
};

type stdioSemioV1TableInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TableInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TableInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TableInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TableInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1TableInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TableInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TableInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TableInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TableInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TableInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1TableInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TableInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TableInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TableInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TableInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TableInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TableInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TableInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TableInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TableInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TableInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TableInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TableInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TableInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TableInferenceGuardNumber(value, at, bounds) : stdioSemioV1TableInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1TableInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TableInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TableInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TableInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioTableInference(value: unknown, at = "$"): SemioTableInference {
  const row = stdioSemioV1TableInferenceGuardObject(value, at);
  return {
    shape: parseSemioTableShape(row["shape"], `${at}.shape`),
  };
}

export function parseSemioTableShape(value: unknown, at = "$"): SemioTableShape {
  const row = stdioSemioV1TableInferenceGuardObject(value, at);
  return {
    columnCount: stdioSemioV1TableInferenceGuardInteger(row["columnCount"], `${at}.columnCount`, {"minimum": 0}),
    rowCount: stdioSemioV1TableInferenceGuardInteger(row["rowCount"], `${at}.rowCount`, {"minimum": 0}),
    nullColumnCount: stdioSemioV1TableInferenceGuardInteger(row["nullColumnCount"], `${at}.nullColumnCount`, {"minimum": 0}),
    boolColumnCount: stdioSemioV1TableInferenceGuardInteger(row["boolColumnCount"], `${at}.boolColumnCount`, {"minimum": 0}),
    intColumnCount: stdioSemioV1TableInferenceGuardInteger(row["intColumnCount"], `${at}.intColumnCount`, {"minimum": 0}),
    floatColumnCount: stdioSemioV1TableInferenceGuardInteger(row["floatColumnCount"], `${at}.floatColumnCount`, {"minimum": 0}),
    strColumnCount: stdioSemioV1TableInferenceGuardInteger(row["strColumnCount"], `${at}.strColumnCount`, {"minimum": 0}),
    bytesColumnCount: stdioSemioV1TableInferenceGuardInteger(row["bytesColumnCount"], `${at}.bytesColumnCount`, {"minimum": 0}),
  };
}
