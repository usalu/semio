/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the PACK-BINARY ENCODING of the same shape. */
export interface SemioTextSnapshotPackHeader {
  format: number;
  schemaLen: number;
  schemaBytes: string;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TextSnapshotBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TextSnapshotBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TextSnapshotBinaryGuardRefusal(at, why);
};

type stdioSemioV1TextSnapshotBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TextSnapshotBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TextSnapshotBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TextSnapshotBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TextSnapshotBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1TextSnapshotBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TextSnapshotBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TextSnapshotBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TextSnapshotBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TextSnapshotBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TextSnapshotBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1TextSnapshotBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TextSnapshotBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TextSnapshotBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TextSnapshotBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TextSnapshotBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TextSnapshotBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TextSnapshotBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TextSnapshotBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TextSnapshotBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TextSnapshotBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TextSnapshotBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TextSnapshotBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TextSnapshotBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TextSnapshotBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TextSnapshotBinaryGuardNumber(value, at, bounds) : stdioSemioV1TextSnapshotBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1TextSnapshotBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TextSnapshotBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TextSnapshotBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TextSnapshotBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioTextSnapshotPackHeader(value: unknown, at = "$"): SemioTextSnapshotPackHeader {
  const row = stdioSemioV1TextSnapshotBinaryGuardObject(value, at);
  return {
    format: stdioSemioV1TextSnapshotBinaryGuardInteger(row["format"], `${at}.format`),
    schemaLen: stdioSemioV1TextSnapshotBinaryGuardInteger(row["schemaLen"], `${at}.schemaLen`),
    schemaBytes: stdioSemioV1TextSnapshotBinaryGuardString(row["schemaBytes"], `${at}.schemaBytes`),
    payload: stdioSemioV1TextSnapshotBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
