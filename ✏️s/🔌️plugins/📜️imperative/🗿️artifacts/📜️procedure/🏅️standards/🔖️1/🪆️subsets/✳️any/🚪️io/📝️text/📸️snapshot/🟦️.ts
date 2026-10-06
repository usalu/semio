/** 📝️ Text representation for `imperative.procedure.snapshot`. */
export type ProcedureSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeSnapshotTextGuardRefusal(at, why);
};

type imperativeImperativeSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeSnapshotTextGuardReject(at, "value is not an object");
export const imperativeImperativeSnapshotTextGuardArray = (value: unknown, at: string, bounds: imperativeImperativeSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeSnapshotTextGuardString = (value: unknown, at: string, bounds: imperativeImperativeSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeSnapshotTextGuardReject(at, "value is not a boolean"));
export const imperativeImperativeSnapshotTextGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeSnapshotTextGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeSnapshotTextGuardNumber(value, at, bounds) : imperativeImperativeSnapshotTextGuardReject(at, "value is not an integer");
export const imperativeImperativeSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureSnapshotText(value: unknown, at = "$"): ProcedureSnapshotText {
  return imperativeImperativeSnapshotTextGuardObject(value, `${at}`);
}
