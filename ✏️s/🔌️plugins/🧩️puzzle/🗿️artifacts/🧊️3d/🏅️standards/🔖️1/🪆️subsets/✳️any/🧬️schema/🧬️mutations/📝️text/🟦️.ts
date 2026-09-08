/** 📝️ Text representation for `puzzle.puzzle3d.mutations`. */
export type Puzzle3dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dMutationsTextGuardRefusal(at, why);
};

type puzzlePuzzle3dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dMutationsTextGuardReject(at, "value is not an object");
export const puzzlePuzzle3dMutationsTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dMutationsTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dMutationsTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dMutationsTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dMutationsTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dMutationsTextGuardNumber(value, at, bounds) : puzzlePuzzle3dMutationsTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dMutationsText(value: unknown, at = "$"): Puzzle3dMutationsText {
  return puzzlePuzzle3dMutationsTextGuardObject(value, `${at}`);
}
