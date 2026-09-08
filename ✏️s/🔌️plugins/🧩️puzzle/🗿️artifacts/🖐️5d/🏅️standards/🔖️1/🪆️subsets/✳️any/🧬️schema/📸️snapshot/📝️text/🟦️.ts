/** 📝️ Text representation for `puzzle.puzzle5d.snapshot`. */
export type Puzzle5dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dSnapshotTextGuardRefusal(at, why);
};

type puzzlePuzzle5dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dSnapshotTextGuardReject(at, "value is not an object");
export const puzzlePuzzle5dSnapshotTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dSnapshotTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dSnapshotTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dSnapshotTextGuardNumber(value, at, bounds) : puzzlePuzzle5dSnapshotTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dSnapshotText(value: unknown, at = "$"): Puzzle5dSnapshotText {
  return puzzlePuzzle5dSnapshotTextGuardObject(value, `${at}`);
}
