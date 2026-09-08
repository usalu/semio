/** 🧬️ Puzzle3dPresence */
export interface Puzzle3dPresence {
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  selectedVortexIds: string[];
  /** @state presence */
  selectedAttractionIds: string[];
  /** @state presence */
  selectedTargetVolumeIds: string[];
  /** @state presence */
  selectedReferenceIds: string[];
  /** @state presence */
  hoveredObjectId?: string;
  /** @state presence */
  hoveredVortexFullId?: string;
  /** @state presence */
  cameraPosition: number[];
  /** @state presence */
  cameraTarget: number[];
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  activeToolId?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dPresenceGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dPresenceGuardRefusal(at, why);
};

type puzzlePuzzle3dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dPresenceGuardReject(at, "value is not an object");
export const puzzlePuzzle3dPresenceGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dPresenceGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dPresenceGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dPresenceGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dPresenceGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dPresenceGuardNumber(value, at, bounds) : puzzlePuzzle3dPresenceGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dPresence(value: unknown, at = "$"): Puzzle3dPresence {
  const row = puzzlePuzzle3dPresenceGuardObject(value, at);
  return {
    selectedObjectIds: puzzlePuzzle3dPresenceGuardArray(row["selectedObjectIds"], `${at}.selectedObjectIds`).map((item, index) => puzzlePuzzle3dPresenceGuardString(item, `${at}.selectedObjectIds[${index}]`)),
    selectedVortexIds: puzzlePuzzle3dPresenceGuardArray(row["selectedVortexIds"], `${at}.selectedVortexIds`).map((item, index) => puzzlePuzzle3dPresenceGuardString(item, `${at}.selectedVortexIds[${index}]`)),
    selectedAttractionIds: puzzlePuzzle3dPresenceGuardArray(row["selectedAttractionIds"], `${at}.selectedAttractionIds`).map((item, index) => puzzlePuzzle3dPresenceGuardString(item, `${at}.selectedAttractionIds[${index}]`)),
    selectedTargetVolumeIds: puzzlePuzzle3dPresenceGuardArray(row["selectedTargetVolumeIds"], `${at}.selectedTargetVolumeIds`).map((item, index) => puzzlePuzzle3dPresenceGuardString(item, `${at}.selectedTargetVolumeIds[${index}]`)),
    selectedReferenceIds: puzzlePuzzle3dPresenceGuardArray(row["selectedReferenceIds"], `${at}.selectedReferenceIds`).map((item, index) => puzzlePuzzle3dPresenceGuardString(item, `${at}.selectedReferenceIds[${index}]`)),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : puzzlePuzzle3dPresenceGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredVortexFullId: row["hoveredVortexFullId"] === undefined ? undefined : puzzlePuzzle3dPresenceGuardString(row["hoveredVortexFullId"], `${at}.hoveredVortexFullId`),
    cameraPosition: puzzlePuzzle3dPresenceGuardArray(row["cameraPosition"], `${at}.cameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dPresenceGuardNumber(item, `${at}.cameraPosition[${index}]`)),
    cameraTarget: puzzlePuzzle3dPresenceGuardArray(row["cameraTarget"], `${at}.cameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dPresenceGuardNumber(item, `${at}.cameraTarget[${index}]`)),
    cameraZoom: puzzlePuzzle3dPresenceGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    activeUtilityId: puzzlePuzzle3dPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    activeToolId: row["activeToolId"] === undefined ? undefined : puzzlePuzzle3dPresenceGuardString(row["activeToolId"], `${at}.activeToolId`),
  };
}
