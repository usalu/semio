/** 💡️ Txt inference schema — document outline (line/word/char counts). */

export interface TxtOutline {
  lineCount: number;
  wordCount: number;
  charCount: number;
}

export interface TxtInference {
  /** @derived */
  outline: TxtOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTxtUtf8AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTxtUtf8AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioTxtUtf8AnyInferenceGuardRefusal(at, why);
};

type stdioTxtUtf8AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTxtUtf8AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTxtUtf8AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTxtUtf8AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTxtUtf8AnyInferenceGuardReject(at, "value is not an object");
export const stdioTxtUtf8AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTxtUtf8AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTxtUtf8AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTxtUtf8AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTxtUtf8AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTxtUtf8AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTxtUtf8AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTxtUtf8AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTxtUtf8AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTxtUtf8AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTxtUtf8AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioTxtUtf8AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTxtUtf8AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTxtUtf8AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTxtUtf8AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTxtUtf8AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTxtUtf8AnyInferenceGuardNumber(value, at, bounds) : stdioTxtUtf8AnyInferenceGuardReject(at, "value is not an integer");
export const stdioTxtUtf8AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTxtUtf8AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTxtUtf8AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTxtUtf8AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTxtInference(value: unknown, at = "$"): TxtInference {
  const row = stdioTxtUtf8AnyInferenceGuardObject(value, at);
  return {
    outline: parseTxtOutline(row["outline"], `${at}.outline`),
  };
}

export function parseTxtOutline(value: unknown, at = "$"): TxtOutline {
  const row = stdioTxtUtf8AnyInferenceGuardObject(value, at);
  return {
    lineCount: stdioTxtUtf8AnyInferenceGuardInteger(row["lineCount"], `${at}.lineCount`),
    wordCount: stdioTxtUtf8AnyInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
    charCount: stdioTxtUtf8AnyInferenceGuardInteger(row["charCount"], `${at}.charCount`),
  };
}
