/** 📝️ Text-facet grammar mirror (descriptive) for s.stdio.semio.object.diff's DSL text encoding. */
export interface SemioObjectDiffDsl { line: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectDiffTextGuardRefusal(at, why);
};

type stdioSemioV1ObjectDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectDiffTextGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectDiffTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectDiffTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectDiffTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectDiffTextGuardNumber(value, at, bounds) : stdioSemioV1ObjectDiffTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectDiffDsl(value: unknown, at = "$"): SemioObjectDiffDsl {
  const row = stdioSemioV1ObjectDiffTextGuardObject(value, at);
  return {
    line: row["line"] === undefined ? undefined : stdioSemioV1ObjectDiffTextGuardString(row["line"], `${at}.line`),
  };
}
