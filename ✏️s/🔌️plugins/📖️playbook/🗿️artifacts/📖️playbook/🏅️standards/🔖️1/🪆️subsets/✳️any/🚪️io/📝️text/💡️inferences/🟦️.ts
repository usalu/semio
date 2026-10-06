/** 📝️ Text representation for `playbook.playbook.inference`. */
export type PlaybookInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookInferenceTextGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookInferenceTextGuardRefusal(at, why);
};

type playbookPlaybookInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookInferenceTextGuardReject(at, "value is not an object");
export const playbookPlaybookInferenceTextGuardArray = (value: unknown, at: string, bounds: playbookPlaybookInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookInferenceTextGuardString = (value: unknown, at: string, bounds: playbookPlaybookInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookInferenceTextGuardReject(at, "value is not a boolean"));
export const playbookPlaybookInferenceTextGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookInferenceTextGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookInferenceTextGuardNumber(value, at, bounds) : playbookPlaybookInferenceTextGuardReject(at, "value is not an integer");
export const playbookPlaybookInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookInferenceText(value: unknown, at = "$"): PlaybookInferenceText {
  return playbookPlaybookInferenceTextGuardObject(value, `${at}`);
}
