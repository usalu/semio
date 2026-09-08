/** 🧬️ Puzzle2d artifact schema — every field with its state class. */

export interface Puzzle2dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  camera: Puzzle2dCamera;
  /** @state artifact */
  nodes: Puzzle2dNode[];
  /** @state artifact */
  edges: Puzzle2dEdge[];
  /** @state artifact */
  meta: Puzzle2dMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  suggestionOffset: number;
  /** @state config */
  fillCount: number;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  brushCandidateSourceHandleId: string;
  /** @state config */
  /** @state config */
  /** @state config */
  lodModeByPaneJson: string;
  /** @state config */
  engagementInputByPaneJson: string;
  /** @state config */
  brushCandidatesJson: string;
  /** @state config */
  nodeKindWeightsJson: string;
  /** @state config */
  handleKindWeightsJson: string;
  /** @state config */
  activeUtilityByWindowIdJson: string;
  /** @state artifact */
  hoveredNodeId?: string;
  /** @state artifact */
  previewSeq: number;
}

export type Puzzle2dNodeAnchor = "fixed" | "derived";

export interface Puzzle2dCamera {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle2dHandle {
  id: string;
  handleKind?: string;
  angle: number;
  radius?: number;
  color?: string;
  iconKind?: string;
  scale?: number;
  visible?: boolean;
  locked?: boolean;
}

export interface Puzzle2dNode {
  id: string;
  nodeKind?: string;
  shape?: string;
  x: number;
  y: number;
  radius?: number;
  width?: number;
  height?: number;
  text?: string;
  iconKind?: string;
  root?: boolean;
  scale?: number;
  visible?: boolean;
  locked?: boolean;
  anchor: Puzzle2dNodeAnchor;
  handles: Puzzle2dHandle[];
}

export interface Puzzle2dEdge {
  id: string;
  source: string;
  target: string;
  edgeKind?: string;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
  sourceTip?: string;
  targetTip?: string;
  visible?: boolean;
  locked?: boolean;
}

export type Puzzle2dCompatSpecificity = "general" | "node" | "edge" | "handle" | "wire" | "vortex";

export interface Puzzle2dKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle2dCompatSpecificity;
}

export interface Puzzle2dAttribute {
  id: string;
  key: string;
  value: string;
  definition?: string;
}

export interface Puzzle2dAuthor {
  id: string;
  name: string;
  email: string;
  role?: string;
  rank?: number;
}

export interface Puzzle2dRepresentation {
  id: string;
  name: string;
  url: string;
  mime: string;
  tags: string[];
  lod?: string;
  description: string;
}

export interface Puzzle2dHandleTemplate {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  handleKind?: string;
  angle: number;
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

export interface Puzzle2dCatalogNodeKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  image: string;
  unit: string;
  abstract: boolean;
  baseKinds: string[];
  representations: Puzzle2dRepresentation[];
  handles: Puzzle2dHandleTemplate[];
  attributes: Puzzle2dAttribute[];
  authors: Puzzle2dAuthor[];
}

export interface Puzzle2dCatalogHandleKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith: string[];
  description: string;
  icon: string;
  color: string;
  defaultWireKind: string;
}

export interface Puzzle2dCatalogEdgeKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  color: string;
}

export interface Puzzle2dCatalogWireKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  color: string;
  defaultEdgeKind: string;
}

export interface Puzzle2dKindCatalogs {
  nodes: Puzzle2dCatalogNodeKind[];
  handles: Puzzle2dCatalogHandleKind[];
  edges: Puzzle2dCatalogEdgeKind[];
  wires: Puzzle2dCatalogWireKind[];
}

export interface Puzzle2dMeta {
  manifestId?: string;
  kindCompatibility: Puzzle2dKindCompatibility[];
  kindCatalogs?: Puzzle2dKindCatalogs;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dArtifactGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dArtifactGuardRefusal(at, why);
};

