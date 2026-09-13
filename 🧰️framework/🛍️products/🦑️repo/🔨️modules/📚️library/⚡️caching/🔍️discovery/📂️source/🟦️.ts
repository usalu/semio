import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, sep } from "node:path";

export const CACHE_POLICY_PATH = fileURLToPath(new URL("../../🔣️policy.json", import.meta.url));
export const CACHE_POLICY = JSON.parse(readFileSync(CACHE_POLICY_PATH, "utf8"));

export interface CacheSourceEntry {
  readonly name: string;
  isDirectory(): boolean;
  isFile(): boolean;
  isSymbolicLink(): boolean;
}

export interface CacheSourceOperations {
  readDirectory(path: string): readonly CacheSourceEntry[];
}

export const slash = (path: string): string => path.split(sep).join("/");

/** 📂️ Enumerates source entries without traversing generated stores, symlinks or opaque trees. */
export function sourceFiles(root: string, operations: CacheSourceOperations = { readDirectory: (path) => readdirSync(path, { withFileTypes: true }) }): string[] {
  const files: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of operations.readDirectory(join(root, directory))) {
      const path = slash(join(directory, entry.name));
      if (entry.isSymbolicLink() || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || CACHE_POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.push(path);
    }
  };
  walk("");
  return files.sort();
}
