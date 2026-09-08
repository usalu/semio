/** 💡️ Xlsx inference schema — document outline (sheet names/count, cell count). */

export interface XlsxOutline {
  sheetNames: string[];
  sheetCount: number;
  cellCount: number;
}

export interface XlsxInference {
  /** @derived */
  outline: XlsxOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXlsxEcma376BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXlsxEcma376BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioXlsxEcma376BaseInferenceGuardRefusal(at, why);
};

type stdioXlsxEcma376BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXlsxEcma376BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXlsxEcma376BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXlsxEcma376BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXlsxEcma376BaseInferenceGuardReject(at, "value is not an object");
export const stdioXlsxEcma376BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXlsxEcma376BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXlsxEcma376BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXlsxEcma376BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXlsxEcma376BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXlsxEcma376BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXlsxEcma376BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXlsxEcma376BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXlsxEcma376BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXlsxEcma376BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXlsxEcma376BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioXlsxEcma376BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXlsxEcma376BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXlsxEcma376BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXlsxEcma376BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXlsxEcma376BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXlsxEcma376BaseInferenceGuardNumber(value, at, bounds) : stdioXlsxEcma376BaseInferenceGuardReject(at, "value is not an integer");
export const stdioXlsxEcma376BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXlsxEcma376BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXlsxEcma376BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXlsxEcma376BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXlsxInference(value: unknown, at = "$"): XlsxInference {
  const row = stdioXlsxEcma376BaseInferenceGuardObject(value, at);
  return {
    outline: parseXlsxOutline(row["outline"], `${at}.outline`),
  };
}

export function parseXlsxOutline(value: unknown, at = "$"): XlsxOutline {
  const row = stdioXlsxEcma376BaseInferenceGuardObject(value, at);
  return {
    sheetNames: stdioXlsxEcma376BaseInferenceGuardArray(row["sheetNames"], `${at}.sheetNames`).map((item, index) => stdioXlsxEcma376BaseInferenceGuardString(item, `${at}.sheetNames[${index}]`)),
    sheetCount: stdioXlsxEcma376BaseInferenceGuardInteger(row["sheetCount"], `${at}.sheetCount`),
    cellCount: stdioXlsxEcma376BaseInferenceGuardInteger(row["cellCount"], `${at}.cellCount`),
  };
}
