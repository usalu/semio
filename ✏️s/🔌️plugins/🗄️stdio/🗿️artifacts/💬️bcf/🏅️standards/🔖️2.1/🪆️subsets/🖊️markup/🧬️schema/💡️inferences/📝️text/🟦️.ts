/** 📝️ Text representation for `s.stdio.bcf.inference`. */
export type BcfInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBcf21MarkupInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBcf21MarkupInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioBcf21MarkupInferenceTextGuardRefusal(at, why);
};

type stdioBcf21MarkupInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBcf21MarkupInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBcf21MarkupInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBcf21MarkupInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBcf21MarkupInferenceTextGuardReject(at, "value is not an object");
export const stdioBcf21MarkupInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBcf21MarkupInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBcf21MarkupInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBcf21MarkupInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBcf21MarkupInferenceTextGuardString = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBcf21MarkupInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBcf21MarkupInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBcf21MarkupInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBcf21MarkupInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBcf21MarkupInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBcf21MarkupInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioBcf21MarkupInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBcf21MarkupInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBcf21MarkupInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBcf21MarkupInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBcf21MarkupInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioBcf21MarkupInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBcf21MarkupInferenceTextGuardNumber(value, at, bounds) : stdioBcf21MarkupInferenceTextGuardReject(at, "value is not an integer");
export const stdioBcf21MarkupInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBcf21MarkupInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBcf21MarkupInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBcf21MarkupInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBcfInferenceText(value: unknown, at = "$"): BcfInferenceText {
  return stdioBcf21MarkupInferenceTextGuardObject(value, `${at}`);
}
