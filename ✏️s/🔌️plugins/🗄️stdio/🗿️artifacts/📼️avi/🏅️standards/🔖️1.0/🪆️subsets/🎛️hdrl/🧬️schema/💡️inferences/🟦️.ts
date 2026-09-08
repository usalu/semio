/** 💡️ avi inference schema — real `avih` MainAVIHeader-derived playback duration. */

export interface AviDuration {
  durationSeconds: number;
  streamCount: number;
  totalFrames: number;
}

export interface AviInference {
  /** @derived */
  duration: AviDuration;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioAvi10HdrlInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioAvi10HdrlInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioAvi10HdrlInferenceGuardRefusal(at, why);
};

type stdioAvi10HdrlInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioAvi10HdrlInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioAvi10HdrlInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioAvi10HdrlInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioAvi10HdrlInferenceGuardReject(at, "value is not an object");
export const stdioAvi10HdrlInferenceGuardArray = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioAvi10HdrlInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioAvi10HdrlInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioAvi10HdrlInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioAvi10HdrlInferenceGuardString = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioAvi10HdrlInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioAvi10HdrlInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioAvi10HdrlInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioAvi10HdrlInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioAvi10HdrlInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioAvi10HdrlInferenceGuardReject(at, "value is not a boolean"));
export const stdioAvi10HdrlInferenceGuardNumber = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioAvi10HdrlInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioAvi10HdrlInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioAvi10HdrlInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioAvi10HdrlInferenceGuardInteger = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioAvi10HdrlInferenceGuardNumber(value, at, bounds) : stdioAvi10HdrlInferenceGuardReject(at, "value is not an integer");
export const stdioAvi10HdrlInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioAvi10HdrlInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioAvi10HdrlInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioAvi10HdrlInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseAviInference(value: unknown, at = "$"): AviInference {
  const row = stdioAvi10HdrlInferenceGuardObject(value, at);
  return {
    duration: parseAviDuration(row["duration"], `${at}.duration`),
  };
}

export function parseAviDuration(value: unknown, at = "$"): AviDuration {
  const row = stdioAvi10HdrlInferenceGuardObject(value, at);
  return {
    durationSeconds: stdioAvi10HdrlInferenceGuardNumber(row["durationSeconds"], `${at}.durationSeconds`, {"minimum": 0}),
    streamCount: stdioAvi10HdrlInferenceGuardInteger(row["streamCount"], `${at}.streamCount`, {"minimum": 0}),
    totalFrames: stdioAvi10HdrlInferenceGuardInteger(row["totalFrames"], `${at}.totalFrames`, {"minimum": 0}),
  };
}
