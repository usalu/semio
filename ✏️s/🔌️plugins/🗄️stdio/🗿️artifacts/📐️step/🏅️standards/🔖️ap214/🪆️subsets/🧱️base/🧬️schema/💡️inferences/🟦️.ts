/** 💡️ STEP AP214 inference schema — `CARTESIAN_POINT`-derived spatial bounding box. */

export interface StepBounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}

export interface StepInference {
  /** @derived */
  bounds: StepBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStepAp214BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStepAp214BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioStepAp214BaseInferenceGuardRefusal(at, why);
};

type stdioStepAp214BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStepAp214BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStepAp214BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStepAp214BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStepAp214BaseInferenceGuardReject(at, "value is not an object");
export const stdioStepAp214BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioStepAp214BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStepAp214BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStepAp214BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStepAp214BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStepAp214BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioStepAp214BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStepAp214BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStepAp214BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStepAp214BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStepAp214BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStepAp214BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStepAp214BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioStepAp214BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioStepAp214BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStepAp214BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStepAp214BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStepAp214BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStepAp214BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioStepAp214BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStepAp214BaseInferenceGuardNumber(value, at, bounds) : stdioStepAp214BaseInferenceGuardReject(at, "value is not an integer");
export const stdioStepAp214BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStepAp214BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStepAp214BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStepAp214BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStepInference(value: unknown, at = "$"): StepInference {
  const row = stdioStepAp214BaseInferenceGuardObject(value, at);
  return {
    bounds: parseStepBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseStepBounds(value: unknown, at = "$"): StepBounds {
  const row = stdioStepAp214BaseInferenceGuardObject(value, at);
  return {
    min: stdioStepAp214BaseInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStepAp214BaseInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioStepAp214BaseInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStepAp214BaseInferenceGuardNumber(item, `${at}.max[${index}]`)),
    pointCount: stdioStepAp214BaseInferenceGuardInteger(row["pointCount"], `${at}.pointCount`, {"minimum": 0}),
  };
}
