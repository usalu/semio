/** 💡️ mp3 inference schema — real MPEG-1/2/2.5 Layer III frame-header-derived playback duration. */

export interface Mp3Duration {
  durationSeconds: number;
  frameCount: number;
  channelCount: number;
}

export interface Mp3Inference {
  /** @derived */
  duration: Mp3Duration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp3Mpeg1layer3AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp3Mpeg1layer3AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioMp3Mpeg1layer3AnyInferenceGuardRefusal(at, why);
};

type stdioMp3Mpeg1layer3AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp3Mpeg1layer3AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp3Mpeg1layer3AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp3Mpeg1layer3AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, "value is not an object");
export const stdioMp3Mpeg1layer3AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioMp3Mpeg1layer3AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp3Mpeg1layer3AnyInferenceGuardNumber(value, at, bounds) : stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, "value is not an integer");
export const stdioMp3Mpeg1layer3AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp3Mpeg1layer3AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp3Mpeg1layer3AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp3Inference(value: unknown, at = "$"): Mp3Inference {
  const row = stdioMp3Mpeg1layer3AnyInferenceGuardObject(value, at);
  return {
    duration: parseMp3Duration(row["duration"], `${at}.duration`),
  };
}

export function parseMp3Duration(value: unknown, at = "$"): Mp3Duration {
  const row = stdioMp3Mpeg1layer3AnyInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioMp3Mpeg1layer3AnyInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    frameCount: stdioMp3Mpeg1layer3AnyInferenceGuardInteger(row["frameCount"], `${at}.frameCount`, {"minimum": 0}),
    channelCount: stdioMp3Mpeg1layer3AnyInferenceGuardInteger(row["channelCount"], `${at}.channelCount`, {"minimum": 0}),
  };
}
