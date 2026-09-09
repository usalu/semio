/** 🧬️ Block3d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../🟦️";
import type { Block3dVortexKindExtra, Block3dVortexTemplate, Block3dWindowView } from "../../../../../🟦️";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface Block3dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objectKind: BlockKindIdentity;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog: ArtifactChild;
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
    catalog: parseArtifactChild(row["catalog"]),
    vortexKindExtra: blockBlock3dArtifactGuardArray(row["vortexKindExtra"], `${at}.vortexKindExtra`).map((item, index) => parseBlock3dVortexKindExtra(item, `${at}.vortexKindExtra[${index}]`)),
    vortices: blockBlock3dArtifactGuardArray(row["vortices"], `${at}.vortices`).map((item, index) => parseBlock3dVortexTemplate(item, `${at}.vortices[${index}]`)),
    compatibility: blockBlock3dArtifactGuardArray(row["compatibility"], `${at}.compatibility`).map((item, index) => parseBlockCompatibilityRule(item, `${at}.compatibility[${index}]`)),
    attributes: blockBlock3dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => parseBlockAttribute(item, `${at}.attributes[${index}]`)),
    authors: blockBlock3dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => parseBlockAuthor(item, `${at}.authors[${index}]`)),
    camera3d: parseBlockCamera3d(row["camera3d"], `${at}.camera3d`),
    meta: parseBlockMeta(row["meta"], `${at}.meta`),
  };
}

export function parseBlock3dVortexKindExtra(value: unknown, at = "$"): Block3dVortexKindExtra {
  return blockBlock3dArtifactGuardObject(value, at) as unknown as Block3dVortexKindExtra;
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
