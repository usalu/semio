/** 📝️ Text representation for `puzzle.puzzle3d.diff`. */
export type Puzzle3dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dDiffTextGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dDiffTextGuardRefusal(at, why);
};

type puzzlePuzzle3dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dDiffTextGuardReject(at, "value is not an object");
export const puzzlePuzzle3dDiffTextGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dDiffTextGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dDiffTextGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dDiffTextGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dDiffTextGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dDiffTextGuardNumber(value, at, bounds) : puzzlePuzzle3dDiffTextGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dDiffText(value: unknown, at = "$"): Puzzle3dDiffText {
  return puzzlePuzzle3dDiffTextGuardObject(value, `${at}`);
}
