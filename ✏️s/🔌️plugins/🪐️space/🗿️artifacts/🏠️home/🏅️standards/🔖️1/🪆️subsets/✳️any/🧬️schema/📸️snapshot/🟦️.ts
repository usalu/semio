/** 🧬️ S Home snapshot schema — artifact-lane fields only. */

export interface SHomeSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  catalogGeneration: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeSnapshotGuardReject = (at: string, why: string): never => {
  throw new spaceHomeSnapshotGuardRefusal(at, why);
};

type spaceHomeSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeSnapshotGuardReject(at, "value is not an object");
export const spaceHomeSnapshotGuardArray = (value: unknown, at: string, bounds: spaceHomeSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeSnapshotGuardString = (value: unknown, at: string, bounds: spaceHomeSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeSnapshotGuardReject(at, "value is not a boolean"));
export const spaceHomeSnapshotGuardNumber = (value: unknown, at: string, bounds: spaceHomeSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeSnapshotGuardInteger = (value: unknown, at: string, bounds: spaceHomeSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeSnapshotGuardNumber(value, at, bounds) : spaceHomeSnapshotGuardReject(at, "value is not an integer");
export const spaceHomeSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeSnapshot(value: unknown, at = "$"): SHomeSnapshot {
  const row = spaceHomeSnapshotGuardObject(value, at);
  return {
    schema: spaceHomeSnapshotGuardString(row["schema"], `${at}.schema`),
    catalogGeneration: spaceHomeSnapshotGuardInteger(row["catalogGeneration"], `${at}.catalogGeneration`, {"minimum": 0}),
  };
}
