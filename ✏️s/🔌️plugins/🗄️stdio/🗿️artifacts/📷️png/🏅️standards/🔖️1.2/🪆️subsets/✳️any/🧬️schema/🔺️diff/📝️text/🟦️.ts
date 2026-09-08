/** 📝️ Text representation for `stdio.png` (diff). */
export type PngDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPng12AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPng12AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioPng12AnyDiffTextGuardRefusal(at, why);
};

type stdioPng12AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPng12AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPng12AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPng12AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPng12AnyDiffTextGuardReject(at, "value is not an object");
export const stdioPng12AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioPng12AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPng12AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPng12AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPng12AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPng12AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioPng12AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPng12AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPng12AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPng12AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPng12AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPng12AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPng12AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioPng12AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioPng12AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPng12AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPng12AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPng12AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPng12AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioPng12AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPng12AnyDiffTextGuardNumber(value, at, bounds) : stdioPng12AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioPng12AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPng12AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPng12AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPng12AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePngDiffText(value: unknown, at = "$"): PngDiffText {
  return stdioPng12AnyDiffTextGuardObject(value, `${at}`);
}
