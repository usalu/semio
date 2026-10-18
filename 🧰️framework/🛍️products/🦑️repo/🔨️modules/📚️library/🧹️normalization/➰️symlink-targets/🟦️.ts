import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { posix } from "node:path";
import { type TaxonomyEmbeddedTicketRootDisposition, type TaxonomyEmbeddedTicketRootRelocation, type TaxonomyEvidenceRemoval, type TaxonomyInventory, type TaxonomyInventoryEntry, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";

export function projectedPath(path: string, entries: readonly TaxonomyInventoryEntry[]): string {
  const mappings = entries.filter((entry) => entry.sourcePath !== entry.normalizedPath && (path === entry.sourcePath || path.startsWith(`${entry.sourcePath}/`))).sort((left, right) => right.sourcePath.length - left.sourcePath.length || generatorPathCompare(left.sourcePath, right.sourcePath));
  if (mappings.length === 0) return path;
  const longest = requireTaxonomyValue(mappings[0],"mappings[0]").sourcePath.length;
  const destinations = new Set(mappings.filter((entry) => entry.sourcePath.length === longest).map((entry) => `${entry.normalizedPath}${path.slice(entry.sourcePath.length)}`));
  if (destinations.size !== 1) throw new Error(`Path projection is ambiguous for ${path}`);
  return requireTaxonomyValue([...destinations][0],"unique projected destination");
}

/** 🧷️ Resolves a POSIX, drive-rooted, or UNC absolute target only when it is lexically inside the same repository root. */
export function repositoryLocalSymlinkTargetPath(repoRoot: string, target: string): string | null {
  if (repoRoot.includes("\u0000") || target.includes("\u0000")) return null;
  const slash = (value: string): string => value.replaceAll("\\", "/").replace(/\/+$/u, "");
  const root = slash(repoRoot);
  const candidate = slash(target);
  const drive = /^([A-Za-z]):\/(.*)$/u;
  const rootDrive = drive.exec(root);
  const targetDrive = drive.exec(candidate);
  const unc = /^\/\/([^/]+)\/([^/]+)(?:\/(.*))?$/u;
  const rootUnc = unc.exec(root);
  const targetUnc = unc.exec(candidate);
  let suffix: string;
  if (rootDrive || targetDrive) {
    if (!rootDrive || !targetDrive || requireTaxonomyValue(rootDrive[1],"rootDrive[1]").toLowerCase() !== requireTaxonomyValue(targetDrive[1],"targetDrive[1]").toLowerCase()) return null;
    const rootTail = requireTaxonomyValue(rootDrive[2],"rootDrive[2]").replace(/\/+$/u, "");
    const targetTail = requireTaxonomyValue(targetDrive[2],"targetDrive[2]");
    if (targetTail.toLowerCase() !== rootTail.toLowerCase() && !targetTail.toLowerCase().startsWith(`${rootTail.toLowerCase()}/`)) return null;
    suffix = targetTail.slice(rootTail.length).replace(/^\//u, "");
  } else if (rootUnc || targetUnc) {
    if (!rootUnc || !targetUnc || requireTaxonomyValue(rootUnc[1],"rootUnc[1]").toLowerCase() !== requireTaxonomyValue(targetUnc[1],"targetUnc[1]").toLowerCase() || requireTaxonomyValue(rootUnc[2],"rootUnc[2]").toLowerCase() !== requireTaxonomyValue(targetUnc[2],"targetUnc[2]").toLowerCase()) return null;
    const rootTail = (rootUnc[3] ?? "").replace(/\/+$/u, "");
    const targetTail = targetUnc[3] ?? "";
    if (targetTail.toLowerCase() !== rootTail.toLowerCase() && !targetTail.toLowerCase().startsWith(`${rootTail.toLowerCase()}/`)) return null;
    suffix = targetTail.slice(rootTail.length).replace(/^\//u, "");
  } else {
    if (!root.startsWith("/") || !candidate.startsWith("/") || (candidate !== root && !candidate.startsWith(`${root}/`))) return null;
    suffix = candidate.slice(root.length).replace(/^\//u, "");
  }
  if (!suffix || suffix.split("/").some((segment) => segment === "" || segment === "." || segment === "..")) return null;
  try { return normalizeRelative(suffix); } catch { return null; }
}

export function logicalRepositorySymlinkTargetPath(repoRoot: string, sourcePath: string, target: string): string | null {
  const absoluteSyntax = target.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(target) || /^(?:\\\\|\/\/)/u.test(target);
  if (absoluteSyntax) return repositoryLocalSymlinkTargetPath(repoRoot, target);
  try { return normalizeRelative(posix.join(posix.dirname(sourcePath), target.replaceAll("\\", "/"))); } catch { return null; }
}

export interface EmbeddedDispositionPlanning {
  readonly roots: readonly TaxonomyEmbeddedTicketRootDisposition[];
  readonly relocations: readonly TaxonomyEmbeddedTicketRootRelocation[];
  readonly removals: readonly TaxonomyEvidenceRemoval[];
  readonly violations: readonly TaxonomyViolation[];
}

export function incomingEmbeddedReferences(inventory: TaxonomyInventory, root: string): readonly string[] {
  const rows = new Set<string>();
  for (const entry of inventory.entries.filter((candidate) => candidate.sourcePath === root || candidate.sourcePath.startsWith(`${root}/`))) for (const source of entry.referencesIn) if (source !== root && !source.startsWith(`${root}/`)) rows.add(`text\u0000${source}\u0000${entry.sourcePath}`);
  for (const link of inventory.entries.filter((candidate) => candidate.nodeKind === "symlink" && candidate.symlinkTarget !== undefined && candidate.sourcePath !== root && !candidate.sourcePath.startsWith(`${root}/`))) {
    let target: string | null = null;
    if (link.symlinkTarget!.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(link.symlinkTarget!) || /^(?:\\\\|\/\/)/u.test(link.symlinkTarget!)) target = repositoryLocalSymlinkTargetPath(inventory.repoRoot, link.symlinkTarget!);
    else try { target = normalizeRelative(posix.join(posix.dirname(link.sourcePath), link.symlinkTarget!.replaceAll("\\", "/"))); } catch { target = null; }
    if (target && (target === root || target.startsWith(`${root}/`))) rows.add(`symlink\u0000${link.sourcePath}\u0000${target}`);
  }
  return [...rows].sort(generatorPathCompare);
}

export interface JsonStringCoordinate { readonly pointer: string; readonly start: number; readonly end: number; readonly value: string; readonly rawValue?: string }

export function jsonStringCoordinates(content: string, includeEscaped = false): readonly JsonStringCoordinate[] {
  try { JSON.parse(content); } catch { return []; }
  const lexemes = [...content.matchAll(/"(?:\\.|[^"\\])*"|[{}\[\]:,]|true|false|null|-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/gu)];
  const rows: JsonStringCoordinate[] = [];
  let cursor = 0;
  const require = (value: string): void => { if (lexemes[cursor++]?.[0] !== value) throw new Error("Invalid JSON coordinate syntax"); };
  const visit = (pointer: string): void => {
    const token = lexemes[cursor++];
    if (!token) throw new Error("Missing JSON coordinate value");
    if (token[0] === "{") {
      const keys = new Set<string>();
      while (lexemes[cursor]?.[0] !== "}") {
        const key = JSON.parse(lexemes[cursor++]![0]) as string;
        if (typeof key !== "string" || keys.has(key)) throw new Error("Duplicate JSON coordinate key");
        keys.add(key);
        require(":");
        visit(pointer + "/" + key.replaceAll("~", "~0").replaceAll("/", "~1"));
        if (lexemes[cursor]?.[0] === "}") break;
        require(",");
      }
      require("}");
    } else if (token[0] === "[") {
      let index = 0;
      while (lexemes[cursor]?.[0] !== "]") {
        visit(pointer + "/" + index++);
        if (lexemes[cursor]?.[0] === "]") break;
        require(",");
      }
      require("]");
    } else if (token[0].startsWith('"')) {
      const value = JSON.parse(token[0]) as string;
      const rawValue = token[0].slice(1, -1);
      if (rawValue === value || includeEscaped) rows.push({ pointer, start: token.index! + 1, end: token.index! + token[0].length - 1, value, ...(rawValue === value ? {} : { rawValue }) });
    }
  };
  try { visit(""); return cursor === lexemes.length ? rows : []; } catch { return []; }
}
