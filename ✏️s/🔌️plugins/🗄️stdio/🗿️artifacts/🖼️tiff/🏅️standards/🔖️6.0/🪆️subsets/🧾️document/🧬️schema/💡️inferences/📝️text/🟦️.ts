/** 📝️ Text representation for `stdio.tiff`. */
export type TiffInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentInferenceTextGuardRefusal(at, why);
};

type stdioTiff60DocumentInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentInferenceTextGuardReject(at, "value is not an object");
export const stdioTiff60DocumentInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentInferenceTextGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentInferenceTextGuardNumber(value, at, bounds) : stdioTiff60DocumentInferenceTextGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffInferenceText(value: unknown, at = "$"): TiffInferenceText {
  return stdioTiff60DocumentInferenceTextGuardObject(value, `${at}`);
}
