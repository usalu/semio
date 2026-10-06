/** 📝️ Text representation for `stdio.md` (diff). */
export type MdDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnyDiffTextGuardRefusal(at, why);
};

type stdioMdCommonmarkAnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnyDiffTextGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnyDiffTextGuardNumber(value, at, bounds) : stdioMdCommonmarkAnyDiffTextGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdDiffText(value: unknown, at = "$"): MdDiffText {
  return stdioMdCommonmarkAnyDiffTextGuardObject(value, `${at}`);
}
