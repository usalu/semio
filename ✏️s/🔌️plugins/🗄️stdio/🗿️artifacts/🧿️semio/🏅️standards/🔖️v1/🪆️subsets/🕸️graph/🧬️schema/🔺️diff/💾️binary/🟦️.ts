/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the DIFF PACK-BINARY ENCODING of the same shape. */
export interface SemioGraphDiffFrameHeader {
  format: number;
  presence: number;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphDiffBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphDiffBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphDiffBinaryGuardRefusal(at, why);
};

type stdioSemioV1GraphDiffBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphDiffBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphDiffBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphDiffBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphDiffBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1GraphDiffBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphDiffBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphDiffBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphDiffBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphDiffBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphDiffBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphDiffBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphDiffBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphDiffBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphDiffBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphDiffBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphDiffBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphDiffBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphDiffBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphDiffBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphDiffBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphDiffBinaryGuardNumber(value, at, bounds) : stdioSemioV1GraphDiffBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphDiffBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphDiffBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphDiffBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphDiffBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioGraphDiffFrameHeader(value: unknown, at = "$"): SemioGraphDiffFrameHeader {
  const row = stdioSemioV1GraphDiffBinaryGuardObject(value, at);
  return {
    format: stdioSemioV1GraphDiffBinaryGuardInteger(row["format"], `${at}.format`),
    presence: stdioSemioV1GraphDiffBinaryGuardInteger(row["presence"], `${at}.presence`),
    payload: stdioSemioV1GraphDiffBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
