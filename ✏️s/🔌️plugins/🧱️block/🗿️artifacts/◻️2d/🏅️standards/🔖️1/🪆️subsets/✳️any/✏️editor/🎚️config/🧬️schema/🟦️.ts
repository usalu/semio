/** 🧬️ Block2dConfig */
export interface Block2dConfig {
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class block2dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const block2dConfigGuardReject = (at: string, why: string): never => {
  throw new block2dConfigGuardRefusal(at, why);
};

type block2dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type block2dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type block2dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const block2dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : block2dConfigGuardReject(at, "value is not an object");
export const block2dConfigGuardArray = (value: unknown, at: string, bounds: block2dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return block2dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) block2dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) block2dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const block2dConfigGuardString = (value: unknown, at: string, bounds: block2dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return block2dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) block2dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) block2dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) block2dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const block2dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : block2dConfigGuardReject(at, "value is not a boolean"));
export const block2dConfigGuardNumber = (value: unknown, at: string, bounds: block2dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return block2dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) block2dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) block2dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const block2dConfigGuardInteger = (value: unknown, at: string, bounds: block2dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? block2dConfigGuardNumber(value, at, bounds) : block2dConfigGuardReject(at, "value is not an integer");
export const block2dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : block2dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const block2dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : block2dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dConfig(value: unknown, at = "$"): Block2dConfig {
  return block2dConfigGuardObject(value, `${at}`);
}
