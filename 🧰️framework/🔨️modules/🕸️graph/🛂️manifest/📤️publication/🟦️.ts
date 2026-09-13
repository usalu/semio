/** 📤️ Exact no-follow graph artifact publication and stale removal. */
import { existsSync, lstatSync, mkdirSync, readdirSync, rmdirSync, unlinkSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { writeGeneratedFileIfChanged } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts";
import type { GraphArtifact } from "../📽️projection/🟦️.ts";

export type GraphOutputNode = Readonly<{ path: string; nodeKind: "file" | "directory" }>;

/** 🌳️Reads the exact no-follow output inventory, including nested manifest owners. */
export function graphOutputInventory(outDir: string): readonly GraphOutputNode[] {
  if (!existsSync(outDir)) return [];
  if (!lstatSync(outDir).isDirectory()) throw new Error("graph output root must be a real directory");
  const nodes: GraphOutputNode[] = [];
  function visit(dir: string): void {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (!entry.isDirectory() && !entry.isFile()) throw new Error(`graph output contains an unsupported entry: ${entry.name}`);
      const absolute = join(dir, entry.name);
      nodes.push({ path: relative(outDir, absolute).replaceAll("\\", "/"), nodeKind: entry.isDirectory() ? "directory" : "file" });
      if (entry.isDirectory()) visit(absolute);
    }
  }
  visit(outDir);
  return nodes.sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

/** 🗺️Derives parent directories solely from already explicit rendered output paths. */
export function graphOutputNodes(outDir: string, artifacts: readonly GraphArtifact[]): readonly GraphOutputNode[] {
  const nodes = new Map<string, "file" | "directory">();
  for (const artifact of artifacts) {
    const path = relative(resolve(outDir), resolve(artifact.path)).replaceAll("\\", "/");
    if (!path || path.startsWith("/") || /^[A-Za-z]:/u.test(path) || path.split("/").some((part) => part === ".." || part === ".") || nodes.has(path)) throw new Error("graph output artifact is duplicated or outside its owner");
    nodes.set(path, "file");
    let parent = dirname(path).replaceAll("\\", "/");
    while (parent !== ".") {
      if (nodes.get(parent) === "file") throw new Error("graph output file conflicts with a directory");
      nodes.set(parent, "directory");
      parent = dirname(parent).replaceAll("\\", "/");
    }
  }
  return [...nodes].map(([path, nodeKind]) => ({ path, nodeKind })).sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

/** 🧹️Writes the exact nested set after preflight; removes stale leaves and only empty directories. */
export function writeGraphArtifacts(outDir: string, artifacts: readonly GraphArtifact[]): void {
  const expected = graphOutputNodes(outDir, artifacts);
  const actual = graphOutputInventory(outDir);
  const kinds = new Map(expected.map((entry) => [entry.path, entry.nodeKind]));
  const stale = actual.filter((entry) => kinds.get(entry.path) !== entry.nodeKind);
  mkdirSync(outDir, { recursive: true });
  for (const entry of stale.filter((entry) => entry.nodeKind === "file")) unlinkSync(join(outDir, entry.path));
  for (const entry of stale.filter((entry) => entry.nodeKind === "directory").sort((left, right) => right.path.length - left.path.length)) rmdirSync(join(outDir, entry.path));
  for (const entry of expected.filter((entry) => entry.nodeKind === "directory")) mkdirSync(join(outDir, entry.path), { recursive: true });
  for (const artifact of artifacts) writeGeneratedFileIfChanged(artifact.path, artifact.content);
}
