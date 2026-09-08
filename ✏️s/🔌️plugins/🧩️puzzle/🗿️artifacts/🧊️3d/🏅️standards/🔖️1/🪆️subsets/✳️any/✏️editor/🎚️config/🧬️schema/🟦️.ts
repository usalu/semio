/** 🧬️ Puzzle3dConfig */
export interface Puzzle3dConfig {
  /** @state config */
  selection: Puzzle3dSelection;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredObjectId?: string;
  /** @state config */
  hoveredVortexFullId?: string;
  /** @state config */
  suggestionMenu?: Puzzle3dSuggestionMenu;
  /** @state config */
  overlapBudget: number;
  /** @state config */
  fillCount: number;
  /** @state config */
  fillApplyGeneration: number;
  /** @state config */
  fillAppliedCount: number;
  /** @state config */
  fillCheckpoint: number[];
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  objectKindWeights: Record<string, number>;
  /** @state config */
  vortexKindWeights: Record<string, number>;
  /** @state config */
  lodAutomatic: boolean;
  /** @state config */
  lodDepthVariable: boolean;
  /** @state config */
  gridVisible: boolean;
  /** @state config */
  lodManual: number;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridSpacing: number;
  /** @state config */
  selectableKinds: Puzzle3dSelectableKinds;
  /** @state config */
  hoveredKindId?: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  selectionModeDefault: string;
  /** @state config */
  proximityRadius: number;
  /** @state config */
  chunkSize: number;
  /** @state config */
  voxelDims: number[];
  /** @state config */
  transformMove: boolean;
  /** @state config */
  transformRotate: boolean;
  /** @state config */
  vortexShow: string;
  /** @state config */
  vortexDirection: string;
  /** @state config */
  sun: WorldSunConfig;
  /** @state config */
  camera: Puzzle3dCamera;
  /** @state config */
  windowOptions: Record<string, Puzzle3dWindowOptions>;
  /** @state config */
  activeUtilityByWindowId: Record<string, string>;
  /** @state config */
  activeToolId?: string;
  /** @state config */
  /** @state config */
  /** @state config */
  windowIds: string[];
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

export interface Puzzle3dCamera {
  position: number[];
  target: number[];
  zoom: number;
  up?: number[];
  projection: WorldProjectionConfig;
}

export interface Puzzle3dSelection {
  objectIds: SelectionSet;
  vortexIds: SelectionSet;
  attractionIds: SelectionSet;
  targetVolumeIds: SelectionSet;
  referenceIds: SelectionSet;
}

export interface Puzzle3dSelectableKinds {
  objects: boolean;
  vortices: boolean;
  attractions: boolean;
}

export interface Puzzle3dSuggestionMenu {
  x: number;
  y: number;
  windowId: string;
}

export interface Puzzle3dWindowOptions {
  selectionMethod: string;
  lodAutomatic: boolean;
  lodDepthVariable: boolean;
  gridVisible: boolean;
  lodManual: number;
  gridSnapEnabled: boolean;
  gridSpacing: number;
  selectableKinds: Puzzle3dSelectableKinds;
  engagementInput: string;
  selectionModeDefault: string;
  proximityRadius: number;
  chunkSize: number;
  voxelDims: number[];
  transformMove: boolean;
  transformRotate: boolean;
  vortexShow: string;
  vortexDirection: string;
  sun: WorldSunConfig;
  camera: Puzzle3dCamera;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dConfigGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dConfigGuardRefusal(at, why);
};

type puzzlePuzzle3dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dConfigGuardReject(at, "value is not an object");
export const puzzlePuzzle3dConfigGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dConfigGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dConfigGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dConfigGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dConfigGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dConfigGuardNumber(value, at, bounds) : puzzlePuzzle3dConfigGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dConfig(value: unknown, at = "$"): Puzzle3dConfig {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    selection: parsePuzzle3dSelection(row["selection"], `${at}.selection`),
    selectionMethod: puzzlePuzzle3dConfigGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : puzzlePuzzle3dConfigGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredVortexFullId: row["hoveredVortexFullId"] === undefined ? undefined : puzzlePuzzle3dConfigGuardString(row["hoveredVortexFullId"], `${at}.hoveredVortexFullId`),
    suggestionMenu: row["suggestionMenu"] === undefined ? undefined : parsePuzzle3dSuggestionMenu(row["suggestionMenu"], `${at}.suggestionMenu`),
    overlapBudget: puzzlePuzzle3dConfigGuardNumber(row["overlapBudget"], `${at}.overlapBudget`),
    fillCount: puzzlePuzzle3dConfigGuardInteger(row["fillCount"], `${at}.fillCount`, {"minimum": 0}),
    fillApplyGeneration: puzzlePuzzle3dConfigGuardInteger(row["fillApplyGeneration"], `${at}.fillApplyGeneration`, {"minimum": 0}),
    fillAppliedCount: puzzlePuzzle3dConfigGuardInteger(row["fillAppliedCount"], `${at}.fillAppliedCount`, {"minimum": 0}),
    fillCheckpoint: puzzlePuzzle3dConfigGuardArray(row["fillCheckpoint"], `${at}.fillCheckpoint`).map((item, index) => puzzlePuzzle3dConfigGuardInteger(item, `${at}.fillCheckpoint[${index}]`, {"minimum": 0, "maximum": 255})),
    brushCandidateIndex: puzzlePuzzle3dConfigGuardInteger(row["brushCandidateIndex"], `${at}.brushCandidateIndex`, {"minimum": 0}),
    objectKindWeights: puzzlePuzzle3dConfigGuardObject(row["objectKindWeights"], `${at}.objectKindWeights`),
    vortexKindWeights: puzzlePuzzle3dConfigGuardObject(row["vortexKindWeights"], `${at}.vortexKindWeights`),
    lodAutomatic: puzzlePuzzle3dConfigGuardBoolean(row["lodAutomatic"], `${at}.lodAutomatic`),
    lodDepthVariable: puzzlePuzzle3dConfigGuardBoolean(row["lodDepthVariable"], `${at}.lodDepthVariable`),
    gridVisible: puzzlePuzzle3dConfigGuardBoolean(row["gridVisible"], `${at}.gridVisible`),
    lodManual: puzzlePuzzle3dConfigGuardNumber(row["lodManual"], `${at}.lodManual`),
    gridSnapEnabled: puzzlePuzzle3dConfigGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridSpacing: puzzlePuzzle3dConfigGuardNumber(row["gridSpacing"], `${at}.gridSpacing`),
    selectableKinds: parsePuzzle3dSelectableKinds(row["selectableKinds"], `${at}.selectableKinds`),
    hoveredKindId: row["hoveredKindId"] === undefined ? undefined : puzzlePuzzle3dConfigGuardString(row["hoveredKindId"], `${at}.hoveredKindId`),
    engagementInput: puzzlePuzzle3dConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    selectionModeDefault: puzzlePuzzle3dConfigGuardString(row["selectionModeDefault"], `${at}.selectionModeDefault`),
    proximityRadius: puzzlePuzzle3dConfigGuardNumber(row["proximityRadius"], `${at}.proximityRadius`),
    chunkSize: puzzlePuzzle3dConfigGuardNumber(row["chunkSize"], `${at}.chunkSize`),
    voxelDims: puzzlePuzzle3dConfigGuardArray(row["voxelDims"], `${at}.voxelDims`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dConfigGuardInteger(item, `${at}.voxelDims[${index}]`, {"minimum": 0})),
    transformMove: puzzlePuzzle3dConfigGuardBoolean(row["transformMove"], `${at}.transformMove`),
    transformRotate: puzzlePuzzle3dConfigGuardBoolean(row["transformRotate"], `${at}.transformRotate`),
    vortexShow: puzzlePuzzle3dConfigGuardString(row["vortexShow"], `${at}.vortexShow`),
    vortexDirection: puzzlePuzzle3dConfigGuardString(row["vortexDirection"], `${at}.vortexDirection`),
    sun: parseWorldSunConfig(row["sun"], `${at}.sun`),
    camera: parsePuzzle3dCamera(row["camera"], `${at}.camera`),
    windowOptions: puzzlePuzzle3dConfigGuardObject(row["windowOptions"], `${at}.windowOptions`),
    activeUtilityByWindowId: puzzlePuzzle3dConfigGuardObject(row["activeUtilityByWindowId"], `${at}.activeUtilityByWindowId`),
    activeToolId: row["activeToolId"] === undefined ? undefined : puzzlePuzzle3dConfigGuardString(row["activeToolId"], `${at}.activeToolId`),
    windowIds: puzzlePuzzle3dConfigGuardArray(row["windowIds"], `${at}.windowIds`).map((item, index) => puzzlePuzzle3dConfigGuardString(item, `${at}.windowIds[${index}]`)),
  };
}

