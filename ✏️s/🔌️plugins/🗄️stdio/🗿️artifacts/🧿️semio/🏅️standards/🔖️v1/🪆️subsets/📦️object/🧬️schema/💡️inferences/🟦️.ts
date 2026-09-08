/** 💡️ Semio object inference schema — composition census + own placement. */

export interface SemioPoint3 {
  x: number;
  y: number;
  z: number;
}

export interface SemioObjectComposition {
  hasBrep: boolean;
  hasMesh: boolean;
  hasProperties: boolean;
  position: SemioPoint3;
}

export interface SemioObjectInference {
  /** @derived */
  composition: SemioObjectComposition;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectInferenceGuardRefusal(at, why);
};

type stdioSemioV1ObjectInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectInferenceGuardNumber(value, at, bounds) : stdioSemioV1ObjectInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectInference(value: unknown, at = "$"): SemioObjectInference {
  const row = stdioSemioV1ObjectInferenceGuardObject(value, at);
  return {
    composition: parseSemioObjectComposition(row["composition"], `${at}.composition`),
  };
}

export function parseSemioObjectComposition(value: unknown, at = "$"): SemioObjectComposition {
  const row = stdioSemioV1ObjectInferenceGuardObject(value, at);
  return {
    hasBrep: stdioSemioV1ObjectInferenceGuardBoolean(row["hasBrep"], `${at}.hasBrep`),
    hasMesh: stdioSemioV1ObjectInferenceGuardBoolean(row["hasMesh"], `${at}.hasMesh`),
    hasProperties: stdioSemioV1ObjectInferenceGuardBoolean(row["hasProperties"], `${at}.hasProperties`),
    position: parseSemioPoint3(row["position"], `${at}.position`),
  };
}

export function parseSemioPoint3(value: unknown, at = "$"): SemioPoint3 {
  const row = stdioSemioV1ObjectInferenceGuardObject(value, at);
  return {
    x: stdioSemioV1ObjectInferenceGuardNumber(row["x"], `${at}.x`),
    y: stdioSemioV1ObjectInferenceGuardNumber(row["y"], `${at}.y`),
    z: stdioSemioV1ObjectInferenceGuardNumber(row["z"], `${at}.z`),
  };
}
