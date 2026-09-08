/** 📝️ Text representation for `puzzle.puzzle3d.inference`. */
export type Puzzle3dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dInferenceTextGuardRefusal(at, why);
};

type puzzlePuzzle3dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dInferenceTextGuardReject(at, "value is not an object");
export const puzzlePuzzle3dInferenceTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dInferenceTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dInferenceTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dInferenceTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dInferenceTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dInferenceTextGuardNumber(value, at, bounds) : puzzlePuzzle3dInferenceTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dInferenceText(value: unknown, at = "$"): Puzzle3dInferenceText {
  return puzzlePuzzle3dInferenceTextGuardObject(value, `${at}`);
}
