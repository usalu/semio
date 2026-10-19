import { createHash } from "node:crypto";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { pathBudgetCodes, pathBudgetStatement } from "../📏️path-budget/🟦️.ts";

/** 🪆️ Stable profile identity rendered only from captured standard/subset values. */
export interface SemanticProjectionProfileIdentity {
  readonly artifactId: string;
  readonly standardVersion: string;
  readonly subsetId: string;
}

/** 🧱️ One exact source node supplied to the read-only projection authority. */
export interface SemanticProjectionAuthorityNode {
  readonly path: string;
  readonly nodeKind: "directory" | "file" | "symlink";
  readonly content?: string;
}

/** 🧭️ Inputs shared by the two artifact projection authorities. */
export interface SemanticPathProjectionAuthorityOptions {
  readonly artifactRoot: string;
  readonly contractId: string;
  readonly sourceRoot: string;
  readonly nodes: readonly SemanticProjectionAuthorityNode[];
  readonly occupiedPaths?: readonly string[];
  /** 🚪️ Paths at or below this root are not occupants (the projection's own source), so one shared `occupiedPaths` array serves every root. */
  readonly occupiedExclusionRoot?: string;
  readonly layout?: "source" | "destination";
}

/** 🛤️ One deterministic existing-file projection pair. */
export interface SemanticPathProjectionMapping {
  readonly sourcePath: string;
  readonly destinationPath: string;
}

/** 📊️ Fail-closed projection result; mappings are exposed only when problems is empty. */
export interface SemanticPathProjectionAuthority {
  readonly contractId: string;
  readonly sourceRoot: string;
  readonly destinationRoot: string;
  readonly mappings: readonly SemanticPathProjectionMapping[];
  readonly destinationDirectoryCount: number;
  readonly destinationNodeCount: number;
  readonly mappingDigest: string;
  readonly maxPathUtf16Units: number;
  readonly problems: readonly string[];
}

/** 🎨️ Shared forward-only root rendering result for artifact profile projections. */
export interface ArtifactPathProjectionRoot {
  readonly captures: Readonly<Record<string, string>>;
  readonly destinationRoot: string;
  readonly problems: readonly string[];
}

export function projectionByteCompare(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

export function projectionDirectories(root: string, paths: readonly string[]): string[] {
  const directories = new Set<string>([root]);
  for (const path of paths) {
    const segments = path.slice(root.length + 1).split("/");
    segments.pop();
    let current = root;
    for (const segment of segments) {
      current = `${current}/${segment}`;
      directories.add(current);
    }
  }
  return [...directories].sort(projectionByteCompare);
}

export function projectionCanonicalKey(path: string, comparison: "nfc" | "case-fold" | "vs16-fold"): string {
  const normalized = path.normalize("NFC");
  if (comparison === "nfc") return normalized;
  if (comparison === "case-fold") return normalized.toLocaleLowerCase("und");
  return normalized.replaceAll("\uFE0F", "");
}

const projectionOccupancyIndexes = new WeakMap<readonly string[], ReadonlyMap<string, readonly string[]>>();

/** 🗝️ Every occupied path under each of its raw and canonical comparison keys, built once per occupied-path array. */
function projectionOccupancyIndex(occupiedPaths: readonly string[]): ReadonlyMap<string, readonly string[]> {
  const cached = projectionOccupancyIndexes.get(occupiedPaths);
  if (cached) return cached;
  const index = new Map<string, string[]>();
  for (const path of occupiedPaths) for (const key of new Set([path, projectionCanonicalKey(path, "nfc"), projectionCanonicalKey(path, "case-fold"), projectionCanonicalKey(path, "vs16-fold")])) {
    const rows = index.get(key);
    if (rows) rows.push(path);
    else index.set(key, [path]);
  }
  projectionOccupancyIndexes.set(occupiedPaths, index);
  return index;
}

export function artifactProjectionPathProblems(paths: readonly Readonly<{ path: string; nodeKind: "directory" | "file" }>[], occupiedPaths: readonly string[], taxonomy: Taxonomy, occupiedExclusionRoot?: string): string[] {
  const problems: string[] = [];
  for (const comparison of ["nfc", "case-fold", "vs16-fold"] as const) {
    const seen = new Map<string, string>();
    for (const entry of paths) {
      const key = `${entry.nodeKind}\0${projectionCanonicalKey(entry.path, comparison)}`;
      const prior = seen.get(key);
      if (prior && prior !== entry.path) problems.push(`${comparison} collision between ${JSON.stringify(prior)} and ${JSON.stringify(entry.path)}.`);
      seen.set(key, entry.path);
    }
  }
  const occupied = projectionOccupancyIndex(occupiedPaths), occupant = (candidate: string): boolean => occupiedExclusionRoot === undefined || candidate !== occupiedExclusionRoot && !candidate.startsWith(`${occupiedExclusionRoot}/`);
  for (const { path } of paths) {
    if ([path, projectionCanonicalKey(path, "nfc"), projectionCanonicalKey(path, "case-fold"), projectionCanonicalKey(path, "vs16-fold")].some((key) => occupied.get(key)?.some(occupant))) problems.push(`Projected destination ${JSON.stringify(path)} is occupied.`);
    for (const code of pathBudgetCodes(path, taxonomy.collisionPolicy)) problems.push(`Projected destination ${JSON.stringify(path)} violates ${code} (${pathBudgetStatement(taxonomy.collisionPolicy)}).`);
  }
  return problems;
}

export function projectionJsonManifest(node: SemanticProjectionAuthorityNode, scope: string, problems: string[]): Record<string, unknown> | null {
  if (node.nodeKind !== "file" || typeof node.content !== "string") {
    problems.push(`${scope} must be a readable JSON file.`);
    return null;
  }
  try {
    const value = JSON.parse(node.content) as unknown;
    if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("not an object");
    return value as Record<string, unknown>;
  } catch {
    problems.push(`${scope} must contain one JSON object.`);
    return null;
  }
}

export function projectionFileMappingDigest(mappings: readonly SemanticPathProjectionMapping[]): string {
  return createHash("sha256").update(mappings.map(({ sourcePath, destinationPath }) => `${sourcePath}\0${destinationPath}`).join("\n")).digest("hex");
}
