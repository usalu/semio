import { existsSync, lstatSync, readdirSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { exactCargoGeneratedOutputHasLiveLease, HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, runProbe, SPACE_DATA_DIR_NAME } from "../../🟦️.ts";
import {
  CLEAN_CACHE_DIR_NAME,
  CLEAN_CANONICAL_REPO_DIR,
  CLEAN_PROTECTION_VIEW,
  type CleanRemoval,
  cleanIntersectsProtected,
  cleanIsBuildArtifactDirName,
  cleanIsCanonicalRepoDir,
  cleanIsCanonicalTicketsDir,
  cleanIsCargoTargetDir,
  cleanIsMisplacedRepoDir,
  cleanIsMisplacedTicketsDir,
  cleanIsProtected,
  cleanIsSemioRootName,
  cleanIsTicketGeneratedOutputDir,
  cleanIsWindowsIllegalName,
  cleanTicketFolderForPath,
  cleanTicketManifestIsClosed,
} from "../🛡️protection/🟦️.ts";

export const CLEAN_TICKET_FILE_MAX_BYTES = 5 * 1024 * 1024;
export const CLEAN_TICKET_DIR_MAX_BYTES = 10 * 1024 * 1024;
export const CLEAN_BUILD_ARTIFACT_MAX_BYTES = 10 * 1024 * 1024 * 1024;
export function cleanPathBytes(abs: string): number {
  try {
    const st = lstatSync(abs);
    if (st.isSymbolicLink() || st.isFile()) return st.size;
    if (!st.isDirectory()) return 0;
    let total = 0;
    for (const name of readdirSync(abs)) total += cleanPathBytes(join(abs, name));
    return total;
  } catch {
    return 0;
  }
}

export function cleanWalkDirs(root: string, visit: (abs: string, name: string) => "enter" | "skip" | "stop"): void {
  const stack = [root];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      const abs = join(dir, name);
      let st;
      try {
        st = lstatSync(abs);
      } catch {
        continue;
      }
      if (!st.isDirectory() || st.isSymbolicLink()) continue;
      const action = visit(abs, name);
      if (action === "stop") return;
      if (action === "enter") stack.push(abs);
    }
  }
}

export function cleanCollectMisplaced(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  const seen = new Set<string>();
  const push = (abs: string): void => {
    if (cleanIntersectsProtected(abs, protectedPrefixes) || seen.has(abs)) return;
    seen.add(abs);
    out.push({ kind: "misplaced", path: relative(root, abs) || ".", bytes: cleanPathBytes(abs) });
  };
  const rootMisplaced = join(root, CLEAN_CANONICAL_REPO_DIR);
  if (existsSync(rootMisplaced)) push(rootMisplaced);
  cleanWalkDirs(root, (abs, name) => {
    if (name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME) return "skip";
    if (name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) return "skip";
    if (cleanIsBuildArtifactDirName(name)) return "skip";
    if (cleanIsSemioRootName(name)) {
      let children: string[];
      try {
        children = readdirSync(abs);
      } catch {
        return "skip";
      }
      for (const child of children) {
        const childAbs = join(abs, child);
        try {
          if (!lstatSync(childAbs).isDirectory()) continue;
        } catch {
          continue;
        }
        if (cleanIsMisplacedRepoDir(child)) push(childAbs);
        if (cleanIsCanonicalRepoDir(child)) {
          let ticketsKids: string[];
          try {
            ticketsKids = readdirSync(childAbs);
          } catch {
            continue;
          }
          for (const ticketsName of ticketsKids) {
            if (!cleanIsMisplacedTicketsDir(ticketsName)) continue;
            const ticketsAbs = join(childAbs, ticketsName);
            try {
              if (!lstatSync(ticketsAbs).isDirectory()) continue;
            } catch {
              continue;
            }
            push(ticketsAbs);
          }
        }
      }
      return "skip";
    }
    return "enter";
  });
  return out;
}

export function cleanDiscoverTicketRoots(root: string): string[] {
  const roots: string[] = [];
  cleanWalkDirs(root, (abs, name) => {
    if (name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME) return "skip";
    if (name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) return "skip";
    if (cleanIsBuildArtifactDirName(name)) return "skip";
    if (cleanIsCanonicalTicketsDir(name)) {
      roots.push(abs);
      return "skip";
    }
    return "enter";
  });
  return roots;
}

/** 📂Individual ticket slug folders (`…/🎆️YY/🌙️MM/☀️DD/TICKETSLUG`) — never year/month/day parents. */
export function cleanDiscoverTicketFolders(ticketsRoot: string): string[] {
  const folders: string[] = [];
  cleanWalkDirs(ticketsRoot, (abs, _name) => {
    const depth = relative(ticketsRoot, abs).split(sep).filter(Boolean).length;
    if (depth === 4) {
      folders.push(abs);
      return "skip";
    }
    if (depth > 4) return "skip";
    return "enter";
  });
  return folders;
}

