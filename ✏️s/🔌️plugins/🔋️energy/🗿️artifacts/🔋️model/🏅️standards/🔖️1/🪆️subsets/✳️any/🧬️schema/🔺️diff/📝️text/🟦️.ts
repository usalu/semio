/** 📝️ Text representation for `energy.model.diff`. */
export type EnergyModelDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class energyModelDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const energyModelDiffTextGuardReject = (at: string, why: string): never => {
  throw new energyModelDiffTextGuardRefusal(at, why);
};

type energyModelDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type energyModelDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type energyModelDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const energyModelDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : energyModelDiffTextGuardReject(at, "value is not an object");
export const energyModelDiffTextGuardArray = (value: unknown, at: string, bounds: energyModelDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return energyModelDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) energyModelDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) energyModelDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const energyModelDiffTextGuardString = (value: unknown, at: string, bounds: energyModelDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return energyModelDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) energyModelDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) energyModelDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) energyModelDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const energyModelDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : energyModelDiffTextGuardReject(at, "value is not a boolean"));
export const energyModelDiffTextGuardNumber = (value: unknown, at: string, bounds: energyModelDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return energyModelDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) energyModelDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) energyModelDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const energyModelDiffTextGuardInteger = (value: unknown, at: string, bounds: energyModelDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? energyModelDiffTextGuardNumber(value, at, bounds) : energyModelDiffTextGuardReject(at, "value is not an integer");
export const energyModelDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : energyModelDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const energyModelDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : energyModelDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEnergyModelDiffText(value: unknown, at = "$"): EnergyModelDiffText {
  return energyModelDiffTextGuardObject(value, `${at}`);
}
