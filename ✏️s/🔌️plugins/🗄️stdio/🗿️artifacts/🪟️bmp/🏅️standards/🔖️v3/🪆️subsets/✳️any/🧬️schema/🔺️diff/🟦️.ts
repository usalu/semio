/** 🔺️ BmpDiff schema facet — mirrors 🦀️.rs field-for-field. No full-replace slot:
 * every BITMAPINFOHEADER field is a sparse patch scalar, `palette` is an index-keyed
 * removed/modified/added triple, `pixels` is a whole-buffer replace. */

export interface BmpPaletteModified {
  index: number;
  entry: import('../📸️snapshot/🟦️.ts').BmpPaletteEntry;
}

export interface BmpPaletteAdded {
  index: number;
  entry: import('../📸️snapshot/🟦️.ts').BmpPaletteEntry;
}

export interface BmpPaletteDiff {
  removed?: number[];
  modified?: BmpPaletteModified[];
  added?: BmpPaletteAdded[];
}

export interface BmpDiff {
  headerSize?: number;
  width?: number;
  height?: number;
  rowOrder?: import('../📸️snapshot/🟦️.ts').BmpRowOrder;
  planes?: number;
  bitsPerPixel?: number;
  compression?: number;
  imageSize?: number;
  xPixelsPerMeter?: number;
  yPixelsPerMeter?: number;
  colorsUsed?: number;
  colorsImportant?: number;
  palette?: BmpPaletteDiff;
  pixels?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBmpV3AnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBmpV3AnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioBmpV3AnyDiffGuardRefusal(at, why);
};

type stdioBmpV3AnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBmpV3AnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBmpV3AnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBmpV3AnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBmpV3AnyDiffGuardReject(at, "value is not an object");
export const stdioBmpV3AnyDiffGuardArray = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBmpV3AnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBmpV3AnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBmpV3AnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBmpV3AnyDiffGuardString = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBmpV3AnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBmpV3AnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBmpV3AnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBmpV3AnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBmpV3AnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBmpV3AnyDiffGuardReject(at, "value is not a boolean"));
export const stdioBmpV3AnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBmpV3AnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBmpV3AnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBmpV3AnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBmpV3AnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioBmpV3AnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBmpV3AnyDiffGuardNumber(value, at, bounds) : stdioBmpV3AnyDiffGuardReject(at, "value is not an integer");
export const stdioBmpV3AnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBmpV3AnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBmpV3AnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBmpV3AnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBmpPaletteDiff(value: unknown, at = "$"): BmpPaletteDiff {
  const row = stdioBmpV3AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioBmpV3AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioBmpV3AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioBmpV3AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseBmpPaletteModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioBmpV3AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseBmpPaletteAdded(item, `${at}.added[${index}]`)),
  };
}
