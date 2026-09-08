/** 📝️ Text representation for `trinity.jack.mutations`. */
export type JackMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackMutationsTextGuardReject = (at: string, why: string): never => {
  throw new trinityJackMutationsTextGuardRefusal(at, why);
};

type trinityJackMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackMutationsTextGuardReject(at, "value is not an object");
export const trinityJackMutationsTextGuardArray = (value: unknown, at: string, bounds: trinityJackMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackMutationsTextGuardString = (value: unknown, at: string, bounds: trinityJackMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackMutationsTextGuardReject(at, "value is not a boolean"));
export const trinityJackMutationsTextGuardNumber = (value: unknown, at: string, bounds: trinityJackMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackMutationsTextGuardInteger = (value: unknown, at: string, bounds: trinityJackMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackMutationsTextGuardNumber(value, at, bounds) : trinityJackMutationsTextGuardReject(at, "value is not an integer");
export const trinityJackMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackMutationsText(value: unknown, at = "$"): JackMutationsText {
  return trinityJackMutationsTextGuardObject(value, `${at}`);
}
