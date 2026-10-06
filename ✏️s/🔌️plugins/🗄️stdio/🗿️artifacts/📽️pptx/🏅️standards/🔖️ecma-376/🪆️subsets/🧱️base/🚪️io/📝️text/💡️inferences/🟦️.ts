/** 📝️ Text representation for `stdio.pptx.inference`. */
export type PptxInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPptxEcma376BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPptxEcma376BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioPptxEcma376BaseInferenceTextGuardRefusal(at, why);
};

type stdioPptxEcma376BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPptxEcma376BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPptxEcma376BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPptxEcma376BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPptxEcma376BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioPptxEcma376BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPptxEcma376BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPptxEcma376BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPptxEcma376BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPptxEcma376BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPptxEcma376BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPptxEcma376BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPptxEcma376BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPptxEcma376BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPptxEcma376BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPptxEcma376BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioPptxEcma376BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPptxEcma376BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPptxEcma376BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPptxEcma376BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPptxEcma376BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioPptxEcma376BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPptxEcma376BaseInferenceTextGuardNumber(value, at, bounds) : stdioPptxEcma376BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioPptxEcma376BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPptxEcma376BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPptxEcma376BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPptxEcma376BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePptxInferenceText(value: unknown, at = "$"): PptxInferenceText {
  return stdioPptxEcma376BaseInferenceTextGuardObject(value, `${at}`);
}
