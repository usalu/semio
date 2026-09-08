/** 📝️ Text representation for `stdio.xml.inference`. */
export type XmlInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXml10BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXml10BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioXml10BaseInferenceTextGuardRefusal(at, why);
};

type stdioXml10BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXml10BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXml10BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXml10BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXml10BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioXml10BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioXml10BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXml10BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXml10BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXml10BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXml10BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioXml10BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXml10BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXml10BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXml10BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXml10BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXml10BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXml10BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioXml10BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioXml10BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXml10BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXml10BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXml10BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXml10BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioXml10BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXml10BaseInferenceTextGuardNumber(value, at, bounds) : stdioXml10BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioXml10BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXml10BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXml10BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXml10BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXmlInferenceText(value: unknown, at = "$"): XmlInferenceText {
  return stdioXml10BaseInferenceTextGuardObject(value, `${at}`);
}
