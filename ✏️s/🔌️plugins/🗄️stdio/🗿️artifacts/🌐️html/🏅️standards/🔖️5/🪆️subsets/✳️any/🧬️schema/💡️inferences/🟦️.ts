/** 💡️ Html inference schema — document outline (element count, max depth, text length). */

export interface HtmlOutline {
  elementCount: number;
  maxDepth: number;
  textLength: number;
}

export interface HtmlInference {
  /** @derived */
  outline: HtmlOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioHtml5AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioHtml5AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioHtml5AnyInferenceGuardRefusal(at, why);
};

type stdioHtml5AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioHtml5AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioHtml5AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioHtml5AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioHtml5AnyInferenceGuardReject(at, "value is not an object");
export const stdioHtml5AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioHtml5AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioHtml5AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioHtml5AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioHtml5AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioHtml5AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioHtml5AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioHtml5AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioHtml5AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioHtml5AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioHtml5AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioHtml5AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioHtml5AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioHtml5AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioHtml5AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioHtml5AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioHtml5AnyInferenceGuardNumber(value, at, bounds) : stdioHtml5AnyInferenceGuardReject(at, "value is not an integer");
export const stdioHtml5AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioHtml5AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioHtml5AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioHtml5AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHtmlInference(value: unknown, at = "$"): HtmlInference {
  const row = stdioHtml5AnyInferenceGuardObject(value, at);
  return {
    outline: parseHtmlOutline(row["outline"], `${at}.outline`),
  };
}

export function parseHtmlOutline(value: unknown, at = "$"): HtmlOutline {
  const row = stdioHtml5AnyInferenceGuardObject(value, at);
  return {
    elementCount: stdioHtml5AnyInferenceGuardInteger(row["elementCount"], `${at}.elementCount`),
    maxDepth: stdioHtml5AnyInferenceGuardInteger(row["maxDepth"], `${at}.maxDepth`),
    textLength: stdioHtml5AnyInferenceGuardInteger(row["textLength"], `${at}.textLength`),
  };
}
