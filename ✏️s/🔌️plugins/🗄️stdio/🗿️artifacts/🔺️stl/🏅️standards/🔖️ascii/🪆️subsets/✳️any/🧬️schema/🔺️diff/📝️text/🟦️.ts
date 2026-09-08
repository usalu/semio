/** 📝️ Text representation for `stdio.stl` (diff). */
export type StlDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStlAsciiAnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStlAsciiAnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioStlAsciiAnyDiffTextGuardRefusal(at, why);
};

type stdioStlAsciiAnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStlAsciiAnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStlAsciiAnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStlAsciiAnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStlAsciiAnyDiffTextGuardReject(at, "value is not an object");
export const stdioStlAsciiAnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStlAsciiAnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStlAsciiAnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStlAsciiAnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStlAsciiAnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStlAsciiAnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStlAsciiAnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStlAsciiAnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStlAsciiAnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStlAsciiAnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStlAsciiAnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioStlAsciiAnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStlAsciiAnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStlAsciiAnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStlAsciiAnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStlAsciiAnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioStlAsciiAnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStlAsciiAnyDiffTextGuardNumber(value, at, bounds) : stdioStlAsciiAnyDiffTextGuardReject(at, "value is not an integer");
export const stdioStlAsciiAnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStlAsciiAnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStlAsciiAnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStlAsciiAnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStlDiffText(value: unknown, at = "$"): StlDiffText {
  return stdioStlAsciiAnyDiffTextGuardObject(value, `${at}`);
}
