/** 📝️ Text representation for `playbook.playbook.snapshot`. */
export type PlaybookSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookSnapshotTextGuardRefusal(at, why);
};

type playbookPlaybookSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookSnapshotTextGuardReject(at, "value is not an object");
export const playbookPlaybookSnapshotTextGuardArray = (value: unknown, at: string, bounds: playbookPlaybookSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookSnapshotTextGuardString = (value: unknown, at: string, bounds: playbookPlaybookSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookSnapshotTextGuardReject(at, "value is not a boolean"));
export const playbookPlaybookSnapshotTextGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookSnapshotTextGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookSnapshotTextGuardNumber(value, at, bounds) : playbookPlaybookSnapshotTextGuardReject(at, "value is not an integer");
export const playbookPlaybookSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookSnapshotText(value: unknown, at = "$"): PlaybookSnapshotText {
  return playbookPlaybookSnapshotTextGuardObject(value, `${at}`);
}
