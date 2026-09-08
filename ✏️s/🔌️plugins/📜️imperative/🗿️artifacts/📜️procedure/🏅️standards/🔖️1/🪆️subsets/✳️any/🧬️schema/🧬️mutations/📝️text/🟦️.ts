/** 📝️ Text representation for `imperative.procedure.mutations`. */
export type ProcedureMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeMutationsTextGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeMutationsTextGuardRefusal(at, why);
};

type imperativeImperativeMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeMutationsTextGuardReject(at, "value is not an object");
export const imperativeImperativeMutationsTextGuardArray = (value: unknown, at: string, bounds: imperativeImperativeMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeMutationsTextGuardString = (value: unknown, at: string, bounds: imperativeImperativeMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeMutationsTextGuardReject(at, "value is not a boolean"));
export const imperativeImperativeMutationsTextGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeMutationsTextGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeMutationsTextGuardNumber(value, at, bounds) : imperativeImperativeMutationsTextGuardReject(at, "value is not an integer");
export const imperativeImperativeMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureMutationsText(value: unknown, at = "$"): ProcedureMutationsText {
  return imperativeImperativeMutationsTextGuardObject(value, `${at}`);
}
