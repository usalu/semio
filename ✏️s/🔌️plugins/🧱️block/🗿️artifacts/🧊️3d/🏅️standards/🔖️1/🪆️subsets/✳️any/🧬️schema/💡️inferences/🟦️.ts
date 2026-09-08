/** 💡️ Block3d inference schema — bounds (bounding box + vertex count) over the object kind's rim vortex templates. */

export interface BoundingBox3d {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Block3dBounds {
  boundingBox: BoundingBox3d | null;
  vertexCount: number;
}

export interface Block3dInference {
  /** @derived */
  bounds: Block3dBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dInferenceGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dInferenceGuardRefusal(at, why);
};

type blockBlock3dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dInferenceGuardReject(at, "value is not an object");
export const blockBlock3dInferenceGuardArray = (value: unknown, at: string, bounds: blockBlock3dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dInferenceGuardString = (value: unknown, at: string, bounds: blockBlock3dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dInferenceGuardReject(at, "value is not a boolean"));
export const blockBlock3dInferenceGuardNumber = (value: unknown, at: string, bounds: blockBlock3dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dInferenceGuardInteger = (value: unknown, at: string, bounds: blockBlock3dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dInferenceGuardNumber(value, at, bounds) : blockBlock3dInferenceGuardReject(at, "value is not an integer");
export const blockBlock3dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dInference(value: unknown, at = "$"): Block3dInference {
  const row = blockBlock3dInferenceGuardObject(value, at);
  return {
    bounds: parseBlock3dBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseBoundingBox3d(value: unknown, at = "$"): BoundingBox3d {
  const row = blockBlock3dInferenceGuardObject(value, at);
  return {
    min: blockBlock3dInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => blockBlock3dInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: blockBlock3dInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => blockBlock3dInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}
