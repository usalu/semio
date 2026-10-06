/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the OP PACK-BINARY ENCODING header of the mutation dispatch shape. */
export interface SemioTextMutationOpFrameHeader {
  format: number;
  tag: number;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TextMutationsBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TextMutationsBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TextMutationsBinaryGuardRefusal(at, why);
};

type stdioSemioV1TextMutationsBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TextMutationsBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TextMutationsBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TextMutationsBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TextMutationsBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1TextMutationsBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TextMutationsBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TextMutationsBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TextMutationsBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TextMutationsBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TextMutationsBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TextMutationsBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TextMutationsBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TextMutationsBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TextMutationsBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TextMutationsBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TextMutationsBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TextMutationsBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TextMutationsBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TextMutationsBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TextMutationsBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TextMutationsBinaryGuardNumber(value, at, bounds) : stdioSemioV1TextMutationsBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1TextMutationsBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TextMutationsBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TextMutationsBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TextMutationsBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioTextMutationBinary {
  readonly format?: number;
  readonly tag?: number;
  readonly payload?: string;
}

export function parseSemioTextMutationBinary(value: unknown, at = "$"): SemioTextMutationBinary {
  const row = stdioSemioV1TextMutationsBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1TextMutationsBinaryGuardInteger(row["format"], `${at}.format`),
    tag: row["tag"] === undefined ? undefined : stdioSemioV1TextMutationsBinaryGuardInteger(row["tag"], `${at}.tag`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1TextMutationsBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
