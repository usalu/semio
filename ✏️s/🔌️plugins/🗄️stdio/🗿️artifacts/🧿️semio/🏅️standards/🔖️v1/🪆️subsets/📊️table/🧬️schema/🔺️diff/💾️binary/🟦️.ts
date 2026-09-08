/** 💾️ Binary representation codec surface for `stdio.semio.table` (diff) — descriptive twin. */
export const COMPONENT_PROTOCOL_PATH = "🧬️schema/🔺️diff/💾️binary/📡️.protocol.semio";

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TableDiffBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TableDiffBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TableDiffBinaryGuardRefusal(at, why);
};

type stdioSemioV1TableDiffBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TableDiffBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TableDiffBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TableDiffBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TableDiffBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1TableDiffBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TableDiffBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TableDiffBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TableDiffBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TableDiffBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TableDiffBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1TableDiffBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TableDiffBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TableDiffBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TableDiffBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TableDiffBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TableDiffBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TableDiffBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TableDiffBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TableDiffBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TableDiffBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TableDiffBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TableDiffBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TableDiffBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TableDiffBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TableDiffBinaryGuardNumber(value, at, bounds) : stdioSemioV1TableDiffBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1TableDiffBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TableDiffBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TableDiffBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TableDiffBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioTableDiffBinaryFrame {
  readonly format?: number;
  readonly presence?: number;
  readonly payload?: string;
}

export function parseSemioTableDiffBinaryFrame(value: unknown, at = "$"): SemioTableDiffBinaryFrame {
  const row = stdioSemioV1TableDiffBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1TableDiffBinaryGuardInteger(row["format"], `${at}.format`),
    presence: row["presence"] === undefined ? undefined : stdioSemioV1TableDiffBinaryGuardInteger(row["presence"], `${at}.presence`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1TableDiffBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
