import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { getRepoMetaDir, getSemioRoot, HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, REPO_META_DIR_NAME, SPACE_DATA_DIR_NAME } from "../../🟦️.ts";
import { TICKET_GENERATED_OUTPUT_DIRECTORY } from "../../🧹️normalization/🟦️.ts";

export const CLEAN_CANONICAL_REPO_DIR = REPO_META_DIR_NAME;
export const CLEAN_CANONICAL_TICKETS_DIR = "🎫️tickets";
export const CLEAN_BUILD_DIR_NAMES = new Set(["target", "dist", "build", "out"]);
export const CLEAN_CACHE_DIR_NAME = "⚡️cache";
export const CLEAN_TICKET_GENERATED_OUTPUT_DIRS = new Set(["🗑️generated", TICKET_GENERATED_OUTPUT_DIRECTORY, "🧾️runs", "🧪️runs", "🧾️taxonomy-transaction"]);
export const CLEAN_TICKET_GENERATED_PROBE_PREFIXES = ["🧪️purity-", "🧪️cli-", "🧪️inventory-"];

export type CleanRemovalKind = "misplaced" | "gitignore" | "ticket-file" | "ticket-dir" | "build-artifact" | "ticket-generated" | "windows-illegal" | "marker-only-folder";

export type CleanRemoval = {
  kind: CleanRemovalKind;
  path: string;
  bytes: number;
};
export function cleanEndsWithAscii(name: string, ascii: string): boolean {
  return name.endsWith(ascii);
}

export function cleanIsCanonicalRepoDir(name: string): boolean {
  return name === CLEAN_CANONICAL_REPO_DIR;
}

export function cleanIsCanonicalTicketsDir(name: string): boolean {
  return name === CLEAN_CANONICAL_TICKETS_DIR;
}

export function cleanIsMisplacedRepoDir(name: string): boolean {
  return cleanEndsWithAscii(name, "repo") && !cleanIsCanonicalRepoDir(name);
}

export function cleanIsMisplacedTicketsDir(name: string): boolean {
  return cleanEndsWithAscii(name, "tickets") && !cleanIsCanonicalTicketsDir(name);
}

export function cleanIsBuildArtifactDirName(name: string): boolean {
  if (CLEAN_BUILD_DIR_NAMES.has(name)) return true;
  if (cleanIsCargoTargetDirName(name)) return true;
  return false;
}

/** 🦀️Every Cargo target dir outside the shared cache root is stray — nothing writes there since `.cargo/config.toml` moved `build.target-dir`/`build.build-dir` under the cache root. Names are exact (`🎯️targets` is a source taxonomy folder). */
export function cleanIsCargoTargetDirName(name: string): boolean {
  return name === "target" || name === "🎯️target" || name.startsWith("target-") || name.startsWith("🎯️target-");
}

/** 🏷️Cargo stamps every target and build dir with `CACHEDIR.TAG`; only such dirs are provably Cargo output. https://bford.info/cachedir/ */
export function cleanIsCargoTargetDir(abs: string, name: string): boolean {
  return cleanIsCargoTargetDirName(name) && existsSync(join(abs, "CACHEDIR.TAG"));
}

export function cleanIsSemioRootName(name: string): boolean {
  return name === ".🧬semio" || (name.startsWith(".🧬") && name.endsWith("semio"));
}

