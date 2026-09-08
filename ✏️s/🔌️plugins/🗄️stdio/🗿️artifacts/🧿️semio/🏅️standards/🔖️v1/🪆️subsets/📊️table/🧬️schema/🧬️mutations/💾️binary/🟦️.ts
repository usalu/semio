/** 💾️ Binary representation codec surface for `stdio.semio.table` (mutations) — descriptive twin. */
export const COMPONENT_PROTOCOL_PATH = "🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio";

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TableMutationsBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TableMutationsBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TableMutationsBinaryGuardRefusal(at, why);
};

type stdioSemioV1TableMutationsBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TableMutationsBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TableMutationsBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TableMutationsBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TableMutationsBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1TableMutationsBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TableMutationsBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TableMutationsBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TableMutationsBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TableMutationsBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TableMutationsBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1TableMutationsBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TableMutationsBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TableMutationsBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TableMutationsBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TableMutationsBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TableMutationsBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TableMutationsBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TableMutationsBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TableMutationsBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TableMutationsBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TableMutationsBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TableMutationsBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TableMutationsBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TableMutationsBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TableMutationsBinaryGuardNumber(value, at, bounds) : stdioSemioV1TableMutationsBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1TableMutationsBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TableMutationsBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TableMutationsBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TableMutationsBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioTableMutationBinary {
  readonly format?: number;
  readonly tag?: number;
  readonly payload?: string;
}

export function parseSemioTableMutationBinary(value: unknown, at = "$"): SemioTableMutationBinary {
  const row = stdioSemioV1TableMutationsBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1TableMutationsBinaryGuardInteger(row["format"], `${at}.format`),
    tag: row["tag"] === undefined ? undefined : stdioSemioV1TableMutationsBinaryGuardInteger(row["tag"], `${at}.tag`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1TableMutationsBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
