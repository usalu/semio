/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the DIFF PACK-BINARY ENCODING header of the same shape. */
export interface SemioTextDiffFrameHeader {
  format: number;
  presence: number;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TextDiffBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TextDiffBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TextDiffBinaryGuardRefusal(at, why);
};

type stdioSemioV1TextDiffBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TextDiffBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TextDiffBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TextDiffBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TextDiffBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1TextDiffBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TextDiffBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TextDiffBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TextDiffBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TextDiffBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TextDiffBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1TextDiffBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TextDiffBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TextDiffBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TextDiffBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TextDiffBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TextDiffBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TextDiffBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TextDiffBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TextDiffBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TextDiffBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TextDiffBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TextDiffBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TextDiffBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TextDiffBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TextDiffBinaryGuardNumber(value, at, bounds) : stdioSemioV1TextDiffBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1TextDiffBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TextDiffBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TextDiffBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TextDiffBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioTextDiffFrameHeader(value: unknown, at = "$"): SemioTextDiffFrameHeader {
  const row = stdioSemioV1TextDiffBinaryGuardObject(value, at);
  return {
    format: stdioSemioV1TextDiffBinaryGuardInteger(row["format"], `${at}.format`),
    presence: stdioSemioV1TextDiffBinaryGuardInteger(row["presence"], `${at}.presence`),
    payload: stdioSemioV1TextDiffBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
