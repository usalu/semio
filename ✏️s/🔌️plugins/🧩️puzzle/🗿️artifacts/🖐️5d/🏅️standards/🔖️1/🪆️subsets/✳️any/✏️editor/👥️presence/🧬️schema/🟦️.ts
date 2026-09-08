/** 🧬️ Puzzle5dPresence */
export interface Puzzle5dPresence {
  /** @state presence */
  selectedPartIds: string[];
  /** @state presence */
  selectedGripIds: string[];
  /** @state presence */
  selectedFastenerIds: string[];
  /** @state presence */
  hoveredPartId?: string;
  /** @state presence */
  camera2dX: number;
  /** @state presence */
  camera2dY: number;
  /** @state presence */
  camera2dZoom: number;
  /** @state presence */
  camera3dPosition: number[];
  /** @state presence */
  camera3dTarget: number[];
  /** @state presence */
  camera3dZoom: number;
  /** @state presence */
  activeUtilityId: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dPresenceGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dPresenceGuardRefusal(at, why);
};

type puzzlePuzzle5dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dPresenceGuardReject(at, "value is not an object");
export const puzzlePuzzle5dPresenceGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dPresenceGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dPresenceGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dPresenceGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dPresenceGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dPresenceGuardNumber(value, at, bounds) : puzzlePuzzle5dPresenceGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dPresence(value: unknown, at = "$"): Puzzle5dPresence {
  const row = puzzlePuzzle5dPresenceGuardObject(value, at);
  return {
    selectedPartIds: puzzlePuzzle5dPresenceGuardArray(row["selectedPartIds"], `${at}.selectedPartIds`).map((item, index) => puzzlePuzzle5dPresenceGuardString(item, `${at}.selectedPartIds[${index}]`)),
    selectedGripIds: puzzlePuzzle5dPresenceGuardArray(row["selectedGripIds"], `${at}.selectedGripIds`).map((item, index) => puzzlePuzzle5dPresenceGuardString(item, `${at}.selectedGripIds[${index}]`)),
    selectedFastenerIds: puzzlePuzzle5dPresenceGuardArray(row["selectedFastenerIds"], `${at}.selectedFastenerIds`).map((item, index) => puzzlePuzzle5dPresenceGuardString(item, `${at}.selectedFastenerIds[${index}]`)),
    hoveredPartId: row["hoveredPartId"] === undefined ? undefined : puzzlePuzzle5dPresenceGuardString(row["hoveredPartId"], `${at}.hoveredPartId`),
    camera2dX: puzzlePuzzle5dPresenceGuardNumber(row["camera2dX"], `${at}.camera2dX`),
    camera2dY: puzzlePuzzle5dPresenceGuardNumber(row["camera2dY"], `${at}.camera2dY`),
    camera2dZoom: puzzlePuzzle5dPresenceGuardNumber(row["camera2dZoom"], `${at}.camera2dZoom`),
    camera3dPosition: puzzlePuzzle5dPresenceGuardArray(row["camera3dPosition"], `${at}.camera3dPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dPresenceGuardNumber(item, `${at}.camera3dPosition[${index}]`)),
    camera3dTarget: puzzlePuzzle5dPresenceGuardArray(row["camera3dTarget"], `${at}.camera3dTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dPresenceGuardNumber(item, `${at}.camera3dTarget[${index}]`)),
    camera3dZoom: puzzlePuzzle5dPresenceGuardNumber(row["camera3dZoom"], `${at}.camera3dZoom`),
    activeUtilityId: puzzlePuzzle5dPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
  };
}
