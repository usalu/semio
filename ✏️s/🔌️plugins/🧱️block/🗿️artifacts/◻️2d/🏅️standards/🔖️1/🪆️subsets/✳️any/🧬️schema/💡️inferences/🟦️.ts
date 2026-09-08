/** 💡️ Block2d inference schema — bounds (bounding box + vertex count) over the node kind's rim handle templates. */

export interface BoundingBox2d {
  min: [number, number];
  max: [number, number];
}

export interface Block2dBounds {
  boundingBox: BoundingBox2d | null;
  vertexCount: number;
}

export interface Block2dInference {
  /** @derived */
  bounds: Block2dBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dInferenceGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dInferenceGuardRefusal(at, why);
};

type blockBlock2dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dInferenceGuardReject(at, "value is not an object");
export const blockBlock2dInferenceGuardArray = (value: unknown, at: string, bounds: blockBlock2dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dInferenceGuardString = (value: unknown, at: string, bounds: blockBlock2dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dInferenceGuardReject(at, "value is not a boolean"));
export const blockBlock2dInferenceGuardNumber = (value: unknown, at: string, bounds: blockBlock2dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dInferenceGuardInteger = (value: unknown, at: string, bounds: blockBlock2dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dInferenceGuardNumber(value, at, bounds) : blockBlock2dInferenceGuardReject(at, "value is not an integer");
export const blockBlock2dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dInference(value: unknown, at = "$"): Block2dInference {
  const row = blockBlock2dInferenceGuardObject(value, at);
  return {
    bounds: parseBlock2dBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseBoundingBox2d(value: unknown, at = "$"): BoundingBox2d {
  const row = blockBlock2dInferenceGuardObject(value, at);
  return {
    min: blockBlock2dInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 2, "maxItems": 2}).map((item, index) => blockBlock2dInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: blockBlock2dInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 2, "maxItems": 2}).map((item, index) => blockBlock2dInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}
