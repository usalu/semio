/** 🧬️ GifDiff schema. */
export interface GifDiff {
  schema?: string;
  bytes?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif89aBaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif89aBaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioGif89aBaseDiffGuardRefusal(at, why);
};

type stdioGif89aBaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif89aBaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif89aBaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif89aBaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif89aBaseDiffGuardReject(at, "value is not an object");
export const stdioGif89aBaseDiffGuardArray = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif89aBaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif89aBaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif89aBaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif89aBaseDiffGuardString = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif89aBaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif89aBaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif89aBaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif89aBaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif89aBaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif89aBaseDiffGuardReject(at, "value is not a boolean"));
export const stdioGif89aBaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif89aBaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif89aBaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif89aBaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif89aBaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif89aBaseDiffGuardNumber(value, at, bounds) : stdioGif89aBaseDiffGuardReject(at, "value is not an integer");
export const stdioGif89aBaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif89aBaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif89aBaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif89aBaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifDiff(value: unknown, at = "$"): GifDiff {
  const row = stdioGif89aBaseDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioGif89aBaseDiffGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioGif89aBaseDiffGuardString(row["bytes"], `${at}.bytes`),
  };
}
