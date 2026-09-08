/** 📝️ Text representation for `puzzle.puzzle2d.mutations`. */
export type Puzzle2dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dMutationsTextGuardRefusal(at, why);
};

type puzzlePuzzle2dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dMutationsTextGuardReject(at, "value is not an object");
export const puzzlePuzzle2dMutationsTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dMutationsTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dMutationsTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dMutationsTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dMutationsTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dMutationsTextGuardNumber(value, at, bounds) : puzzlePuzzle2dMutationsTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dMutationsText(value: unknown, at = "$"): Puzzle2dMutationsText {
  return puzzlePuzzle2dMutationsTextGuardObject(value, `${at}`);
}
