/** 📝️ Text representation for `stdio.jpg` (diff). */
export type JpgDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJpgJfif101DocumentDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJpgJfif101DocumentDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioJpgJfif101DocumentDiffTextGuardRefusal(at, why);
};

type stdioJpgJfif101DocumentDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJpgJfif101DocumentDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJpgJfif101DocumentDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJpgJfif101DocumentDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJpgJfif101DocumentDiffTextGuardReject(at, "value is not an object");
export const stdioJpgJfif101DocumentDiffTextGuardArray = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJpgJfif101DocumentDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJpgJfif101DocumentDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJpgJfif101DocumentDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJpgJfif101DocumentDiffTextGuardString = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJpgJfif101DocumentDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJpgJfif101DocumentDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJpgJfif101DocumentDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJpgJfif101DocumentDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJpgJfif101DocumentDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJpgJfif101DocumentDiffTextGuardReject(at, "value is not a boolean"));
export const stdioJpgJfif101DocumentDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJpgJfif101DocumentDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJpgJfif101DocumentDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJpgJfif101DocumentDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJpgJfif101DocumentDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJpgJfif101DocumentDiffTextGuardNumber(value, at, bounds) : stdioJpgJfif101DocumentDiffTextGuardReject(at, "value is not an integer");
export const stdioJpgJfif101DocumentDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJpgJfif101DocumentDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJpgJfif101DocumentDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJpgJfif101DocumentDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJpgDiffText(value: unknown, at = "$"): JpgDiffText {
  return stdioJpgJfif101DocumentDiffTextGuardObject(value, `${at}`);
}
