/** 📝️ Text representation for `space.home.inference`. */
export type SHomeInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeInferenceTextGuardReject = (at: string, why: string): never => {
  throw new spaceHomeInferenceTextGuardRefusal(at, why);
};

type spaceHomeInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeInferenceTextGuardReject(at, "value is not an object");
export const spaceHomeInferenceTextGuardArray = (value: unknown, at: string, bounds: spaceHomeInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeInferenceTextGuardString = (value: unknown, at: string, bounds: spaceHomeInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeInferenceTextGuardReject(at, "value is not a boolean"));
export const spaceHomeInferenceTextGuardNumber = (value: unknown, at: string, bounds: spaceHomeInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeInferenceTextGuardInteger = (value: unknown, at: string, bounds: spaceHomeInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeInferenceTextGuardNumber(value, at, bounds) : spaceHomeInferenceTextGuardReject(at, "value is not an integer");
export const spaceHomeInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeInferenceText(value: unknown, at = "$"): SHomeInferenceText {
  return spaceHomeInferenceTextGuardObject(value, `${at}`);
}
