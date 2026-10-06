/** 📝️ Text representation for `stdio.ifc` (diff). */
export type IfcDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc4AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc4AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioIfc4AnyDiffTextGuardRefusal(at, why);
};

type stdioIfc4AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc4AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc4AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc4AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc4AnyDiffTextGuardReject(at, "value is not an object");
export const stdioIfc4AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioIfc4AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc4AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc4AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc4AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc4AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioIfc4AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc4AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc4AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc4AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc4AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc4AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc4AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioIfc4AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioIfc4AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc4AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc4AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc4AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc4AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioIfc4AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc4AnyDiffTextGuardNumber(value, at, bounds) : stdioIfc4AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioIfc4AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc4AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc4AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc4AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfcDiffText(value: unknown, at = "$"): IfcDiffText {
  return stdioIfc4AnyDiffTextGuardObject(value, `${at}`);
}
