/** 📝️ Text representation for `playbook.playbook.diff`. */
export type PlaybookDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookDiffTextGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookDiffTextGuardRefusal(at, why);
};

type playbookPlaybookDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookDiffTextGuardReject(at, "value is not an object");
export const playbookPlaybookDiffTextGuardArray = (value: unknown, at: string, bounds: playbookPlaybookDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookDiffTextGuardString = (value: unknown, at: string, bounds: playbookPlaybookDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookDiffTextGuardReject(at, "value is not a boolean"));
export const playbookPlaybookDiffTextGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookDiffTextGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookDiffTextGuardNumber(value, at, bounds) : playbookPlaybookDiffTextGuardReject(at, "value is not an integer");
export const playbookPlaybookDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookDiffText(value: unknown, at = "$"): PlaybookDiffText {
  return playbookPlaybookDiffTextGuardObject(value, `${at}`);
}
