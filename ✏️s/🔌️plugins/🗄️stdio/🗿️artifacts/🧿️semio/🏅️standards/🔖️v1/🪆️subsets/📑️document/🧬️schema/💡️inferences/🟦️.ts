/** 💡️ Semio document inference schema — heading/word/block-derived document outline. */

export interface SemioDocumentHeadingEntry {
  level: number;
  text: string;
}

export interface SemioDocumentOutline {
  sectionOutline: SemioDocumentHeadingEntry[];
  blockCount: number;
  wordCount: number;
}

export interface SemioDocumentInference {
  /** @derived */
  outline: SemioDocumentOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DocumentInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DocumentInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DocumentInferenceGuardRefusal(at, why);
};

type stdioSemioV1DocumentInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DocumentInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DocumentInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DocumentInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DocumentInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1DocumentInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DocumentInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DocumentInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DocumentInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DocumentInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DocumentInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DocumentInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DocumentInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DocumentInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DocumentInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DocumentInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DocumentInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DocumentInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DocumentInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DocumentInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DocumentInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DocumentInferenceGuardNumber(value, at, bounds) : stdioSemioV1DocumentInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1DocumentInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DocumentInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DocumentInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DocumentInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioDocumentInference(value: unknown, at = "$"): SemioDocumentInference {
  const row = stdioSemioV1DocumentInferenceGuardObject(value, at);
  return {
    outline: parseSemioDocumentOutline(row["outline"], `${at}.outline`),
  };
}

export function parseSemioDocumentHeadingEntry(value: unknown, at = "$"): SemioDocumentHeadingEntry {
  const row = stdioSemioV1DocumentInferenceGuardObject(value, at);
  return {
    level: stdioSemioV1DocumentInferenceGuardInteger(row["level"], `${at}.level`, {"minimum": 0}),
    text: stdioSemioV1DocumentInferenceGuardString(row["text"], `${at}.text`),
  };
}

export function parseSemioDocumentOutline(value: unknown, at = "$"): SemioDocumentOutline {
  const row = stdioSemioV1DocumentInferenceGuardObject(value, at);
  return {
    sectionOutline: stdioSemioV1DocumentInferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => parseSemioDocumentHeadingEntry(item, `${at}.sectionOutline[${index}]`)),
    blockCount: stdioSemioV1DocumentInferenceGuardInteger(row["blockCount"], `${at}.blockCount`, {"minimum": 0}),
    wordCount: stdioSemioV1DocumentInferenceGuardInteger(row["wordCount"], `${at}.wordCount`, {"minimum": 0}),
  };
}
