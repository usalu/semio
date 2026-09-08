/** 📝️ Text-facet grammar mirror (descriptive) for s.stdio.semio.object.mutations. */
export interface SemioObjectMutationDsl { line: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectMutationsTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectMutationsTextGuardRefusal(at, why);
};

type stdioSemioV1ObjectMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectMutationsTextGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectMutationsTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectMutationsTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectMutationsTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectMutationsTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectMutationsTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectMutationsTextGuardNumber(value, at, bounds) : stdioSemioV1ObjectMutationsTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioObjectMutationText {
  readonly line?: string;
}

export function parseSemioObjectMutationText(value: unknown, at = "$"): SemioObjectMutationText {
  const row = stdioSemioV1ObjectMutationsTextGuardObject(value, at);
  return {
    line: row["line"] === undefined ? undefined : stdioSemioV1ObjectMutationsTextGuardString(row["line"], `${at}.line`),
  };
}
