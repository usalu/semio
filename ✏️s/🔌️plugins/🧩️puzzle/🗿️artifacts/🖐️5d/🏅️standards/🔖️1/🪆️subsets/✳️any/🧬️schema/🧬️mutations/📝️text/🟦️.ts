/** 📝️ Text representation for `puzzle.puzzle5d.mutations`. */
export type Puzzle5dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dMutationsTextGuardRefusal(at, why);
};

type puzzlePuzzle5dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dMutationsTextGuardReject(at, "value is not an object");
export const puzzlePuzzle5dMutationsTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dMutationsTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dMutationsTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dMutationsTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dMutationsTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dMutationsTextGuardNumber(value, at, bounds) : puzzlePuzzle5dMutationsTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dMutationsText(value: unknown, at = "$"): Puzzle5dMutationsText {
  return puzzlePuzzle5dMutationsTextGuardObject(value, `${at}`);
}
