/** 📝️ Text-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the OP-TEXT ENCODING of the mutation dispatch shape. */
export interface SemioTextMutationOpDsl {
  keyword: string;
  argsWire: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TextMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TextMutationsTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TextMutationsTextGuardRefusal(at, why);
};

type stdioSemioV1TextMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TextMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TextMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TextMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TextMutationsTextGuardReject(at, "value is not an object");
export const stdioSemioV1TextMutationsTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TextMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TextMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TextMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TextMutationsTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TextMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TextMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TextMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TextMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TextMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TextMutationsTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TextMutationsTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TextMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TextMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TextMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TextMutationsTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TextMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TextMutationsTextGuardNumber(value, at, bounds) : stdioSemioV1TextMutationsTextGuardReject(at, "value is not an integer");
export const stdioSemioV1TextMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TextMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TextMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TextMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioTextMutationText {
  readonly keyword?: string;
  readonly argsWire?: string;
}

export function parseSemioTextMutationText(value: unknown, at = "$"): SemioTextMutationText {
  const row = stdioSemioV1TextMutationsTextGuardObject(value, at);
  return {
    keyword: row["keyword"] === undefined ? undefined : stdioSemioV1TextMutationsTextGuardString(row["keyword"], `${at}.keyword`),
    argsWire: row["argsWire"] === undefined ? undefined : stdioSemioV1TextMutationsTextGuardString(row["argsWire"], `${at}.argsWire`),
  };
}
