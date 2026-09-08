/** 💡️ IFC4 inference schema — `IFCCARTESIANPOINT`-derived spatial bounding box. */

export interface IfcBounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}

export interface IfcInference {
  /** @derived */
  bounds: IfcBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc4AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc4AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioIfc4AnyInferenceGuardRefusal(at, why);
};

type stdioIfc4AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc4AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc4AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc4AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc4AnyInferenceGuardReject(at, "value is not an object");
export const stdioIfc4AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioIfc4AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc4AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc4AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc4AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc4AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioIfc4AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc4AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc4AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc4AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc4AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc4AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc4AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioIfc4AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioIfc4AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc4AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc4AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc4AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc4AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioIfc4AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc4AnyInferenceGuardNumber(value, at, bounds) : stdioIfc4AnyInferenceGuardReject(at, "value is not an integer");
export const stdioIfc4AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc4AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc4AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc4AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfcInference(value: unknown, at = "$"): IfcInference {
  const row = stdioIfc4AnyInferenceGuardObject(value, at);
  return {
    bounds: parseIfcBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseIfcBounds(value: unknown, at = "$"): IfcBounds {
  const row = stdioIfc4AnyInferenceGuardObject(value, at);
  return {
    min: stdioIfc4AnyInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioIfc4AnyInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioIfc4AnyInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioIfc4AnyInferenceGuardNumber(item, `${at}.max[${index}]`)),
    pointCount: stdioIfc4AnyInferenceGuardInteger(row["pointCount"], `${at}.pointCount`, {"minimum": 0}),
  };
}
