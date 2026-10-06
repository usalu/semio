/** 📝️ Text representation for `stdio.step` (snapshot). */
export type StepSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStepAp214BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStepAp214BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioStepAp214BaseSnapshotTextGuardRefusal(at, why);
};

type stdioStepAp214BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStepAp214BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStepAp214BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStepAp214BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStepAp214BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioStepAp214BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStepAp214BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStepAp214BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStepAp214BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStepAp214BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStepAp214BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStepAp214BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStepAp214BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStepAp214BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStepAp214BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStepAp214BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioStepAp214BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStepAp214BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStepAp214BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStepAp214BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStepAp214BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStepAp214BaseSnapshotTextGuardNumber(value, at, bounds) : stdioStepAp214BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioStepAp214BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStepAp214BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStepAp214BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStepAp214BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStepSnapshotText(value: unknown, at = "$"): StepSnapshotText {
  return stdioStepAp214BaseSnapshotTextGuardObject(value, `${at}`);
}
