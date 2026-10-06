/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the OP PACK-BINARY ENCODING of the same shape. */
export interface SemioGraphOpFrameHeader {
  format: number;
  tag: number;
  payload: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphMutationsBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphMutationsBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphMutationsBinaryGuardRefusal(at, why);
};

type stdioSemioV1GraphMutationsBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphMutationsBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphMutationsBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphMutationsBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphMutationsBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1GraphMutationsBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphMutationsBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphMutationsBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphMutationsBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphMutationsBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphMutationsBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphMutationsBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphMutationsBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphMutationsBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphMutationsBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphMutationsBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphMutationsBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphMutationsBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphMutationsBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphMutationsBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphMutationsBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphMutationsBinaryGuardNumber(value, at, bounds) : stdioSemioV1GraphMutationsBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphMutationsBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphMutationsBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphMutationsBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphMutationsBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioGraphMutationBinary {
  readonly format?: number;
  readonly tag?: number;
  readonly payload?: string;
}

export function parseSemioGraphMutationBinary(value: unknown, at = "$"): SemioGraphMutationBinary {
  const row = stdioSemioV1GraphMutationsBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1GraphMutationsBinaryGuardInteger(row["format"], `${at}.format`),
    tag: row["tag"] === undefined ? undefined : stdioSemioV1GraphMutationsBinaryGuardInteger(row["tag"], `${at}.tag`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1GraphMutationsBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
