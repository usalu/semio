/** 📝️ Text representation for `stdio.zip` (diff). */
export type ZipDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioZip20BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioZip20BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioZip20BaseDiffTextGuardRefusal(at, why);
};

type stdioZip20BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioZip20BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioZip20BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioZip20BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioZip20BaseDiffTextGuardReject(at, "value is not an object");
export const stdioZip20BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioZip20BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioZip20BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioZip20BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioZip20BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioZip20BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioZip20BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioZip20BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioZip20BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioZip20BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioZip20BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioZip20BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioZip20BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioZip20BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioZip20BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioZip20BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioZip20BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioZip20BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioZip20BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioZip20BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioZip20BaseDiffTextGuardNumber(value, at, bounds) : stdioZip20BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioZip20BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioZip20BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioZip20BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioZip20BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseZipDiffText(value: unknown, at = "$"): ZipDiffText {
  return stdioZip20BaseDiffTextGuardObject(value, `${at}`);
}
