/** 📝️ Text representation for `stdio.pptx` (diff). */
export type PptxDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPptxEcma376BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPptxEcma376BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioPptxEcma376BaseDiffTextGuardRefusal(at, why);
};

type stdioPptxEcma376BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPptxEcma376BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPptxEcma376BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPptxEcma376BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPptxEcma376BaseDiffTextGuardReject(at, "value is not an object");
export const stdioPptxEcma376BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPptxEcma376BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPptxEcma376BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPptxEcma376BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPptxEcma376BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPptxEcma376BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPptxEcma376BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPptxEcma376BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPptxEcma376BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPptxEcma376BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPptxEcma376BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioPptxEcma376BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPptxEcma376BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPptxEcma376BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPptxEcma376BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPptxEcma376BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPptxEcma376BaseDiffTextGuardNumber(value, at, bounds) : stdioPptxEcma376BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioPptxEcma376BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPptxEcma376BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPptxEcma376BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPptxEcma376BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePptxDiffText(value: unknown, at = "$"): PptxDiffText {
  return stdioPptxEcma376BaseDiffTextGuardObject(value, `${at}`);
}
