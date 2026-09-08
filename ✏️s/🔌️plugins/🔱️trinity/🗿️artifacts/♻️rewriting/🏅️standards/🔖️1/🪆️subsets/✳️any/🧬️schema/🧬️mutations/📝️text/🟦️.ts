/** 📝️ Text representation for `trinity.rewriting.mutations`. */
export type RewritingMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingMutationsTextGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingMutationsTextGuardRefusal(at, why);
};

type trinityRewritingMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingMutationsTextGuardReject(at, "value is not an object");
export const trinityRewritingMutationsTextGuardArray = (value: unknown, at: string, bounds: trinityRewritingMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingMutationsTextGuardString = (value: unknown, at: string, bounds: trinityRewritingMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingMutationsTextGuardReject(at, "value is not a boolean"));
export const trinityRewritingMutationsTextGuardNumber = (value: unknown, at: string, bounds: trinityRewritingMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingMutationsTextGuardInteger = (value: unknown, at: string, bounds: trinityRewritingMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingMutationsTextGuardNumber(value, at, bounds) : trinityRewritingMutationsTextGuardReject(at, "value is not an integer");
export const trinityRewritingMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingMutationsText(value: unknown, at = "$"): RewritingMutationsText {
  return trinityRewritingMutationsTextGuardObject(value, `${at}`);
}
