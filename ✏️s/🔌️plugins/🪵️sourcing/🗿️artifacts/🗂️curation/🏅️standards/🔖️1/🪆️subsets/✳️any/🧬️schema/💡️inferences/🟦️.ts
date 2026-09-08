/** 💡️ Curation inference schema — a real census over the stock catalog and curated bill of quantities. */

export interface CurationEntries {
  stockCount: number;
  entryCount: number;
  totalCount: number;
}

export interface CurationInference {
  /** @derived */
  entries: CurationEntries;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sourcingCurationInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sourcingCurationInferenceGuardReject = (at: string, why: string): never => {
  throw new sourcingCurationInferenceGuardRefusal(at, why);
};

type sourcingCurationInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sourcingCurationInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sourcingCurationInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sourcingCurationInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sourcingCurationInferenceGuardReject(at, "value is not an object");
export const sourcingCurationInferenceGuardArray = (value: unknown, at: string, bounds: sourcingCurationInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sourcingCurationInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sourcingCurationInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sourcingCurationInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sourcingCurationInferenceGuardString = (value: unknown, at: string, bounds: sourcingCurationInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sourcingCurationInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sourcingCurationInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sourcingCurationInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sourcingCurationInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sourcingCurationInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sourcingCurationInferenceGuardReject(at, "value is not a boolean"));
export const sourcingCurationInferenceGuardNumber = (value: unknown, at: string, bounds: sourcingCurationInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sourcingCurationInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sourcingCurationInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sourcingCurationInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sourcingCurationInferenceGuardInteger = (value: unknown, at: string, bounds: sourcingCurationInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sourcingCurationInferenceGuardNumber(value, at, bounds) : sourcingCurationInferenceGuardReject(at, "value is not an integer");
export const sourcingCurationInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sourcingCurationInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sourcingCurationInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sourcingCurationInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCurationInference(value: unknown, at = "$"): CurationInference {
  const row = sourcingCurationInferenceGuardObject(value, at);
  return {
    entries: parseCurationEntries(row["entries"], `${at}.entries`),
  };
}

export function parseCurationEntries(value: unknown, at = "$"): CurationEntries {
  const row = sourcingCurationInferenceGuardObject(value, at);
  return {
    stockCount: sourcingCurationInferenceGuardInteger(row["stockCount"], `${at}.stockCount`),
    entryCount: sourcingCurationInferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
    totalCount: sourcingCurationInferenceGuardInteger(row["totalCount"], `${at}.totalCount`),
  };
}
