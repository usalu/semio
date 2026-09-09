/** 🧬️ S Home artifact schema — every field with its state class. */

export interface SHomeArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  catalogGeneration: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeArtifactGuardReject = (at: string, why: string): never => {
  throw new spaceHomeArtifactGuardRefusal(at, why);
};

type spaceHomeArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeArtifactGuardReject(at, "value is not an object");
export const spaceHomeArtifactGuardArray = (value: unknown, at: string, bounds: spaceHomeArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeArtifactGuardString = (value: unknown, at: string, bounds: spaceHomeArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeArtifactGuardReject(at, "value is not a boolean"));
export const spaceHomeArtifactGuardNumber = (value: unknown, at: string, bounds: spaceHomeArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeArtifactGuardInteger = (value: unknown, at: string, bounds: spaceHomeArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeArtifactGuardNumber(value, at, bounds) : spaceHomeArtifactGuardReject(at, "value is not an integer");
export const spaceHomeArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeArtifact(value: unknown, at = "$"): SHomeArtifact {
  const row = spaceHomeArtifactGuardObject(value, at);
  return {
    schema: spaceHomeArtifactGuardString(row["schema"], `${at}.schema`),
    catalogGeneration: spaceHomeArtifactGuardInteger(row["catalogGeneration"], `${at}.catalogGeneration`, {"minimum": 0}),
  };
}
