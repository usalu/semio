/** 🧬️ Puzzle2dPresence */
export interface Puzzle2dPresence {
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  cameraX: number;
  /** @state presence */
  cameraY: number;
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  selectionMethod: string;
  /** @state presence */
  activeUtilityId: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dPresenceGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dPresenceGuardRefusal(at, why);
};

type puzzlePuzzle2dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dPresenceGuardReject(at, "value is not an object");
export const puzzlePuzzle2dPresenceGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dPresenceGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dPresenceGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dPresenceGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dPresenceGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dPresenceGuardNumber(value, at, bounds) : puzzlePuzzle2dPresenceGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dPresence(value: unknown, at = "$"): Puzzle2dPresence {
  const row = puzzlePuzzle2dPresenceGuardObject(value, at);
  return {
    selectedIds: puzzlePuzzle2dPresenceGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => puzzlePuzzle2dPresenceGuardString(item, `${at}.selectedIds[${index}]`)),
    cameraX: puzzlePuzzle2dPresenceGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: puzzlePuzzle2dPresenceGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: puzzlePuzzle2dPresenceGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    selectionMethod: puzzlePuzzle2dPresenceGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    activeUtilityId: puzzlePuzzle2dPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
  };
}
