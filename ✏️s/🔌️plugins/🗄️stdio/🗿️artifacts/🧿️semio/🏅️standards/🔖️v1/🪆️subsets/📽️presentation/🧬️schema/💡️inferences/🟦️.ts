/** 💡️ Semio presentation inference schema — heading outline + census over masters/layouts/slides. */

export interface SemioPresentationHeadingEntry {
  level: number;
  text: string;
}

export interface SemioPresentationOutline {
  sectionOutline: SemioPresentationHeadingEntry[];
  slideCount: number;
  shapeCount: number;
  blockCount: number;
  wordCount: number;
}

export interface SemioPresentationInference {
  /** @derived */
  outline: SemioPresentationOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1PresentationInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1PresentationInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1PresentationInferenceGuardRefusal(at, why);
};

type stdioSemioV1PresentationInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1PresentationInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1PresentationInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1PresentationInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1PresentationInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1PresentationInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1PresentationInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1PresentationInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1PresentationInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1PresentationInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1PresentationInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1PresentationInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1PresentationInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1PresentationInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1PresentationInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1PresentationInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1PresentationInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1PresentationInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1PresentationInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1PresentationInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1PresentationInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1PresentationInferenceGuardNumber(value, at, bounds) : stdioSemioV1PresentationInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1PresentationInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1PresentationInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1PresentationInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1PresentationInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioPresentationInference(value: unknown, at = "$"): SemioPresentationInference {
  const row = stdioSemioV1PresentationInferenceGuardObject(value, at);
  return {
    outline: parseSemioPresentationOutline(row["outline"], `${at}.outline`),
  };
}

export function parseSemioPresentationOutline(value: unknown, at = "$"): SemioPresentationOutline {
  const row = stdioSemioV1PresentationInferenceGuardObject(value, at);
  return {
    sectionOutline: stdioSemioV1PresentationInferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => parseSemioPresentationHeadingEntry(item, `${at}.sectionOutline[${index}]`)),
    slideCount: stdioSemioV1PresentationInferenceGuardInteger(row["slideCount"], `${at}.slideCount`, {"minimum": 0}),
    shapeCount: stdioSemioV1PresentationInferenceGuardInteger(row["shapeCount"], `${at}.shapeCount`, {"minimum": 0}),
    blockCount: stdioSemioV1PresentationInferenceGuardInteger(row["blockCount"], `${at}.blockCount`, {"minimum": 0}),
    wordCount: stdioSemioV1PresentationInferenceGuardInteger(row["wordCount"], `${at}.wordCount`, {"minimum": 0}),
  };
}

export function parseSemioPresentationHeadingEntry(value: unknown, at = "$"): SemioPresentationHeadingEntry {
  const row = stdioSemioV1PresentationInferenceGuardObject(value, at);
  return {
    level: stdioSemioV1PresentationInferenceGuardInteger(row["level"], `${at}.level`, {"minimum": 0}),
    text: stdioSemioV1PresentationInferenceGuardString(row["text"], `${at}.text`),
  };
}
