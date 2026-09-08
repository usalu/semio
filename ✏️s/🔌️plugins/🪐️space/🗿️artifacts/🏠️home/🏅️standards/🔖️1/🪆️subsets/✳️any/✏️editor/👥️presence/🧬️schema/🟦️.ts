/** 🧬️ HomePresence — empty: the home launcher keeps panel tab and locale in `HomeConfig`; there is no multi-user shareable live surface on the launcher. Mirrors `HomePresence` in `../🦀️.rs`. */
export interface HomePresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomePresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomePresenceGuardReject = (at: string, why: string): never => {
  throw new spaceHomePresenceGuardRefusal(at, why);
};

type spaceHomePresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomePresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomePresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomePresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomePresenceGuardReject(at, "value is not an object");
export const spaceHomePresenceGuardArray = (value: unknown, at: string, bounds: spaceHomePresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomePresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomePresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomePresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomePresenceGuardString = (value: unknown, at: string, bounds: spaceHomePresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomePresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomePresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomePresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomePresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomePresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomePresenceGuardReject(at, "value is not a boolean"));
export const spaceHomePresenceGuardNumber = (value: unknown, at: string, bounds: spaceHomePresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomePresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomePresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomePresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomePresenceGuardInteger = (value: unknown, at: string, bounds: spaceHomePresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomePresenceGuardNumber(value, at, bounds) : spaceHomePresenceGuardReject(at, "value is not an integer");
export const spaceHomePresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomePresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomePresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomePresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHomePresence(value: unknown, at = "$"): HomePresence {
  return spaceHomePresenceGuardObject(value, `${at}`);
}
