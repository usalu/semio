/** 📝️ Text representation for `trinity.rewriting.snapshot`. */
export type RewritingSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingSnapshotTextGuardRefusal(at, why);
};

type trinityRewritingSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingSnapshotTextGuardReject(at, "value is not an object");
export const trinityRewritingSnapshotTextGuardArray = (value: unknown, at: string, bounds: trinityRewritingSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingSnapshotTextGuardString = (value: unknown, at: string, bounds: trinityRewritingSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingSnapshotTextGuardReject(at, "value is not a boolean"));
export const trinityRewritingSnapshotTextGuardNumber = (value: unknown, at: string, bounds: trinityRewritingSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingSnapshotTextGuardInteger = (value: unknown, at: string, bounds: trinityRewritingSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingSnapshotTextGuardNumber(value, at, bounds) : trinityRewritingSnapshotTextGuardReject(at, "value is not an integer");
export const trinityRewritingSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingSnapshotText(value: unknown, at = "$"): RewritingSnapshotText {
  return trinityRewritingSnapshotTextGuardObject(value, `${at}`);
}
