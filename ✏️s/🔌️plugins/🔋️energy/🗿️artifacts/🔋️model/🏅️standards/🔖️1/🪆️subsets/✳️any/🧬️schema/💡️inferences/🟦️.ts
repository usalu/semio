/** 💡️ EnergyModel inference schema — opaque-container census of the persisted `modelJson` body. */

export interface EnergyModelEntries {
  entryCount: number;
  byteSize: number;
  contentDigest: string;
}

export interface EnergyModelInference {
  /** @derived */
  entries: EnergyModelEntries;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class energyModelInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const energyModelInferenceGuardReject = (at: string, why: string): never => {
  throw new energyModelInferenceGuardRefusal(at, why);
};

type energyModelInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type energyModelInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type energyModelInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const energyModelInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : energyModelInferenceGuardReject(at, "value is not an object");
export const energyModelInferenceGuardArray = (value: unknown, at: string, bounds: energyModelInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return energyModelInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) energyModelInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) energyModelInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const energyModelInferenceGuardString = (value: unknown, at: string, bounds: energyModelInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return energyModelInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) energyModelInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) energyModelInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) energyModelInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const energyModelInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : energyModelInferenceGuardReject(at, "value is not a boolean"));
export const energyModelInferenceGuardNumber = (value: unknown, at: string, bounds: energyModelInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return energyModelInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) energyModelInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) energyModelInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const energyModelInferenceGuardInteger = (value: unknown, at: string, bounds: energyModelInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? energyModelInferenceGuardNumber(value, at, bounds) : energyModelInferenceGuardReject(at, "value is not an integer");
export const energyModelInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : energyModelInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const energyModelInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : energyModelInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEnergyModelInference(value: unknown, at = "$"): EnergyModelInference {
  const row = energyModelInferenceGuardObject(value, at);
  return {
    entries: parseEnergyModelEntries(row["entries"], `${at}.entries`),
  };
}

export function parseEnergyModelEntries(value: unknown, at = "$"): EnergyModelEntries {
  const row = energyModelInferenceGuardObject(value, at);
  return {
    entryCount: energyModelInferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
    byteSize: energyModelInferenceGuardInteger(row["byteSize"], `${at}.byteSize`),
    contentDigest: energyModelInferenceGuardString(row["contentDigest"], `${at}.contentDigest`),
  };
}
