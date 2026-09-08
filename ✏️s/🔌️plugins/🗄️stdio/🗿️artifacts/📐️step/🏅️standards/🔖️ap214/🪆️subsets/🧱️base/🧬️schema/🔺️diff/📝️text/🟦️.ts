/** 📝️ Text representation for `stdio.step` (diff). */
export type StepDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStepAp214BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStepAp214BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioStepAp214BaseDiffTextGuardRefusal(at, why);
};

type stdioStepAp214BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStepAp214BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStepAp214BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStepAp214BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStepAp214BaseDiffTextGuardReject(at, "value is not an object");
export const stdioStepAp214BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStepAp214BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStepAp214BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStepAp214BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStepAp214BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStepAp214BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStepAp214BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStepAp214BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStepAp214BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStepAp214BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStepAp214BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioStepAp214BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStepAp214BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStepAp214BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStepAp214BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStepAp214BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStepAp214BaseDiffTextGuardNumber(value, at, bounds) : stdioStepAp214BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioStepAp214BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStepAp214BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStepAp214BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStepAp214BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStepDiffText(value: unknown, at = "$"): StepDiffText {
  return stdioStepAp214BaseDiffTextGuardObject(value, `${at}`);
}
