/** 💡️ Semio animation inference schema — keyframe-derived playback duration. */

export interface SemioAnimationDuration {
  durationSeconds: number;
  timelineCount: number;
  channelCount: number;
  keyframeCount: number;
}

export interface SemioAnimationInference {
  /** @derived */
  duration: SemioAnimationDuration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AnimationInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AnimationInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AnimationInferenceGuardRefusal(at, why);
};

type stdioSemioV1AnimationInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AnimationInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AnimationInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AnimationInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AnimationInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1AnimationInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AnimationInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AnimationInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AnimationInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AnimationInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AnimationInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AnimationInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AnimationInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AnimationInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AnimationInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AnimationInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AnimationInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AnimationInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AnimationInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AnimationInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AnimationInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AnimationInferenceGuardNumber(value, at, bounds) : stdioSemioV1AnimationInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1AnimationInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AnimationInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AnimationInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AnimationInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAnimationInference(value: unknown, at = "$"): SemioAnimationInference {
  const row = stdioSemioV1AnimationInferenceGuardObject(value, at);
  return {
    duration: parseSemioAnimationDuration(row["duration"], `${at}.duration`),
  };
}

export function parseSemioAnimationDuration(value: unknown, at = "$"): SemioAnimationDuration {
  const row = stdioSemioV1AnimationInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioSemioV1AnimationInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    timelineCount: stdioSemioV1AnimationInferenceGuardInteger(row["timelineCount"], `${at}.timelineCount`, {"minimum": 0}),
    channelCount: stdioSemioV1AnimationInferenceGuardInteger(row["channelCount"], `${at}.channelCount`, {"minimum": 0}),
    keyframeCount: stdioSemioV1AnimationInferenceGuardInteger(row["keyframeCount"], `${at}.keyframeCount`, {"minimum": 0}),
  };
}
