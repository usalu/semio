/** 💡️ Semio video inference schema — real per-stream max-pts elapsed time. */

export interface SemioVideoDuration {
  durationSeconds: number;
  streamCount: number;
  sampleCount: number;
}

export interface SemioVideoInference {
  /** @derived */
  duration: SemioVideoDuration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1VideoInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1VideoInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1VideoInferenceGuardRefusal(at, why);
};

type stdioSemioV1VideoInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1VideoInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1VideoInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1VideoInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1VideoInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1VideoInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1VideoInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1VideoInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1VideoInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1VideoInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1VideoInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1VideoInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1VideoInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1VideoInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1VideoInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1VideoInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1VideoInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1VideoInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1VideoInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1VideoInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1VideoInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1VideoInferenceGuardNumber(value, at, bounds) : stdioSemioV1VideoInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1VideoInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1VideoInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1VideoInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1VideoInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioVideoInference(value: unknown, at = "$"): SemioVideoInference {
  const row = stdioSemioV1VideoInferenceGuardObject(value, at);
  return {
    duration: parseSemioVideoDuration(row["duration"], `${at}.duration`),
  };
}

export function parseSemioVideoDuration(value: unknown, at = "$"): SemioVideoDuration {
  const row = stdioSemioV1VideoInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioSemioV1VideoInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    streamCount: stdioSemioV1VideoInferenceGuardInteger(row["streamCount"], `${at}.streamCount`, {"minimum": 0}),
    sampleCount: stdioSemioV1VideoInferenceGuardInteger(row["sampleCount"], `${at}.sampleCount`, {"minimum": 0}),
  };
}
