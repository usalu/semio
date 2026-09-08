/** 💡️ Semio audio inference schema — sample-count-derived playback duration. */

export interface SemioAudioDuration {
  durationSeconds: number;
  sampleCount: number;
  channelCount: number;
}

export interface SemioAudioInference {
  /** @derived */
  duration: SemioAudioDuration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AudioInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AudioInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AudioInferenceGuardRefusal(at, why);
};

type stdioSemioV1AudioInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AudioInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AudioInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AudioInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AudioInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1AudioInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AudioInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AudioInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AudioInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AudioInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AudioInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AudioInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AudioInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AudioInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AudioInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AudioInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AudioInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AudioInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AudioInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AudioInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AudioInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AudioInferenceGuardNumber(value, at, bounds) : stdioSemioV1AudioInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1AudioInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AudioInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AudioInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AudioInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAudioInference(value: unknown, at = "$"): SemioAudioInference {
  const row = stdioSemioV1AudioInferenceGuardObject(value, at);
  return {
    duration: parseSemioAudioDuration(row["duration"], `${at}.duration`),
  };
}

export function parseSemioAudioDuration(value: unknown, at = "$"): SemioAudioDuration {
  const row = stdioSemioV1AudioInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioSemioV1AudioInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    sampleCount: stdioSemioV1AudioInferenceGuardInteger(row["sampleCount"], `${at}.sampleCount`, {"minimum": 0}),
    channelCount: stdioSemioV1AudioInferenceGuardInteger(row["channelCount"], `${at}.channelCount`, {"minimum": 0}),
  };
}
