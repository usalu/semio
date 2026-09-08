/** 💾️ Binary-facet grammar mirror (descriptive) for s.stdio.semio.object.mutations. */
export interface SemioObjectMutationFrame { format: number; tag: number; payload: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectMutationsBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectMutationsBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectMutationsBinaryGuardRefusal(at, why);
};

type stdioSemioV1ObjectMutationsBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectMutationsBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectMutationsBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectMutationsBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectMutationsBinaryGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectMutationsBinaryGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectMutationsBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectMutationsBinaryGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectMutationsBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectMutationsBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectMutationsBinaryGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectMutationsBinaryGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectMutationsBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectMutationsBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectMutationsBinaryGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectMutationsBinaryGuardNumber(value, at, bounds) : stdioSemioV1ObjectMutationsBinaryGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectMutationsBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectMutationsBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectMutationsBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectMutationsBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioObjectMutationBinary {
  readonly format?: number;
  readonly tag?: number;
  readonly payload?: string;
}

export function parseSemioObjectMutationBinary(value: unknown, at = "$"): SemioObjectMutationBinary {
  const row = stdioSemioV1ObjectMutationsBinaryGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioSemioV1ObjectMutationsBinaryGuardInteger(row["format"], `${at}.format`),
    tag: row["tag"] === undefined ? undefined : stdioSemioV1ObjectMutationsBinaryGuardInteger(row["tag"], `${at}.tag`),
    payload: row["payload"] === undefined ? undefined : stdioSemioV1ObjectMutationsBinaryGuardString(row["payload"], `${at}.payload`),
  };
}
