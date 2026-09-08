/** 🧬️ ImperativePresence — empty: imperative has no multi-user shareable live state of its own; step selection is the framework-owned `steps` interaction domain, broadcast via `PresencePeer.interaction` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Mirrors `ImperativePresence` in `../🦀️.rs`. */
export interface ImperativePresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativePresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativePresenceGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativePresenceGuardRefusal(at, why);
};

type imperativeImperativePresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativePresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativePresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativePresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativePresenceGuardReject(at, "value is not an object");
export const imperativeImperativePresenceGuardArray = (value: unknown, at: string, bounds: imperativeImperativePresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativePresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativePresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativePresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativePresenceGuardString = (value: unknown, at: string, bounds: imperativeImperativePresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativePresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativePresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativePresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativePresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativePresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativePresenceGuardReject(at, "value is not a boolean"));
export const imperativeImperativePresenceGuardNumber = (value: unknown, at: string, bounds: imperativeImperativePresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativePresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativePresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativePresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativePresenceGuardInteger = (value: unknown, at: string, bounds: imperativeImperativePresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativePresenceGuardNumber(value, at, bounds) : imperativeImperativePresenceGuardReject(at, "value is not an integer");
export const imperativeImperativePresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativePresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativePresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativePresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseImperativePresence(value: unknown, at = "$"): ImperativePresence {
  return imperativeImperativePresenceGuardObject(value, `${at}`);
}
