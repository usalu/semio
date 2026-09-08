/** 💾️ Binary-facet grammar mirror (descriptive) for s.stdio.semio.object.diff's pack encoding. */
export interface SemioObjectDiffFrame { format: number; presence: number; payload: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectDiffBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectDiffBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectDiffBinaryGuardRefusal(at, why);
};

type stdioSemioV1ObjectDiffBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectDiffBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectDiffBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectDiffBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectDiffBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectDiffBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectDiffBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectDiffBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectDiffBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectDiffBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectDiffBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectDiffBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectDiffBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectDiffBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectDiffBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectDiffBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectDiffBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectDiffBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectDiffBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectDiffBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectDiffBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectDiffBinaryGuardNumber(value, at, bounds) : stdioSemioV1ObjectDiffBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectDiffBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectDiffBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectDiffBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectDiffBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectDiffFrame(value: unknown, at = "$"): SemioObjectDiffFrame {
  const row = stdioSemioV1ObjectDiffBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1ObjectDiffBinaryGuardInteger(row["format"], `${at}.format`),
    presence: row["presence"] === undefined ? undefined : stdioSemioV1ObjectDiffBinaryGuardInteger(row["presence"], `${at}.presence`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1ObjectDiffBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
