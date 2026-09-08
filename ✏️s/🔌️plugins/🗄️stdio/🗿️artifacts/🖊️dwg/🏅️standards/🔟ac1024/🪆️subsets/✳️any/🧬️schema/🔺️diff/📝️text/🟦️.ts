/** 📝️ Text representation for `stdio.dwg` (diff). */
export type DwgDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1024AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1024AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1024AnyDiffTextGuardRefusal(at, why);
};

type stdioDwgAc1024AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1024AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1024AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1024AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1024AnyDiffTextGuardReject(at, "value is not an object");
export const stdioDwgAc1024AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1024AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1024AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1024AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1024AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1024AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioDwgAc1024AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1024AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1024AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1024AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1024AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1024AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1024AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1024AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1024AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1024AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1024AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1024AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1024AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1024AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1024AnyDiffTextGuardNumber(value, at, bounds) : stdioDwgAc1024AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioDwgAc1024AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1024AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1024AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1024AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDwgDiffText(value: unknown, at = "$"): DwgDiffText {
  return stdioDwgAc1024AnyDiffTextGuardObject(value, `${at}`);
}
