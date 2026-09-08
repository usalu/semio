/** 💡️ Docx inference schema — document outline (paragraph/table/word counts). */

export interface DocxOutline {
  paragraphCount: number;
  tableCount: number;
  wordCount: number;
}

export interface DocxInference {
  /** @derived */
  outline: DocxOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDocxEcma376BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDocxEcma376BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioDocxEcma376BaseInferenceGuardRefusal(at, why);
};

type stdioDocxEcma376BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDocxEcma376BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDocxEcma376BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDocxEcma376BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDocxEcma376BaseInferenceGuardReject(at, "value is not an object");
export const stdioDocxEcma376BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDocxEcma376BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDocxEcma376BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDocxEcma376BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDocxEcma376BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDocxEcma376BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDocxEcma376BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDocxEcma376BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDocxEcma376BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDocxEcma376BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDocxEcma376BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioDocxEcma376BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDocxEcma376BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDocxEcma376BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDocxEcma376BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDocxEcma376BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDocxEcma376BaseInferenceGuardNumber(value, at, bounds) : stdioDocxEcma376BaseInferenceGuardReject(at, "value is not an integer");
export const stdioDocxEcma376BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDocxEcma376BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDocxEcma376BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDocxEcma376BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDocxInference(value: unknown, at = "$"): DocxInference {
  const row = stdioDocxEcma376BaseInferenceGuardObject(value, at);
  return {
    outline: parseDocxOutline(row["outline"], `${at}.outline`),
  };
}

export function parseDocxOutline(value: unknown, at = "$"): DocxOutline {
  const row = stdioDocxEcma376BaseInferenceGuardObject(value, at);
  return {
    paragraphCount: stdioDocxEcma376BaseInferenceGuardInteger(row["paragraphCount"], `${at}.paragraphCount`),
    tableCount: stdioDocxEcma376BaseInferenceGuardInteger(row["tableCount"], `${at}.tableCount`),
    wordCount: stdioDocxEcma376BaseInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
  };
}