type puzzlePuzzle2dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dArtifactGuardReject(at, "value is not an object");
export const puzzlePuzzle2dArtifactGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dArtifactGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dArtifactGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dArtifactGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dArtifactGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dArtifactGuardNumber(value, at, bounds) : puzzlePuzzle2dArtifactGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dArtifact(value: unknown, at = "$"): Puzzle2dArtifact {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    schema: puzzlePuzzle2dArtifactGuardString(row["schema"], `${at}.schema`),
    camera: parsePuzzle2dCamera(row["camera"], `${at}.camera`),
    nodes: puzzlePuzzle2dArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parsePuzzle2dNode(item, `${at}.nodes[${index}]`)),
    edges: puzzlePuzzle2dArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => parsePuzzle2dEdge(item, `${at}.edges[${index}]`)),
    meta: parsePuzzle2dMeta(row["meta"], `${at}.meta`),
    selectedIds: puzzlePuzzle2dArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => puzzlePuzzle2dArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
    activeUtilityId: puzzlePuzzle2dArtifactGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    cameraX: puzzlePuzzle2dArtifactGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: puzzlePuzzle2dArtifactGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: puzzlePuzzle2dArtifactGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    selectionMethod: puzzlePuzzle2dArtifactGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    gridSnapEnabled: puzzlePuzzle2dArtifactGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridFactor: puzzlePuzzle2dArtifactGuardNumber(row["gridFactor"], `${at}.gridFactor`),
    suggestionOffset: puzzlePuzzle2dArtifactGuardNumber(row["suggestionOffset"], `${at}.suggestionOffset`),
    fillCount: puzzlePuzzle2dArtifactGuardInteger(row["fillCount"], `${at}.fillCount`, {"minimum": 0}),
    brushCandidateIndex: puzzlePuzzle2dArtifactGuardInteger(row["brushCandidateIndex"], `${at}.brushCandidateIndex`, {"minimum": 0}),
    brushCandidateSourceHandleId: puzzlePuzzle2dArtifactGuardString(row["brushCandidateSourceHandleId"], `${at}.brushCandidateSourceHandleId`),
    lodModeByPaneJson: puzzlePuzzle2dArtifactGuardString(row["lodModeByPaneJson"], `${at}.lodModeByPaneJson`),
    engagementInputByPaneJson: puzzlePuzzle2dArtifactGuardString(row["engagementInputByPaneJson"], `${at}.engagementInputByPaneJson`),
    brushCandidatesJson: puzzlePuzzle2dArtifactGuardString(row["brushCandidatesJson"], `${at}.brushCandidatesJson`),
    nodeKindWeightsJson: puzzlePuzzle2dArtifactGuardString(row["nodeKindWeightsJson"], `${at}.nodeKindWeightsJson`),
    handleKindWeightsJson: puzzlePuzzle2dArtifactGuardString(row["handleKindWeightsJson"], `${at}.handleKindWeightsJson`),
    activeUtilityByWindowIdJson: puzzlePuzzle2dArtifactGuardString(row["activeUtilityByWindowIdJson"], `${at}.activeUtilityByWindowIdJson`),
    hoveredNodeId: row["hoveredNodeId"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["hoveredNodeId"], `${at}.hoveredNodeId`),
    previewSeq: puzzlePuzzle2dArtifactGuardInteger(row["previewSeq"], `${at}.previewSeq`),
  };
}

