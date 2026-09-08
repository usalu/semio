/** 📝️ Text representation for `puzzle.puzzle5d.diff`. */
export type Puzzle5dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dDiffTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dDiffTextGuardRefusal(at, why);
};

type puzzlePuzzle5dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dDiffTextGuardReject(at, "value is not an object");
export const puzzlePuzzle5dDiffTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dDiffTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dDiffTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dDiffTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dDiffTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dDiffTextGuardNumber(value, at, bounds) : puzzlePuzzle5dDiffTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dDiffText(value: unknown, at = "$"): Puzzle5dDiffText {
  return puzzlePuzzle5dDiffTextGuardObject(value, `${at}`);
}
