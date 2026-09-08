/** 📝️ Text representation for `raster.raster.snapshot`. */
export type RasterSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new rasterRasterSnapshotTextGuardRefusal(at, why);
};

type rasterRasterSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterSnapshotTextGuardReject(at, "value is not an object");
export const rasterRasterSnapshotTextGuardArray = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterSnapshotTextGuardString = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterSnapshotTextGuardReject(at, "value is not a boolean"));
export const rasterRasterSnapshotTextGuardNumber = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterSnapshotTextGuardInteger = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterSnapshotTextGuardNumber(value, at, bounds) : rasterRasterSnapshotTextGuardReject(at, "value is not an integer");
export const rasterRasterSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterSnapshotText(value: unknown, at = "$"): RasterSnapshotText {
  return rasterRasterSnapshotTextGuardObject(value, `${at}`);
}
