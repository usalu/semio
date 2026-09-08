/** 💡️ Semio text inference schema — word/mark census + distinct languages used. */

export interface SemioTextProfile {
  wordCount: number;
  charCount: number;
  runCount: number;
  markCount: number;
  languages: string[];
}

export interface SemioTextInference {
  /** @derived */
  profile: SemioTextProfile;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TextInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TextInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TextInferenceGuardRefusal(at, why);
};

type stdioSemioV1TextInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TextInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TextInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TextInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TextInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1TextInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TextInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TextInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TextInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TextInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TextInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1TextInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TextInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TextInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TextInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TextInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TextInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TextInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TextInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TextInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TextInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TextInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TextInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TextInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TextInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TextInferenceGuardNumber(value, at, bounds) : stdioSemioV1TextInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1TextInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TextInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TextInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TextInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioTextInference(value: unknown, at = "$"): SemioTextInference {
  const row = stdioSemioV1TextInferenceGuardObject(value, at);
  return {
    profile: parseSemioTextProfile(row["profile"], `${at}.profile`),
  };
}

export function parseSemioTextProfile(value: unknown, at = "$"): SemioTextProfile {
  const row = stdioSemioV1TextInferenceGuardObject(value, at);
  return {
    wordCount: stdioSemioV1TextInferenceGuardInteger(row["wordCount"], `${at}.wordCount`, {"minimum": 0}),
    charCount: stdioSemioV1TextInferenceGuardInteger(row["charCount"], `${at}.charCount`, {"minimum": 0}),
    runCount: stdioSemioV1TextInferenceGuardInteger(row["runCount"], `${at}.runCount`, {"minimum": 0}),
    markCount: stdioSemioV1TextInferenceGuardInteger(row["markCount"], `${at}.markCount`, {"minimum": 0}),
    languages: stdioSemioV1TextInferenceGuardArray(row["languages"], `${at}.languages`).map((item, index) => stdioSemioV1TextInferenceGuardString(item, `${at}.languages[${index}]`)),
  };
}
