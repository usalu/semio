/** 🧬️ Block3dPresence */
export interface Block3dPresence {
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  hoveredVortexFullId?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class block3dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const block3dPresenceGuardReject = (at: string, why: string): never => {
  throw new block3dPresenceGuardRefusal(at, why);
};

type block3dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type block3dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type block3dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const block3dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : block3dPresenceGuardReject(at, "value is not an object");
export const block3dPresenceGuardArray = (value: unknown, at: string, bounds: block3dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return block3dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) block3dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) block3dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const block3dPresenceGuardString = (value: unknown, at: string, bounds: block3dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return block3dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) block3dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) block3dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) block3dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const block3dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : block3dPresenceGuardReject(at, "value is not a boolean"));
export const block3dPresenceGuardNumber = (value: unknown, at: string, bounds: block3dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return block3dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) block3dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) block3dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const block3dPresenceGuardInteger = (value: unknown, at: string, bounds: block3dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? block3dPresenceGuardNumber(value, at, bounds) : block3dPresenceGuardReject(at, "value is not an integer");
export const block3dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : block3dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const block3dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : block3dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dPresence(value: unknown, at = "$"): Block3dPresence {
  const row = block3dPresenceGuardObject(value, at);
  return {
    selectedIds: block3dPresenceGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => block3dPresenceGuardString(item, `${at}.selectedIds[${index}]`)),
    hoveredVortexFullId: row["hoveredVortexFullId"] === undefined ? undefined : block3dPresenceGuardString(row["hoveredVortexFullId"], `${at}.hoveredVortexFullId`),
  };
}
