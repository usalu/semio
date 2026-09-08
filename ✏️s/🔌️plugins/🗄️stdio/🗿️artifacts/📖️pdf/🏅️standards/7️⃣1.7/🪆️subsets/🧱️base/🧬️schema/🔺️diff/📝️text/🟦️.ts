/** 📝️ Text representation for `stdio.pdf` (diff). */
export type PdfDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf17BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf17BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioPdf17BaseDiffTextGuardRefusal(at, why);
};

type stdioPdf17BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf17BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf17BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf17BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf17BaseDiffTextGuardReject(at, "value is not an object");
export const stdioPdf17BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioPdf17BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf17BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf17BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf17BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf17BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioPdf17BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf17BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf17BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf17BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf17BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf17BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf17BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioPdf17BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioPdf17BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf17BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf17BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf17BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf17BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioPdf17BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf17BaseDiffTextGuardNumber(value, at, bounds) : stdioPdf17BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioPdf17BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf17BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf17BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf17BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfDiffText(value: unknown, at = "$"): PdfDiffText {
  return stdioPdf17BaseDiffTextGuardObject(value, `${at}`);
}
