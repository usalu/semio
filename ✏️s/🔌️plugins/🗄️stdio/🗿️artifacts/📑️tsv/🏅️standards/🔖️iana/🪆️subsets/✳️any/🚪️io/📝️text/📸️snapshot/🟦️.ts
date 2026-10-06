/** 📝️ Text representation for `stdio.tsv` (snapshot): the real IANA TSV body. */
export type TsvSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnySnapshotTextGuardRefusal(at, why);
};

type stdioTsvIanaAnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnySnapshotTextGuardReject(at, "value is not an object");
export const stdioTsvIanaAnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnySnapshotTextGuardNumber(value, at, bounds) : stdioTsvIanaAnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvSnapshotText(value: unknown, at = "$"): TsvSnapshotText {
  return stdioTsvIanaAnySnapshotTextGuardObject(value, `${at}`);
}
