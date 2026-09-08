/** 💡️ mp4 inference schema — real ISO-BMFF `stts`-derived (per-track sample-table) duration. */

export interface Mp4Duration {
  durationSeconds: number;
  trackCount: number;
  sampleCount: number;
}

export interface Mp4Inference {
  /** @derived */
  duration: Mp4Duration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp4IsobmffAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp4IsobmffAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioMp4IsobmffAnyInferenceGuardRefusal(at, why);
};

type stdioMp4IsobmffAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp4IsobmffAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp4IsobmffAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp4IsobmffAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp4IsobmffAnyInferenceGuardReject(at, "value is not an object");
export const stdioMp4IsobmffAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp4IsobmffAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp4IsobmffAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp4IsobmffAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp4IsobmffAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp4IsobmffAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp4IsobmffAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp4IsobmffAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp4IsobmffAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp4IsobmffAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp4IsobmffAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioMp4IsobmffAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp4IsobmffAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp4IsobmffAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp4IsobmffAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp4IsobmffAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp4IsobmffAnyInferenceGuardNumber(value, at, bounds) : stdioMp4IsobmffAnyInferenceGuardReject(at, "value is not an integer");
export const stdioMp4IsobmffAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp4IsobmffAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp4IsobmffAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp4IsobmffAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp4Inference(value: unknown, at = "$"): Mp4Inference {
  const row = stdioMp4IsobmffAnyInferenceGuardObject(value, at);
  return {
    duration: parseMp4Duration(row["duration"], `${at}.duration`),
  };
}

export function parseMp4Duration(value: unknown, at = "$"): Mp4Duration {
  const row = stdioMp4IsobmffAnyInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioMp4IsobmffAnyInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    trackCount: stdioMp4IsobmffAnyInferenceGuardInteger(row["trackCount"], `${at}.trackCount`, {"minimum": 0}),
    sampleCount: stdioMp4IsobmffAnyInferenceGuardInteger(row["sampleCount"], `${at}.sampleCount`, {"minimum": 0}),
  };
}
