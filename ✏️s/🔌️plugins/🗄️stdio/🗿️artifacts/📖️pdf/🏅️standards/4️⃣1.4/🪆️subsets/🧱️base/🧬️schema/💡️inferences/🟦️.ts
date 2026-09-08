/** 💡️ Pdf (1.4) inference schema — document outline (page count, word/char counts). */

export interface PdfOutline {
  pageCount: number;
  wordCount: number;
  charCount: number;
}

export interface PdfInference {
  /** @derived */
  outline: PdfOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf14BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf14BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioPdf14BaseInferenceGuardRefusal(at, why);
};

type stdioPdf14BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf14BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf14BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf14BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf14BaseInferenceGuardReject(at, "value is not an object");
export const stdioPdf14BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf14BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf14BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf14BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf14BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf14BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf14BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf14BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf14BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf14BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf14BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioPdf14BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf14BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf14BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf14BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf14BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf14BaseInferenceGuardNumber(value, at, bounds) : stdioPdf14BaseInferenceGuardReject(at, "value is not an integer");
export const stdioPdf14BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf14BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf14BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf14BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfInference(value: unknown, at = "$"): PdfInference {
  const row = stdioPdf14BaseInferenceGuardObject(value, at);
  return {
    outline: parsePdfOutline(row["outline"], `${at}.outline`),
  };
}

export function parsePdfOutline(value: unknown, at = "$"): PdfOutline {
  const row = stdioPdf14BaseInferenceGuardObject(value, at);
  return {
    pageCount: stdioPdf14BaseInferenceGuardInteger(row["pageCount"], `${at}.pageCount`),
    wordCount: stdioPdf14BaseInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
    charCount: stdioPdf14BaseInferenceGuardInteger(row["charCount"], `${at}.charCount`),
  };
}