export const CLEAN_WINDOWS_RESERVED_DEVICE_NAMES = /^(con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³])(\..*)?$/iu;
export const CLEAN_WINDOWS_FORBIDDEN_CHARS = /[<>:"|?*\x00-\x1f]/;

/** 🪟️ Checks whether a path component is forbidden on Windows filesystems (NTFS/Win32). */
export function cleanIsWindowsIllegalName(name: string): boolean {
  if (!name) return true;
  if (name === "." || name === "..") return false;
  if (name.trim().length === 0 || name.endsWith(" ") || name.endsWith(".") || name.startsWith(" ")) return true;
  if (CLEAN_WINDOWS_FORBIDDEN_CHARS.test(name)) return true;
  if (CLEAN_WINDOWS_RESERVED_DEVICE_NAMES.test(name)) return true;
  return false;
}

export function cleanProtectedPrefixes(root: string): string[] {
  const semio = getSemioRoot(root);
  return [join(semio, MAP_CACHE_DIR_NAME), join(semio, HUB_DATA_DIR_NAME), join(semio, SPACE_DATA_DIR_NAME), join(getRepoMetaDir(root), CLEAN_CACHE_DIR_NAME)].map((p) => resolve(p));
}

export function cleanIsProtected(abs: string, protectedPrefixes: readonly string[]): boolean {
  const resolved = resolve(abs);
  return protectedPrefixes.some((prefix) => resolved === prefix || resolved.startsWith(prefix + sep));
}

//#region 🛡️TicketProtection
export type CleanProtectionNodeKind = "directory" | "file" | "symlink" | "missing" | "unreadable";

/** 🪟️ Read-only filesystem projection used by the deletion gate and synthetic safety laws. */
export interface CleanProtectionView {
  kind(path: string): CleanProtectionNodeKind;
  children(path: string): readonly string[] | undefined;
  read(path: string): string | undefined;
}

export const CLEAN_PROTECTION_VIEW: CleanProtectionView = {
  kind(path) {
    try {
      const state = lstatSync(path);
      return state.isSymbolicLink() ? "symlink" : state.isDirectory() ? "directory" : state.isFile() ? "file" : "unreadable";
    } catch (error) {
      return (error as NodeJS.ErrnoException).code === "ENOENT" ? "missing" : "unreadable";
    }
  },
  children(path) {
    try {
      return readdirSync(path);
    } catch {
      return undefined;
    }
  },
  read(path) {
    try {
      return new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(path));
    } catch {
      return undefined;
    }
  },
};

export function cleanIntersectsProtected(abs: string, protectedPrefixes: readonly string[]): boolean {
  const candidate = resolve(abs);
  return protectedPrefixes.some((value) => {
    const prefix = resolve(value);
    return candidate === prefix || candidate.startsWith(prefix + sep) || prefix.startsWith(candidate + sep);
  });
}

export function cleanTicketManifestIsClosed(directory: string, view: CleanProtectionView): boolean {
  const path = join(directory, "🎫️ticket.json");
  try {
    if (view.kind(path) !== "file") return false;
    const text = view.read(path);
    if (text === undefined) return false;
    const manifest: unknown = JSON.parse(text);
    return manifest !== null && typeof manifest === "object" && !Array.isArray(manifest) && Object.hasOwn(manifest, "status") && (manifest as { status: unknown }).status === "closed";
  } catch {
    return false;
  }
}

export function cleanIsTicketFolderBoundary(root: string, directory: string): boolean {
  const segments = relative(root, directory).split(sep).filter(Boolean);
  return segments.some((name, index) => cleanEndsWithAscii(name, "tickets") && segments.length - index - 1 === 4);
}

export function cleanTicketFolderForPath(root: string, abs: string): string | undefined {
  let ancestor = dirname(abs);
  while (true) {
    const local = relative(root, ancestor);
    if (local === "" || local === ".." || local.startsWith(".." + sep) || isAbsolute(local)) return undefined;
    if (cleanIsTicketFolderBoundary(root, ancestor)) return resolve(ancestor);
    const parent = dirname(ancestor);
    if (parent === ancestor) return undefined;
    ancestor = parent;
  }
}

