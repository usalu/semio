/** 📝️ Text representation for `puzzle.puzzle5d.inference`. */
export type Puzzle5dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dInferenceTextGuardRefusal(at, why);
};

type puzzlePuzzle5dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dInferenceTextGuardReject(at, "value is not an object");
export const puzzlePuzzle5dInferenceTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dInferenceTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dInferenceTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dInferenceTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dInferenceTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dInferenceTextGuardNumber(value, at, bounds) : puzzlePuzzle5dInferenceTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dInferenceText(value: unknown, at = "$"): Puzzle5dInferenceText {
  return puzzlePuzzle5dInferenceTextGuardObject(value, `${at}`);
}
