/** 📝️ Text representation for `s.stdio.mp3.inference`. */
export type Mp3InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp3Mpeg1layer3AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp3Mpeg1layer3AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioMp3Mpeg1layer3AnyInferenceTextGuardRefusal(at, why);
};

type stdioMp3Mpeg1layer3AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp3Mpeg1layer3AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp3Mpeg1layer3AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp3Mpeg1layer3AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioMp3Mpeg1layer3AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp3Mpeg1layer3AnyInferenceTextGuardNumber(value, at, bounds) : stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp3Mpeg1layer3AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp3Mpeg1layer3AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp3InferenceText(value: unknown, at = "$"): Mp3InferenceText {
  return stdioMp3Mpeg1layer3AnyInferenceTextGuardObject(value, `${at}`);
}
