/** 💡️ Block5d inference schema — bounds (bounding box + vertex count) over the part kind's rim grip templates' 3d placements. */

export interface BoundingBox3d {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Block5dBounds {
  boundingBox: BoundingBox3d | null;
  vertexCount: number;
}

export interface Block5dInference {
  /** @derived */
  bounds: Block5dBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dInferenceGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dInferenceGuardRefusal(at, why);
};

type blockBlock5dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dInferenceGuardReject(at, "value is not an object");
export const blockBlock5dInferenceGuardArray = (value: unknown, at: string, bounds: blockBlock5dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dInferenceGuardString = (value: unknown, at: string, bounds: blockBlock5dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dInferenceGuardReject(at, "value is not a boolean"));
export const blockBlock5dInferenceGuardNumber = (value: unknown, at: string, bounds: blockBlock5dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dInferenceGuardInteger = (value: unknown, at: string, bounds: blockBlock5dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dInferenceGuardNumber(value, at, bounds) : blockBlock5dInferenceGuardReject(at, "value is not an integer");
export const blockBlock5dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dInference(value: unknown, at = "$"): Block5dInference {
  const row = blockBlock5dInferenceGuardObject(value, at);
  return {
    bounds: parseBlock5dBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseBoundingBox3d(value: unknown, at = "$"): BoundingBox3d {
  const row = blockBlock5dInferenceGuardObject(value, at);
  return {
    min: blockBlock5dInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => blockBlock5dInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: blockBlock5dInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => blockBlock5dInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}
