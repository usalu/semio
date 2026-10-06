/** 📝️ Text representation for `space.home.diff`. */
export type SHomeDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeDiffTextGuardReject = (at: string, why: string): never => {
  throw new spaceHomeDiffTextGuardRefusal(at, why);
};

type spaceHomeDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeDiffTextGuardReject(at, "value is not an object");
export const spaceHomeDiffTextGuardArray = (value: unknown, at: string, bounds: spaceHomeDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeDiffTextGuardString = (value: unknown, at: string, bounds: spaceHomeDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeDiffTextGuardReject(at, "value is not a boolean"));
export const spaceHomeDiffTextGuardNumber = (value: unknown, at: string, bounds: spaceHomeDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeDiffTextGuardInteger = (value: unknown, at: string, bounds: spaceHomeDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeDiffTextGuardNumber(value, at, bounds) : spaceHomeDiffTextGuardReject(at, "value is not an integer");
export const spaceHomeDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeDiffText(value: unknown, at = "$"): SHomeDiffText {
  return spaceHomeDiffTextGuardObject(value, `${at}`);
}
