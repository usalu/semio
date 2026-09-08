/** 💡️ SHome inference schema — contentDigest fingerprint of the persisted document. */

export interface SHomeInference {
  /** @derived */
  contentDigest: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeInferenceGuardReject = (at: string, why: string): never => {
  throw new spaceHomeInferenceGuardRefusal(at, why);
};

type spaceHomeInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeInferenceGuardReject(at, "value is not an object");
export const spaceHomeInferenceGuardArray = (value: unknown, at: string, bounds: spaceHomeInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeInferenceGuardString = (value: unknown, at: string, bounds: spaceHomeInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeInferenceGuardReject(at, "value is not a boolean"));
export const spaceHomeInferenceGuardNumber = (value: unknown, at: string, bounds: spaceHomeInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeInferenceGuardInteger = (value: unknown, at: string, bounds: spaceHomeInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeInferenceGuardNumber(value, at, bounds) : spaceHomeInferenceGuardReject(at, "value is not an integer");
export const spaceHomeInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeInference(value: unknown, at = "$"): SHomeInference {
  const row = spaceHomeInferenceGuardObject(value, at);
  return {
    contentDigest: spaceHomeInferenceGuardString(row["contentDigest"], `${at}.contentDigest`),
  };
}
