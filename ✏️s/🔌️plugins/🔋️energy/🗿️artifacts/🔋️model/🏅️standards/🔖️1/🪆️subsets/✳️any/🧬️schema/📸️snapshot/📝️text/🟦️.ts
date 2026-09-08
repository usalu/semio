/** 📝️ Text representation for `energy.model.snapshot`. */
export type EnergyModelSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class energyModelSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const energyModelSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new energyModelSnapshotTextGuardRefusal(at, why);
};

type energyModelSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type energyModelSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type energyModelSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const energyModelSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : energyModelSnapshotTextGuardReject(at, "value is not an object");
export const energyModelSnapshotTextGuardArray = (value: unknown, at: string, bounds: energyModelSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return energyModelSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) energyModelSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) energyModelSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const energyModelSnapshotTextGuardString = (value: unknown, at: string, bounds: energyModelSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return energyModelSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) energyModelSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) energyModelSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) energyModelSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const energyModelSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : energyModelSnapshotTextGuardReject(at, "value is not a boolean"));
export const energyModelSnapshotTextGuardNumber = (value: unknown, at: string, bounds: energyModelSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return energyModelSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) energyModelSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) energyModelSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const energyModelSnapshotTextGuardInteger = (value: unknown, at: string, bounds: energyModelSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? energyModelSnapshotTextGuardNumber(value, at, bounds) : energyModelSnapshotTextGuardReject(at, "value is not an integer");
export const energyModelSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : energyModelSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const energyModelSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : energyModelSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEnergyModelSnapshotText(value: unknown, at = "$"): EnergyModelSnapshotText {
  return energyModelSnapshotTextGuardObject(value, `${at}`);
}
