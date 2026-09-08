/** 📝️ Text representation for `stdio.las` (diff). */
export type LasDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioLas10HeaderDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioLas10HeaderDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioLas10HeaderDiffTextGuardRefusal(at, why);
};

type stdioLas10HeaderDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioLas10HeaderDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioLas10HeaderDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioLas10HeaderDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioLas10HeaderDiffTextGuardReject(at, "value is not an object");
export const stdioLas10HeaderDiffTextGuardArray = (value: unknown, at: string, bounds: stdioLas10HeaderDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioLas10HeaderDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioLas10HeaderDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioLas10HeaderDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioLas10HeaderDiffTextGuardString = (value: unknown, at: string, bounds: stdioLas10HeaderDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioLas10HeaderDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioLas10HeaderDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioLas10HeaderDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioLas10HeaderDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioLas10HeaderDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioLas10HeaderDiffTextGuardReject(at, "value is not a boolean"));
export const stdioLas10HeaderDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioLas10HeaderDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioLas10HeaderDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioLas10HeaderDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioLas10HeaderDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioLas10HeaderDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioLas10HeaderDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioLas10HeaderDiffTextGuardNumber(value, at, bounds) : stdioLas10HeaderDiffTextGuardReject(at, "value is not an integer");
export const stdioLas10HeaderDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioLas10HeaderDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioLas10HeaderDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioLas10HeaderDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLasDiffText(value: unknown, at = "$"): LasDiffText {
  return stdioLas10HeaderDiffTextGuardObject(value, `${at}`);
}
