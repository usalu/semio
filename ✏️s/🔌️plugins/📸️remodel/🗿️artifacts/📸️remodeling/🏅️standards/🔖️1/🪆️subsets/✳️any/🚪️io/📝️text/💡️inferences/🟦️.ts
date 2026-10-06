/** 📝️ Text representation for `remodel.remodeling.inference`. */
export type RemodelingInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingInferenceTextGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingInferenceTextGuardRefusal(at, why);
};

type remodelRemodelingInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingInferenceTextGuardReject(at, "value is not an object");
export const remodelRemodelingInferenceTextGuardArray = (value: unknown, at: string, bounds: remodelRemodelingInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingInferenceTextGuardString = (value: unknown, at: string, bounds: remodelRemodelingInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingInferenceTextGuardReject(at, "value is not a boolean"));
export const remodelRemodelingInferenceTextGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingInferenceTextGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingInferenceTextGuardNumber(value, at, bounds) : remodelRemodelingInferenceTextGuardReject(at, "value is not an integer");
export const remodelRemodelingInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingInferenceText(value: unknown, at = "$"): RemodelingInferenceText {
  return remodelRemodelingInferenceTextGuardObject(value, `${at}`);
}
