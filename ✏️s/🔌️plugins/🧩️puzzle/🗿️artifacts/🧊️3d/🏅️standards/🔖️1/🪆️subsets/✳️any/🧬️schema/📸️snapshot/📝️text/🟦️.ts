/** 📝️ Text representation for `puzzle.puzzle3d.snapshot`. */
export type Puzzle3dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dSnapshotTextGuardRefusal(at, why);
};

type puzzlePuzzle3dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dSnapshotTextGuardReject(at, "value is not an object");
export const puzzlePuzzle3dSnapshotTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dSnapshotTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dSnapshotTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dSnapshotTextGuardNumber(value, at, bounds) : puzzlePuzzle3dSnapshotTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dSnapshotText(value: unknown, at = "$"): Puzzle3dSnapshotText {
  return puzzlePuzzle3dSnapshotTextGuardObject(value, `${at}`);
}
