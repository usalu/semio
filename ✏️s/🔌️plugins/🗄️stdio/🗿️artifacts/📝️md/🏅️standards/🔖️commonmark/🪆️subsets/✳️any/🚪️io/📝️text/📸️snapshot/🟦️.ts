/** 📝️ Text representation for `stdio.md` (snapshot). */
export type MdSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnySnapshotTextGuardRefusal(at, why);
};

type stdioMdCommonmarkAnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnySnapshotTextGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnySnapshotTextGuardNumber(value, at, bounds) : stdioMdCommonmarkAnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdSnapshotText(value: unknown, at = "$"): MdSnapshotText {
  return stdioMdCommonmarkAnySnapshotTextGuardObject(value, `${at}`);
}
