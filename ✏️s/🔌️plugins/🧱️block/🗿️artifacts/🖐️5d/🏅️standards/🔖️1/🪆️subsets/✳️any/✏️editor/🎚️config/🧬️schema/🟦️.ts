/** 🧬️ Block5dConfig */
export interface Block5dConfig {
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class block5dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const block5dConfigGuardReject = (at: string, why: string): never => {
  throw new block5dConfigGuardRefusal(at, why);
};

type block5dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type block5dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type block5dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const block5dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : block5dConfigGuardReject(at, "value is not an object");
export const block5dConfigGuardArray = (value: unknown, at: string, bounds: block5dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return block5dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) block5dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) block5dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const block5dConfigGuardString = (value: unknown, at: string, bounds: block5dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return block5dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) block5dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) block5dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) block5dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const block5dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : block5dConfigGuardReject(at, "value is not a boolean"));
export const block5dConfigGuardNumber = (value: unknown, at: string, bounds: block5dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return block5dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) block5dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) block5dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const block5dConfigGuardInteger = (value: unknown, at: string, bounds: block5dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? block5dConfigGuardNumber(value, at, bounds) : block5dConfigGuardReject(at, "value is not an integer");
export const block5dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : block5dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const block5dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : block5dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dConfig(value: unknown, at = "$"): Block5dConfig {
  return block5dConfigGuardObject(value, `${at}`);
}
