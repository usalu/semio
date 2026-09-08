/** 📝️ Text representation for `s.stdio.dwg.inference` (ac1024). */
export type DwgInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1024AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1024AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1024AnyInferenceTextGuardRefusal(at, why);
};

type stdioDwgAc1024AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1024AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1024AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1024AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1024AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioDwgAc1024AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1024AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1024AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1024AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1024AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1024AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1024AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1024AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1024AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1024AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1024AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1024AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1024AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1024AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1024AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1024AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1024AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1024AnyInferenceTextGuardNumber(value, at, bounds) : stdioDwgAc1024AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioDwgAc1024AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1024AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1024AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1024AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface DwgAc1024InferenceText {
  readonly schema: "s.stdio.dwg.inference";
  readonly structure: DwgStructureText;
}

export function parseDwgAc1024InferenceText(value: unknown, at = "$"): DwgAc1024InferenceText {
  const row = stdioDwgAc1024AnyInferenceTextGuardObject(value, at);
  return {
    schema: stdioDwgAc1024AnyInferenceTextGuardConstant(row["schema"], `${at}.schema`, "s.stdio.dwg.inference"),
    structure: parseDwgStructureText(row["structure"], `${at}.structure`),
  };
}

export interface DwgStructureText {
  readonly layerCount: number;
  readonly entityCount: number;
  readonly geometryValueCount: number;
  readonly geometryIndexCount: number;
  readonly textCharacterCount: number;
  readonly codepage: number;
  readonly version: string;
}

export function parseDwgStructureText(value: unknown, at = "$"): DwgStructureText {
  const row = stdioDwgAc1024AnyInferenceTextGuardObject(value, at);
  return {
    layerCount: stdioDwgAc1024AnyInferenceTextGuardInteger(row["layerCount"], `${at}.layerCount`, {"minimum": 0}),
    entityCount: stdioDwgAc1024AnyInferenceTextGuardInteger(row["entityCount"], `${at}.entityCount`, {"minimum": 0}),
    geometryValueCount: stdioDwgAc1024AnyInferenceTextGuardInteger(row["geometryValueCount"], `${at}.geometryValueCount`, {"minimum": 0}),
    geometryIndexCount: stdioDwgAc1024AnyInferenceTextGuardInteger(row["geometryIndexCount"], `${at}.geometryIndexCount`, {"minimum": 0}),
    textCharacterCount: stdioDwgAc1024AnyInferenceTextGuardInteger(row["textCharacterCount"], `${at}.textCharacterCount`, {"minimum": 0}),
    codepage: stdioDwgAc1024AnyInferenceTextGuardInteger(row["codepage"], `${at}.codepage`, {"minimum": 0}),
    version: stdioDwgAc1024AnyInferenceTextGuardString(row["version"], `${at}.version`),
  };
}
