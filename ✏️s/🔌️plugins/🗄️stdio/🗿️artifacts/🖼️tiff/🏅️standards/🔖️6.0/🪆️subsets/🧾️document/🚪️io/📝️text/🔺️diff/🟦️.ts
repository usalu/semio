/** 📝️ Text representation for `stdio.tiff` (diff). */
export type TiffDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentDiffTextGuardRefusal(at, why);
};

type stdioTiff60DocumentDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentDiffTextGuardReject(at, "value is not an object");
export const stdioTiff60DocumentDiffTextGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentDiffTextGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentDiffTextGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentDiffTextGuardNumber(value, at, bounds) : stdioTiff60DocumentDiffTextGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffDiffText(value: unknown, at = "$"): TiffDiffText {
  return stdioTiff60DocumentDiffTextGuardObject(value, `${at}`);
}
