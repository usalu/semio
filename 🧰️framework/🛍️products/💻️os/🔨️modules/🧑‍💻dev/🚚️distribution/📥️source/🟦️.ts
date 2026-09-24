/** 🧩️ Semantic distribution source owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { DISTRIBUTION_LAYOUT, distributionOutputOwner, parseDistributionManifest, parseDistributionStaticInputs, type DistributionInput, type DistributionLayout, type DistributionManifest } from "../🟦️.ts";



/** ⚖️ Orders protocol coordinates by UTF-8 bytes on every supported host. */
function distributionPathOrder(left: string, right: string): number { return Buffer.from(left).compare(Buffer.from(right)); }

/** 🔏️ Streams exact file bytes without loading copied namespaces into a manifest. */
async function distributionFileWitness(path: string, coordinate: string): Promise<DistributionInput> {
  const digest = createHash("sha256");
  let bytes = 0;
  for await (const chunk of createReadStream(path)) { bytes += chunk.length; digest.update(chunk); }
  return { path: coordinate, bytes, sha256: digest.digest("hex") };
}

/** 🔎️ Distinguishes absent entries from symlinks, including dangling links. */
function distributionNode(path: string): ReturnType<typeof lstatSync> | null {
  try { return lstatSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
}

/** 🧱️ Rejects a publication coordinate if any existing destination ancestor is not a real directory. */
function distributionRealAncestors(destination: string, path: string): void {
  const segments = path.split("/"), root = distributionNode(destination);
  if (root && (!root.isDirectory() || root.isSymbolicLink())) throw new Error("Distribution destination must be a real directory");
  let current = destination;
  for (const segment of segments.slice(0, -1)) {
    current = join(current, segment);
    const node = distributionNode(current);
    if (!node) return;
    if (!node.isDirectory() || node.isSymbolicLink()) throw new Error(`Distribution publication ancestor is not a real directory: ${relative(destination, current).replaceAll("\\", "/")}`);
  }
}

/** 🕸️ Resolves authored static imports with the installed compiler and records external package boundaries. */
async function distributionStaticSourcePaths(workspace: string, entries: readonly string[]): Promise<string[]> {
  const { registryStaticImports } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"), { builtinModules } = await import("node:module");
  const root = realpathSync(workspace), paths = new Set<string>(), visited = new Set<string>();
  const coordinate = (absolute: string) => {
    const path = relative(root, absolute).replaceAll("\\", "/");
    if (!path || path === ".." || path.startsWith("../") || path.startsWith("/")) throw new Error(`Compiler source escapes its workspace: ${absolute}`);
    return path.normalize("NFC");
  };
  const visit = (absolute: string) => {
    coordinate(absolute);
    const canonical = realpathSync(absolute), path = coordinate(canonical);
    if (visited.has(path)) return;
    if (!statSync(canonical).isFile()) throw new Error(`Compiler source is not a file: ${path}`);
    visited.add(path);
    paths.add(path);
    if (path.split("/").includes("node_modules")) {
      let parent = dirname(canonical);
      while (parent !== root) {
        const manifest = join(parent, "package.json");
        if (existsSync(manifest)) { paths.add(coordinate(realpathSync(manifest))); break; }
        const next = dirname(parent);
        if (next === parent) throw new Error(`Compiler package boundary is missing: ${path}`);
        parent = next;
      }
      return;
    }
    if (!/\.[cm]?[jt]sx?$/u.test(path)) return;
    for (const specifier of registryStaticImports(readFileSync(canonical, "utf8"), path)) {
      if (specifier.startsWith("node:") || specifier.startsWith("bun:") || builtinModules.includes(specifier)) continue;
      const resolved = Bun.resolveSync(specifier, dirname(canonical));
      if (!specifier.startsWith(".")) {
        let parent = dirname(resolved);
        while (parent !== root) {
          const manifest = join(parent, "package.json");
          if (existsSync(manifest)) { paths.add(coordinate(realpathSync(manifest))); break; }
          const next = dirname(parent);
          if (next === parent) throw new Error(`Compiler import has no package boundary: ${specifier}`);
          parent = next;
        }
      }
      visit(resolved);
    }
  };
  for (const entry of entries) {
    if (!entry || entry.startsWith("/") || /[\\\u0000-\u001f:]/u.test(entry) || entry.split("/").some(part => !part || part === "." || part === "..")) throw new Error(`Invalid compiler entry coordinate: ${entry}`);
    visit(join(root, entry));
  }
  return [...paths].sort(distributionPathOrder);
}

export { distributionFileWitness, distributionNode, distributionPathOrder, distributionRealAncestors, distributionStaticSourcePaths };
