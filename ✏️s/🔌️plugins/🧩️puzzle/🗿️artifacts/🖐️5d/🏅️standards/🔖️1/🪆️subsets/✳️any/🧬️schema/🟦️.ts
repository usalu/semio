/** 🧬️ Puzzle5d artifact and persisted snapshot share one exact canonical model. */
export * from "./📸️snapshot/🟦️.ts";
import { parsePuzzle5dSnapshot, type Puzzle5dSnapshot } from "./📸️snapshot/🟦️.ts";
export interface Puzzle5dArtifact extends Puzzle5dSnapshot {}
/** 🚪️ Admit the same persisted fields through the artifact public facade. */
export function parsePuzzle5dArtifact(value: unknown, at = "$"): Puzzle5dArtifact { return parsePuzzle5dSnapshot(value, at); }

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
export const puzzlePuzzle5dArtifactGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) puzzlePuzzle5dArtifactGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
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

export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

export function parsePuzzle5dCompatSpecificity(value: unknown, at = "$"): Puzzle5dCompatSpecificity {
  return puzzlePuzzle5dArtifactGuardMember(value, `${at}`, ["general", "part", "fastener", "grip", "rope"] as const);
}

export interface Puzzle5dKindCompatibility {
  readonly source: string;
  readonly target: string;
  readonly bidirectional?: boolean;
  readonly important?: boolean;
  readonly specificity?: Puzzle5dCompatSpecificity;
}

export function parsePuzzle5dKindCompatibility(value: unknown, at = "$"): Puzzle5dKindCompatibility {
  const row = puzzlePuzzle5dArtifactGuardClosed(puzzlePuzzle5dArtifactGuardObject(value, at), at, ["source", "target", "bidirectional", "important", "specificity"]);
  return {
    source: puzzlePuzzle5dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle5dArtifactGuardString(row["target"], `${at}.target`),
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle5dArtifactGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : parsePuzzle5dCompatSpecificity(row["specificity"], `${at}.specificity`),
  };
}

export type Puzzle5dPartAnchor = "fixed" | "derived";

export function parsePuzzle5dPartAnchor(value: unknown, at = "$"): Puzzle5dPartAnchor {
  return puzzlePuzzle5dArtifactGuardMember(value, `${at}`, ["fixed", "derived"] as const);
}
