/** 🧬️ S Home diff schema — sparse field delta over the artifact. */

export interface SHomeDiff {
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  catalogGeneration?: number;
  /** @state config */
  activePanelTab?: string;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeDiffGuardReject = (at: string, why: string): never => {
  throw new spaceHomeDiffGuardRefusal(at, why);
};

type spaceHomeDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeDiffGuardReject(at, "value is not an object");
export const spaceHomeDiffGuardArray = (value: unknown, at: string, bounds: spaceHomeDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeDiffGuardString = (value: unknown, at: string, bounds: spaceHomeDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeDiffGuardReject(at, "value is not a boolean"));
export const spaceHomeDiffGuardNumber = (value: unknown, at: string, bounds: spaceHomeDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeDiffGuardInteger = (value: unknown, at: string, bounds: spaceHomeDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeDiffGuardNumber(value, at, bounds) : spaceHomeDiffGuardReject(at, "value is not an integer");
export const spaceHomeDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeDiff(value: unknown, at = "$"): SHomeDiff {
  const row = spaceHomeDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : spaceHomeDiffGuardString(row["schema"], `${at}.schema`),
    catalogGeneration: row["catalogGeneration"] === undefined ? undefined : spaceHomeDiffGuardInteger(row["catalogGeneration"], `${at}.catalogGeneration`, {"minimum": 0}),
    activePanelTab: row["activePanelTab"] === undefined ? undefined : spaceHomeDiffGuardString(row["activePanelTab"], `${at}.activePanelTab`),
  };
}
