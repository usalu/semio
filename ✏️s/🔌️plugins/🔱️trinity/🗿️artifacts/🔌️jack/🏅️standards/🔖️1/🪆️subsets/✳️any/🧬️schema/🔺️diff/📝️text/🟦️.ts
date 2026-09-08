/** 📝️ Text representation for `trinity.jack.diff`. */
export type JackDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackDiffTextGuardReject = (at: string, why: string): never => {
  throw new trinityJackDiffTextGuardRefusal(at, why);
};

type trinityJackDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackDiffTextGuardReject(at, "value is not an object");
export const trinityJackDiffTextGuardArray = (value: unknown, at: string, bounds: trinityJackDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackDiffTextGuardString = (value: unknown, at: string, bounds: trinityJackDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackDiffTextGuardReject(at, "value is not a boolean"));
export const trinityJackDiffTextGuardNumber = (value: unknown, at: string, bounds: trinityJackDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackDiffTextGuardInteger = (value: unknown, at: string, bounds: trinityJackDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackDiffTextGuardNumber(value, at, bounds) : trinityJackDiffTextGuardReject(at, "value is not an integer");
export const trinityJackDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackDiffText(value: unknown, at = "$"): JackDiffText {
  return trinityJackDiffTextGuardObject(value, `${at}`);
}
