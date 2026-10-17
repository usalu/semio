/** 💡️ Puzzle3d inference schema — flatPosition (plane + center) per object. */

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

export interface Puzzle3dInference {
  /** @derived */
  flatPositions: Record<string, FlattenPose>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dInferenceGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dInferenceGuardRefusal(at, why);
};

type puzzlePuzzle3dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dInferenceGuardReject(at, "value is not an object");
export const puzzlePuzzle3dInferenceGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) puzzlePuzzle3dInferenceGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const puzzlePuzzle3dInferenceGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dInferenceGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dInferenceGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dInferenceGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dInferenceGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dInferenceGuardNumber(value, at, bounds) : puzzlePuzzle3dInferenceGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dInference(value: unknown, at = "$"): Puzzle3dInference {
  const row = puzzlePuzzle3dInferenceGuardClosed(puzzlePuzzle3dInferenceGuardObject(value, at), at, ["flatPositions"]);
  return {
    flatPositions: Object.fromEntries(Object.entries(puzzlePuzzle3dInferenceGuardObject(row["flatPositions"], `${at}.flatPositions`)).map(([key0, value0]) => [key0, parseFlattenPose(value0, `${at}.flatPositions.${key0}`)])),
  };
}

export function parseFlattenPlane(value: unknown, at = "$"): FlattenPlane {
  const row = puzzlePuzzle3dInferenceGuardClosed(puzzlePuzzle3dInferenceGuardObject(value, at), at, ["origin", "xAxis", "yAxis"]);
  return {
    origin: ((items0) => [puzzlePuzzle3dInferenceGuardNumber(items0[0], `${at}.origin[0]`), puzzlePuzzle3dInferenceGuardNumber(items0[1], `${at}.origin[1]`), puzzlePuzzle3dInferenceGuardNumber(items0[2], `${at}.origin[2]`)])(puzzlePuzzle3dInferenceGuardArray(row["origin"], `${at}.origin`, {"minItems": 3, "maxItems": 3})),
    xAxis: ((items0) => [puzzlePuzzle3dInferenceGuardNumber(items0[0], `${at}.xAxis[0]`), puzzlePuzzle3dInferenceGuardNumber(items0[1], `${at}.xAxis[1]`), puzzlePuzzle3dInferenceGuardNumber(items0[2], `${at}.xAxis[2]`)])(puzzlePuzzle3dInferenceGuardArray(row["xAxis"], `${at}.xAxis`, {"minItems": 3, "maxItems": 3})),
    yAxis: ((items0) => [puzzlePuzzle3dInferenceGuardNumber(items0[0], `${at}.yAxis[0]`), puzzlePuzzle3dInferenceGuardNumber(items0[1], `${at}.yAxis[1]`), puzzlePuzzle3dInferenceGuardNumber(items0[2], `${at}.yAxis[2]`)])(puzzlePuzzle3dInferenceGuardArray(row["yAxis"], `${at}.yAxis`, {"minItems": 3, "maxItems": 3})),
  };
}

export function parseFlattenPose(value: unknown, at = "$"): FlattenPose {
  const row = puzzlePuzzle3dInferenceGuardClosed(puzzlePuzzle3dInferenceGuardObject(value, at), at, ["plane", "center", "orientation"]);
  return {
    plane: parseFlattenPlane(row["plane"], `${at}.plane`),
    center: ((items0) => [puzzlePuzzle3dInferenceGuardNumber(items0[0], `${at}.center[0]`), puzzlePuzzle3dInferenceGuardNumber(items0[1], `${at}.center[1]`)])(puzzlePuzzle3dInferenceGuardArray(row["center"], `${at}.center`, {"minItems": 2, "maxItems": 2})),
    orientation: ((items0) => [puzzlePuzzle3dInferenceGuardNumber(items0[0], `${at}.orientation[0]`), puzzlePuzzle3dInferenceGuardNumber(items0[1], `${at}.orientation[1]`), puzzlePuzzle3dInferenceGuardNumber(items0[2], `${at}.orientation[2]`), puzzlePuzzle3dInferenceGuardNumber(items0[3], `${at}.orientation[3]`)])(puzzlePuzzle3dInferenceGuardArray(row["orientation"], `${at}.orientation`, {"minItems": 4, "maxItems": 4})),
  };
}

const puzzle3dInferenceTuple3=(value:unknown,at:string):[number,number,number]=>{const items=puzzlePuzzle3dInferenceGuardArray(value,at,{minItems:3,maxItems:3}).map((item,index)=>puzzlePuzzle3dInferenceGuardNumber(item,at+"["+index+"]"));return[items[0]!,items[1]!,items[2]!]};
const puzzle3dInferenceTuple2=(value:unknown,at:string):[number,number]=>{const items=puzzlePuzzle3dInferenceGuardArray(value,at,{minItems:2,maxItems:2}).map((item,index)=>puzzlePuzzle3dInferenceGuardNumber(item,at+"["+index+"]"));return[items[0]!,items[1]!]};
const puzzle3dInferenceTuple4=(value:unknown,at:string):[number,number,number,number]=>{const items=puzzlePuzzle3dInferenceGuardArray(value,at,{minItems:4,maxItems:4}).map((item,index)=>puzzlePuzzle3dInferenceGuardNumber(item,at+"["+index+"]"));return[items[0]!,items[1]!,items[2]!,items[3]!]};
