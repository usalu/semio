/** 📝️ Text-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the OP-TEXT ENCODING of the same shape. */
export interface SemioGraphOpDsl {
  keyword: string;
  args: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphMutationsTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphMutationsTextGuardRefusal(at, why);
};

type stdioSemioV1GraphMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphMutationsTextGuardReject(at, "value is not an object");
export const stdioSemioV1GraphMutationsTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphMutationsTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphMutationsTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphMutationsTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphMutationsTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphMutationsTextGuardNumber(value, at, bounds) : stdioSemioV1GraphMutationsTextGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioGraphMutationText {
  readonly keyword?: string;
  readonly args?: string;
}

export function parseSemioGraphMutationText(value: unknown, at = "$"): SemioGraphMutationText {
  const row = stdioSemioV1GraphMutationsTextGuardObject(value, at);
  return {
    keyword: row["keyword"] === undefined ? undefined : stdioSemioV1GraphMutationsTextGuardString(row["keyword"], `${at}.keyword`),
    args: row["args"] === undefined ? undefined : stdioSemioV1GraphMutationsTextGuardString(row["args"], `${at}.args`),
  };
}