export function parsePuzzle2dCamera(value: unknown, at = "$"): Puzzle2dCamera {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    x: puzzlePuzzle2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: puzzlePuzzle2dArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: puzzlePuzzle2dArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parsePuzzle2dNode(value: unknown, at = "$"): Puzzle2dNode {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    nodeKind: row["nodeKind"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["nodeKind"], `${at}.nodeKind`),
    shape: row["shape"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["shape"], `${at}.shape`),
    x: puzzlePuzzle2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: puzzlePuzzle2dArtifactGuardNumber(row["y"], `${at}.y`),
    radius: row["radius"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["radius"], `${at}.radius`),
    width: row["width"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["height"], `${at}.height`),
    text: row["text"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["text"], `${at}.text`),
    iconKind: row["iconKind"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["iconKind"], `${at}.iconKind`),
    root: row["root"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["root"], `${at}.root`),
    scale: row["scale"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["scale"], `${at}.scale`),
    visible: row["visible"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["locked"], `${at}.locked`),
    anchor: parsePuzzle2dNodeAnchor(row["anchor"], `${at}.anchor`),
    handles: puzzlePuzzle2dArtifactGuardArray(row["handles"], `${at}.handles`).map((item, index) => parsePuzzle2dHandle(item, `${at}.handles[${index}]`)),
  };
}

export function parsePuzzle2dEdge(value: unknown, at = "$"): Puzzle2dEdge {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    source: puzzlePuzzle2dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle2dArtifactGuardString(row["target"], `${at}.target`),
    edgeKind: row["edgeKind"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["edgeKind"], `${at}.edgeKind`),
    gap: puzzlePuzzle2dArtifactGuardNumber(row["gap"], `${at}.gap`),
    shift: puzzlePuzzle2dArtifactGuardNumber(row["shift"], `${at}.shift`),
    rise: puzzlePuzzle2dArtifactGuardNumber(row["rise"], `${at}.rise`),
    rotation: puzzlePuzzle2dArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    turn: puzzlePuzzle2dArtifactGuardNumber(row["turn"], `${at}.turn`),
    tilt: puzzlePuzzle2dArtifactGuardNumber(row["tilt"], `${at}.tilt`),
    x: puzzlePuzzle2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: puzzlePuzzle2dArtifactGuardNumber(row["y"], `${at}.y`),
    sourceTip: row["sourceTip"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["sourceTip"], `${at}.sourceTip`),
    targetTip: row["targetTip"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["targetTip"], `${at}.targetTip`),
    visible: row["visible"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle2dMeta(value: unknown, at = "$"): Puzzle2dMeta {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    manifestId: row["manifestId"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["manifestId"], `${at}.manifestId`),
    kindCompatibility: puzzlePuzzle2dArtifactGuardArray(row["kindCompatibility"], `${at}.kindCompatibility`).map((item, index) => parsePuzzle2dKindCompatibility(item, `${at}.kindCompatibility[${index}]`)),
    kindCatalogs: row["kindCatalogs"] === undefined ? undefined : parsePuzzle2dKindCatalogs(row["kindCatalogs"], `${at}.kindCatalogs`),
  };
}

export function parsePuzzle2dNodeAnchor(value: unknown, at = "$"): Puzzle2dNodeAnchor {
  return puzzlePuzzle2dArtifactGuardMember(value, `${at}`, ["fixed", "derived"] as const);
}

export function parsePuzzle2dHandle(value: unknown, at = "$"): Puzzle2dHandle {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    handleKind: row["handleKind"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["handleKind"], `${at}.handleKind`),
    angle: puzzlePuzzle2dArtifactGuardNumber(row["angle"], `${at}.angle`),
    radius: row["radius"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["radius"], `${at}.radius`),
    color: row["color"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["color"], `${at}.color`),
    iconKind: row["iconKind"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["iconKind"], `${at}.iconKind`),
    scale: row["scale"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["scale"], `${at}.scale`),
    visible: row["visible"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle2dCompatSpecificity(value: unknown, at = "$"): Puzzle2dCompatSpecificity {
  return puzzlePuzzle2dArtifactGuardMember(value, `${at}`, ["general", "node", "edge", "handle", "wire", "vortex"] as const);
}

export function parsePuzzle2dKindCompatibility(value: unknown, at = "$"): Puzzle2dKindCompatibility {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    source: puzzlePuzzle2dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle2dArtifactGuardString(row["target"], `${at}.target`),
    bidirectional: puzzlePuzzle2dArtifactGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: puzzlePuzzle2dArtifactGuardBoolean(row["important"], `${at}.important`),
    specificity: parsePuzzle2dCompatSpecificity(row["specificity"], `${at}.specificity`),
  };
}

export function parsePuzzle2dAttribute(value: unknown, at = "$"): Puzzle2dAttribute {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    key: puzzlePuzzle2dArtifactGuardString(row["key"], `${at}.key`),
    value: puzzlePuzzle2dArtifactGuardString(row["value"], `${at}.value`),
    definition: row["definition"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["definition"], `${at}.definition`),
  };
}

export function parsePuzzle2dAuthor(value: unknown, at = "$"): Puzzle2dAuthor {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle2dArtifactGuardString(row["name"], `${at}.name`),
    email: puzzlePuzzle2dArtifactGuardString(row["email"], `${at}.email`),
    role: row["role"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["role"], `${at}.role`),
    rank: row["rank"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardInteger(row["rank"], `${at}.rank`),
  };
}

export function parsePuzzle2dRepresentation(value: unknown, at = "$"): Puzzle2dRepresentation {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle2dArtifactGuardString(row["name"], `${at}.name`),
    url: puzzlePuzzle2dArtifactGuardString(row["url"], `${at}.url`),
    mime: puzzlePuzzle2dArtifactGuardString(row["mime"], `${at}.mime`),
    tags: puzzlePuzzle2dArtifactGuardArray(row["tags"], `${at}.tags`).map((item, index) => puzzlePuzzle2dArtifactGuardString(item, `${at}.tags[${index}]`)),
    lod: row["lod"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["lod"], `${at}.lod`),
    description: puzzlePuzzle2dArtifactGuardString(row["description"], `${at}.description`),
  };
}

