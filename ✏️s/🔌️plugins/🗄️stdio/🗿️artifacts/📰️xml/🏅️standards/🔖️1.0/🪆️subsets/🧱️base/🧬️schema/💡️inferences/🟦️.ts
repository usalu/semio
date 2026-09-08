/** 💡️ Xml inference schema — document outline (element count, max depth, hasDoctype). */

export interface XmlOutline {
  elementCount: number;
  maxDepth: number;
  hasDoctype: boolean;
}

export interface XmlInference {
  /** @derived */
  outline: XmlOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXml10BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXml10BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioXml10BaseInferenceGuardRefusal(at, why);
};

type stdioXml10BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXml10BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXml10BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXml10BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXml10BaseInferenceGuardReject(at, "value is not an object");
export const stdioXml10BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioXml10BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXml10BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXml10BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXml10BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXml10BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioXml10BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXml10BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXml10BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXml10BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXml10BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXml10BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXml10BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioXml10BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioXml10BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXml10BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXml10BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXml10BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXml10BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioXml10BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXml10BaseInferenceGuardNumber(value, at, bounds) : stdioXml10BaseInferenceGuardReject(at, "value is not an integer");
export const stdioXml10BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXml10BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXml10BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXml10BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXmlInference(value: unknown, at = "$"): XmlInference {
  const row = stdioXml10BaseInferenceGuardObject(value, at);
  return {
    outline: parseXmlOutline(row["outline"], `${at}.outline`),
  };
}

export function parseXmlOutline(value: unknown, at = "$"): XmlOutline {
  const row = stdioXml10BaseInferenceGuardObject(value, at);
  return {
    elementCount: stdioXml10BaseInferenceGuardInteger(row["elementCount"], `${at}.elementCount`),
    maxDepth: stdioXml10BaseInferenceGuardInteger(row["maxDepth"], `${at}.maxDepth`),
    hasDoctype: stdioXml10BaseInferenceGuardBoolean(row["hasDoctype"], `${at}.hasDoctype`),
  };
}
