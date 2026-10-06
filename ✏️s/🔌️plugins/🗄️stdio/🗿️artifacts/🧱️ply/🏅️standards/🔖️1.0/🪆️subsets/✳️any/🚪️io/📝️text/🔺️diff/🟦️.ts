/** 📝️ Text representation for `stdio.ply` (diff). */
export type PlyDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPly10AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPly10AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioPly10AnyDiffTextGuardRefusal(at, why);
};

type stdioPly10AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPly10AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPly10AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPly10AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPly10AnyDiffTextGuardReject(at, "value is not an object");
export const stdioPly10AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioPly10AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPly10AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPly10AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPly10AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPly10AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioPly10AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPly10AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPly10AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPly10AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPly10AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPly10AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPly10AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioPly10AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioPly10AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPly10AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPly10AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPly10AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPly10AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioPly10AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPly10AnyDiffTextGuardNumber(value, at, bounds) : stdioPly10AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioPly10AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPly10AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPly10AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPly10AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlyDiffText(value: unknown, at = "$"): PlyDiffText {
  return stdioPly10AnyDiffTextGuardObject(value, `${at}`);
}
