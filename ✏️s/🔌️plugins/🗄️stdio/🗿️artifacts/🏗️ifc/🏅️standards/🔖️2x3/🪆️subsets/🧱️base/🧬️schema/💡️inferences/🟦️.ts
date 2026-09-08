/** 💡️ IFC2X3 inference schema — `IFCCARTESIANPOINT`-derived spatial bounding box. */

export interface Ifc2x3Bounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}

export interface Ifc2x3Inference {
  /** @derived */
  bounds: Ifc2x3Bounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc2x3BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc2x3BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioIfc2x3BaseInferenceGuardRefusal(at, why);
};

type stdioIfc2x3BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc2x3BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc2x3BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc2x3BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc2x3BaseInferenceGuardReject(at, "value is not an object");
export const stdioIfc2x3BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioIfc2x3BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc2x3BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc2x3BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc2x3BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc2x3BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioIfc2x3BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc2x3BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc2x3BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc2x3BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc2x3BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc2x3BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc2x3BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioIfc2x3BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioIfc2x3BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc2x3BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc2x3BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc2x3BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc2x3BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioIfc2x3BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc2x3BaseInferenceGuardNumber(value, at, bounds) : stdioIfc2x3BaseInferenceGuardReject(at, "value is not an integer");
export const stdioIfc2x3BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc2x3BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc2x3BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc2x3BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfc2x3Inference(value: unknown, at = "$"): Ifc2x3Inference {
  const row = stdioIfc2x3BaseInferenceGuardObject(value, at);
  return {
    bounds: parseIfc2x3Bounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseIfc2x3Bounds(value: unknown, at = "$"): Ifc2x3Bounds {
  const row = stdioIfc2x3BaseInferenceGuardObject(value, at);
  return {
    min: stdioIfc2x3BaseInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioIfc2x3BaseInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioIfc2x3BaseInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioIfc2x3BaseInferenceGuardNumber(item, `${at}.max[${index}]`)),
    pointCount: stdioIfc2x3BaseInferenceGuardInteger(row["pointCount"], `${at}.pointCount`, {"minimum": 0}),
  };
}
