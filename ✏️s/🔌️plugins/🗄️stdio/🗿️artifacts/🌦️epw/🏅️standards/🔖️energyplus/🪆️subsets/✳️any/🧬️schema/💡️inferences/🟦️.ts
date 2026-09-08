/** 💡️ Epw inference schema — hourly dry-bulb temperature min/max/avg derived from `records`. */

export interface EpwClimateSummary {
  recordCount: number;
  parsedTempCount: number;
  minDryBulbC: number;
  maxDryBulbC: number;
  avgDryBulbC: number;
}

export interface EpwInference {
  /** @derived */
  climate: EpwClimateSummary;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioEpwEnergyplusAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioEpwEnergyplusAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioEpwEnergyplusAnyInferenceGuardRefusal(at, why);
};

type stdioEpwEnergyplusAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioEpwEnergyplusAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioEpwEnergyplusAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioEpwEnergyplusAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioEpwEnergyplusAnyInferenceGuardReject(at, "value is not an object");
export const stdioEpwEnergyplusAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioEpwEnergyplusAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioEpwEnergyplusAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioEpwEnergyplusAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioEpwEnergyplusAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioEpwEnergyplusAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioEpwEnergyplusAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioEpwEnergyplusAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioEpwEnergyplusAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioEpwEnergyplusAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioEpwEnergyplusAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioEpwEnergyplusAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioEpwEnergyplusAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioEpwEnergyplusAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioEpwEnergyplusAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioEpwEnergyplusAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioEpwEnergyplusAnyInferenceGuardNumber(value, at, bounds) : stdioEpwEnergyplusAnyInferenceGuardReject(at, "value is not an integer");
export const stdioEpwEnergyplusAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioEpwEnergyplusAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioEpwEnergyplusAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioEpwEnergyplusAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEpwInference(value: unknown, at = "$"): EpwInference {
  const row = stdioEpwEnergyplusAnyInferenceGuardObject(value, at);
  return {
    climate: parseEpwClimateSummary(row["climate"], `${at}.climate`),
  };
}

export function parseEpwClimateSummary(value: unknown, at = "$"): EpwClimateSummary {
  const row = stdioEpwEnergyplusAnyInferenceGuardObject(value, at);
  return {
    recordCount: stdioEpwEnergyplusAnyInferenceGuardInteger(row["recordCount"], `${at}.recordCount`, {"minimum": 0}),
    parsedTempCount: stdioEpwEnergyplusAnyInferenceGuardInteger(row["parsedTempCount"], `${at}.parsedTempCount`, {"minimum": 0}),
    minDryBulbC: stdioEpwEnergyplusAnyInferenceGuardNumber(row["minDryBulbC"], `${at}.minDryBulbC`),
    maxDryBulbC: stdioEpwEnergyplusAnyInferenceGuardNumber(row["maxDryBulbC"], `${at}.maxDryBulbC`),
    avgDryBulbC: stdioEpwEnergyplusAnyInferenceGuardNumber(row["avgDryBulbC"], `${at}.avgDryBulbC`),
  };
}
