/** 🧬️ Block5dPresence */
export interface Block5dPresence {
  /** @state presence */
  selectedIds: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class block5dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const block5dPresenceGuardReject = (at: string, why: string): never => {
  throw new block5dPresenceGuardRefusal(at, why);
};

type block5dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type block5dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type block5dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const block5dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : block5dPresenceGuardReject(at, "value is not an object");
export const block5dPresenceGuardArray = (value: unknown, at: string, bounds: block5dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return block5dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) block5dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) block5dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const block5dPresenceGuardString = (value: unknown, at: string, bounds: block5dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return block5dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) block5dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) block5dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) block5dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const block5dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : block5dPresenceGuardReject(at, "value is not a boolean"));
export const block5dPresenceGuardNumber = (value: unknown, at: string, bounds: block5dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return block5dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) block5dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) block5dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const block5dPresenceGuardInteger = (value: unknown, at: string, bounds: block5dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? block5dPresenceGuardNumber(value, at, bounds) : block5dPresenceGuardReject(at, "value is not an integer");
export const block5dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : block5dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const block5dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : block5dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dPresence(value: unknown, at = "$"): Block5dPresence {
  const row = block5dPresenceGuardObject(value, at);
  return {
    selectedIds: block5dPresenceGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => block5dPresenceGuardString(item, `${at}.selectedIds[${index}]`)),
  };
}
