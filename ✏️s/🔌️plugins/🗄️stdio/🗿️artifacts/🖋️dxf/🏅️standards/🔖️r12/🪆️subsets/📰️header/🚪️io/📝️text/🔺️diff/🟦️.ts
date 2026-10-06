/** 📝️ Text representation for `stdio.dxf` (diff). */
export type DxfDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDxfR12HeaderDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDxfR12HeaderDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioDxfR12HeaderDiffTextGuardRefusal(at, why);
};

type stdioDxfR12HeaderDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDxfR12HeaderDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDxfR12HeaderDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDxfR12HeaderDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDxfR12HeaderDiffTextGuardReject(at, "value is not an object");
export const stdioDxfR12HeaderDiffTextGuardArray = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDxfR12HeaderDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDxfR12HeaderDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDxfR12HeaderDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDxfR12HeaderDiffTextGuardString = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDxfR12HeaderDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDxfR12HeaderDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDxfR12HeaderDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDxfR12HeaderDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDxfR12HeaderDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDxfR12HeaderDiffTextGuardReject(at, "value is not a boolean"));
export const stdioDxfR12HeaderDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDxfR12HeaderDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDxfR12HeaderDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDxfR12HeaderDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDxfR12HeaderDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDxfR12HeaderDiffTextGuardNumber(value, at, bounds) : stdioDxfR12HeaderDiffTextGuardReject(at, "value is not an integer");
export const stdioDxfR12HeaderDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDxfR12HeaderDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDxfR12HeaderDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDxfR12HeaderDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDxfDiffText(value: unknown, at = "$"): DxfDiffText {
  return stdioDxfR12HeaderDiffTextGuardObject(value, `${at}`);
}
