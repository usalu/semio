/** 💡️ Semio model inference schema — position envelope over spatial nodes + elements. */

export interface SemioPoint3 {
  x: number;
  y: number;
  z: number;
}

export interface SemioModelBounds {
  min: SemioPoint3;
  max: SemioPoint3;
  entityCount: number;
}

export interface SemioModelInference {
  /** @derived */
  bounds: SemioModelBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ModelInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ModelInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ModelInferenceGuardRefusal(at, why);
};

type stdioSemioV1ModelInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ModelInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ModelInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ModelInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ModelInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1ModelInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ModelInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ModelInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ModelInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ModelInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ModelInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ModelInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ModelInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ModelInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ModelInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ModelInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ModelInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ModelInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ModelInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ModelInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ModelInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ModelInferenceGuardNumber(value, at, bounds) : stdioSemioV1ModelInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1ModelInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ModelInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ModelInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ModelInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioModelInference(value: unknown, at = "$"): SemioModelInference {
  const row = stdioSemioV1ModelInferenceGuardObject(value, at);
  return {
    bounds: parseSemioModelBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseSemioModelBounds(value: unknown, at = "$"): SemioModelBounds {
  const row = stdioSemioV1ModelInferenceGuardObject(value, at);
  return {
    min: parseSemioPoint3(row["min"], `${at}.min`),
    max: parseSemioPoint3(row["max"], `${at}.max`),
    entityCount: stdioSemioV1ModelInferenceGuardInteger(row["entityCount"], `${at}.entityCount`, {"minimum": 0}),
  };
}

export function parseSemioPoint3(value: unknown, at = "$"): SemioPoint3 {
  const row = stdioSemioV1ModelInferenceGuardObject(value, at);
  return {
    x: stdioSemioV1ModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: stdioSemioV1ModelInferenceGuardNumber(row["y"], `${at}.y`),
    z: stdioSemioV1ModelInferenceGuardNumber(row["z"], `${at}.z`),
  };
}
