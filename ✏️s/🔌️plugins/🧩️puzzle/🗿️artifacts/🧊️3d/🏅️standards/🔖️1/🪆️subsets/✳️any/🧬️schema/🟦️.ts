/** 🧬️ Puzzle3d artifact schema — every field with its state class. */

export interface Puzzle3dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  domain: string;
  /** @state artifact */
  meta: Puzzle3dMeta;
  /** @state artifact */
  objects: Puzzle3dObject[];
  /** @state artifact */
  attractions: Puzzle3dAttraction[];
  /** @state artifact */
  targetVolumes: Puzzle3dTargetVolume[];
  /** @state artifact */
  references: Puzzle3dReference[];
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
  activeUtilityId: string;
  /** @state config */
  cameraPositionX: number;
  /** @state config */
  cameraPositionY: number;
  /** @state config */
  cameraPositionZ: number;
  /** @state config */
  cameraTargetX: number;
  /** @state config */
  cameraTargetY: number;
  /** @state config */
  cameraTargetZ: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  selectionModeDefault: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  gridVisible: boolean;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridSpacing: number;
  /** @state config */
  overlapBudget: number;
  /** @state config */
  fillCount: number;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  lodAutomatic: boolean;
  /** @state config */
  lodDepthVariable: boolean;
  /** @state config */
  lodManual: number;
  /** @state config */
  proximityRadius: number;
  /** @state config */
  /** @state config */
  runtimeExtrasJson: string;
  /** @state artifact */
  hoveredObjectId?: string;
  /** @state artifact */
  hoveredVortexFullId?: string;
  /** @state artifact */
  hoveredKindId?: string;
  /** @state artifact */
  previewSeq: number;
}

export type Puzzle3dObjectAnchor = "fixed" | "derived";
export type Puzzle3dCompatSpecificity = "general" | "object" | "attraction" | "vortex" | "cable";

export interface Puzzle3dVortex {
  id: string;
  vortexKind?: string;
  label?: string;
  position: [number, number, number];
  direction?: [number, number, number];
  radius?: number;
  hidden?: boolean;
  locked?: boolean;
}

export interface Puzzle3dObject {
  id: string;
  label?: string;
  objectKind?: string;
  anchor?: Puzzle3dObjectAnchor;
  origin: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: number | [number, number, number];
  meshUrl?: string;
  vortices?: Puzzle3dVortex[];
  hidden?: boolean;
  locked?: boolean;
}

export interface Puzzle3dAttraction {
  id?: string;
  attracting: string;
  attracted: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}

export interface Puzzle3dRepresentation {
  id: string;
  name: string;
  url: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

export interface Puzzle3dCatalogVortexTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  vortexKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

export interface Puzzle3dCatalogObjectKind {
  id: string;
  name: string;
  label: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle3dRepresentation[];
  vortices?: Puzzle3dCatalogVortexTemplate[];
  attributes?: Array<{ id?: string; key: string; value: string; definition?: string }>;
  authors?: Array<{ id?: string; name: string; email?: string; role?: string; rank?: number }>;
}

export interface Puzzle3dCatalogVortexKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultCableKind?: string;
}

export interface Puzzle3dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle3dCompatSpecificity;
}

export interface Puzzle3dMeta {
  kindCatalogs?: {
    objects?: Puzzle3dCatalogObjectKind[];
    vortices?: Puzzle3dCatalogVortexKind[];
    cables?: unknown[];
    attractions?: unknown[];
  };
  kindCompatibility?: Puzzle3dKindCompatibility[];
}

export interface Puzzle3dTargetVolume { id: string; [key: string]: unknown; }
export interface Puzzle3dReference { id: string; [key: string]: unknown; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dArtifactGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dArtifactGuardRefusal(at, why);
};

