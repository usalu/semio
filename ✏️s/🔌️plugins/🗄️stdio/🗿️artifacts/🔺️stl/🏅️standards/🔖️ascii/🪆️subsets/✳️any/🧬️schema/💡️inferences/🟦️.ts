/** 💡️ Stl inference schema — triangle-soup bounding box and triangle count. */

export interface StlBounds {
  min: [number, number, number];
  max: [number, number, number];
  triangleCount: number;
}

export interface StlInference {
  /** @derived */
  bounds: StlBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStlAsciiAnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStlAsciiAnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioStlAsciiAnyInferenceGuardRefusal(at, why);
};

type stdioStlAsciiAnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStlAsciiAnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStlAsciiAnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStlAsciiAnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStlAsciiAnyInferenceGuardReject(at, "value is not an object");
export const stdioStlAsciiAnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioStlAsciiAnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStlAsciiAnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStlAsciiAnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStlAsciiAnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStlAsciiAnyInferenceGuardString = (value: unknown, at: string, bounds: stdioStlAsciiAnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStlAsciiAnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStlAsciiAnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStlAsciiAnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStlAsciiAnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStlAsciiAnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStlAsciiAnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioStlAsciiAnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioStlAsciiAnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStlAsciiAnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStlAsciiAnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStlAsciiAnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStlAsciiAnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioStlAsciiAnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStlAsciiAnyInferenceGuardNumber(value, at, bounds) : stdioStlAsciiAnyInferenceGuardReject(at, "value is not an integer");
export const stdioStlAsciiAnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStlAsciiAnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStlAsciiAnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStlAsciiAnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStlInference(value: unknown, at = "$"): StlInference {
  const row = stdioStlAsciiAnyInferenceGuardObject(value, at);
  return {
    bounds: parseStlBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseStlBounds(value: unknown, at = "$"): StlBounds {
  const row = stdioStlAsciiAnyInferenceGuardObject(value, at);
  return {
    min: stdioStlAsciiAnyInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioStlAsciiAnyInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioStlAsciiAnyInferenceGuardNumber(item, `${at}.max[${index}]`)),
    triangleCount: stdioStlAsciiAnyInferenceGuardInteger(row["triangleCount"], `${at}.triangleCount`, {"minimum": 0}),
  };
}
