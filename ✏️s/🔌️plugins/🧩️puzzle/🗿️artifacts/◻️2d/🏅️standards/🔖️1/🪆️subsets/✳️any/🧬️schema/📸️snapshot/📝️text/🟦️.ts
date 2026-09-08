/** 📝️ Text representation for `puzzle.puzzle2d.snapshot`. */
export type Puzzle2dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dSnapshotTextGuardRefusal(at, why);
};

type puzzlePuzzle2dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dSnapshotTextGuardReject(at, "value is not an object");
export const puzzlePuzzle2dSnapshotTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dSnapshotTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dSnapshotTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dSnapshotTextGuardNumber(value, at, bounds) : puzzlePuzzle2dSnapshotTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dSnapshotText(value: unknown, at = "$"): Puzzle2dSnapshotText {
  return puzzlePuzzle2dSnapshotTextGuardObject(value, `${at}`);
}
