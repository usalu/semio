/** 🧬️ Block2dPresence */
export interface Block2dPresence {
  /** @state presence */
  selectedIds: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class block2dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const block2dPresenceGuardReject = (at: string, why: string): never => {
  throw new block2dPresenceGuardRefusal(at, why);
};

type block2dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type block2dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type block2dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const block2dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : block2dPresenceGuardReject(at, "value is not an object");
export const block2dPresenceGuardArray = (value: unknown, at: string, bounds: block2dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return block2dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) block2dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) block2dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const block2dPresenceGuardString = (value: unknown, at: string, bounds: block2dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return block2dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) block2dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) block2dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) block2dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const block2dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : block2dPresenceGuardReject(at, "value is not a boolean"));
export const block2dPresenceGuardNumber = (value: unknown, at: string, bounds: block2dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return block2dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) block2dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) block2dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const block2dPresenceGuardInteger = (value: unknown, at: string, bounds: block2dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? block2dPresenceGuardNumber(value, at, bounds) : block2dPresenceGuardReject(at, "value is not an integer");
export const block2dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : block2dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const block2dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : block2dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dPresence(value: unknown, at = "$"): Block2dPresence {
  const row = block2dPresenceGuardObject(value, at);
  return {
    selectedIds: block2dPresenceGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => block2dPresenceGuardString(item, `${at}.selectedIds[${index}]`)),
  };
}