export function parsePuzzle2dHandleTemplate(value: unknown, at = "$"): Puzzle2dHandleTemplate {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle2dArtifactGuardString(row["name"], `${at}.name`),
    label: puzzlePuzzle2dArtifactGuardString(row["label"], `${at}.label`),
    description: puzzlePuzzle2dArtifactGuardString(row["description"], `${at}.description`),
    icon: puzzlePuzzle2dArtifactGuardString(row["icon"], `${at}.icon`),
    handleKind: row["handleKind"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["handleKind"], `${at}.handleKind`),
    angle: puzzlePuzzle2dArtifactGuardNumber(row["angle"], `${at}.angle`),
    t: row["t"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["t"], `${at}.t`),
    mandatory: row["mandatory"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardBoolean(row["mandatory"], `${at}.mandatory`),
    radius: row["radius"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardNumber(row["radius"], `${at}.radius`),
  };
}

export function parsePuzzle2dCatalogNodeKind(value: unknown, at = "$"): Puzzle2dCatalogNodeKind {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle2dArtifactGuardString(row["name"], `${at}.name`),
    label: puzzlePuzzle2dArtifactGuardString(row["label"], `${at}.label`),
    description: puzzlePuzzle2dArtifactGuardString(row["description"], `${at}.description`),
    icon: puzzlePuzzle2dArtifactGuardString(row["icon"], `${at}.icon`),
    image: puzzlePuzzle2dArtifactGuardString(row["image"], `${at}.image`),
    unit: puzzlePuzzle2dArtifactGuardString(row["unit"], `${at}.unit`),
    abstract: puzzlePuzzle2dArtifactGuardBoolean(row["abstract"], `${at}.abstract`),
    baseKinds: puzzlePuzzle2dArtifactGuardArray(row["baseKinds"], `${at}.baseKinds`).map((item, index) => puzzlePuzzle2dArtifactGuardString(item, `${at}.baseKinds[${index}]`)),
    representations: puzzlePuzzle2dArtifactGuardArray(row["representations"], `${at}.representations`).map((item, index) => parsePuzzle2dRepresentation(item, `${at}.representations[${index}]`)),
    handles: puzzlePuzzle2dArtifactGuardArray(row["handles"], `${at}.handles`).map((item, index) => parsePuzzle2dHandleTemplate(item, `${at}.handles[${index}]`)),
    attributes: puzzlePuzzle2dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => parsePuzzle2dAttribute(item, `${at}.attributes[${index}]`)),
    authors: puzzlePuzzle2dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => parsePuzzle2dAuthor(item, `${at}.authors[${index}]`)),
  };
}

