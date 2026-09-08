/** 💡️ Pdf (1.7) inference schema — document outline (page count, word count, title). */

export interface Pdf17Outline {
  pageCount: number;
  wordCount: number;
  title?: string;
}

export interface Pdf17Inference {
  /** @derived */
  outline: Pdf17Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf17BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf17BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioPdf17BaseInferenceGuardRefusal(at, why);
};

type stdioPdf17BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf17BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf17BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf17BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf17BaseInferenceGuardReject(at, "value is not an object");
export const stdioPdf17BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioPdf17BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf17BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf17BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf17BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf17BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioPdf17BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf17BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf17BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf17BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf17BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf17BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf17BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioPdf17BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioPdf17BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf17BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf17BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf17BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf17BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioPdf17BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf17BaseInferenceGuardNumber(value, at, bounds) : stdioPdf17BaseInferenceGuardReject(at, "value is not an integer");
export const stdioPdf17BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf17BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf17BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf17BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdf17Inference(value: unknown, at = "$"): Pdf17Inference {
  const row = stdioPdf17BaseInferenceGuardObject(value, at);
  return {
    outline: parsePdf17Outline(row["outline"], `${at}.outline`),
  };
}

export function parsePdf17Outline(value: unknown, at = "$"): Pdf17Outline {
  const row = stdioPdf17BaseInferenceGuardObject(value, at);
  return {
    pageCount: stdioPdf17BaseInferenceGuardInteger(row["pageCount"], `${at}.pageCount`),
    wordCount: stdioPdf17BaseInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
    title: row["title"] === undefined ? undefined : stdioPdf17BaseInferenceGuardString(row["title"], `${at}.title`),
  };
}
