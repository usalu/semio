/** 🧬️ Puzzle5d artifact schema — every field with its state class. */

export interface Puzzle5dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  domain: string;
  /** @state artifact */
  label?: string;
  /** @state artifact */
  meta: Puzzle5dMeta;
  /** @state artifact */
  kindCatalogs?: Puzzle5dKindCatalogs;
  /** @state artifact */
  kindCompatibility: Puzzle5dKindCompatibility[];
  /** @state artifact */
  parts: Puzzle5dPart[];
  /** @state artifact */
  fasteners: Puzzle5dFastener[];
  /** @state presence */
  selectedPartIds: string[];
  /** @state presence */
  selectedGripIds: string[];
  /** @state presence */
  selectedFastenerIds: string[];
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  camera2dX: number;
  /** @state config */
  camera2dY: number;
  /** @state config */
  camera2dZoom: number;
  /** @state config */
  camera3dPositionX: number;
  /** @state config */
  camera3dPositionY: number;
  /** @state config */
  camera3dPositionZ: number;
  /** @state config */
  camera3dTargetX: number;
  /** @state config */
  camera3dTargetY: number;
  /** @state config */
  camera3dTargetZ: number;
  /** @state config */
  camera3dZoom: number;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  suggestionOffset: number;
  /** @state config */
  overlapBudget: number;
  /** @state config */
  fillCount: number;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  lodMode: string;
  /** @state config */
  /** @state config */
  runtimeExtrasJson: string;
  /** @state artifact */
  hoveredPartId?: string;
  /** @state artifact */
  previewSeq: number;
}



/** ⚓️ Part root plane policy. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🔗️ Compat row specificity. */
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

/** 🏷️ Part-kind attribute. */
export interface Puzzle5dAttribute {
  id?: string;
  key?: string;
  value?: string;
  definition?: string;
}

/** ✍️ Part-kind author. */
export interface Puzzle5dAuthor {
  id?: string;
  name?: string;
  email?: string;
  role?: string;
  rank?: number;
}

/** 🖼️ Part-kind representation. */
export interface Puzzle5dRepresentation {
  id?: string;
  name?: string;
  url?: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

/** 🌱️ Grip template on a part-kind. */
export interface Puzzle5dGripTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  gripKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

/** 🧱️ Part-kind catalog row. */
export interface Puzzle5dCatalogPartKind {
  id: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle5dRepresentation[];
  grips?: Puzzle5dGripTemplate[];
  attributes?: Puzzle5dAttribute[];
  authors?: Puzzle5dAuthor[];
}

/** 🔘️ Grip-kind catalog row. */
export interface Puzzle5dCatalogGripKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultRopeKind?: string;
}

/** 🔗️ Fastener-kind catalog row. */
export interface Puzzle5dCatalogFastenerKind {
  id: string;
  name?: string;
  label?: string;
}

/** 🧵️ Rope-kind catalog row. */
export interface Puzzle5dCatalogRopeKind {
  id: string;
  name?: string;
  label?: string;
  defaultFastenerKind?: string;
}

/** 🗂️ Kind catalogs bundle. */
export interface Puzzle5dKindCatalogs {
  parts?: Puzzle5dCatalogPartKind[];
  grips?: Puzzle5dCatalogGripKind[];
  fasteners?: Puzzle5dCatalogFastenerKind[];
  ropes?: Puzzle5dCatalogRopeKind[];
}

/** 🔗️ Kind compatibility row. */
export interface Puzzle5dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle5dCompatSpecificity;
}

/** 📝️ Meta. */
export interface Puzzle5dMeta {
  description?: string;
}

/** 🧱️ Part. */
export interface Puzzle5dPart {
  id: string;
  partKind?: string;
  anchor?: Puzzle5dPartAnchor;
  "2d"?: Record<string, unknown>;
  "3d"?: Record<string, unknown>;
  grips?: Record<string, unknown>[];
}

/** 🔗️ Fastener with eight transform params. */
export interface Puzzle5dFastener {
  id: string;
  source: string;
  target: string;
  fastenerKind?: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dArtifactGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dArtifactGuardRefusal(at, why);
};

