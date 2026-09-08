/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the PACK-BINARY ENCODING of the same shape. */
export interface SemioObjectSnapshotPackHeader {
  format: number;
  schemaLen: number;
  schemaBytes: string;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectSnapshotBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectSnapshotBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectSnapshotBinaryGuardRefusal(at, why);
};

type stdioSemioV1ObjectSnapshotBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectSnapshotBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectSnapshotBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectSnapshotBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectSnapshotBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectSnapshotBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectSnapshotBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectSnapshotBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectSnapshotBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectSnapshotBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectSnapshotBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectSnapshotBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectSnapshotBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectSnapshotBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectSnapshotBinaryGuardNumber(value, at, bounds) : stdioSemioV1ObjectSnapshotBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectSnapshotBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectSnapshotBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectSnapshotBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectSnapshotPackHeader(value: unknown, at = "$"): SemioObjectSnapshotPackHeader {
  const row = stdioSemioV1ObjectSnapshotBinaryGuardObject(value, at);
  return {
    format: stdioSemioV1ObjectSnapshotBinaryGuardInteger(row["format"], `${at}.format`),
    schemaLen: stdioSemioV1ObjectSnapshotBinaryGuardInteger(row["schemaLen"], `${at}.schemaLen`),
    schemaBytes: stdioSemioV1ObjectSnapshotBinaryGuardString(row["schemaBytes"], `${at}.schemaBytes`),
    payload: stdioSemioV1ObjectSnapshotBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
