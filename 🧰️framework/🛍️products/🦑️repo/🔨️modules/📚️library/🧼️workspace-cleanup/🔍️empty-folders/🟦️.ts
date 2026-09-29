import { lstatSync, readdirSync } from "node:fs";
import { relative } from "node:path";
import { HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, SPACE_DATA_DIR_NAME } from "../../🟦️.ts";
import { cleanWalkDirs, cleanPathBytes } from "../🔍️candidate-discovery/🟦️.ts";
import {
  CLEAN_CACHE_DIR_NAME,
  type CleanRemoval,
  cleanIsBuildArtifactDirName,
  cleanIsProtected,
} from "../🛡️protection/🟦️.ts";

function cleanSkipWalkDir(name: string): boolean {
  return name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME || name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME || cleanIsBuildArtifactDirName(name);
}

/** 📭 True when `dir` is an existing regular directory with no entries (symlinks are not followed). */
export function cleanDirectoryIsEmpty(dir: string): boolean {
  let stat;
  try {
    stat = lstatSync(dir);
  } catch {
    return false;
  }
  if (!stat.isDirectory() || stat.isSymbolicLink()) return false;
  try {
    return readdirSync(dir).length === 0;
  } catch {
    return false;
  }
}

/** 🧹 Collects every directory whose immediate children list is empty, deepest paths first. */
export function cleanCollectEmptyFolderRemovals(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const candidates = new Set<string>();
  cleanWalkDirs(root, (abs, name) => {
    if (cleanSkipWalkDir(name)) return "skip";
    if (cleanIsProtected(abs, protectedPrefixes)) return "skip";
    if (cleanDirectoryIsEmpty(abs)) candidates.add(abs);
    return "enter";
  });
  return [...candidates]
    .sort((left, right) => right.length - left.length || left.localeCompare(right))
    .map((abs) => ({ kind: "empty-folder" as const, path: relative(root, abs) || ".", bytes: cleanPathBytes(abs) }));
}
