/** 🧬️ DocxDiff schema. */
export interface DocxDiff {
  schema?: string;
  bytes?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDocxEcma376BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDocxEcma376BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioDocxEcma376BaseDiffGuardRefusal(at, why);
};

type stdioDocxEcma376BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDocxEcma376BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDocxEcma376BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDocxEcma376BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDocxEcma376BaseDiffGuardReject(at, "value is not an object");
export const stdioDocxEcma376BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDocxEcma376BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDocxEcma376BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDocxEcma376BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDocxEcma376BaseDiffGuardString = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDocxEcma376BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDocxEcma376BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDocxEcma376BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDocxEcma376BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDocxEcma376BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDocxEcma376BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioDocxEcma376BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDocxEcma376BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDocxEcma376BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDocxEcma376BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDocxEcma376BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioDocxEcma376BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDocxEcma376BaseDiffGuardNumber(value, at, bounds) : stdioDocxEcma376BaseDiffGuardReject(at, "value is not an integer");
export const stdioDocxEcma376BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDocxEcma376BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDocxEcma376BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDocxEcma376BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDocxDiff(value: unknown, at = "$"): DocxDiff {
  const row = stdioDocxEcma376BaseDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioDocxEcma376BaseDiffGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioDocxEcma376BaseDiffGuardString(row["bytes"], `${at}.bytes`),
  };
}