type puzzlePuzzle3dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dArtifactGuardReject(at, "value is not an object");
export const puzzlePuzzle3dArtifactGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dArtifactGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dArtifactGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dArtifactGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dArtifactGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dArtifactGuardNumber(value, at, bounds) : puzzlePuzzle3dArtifactGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dArtifact(value: unknown, at = "$"): Puzzle3dArtifact {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    schema: puzzlePuzzle3dArtifactGuardString(row["schema"], `${at}.schema`),
    domain: puzzlePuzzle3dArtifactGuardString(row["domain"], `${at}.domain`),
    meta: parsePuzzle3dMeta(row["meta"], `${at}.meta`),
    objects: puzzlePuzzle3dArtifactGuardArray(row["objects"], `${at}.objects`).map((item, index) => parsePuzzle3dObject(item, `${at}.objects[${index}]`)),
    attractions: puzzlePuzzle3dArtifactGuardArray(row["attractions"], `${at}.attractions`).map((item, index) => parsePuzzle3dAttraction(item, `${at}.attractions[${index}]`)),
    targetVolumes: puzzlePuzzle3dArtifactGuardArray(row["targetVolumes"], `${at}.targetVolumes`).map((item, index) => parsePuzzle3dTargetVolume(item, `${at}.targetVolumes[${index}]`)),
    references: puzzlePuzzle3dArtifactGuardArray(row["references"], `${at}.references`).map((item, index) => parsePuzzle3dReference(item, `${at}.references[${index}]`)),
    selectedObjectIds: puzzlePuzzle3dArtifactGuardArray(row["selectedObjectIds"], `${at}.selectedObjectIds`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.selectedObjectIds[${index}]`)),
    selectedVortexIds: puzzlePuzzle3dArtifactGuardArray(row["selectedVortexIds"], `${at}.selectedVortexIds`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.selectedVortexIds[${index}]`)),
    selectedAttractionIds: puzzlePuzzle3dArtifactGuardArray(row["selectedAttractionIds"], `${at}.selectedAttractionIds`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.selectedAttractionIds[${index}]`)),
    selectedTargetVolumeIds: puzzlePuzzle3dArtifactGuardArray(row["selectedTargetVolumeIds"], `${at}.selectedTargetVolumeIds`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.selectedTargetVolumeIds[${index}]`)),
    selectedReferenceIds: puzzlePuzzle3dArtifactGuardArray(row["selectedReferenceIds"], `${at}.selectedReferenceIds`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.selectedReferenceIds[${index}]`)),
    activeUtilityId: puzzlePuzzle3dArtifactGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    cameraPositionX: puzzlePuzzle3dArtifactGuardNumber(row["cameraPositionX"], `${at}.cameraPositionX`),
    cameraPositionY: puzzlePuzzle3dArtifactGuardNumber(row["cameraPositionY"], `${at}.cameraPositionY`),
    cameraPositionZ: puzzlePuzzle3dArtifactGuardNumber(row["cameraPositionZ"], `${at}.cameraPositionZ`),
    cameraTargetX: puzzlePuzzle3dArtifactGuardNumber(row["cameraTargetX"], `${at}.cameraTargetX`),
    cameraTargetY: puzzlePuzzle3dArtifactGuardNumber(row["cameraTargetY"], `${at}.cameraTargetY`),
    cameraTargetZ: puzzlePuzzle3dArtifactGuardNumber(row["cameraTargetZ"], `${at}.cameraTargetZ`),
    cameraZoom: puzzlePuzzle3dArtifactGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    selectionMethod: puzzlePuzzle3dArtifactGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    selectionModeDefault: puzzlePuzzle3dArtifactGuardString(row["selectionModeDefault"], `${at}.selectionModeDefault`),
    engagementInput: puzzlePuzzle3dArtifactGuardString(row["engagementInput"], `${at}.engagementInput`),
    gridVisible: puzzlePuzzle3dArtifactGuardBoolean(row["gridVisible"], `${at}.gridVisible`),
    gridSnapEnabled: puzzlePuzzle3dArtifactGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridSpacing: puzzlePuzzle3dArtifactGuardNumber(row["gridSpacing"], `${at}.gridSpacing`),
    overlapBudget: puzzlePuzzle3dArtifactGuardNumber(row["overlapBudget"], `${at}.overlapBudget`),
    fillCount: puzzlePuzzle3dArtifactGuardInteger(row["fillCount"], `${at}.fillCount`, {"minimum": 0}),
    brushCandidateIndex: puzzlePuzzle3dArtifactGuardInteger(row["brushCandidateIndex"], `${at}.brushCandidateIndex`, {"minimum": 0}),
    lodAutomatic: puzzlePuzzle3dArtifactGuardBoolean(row["lodAutomatic"], `${at}.lodAutomatic`),
    lodDepthVariable: puzzlePuzzle3dArtifactGuardBoolean(row["lodDepthVariable"], `${at}.lodDepthVariable`),
    lodManual: puzzlePuzzle3dArtifactGuardNumber(row["lodManual"], `${at}.lodManual`),
    proximityRadius: puzzlePuzzle3dArtifactGuardNumber(row["proximityRadius"], `${at}.proximityRadius`),
    runtimeExtrasJson: puzzlePuzzle3dArtifactGuardString(row["runtimeExtrasJson"], `${at}.runtimeExtrasJson`),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredVortexFullId: row["hoveredVortexFullId"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["hoveredVortexFullId"], `${at}.hoveredVortexFullId`),
    hoveredKindId: row["hoveredKindId"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["hoveredKindId"], `${at}.hoveredKindId`),
    previewSeq: puzzlePuzzle3dArtifactGuardInteger(row["previewSeq"], `${at}.previewSeq`),
  };
}

export function parsePuzzle3dMeta(value: unknown, at = "$"): Puzzle3dMeta {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    kindCatalogs: row["kindCatalogs"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardObject(row["kindCatalogs"], `${at}.kindCatalogs`),
    kindCompatibility: row["kindCompatibility"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["kindCompatibility"], `${at}.kindCompatibility`).map((item, index) => parsePuzzle3dKindCompatibility(item, `${at}.kindCompatibility[${index}]`)),
  };
}

export function parsePuzzle3dAttraction(value: unknown, at = "$"): Puzzle3dAttraction {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    attracting: puzzlePuzzle3dArtifactGuardString(row["attracting"], `${at}.attracting`),
    attracted: puzzlePuzzle3dArtifactGuardString(row["attracted"], `${at}.attracted`),
    gap: row["gap"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["gap"], `${at}.gap`),
    shift: row["shift"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["shift"], `${at}.shift`),
    rise: row["rise"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["rise"], `${at}.rise`),
    rotation: row["rotation"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    turn: row["turn"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["turn"], `${at}.turn`),
    tilt: row["tilt"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["tilt"], `${at}.tilt`),
    x: row["x"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePuzzle3dTargetVolume(value: unknown, at = "$"): Puzzle3dTargetVolume {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
  };
}

export function parsePuzzle3dReference(value: unknown, at = "$"): Puzzle3dReference {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
  };
}

export function parsePuzzle3dVortex(value: unknown, at = "$"): Puzzle3dVortex {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    vortexKind: row["vortexKind"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["vortexKind"], `${at}.vortexKind`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`),
    position: puzzlePuzzle3dArtifactGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dArtifactGuardNumber(item, `${at}.position[${index}]`)),
    direction: row["direction"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["direction"], `${at}.direction`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dArtifactGuardNumber(item, `${at}.direction[${index}]`)),
    radius: row["radius"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["radius"], `${at}.radius`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle3dKindCompatibility(value: unknown, at = "$"): Puzzle3dKindCompatibility {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    source: puzzlePuzzle3dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle3dArtifactGuardString(row["target"], `${at}.target`),
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardMember(row["specificity"], `${at}.specificity`, ["general", "object", "attraction", "vortex", "cable"] as const),
  };
}

