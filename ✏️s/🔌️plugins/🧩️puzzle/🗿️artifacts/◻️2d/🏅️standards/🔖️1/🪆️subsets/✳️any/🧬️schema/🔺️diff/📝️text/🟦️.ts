/** 📝️ Text representation for `puzzle.puzzle2d.diff`. */
export type Puzzle2dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dDiffTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dDiffTextGuardRefusal(at, why);
};

type puzzlePuzzle2dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dDiffTextGuardReject(at, "value is not an object");
export const puzzlePuzzle2dDiffTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dDiffTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dDiffTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dDiffTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dDiffTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dDiffTextGuardNumber(value, at, bounds) : puzzlePuzzle2dDiffTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dDiffText(value: unknown, at = "$"): Puzzle2dDiffText {
  return puzzlePuzzle2dDiffTextGuardObject(value, `${at}`);
}
