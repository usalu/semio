/** 💡️ Ply inference schema — vertex-element bounding box plus vertex/face counts. */

export interface PlyBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  faceCount: number;
}

export interface PlyInference {
  /** @derived */
  bounds: PlyBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPly10AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPly10AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioPly10AnyInferenceGuardRefusal(at, why);
};

type stdioPly10AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPly10AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPly10AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPly10AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPly10AnyInferenceGuardReject(at, "value is not an object");
export const stdioPly10AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioPly10AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPly10AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPly10AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPly10AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPly10AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioPly10AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPly10AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPly10AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPly10AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPly10AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPly10AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPly10AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioPly10AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioPly10AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPly10AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPly10AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPly10AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPly10AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioPly10AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPly10AnyInferenceGuardNumber(value, at, bounds) : stdioPly10AnyInferenceGuardReject(at, "value is not an integer");
export const stdioPly10AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPly10AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPly10AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPly10AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlyInference(value: unknown, at = "$"): PlyInference {
  const row = stdioPly10AnyInferenceGuardObject(value, at);
  return {
    bounds: parsePlyBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parsePlyBounds(value: unknown, at = "$"): PlyBounds {
  const row = stdioPly10AnyInferenceGuardObject(value, at);
  return {
    min: stdioPly10AnyInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioPly10AnyInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioPly10AnyInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioPly10AnyInferenceGuardNumber(item, `${at}.max[${index}]`)),
    vertexCount: stdioPly10AnyInferenceGuardInteger(row["vertexCount"], `${at}.vertexCount`, {"minimum": 0}),
    faceCount: stdioPly10AnyInferenceGuardInteger(row["faceCount"], `${at}.faceCount`, {"minimum": 0}),
  };
}
