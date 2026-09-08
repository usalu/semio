/** 📝️ Text representation for `remodel.remodeling.diff`. */
export type RemodelingDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingDiffTextGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingDiffTextGuardRefusal(at, why);
};

type remodelRemodelingDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingDiffTextGuardReject(at, "value is not an object");
export const remodelRemodelingDiffTextGuardArray = (value: unknown, at: string, bounds: remodelRemodelingDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingDiffTextGuardString = (value: unknown, at: string, bounds: remodelRemodelingDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingDiffTextGuardReject(at, "value is not a boolean"));
export const remodelRemodelingDiffTextGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingDiffTextGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingDiffTextGuardNumber(value, at, bounds) : remodelRemodelingDiffTextGuardReject(at, "value is not an integer");
export const remodelRemodelingDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingDiffText(value: unknown, at = "$"): RemodelingDiffText {
  return remodelRemodelingDiffTextGuardObject(value, `${at}`);
}
