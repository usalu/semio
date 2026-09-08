/** 💡️ Dxf inference schema — entity-derived 3D bounding box over top-level and block-nested
 * entities. */

export interface DxfBounds {
  min: [number, number, number];
  max: [number, number, number];
  entityCount: number;
}

export interface DxfInference {
  /** @derived */
  bounds: DxfBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDxfR12HeaderInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDxfR12HeaderInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioDxfR12HeaderInferenceGuardRefusal(at, why);
};

type stdioDxfR12HeaderInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDxfR12HeaderInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDxfR12HeaderInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDxfR12HeaderInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDxfR12HeaderInferenceGuardReject(at, "value is not an object");
export const stdioDxfR12HeaderInferenceGuardArray = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDxfR12HeaderInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDxfR12HeaderInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDxfR12HeaderInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDxfR12HeaderInferenceGuardString = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDxfR12HeaderInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDxfR12HeaderInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDxfR12HeaderInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDxfR12HeaderInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDxfR12HeaderInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDxfR12HeaderInferenceGuardReject(at, "value is not a boolean"));
export const stdioDxfR12HeaderInferenceGuardNumber = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDxfR12HeaderInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDxfR12HeaderInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDxfR12HeaderInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDxfR12HeaderInferenceGuardInteger = (value: unknown, at: string, bounds: stdioDxfR12HeaderInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDxfR12HeaderInferenceGuardNumber(value, at, bounds) : stdioDxfR12HeaderInferenceGuardReject(at, "value is not an integer");
export const stdioDxfR12HeaderInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDxfR12HeaderInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDxfR12HeaderInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDxfR12HeaderInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDxfInference(value: unknown, at = "$"): DxfInference {
  const row = stdioDxfR12HeaderInferenceGuardObject(value, at);
  return {
    bounds: parseDxfBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseDxfBounds(value: unknown, at = "$"): DxfBounds {
  const row = stdioDxfR12HeaderInferenceGuardObject(value, at);
  return {
    min: stdioDxfR12HeaderInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioDxfR12HeaderInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: stdioDxfR12HeaderInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioDxfR12HeaderInferenceGuardNumber(item, `${at}.max[${index}]`)),
    entityCount: stdioDxfR12HeaderInferenceGuardInteger(row["entityCount"], `${at}.entityCount`, {"minimum": 0}),
  };
}