type puzzlePuzzle5dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dArtifactGuardReject(at, "value is not an object");
export const puzzlePuzzle5dArtifactGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dArtifactGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dArtifactGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dArtifactGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dArtifactGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dArtifactGuardNumber(value, at, bounds) : puzzlePuzzle5dArtifactGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dArtifact(value: unknown, at = "$"): Puzzle5dArtifact {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    schema: puzzlePuzzle5dArtifactGuardString(row["schema"], `${at}.schema`),
    domain: puzzlePuzzle5dArtifactGuardString(row["domain"], `${at}.domain`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["label"], `${at}.label`),
    meta: parsePuzzle5dMeta(row["meta"], `${at}.meta`),
    kindCatalogs: row["kindCatalogs"] === undefined ? undefined : parsePuzzle5dKindCatalogs(row["kindCatalogs"], `${at}.kindCatalogs`),
    kindCompatibility: puzzlePuzzle5dArtifactGuardArray(row["kindCompatibility"], `${at}.kindCompatibility`).map((item, index) => parsePuzzle5dKindCompatibility(item, `${at}.kindCompatibility[${index}]`)),
    parts: puzzlePuzzle5dArtifactGuardArray(row["parts"], `${at}.parts`).map((item, index) => parsePuzzle5dPart(item, `${at}.parts[${index}]`)),
    fasteners: puzzlePuzzle5dArtifactGuardArray(row["fasteners"], `${at}.fasteners`).map((item, index) => parsePuzzle5dFastener(item, `${at}.fasteners[${index}]`)),
    selectedPartIds: puzzlePuzzle5dArtifactGuardArray(row["selectedPartIds"], `${at}.selectedPartIds`).map((item, index) => puzzlePuzzle5dArtifactGuardString(item, `${at}.selectedPartIds[${index}]`)),
    selectedGripIds: puzzlePuzzle5dArtifactGuardArray(row["selectedGripIds"], `${at}.selectedGripIds`).map((item, index) => puzzlePuzzle5dArtifactGuardString(item, `${at}.selectedGripIds[${index}]`)),
    selectedFastenerIds: puzzlePuzzle5dArtifactGuardArray(row["selectedFastenerIds"], `${at}.selectedFastenerIds`).map((item, index) => puzzlePuzzle5dArtifactGuardString(item, `${at}.selectedFastenerIds[${index}]`)),
    activeUtilityId: puzzlePuzzle5dArtifactGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    camera2dX: puzzlePuzzle5dArtifactGuardNumber(row["camera2dX"], `${at}.camera2dX`),
    camera2dY: puzzlePuzzle5dArtifactGuardNumber(row["camera2dY"], `${at}.camera2dY`),
    camera2dZoom: puzzlePuzzle5dArtifactGuardNumber(row["camera2dZoom"], `${at}.camera2dZoom`),
    camera3dPositionX: puzzlePuzzle5dArtifactGuardNumber(row["camera3dPositionX"], `${at}.camera3dPositionX`),
    camera3dPositionY: puzzlePuzzle5dArtifactGuardNumber(row["camera3dPositionY"], `${at}.camera3dPositionY`),
    camera3dPositionZ: puzzlePuzzle5dArtifactGuardNumber(row["camera3dPositionZ"], `${at}.camera3dPositionZ`),
    camera3dTargetX: puzzlePuzzle5dArtifactGuardNumber(row["camera3dTargetX"], `${at}.camera3dTargetX`),
    camera3dTargetY: puzzlePuzzle5dArtifactGuardNumber(row["camera3dTargetY"], `${at}.camera3dTargetY`),
    camera3dTargetZ: puzzlePuzzle5dArtifactGuardNumber(row["camera3dTargetZ"], `${at}.camera3dTargetZ`),
    camera3dZoom: puzzlePuzzle5dArtifactGuardNumber(row["camera3dZoom"], `${at}.camera3dZoom`),
    selectionMethod: puzzlePuzzle5dArtifactGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    gridSnapEnabled: puzzlePuzzle5dArtifactGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridFactor: puzzlePuzzle5dArtifactGuardNumber(row["gridFactor"], `${at}.gridFactor`),
    suggestionOffset: puzzlePuzzle5dArtifactGuardNumber(row["suggestionOffset"], `${at}.suggestionOffset`),
    overlapBudget: puzzlePuzzle5dArtifactGuardNumber(row["overlapBudget"], `${at}.overlapBudget`),
    fillCount: puzzlePuzzle5dArtifactGuardInteger(row["fillCount"], `${at}.fillCount`, {"minimum": 0}),
    brushCandidateIndex: puzzlePuzzle5dArtifactGuardInteger(row["brushCandidateIndex"], `${at}.brushCandidateIndex`, {"minimum": 0}),
    lodMode: puzzlePuzzle5dArtifactGuardString(row["lodMode"], `${at}.lodMode`),
    runtimeExtrasJson: puzzlePuzzle5dArtifactGuardString(row["runtimeExtrasJson"], `${at}.runtimeExtrasJson`),
    hoveredPartId: row["hoveredPartId"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["hoveredPartId"], `${at}.hoveredPartId`),
    previewSeq: puzzlePuzzle5dArtifactGuardInteger(row["previewSeq"], `${at}.previewSeq`),
  };
}

