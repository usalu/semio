/** 💡️ Svg inference schema — root `<svg>` intrinsic size. */

export interface SvgDimensions {
  width: number;
  height: number;
}

export interface SvgInference {
  /** @derived */
  dimensions: SvgDimensions;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSvg11BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSvg11BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSvg11BaseInferenceGuardRefusal(at, why);
};

type stdioSvg11BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSvg11BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSvg11BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSvg11BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSvg11BaseInferenceGuardReject(at, "value is not an object");
export const stdioSvg11BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSvg11BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSvg11BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSvg11BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSvg11BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSvg11BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSvg11BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSvg11BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSvg11BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSvg11BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSvg11BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioSvg11BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSvg11BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSvg11BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSvg11BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSvg11BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSvg11BaseInferenceGuardNumber(value, at, bounds) : stdioSvg11BaseInferenceGuardReject(at, "value is not an integer");
export const stdioSvg11BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSvg11BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSvg11BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSvg11BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSvgInference(value: unknown, at = "$"): SvgInference {
  const row = stdioSvg11BaseInferenceGuardObject(value, at);
  return {
    dimensions: parseSvgDimensions(row["dimensions"], `${at}.dimensions`),
  };
}

export function parseSvgDimensions(value: unknown, at = "$"): SvgDimensions {
  const row = stdioSvg11BaseInferenceGuardObject(value, at);
  return {
    width: stdioSvg11BaseInferenceGuardNumber(row["width"], `${at}.width`),
    height: stdioSvg11BaseInferenceGuardNumber(row["height"], `${at}.height`),
  };
}
