/** 🧬️ NotePresence */
export interface NotePresence {
  /** @state presence */
  cameraX: number;
  /** @state presence */
  cameraY: number;
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  activeUtilityId: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class noteNotePresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const noteNotePresenceGuardReject = (at: string, why: string): never => {
  throw new noteNotePresenceGuardRefusal(at, why);
};

type noteNotePresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type noteNotePresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type noteNotePresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const noteNotePresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : noteNotePresenceGuardReject(at, "value is not an object");
export const noteNotePresenceGuardArray = (value: unknown, at: string, bounds: noteNotePresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return noteNotePresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) noteNotePresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) noteNotePresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const noteNotePresenceGuardString = (value: unknown, at: string, bounds: noteNotePresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return noteNotePresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) noteNotePresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) noteNotePresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) noteNotePresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const noteNotePresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : noteNotePresenceGuardReject(at, "value is not a boolean"));
export const noteNotePresenceGuardNumber = (value: unknown, at: string, bounds: noteNotePresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return noteNotePresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) noteNotePresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) noteNotePresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const noteNotePresenceGuardInteger = (value: unknown, at: string, bounds: noteNotePresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? noteNotePresenceGuardNumber(value, at, bounds) : noteNotePresenceGuardReject(at, "value is not an integer");
export const noteNotePresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : noteNotePresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const noteNotePresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : noteNotePresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNotePresence(value: unknown, at = "$"): NotePresence {
  const row = noteNotePresenceGuardObject(value, at);
  return {
    cameraX: noteNotePresenceGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: noteNotePresenceGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: noteNotePresenceGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    activeUtilityId: noteNotePresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
  };
}
