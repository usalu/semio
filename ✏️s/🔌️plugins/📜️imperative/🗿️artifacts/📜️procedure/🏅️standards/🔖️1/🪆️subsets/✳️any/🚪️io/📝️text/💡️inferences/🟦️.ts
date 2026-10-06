/** 📝️ Text representation for `imperative.procedure.inference`. */
export type ProcedureInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeInferenceTextGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeInferenceTextGuardRefusal(at, why);
};

type imperativeImperativeInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeInferenceTextGuardReject(at, "value is not an object");
export const imperativeImperativeInferenceTextGuardArray = (value: unknown, at: string, bounds: imperativeImperativeInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeInferenceTextGuardString = (value: unknown, at: string, bounds: imperativeImperativeInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeInferenceTextGuardReject(at, "value is not a boolean"));
export const imperativeImperativeInferenceTextGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeInferenceTextGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeInferenceTextGuardNumber(value, at, bounds) : imperativeImperativeInferenceTextGuardReject(at, "value is not an integer");
export const imperativeImperativeInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureInferenceText(value: unknown, at = "$"): ProcedureInferenceText {
  return imperativeImperativeInferenceTextGuardObject(value, `${at}`);
}
