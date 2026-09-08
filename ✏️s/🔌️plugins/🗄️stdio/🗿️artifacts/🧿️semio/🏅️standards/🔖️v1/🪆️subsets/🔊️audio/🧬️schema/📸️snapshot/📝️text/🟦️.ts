/** 📝️ Text representation for `s.stdio.semio.audio` (snapshot). */
export type SemioAudioSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AudioSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AudioSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AudioSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1AudioSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AudioSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AudioSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AudioSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AudioSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1AudioSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AudioSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AudioSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AudioSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AudioSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AudioSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1AudioSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AudioSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AudioSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AudioSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AudioSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AudioSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AudioSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AudioSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AudioSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AudioSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AudioSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AudioSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AudioSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AudioSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AudioSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1AudioSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1AudioSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AudioSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AudioSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AudioSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAudioSnapshotText(value: unknown, at = "$"): SemioAudioSnapshotText {
  return stdioSemioV1AudioSnapshotTextGuardObject(value, `${at}`);
}
