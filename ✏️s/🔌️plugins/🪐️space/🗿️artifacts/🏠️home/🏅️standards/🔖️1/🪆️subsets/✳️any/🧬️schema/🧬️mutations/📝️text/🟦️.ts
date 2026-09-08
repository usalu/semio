/** 📝️ Text representation for `space.home.mutations`. */
export type SHomeMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeMutationsTextGuardReject = (at: string, why: string): never => {
  throw new spaceHomeMutationsTextGuardRefusal(at, why);
};

type spaceHomeMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeMutationsTextGuardReject(at, "value is not an object");
export const spaceHomeMutationsTextGuardArray = (value: unknown, at: string, bounds: spaceHomeMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeMutationsTextGuardString = (value: unknown, at: string, bounds: spaceHomeMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeMutationsTextGuardReject(at, "value is not a boolean"));
export const spaceHomeMutationsTextGuardNumber = (value: unknown, at: string, bounds: spaceHomeMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeMutationsTextGuardInteger = (value: unknown, at: string, bounds: spaceHomeMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeMutationsTextGuardNumber(value, at, bounds) : spaceHomeMutationsTextGuardReject(at, "value is not an integer");
export const spaceHomeMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeMutationsText(value: unknown, at = "$"): SHomeMutationsText {
  return spaceHomeMutationsTextGuardObject(value, `${at}`);
}
