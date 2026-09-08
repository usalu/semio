/** 📝️ Text representation for `trinity.jack.inference`. */
export type JackInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackInferenceTextGuardReject = (at: string, why: string): never => {
  throw new trinityJackInferenceTextGuardRefusal(at, why);
};

type trinityJackInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackInferenceTextGuardReject(at, "value is not an object");
export const trinityJackInferenceTextGuardArray = (value: unknown, at: string, bounds: trinityJackInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackInferenceTextGuardString = (value: unknown, at: string, bounds: trinityJackInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackInferenceTextGuardReject(at, "value is not a boolean"));
export const trinityJackInferenceTextGuardNumber = (value: unknown, at: string, bounds: trinityJackInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackInferenceTextGuardInteger = (value: unknown, at: string, bounds: trinityJackInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackInferenceTextGuardNumber(value, at, bounds) : trinityJackInferenceTextGuardReject(at, "value is not an integer");
export const trinityJackInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackInferenceText(value: unknown, at = "$"): JackInferenceText {
  return trinityJackInferenceTextGuardObject(value, `${at}`);
}
