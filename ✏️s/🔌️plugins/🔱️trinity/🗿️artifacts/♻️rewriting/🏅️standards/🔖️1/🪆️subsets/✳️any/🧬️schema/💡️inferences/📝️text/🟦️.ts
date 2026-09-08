/** 📝️ Text representation for `trinity.rewriting.inference`. */
export type RewritingInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingInferenceTextGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingInferenceTextGuardRefusal(at, why);
};

type trinityRewritingInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingInferenceTextGuardReject(at, "value is not an object");
export const trinityRewritingInferenceTextGuardArray = (value: unknown, at: string, bounds: trinityRewritingInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingInferenceTextGuardString = (value: unknown, at: string, bounds: trinityRewritingInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingInferenceTextGuardReject(at, "value is not a boolean"));
export const trinityRewritingInferenceTextGuardNumber = (value: unknown, at: string, bounds: trinityRewritingInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingInferenceTextGuardInteger = (value: unknown, at: string, bounds: trinityRewritingInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingInferenceTextGuardNumber(value, at, bounds) : trinityRewritingInferenceTextGuardReject(at, "value is not an integer");
export const trinityRewritingInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingInferenceText(value: unknown, at = "$"): RewritingInferenceText {
  return trinityRewritingInferenceTextGuardObject(value, `${at}`);
}
