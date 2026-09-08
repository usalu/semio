/** 💡️ wav inference schema — real RIFF/WAVE `fmt ` + decoded `data` sample-count playback duration. */

export interface WavDuration {
  durationSeconds: number;
  frameCount: number;
  bitsPerSample: number;
}

export interface WavInference {
  /** @derived */
  duration: WavDuration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioWavRiffpcmAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioWavRiffpcmAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioWavRiffpcmAnyInferenceGuardRefusal(at, why);
};

type stdioWavRiffpcmAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioWavRiffpcmAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioWavRiffpcmAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioWavRiffpcmAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioWavRiffpcmAnyInferenceGuardReject(at, "value is not an object");
export const stdioWavRiffpcmAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioWavRiffpcmAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioWavRiffpcmAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioWavRiffpcmAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioWavRiffpcmAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioWavRiffpcmAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioWavRiffpcmAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioWavRiffpcmAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioWavRiffpcmAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioWavRiffpcmAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioWavRiffpcmAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioWavRiffpcmAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioWavRiffpcmAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioWavRiffpcmAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioWavRiffpcmAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioWavRiffpcmAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioWavRiffpcmAnyInferenceGuardNumber(value, at, bounds) : stdioWavRiffpcmAnyInferenceGuardReject(at, "value is not an integer");
export const stdioWavRiffpcmAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioWavRiffpcmAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioWavRiffpcmAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioWavRiffpcmAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWavInference(value: unknown, at = "$"): WavInference {
  const row = stdioWavRiffpcmAnyInferenceGuardObject(value, at);
  return {
    duration: parseWavDuration(row["duration"], `${at}.duration`),
  };
}

export function parseWavDuration(value: unknown, at = "$"): WavDuration {
  const row = stdioWavRiffpcmAnyInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioWavRiffpcmAnyInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    frameCount: stdioWavRiffpcmAnyInferenceGuardInteger(row["frameCount"], `${at}.frameCount`, {"minimum": 0}),
    bitsPerSample: stdioWavRiffpcmAnyInferenceGuardInteger(row["bitsPerSample"], `${at}.bitsPerSample`, {"minimum": 0}),
  };
}