export function parsePuzzle2dCatalogHandleKind(value: unknown, at = "$"): Puzzle2dCatalogHandleKind {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    code: row["code"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["code"], `${at}.code`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardString(row["label"], `${at}.label`),
    order: row["order"] === undefined ? undefined : puzzlePuzzle2dArtifactGuardInteger(row["order"], `${at}.order`),
    compatibleWith: puzzlePuzzle2dArtifactGuardArray(row["compatibleWith"], `${at}.compatibleWith`).map((item, index) => puzzlePuzzle2dArtifactGuardString(item, `${at}.compatibleWith[${index}]`)),
    description: puzzlePuzzle2dArtifactGuardString(row["description"], `${at}.description`),
    icon: puzzlePuzzle2dArtifactGuardString(row["icon"], `${at}.icon`),
    color: puzzlePuzzle2dArtifactGuardString(row["color"], `${at}.color`),
    defaultWireKind: puzzlePuzzle2dArtifactGuardString(row["defaultWireKind"], `${at}.defaultWireKind`),
  };
}

export function parsePuzzle2dCatalogEdgeKind(value: unknown, at = "$"): Puzzle2dCatalogEdgeKind {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle2dArtifactGuardString(row["name"], `${at}.name`),
    label: puzzlePuzzle2dArtifactGuardString(row["label"], `${at}.label`),
    description: puzzlePuzzle2dArtifactGuardString(row["description"], `${at}.description`),
    icon: puzzlePuzzle2dArtifactGuardString(row["icon"], `${at}.icon`),
    color: puzzlePuzzle2dArtifactGuardString(row["color"], `${at}.color`),
  };
}

export function parsePuzzle2dCatalogWireKind(value: unknown, at = "$"): Puzzle2dCatalogWireKind {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle2dArtifactGuardString(row["id"], `${at}.id`),
    name: puzzlePuzzle2dArtifactGuardString(row["name"], `${at}.name`),
    label: puzzlePuzzle2dArtifactGuardString(row["label"], `${at}.label`),
    description: puzzlePuzzle2dArtifactGuardString(row["description"], `${at}.description`),
    icon: puzzlePuzzle2dArtifactGuardString(row["icon"], `${at}.icon`),
    color: puzzlePuzzle2dArtifactGuardString(row["color"], `${at}.color`),
    defaultEdgeKind: puzzlePuzzle2dArtifactGuardString(row["defaultEdgeKind"], `${at}.defaultEdgeKind`),
  };
}

export function parsePuzzle2dKindCatalogs(value: unknown, at = "$"): Puzzle2dKindCatalogs {
  const row = puzzlePuzzle2dArtifactGuardObject(value, at);
  return {
    nodes: puzzlePuzzle2dArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parsePuzzle2dCatalogNodeKind(item, `${at}.nodes[${index}]`)),
    handles: puzzlePuzzle2dArtifactGuardArray(row["handles"], `${at}.handles`).map((item, index) => parsePuzzle2dCatalogHandleKind(item, `${at}.handles[${index}]`)),
    edges: puzzlePuzzle2dArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => parsePuzzle2dCatalogEdgeKind(item, `${at}.edges[${index}]`)),
    wires: puzzlePuzzle2dArtifactGuardArray(row["wires"], `${at}.wires`).map((item, index) => parsePuzzle2dCatalogWireKind(item, `${at}.wires[${index}]`)),
  };
}
