/** 🧬️ BmpSnapshot schema facet — mirrors 🦀️.rs field-for-field. */

/** 📐 BITMAPINFOHEADER's signed `height` field: `bottomUp` (positive, the common case) or
 * `topDown` (negative). Decoded `pixels` are always canonicalized to row 0 = image top
 * regardless of this value. */
export type BmpRowOrder = 'bottomUp' | 'topDown';

/** 🎨 One BITMAPINFOHEADER color-table entry, on-disk field order (present when
 * `bitsPerPixel <= 8`) — a weak/value entity, whole-value replaced in diffs. */
export interface BmpPaletteEntry {
  b: number;
  g: number;
  r: number;
  reserved: number;
}

/** 📸️ Persisted `stdio.bmp` snapshot: full BITMAPINFOHEADER (11 real fields) + palette +
 * decoded canonical 8-bit RGBA `pixels` (`width * height * 4` bytes, row 0 = image top). */
export interface BmpSnapshot {
  schema: string;
  headerSize: number;
  width: number;
  height: number;
  rowOrder: BmpRowOrder;
  planes: number;
  bitsPerPixel: number;
  compression: number;
  imageSize: number;
  xPixelsPerMeter: number;
  yPixelsPerMeter: number;
  colorsUsed: number;
  colorsImportant: number;
  palette: BmpPaletteEntry[];
  pixels: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBmpV3AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBmpV3AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioBmpV3AnySnapshotGuardRefusal(at, why);
};

type stdioBmpV3AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBmpV3AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBmpV3AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBmpV3AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBmpV3AnySnapshotGuardReject(at, "value is not an object");
export const stdioBmpV3AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioBmpV3AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBmpV3AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBmpV3AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBmpV3AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBmpV3AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioBmpV3AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBmpV3AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBmpV3AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBmpV3AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBmpV3AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBmpV3AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBmpV3AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioBmpV3AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioBmpV3AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBmpV3AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBmpV3AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBmpV3AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBmpV3AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioBmpV3AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBmpV3AnySnapshotGuardNumber(value, at, bounds) : stdioBmpV3AnySnapshotGuardReject(at, "value is not an integer");
export const stdioBmpV3AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBmpV3AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBmpV3AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBmpV3AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBmpSnapshot(value: unknown, at = "$"): BmpSnapshot {
  const row = stdioBmpV3AnySnapshotGuardObject(value, at);
  return {
    schema: stdioBmpV3AnySnapshotGuardString(row["schema"], `${at}.schema`),
    headerSize: stdioBmpV3AnySnapshotGuardInteger(row["headerSize"], `${at}.headerSize`, {"minimum": 0}),
    width: stdioBmpV3AnySnapshotGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioBmpV3AnySnapshotGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    rowOrder: parseBmpRowOrder(row["rowOrder"], `${at}.rowOrder`),
    planes: stdioBmpV3AnySnapshotGuardInteger(row["planes"], `${at}.planes`, {"minimum": 0}),
    bitsPerPixel: stdioBmpV3AnySnapshotGuardInteger(row["bitsPerPixel"], `${at}.bitsPerPixel`, {"minimum": 0}),
    compression: stdioBmpV3AnySnapshotGuardInteger(row["compression"], `${at}.compression`, {"minimum": 0}),
    imageSize: stdioBmpV3AnySnapshotGuardInteger(row["imageSize"], `${at}.imageSize`, {"minimum": 0}),
    xPixelsPerMeter: stdioBmpV3AnySnapshotGuardInteger(row["xPixelsPerMeter"], `${at}.xPixelsPerMeter`),
    yPixelsPerMeter: stdioBmpV3AnySnapshotGuardInteger(row["yPixelsPerMeter"], `${at}.yPixelsPerMeter`),
    colorsUsed: stdioBmpV3AnySnapshotGuardInteger(row["colorsUsed"], `${at}.colorsUsed`, {"minimum": 0}),
    colorsImportant: stdioBmpV3AnySnapshotGuardInteger(row["colorsImportant"], `${at}.colorsImportant`, {"minimum": 0}),
    palette: stdioBmpV3AnySnapshotGuardArray(row["palette"], `${at}.palette`).map((item, index) => parseBmpPaletteEntry(item, `${at}.palette[${index}]`)),
    pixels: stdioBmpV3AnySnapshotGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => stdioBmpV3AnySnapshotGuardInteger(item, `${at}.pixels[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}

export function parseBmpRowOrder(value: unknown, at = "$"): BmpRowOrder {
  return stdioBmpV3AnySnapshotGuardMember(value, `${at}`, ["bottomUp", "topDown"] as const);
}

export function parseBmpPaletteEntry(value: unknown, at = "$"): BmpPaletteEntry {
  const row = stdioBmpV3AnySnapshotGuardObject(value, at);
  return {
    b: stdioBmpV3AnySnapshotGuardInteger(row["b"], `${at}.b`, {"minimum": 0, "maximum": 255}),
    g: stdioBmpV3AnySnapshotGuardInteger(row["g"], `${at}.g`, {"minimum": 0, "maximum": 255}),
    r: stdioBmpV3AnySnapshotGuardInteger(row["r"], `${at}.r`, {"minimum": 0, "maximum": 255}),
    reserved: stdioBmpV3AnySnapshotGuardInteger(row["reserved"], `${at}.reserved`, {"minimum": 0, "maximum": 255}),
  };
}
