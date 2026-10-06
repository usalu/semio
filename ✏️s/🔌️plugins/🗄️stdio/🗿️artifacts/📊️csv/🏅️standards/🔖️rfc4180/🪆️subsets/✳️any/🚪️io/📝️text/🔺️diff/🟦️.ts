/** 📝️ Text representation for `stdio.csv` (diff). */
export type CsvDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioCsvRfc4180AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioCsvRfc4180AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioCsvRfc4180AnyDiffTextGuardRefusal(at, why);
};

type stdioCsvRfc4180AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioCsvRfc4180AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioCsvRfc4180AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioCsvRfc4180AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioCsvRfc4180AnyDiffTextGuardReject(at, "value is not an object");
export const stdioCsvRfc4180AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioCsvRfc4180AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioCsvRfc4180AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioCsvRfc4180AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioCsvRfc4180AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioCsvRfc4180AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioCsvRfc4180AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioCsvRfc4180AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioCsvRfc4180AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioCsvRfc4180AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioCsvRfc4180AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioCsvRfc4180AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioCsvRfc4180AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioCsvRfc4180AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioCsvRfc4180AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioCsvRfc4180AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioCsvRfc4180AnyDiffTextGuardNumber(value, at, bounds) : stdioCsvRfc4180AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioCsvRfc4180AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioCsvRfc4180AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioCsvRfc4180AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioCsvRfc4180AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCsvDiffText(value: unknown, at = "$"): CsvDiffText {
  return stdioCsvRfc4180AnyDiffTextGuardObject(value, `${at}`);
}
