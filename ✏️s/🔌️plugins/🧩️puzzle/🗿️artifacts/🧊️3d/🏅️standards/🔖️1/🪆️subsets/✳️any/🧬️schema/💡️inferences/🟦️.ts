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
  const row = puzzlePuzzle3dInferenceGuardObject(value, at);
  return {
    flatPositions: Object.fromEntries(Object.entries(puzzlePuzzle3dInferenceGuardObject(row["flatPositions"], `${at}.flatPositions`)).map(([key,value])=>[key,parseFlattenPose(value,at+".flatPositions."+key)])),
  };
}

export function parseFlattenPlane(value: unknown, at = "$"): FlattenPlane {
  const row = puzzlePuzzle3dInferenceGuardObject(value, at);
  return {
    origin: puzzle3dInferenceTuple3(row["origin"],at+".origin"),
    xAxis: puzzle3dInferenceTuple3(row["xAxis"],at+".xAxis"),
    yAxis: puzzle3dInferenceTuple3(row["yAxis"],at+".yAxis"),
  };
}

export function parseFlattenPose(value: unknown, at = "$"): FlattenPose {
  const row = puzzlePuzzle3dInferenceGuardObject(value, at);
  return {
    plane: parseFlattenPlane(row["plane"], `${at}.plane`),
    center: puzzle3dInferenceTuple2(row["center"],at+".center"),
    orientation: puzzle3dInferenceTuple4(row["orientation"],at+".orientation"),
  };
}

const puzzle3dInferenceTuple3=(value:unknown,at:string):[number,number,number]=>{const items=puzzlePuzzle3dInferenceGuardArray(value,at,{minItems:3,maxItems:3}).map((item,index)=>puzzlePuzzle3dInferenceGuardNumber(item,at+"["+index+"]"));return[items[0]!,items[1]!,items[2]!]};
const puzzle3dInferenceTuple2=(value:unknown,at:string):[number,number]=>{const items=puzzlePuzzle3dInferenceGuardArray(value,at,{minItems:2,maxItems:2}).map((item,index)=>puzzlePuzzle3dInferenceGuardNumber(item,at+"["+index+"]"));return[items[0]!,items[1]!]};
const puzzle3dInferenceTuple4=(value:unknown,at:string):[number,number,number,number]=>{const items=puzzlePuzzle3dInferenceGuardArray(value,at,{minItems:4,maxItems:4}).map((item,index)=>puzzlePuzzle3dInferenceGuardNumber(item,at+"["+index+"]"));return[items[0]!,items[1]!,items[2]!,items[3]!]};
