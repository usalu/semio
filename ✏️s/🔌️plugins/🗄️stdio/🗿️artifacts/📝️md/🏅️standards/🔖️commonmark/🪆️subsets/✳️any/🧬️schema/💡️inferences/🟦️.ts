/** 💡️ Md inference schema — document outline (heading outline, block/word counts). */

export interface MdHeadingEntry {
  level: number;
  text: string;
}

export interface MdOutline {
  sectionOutline: MdHeadingEntry[];
  blockCount: number;
  wordCount: number;
}

export interface MdInference {
  /** @derived */
  outline: MdOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnyInferenceGuardRefusal(at, why);
};

type stdioMdCommonmarkAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnyInferenceGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnyInferenceGuardNumber(value, at, bounds) : stdioMdCommonmarkAnyInferenceGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdInference(value: unknown, at = "$"): MdInference {
  const row = stdioMdCommonmarkAnyInferenceGuardObject(value, at);
  return {
    outline: parseMdOutline(row["outline"], `${at}.outline`),
  };
}

export function parseMdHeadingEntry(value: unknown, at = "$"): MdHeadingEntry {
  const row = stdioMdCommonmarkAnyInferenceGuardObject(value, at);
  return {
    level: stdioMdCommonmarkAnyInferenceGuardInteger(row["level"], `${at}.level`),
    text: stdioMdCommonmarkAnyInferenceGuardString(row["text"], `${at}.text`),
  };
}

export function parseMdOutline(value: unknown, at = "$"): MdOutline {
  const row = stdioMdCommonmarkAnyInferenceGuardObject(value, at);
  return {
    sectionOutline: stdioMdCommonmarkAnyInferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => parseMdHeadingEntry(item, `${at}.sectionOutline[${index}]`)),
    blockCount: stdioMdCommonmarkAnyInferenceGuardInteger(row["blockCount"], `${at}.blockCount`),
    wordCount: stdioMdCommonmarkAnyInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
  };
}
