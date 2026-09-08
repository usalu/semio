/** 📝️ Text representation for `stdio.xlsx` (diff). */
export type XlsxDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXlsxEcma376BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXlsxEcma376BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioXlsxEcma376BaseDiffTextGuardRefusal(at, why);
};

type stdioXlsxEcma376BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXlsxEcma376BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXlsxEcma376BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXlsxEcma376BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXlsxEcma376BaseDiffTextGuardReject(at, "value is not an object");
export const stdioXlsxEcma376BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXlsxEcma376BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXlsxEcma376BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXlsxEcma376BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXlsxEcma376BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXlsxEcma376BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXlsxEcma376BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXlsxEcma376BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXlsxEcma376BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXlsxEcma376BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXlsxEcma376BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioXlsxEcma376BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXlsxEcma376BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXlsxEcma376BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXlsxEcma376BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXlsxEcma376BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXlsxEcma376BaseDiffTextGuardNumber(value, at, bounds) : stdioXlsxEcma376BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioXlsxEcma376BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXlsxEcma376BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXlsxEcma376BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXlsxEcma376BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXlsxDiffText(value: unknown, at = "$"): XlsxDiffText {
  return stdioXlsxEcma376BaseDiffTextGuardObject(value, `${at}`);
}
