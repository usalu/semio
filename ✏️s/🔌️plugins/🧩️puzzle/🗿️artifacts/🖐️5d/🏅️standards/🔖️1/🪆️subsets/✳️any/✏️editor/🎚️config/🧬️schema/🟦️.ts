/** 🧬️ Puzzle5dConfig */
export interface Puzzle5dConfig {
  /** @state config */
  camera2d: Puzzle5dCamera2d;
  /** @state config */
  camera3d: Puzzle5dCamera3d;
  /** @state config */
  selection: Puzzle5dSelection;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredPartId?: string;
  /** @state config */
  fillCount: number;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  overlapBudget: number;
  /** @state config */
  lodMode: string;
  /** @state config */
  suggestionOffset: number;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  engagementInputByWindow: Record<string, string>;
  /** @state config */
  objectKindWeights: Record<string, number>;
  /** @state config */
  vortexKindWeights: Record<string, number>;
  /** @state config */
  sun: WorldSunConfig;
  /** @state config */
  activeUtilityByWindowId: Record<string, string>;
  /** @state config */
  /** @state config */
}

export type SelectionSet = string[];

export interface WorldSunConfig {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface WorldProjectionConfig {
  kind: string;
  orthographicView: string;
  axonometricVariant: string;
  axonometricAngleA: number;
  axonometricAngleB: number;
  axonometricQuadrant: string;
  obliqueVariant: string;
  obliqueAngle: number;
  obliqueDepth: number;
  onePointAxis: string;
  fov: number;
  twoPointShift: number;
  curvilinearFov: number;
  curvilinearStrength: number;
  curvilinearMapping: string;
}

export interface Puzzle5dCamera2d {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle5dCamera3d {
  position: number[];
  target: number[];
  zoom: number;
}

export interface Puzzle5dSelection {
  partIds: SelectionSet;
  gripIds: SelectionSet;
  fastenerIds: SelectionSet;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dConfigGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dConfigGuardRefusal(at, why);
};

type puzzlePuzzle5dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dConfigGuardReject(at, "value is not an object");
export const puzzlePuzzle5dConfigGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dConfigGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dConfigGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dConfigGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dConfigGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dConfigGuardNumber(value, at, bounds) : puzzlePuzzle5dConfigGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dConfig(value: unknown, at = "$"): Puzzle5dConfig {
  const row = puzzlePuzzle5dConfigGuardObject(value, at);
  return {
    camera2d: parsePuzzle5dCamera2d(row["camera2d"], `${at}.camera2d`),
    camera3d: parsePuzzle5dCamera3d(row["camera3d"], `${at}.camera3d`),
    selection: parsePuzzle5dSelection(row["selection"], `${at}.selection`),
    selectionMethod: puzzlePuzzle5dConfigGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    hoveredPartId: row["hoveredPartId"] === undefined ? undefined : puzzlePuzzle5dConfigGuardString(row["hoveredPartId"], `${at}.hoveredPartId`),
    fillCount: puzzlePuzzle5dConfigGuardInteger(row["fillCount"], `${at}.fillCount`, {"minimum": 0}),
    brushCandidateIndex: puzzlePuzzle5dConfigGuardInteger(row["brushCandidateIndex"], `${at}.brushCandidateIndex`, {"minimum": 0}),
    overlapBudget: puzzlePuzzle5dConfigGuardNumber(row["overlapBudget"], `${at}.overlapBudget`),
    lodMode: puzzlePuzzle5dConfigGuardString(row["lodMode"], `${at}.lodMode`),
    suggestionOffset: puzzlePuzzle5dConfigGuardNumber(row["suggestionOffset"], `${at}.suggestionOffset`),
    gridSnapEnabled: puzzlePuzzle5dConfigGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridFactor: puzzlePuzzle5dConfigGuardNumber(row["gridFactor"], `${at}.gridFactor`),
    engagementInputByWindow: puzzlePuzzle5dConfigGuardObject(row["engagementInputByWindow"], `${at}.engagementInputByWindow`),
    objectKindWeights: puzzlePuzzle5dConfigGuardObject(row["objectKindWeights"], `${at}.objectKindWeights`),
    vortexKindWeights: puzzlePuzzle5dConfigGuardObject(row["vortexKindWeights"], `${at}.vortexKindWeights`),
    sun: parseWorldSunConfig(row["sun"], `${at}.sun`),
    activeUtilityByWindowId: puzzlePuzzle5dConfigGuardObject(row["activeUtilityByWindowId"], `${at}.activeUtilityByWindowId`),
  };
}

export function parseSelectionSet(value: unknown, at = "$"): SelectionSet {
  return puzzlePuzzle5dConfigGuardArray(value, `${at}`).map((item, index) => puzzlePuzzle5dConfigGuardString(item, `${at}[${index}]`));
}

export function parseWorldSunConfig(value: unknown, at = "$"): WorldSunConfig {
  const row = puzzlePuzzle5dConfigGuardObject(value, at);
  return {
    enabled: puzzlePuzzle5dConfigGuardBoolean(row["enabled"], `${at}.enabled`),
    azimuth: puzzlePuzzle5dConfigGuardNumber(row["azimuth"], `${at}.azimuth`),
    elevation: puzzlePuzzle5dConfigGuardNumber(row["elevation"], `${at}.elevation`),
    intensity: puzzlePuzzle5dConfigGuardNumber(row["intensity"], `${at}.intensity`),
    color: puzzlePuzzle5dConfigGuardString(row["color"], `${at}.color`),
  };
}

export function parsePuzzle5dCamera2d(value: unknown, at = "$"): Puzzle5dCamera2d {
  const row = puzzlePuzzle5dConfigGuardObject(value, at);
  return {
    x: puzzlePuzzle5dConfigGuardNumber(row["x"], `${at}.x`),
    y: puzzlePuzzle5dConfigGuardNumber(row["y"], `${at}.y`),
    zoom: puzzlePuzzle5dConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parsePuzzle5dCamera3d(value: unknown, at = "$"): Puzzle5dCamera3d {
  const row = puzzlePuzzle5dConfigGuardObject(value, at);
  return {
    position: puzzlePuzzle5dConfigGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dConfigGuardNumber(item, `${at}.position[${index}]`)),
    target: puzzlePuzzle5dConfigGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dConfigGuardNumber(item, `${at}.target[${index}]`)),
    zoom: puzzlePuzzle5dConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parsePuzzle5dSelection(value: unknown, at = "$"): Puzzle5dSelection {
  const row = puzzlePuzzle5dConfigGuardObject(value, at);
  return {
    partIds: parseSelectionSet(row["partIds"], `${at}.partIds`),
    gripIds: parseSelectionSet(row["gripIds"], `${at}.gripIds`),
    fastenerIds: parseSelectionSet(row["fastenerIds"], `${at}.fastenerIds`),
  };
}
