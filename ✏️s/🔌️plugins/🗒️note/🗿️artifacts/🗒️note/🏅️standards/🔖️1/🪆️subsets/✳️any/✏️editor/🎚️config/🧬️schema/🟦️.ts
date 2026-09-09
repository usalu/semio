/** 🧬️ NoteConfig */
export interface NoteConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: NoteCamera;
  /** @state config */
}

export interface NoteCamera {
  x: number;
  y: number;
  zoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class noteNoteConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const noteNoteConfigGuardReject = (at: string, why: string): never => {
  throw new noteNoteConfigGuardRefusal(at, why);
};

type noteNoteConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type noteNoteConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type noteNoteConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const noteNoteConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : noteNoteConfigGuardReject(at, "value is not an object");
export const noteNoteConfigGuardArray = (value: unknown, at: string, bounds: noteNoteConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return noteNoteConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) noteNoteConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) noteNoteConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const noteNoteConfigGuardString = (value: unknown, at: string, bounds: noteNoteConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return noteNoteConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) noteNoteConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) noteNoteConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) noteNoteConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const noteNoteConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : noteNoteConfigGuardReject(at, "value is not a boolean"));
export const noteNoteConfigGuardNumber = (value: unknown, at: string, bounds: noteNoteConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return noteNoteConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) noteNoteConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) noteNoteConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const noteNoteConfigGuardInteger = (value: unknown, at: string, bounds: noteNoteConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? noteNoteConfigGuardNumber(value, at, bounds) : noteNoteConfigGuardReject(at, "value is not an integer");
export const noteNoteConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : noteNoteConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const noteNoteConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : noteNoteConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNoteConfig(value: unknown, at = "$"): NoteConfig {
  const row = noteNoteConfigGuardObject(value, at);
  return {
    engagementInput: noteNoteConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    camera: parseNoteCamera(row["camera"], `${at}.camera`),
  };
}

export function parseNoteCamera(value: unknown, at = "$"): NoteCamera {
  const row = noteNoteConfigGuardObject(value, at);
  return {
    x: noteNoteConfigGuardNumber(row["x"], `${at}.x`),
    y: noteNoteConfigGuardNumber(row["y"], `${at}.y`),
    zoom: noteNoteConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
