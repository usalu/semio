/** 🧬️ S Space index artifact schema facet — re-exports the subset's schema types. */
export type { SSpaceSnapshot, SpaceArtifactRow, SpaceArtifactDialect } from "./📸️snapshot/🟦️.ts";
export type { SSpaceDiff } from "./🔺️diff/🟦️.ts";
export type { SSpaceMutation } from "./🧬️mutations/🟦️.ts";

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceSpaceArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceSpaceArtifactGuardReject = (at: string, why: string): never => {
  throw new spaceSpaceArtifactGuardRefusal(at, why);
};

type spaceSpaceArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceSpaceArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceSpaceArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceSpaceArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceSpaceArtifactGuardReject(at, "value is not an object");
export const spaceSpaceArtifactGuardArray = (value: unknown, at: string, bounds: spaceSpaceArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceSpaceArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceSpaceArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceSpaceArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceSpaceArtifactGuardString = (value: unknown, at: string, bounds: spaceSpaceArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceSpaceArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceSpaceArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceSpaceArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceSpaceArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceSpaceArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceSpaceArtifactGuardReject(at, "value is not a boolean"));
export const spaceSpaceArtifactGuardNumber = (value: unknown, at: string, bounds: spaceSpaceArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceSpaceArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceSpaceArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceSpaceArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceSpaceArtifactGuardInteger = (value: unknown, at: string, bounds: spaceSpaceArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceSpaceArtifactGuardNumber(value, at, bounds) : spaceSpaceArtifactGuardReject(at, "value is not an integer");
export const spaceSpaceArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceSpaceArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceSpaceArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceSpaceArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SSpaceArtifact {
  readonly schema: string;
  readonly spaceId: string;
  readonly artifacts: readonly SpaceArtifactRow[];
}

export function parseSSpaceArtifact(value: unknown, at = "$"): SSpaceArtifact {
  const row = spaceSpaceArtifactGuardObject(value, at);
  return {
    schema: spaceSpaceArtifactGuardString(row["schema"], `${at}.schema`),
    spaceId: spaceSpaceArtifactGuardString(row["spaceId"], `${at}.spaceId`),
    artifacts: spaceSpaceArtifactGuardArray(row["artifacts"], `${at}.artifacts`).map((item, index) => parseSpaceArtifactRow(item, `${at}.artifacts[${index}]`)),
  };
}

export interface SpaceArtifactDialect {
  readonly artifactKind: string;
  readonly standard: string;
  readonly subset: string;
}

export function parseSpaceArtifactDialect(value: unknown, at = "$"): SpaceArtifactDialect {
  const row = spaceSpaceArtifactGuardObject(value, at);
  return {
    artifactKind: spaceSpaceArtifactGuardString(row["artifactKind"], `${at}.artifactKind`),
    standard: spaceSpaceArtifactGuardString(row["standard"], `${at}.standard`),
    subset: spaceSpaceArtifactGuardString(row["subset"], `${at}.subset`),
  };
}

export interface SpaceArtifactRow {
  readonly id: string;
  readonly name: string;
  readonly kindId: string;
  readonly schema: string;
  readonly dialect: SpaceArtifactDialect;
  readonly createdAtMs: number;
  readonly createdBy: string;
  readonly updatedAtMs: number;
  readonly updatedBy: string;
}

export function parseSpaceArtifactRow(value: unknown, at = "$"): SpaceArtifactRow {
  const row = spaceSpaceArtifactGuardObject(value, at);
  return {
    id: spaceSpaceArtifactGuardString(row["id"], `${at}.id`),
    name: spaceSpaceArtifactGuardString(row["name"], `${at}.name`),
    kindId: spaceSpaceArtifactGuardString(row["kindId"], `${at}.kindId`),
    schema: spaceSpaceArtifactGuardString(row["schema"], `${at}.schema`),
    dialect: parseSpaceArtifactDialect(row["dialect"], `${at}.dialect`),
    createdAtMs: spaceSpaceArtifactGuardInteger(row["createdAtMs"], `${at}.createdAtMs`, {"minimum": 0}),
    createdBy: spaceSpaceArtifactGuardString(row["createdBy"], `${at}.createdBy`),
    updatedAtMs: spaceSpaceArtifactGuardInteger(row["updatedAtMs"], `${at}.updatedAtMs`, {"minimum": 0}),
    updatedBy: spaceSpaceArtifactGuardString(row["updatedBy"], `${at}.updatedBy`),
  };
}
