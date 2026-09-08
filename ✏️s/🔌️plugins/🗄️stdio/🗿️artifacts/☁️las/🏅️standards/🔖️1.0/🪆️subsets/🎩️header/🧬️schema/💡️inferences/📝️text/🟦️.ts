/** 📝️ Text representation for `s.stdio.las.inference`. */
export type LasInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioLas10HeaderInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioLas10HeaderInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioLas10HeaderInferenceTextGuardRefusal(at, why);
};

type stdioLas10HeaderInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioLas10HeaderInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioLas10HeaderInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioLas10HeaderInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioLas10HeaderInferenceTextGuardReject(at, "value is not an object");
export const stdioLas10HeaderInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioLas10HeaderInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioLas10HeaderInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioLas10HeaderInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioLas10HeaderInferenceTextGuardString = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioLas10HeaderInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioLas10HeaderInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioLas10HeaderInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioLas10HeaderInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioLas10HeaderInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioLas10HeaderInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioLas10HeaderInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioLas10HeaderInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioLas10HeaderInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioLas10HeaderInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioLas10HeaderInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioLas10HeaderInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioLas10HeaderInferenceTextGuardNumber(value, at, bounds) : stdioLas10HeaderInferenceTextGuardReject(at, "value is not an integer");
export const stdioLas10HeaderInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioLas10HeaderInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioLas10HeaderInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioLas10HeaderInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLasInferenceText(value: unknown, at = "$"): LasInferenceText {
  return stdioLas10HeaderInferenceTextGuardObject(value, `${at}`);
}
