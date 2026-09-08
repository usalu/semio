/** 💡️ Puzzle5d inference schema — flatPosition (plane + center) per part. */

export interface FlattenPlane {
  origin: [number, number, number];
  xAxis: [number, number, number];
  yAxis: [number, number, number];
}

export interface FlattenPose {
  plane: FlattenPlane;
  center: [number, number];
  orientation: [number, number, number, number];
}

export interface Puzzle5dInference {
  /** @derived */
  flatPositions: Record<string, FlattenPose>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dInferenceGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dInferenceGuardRefusal(at, why);
};

type puzzlePuzzle5dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dInferenceGuardReject(at, "value is not an object");
export const puzzlePuzzle5dInferenceGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dInferenceGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dInferenceGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dInferenceGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dInferenceGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dInferenceGuardNumber(value, at, bounds) : puzzlePuzzle5dInferenceGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dInference(value: unknown, at = "$"): Puzzle5dInference {
  const row = puzzlePuzzle5dInferenceGuardObject(value, at);
  return {
    flatPositions: puzzlePuzzle5dInferenceGuardObject(row["flatPositions"], `${at}.flatPositions`),
  };
}

export function parseFlattenPlane(value: unknown, at = "$"): FlattenPlane {
  const row = puzzlePuzzle5dInferenceGuardObject(value, at);
  return {
    origin: puzzlePuzzle5dInferenceGuardArray(row["origin"], `${at}.origin`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dInferenceGuardNumber(item, `${at}.origin[${index}]`)),
    xAxis: puzzlePuzzle5dInferenceGuardArray(row["xAxis"], `${at}.xAxis`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dInferenceGuardNumber(item, `${at}.xAxis[${index}]`)),
    yAxis: puzzlePuzzle5dInferenceGuardArray(row["yAxis"], `${at}.yAxis`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dInferenceGuardNumber(item, `${at}.yAxis[${index}]`)),
  };
}

export function parseFlattenPose(value: unknown, at = "$"): FlattenPose {
  const row = puzzlePuzzle5dInferenceGuardObject(value, at);
  return {
    plane: parseFlattenPlane(row["plane"], `${at}.plane`),
    center: puzzlePuzzle5dInferenceGuardArray(row["center"], `${at}.center`, {"minItems": 2, "maxItems": 2}).map((item, index) => puzzlePuzzle5dInferenceGuardNumber(item, `${at}.center[${index}]`)),
    orientation: puzzlePuzzle5dInferenceGuardArray(row["orientation"], `${at}.orientation`, {"minItems": 4, "maxItems": 4}).map((item, index) => puzzlePuzzle5dInferenceGuardNumber(item, `${at}.orientation[${index}]`)),
  };
}
