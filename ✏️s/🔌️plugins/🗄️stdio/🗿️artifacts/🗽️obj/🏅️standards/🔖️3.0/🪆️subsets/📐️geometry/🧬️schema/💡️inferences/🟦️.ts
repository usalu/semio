/** 💡️ Obj inference schema — vertex-derived bounding box. */

export interface ObjBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  faceCount: number;
  groupCount: number;
}

export interface ObjInference {
  /** @derived */
  bounds: ObjBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioObj30GeometryInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioObj30GeometryInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioObj30GeometryInferenceGuardRefusal(at, why);
};

type stdioObj30GeometryInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioObj30GeometryInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioObj30GeometryInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioObj30GeometryInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioObj30GeometryInferenceGuardReject(at, "value is not an object");
export const stdioObj30GeometryInferenceGuardArray = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioObj30GeometryInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioObj30GeometryInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioObj30GeometryInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioObj30GeometryInferenceGuardString = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioObj30GeometryInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioObj30GeometryInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioObj30GeometryInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioObj30GeometryInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioObj30GeometryInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioObj30GeometryInferenceGuardReject(at, "value is not a boolean"));
export const stdioObj30GeometryInferenceGuardNumber = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioObj30GeometryInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioObj30GeometryInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioObj30GeometryInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioObj30GeometryInferenceGuardInteger = (value: unknown, at: string, bounds: stdioObj30GeometryInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioObj30GeometryInferenceGuardNumber(value, at, bounds) : stdioObj30GeometryInferenceGuardReject(at, "value is not an integer");
export const stdioObj30GeometryInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioObj30GeometryInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioObj30GeometryInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioObj30GeometryInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseObjInference(value: unknown, at = "$"): ObjInference {
  const row = stdioObj30GeometryInferenceGuardObject(value, at);
  return {
    bounds: parseObjBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseObjBounds(value: unknown, at = "$"): ObjBounds {
  const row = stdioObj30GeometryInferenceGuardObject(value, at);
  return {
    min: stdioObj30GeometryInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioObj30GeometryInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioObj30GeometryInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioObj30GeometryInferenceGuardNumber(item, `${at}.max[${index}]`)),
    vertexCount: stdioObj30GeometryInferenceGuardInteger(row["vertexCount"], `${at}.vertexCount`, {"minimum": 0}),
    faceCount: stdioObj30GeometryInferenceGuardInteger(row["faceCount"], `${at}.faceCount`, {"minimum": 0}),
    groupCount: stdioObj30GeometryInferenceGuardInteger(row["groupCount"], `${at}.groupCount`, {"minimum": 0}),
  };
}
