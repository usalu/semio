/** 📝️ Text representation for `stdio.bcf` (diff). */
export type BcfDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBcf21MarkupDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBcf21MarkupDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioBcf21MarkupDiffTextGuardRefusal(at, why);
};

type stdioBcf21MarkupDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBcf21MarkupDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBcf21MarkupDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBcf21MarkupDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBcf21MarkupDiffTextGuardReject(at, "value is not an object");
export const stdioBcf21MarkupDiffTextGuardArray = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBcf21MarkupDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBcf21MarkupDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBcf21MarkupDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBcf21MarkupDiffTextGuardString = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBcf21MarkupDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBcf21MarkupDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBcf21MarkupDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBcf21MarkupDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBcf21MarkupDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBcf21MarkupDiffTextGuardReject(at, "value is not a boolean"));
export const stdioBcf21MarkupDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBcf21MarkupDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBcf21MarkupDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBcf21MarkupDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBcf21MarkupDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBcf21MarkupDiffTextGuardNumber(value, at, bounds) : stdioBcf21MarkupDiffTextGuardReject(at, "value is not an integer");
export const stdioBcf21MarkupDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBcf21MarkupDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBcf21MarkupDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBcf21MarkupDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBcfDiffText(value: unknown, at = "$"): BcfDiffText {
  return stdioBcf21MarkupDiffTextGuardObject(value, `${at}`);
}
