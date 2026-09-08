/** 💡️ Semio cad inference schema — entity-derived planar bounding box. */

/** Mirrors the shared `engine::geometry::SemioPoint2` reused by the Rust side. */
export interface SemioPoint2 {
  x: number;
  y: number;
}

export interface SemioCadBounds {
  min: SemioPoint2;
  max: SemioPoint2;
  entityCount: number;
}

export interface SemioCadInference {
  /** @derived */
  bounds: SemioCadBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1CadInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1CadInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1CadInferenceGuardRefusal(at, why);
};

type stdioSemioV1CadInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1CadInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1CadInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1CadInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1CadInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1CadInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1CadInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1CadInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1CadInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1CadInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1CadInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1CadInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1CadInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1CadInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1CadInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1CadInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1CadInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1CadInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1CadInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1CadInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1CadInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1CadInferenceGuardNumber(value, at, bounds) : stdioSemioV1CadInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1CadInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1CadInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1CadInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1CadInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioCadInference(value: unknown, at = "$"): SemioCadInference {
  const row = stdioSemioV1CadInferenceGuardObject(value, at);
  return {
    bounds: parseSemioCadBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseSemioPoint2(value: unknown, at = "$"): SemioPoint2 {
  const row = stdioSemioV1CadInferenceGuardObject(value, at);
  return {
    x: stdioSemioV1CadInferenceGuardNumber(row["x"], `${at}.x`),
    y: stdioSemioV1CadInferenceGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseSemioCadBounds(value: unknown, at = "$"): SemioCadBounds {
  const row = stdioSemioV1CadInferenceGuardObject(value, at);
  return {
    min: parseSemioPoint2(row["min"], `${at}.min`),
    max: parseSemioPoint2(row["max"], `${at}.max`),
    entityCount: stdioSemioV1CadInferenceGuardInteger(row["entityCount"], `${at}.entityCount`, {"minimum": 0}),
  };
}
