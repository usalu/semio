/** 🧬️ Rewriting snapshot schema — artifact-lane fields only. */

export interface RewritingSnapshot {
  /** @state artifact */
  beforeFixtureJson: string;
  /** @state artifact */
  lhsJson: string;
  /** @state artifact */
  rhsJson: string;
  /** @state artifact */
  parameterBindings: Record<string, PropertyValue>;
  /** @state artifact */
  ruleLayout: Record<string, LayoutPoint>;
}

export type PropertyValue =
  | null
  | boolean
  | number
  | string
  | PropertyValue[]
  | { [key: string]: PropertyValue };

export interface LayoutPoint {
  x: number;
  y: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingSnapshotGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingSnapshotGuardRefusal(at, why);
};

type trinityRewritingSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingSnapshotGuardReject(at, "value is not an object");
export const trinityRewritingSnapshotGuardArray = (value: unknown, at: string, bounds: trinityRewritingSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingSnapshotGuardString = (value: unknown, at: string, bounds: trinityRewritingSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingSnapshotGuardReject(at, "value is not a boolean"));
export const trinityRewritingSnapshotGuardNumber = (value: unknown, at: string, bounds: trinityRewritingSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingSnapshotGuardInteger = (value: unknown, at: string, bounds: trinityRewritingSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingSnapshotGuardNumber(value, at, bounds) : trinityRewritingSnapshotGuardReject(at, "value is not an integer");
export const trinityRewritingSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingSnapshot(value: unknown, at = "$"): RewritingSnapshot {
  const row = trinityRewritingSnapshotGuardObject(value, at);
  return {
    beforeFixtureJson: trinityRewritingSnapshotGuardString(row["beforeFixtureJson"], `${at}.beforeFixtureJson`),
    lhsJson: trinityRewritingSnapshotGuardString(row["lhsJson"], `${at}.lhsJson`),
    rhsJson: trinityRewritingSnapshotGuardString(row["rhsJson"], `${at}.rhsJson`),
    parameterBindings: trinityRewritingSnapshotGuardObject(row["parameterBindings"], `${at}.parameterBindings`),
    ruleLayout: trinityRewritingSnapshotGuardObject(row["ruleLayout"], `${at}.ruleLayout`),
  };
}