export function parseSelectionSet(value: unknown, at = "$"): SelectionSet {
  return puzzlePuzzle3dConfigGuardArray(value, `${at}`).map((item, index) => puzzlePuzzle3dConfigGuardString(item, `${at}[${index}]`));
}

export function parseWorldSunConfig(value: unknown, at = "$"): WorldSunConfig {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    enabled: puzzlePuzzle3dConfigGuardBoolean(row["enabled"], `${at}.enabled`),
    azimuth: puzzlePuzzle3dConfigGuardNumber(row["azimuth"], `${at}.azimuth`),
    elevation: puzzlePuzzle3dConfigGuardNumber(row["elevation"], `${at}.elevation`),
    intensity: puzzlePuzzle3dConfigGuardNumber(row["intensity"], `${at}.intensity`),
    color: puzzlePuzzle3dConfigGuardString(row["color"], `${at}.color`),
  };
}

export function parseWorldProjectionConfig(value: unknown, at = "$"): WorldProjectionConfig {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    kind: puzzlePuzzle3dConfigGuardString(row["kind"], `${at}.kind`),
    orthographicView: puzzlePuzzle3dConfigGuardString(row["orthographicView"], `${at}.orthographicView`),
    axonometricVariant: puzzlePuzzle3dConfigGuardString(row["axonometricVariant"], `${at}.axonometricVariant`),
    axonometricAngleA: puzzlePuzzle3dConfigGuardNumber(row["axonometricAngleA"], `${at}.axonometricAngleA`),
    axonometricAngleB: puzzlePuzzle3dConfigGuardNumber(row["axonometricAngleB"], `${at}.axonometricAngleB`),
    axonometricQuadrant: puzzlePuzzle3dConfigGuardString(row["axonometricQuadrant"], `${at}.axonometricQuadrant`),
    obliqueVariant: puzzlePuzzle3dConfigGuardString(row["obliqueVariant"], `${at}.obliqueVariant`),
    obliqueAngle: puzzlePuzzle3dConfigGuardNumber(row["obliqueAngle"], `${at}.obliqueAngle`),
    obliqueDepth: puzzlePuzzle3dConfigGuardNumber(row["obliqueDepth"], `${at}.obliqueDepth`),
    onePointAxis: puzzlePuzzle3dConfigGuardString(row["onePointAxis"], `${at}.onePointAxis`),
    fov: puzzlePuzzle3dConfigGuardNumber(row["fov"], `${at}.fov`),
    twoPointShift: puzzlePuzzle3dConfigGuardNumber(row["twoPointShift"], `${at}.twoPointShift`),
    curvilinearFov: puzzlePuzzle3dConfigGuardNumber(row["curvilinearFov"], `${at}.curvilinearFov`),
    curvilinearStrength: puzzlePuzzle3dConfigGuardNumber(row["curvilinearStrength"], `${at}.curvilinearStrength`),
    curvilinearMapping: puzzlePuzzle3dConfigGuardString(row["curvilinearMapping"], `${at}.curvilinearMapping`),
  };
}

