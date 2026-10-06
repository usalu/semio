export interface SemioKitDiffFrame { format: number; presence: number; payload: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitDiffBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitDiffBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitDiffBinaryGuardRefusal(at, why);
};

type stdioSemioV1KitDiffBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitDiffBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitDiffBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitDiffBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitDiffBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1KitDiffBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitDiffBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitDiffBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitDiffBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitDiffBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitDiffBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitDiffBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitDiffBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitDiffBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitDiffBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitDiffBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitDiffBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitDiffBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitDiffBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitDiffBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitDiffBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitDiffBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitDiffBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitDiffBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitDiffBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitDiffBinaryGuardNumber(value, at, bounds) : stdioSemioV1KitDiffBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1KitDiffBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitDiffBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitDiffBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitDiffBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioKitDiffFrame(value: unknown, at = "$"): SemioKitDiffFrame {
  const row = stdioSemioV1KitDiffBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1KitDiffBinaryGuardInteger(row["format"], `${at}.format`),
    presence: row["presence"] === undefined ? undefined : stdioSemioV1KitDiffBinaryGuardInteger(row["presence"], `${at}.presence`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1KitDiffBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
