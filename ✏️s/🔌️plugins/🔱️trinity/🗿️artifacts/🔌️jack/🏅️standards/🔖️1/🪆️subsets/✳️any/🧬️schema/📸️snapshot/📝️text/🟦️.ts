/** 📝️ Text representation for `trinity.jack.snapshot`. */
export type JackSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new trinityJackSnapshotTextGuardRefusal(at, why);
};

type trinityJackSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackSnapshotTextGuardReject(at, "value is not an object");
export const trinityJackSnapshotTextGuardArray = (value: unknown, at: string, bounds: trinityJackSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackSnapshotTextGuardString = (value: unknown, at: string, bounds: trinityJackSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackSnapshotTextGuardReject(at, "value is not a boolean"));
export const trinityJackSnapshotTextGuardNumber = (value: unknown, at: string, bounds: trinityJackSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackSnapshotTextGuardInteger = (value: unknown, at: string, bounds: trinityJackSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackSnapshotTextGuardNumber(value, at, bounds) : trinityJackSnapshotTextGuardReject(at, "value is not an integer");
export const trinityJackSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackSnapshotText(value: unknown, at = "$"): JackSnapshotText {
  return trinityJackSnapshotTextGuardObject(value, `${at}`);
}
