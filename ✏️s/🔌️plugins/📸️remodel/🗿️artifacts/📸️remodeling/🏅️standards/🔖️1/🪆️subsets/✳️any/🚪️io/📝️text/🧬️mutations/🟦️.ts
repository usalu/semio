/** 📝️ Text representation for `remodel.remodeling.mutations`. */
export type RemodelingMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingMutationsTextGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingMutationsTextGuardRefusal(at, why);
};

type remodelRemodelingMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingMutationsTextGuardReject(at, "value is not an object");
export const remodelRemodelingMutationsTextGuardArray = (value: unknown, at: string, bounds: remodelRemodelingMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingMutationsTextGuardString = (value: unknown, at: string, bounds: remodelRemodelingMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingMutationsTextGuardReject(at, "value is not a boolean"));
export const remodelRemodelingMutationsTextGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingMutationsTextGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingMutationsTextGuardNumber(value, at, bounds) : remodelRemodelingMutationsTextGuardReject(at, "value is not an integer");
export const remodelRemodelingMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingMutationsText(value: unknown, at = "$"): RemodelingMutationsText {
  return remodelRemodelingMutationsTextGuardObject(value, `${at}`);
}
