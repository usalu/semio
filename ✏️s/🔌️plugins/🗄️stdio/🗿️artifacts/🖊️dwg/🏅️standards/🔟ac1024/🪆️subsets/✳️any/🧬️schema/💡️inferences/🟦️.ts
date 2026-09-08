/** 💡️ Dwg (ac1024) inference schema over logical drawing concepts. */

export interface DwgStructure {
  layerCount: number;
  entityCount: number;
  geometryValueCount: number;
  geometryIndexCount: number;
  textCharacterCount: number;
  codepage: number;
  version: string;
}

export interface DwgInference {
  /** @derived */
  structure: DwgStructure;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1024AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1024AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1024AnyInferenceGuardRefusal(at, why);
};

type stdioDwgAc1024AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1024AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1024AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1024AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1024AnyInferenceGuardReject(at, "value is not an object");
export const stdioDwgAc1024AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1024AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1024AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1024AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1024AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1024AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1024AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1024AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1024AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1024AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1024AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1024AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1024AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1024AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1024AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1024AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1024AnyInferenceGuardNumber(value, at, bounds) : stdioDwgAc1024AnyInferenceGuardReject(at, "value is not an integer");
export const stdioDwgAc1024AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1024AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1024AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1024AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDwgInference(value: unknown, at = "$"): DwgInference {
  const row = stdioDwgAc1024AnyInferenceGuardObject(value, at);
  return {
    structure: parseDwgStructure(row["structure"], `${at}.structure`),
  };
}

export function parseDwgStructure(value: unknown, at = "$"): DwgStructure {
  const row = stdioDwgAc1024AnyInferenceGuardObject(value, at);
  return {
    layerCount: stdioDwgAc1024AnyInferenceGuardInteger(row["layerCount"], `${at}.layerCount`, {"minimum": 0}),
    entityCount: stdioDwgAc1024AnyInferenceGuardInteger(row["entityCount"], `${at}.entityCount`, {"minimum": 0}),
    geometryValueCount: stdioDwgAc1024AnyInferenceGuardInteger(row["geometryValueCount"], `${at}.geometryValueCount`, {"minimum": 0}),
    geometryIndexCount: stdioDwgAc1024AnyInferenceGuardInteger(row["geometryIndexCount"], `${at}.geometryIndexCount`, {"minimum": 0}),
    textCharacterCount: stdioDwgAc1024AnyInferenceGuardInteger(row["textCharacterCount"], `${at}.textCharacterCount`, {"minimum": 0}),
    codepage: stdioDwgAc1024AnyInferenceGuardInteger(row["codepage"], `${at}.codepage`, {"minimum": 0}),
    version: stdioDwgAc1024AnyInferenceGuardString(row["version"], `${at}.version`),
  };
}
