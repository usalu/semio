/** 🧬️ BinarySnapshot schema. */
export interface BinarySnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ bytes: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBinaryRawAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBinaryRawAnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioBinaryRawAnySnapshotGuardRefusal(at, why);
};

type stdioBinaryRawAnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBinaryRawAnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBinaryRawAnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBinaryRawAnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBinaryRawAnySnapshotGuardReject(at, "value is not an object");
export const stdioBinaryRawAnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBinaryRawAnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBinaryRawAnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBinaryRawAnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBinaryRawAnySnapshotGuardString = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBinaryRawAnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBinaryRawAnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBinaryRawAnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBinaryRawAnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBinaryRawAnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBinaryRawAnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioBinaryRawAnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBinaryRawAnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBinaryRawAnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBinaryRawAnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBinaryRawAnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBinaryRawAnySnapshotGuardNumber(value, at, bounds) : stdioBinaryRawAnySnapshotGuardReject(at, "value is not an integer");
export const stdioBinaryRawAnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBinaryRawAnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBinaryRawAnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBinaryRawAnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBinarySnapshot(value: unknown, at = "$"): BinarySnapshot {
  const row = stdioBinaryRawAnySnapshotGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioBinaryRawAnySnapshotGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioBinaryRawAnySnapshotGuardString(row["bytes"], `${at}.bytes`),
  };
}
