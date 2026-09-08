/** 📝️ Text representation for `stdio.dwg` (diff). */
export type DwgDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1018AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1018AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1018AnyDiffTextGuardRefusal(at, why);
};

type stdioDwgAc1018AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1018AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1018AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1018AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1018AnyDiffTextGuardReject(at, "value is not an object");
export const stdioDwgAc1018AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1018AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1018AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1018AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1018AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1018AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioDwgAc1018AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1018AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1018AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1018AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1018AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1018AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1018AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1018AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1018AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1018AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1018AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1018AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1018AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1018AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1018AnyDiffTextGuardNumber(value, at, bounds) : stdioDwgAc1018AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioDwgAc1018AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1018AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1018AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1018AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDwgDiffText(value: unknown, at = "$"): DwgDiffText {
  return stdioDwgAc1018AnyDiffTextGuardObject(value, `${at}`);
}
