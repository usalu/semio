/** 🧬️ Block5d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../🟦️";
import type { Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d } from "../../../../../🟦️";

export interface Block5dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  partKind: BlockKindIdentity;
  /** @state artifact */
  part2d: Block5dPart2d;
  /** @state artifact */
  part3d: Block5dPart3d;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact */
  gripKinds: Block5dGripKind[];
  /** @state artifact */
  grips: Block5dGripTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera2d: BlockCamera2d;
  /** @state artifact */
  camera3d: BlockCamera3d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dArtifactGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dArtifactGuardRefusal(at, why);
};

type blockBlock5dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dArtifactGuardReject(at, "value is not an object");
export const blockBlock5dArtifactGuardArray = (value: unknown, at: string, bounds: blockBlock5dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dArtifactGuardString = (value: unknown, at: string, bounds: blockBlock5dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dArtifactGuardReject(at, "value is not a boolean"));
export const blockBlock5dArtifactGuardNumber = (value: unknown, at: string, bounds: blockBlock5dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dArtifactGuardInteger = (value: unknown, at: string, bounds: blockBlock5dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dArtifactGuardNumber(value, at, bounds) : blockBlock5dArtifactGuardReject(at, "value is not an integer");
export const blockBlock5dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dArtifact(value: unknown, at = "$"): Block5dArtifact {
  const row = blockBlock5dArtifactGuardObject(value, at);
  return {
    schema: blockBlock5dArtifactGuardString(row["schema"], `${at}.schema`),
    partKind: parseBlockKindIdentity(row["partKind"], `${at}.partKind`),
    part2d: parseBlock5dPart2d(row["part2d"], `${at}.part2d`),
    part3d: parseBlock5dPart3d(row["part3d"], `${at}.part3d`),
    representations: blockBlock5dArtifactGuardArray(row["representations"], `${at}.representations`).map((item, index) => parseBlockRepresentation(item, `${at}.representations[${index}]`)),
    gripKinds: blockBlock5dArtifactGuardArray(row["gripKinds"], `${at}.gripKinds`).map((item, index) => parseBlock5dGripKind(item, `${at}.gripKinds[${index}]`)),
    grips: blockBlock5dArtifactGuardArray(row["grips"], `${at}.grips`).map((item, index) => parseBlock5dGripTemplate(item, `${at}.grips[${index}]`)),
    compatibility: blockBlock5dArtifactGuardArray(row["compatibility"], `${at}.compatibility`).map((item, index) => parseBlockCompatibilityRule(item, `${at}.compatibility[${index}]`)),
    attributes: blockBlock5dArtifactGuardArray(row["attributes"], `${at}.attributes`).map((item, index) => parseBlockAttribute(item, `${at}.attributes[${index}]`)),
    authors: blockBlock5dArtifactGuardArray(row["authors"], `${at}.authors`).map((item, index) => parseBlockAuthor(item, `${at}.authors[${index}]`)),
    camera2d: parseBlockCamera2d(row["camera2d"], `${at}.camera2d`),
    camera3d: parseBlockCamera3d(row["camera3d"], `${at}.camera3d`),
    meta: parseBlockMeta(row["meta"], `${at}.meta`),
    selectedIds: blockBlock5dArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => blockBlock5dArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
  };
}

export type BlockKindIdentity = Readonly<Record<string, unknown>>;

export function parseBlockKindIdentity(value: unknown, at = "$"): BlockKindIdentity {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type Block5dPart2d = Readonly<Record<string, unknown>>;

export function parseBlock5dPart2d(value: unknown, at = "$"): Block5dPart2d {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type Block5dPart3d = Readonly<Record<string, unknown>>;

export function parseBlock5dPart3d(value: unknown, at = "$"): Block5dPart3d {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockRepresentation = Readonly<Record<string, unknown>>;

export function parseBlockRepresentation(value: unknown, at = "$"): BlockRepresentation {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type Block5dGripKind = Readonly<Record<string, unknown>>;

export function parseBlock5dGripKind(value: unknown, at = "$"): Block5dGripKind {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type Block5dGripTemplate = Readonly<Record<string, unknown>>;

export function parseBlock5dGripTemplate(value: unknown, at = "$"): Block5dGripTemplate {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockCompatibilityRule = Readonly<Record<string, unknown>>;

export function parseBlockCompatibilityRule(value: unknown, at = "$"): BlockCompatibilityRule {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockAttribute = Readonly<Record<string, unknown>>;

export function parseBlockAttribute(value: unknown, at = "$"): BlockAttribute {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockAuthor = Readonly<Record<string, unknown>>;

export function parseBlockAuthor(value: unknown, at = "$"): BlockAuthor {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockCamera2d = Readonly<Record<string, unknown>>;

export function parseBlockCamera2d(value: unknown, at = "$"): BlockCamera2d {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockCamera3d = Readonly<Record<string, unknown>>;

export function parseBlockCamera3d(value: unknown, at = "$"): BlockCamera3d {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}

export type BlockMeta = Readonly<Record<string, unknown>>;

export function parseBlockMeta(value: unknown, at = "$"): BlockMeta {
  return blockBlock5dArtifactGuardObject(value, `${at}`);
}