export function parsePuzzle3dCamera(value: unknown, at = "$"): Puzzle3dCamera {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    position: puzzlePuzzle3dConfigGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dConfigGuardNumber(item, `${at}.position[${index}]`)),
    target: puzzlePuzzle3dConfigGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dConfigGuardNumber(item, `${at}.target[${index}]`)),
    zoom: puzzlePuzzle3dConfigGuardNumber(row["zoom"], `${at}.zoom`),
    up: row["up"] === undefined ? undefined : puzzlePuzzle3dConfigGuardArray(row["up"], `${at}.up`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dConfigGuardNumber(item, `${at}.up[${index}]`)),
    projection: parseWorldProjectionConfig(row["projection"], `${at}.projection`),
  };
}

export function parsePuzzle3dSelection(value: unknown, at = "$"): Puzzle3dSelection {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    objectIds: parseSelectionSet(row["objectIds"], `${at}.objectIds`),
    vortexIds: parseSelectionSet(row["vortexIds"], `${at}.vortexIds`),
    attractionIds: parseSelectionSet(row["attractionIds"], `${at}.attractionIds`),
    targetVolumeIds: parseSelectionSet(row["targetVolumeIds"], `${at}.targetVolumeIds`),
    referenceIds: parseSelectionSet(row["referenceIds"], `${at}.referenceIds`),
  };
}

