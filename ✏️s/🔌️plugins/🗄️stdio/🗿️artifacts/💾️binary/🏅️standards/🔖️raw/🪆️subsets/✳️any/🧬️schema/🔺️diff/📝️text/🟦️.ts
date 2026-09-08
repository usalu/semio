/** 📝️ Text representation for `stdio.binary` (diff). */
export type BinaryDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBinaryRawAnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBinaryRawAnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioBinaryRawAnyDiffTextGuardRefusal(at, why);
};

type stdioBinaryRawAnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBinaryRawAnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBinaryRawAnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBinaryRawAnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBinaryRawAnyDiffTextGuardReject(at, "value is not an object");
export const stdioBinaryRawAnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioBinaryRawAnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBinaryRawAnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBinaryRawAnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBinaryRawAnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBinaryRawAnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioBinaryRawAnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBinaryRawAnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBinaryRawAnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBinaryRawAnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBinaryRawAnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBinaryRawAnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBinaryRawAnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioBinaryRawAnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioBinaryRawAnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBinaryRawAnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBinaryRawAnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBinaryRawAnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBinaryRawAnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioBinaryRawAnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBinaryRawAnyDiffTextGuardNumber(value, at, bounds) : stdioBinaryRawAnyDiffTextGuardReject(at, "value is not an integer");
export const stdioBinaryRawAnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBinaryRawAnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBinaryRawAnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBinaryRawAnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBinaryDiffText(value: unknown, at = "$"): BinaryDiffText {
  return stdioBinaryRawAnyDiffTextGuardObject(value, `${at}`);
}
