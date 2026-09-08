/** 📝️ Text representation for `imperative.procedure.diff`. */
export type ProcedureDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeDiffTextGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeDiffTextGuardRefusal(at, why);
};

type imperativeImperativeDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeDiffTextGuardReject(at, "value is not an object");
export const imperativeImperativeDiffTextGuardArray = (value: unknown, at: string, bounds: imperativeImperativeDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeDiffTextGuardString = (value: unknown, at: string, bounds: imperativeImperativeDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeDiffTextGuardReject(at, "value is not a boolean"));
export const imperativeImperativeDiffTextGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeDiffTextGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeDiffTextGuardNumber(value, at, bounds) : imperativeImperativeDiffTextGuardReject(at, "value is not an integer");
export const imperativeImperativeDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureDiffText(value: unknown, at = "$"): ProcedureDiffText {
  return imperativeImperativeDiffTextGuardObject(value, `${at}`);
}