export function parsePuzzle3dSelectableKinds(value: unknown, at = "$"): Puzzle3dSelectableKinds {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    objects: puzzlePuzzle3dConfigGuardBoolean(row["objects"], `${at}.objects`),
    vortices: puzzlePuzzle3dConfigGuardBoolean(row["vortices"], `${at}.vortices`),
    attractions: puzzlePuzzle3dConfigGuardBoolean(row["attractions"], `${at}.attractions`),
  };
}

export function parsePuzzle3dSuggestionMenu(value: unknown, at = "$"): Puzzle3dSuggestionMenu {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    x: puzzlePuzzle3dConfigGuardNumber(row["x"], `${at}.x`),
    y: puzzlePuzzle3dConfigGuardNumber(row["y"], `${at}.y`),
    windowId: puzzlePuzzle3dConfigGuardString(row["windowId"], `${at}.windowId`),
  };
}

export function parsePuzzle3dWindowOptions(value: unknown, at = "$"): Puzzle3dWindowOptions {
  const row = puzzlePuzzle3dConfigGuardObject(value, at);
  return {
    selectionMethod: puzzlePuzzle3dConfigGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    lodAutomatic: puzzlePuzzle3dConfigGuardBoolean(row["lodAutomatic"], `${at}.lodAutomatic`),
    lodDepthVariable: puzzlePuzzle3dConfigGuardBoolean(row["lodDepthVariable"], `${at}.lodDepthVariable`),
    gridVisible: puzzlePuzzle3dConfigGuardBoolean(row["gridVisible"], `${at}.gridVisible`),
    lodManual: puzzlePuzzle3dConfigGuardNumber(row["lodManual"], `${at}.lodManual`),
    gridSnapEnabled: puzzlePuzzle3dConfigGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridSpacing: puzzlePuzzle3dConfigGuardNumber(row["gridSpacing"], `${at}.gridSpacing`),
    selectableKinds: parsePuzzle3dSelectableKinds(row["selectableKinds"], `${at}.selectableKinds`),
    engagementInput: puzzlePuzzle3dConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    selectionModeDefault: puzzlePuzzle3dConfigGuardString(row["selectionModeDefault"], `${at}.selectionModeDefault`),
    proximityRadius: puzzlePuzzle3dConfigGuardNumber(row["proximityRadius"], `${at}.proximityRadius`),
    chunkSize: puzzlePuzzle3dConfigGuardNumber(row["chunkSize"], `${at}.chunkSize`),
    voxelDims: puzzlePuzzle3dConfigGuardArray(row["voxelDims"], `${at}.voxelDims`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dConfigGuardInteger(item, `${at}.voxelDims[${index}]`, {"minimum": 0})),
    transformMove: puzzlePuzzle3dConfigGuardBoolean(row["transformMove"], `${at}.transformMove`),
    transformRotate: puzzlePuzzle3dConfigGuardBoolean(row["transformRotate"], `${at}.transformRotate`),
    vortexShow: puzzlePuzzle3dConfigGuardString(row["vortexShow"], `${at}.vortexShow`),
    vortexDirection: puzzlePuzzle3dConfigGuardString(row["vortexDirection"], `${at}.vortexDirection`),
    sun: parseWorldSunConfig(row["sun"], `${at}.sun`),
    camera: parsePuzzle3dCamera(row["camera"], `${at}.camera`),
  };
}
