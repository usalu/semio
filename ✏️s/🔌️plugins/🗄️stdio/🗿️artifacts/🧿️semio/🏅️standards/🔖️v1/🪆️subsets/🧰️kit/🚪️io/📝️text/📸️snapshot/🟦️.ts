/** 📝️ Text-facet grammar mirror (descriptive) for s.stdio.semio.kit's DSL text encoding. */
export interface SemioKitSnapshotDsl { line: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1KitSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1KitSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1KitSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1KitSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioKitSnapshotDsl(value: unknown, at = "$"): SemioKitSnapshotDsl {
  const row = stdioSemioV1KitSnapshotTextGuardObject(value, at);
  return {
    line: row["line"] === undefined ? undefined : stdioSemioV1KitSnapshotTextGuardString(row["line"], `${at}.line`),
  };
}
