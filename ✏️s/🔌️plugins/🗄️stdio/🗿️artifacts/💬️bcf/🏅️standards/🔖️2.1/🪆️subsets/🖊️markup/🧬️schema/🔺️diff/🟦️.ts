/** 🧬️ BcfDiff schema. */
export interface BcfDiff {
  schema?: string;
  bytes?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBcf21MarkupDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBcf21MarkupDiffGuardReject = (at: string, why: string): never => {
  throw new stdioBcf21MarkupDiffGuardRefusal(at, why);
};

type stdioBcf21MarkupDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBcf21MarkupDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBcf21MarkupDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBcf21MarkupDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBcf21MarkupDiffGuardReject(at, "value is not an object");
export const stdioBcf21MarkupDiffGuardArray = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBcf21MarkupDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBcf21MarkupDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBcf21MarkupDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBcf21MarkupDiffGuardString = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBcf21MarkupDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBcf21MarkupDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBcf21MarkupDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBcf21MarkupDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBcf21MarkupDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBcf21MarkupDiffGuardReject(at, "value is not a boolean"));
export const stdioBcf21MarkupDiffGuardNumber = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBcf21MarkupDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBcf21MarkupDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBcf21MarkupDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBcf21MarkupDiffGuardInteger = (value: unknown, at: string, bounds: stdioBcf21MarkupDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBcf21MarkupDiffGuardNumber(value, at, bounds) : stdioBcf21MarkupDiffGuardReject(at, "value is not an integer");
export const stdioBcf21MarkupDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBcf21MarkupDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBcf21MarkupDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBcf21MarkupDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBcfDiff(value: unknown, at = "$"): BcfDiff {
  const row = stdioBcf21MarkupDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioBcf21MarkupDiffGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioBcf21MarkupDiffGuardString(row["bytes"], `${at}.bytes`),
  };
}
