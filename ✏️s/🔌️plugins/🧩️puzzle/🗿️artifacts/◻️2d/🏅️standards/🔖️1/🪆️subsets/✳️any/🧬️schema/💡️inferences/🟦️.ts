/** 💡️ Puzzle2d inference schema — flatPosition (BFS-resolved node position) per node. */

export interface Puzzle2dFlatPositionXy {
  x: number;
  y: number;
}

export interface Puzzle2dFlatPosition {
  positions: Record<string, Puzzle2dFlatPositionXy>;
}

export interface Puzzle2dInference {
  /** @derived */
  flatPosition: Puzzle2dFlatPosition;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dInferenceGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dInferenceGuardRefusal(at, why);
};

type puzzlePuzzle2dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dInferenceGuardReject(at, "value is not an object");
export const puzzlePuzzle2dInferenceGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dInferenceGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dInferenceGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dInferenceGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dInferenceGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dInferenceGuardNumber(value, at, bounds) : puzzlePuzzle2dInferenceGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dInference(value: unknown, at = "$"): Puzzle2dInference {
  const row = puzzlePuzzle2dInferenceGuardObject(value, at);
  return {
    flatPosition: parsePuzzle2dFlatPosition(row["flatPosition"], `${at}.flatPosition`),
  };
}

export function parsePuzzle2dFlatPositionXy(value: unknown, at = "$"): Puzzle2dFlatPositionXy {
  const row = puzzlePuzzle2dInferenceGuardObject(value, at);
  return {
    x: puzzlePuzzle2dInferenceGuardNumber(row["x"], `${at}.x`),
    y: puzzlePuzzle2dInferenceGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePuzzle2dFlatPosition(value: unknown, at = "$"): Puzzle2dFlatPosition {
  const row = puzzlePuzzle2dInferenceGuardObject(value, at);
  return {
    positions: puzzlePuzzle2dInferenceGuardObject(row["positions"], `${at}.positions`),
  };
}
