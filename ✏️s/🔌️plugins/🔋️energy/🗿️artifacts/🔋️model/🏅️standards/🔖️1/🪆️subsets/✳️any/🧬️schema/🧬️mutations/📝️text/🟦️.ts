/** 📝️ Text representation for `energy.model.mutations`. */
export type EnergyModelMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class energyModelMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const energyModelMutationsTextGuardReject = (at: string, why: string): never => {
  throw new energyModelMutationsTextGuardRefusal(at, why);
};

type energyModelMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type energyModelMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type energyModelMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const energyModelMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : energyModelMutationsTextGuardReject(at, "value is not an object");
export const energyModelMutationsTextGuardArray = (value: unknown, at: string, bounds: energyModelMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return energyModelMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) energyModelMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) energyModelMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const energyModelMutationsTextGuardString = (value: unknown, at: string, bounds: energyModelMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return energyModelMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) energyModelMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) energyModelMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) energyModelMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const energyModelMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : energyModelMutationsTextGuardReject(at, "value is not a boolean"));
export const energyModelMutationsTextGuardNumber = (value: unknown, at: string, bounds: energyModelMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return energyModelMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) energyModelMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) energyModelMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const energyModelMutationsTextGuardInteger = (value: unknown, at: string, bounds: energyModelMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? energyModelMutationsTextGuardNumber(value, at, bounds) : energyModelMutationsTextGuardReject(at, "value is not an integer");
export const energyModelMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : energyModelMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const energyModelMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : energyModelMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEnergyModelMutationsText(value: unknown, at = "$"): EnergyModelMutationsText {
  return energyModelMutationsTextGuardObject(value, `${at}`);
}
