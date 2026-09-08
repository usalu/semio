/** 📝️ Text representation for `trinity.rewriting.diff`. */
export type RewritingDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingDiffTextGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingDiffTextGuardRefusal(at, why);
};

type trinityRewritingDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingDiffTextGuardReject(at, "value is not an object");
export const trinityRewritingDiffTextGuardArray = (value: unknown, at: string, bounds: trinityRewritingDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingDiffTextGuardString = (value: unknown, at: string, bounds: trinityRewritingDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingDiffTextGuardReject(at, "value is not a boolean"));
export const trinityRewritingDiffTextGuardNumber = (value: unknown, at: string, bounds: trinityRewritingDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingDiffTextGuardInteger = (value: unknown, at: string, bounds: trinityRewritingDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingDiffTextGuardNumber(value, at, bounds) : trinityRewritingDiffTextGuardReject(at, "value is not an integer");
export const trinityRewritingDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingDiffText(value: unknown, at = "$"): RewritingDiffText {
  return trinityRewritingDiffTextGuardObject(value, `${at}`);
}
