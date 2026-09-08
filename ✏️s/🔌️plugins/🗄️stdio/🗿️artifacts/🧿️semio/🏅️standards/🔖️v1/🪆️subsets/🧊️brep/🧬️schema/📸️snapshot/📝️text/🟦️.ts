/** 📝️ Text (DSL) representation for `stdio.semio.brep` (snapshot): a hex dump of the JSON-pack
 * bytes of a `SemioBrepSnapshot`, preamble-prefixed. */
export type SemioBrepSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1BrepSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1BrepSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1BrepSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1BrepSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1BrepSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1BrepSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1BrepSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1BrepSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1BrepSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1BrepSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1BrepSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1BrepSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1BrepSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1BrepSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1BrepSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1BrepSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1BrepSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1BrepSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1BrepSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1BrepSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1BrepSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1BrepSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1BrepSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1BrepSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1BrepSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1BrepSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1BrepSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1BrepSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1BrepSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1BrepSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1BrepSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1BrepSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1BrepSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1BrepSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioBrepSnapshotText(value: unknown, at = "$"): SemioBrepSnapshotText {
  return stdioSemioV1BrepSnapshotTextGuardObject(value, `${at}`);
}