export function parsePuzzle5dMeta(value: unknown, at = "$"): Puzzle5dMeta {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["description"], `${at}.description`),
  };
}

export function parsePuzzle5dKindCatalogs(value: unknown, at = "$"): Puzzle5dKindCatalogs {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    parts: row["parts"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["parts"], `${at}.parts`).map((item, index) => parsePuzzle5dCatalogPartKind(item, `${at}.parts[${index}]`)),
    grips: row["grips"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["grips"], `${at}.grips`).map((item, index) => parsePuzzle5dCatalogGripKind(item, `${at}.grips[${index}]`)),
    fasteners: row["fasteners"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["fasteners"], `${at}.fasteners`).map((item, index) => parsePuzzle5dCatalogFastenerKind(item, `${at}.fasteners[${index}]`)),
    ropes: row["ropes"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["ropes"], `${at}.ropes`).map((item, index) => parsePuzzle5dCatalogRopeKind(item, `${at}.ropes[${index}]`)),
  };
}

export function parsePuzzle5dKindCompatibility(value: unknown, at = "$"): Puzzle5dKindCompatibility {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    source: puzzlePuzzle5dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle5dArtifactGuardString(row["target"], `${at}.target`),
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : parsePuzzle5dCompatSpecificity(row["specificity"], `${at}.specificity`),
  };
}

export function parsePuzzle5dPart(value: unknown, at = "$"): Puzzle5dPart {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    partKind: row["partKind"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["partKind"], `${at}.partKind`),
    anchor: row["anchor"] === undefined ? undefined : parsePuzzle5dPartAnchor(row["anchor"], `${at}.anchor`),
    2d: row["2d"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardObject(row["2d"], `${at}.2d`),
    3d: row["3d"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardObject(row["3d"], `${at}.3d`),
    grips: row["grips"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["grips"], `${at}.grips`).map((item, index) => puzzlePuzzle5dArtifactGuardObject(item, `${at}.grips[${index}]`)),
  };
}

export function parsePuzzle5dFastener(value: unknown, at = "$"): Puzzle5dFastener {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    source: puzzlePuzzle5dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle5dArtifactGuardString(row["target"], `${at}.target`),
    fastenerKind: row["fastenerKind"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["fastenerKind"], `${at}.fastenerKind`),
    gap: row["gap"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["gap"], `${at}.gap`),
    shift: row["shift"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["shift"], `${at}.shift`),
    rise: row["rise"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["rise"], `${at}.rise`),
    rotation: row["rotation"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    turn: row["turn"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["turn"], `${at}.turn`),
    tilt: row["tilt"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["tilt"], `${at}.tilt`),
    x: row["x"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePuzzle5dPartAnchor(value: unknown, at = "$"): Puzzle5dPartAnchor {
  return puzzlePuzzle5dArtifactGuardMember(value, `${at}`, ["fixed", "derived"] as const);
}

export function parsePuzzle5dCompatSpecificity(value: unknown, at = "$"): Puzzle5dCompatSpecificity {
  return puzzlePuzzle5dArtifactGuardMember(value, `${at}`, ["general", "part", "fastener", "grip", "rope"] as const);
}

export function parsePuzzle5dAttribute(value: unknown, at = "$"): Puzzle5dAttribute {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    key: row["key"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["key"], `${at}.key`),
    value: row["value"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["value"], `${at}.value`),
    definition: row["definition"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["definition"], `${at}.definition`),
  };
}

export function parsePuzzle5dAuthor(value: unknown, at = "$"): Puzzle5dAuthor {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["name"], `${at}.name`),
    email: row["email"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["email"], `${at}.email`),
    role: row["role"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["role"], `${at}.role`),
    rank: row["rank"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardInteger(row["rank"], `${at}.rank`),
  };
}

export function parsePuzzle5dRepresentation(value: unknown, at = "$"): Puzzle5dRepresentation {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["name"], `${at}.name`),
    url: row["url"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["url"], `${at}.url`),
    mime: row["mime"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["mime"], `${at}.mime`),
    tags: row["tags"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["tags"], `${at}.tags`).map((item, index) => puzzlePuzzle5dArtifactGuardString(item, `${at}.tags[${index}]`)),
    lod: row["lod"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["lod"], `${at}.lod`),
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["description"], `${at}.description`),
  };
}

