/** 📝️ Text representation for `remodel.remodeling.snapshot`. */
export type RemodelingSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingSnapshotTextGuardRefusal(at, why);
};

type remodelRemodelingSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingSnapshotTextGuardReject(at, "value is not an object");
export const remodelRemodelingSnapshotTextGuardArray = (value: unknown, at: string, bounds: remodelRemodelingSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingSnapshotTextGuardString = (value: unknown, at: string, bounds: remodelRemodelingSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingSnapshotTextGuardReject(at, "value is not a boolean"));
export const remodelRemodelingSnapshotTextGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingSnapshotTextGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingSnapshotTextGuardNumber(value, at, bounds) : remodelRemodelingSnapshotTextGuardReject(at, "value is not an integer");
export const remodelRemodelingSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingSnapshotText(value: unknown, at = "$"): RemodelingSnapshotText {
  return remodelRemodelingSnapshotTextGuardObject(value, `${at}`);
}
