/** 📝️ Text representation for `playbook.playbook.mutations`. */
export type PlaybookMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookMutationsTextGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookMutationsTextGuardRefusal(at, why);
};

type playbookPlaybookMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookMutationsTextGuardReject(at, "value is not an object");
export const playbookPlaybookMutationsTextGuardArray = (value: unknown, at: string, bounds: playbookPlaybookMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookMutationsTextGuardString = (value: unknown, at: string, bounds: playbookPlaybookMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookMutationsTextGuardReject(at, "value is not a boolean"));
export const playbookPlaybookMutationsTextGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookMutationsTextGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookMutationsTextGuardNumber(value, at, bounds) : playbookPlaybookMutationsTextGuardReject(at, "value is not an integer");
export const playbookPlaybookMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookMutationsText(value: unknown, at = "$"): PlaybookMutationsText {
  return playbookPlaybookMutationsTextGuardObject(value, `${at}`);
}
