/** 💡️ Remodeling inference schema — reconstructed-mesh bounds (bounding box + vertex/face counts). */

export interface RemodelingBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface RemodelingBounds {
  boundingBox: RemodelingBoundingBox;
  vertexCount: number;
  faceCount: number;
}

export interface RemodelingInference {
  /** @derived */
  bounds: RemodelingBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingInferenceGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingInferenceGuardRefusal(at, why);
};

type remodelRemodelingInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingInferenceGuardReject(at, "value is not an object");
export const remodelRemodelingInferenceGuardArray = (value: unknown, at: string, bounds: remodelRemodelingInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingInferenceGuardString = (value: unknown, at: string, bounds: remodelRemodelingInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingInferenceGuardReject(at, "value is not a boolean"));
export const remodelRemodelingInferenceGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingInferenceGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingInferenceGuardNumber(value, at, bounds) : remodelRemodelingInferenceGuardReject(at, "value is not an integer");
export const remodelRemodelingInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingInference(value: unknown, at = "$"): RemodelingInference {
  const row = remodelRemodelingInferenceGuardObject(value, at);
  return {
    bounds: parseRemodelingBounds(row["bounds"], `${at}.bounds`),
    relativeCameraPoses: remodelRemodelingInferenceGuardObject(row["relativeCameraPoses"], `${at}.relativeCameraPoses`),
  };
}

export function parseRemodelingBoundingBox(value: unknown, at = "$"): RemodelingBoundingBox {
  const row = remodelRemodelingInferenceGuardObject(value, at);
  return {
    min: remodelRemodelingInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: remodelRemodelingInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}

export function parseRemodelingBounds(value: unknown, at = "$"): RemodelingBounds {
  const row = remodelRemodelingInferenceGuardObject(value, at);
  return {
    boundingBox: parseRemodelingBoundingBox(row["boundingBox"], `${at}.boundingBox`),
    vertexCount: remodelRemodelingInferenceGuardInteger(row["vertexCount"], `${at}.vertexCount`, {"minimum": 0}),
    faceCount: remodelRemodelingInferenceGuardInteger(row["faceCount"], `${at}.faceCount`, {"minimum": 0}),
  };
}

export interface RemodelingPoseDelta {
  readonly translationDelta: readonly number[];
  readonly rotationAngleRad: number;
}

export function parseRemodelingPoseDelta(value: unknown, at = "$"): RemodelingPoseDelta {
  const row = remodelRemodelingInferenceGuardObject(value, at);
  return {
    translationDelta: remodelRemodelingInferenceGuardArray(row["translationDelta"], `${at}.translationDelta`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingInferenceGuardNumber(item, `${at}.translationDelta[${index}]`)),
    rotationAngleRad: remodelRemodelingInferenceGuardNumber(row["rotationAngleRad"], `${at}.rotationAngleRad`),
  };
}
