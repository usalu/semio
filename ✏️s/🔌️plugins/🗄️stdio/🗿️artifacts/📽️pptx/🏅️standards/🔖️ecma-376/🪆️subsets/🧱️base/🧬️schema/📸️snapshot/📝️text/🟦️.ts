/** 📝️ Text representation for `stdio.pptx` (snapshot). */
export type PptxSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPptxEcma376BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPptxEcma376BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioPptxEcma376BaseSnapshotTextGuardRefusal(at, why);
};

type stdioPptxEcma376BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPptxEcma376BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPptxEcma376BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPptxEcma376BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPptxEcma376BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioPptxEcma376BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioPptxEcma376BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPptxEcma376BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPptxEcma376BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioPptxEcma376BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPptxEcma376BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPptxEcma376BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPptxEcma376BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioPptxEcma376BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioPptxEcma376BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPptxEcma376BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPptxEcma376BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPptxEcma376BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioPptxEcma376BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPptxEcma376BaseSnapshotTextGuardNumber(value, at, bounds) : stdioPptxEcma376BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioPptxEcma376BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPptxEcma376BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPptxEcma376BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPptxEcma376BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePptxSnapshotText(value: unknown, at = "$"): PptxSnapshotText {
  return stdioPptxEcma376BaseSnapshotTextGuardObject(value, `${at}`);
}