export function parsePuzzle5dGripTemplate(value: unknown, at = "$"): Puzzle5dGripTemplate {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["label"], `${at}.label`),
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["icon"], `${at}.icon`),
    gripKind: row["gripKind"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["gripKind"], `${at}.gripKind`),
    point: row["point"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["point"], `${at}.point`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dArtifactGuardNumber(item, `${at}.point[${index}]`)),
    direction: row["direction"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["direction"], `${at}.direction`, {"minItems": 3, "maxItems": 3}).map((item, index) => puzzlePuzzle5dArtifactGuardNumber(item, `${at}.direction[${index}]`)),
    t: row["t"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["t"], `${at}.t`),
    mandatory: row["mandatory"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardBoolean(row["mandatory"], `${at}.mandatory`),
    radius: row["radius"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardNumber(row["radius"], `${at}.radius`),
  };
}

export function parsePuzzle5dCatalogPartKind(value: unknown, at = "$"): Puzzle5dCatalogPartKind {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["label"], `${at}.label`),
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["icon"], `${at}.icon`),
    image: row["image"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["image"], `${at}.image`),
    unit: row["unit"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["unit"], `${at}.unit`),
    abstract: row["abstract"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardBoolean(row["abstract"], `${at}.abstract`),
    baseKinds: row["baseKinds"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["baseKinds"], `${at}.baseKinds`).map((item, index) => puzzlePuzzle5dArtifactGuardString(item, `${at}.baseKinds[${index}]`)),
    representations: row["representations"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["representations"], `${at}.representations`).map((item, index) => parsePuzzle5dRepresentation(item, `${at}.representations[${index}]`)),
    grips: row["grips"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["grips"], `${at}.grips`).map((item, index) => parsePuzzle5dGripTemplate(item, `${at}.grips[${index}]`)),
    attributes: row["attributes"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => parsePuzzle5dAttribute(item, `${at}.attributes[${index}]`)),
    authors: row["authors"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => parsePuzzle5dAuthor(item, `${at}.authors[${index}]`)),
  };
}

export function parsePuzzle5dCatalogGripKind(value: unknown, at = "$"): Puzzle5dCatalogGripKind {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    code: row["code"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["code"], `${at}.code`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["label"], `${at}.label`),
    order: row["order"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardInteger(row["order"], `${at}.order`),
    compatibleWith: row["compatibleWith"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardArray(row["compatibleWith"], `${at}.compatibleWith`).map((item, index) => puzzlePuzzle5dArtifactGuardString(item, `${at}.compatibleWith[${index}]`)),
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["icon"], `${at}.icon`),
    color: row["color"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["color"], `${at}.color`),
    defaultRopeKind: row["defaultRopeKind"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["defaultRopeKind"], `${at}.defaultRopeKind`),
  };
}

export function parsePuzzle5dCatalogFastenerKind(value: unknown, at = "$"): Puzzle5dCatalogFastenerKind {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["label"], `${at}.label`),
  };
}

export function parsePuzzle5dCatalogRopeKind(value: unknown, at = "$"): Puzzle5dCatalogRopeKind {
  const row = puzzlePuzzle5dArtifactGuardObject(value, at);
  return {
    id: puzzlePuzzle5dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["label"], `${at}.label`),
    defaultFastenerKind: row["defaultFastenerKind"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardString(row["defaultFastenerKind"], `${at}.defaultFastenerKind`),
  };
}
