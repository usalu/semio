/** 📝️ Text representation for `stdio.docx` (diff). */
export type DocxDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDocxEcma376BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDocxEcma376BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioDocxEcma376BaseDiffTextGuardRefusal(at, why);
};

type stdioDocxEcma376BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDocxEcma376BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDocxEcma376BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDocxEcma376BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDocxEcma376BaseDiffTextGuardReject(at, "value is not an object");
export const stdioDocxEcma376BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDocxEcma376BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDocxEcma376BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDocxEcma376BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDocxEcma376BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDocxEcma376BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDocxEcma376BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDocxEcma376BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDocxEcma376BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDocxEcma376BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDocxEcma376BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioDocxEcma376BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDocxEcma376BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDocxEcma376BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDocxEcma376BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDocxEcma376BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDocxEcma376BaseDiffTextGuardNumber(value, at, bounds) : stdioDocxEcma376BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioDocxEcma376BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDocxEcma376BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDocxEcma376BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDocxEcma376BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDocxDiffText(value: unknown, at = "$"): DocxDiffText {
  return stdioDocxEcma376BaseDiffTextGuardObject(value, `${at}`);
}
