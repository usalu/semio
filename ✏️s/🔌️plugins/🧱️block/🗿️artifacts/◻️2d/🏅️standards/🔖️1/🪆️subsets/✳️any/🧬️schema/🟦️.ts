/** 🧬️ Block2d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta } from "../../../../../../../🟦️";
import type { Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation } from "../../../../../🟦️";

export interface Block2dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  nodeKind: BlockKindIdentity;
  /** @state artifact */
  presentation: Block2dPresentation;
  /** @state artifact */
  handleKinds: Block2dHandleKind[];
  /** @state artifact */
  handles: Block2dHandleTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera2d: BlockCamera2d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dArtifactGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dArtifactGuardRefusal(at, why);
};

type blockBlock2dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dArtifactGuardReject(at, "value is not an object");
export const blockBlock2dArtifactGuardArray = (value: unknown, at: string, bounds: blockBlock2dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dArtifactGuardString = (value: unknown, at: string, bounds: blockBlock2dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dArtifactGuardReject(at, "value is not a boolean"));
export const blockBlock2dArtifactGuardNumber = (value: unknown, at: string, bounds: blockBlock2dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dArtifactGuardInteger = (value: unknown, at: string, bounds: blockBlock2dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dArtifactGuardNumber(value, at, bounds) : blockBlock2dArtifactGuardReject(at, "value is not an integer");
export const blockBlock2dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dArtifact(value: unknown, at = "$"): Block2dArtifact {
  const row = blockBlock2dArtifactGuardObject(value, at);
  return {
    schema: blockBlock2dArtifactGuardString(row["schema"], `${at}.schema`),
    nodeKind: parseBlockKindIdentity(row["nodeKind"], `${at}.nodeKind`),
    presentation: parseBlock2dPresentation(row["presentation"], `${at}.presentation`),
    handleKinds: blockBlock2dArtifactGuardArray(row["handleKinds"], `${at}.handleKinds`).map((item, index) => parseBlock2dHandleKind(item, `${at}.handleKinds[${index}]`)),
    handles: blockBlock2dArtifactGuardArray(row["handles"], `${at}.handles`).map((item, index) => parseBlock2dHandleTemplate(item, `${at}.handles[${index}]`)),
    compatibility: blockBlock2dArtifactGuardArray(row["compatibility"], `${at}.compatibility`).map((item, index) => parseBlockCompatibilityRule(item, `${at}.compatibility[${index}]`)),
    attributes: blockBlock2dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => parseBlockAttribute(item, `${at}.attributes[${index}]`)),
    authors: blockBlock2dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => parseBlockAuthor(item, `${at}.authors[${index}]`)),
    camera2d: parseBlockCamera2d(row["camera2d"], `${at}.camera2d`),
    meta: parseBlockMeta(row["meta"], `${at}.meta`),
    selectedIds: blockBlock2dArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => blockBlock2dArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
  };
}

export type BlockKindIdentity = Readonly<Record<string, unknown>>;

export function parseBlockKindIdentity(value: unknown, at = "$"): BlockKindIdentity {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type Block2dPresentation = Readonly<Record<string, unknown>>;

export function parseBlock2dPresentation(value: unknown, at = "$"): Block2dPresentation {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type Block2dHandleKind = Readonly<Record<string, unknown>>;

export function parseBlock2dHandleKind(value: unknown, at = "$"): Block2dHandleKind {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type Block2dHandleTemplate = Readonly<Record<string, unknown>>;

export function parseBlock2dHandleTemplate(value: unknown, at = "$"): Block2dHandleTemplate {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type BlockCompatibilityRule = Readonly<Record<string, unknown>>;

export function parseBlockCompatibilityRule(value: unknown, at = "$"): BlockCompatibilityRule {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type BlockAttribute = Readonly<Record<string, unknown>>;

export function parseBlockAttribute(value: unknown, at = "$"): BlockAttribute {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type BlockAuthor = Readonly<Record<string, unknown>>;

export function parseBlockAuthor(value: unknown, at = "$"): BlockAuthor {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type BlockCamera2d = Readonly<Record<string, unknown>>;

export function parseBlockCamera2d(value: unknown, at = "$"): BlockCamera2d {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}

export type BlockMeta = Readonly<Record<string, unknown>>;

export function parseBlockMeta(value: unknown, at = "$"): BlockMeta {
  return blockBlock2dArtifactGuardObject(value, `${at}`);
}
