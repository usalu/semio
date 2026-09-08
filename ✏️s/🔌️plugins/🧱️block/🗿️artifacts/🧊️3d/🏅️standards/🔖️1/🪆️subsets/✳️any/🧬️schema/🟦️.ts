/** 🧬️ Block3d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../🟦️";
import type { Block3dBrushPreview, Block3dVortexKindExtra, Block3dVortexTemplate, Block3dWindowView } from "../../../../../🟦️";

/** 🗂️ Dialect coordinate a child artifact is claimed against. */
export interface ArtifactDialect { artifactKind: string; standard: string; subset: string; }

/** 🎯️ What a child handle points at — verified against the real fixture
 * `…/🙅remove-author/🧪️tests/✏️uncredits-ada/📸️snapshot/⬅️before/🔣️.json`'s `catalog.target`
 * (NOT a plain string, unlike some sibling plugins' unverified `ArtifactChildHandle` stubs). */
export interface ArtifactChildTarget { artifactId: string; dialect: ArtifactDialect; }

/** 🧒️ `store::ArtifactChild<T>` wire handle — child artifact id plus its kind claim. */
export interface ArtifactChildHandle { childId: string; target: ArtifactChildTarget; }

export interface Block3dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objectKind: BlockKindIdentity;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog: ArtifactChildHandle;
  /** @state artifact */
  vortexKindExtra: Block3dVortexKindExtra[];
  /** @state artifact */
  vortices: Block3dVortexTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera3d: BlockCamera3d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  activeRepresentationId?: string;
  /** @state presence */
  wantedTags: string[];
  /** @state config */
  /** @state config */
  windows: Block3dWindowView[];
  /** @state config */
  brushVortexKindId?: string;
  /** @state config */
  brushRadius: number;
  /** @state config */
  brushFlip: boolean;
  /** @state artifact */
  brushPreview?: Block3dBrushPreview;
  /** @state config */
  camera?: BlockCamera3d;
  /** @state artifact */
  hoveredVortexFullId?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dArtifactGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dArtifactGuardRefusal(at, why);
};

type blockBlock3dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dArtifactGuardReject(at, "value is not an object");
export const blockBlock3dArtifactGuardArray = (value: unknown, at: string, bounds: blockBlock3dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dArtifactGuardString = (value: unknown, at: string, bounds: blockBlock3dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dArtifactGuardReject(at, "value is not a boolean"));
export const blockBlock3dArtifactGuardNumber = (value: unknown, at: string, bounds: blockBlock3dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dArtifactGuardInteger = (value: unknown, at: string, bounds: blockBlock3dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dArtifactGuardNumber(value, at, bounds) : blockBlock3dArtifactGuardReject(at, "value is not an integer");
export const blockBlock3dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dArtifact(value: unknown, at = "$"): Block3dArtifact {
  const row = blockBlock3dArtifactGuardObject(value, at);
  return {
    schema: blockBlock3dArtifactGuardString(row["schema"], `${at}.schema`),
    objectKind: parseBlockKindIdentity(row["objectKind"], `${at}.objectKind`),
    representations: blockBlock3dArtifactGuardArray(row["representations"], `${at}.representations`).map((item, index) => parseBlockRepresentation(item, `${at}.representations[${index}]`)),
    vortexKinds: blockBlock3dArtifactGuardArray(row["vortexKinds"], `${at}.vortexKinds`).map((item, index) => parseBlock3dVortexKind(item, `${at}.vortexKinds[${index}]`)),
    vortices: blockBlock3dArtifactGuardArray(row["vortices"], `${at}.vortices`).map((item, index) => parseBlock3dVortexTemplate(item, `${at}.vortices[${index}]`)),
    compatibility: blockBlock3dArtifactGuardArray(row["compatibility"], `${at}.compatibility`).map((item, index) => parseBlockCompatibilityRule(item, `${at}.compatibility[${index}]`)),
    attributes: blockBlock3dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => parseBlockAttribute(item, `${at}.attributes[${index}]`)),
    authors: blockBlock3dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => parseBlockAuthor(item, `${at}.authors[${index}]`)),
    camera3d: parseBlockCamera3d(row["camera3d"], `${at}.camera3d`),
    meta: parseBlockMeta(row["meta"], `${at}.meta`),
    selectedIds: blockBlock3dArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => blockBlock3dArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
    activeRepresentationId: row["activeRepresentationId"] === undefined ? undefined : blockBlock3dArtifactGuardString(row["activeRepresentationId"], `${at}.activeRepresentationId`),
    wantedTags: blockBlock3dArtifactGuardArray(row["wantedTags"], `${at}.wantedTags`).map((item, index) => blockBlock3dArtifactGuardString(item, `${at}.wantedTags[${index}]`)),
    windows: blockBlock3dArtifactGuardArray(row["windows"], `${at}.windows`).map((item, index) => parseBlock3dWindowView(item, `${at}.windows[${index}]`)),
    brushVortexKindId: row["brushVortexKindId"] === undefined ? undefined : blockBlock3dArtifactGuardString(row["brushVortexKindId"], `${at}.brushVortexKindId`),
    brushRadius: blockBlock3dArtifactGuardNumber(row["brushRadius"], `${at}.brushRadius`),
    brushFlip: blockBlock3dArtifactGuardBoolean(row["brushFlip"], `${at}.brushFlip`),
    brushPreview: row["brushPreview"] === undefined ? undefined : parseBlock3dBrushPreview(row["brushPreview"], `${at}.brushPreview`),
    camera: row["camera"] === undefined ? undefined : parseBlockCamera3d(row["camera"], `${at}.camera`),
    hoveredVortexFullId: row["hoveredVortexFullId"] === undefined ? undefined : blockBlock3dArtifactGuardString(row["hoveredVortexFullId"], `${at}.hoveredVortexFullId`),
  };
}

export type BlockKindIdentity = Readonly<Record<string, unknown>>;

export function parseBlockKindIdentity(value: unknown, at = "$"): BlockKindIdentity {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type BlockRepresentation = Readonly<Record<string, unknown>>;

export function parseBlockRepresentation(value: unknown, at = "$"): BlockRepresentation {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type Block3dVortexKind = Readonly<Record<string, unknown>>;

export function parseBlock3dVortexKind(value: unknown, at = "$"): Block3dVortexKind {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type Block3dVortexTemplate = Readonly<Record<string, unknown>>;

export function parseBlock3dVortexTemplate(value: unknown, at = "$"): Block3dVortexTemplate {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type BlockCompatibilityRule = Readonly<Record<string, unknown>>;

export function parseBlockCompatibilityRule(value: unknown, at = "$"): BlockCompatibilityRule {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type BlockAttribute = Readonly<Record<string, unknown>>;

export function parseBlockAttribute(value: unknown, at = "$"): BlockAttribute {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type BlockAuthor = Readonly<Record<string, unknown>>;

export function parseBlockAuthor(value: unknown, at = "$"): BlockAuthor {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type BlockCamera3d = Readonly<Record<string, unknown>>;

export function parseBlockCamera3d(value: unknown, at = "$"): BlockCamera3d {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type BlockMeta = Readonly<Record<string, unknown>>;

export function parseBlockMeta(value: unknown, at = "$"): BlockMeta {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type Block3dWindowView = Readonly<Record<string, unknown>>;

export function parseBlock3dWindowView(value: unknown, at = "$"): Block3dWindowView {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}

export type Block3dBrushPreview = Readonly<Record<string, unknown>>;

export function parseBlock3dBrushPreview(value: unknown, at = "$"): Block3dBrushPreview {
  return blockBlock3dArtifactGuardObject(value, `${at}`);
}
