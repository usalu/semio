/** 📝️ Text representation for `energy.model.inference`. */
export type EnergyModelInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class energyModelInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const energyModelInferenceTextGuardReject = (at: string, why: string): never => {
  throw new energyModelInferenceTextGuardRefusal(at, why);
};

type energyModelInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type energyModelInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type energyModelInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const energyModelInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : energyModelInferenceTextGuardReject(at, "value is not an object");
export const energyModelInferenceTextGuardArray = (value: unknown, at: string, bounds: energyModelInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return energyModelInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) energyModelInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) energyModelInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const energyModelInferenceTextGuardString = (value: unknown, at: string, bounds: energyModelInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return energyModelInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) energyModelInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) energyModelInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) energyModelInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const energyModelInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : energyModelInferenceTextGuardReject(at, "value is not a boolean"));
export const energyModelInferenceTextGuardNumber = (value: unknown, at: string, bounds: energyModelInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return energyModelInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) energyModelInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) energyModelInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const energyModelInferenceTextGuardInteger = (value: unknown, at: string, bounds: energyModelInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? energyModelInferenceTextGuardNumber(value, at, bounds) : energyModelInferenceTextGuardReject(at, "value is not an integer");
export const energyModelInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : energyModelInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const energyModelInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : energyModelInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEnergyModelInferenceText(value: unknown, at = "$"): EnergyModelInferenceText {
  return energyModelInferenceTextGuardObject(value, `${at}`);
}
