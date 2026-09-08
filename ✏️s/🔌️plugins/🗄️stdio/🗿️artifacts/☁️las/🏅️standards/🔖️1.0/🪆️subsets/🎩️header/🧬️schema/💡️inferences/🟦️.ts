/** 💡️ Las inference schema — header-declared bounding box and point count. */

export interface LasBounds {
  minX: number;
  minY: number;
  minZ: number;
  maxX: number;
  maxY: number;
  maxZ: number;
  pointCount: number;
}

export interface LasInference {
  /** @derived */
  bounds: LasBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioLas10HeaderInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioLas10HeaderInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioLas10HeaderInferenceGuardRefusal(at, why);
};

type stdioLas10HeaderInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioLas10HeaderInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioLas10HeaderInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioLas10HeaderInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioLas10HeaderInferenceGuardReject(at, "value is not an object");
export const stdioLas10HeaderInferenceGuardArray = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioLas10HeaderInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioLas10HeaderInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioLas10HeaderInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioLas10HeaderInferenceGuardString = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioLas10HeaderInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioLas10HeaderInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioLas10HeaderInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioLas10HeaderInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioLas10HeaderInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioLas10HeaderInferenceGuardReject(at, "value is not a boolean"));
export const stdioLas10HeaderInferenceGuardNumber = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioLas10HeaderInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioLas10HeaderInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioLas10HeaderInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioLas10HeaderInferenceGuardInteger = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioLas10HeaderInferenceGuardNumber(value, at, bounds) : stdioLas10HeaderInferenceGuardReject(at, "value is not an integer");
export const stdioLas10HeaderInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioLas10HeaderInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioLas10HeaderInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioLas10HeaderInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLasInference(value: unknown, at = "$"): LasInference {
  const row = stdioLas10HeaderInferenceGuardObject(value, at);
  return {
    bounds: parseLasBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseLasBounds(value: unknown, at = "$"): LasBounds {
  const row = stdioLas10HeaderInferenceGuardObject(value, at);
  return {
    minX: stdioLas10HeaderInferenceGuardNumber(row["minX"], `${at}.minX`),
    minY: stdioLas10HeaderInferenceGuardNumber(row["minY"], `${at}.minY`),
    minZ: stdioLas10HeaderInferenceGuardNumber(row["minZ"], `${at}.minZ`),
    maxX: stdioLas10HeaderInferenceGuardNumber(row["maxX"], `${at}.maxX`),
    maxY: stdioLas10HeaderInferenceGuardNumber(row["maxY"], `${at}.maxY`),
    maxZ: stdioLas10HeaderInferenceGuardNumber(row["maxZ"], `${at}.maxZ`),
    pointCount: stdioLas10HeaderInferenceGuardInteger(row["pointCount"], `${at}.pointCount`, {"minimum": 0}),
  };
}
