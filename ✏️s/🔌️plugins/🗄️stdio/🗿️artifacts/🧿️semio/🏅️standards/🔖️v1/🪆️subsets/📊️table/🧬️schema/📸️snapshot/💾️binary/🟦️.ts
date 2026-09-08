/** 💾️ Binary representation codec surface for `stdio.semio.table` (snapshot) — descriptive twin. */
export const COMPONENT_PROTOCOL_PATH = "🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio";

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TableSnapshotBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TableSnapshotBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TableSnapshotBinaryGuardRefusal(at, why);
};

type stdioSemioV1TableSnapshotBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TableSnapshotBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TableSnapshotBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TableSnapshotBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TableSnapshotBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1TableSnapshotBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TableSnapshotBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TableSnapshotBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TableSnapshotBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TableSnapshotBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TableSnapshotBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1TableSnapshotBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TableSnapshotBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TableSnapshotBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TableSnapshotBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TableSnapshotBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TableSnapshotBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TableSnapshotBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TableSnapshotBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TableSnapshotBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TableSnapshotBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TableSnapshotBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TableSnapshotBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TableSnapshotBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TableSnapshotBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TableSnapshotBinaryGuardNumber(value, at, bounds) : stdioSemioV1TableSnapshotBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1TableSnapshotBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TableSnapshotBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TableSnapshotBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TableSnapshotBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioTableSnapshotBinaryFrame {
  readonly format?: number;
  readonly schemaLen?: number;
  readonly schemaBytes?: string;
  readonly payload?: string;
}

export function parseSemioTableSnapshotBinaryFrame(value: unknown, at = "$"): SemioTableSnapshotBinaryFrame {
  const row = stdioSemioV1TableSnapshotBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1TableSnapshotBinaryGuardInteger(row["format"], `${at}.format`),
    schemaLen: row["schemaLen"] === undefined ? undefined : stdioSemioV1TableSnapshotBinaryGuardInteger(row["schemaLen"], `${at}.schemaLen`),
    schemaBytes: row["schemaBytes"] === undefined ? undefined : stdioSemioV1TableSnapshotBinaryGuardString(row["schemaBytes"], `${at}.schemaBytes`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1TableSnapshotBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
