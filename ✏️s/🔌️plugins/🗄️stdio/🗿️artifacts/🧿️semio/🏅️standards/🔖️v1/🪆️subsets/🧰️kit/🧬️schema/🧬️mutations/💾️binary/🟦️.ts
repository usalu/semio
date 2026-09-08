export interface SemioKitMutationFrame { format: number; tag: number; payload: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitMutationsBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitMutationsBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitMutationsBinaryGuardRefusal(at, why);
};

type stdioSemioV1KitMutationsBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitMutationsBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitMutationsBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitMutationsBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitMutationsBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1KitMutationsBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitMutationsBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitMutationsBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitMutationsBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitMutationsBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitMutationsBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitMutationsBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitMutationsBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitMutationsBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitMutationsBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitMutationsBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitMutationsBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitMutationsBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitMutationsBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitMutationsBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitMutationsBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitMutationsBinaryGuardNumber(value, at, bounds) : stdioSemioV1KitMutationsBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1KitMutationsBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitMutationsBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitMutationsBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitMutationsBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioKitMutationBinary {
  readonly format?: number;
  readonly tag?: number;
  readonly payload?: string;
}

export function parseSemioKitMutationBinary(value: unknown, at = "$"): SemioKitMutationBinary {
  const row = stdioSemioV1KitMutationsBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1KitMutationsBinaryGuardInteger(row["format"], `${at}.format`),
    tag: row["tag"] === undefined ? undefined : stdioSemioV1KitMutationsBinaryGuardInteger(row["tag"], `${at}.tag`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1KitMutationsBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
