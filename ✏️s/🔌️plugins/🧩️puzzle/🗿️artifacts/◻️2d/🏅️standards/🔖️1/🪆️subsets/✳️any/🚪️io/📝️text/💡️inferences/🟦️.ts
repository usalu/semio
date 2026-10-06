/** 📝️ Text representation for `puzzle.puzzle2d.inference`. */
export type Puzzle2dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dInferenceTextGuardRefusal(at, why);
};

type puzzlePuzzle2dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dInferenceTextGuardReject(at, "value is not an object");
export const puzzlePuzzle2dInferenceTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dInferenceTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dInferenceTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dInferenceTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dInferenceTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dInferenceTextGuardNumber(value, at, bounds) : puzzlePuzzle2dInferenceTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dInferenceText(value: unknown, at = "$"): Puzzle2dInferenceText {
  return puzzlePuzzle2dInferenceTextGuardObject(value, `${at}`);
}
