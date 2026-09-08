export interface DwgStructure { layerCount: number; entityCount: number; geometryValueCount: number; geometryIndexCount: number; textCharacterCount: number; codepage: number; version: string; }
export interface DwgInference { structure: DwgStructure; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1018AnyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1018AnyInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1018AnyInferenceGuardRefusal(at, why);
};

type stdioDwgAc1018AnyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1018AnyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1018AnyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1018AnyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1018AnyInferenceGuardReject(at, "value is not an object");
export const stdioDwgAc1018AnyInferenceGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1018AnyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1018AnyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1018AnyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1018AnyInferenceGuardString = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1018AnyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1018AnyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1018AnyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1018AnyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1018AnyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1018AnyInferenceGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1018AnyInferenceGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1018AnyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1018AnyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1018AnyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1018AnyInferenceGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1018AnyInferenceGuardNumber(value, at, bounds) : stdioDwgAc1018AnyInferenceGuardReject(at, "value is not an integer");
export const stdioDwgAc1018AnyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1018AnyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1018AnyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1018AnyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDwgInference(value: unknown, at = "$"): DwgInference {
  const row = stdioDwgAc1018AnyInferenceGuardObject(value, at);
  return {
    structure: parseDwgStructure(row["structure"], `${at}.structure`),
  };
}

export function parseDwgStructure(value: unknown, at = "$"): DwgStructure {
  const row = stdioDwgAc1018AnyInferenceGuardObject(value, at);
  return {
    layerCount: stdioDwgAc1018AnyInferenceGuardInteger(row["layerCount"], `${at}.layerCount`),
    entityCount: stdioDwgAc1018AnyInferenceGuardInteger(row["entityCount"], `${at}.entityCount`),
    geometryValueCount: stdioDwgAc1018AnyInferenceGuardInteger(row["geometryValueCount"], `${at}.geometryValueCount`),
    geometryIndexCount: stdioDwgAc1018AnyInferenceGuardInteger(row["geometryIndexCount"], `${at}.geometryIndexCount`),
    textCharacterCount: stdioDwgAc1018AnyInferenceGuardInteger(row["textCharacterCount"], `${at}.textCharacterCount`),
    codepage: stdioDwgAc1018AnyInferenceGuardInteger(row["codepage"], `${at}.codepage`),
    version: stdioDwgAc1018AnyInferenceGuardString(row["version"], `${at}.version`),
  };
}
