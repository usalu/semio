/** 🧬️ XlsxDiff schema. */
export interface XlsxDiff {
  schema?: string;
  bytes?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXlsxEcma376BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXlsxEcma376BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioXlsxEcma376BaseDiffGuardRefusal(at, why);
};

type stdioXlsxEcma376BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXlsxEcma376BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXlsxEcma376BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXlsxEcma376BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXlsxEcma376BaseDiffGuardReject(at, "value is not an object");
export const stdioXlsxEcma376BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXlsxEcma376BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXlsxEcma376BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXlsxEcma376BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXlsxEcma376BaseDiffGuardString = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXlsxEcma376BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXlsxEcma376BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXlsxEcma376BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXlsxEcma376BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXlsxEcma376BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXlsxEcma376BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioXlsxEcma376BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXlsxEcma376BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXlsxEcma376BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXlsxEcma376BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXlsxEcma376BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXlsxEcma376BaseDiffGuardNumber(value, at, bounds) : stdioXlsxEcma376BaseDiffGuardReject(at, "value is not an integer");
export const stdioXlsxEcma376BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXlsxEcma376BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXlsxEcma376BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXlsxEcma376BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXlsxDiff(value: unknown, at = "$"): XlsxDiff {
  const row = stdioXlsxEcma376BaseDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioXlsxEcma376BaseDiffGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioXlsxEcma376BaseDiffGuardString(row["bytes"], `${at}.bytes`),
  };
}
