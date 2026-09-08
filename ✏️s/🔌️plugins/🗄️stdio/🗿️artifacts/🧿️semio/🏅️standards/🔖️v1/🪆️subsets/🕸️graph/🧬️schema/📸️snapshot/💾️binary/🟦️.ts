/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the PACK-BINARY ENCODING of the same shape. */
export interface SemioGraphSnapshotPackHeader {
  format: number;
  schemaLen: number;
  schemaBytes: string;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphSnapshotBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphSnapshotBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphSnapshotBinaryGuardRefusal(at, why);
};

type stdioSemioV1GraphSnapshotBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphSnapshotBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphSnapshotBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphSnapshotBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphSnapshotBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1GraphSnapshotBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphSnapshotBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphSnapshotBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphSnapshotBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphSnapshotBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphSnapshotBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphSnapshotBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphSnapshotBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphSnapshotBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphSnapshotBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphSnapshotBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphSnapshotBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphSnapshotBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphSnapshotBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphSnapshotBinaryGuardNumber(value, at, bounds) : stdioSemioV1GraphSnapshotBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphSnapshotBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphSnapshotBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphSnapshotBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphSnapshotBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioGraphSnapshotPackHeader(value: unknown, at = "$"): SemioGraphSnapshotPackHeader {
  const row = stdioSemioV1GraphSnapshotBinaryGuardObject(value, at);
  return {
    format: stdioSemioV1GraphSnapshotBinaryGuardInteger(row["format"], `${at}.format`),
    schemaLen: stdioSemioV1GraphSnapshotBinaryGuardInteger(row["schemaLen"], `${at}.schemaLen`),
    schemaBytes: stdioSemioV1GraphSnapshotBinaryGuardString(row["schemaBytes"], `${at}.schemaBytes`),
    payload: stdioSemioV1GraphSnapshotBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
