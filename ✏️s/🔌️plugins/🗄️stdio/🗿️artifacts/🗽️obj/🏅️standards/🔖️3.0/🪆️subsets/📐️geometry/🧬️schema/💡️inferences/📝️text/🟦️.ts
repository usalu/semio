/** 📝️ Text representation for `s.stdio.obj.inference`. */
export type ObjInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioObj30GeometryInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioObj30GeometryInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioObj30GeometryInferenceTextGuardRefusal(at, why);
};

type stdioObj30GeometryInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioObj30GeometryInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioObj30GeometryInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioObj30GeometryInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioObj30GeometryInferenceTextGuardReject(at, "value is not an object");
export const stdioObj30GeometryInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioObj30GeometryInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioObj30GeometryInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioObj30GeometryInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioObj30GeometryInferenceTextGuardString = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioObj30GeometryInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioObj30GeometryInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioObj30GeometryInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioObj30GeometryInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioObj30GeometryInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioObj30GeometryInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioObj30GeometryInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioObj30GeometryInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioObj30GeometryInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioObj30GeometryInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioObj30GeometryInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioObj30GeometryInferenceTextGuardNumber(value, at, bounds) : stdioObj30GeometryInferenceTextGuardReject(at, "value is not an integer");
export const stdioObj30GeometryInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioObj30GeometryInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioObj30GeometryInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioObj30GeometryInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseObjInferenceText(value: unknown, at = "$"): ObjInferenceText {
  return stdioObj30GeometryInferenceTextGuardObject(value, `${at}`);
}
