/** 📝️ Text representation for `stdio.tsv` (diff): the hand-rolled `keyword=hex` token line. */
export type TsvDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnyDiffTextGuardRefusal(at, why);
};

type stdioTsvIanaAnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnyDiffTextGuardReject(at, "value is not an object");
export const stdioTsvIanaAnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnyDiffTextGuardNumber(value, at, bounds) : stdioTsvIanaAnyDiffTextGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvDiffText(value: unknown, at = "$"): TsvDiffText {
  return stdioTsvIanaAnyDiffTextGuardObject(value, `${at}`);
}