export function cleanTicketSizeRemovals(root: string, ticketFolder: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  type Node = { abs: string; rel: string; isDir: boolean; size: number; children: Node[] };
  const build = (abs: string): Node | null => {
    if (cleanIsProtected(abs, protectedPrefixes)) return null;
    let st;
    try {
      st = lstatSync(abs);
    } catch {
      return null;
    }
    const rel = relative(root, abs);
    if (st.isSymbolicLink() || st.isFile()) return { abs, rel, isDir: false, size: st.size, children: [] };
    if (!st.isDirectory()) return null;
    const children: Node[] = [];
    let size = 0;
    for (const name of readdirSync(abs)) {
      const child = build(join(abs, name));
      if (!child) continue;
      children.push(child);
      size += child.size;
    }
    return { abs, rel, isDir: true, size, children };
  };
  const tree = build(ticketFolder);
  if (!tree) return [];
  const out: CleanRemoval[] = [];
  const visit = (node: Node): void => {
    for (const child of node.children) visit(child);
    if (!node.isDir && node.size > CLEAN_TICKET_FILE_MAX_BYTES) {
      out.push({ kind: "ticket-file", path: node.rel, bytes: node.size });
      return;
    }
    if (node.isDir && node.size > CLEAN_TICKET_DIR_MAX_BYTES) {
      out.push({ kind: "ticket-dir", path: node.rel, bytes: node.size });
    }
  };
  for (const child of tree.children) visit(child);
  return out;
}

export function cleanTicketGeneratedOutputRemovals(root: string, ticketFolder: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  cleanWalkDirs(ticketFolder, (abs, name) => {
    if (!cleanIsTicketGeneratedOutputDir(name)) return "enter";
    const applicablePrefixes = protectedPrefixes.filter((prefix) => resolve(prefix) !== resolve(ticketFolder));
    if (cleanIntersectsProtected(abs, applicablePrefixes) || cleanIsProtected(abs, applicablePrefixes)) return "skip";
    if (exactCargoGeneratedOutputHasLiveLease(abs)) return "skip";
    out.push({ kind: "ticket-generated", path: relative(root, abs), bytes: cleanPathBytes(abs) });
    return "skip";
  });
  return out;
}

export function cleanGitignoredMapForTicketRoots(root: string, ticketRoots: readonly string[]): Map<string, string[]> {
  const map = new Map<string, string[]>();
  for (const ticketRoot of ticketRoots) {
    const rel = relative(root, ticketRoot);
    const probe = runProbe("git", ["ls-files", "--others", "-i", "--exclude-standard", "--directory", "--", rel], {
      cwd: root,
      budgetMs: 120_000,
    });
    if ((probe.status ?? 1) !== 0) continue;
    for (const line of probe.stdout
      .split("\n")
      .map((l) => l.trim())
      .filter(Boolean)) {
      const abs = join(root, line.replace(/\/$/, ""));
      const folder = cleanTicketFolderForPath(root, abs);
      if (folder) {
        let list = map.get(folder);
        if (!list) {
          list = [];
          map.set(folder, list);
        }
        list.push(abs);
      }
    }
  }
  return map;
}

export function cleanCollectWindowsIllegal(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  const seen = new Set<string>();
  const stack = [root];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      if (name === ".git" || name === "node_modules") continue;
      if (name === CLEAN_CACHE_DIR_NAME || name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) continue;
      if (cleanIsBuildArtifactDirName(name)) continue;
      const abs = join(dir, name);
      if (cleanIntersectsProtected(abs, protectedPrefixes)) continue;
      if (cleanIsWindowsIllegalName(name)) {
        if (!seen.has(abs)) {
          seen.add(abs);
          out.push({ kind: "windows-illegal", path: relative(root, abs), bytes: cleanPathBytes(abs) });
        }
        continue;
      }
      let st;
      try {
        st = lstatSync(abs);
      } catch {
        continue;
      }
      if (st.isDirectory() && !st.isSymbolicLink()) {
        stack.push(abs);
      }
    }
  }
  const probe = runProbe("git", ["ls-files", "-z"], { cwd: root, budgetMs: 60_000 });
  if ((probe.status ?? 1) === 0) {
    for (const file of probe.stdout.split("\0").filter(Boolean)) {
      const parts = file.split("/");
      if (parts.some((part) => cleanIsWindowsIllegalName(part))) {
        const abs = join(root, file);
        if (!cleanIntersectsProtected(abs, protectedPrefixes) && !seen.has(abs)) {
          seen.add(abs);
          out.push({ kind: "windows-illegal", path: file, bytes: cleanPathBytes(abs) });
        }
      }
    }
  }
  return out;
}

export function cleanBuildArtifactRemovals(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  cleanWalkDirs(root, (abs, name) => {
    if (name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME) return "skip";
    if (name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) return "skip";
    if (cleanIsCanonicalTicketsDir(name)) return "skip";
    if (cleanIsProtected(abs, protectedPrefixes)) return "skip";
    if (cleanIsBuildArtifactDirName(name)) {
      if (cleanIntersectsProtected(abs, protectedPrefixes)) return "skip";
      const bytes = cleanPathBytes(abs);
      if (cleanIsCargoTargetDir(abs, name) || bytes > CLEAN_BUILD_ARTIFACT_MAX_BYTES) out.push({ kind: "build-artifact", path: relative(root, abs), bytes });
      return "skip";
    }
    return "enter";
  });
  return out;
}

/** Ticket size hits keep the deepest path so `target/` wins over year/month parents. */
export function cleanDedupePreferDeepest(removals: readonly CleanRemoval[]): CleanRemoval[] {
  const sorted = [...removals].sort((a, b) => b.path.length - a.path.length || a.path.localeCompare(b.path));
  const kept: CleanRemoval[] = [];
  for (const row of sorted) {
    if (kept.some((k) => k.path === row.path || k.path.startsWith(row.path + "/"))) continue;
    kept.push(row);
  }
  return kept;
}

/** Prefer shallow removals across kinds so a parent delete absorbs nested hits. */
