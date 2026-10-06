/** 📝️ Text representation for `stdio.bmp` (diff). */
export type BmpDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBmpV3AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBmpV3AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioBmpV3AnyDiffTextGuardRefusal(at, why);
};

type stdioBmpV3AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBmpV3AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBmpV3AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBmpV3AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBmpV3AnyDiffTextGuardReject(at, "value is not an object");
export const stdioBmpV3AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBmpV3AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBmpV3AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBmpV3AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBmpV3AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBmpV3AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBmpV3AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBmpV3AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBmpV3AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBmpV3AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBmpV3AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioBmpV3AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBmpV3AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBmpV3AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBmpV3AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBmpV3AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBmpV3AnyDiffTextGuardNumber(value, at, bounds) : stdioBmpV3AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioBmpV3AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBmpV3AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBmpV3AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBmpV3AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBmpDiffText(value: unknown, at = "$"): BmpDiffText {
  return stdioBmpV3AnyDiffTextGuardObject(value, `${at}`);
}
