export interface SemioKitMutationDsl { line: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitMutationsTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitMutationsTextGuardRefusal(at, why);
};

type stdioSemioV1KitMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitMutationsTextGuardReject(at, "value is not an object");
export const stdioSemioV1KitMutationsTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitMutationsTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitMutationsTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitMutationsTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitMutationsTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitMutationsTextGuardNumber(value, at, bounds) : stdioSemioV1KitMutationsTextGuardReject(at, "value is not an integer");
export const stdioSemioV1KitMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioKitMutationText {
  readonly line?: string;
}

export function parseSemioKitMutationText(value: unknown, at = "$"): SemioKitMutationText {
  const row = stdioSemioV1KitMutationsTextGuardObject(value, at);
  return {
    line: row["line"] === undefined ? undefined : stdioSemioV1KitMutationsTextGuardString(row["line"], `${at}.line`),
  };
}