export function parsePuzzle3dRepresentation(value: unknown, at = "$"): Puzzle3dRepresentation {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
    url: puzzlePuzzle3dArtifactGuardString(row["url"], `${at}.url`),
    mime: row["mime"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["mime"], `${at}.mime`),
    tags: row["tags"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["tags"], `${at}.tags`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.tags[${index}]`)),
    lod: row["lod"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["lod"], `${at}.lod`),
    description: row["description"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["description"], `${at}.description`),
  };
}

export function parsePuzzle3dCatalogVortexTemplate(value: unknown, at = "$"): Puzzle3dCatalogVortexTemplate {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`),
    description: row["description"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["icon"], `${at}.icon`),
    vortexKind: row["vortexKind"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["vortexKind"], `${at}.vortexKind`),
    point: row["point"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["point"], `${at}.point`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dArtifactGuardNumber(item, `${at}.point[${index}]`)),
    direction: row["direction"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["direction"], `${at}.direction`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle3dArtifactGuardNumber(item, `${at}.direction[${index}]`)),
    t: row["t"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["t"], `${at}.t`),
    mandatory: row["mandatory"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["mandatory"], `${at}.mandatory`),
    radius: row["radius"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardNumber(row["radius"], `${at}.radius`),
  };
}

export function parsePuzzle3dCatalogObjectKind(value: unknown, at = "$"): Puzzle3dCatalogObjectKind {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
    label: puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`),
    description: row["description"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["icon"], `${at}.icon`),
    image: row["image"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["image"], `${at}.image`),
    unit: row["unit"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["unit"], `${at}.unit`),
    abstract: row["abstract"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["abstract"], `${at}.abstract`),
    baseKinds: row["baseKinds"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["baseKinds"], `${at}.baseKinds`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.baseKinds[${index}]`)),
    representations: row["representations"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["representations"], `${at}.representations`).map((item, index) => parsePuzzle3dRepresentation(item, `${at}.representations[${index}]`)),
    vortices: row["vortices"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["vortices"], `${at}.vortices`).map((item, index) => parsePuzzle3dCatalogVortexTemplate(item, `${at}.vortices[${index}]`)),
    attributes: row["attributes"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => puzzlePuzzle3dArtifactGuardObject(item, `${at}.attributes[${index}]`)),
    authors: row["authors"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => puzzlePuzzle3dArtifactGuardObject(item, `${at}.authors[${index}]`)),
  };
}

export function parsePuzzle3dCatalogVortexKind(value: unknown, at = "$"): Puzzle3dCatalogVortexKind {
  const row = puzzlePuzzle3dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    code: row["code"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["code"], `${at}.code`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`),
    order: row["order"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardInteger(row["order"], `${at}.order`),
    compatibleWith: row["compatibleWith"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["compatibleWith"], `${at}.compatibleWith`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.compatibleWith[${index}]`)),
    description: row["description"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["icon"], `${at}.icon`),
    color: row["color"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["color"], `${at}.color`),
    defaultCableKind: row["defaultCableKind"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["defaultCableKind"], `${at}.defaultCableKind`),
  };
}