/** 🛡️ Rejects a removal intersecting any non-closed ticket, unsafe path, or unreadable subtree without following symlinks. */
export function cleanRemovalProtection(root: string, candidate: string, view: CleanProtectionView = CLEAN_PROTECTION_VIEW, allowedOpenTicket?: string): string[] {
  const workspace = resolve(root),
    target = resolve(workspace, candidate);
  const local = relative(workspace, target);
  if (local === "" || local === ".." || local.startsWith(".." + sep) || isAbsolute(local)) return [target];
  const protectedPaths = new Set<string>(),
    closedTickets = new Set<string>();
  const ancestors: { path: string; kind: CleanProtectionNodeKind }[] = [{ path: workspace, kind: "directory" }];
  const inspectDirectory = (directory: string): void => {
    const manifestKind = view.kind(join(directory, "🎫️ticket.json"));
    if (!cleanIsTicketFolderBoundary(workspace, directory) && manifestKind === "missing") return;
    if (cleanTicketManifestIsClosed(directory, view)) closedTickets.add(directory);
    else if (directory !== allowedOpenTicket) protectedPaths.add(directory);
  };
  let ancestor = workspace;
  try {
    if (view.kind(workspace) !== "directory") return [workspace];
    inspectDirectory(workspace);
    for (const segment of local.split(sep)) {
      ancestor = join(ancestor, segment);
      const kind = view.kind(ancestor);
      if (kind === "symlink" || kind === "missing" || kind === "unreadable" || (ancestor !== target && kind !== "directory")) return [ancestor];
      ancestors.push({ path: ancestor, kind });
      if (kind === "directory") inspectDirectory(ancestor);
    }
    if (protectedPaths.size > 0) return [...protectedPaths];
    const stack = view.kind(target) === "directory" ? [target] : [];
    while (stack.length > 0) {
      const directory = stack.pop()!;
      inspectDirectory(directory);
      if (protectedPaths.has(directory)) continue;
      const names = view.children(directory);
      if (!names) {
        protectedPaths.add(directory);
        continue;
      }
      for (const name of names) {
        if (!name || name === "." || name === ".." || name.includes("/") || name.includes("\\")) {
          protectedPaths.add(directory);
          continue;
        }
        const child = join(directory, name),
          kind = view.kind(child);
        if (kind === "directory") stack.push(child);
        else if (kind !== "file") protectedPaths.add(child);
      }
    }
    for (const entry of ancestors) if (view.kind(entry.path) !== entry.kind) protectedPaths.add(entry.path);
    for (const directory of closedTickets) if (view.kind(directory) !== "directory" || !cleanTicketManifestIsClosed(directory, view)) protectedPaths.add(directory);
  } catch {
    protectedPaths.add(ancestor);
  }
  return [...protectedPaths];
}

/** 🧮️ Removes unsafe candidates before shallow deduplication so a protected parent cannot absorb eligible siblings. */
export function cleanProjectRemovals(root: string, removals: readonly CleanRemoval[], protectedPrefixes: readonly string[] = [], view: CleanProtectionView = CLEAN_PROTECTION_VIEW, onProtected?: (path: string) => void): CleanRemoval[] {
  return cleanDedupePreferShallowest(
    removals.filter((row) => {
      const absolute = resolve(root, row.path);
      const allowedOpenTicket = row.kind === "ticket-generated" ? cleanTicketGeneratedOutputTicketRoot(root, absolute) : row.kind === "windows-illegal" ? cleanTicketFolderForPath(root, absolute) : undefined;
      const applicablePrefixes = allowedOpenTicket ? protectedPrefixes.filter((prefix) => resolve(prefix) !== allowedOpenTicket) : protectedPrefixes;
      const protectedPaths = cleanIntersectsProtected(absolute, applicablePrefixes) ? [absolute] : allowedOpenTicket ? [] : cleanRemovalProtection(root, row.path, view, allowedOpenTicket);
      for (const path of protectedPaths) onProtected?.(path);
      return protectedPaths.length === 0;
    }),
  );
}
//#endregion 🛡️TicketProtection

export function cleanIsTicketGeneratedOutputDir(name: string): boolean {
  return CLEAN_TICKET_GENERATED_OUTPUT_DIRS.has(name) || CLEAN_TICKET_GENERATED_PROBE_PREFIXES.some((prefix) => name.startsWith(prefix));
}

export function cleanTicketGeneratedOutputTicketRoot(root: string, abs: string): string | undefined {
  const name = relative(dirname(abs), abs);
  if (name.includes(sep) || !cleanIsTicketGeneratedOutputDir(name)) return undefined;
  return cleanTicketFolderForPath(root, abs);
}
export function cleanDedupePreferShallowest(removals: readonly CleanRemoval[]): CleanRemoval[] {
  const sorted = [...removals].sort((a, b) => a.path.length - b.path.length || a.path.localeCompare(b.path));
  const kept: CleanRemoval[] = [];
  for (const row of sorted) {
    if (kept.some((k) => row.path === k.path || row.path.startsWith(k.path + "/"))) continue;
    kept.push(row);
  }
  return kept;
}
