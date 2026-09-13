import { lstatSync, readdirSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import { canonicalFilenameForKind, loadCatalogTaxonomy, type Taxonomy } from "../../🔍️discovery/🟦️.ts";
import { HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, SPACE_DATA_DIR_NAME } from "../../🟦️.ts";
import { cleanWalkDirs, cleanPathBytes } from "../🔍️candidate-discovery/🟦️.ts";
import { CLEAN_CACHE_DIR_NAME, type CleanRemoval, cleanIsBuildArtifactDirName, cleanIsProtected } from "../🛡️protection/🟦️.ts";

const DIFF_FACET_DIR = "🔺️diff";
const TICKET_IMPORTANT_LEAF = "📝️.md";

/** 🏷️ Builds the zero-byte marker filename set from taxonomy plus ticket lifecycle leaves. */
export function cleanEmptyMarkerFilenames(taxonomy: Taxonomy = loadCatalogTaxonomy()): ReadonlySet<string> {
  const names = new Set<string>();
  for (const [kindId, kind] of Object.entries(taxonomy.fileKinds)) {
    if (kind.role !== "marker") continue;
    names.add(canonicalFilenameForKind(kindId, taxonomy));
  }
  names.add(TICKET_IMPORTANT_LEAF);
  return names;
}

/** 🪪 True when `path` is a regular zero-byte file whose basename is a tracked empty marker. */
export function cleanIsEmptyMarkerFile(abs: string, names: ReadonlySet<string>): boolean {
  let stat;
  try {
    stat = lstatSync(abs);
  } catch {
    return false;
  }
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size !== 0) return false;
  return names.has(basename(abs));
}

/** 🌲 True when every file in `dir` and its descendants is an empty marker and `dir` is not a diff facet root. */
export function cleanDirectoryTreeIsOnlyEmptyMarkers(dir: string, names: ReadonlySet<string>): boolean {
  if (basename(dir) === DIFF_FACET_DIR) return false;
  let sawFile = false;
  const stack = [dir];
  while (stack.length > 0) {
    const current = stack.pop()!;
    for (const name of readdirSync(current)) {
      const abs = join(current, name);
      let stat;
      try {
        stat = lstatSync(abs);
      } catch {
        return false;
      }
      if (stat.isSymbolicLink()) return false;
      if (stat.isDirectory()) {
        if (name === DIFF_FACET_DIR) return false;
        stack.push(abs);
        continue;
      }
      if (!stat.isFile()) return false;
      sawFile = true;
      if (!cleanIsEmptyMarkerFile(abs, names)) return false;
    }
  }
  return sawFile;
}

function cleanSkipWalkDir(name: string): boolean {
  return name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME || name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME || cleanIsBuildArtifactDirName(name);
}

/** 🧹 Collects topmost directories whose entire tree is only empty marker files (diff facets excluded). */
export function cleanCollectMarkerOnlyFolderRemovals(root: string, protectedPrefixes: readonly string[], taxonomy: Taxonomy = loadCatalogTaxonomy()): CleanRemoval[] {
  const names = cleanEmptyMarkerFilenames(taxonomy);
  const markerOnly = new Set<string>();
  cleanWalkDirs(root, (abs, name) => {
    if (cleanSkipWalkDir(name)) return "skip";
    if (cleanIsProtected(abs, protectedPrefixes)) return "skip";
    if (cleanDirectoryTreeIsOnlyEmptyMarkers(abs, names)) markerOnly.add(abs);
    return "enter";
  });
  const roots: string[] = [];
  for (const abs of markerOnly) {
    const parent = resolve(abs, "..");
    if (markerOnly.has(parent)) continue;
    roots.push(abs);
  }
  return roots
    .sort((left, right) => right.length - left.length || left.localeCompare(right))
    .map((abs) => ({ kind: "marker-only-folder" as const, path: relative(root, abs) || ".", bytes: cleanPathBytes(abs) }));
}
