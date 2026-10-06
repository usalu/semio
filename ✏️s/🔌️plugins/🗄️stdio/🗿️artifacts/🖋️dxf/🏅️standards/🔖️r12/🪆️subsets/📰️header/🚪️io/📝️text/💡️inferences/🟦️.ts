/** 📝️ Text representation for `s.stdio.dxf.inference`. */
export type DxfInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDxfR12HeaderInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDxfR12HeaderInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioDxfR12HeaderInferenceTextGuardRefusal(at, why);
};

type stdioDxfR12HeaderInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDxfR12HeaderInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDxfR12HeaderInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDxfR12HeaderInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDxfR12HeaderInferenceTextGuardReject(at, "value is not an object");
export const stdioDxfR12HeaderInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDxfR12HeaderInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDxfR12HeaderInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDxfR12HeaderInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDxfR12HeaderInferenceTextGuardString = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDxfR12HeaderInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDxfR12HeaderInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDxfR12HeaderInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDxfR12HeaderInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDxfR12HeaderInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDxfR12HeaderInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioDxfR12HeaderInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDxfR12HeaderInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDxfR12HeaderInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDxfR12HeaderInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDxfR12HeaderInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDxfR12HeaderInferenceTextGuardNumber(value, at, bounds) : stdioDxfR12HeaderInferenceTextGuardReject(at, "value is not an integer");
export const stdioDxfR12HeaderInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDxfR12HeaderInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDxfR12HeaderInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDxfR12HeaderInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDxfInferenceText(value: unknown, at = "$"): DxfInferenceText {
  return stdioDxfR12HeaderInferenceTextGuardObject(value, `${at}`);
}
