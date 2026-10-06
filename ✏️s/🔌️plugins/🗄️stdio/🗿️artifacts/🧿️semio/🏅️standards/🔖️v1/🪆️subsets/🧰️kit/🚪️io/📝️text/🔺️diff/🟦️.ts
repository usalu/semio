export interface SemioKitDiffDsl { line: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1KitDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1KitDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1KitDiffTextGuardRefusal(at, why);
};

type stdioSemioV1KitDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1KitDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1KitDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1KitDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1KitDiffTextGuardReject(at, "value is not an object");
export const stdioSemioV1KitDiffTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1KitDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1KitDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1KitDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1KitDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1KitDiffTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1KitDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1KitDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1KitDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1KitDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1KitDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1KitDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1KitDiffTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1KitDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1KitDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1KitDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1KitDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1KitDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1KitDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1KitDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1KitDiffTextGuardNumber(value, at, bounds) : stdioSemioV1KitDiffTextGuardReject(at, "value is not an integer");
export const stdioSemioV1KitDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1KitDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1KitDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1KitDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioKitDiffDsl(value: unknown, at = "$"): SemioKitDiffDsl {
  const row = stdioSemioV1KitDiffTextGuardObject(value, at);
  return {
    line: row["line"] === undefined ? undefined : stdioSemioV1KitDiffTextGuardString(row["line"], `${at}.line`),
  };
}
