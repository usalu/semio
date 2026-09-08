/** 📝️ Text representation for `stdio.obj` (diff). */
export type ObjDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioObj30GeometryDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioObj30GeometryDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioObj30GeometryDiffTextGuardRefusal(at, why);
};

type stdioObj30GeometryDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioObj30GeometryDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioObj30GeometryDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioObj30GeometryDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioObj30GeometryDiffTextGuardReject(at, "value is not an object");
export const stdioObj30GeometryDiffTextGuardArray = (value: unknown, at: string, bounds: stdioObj30GeometryDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioObj30GeometryDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioObj30GeometryDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioObj30GeometryDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioObj30GeometryDiffTextGuardString = (value: unknown, at: string, bounds: stdioObj30GeometryDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioObj30GeometryDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioObj30GeometryDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioObj30GeometryDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioObj30GeometryDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioObj30GeometryDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioObj30GeometryDiffTextGuardReject(at, "value is not a boolean"));
export const stdioObj30GeometryDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioObj30GeometryDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioObj30GeometryDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioObj30GeometryDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioObj30GeometryDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioObj30GeometryDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioObj30GeometryDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioObj30GeometryDiffTextGuardNumber(value, at, bounds) : stdioObj30GeometryDiffTextGuardReject(at, "value is not an integer");
export const stdioObj30GeometryDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioObj30GeometryDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioObj30GeometryDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioObj30GeometryDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseObjDiffText(value: unknown, at = "$"): ObjDiffText {
  return stdioObj30GeometryDiffTextGuardObject(value, `${at}`);
}
