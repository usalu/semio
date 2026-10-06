/** 💾️ Binary-facet grammar mirror (descriptive) for s.stdio.semio.kit's pack encoding. */
export interface SemioKitSnapshotPackHeader { format: number; schemaLen: number; schemaBytes: string; payload: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitSnapshotBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitSnapshotBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitSnapshotBinaryGuardRefusal(at, why);
};

type stdioSemioV1KitSnapshotBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitSnapshotBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitSnapshotBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitSnapshotBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitSnapshotBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1KitSnapshotBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitSnapshotBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitSnapshotBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitSnapshotBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitSnapshotBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitSnapshotBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitSnapshotBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitSnapshotBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitSnapshotBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitSnapshotBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitSnapshotBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitSnapshotBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitSnapshotBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitSnapshotBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitSnapshotBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitSnapshotBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitSnapshotBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitSnapshotBinaryGuardNumber(value, at, bounds) : stdioSemioV1KitSnapshotBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1KitSnapshotBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitSnapshotBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitSnapshotBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitSnapshotBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioKitSnapshotPackHeader(value: unknown, at = "$"): SemioKitSnapshotPackHeader {
  const row = stdioSemioV1KitSnapshotBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1KitSnapshotBinaryGuardInteger(row["format"], `${at}.format`),
    schemaLen: row["schemaLen"] === undefined ? undefined : stdioSemioV1KitSnapshotBinaryGuardInteger(row["schemaLen"], `${at}.schemaLen`),
    schemaBytes: row["schemaBytes"] === undefined ? undefined : stdioSemioV1KitSnapshotBinaryGuardString(row["schemaBytes"], `${at}.schemaBytes`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1KitSnapshotBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
