import { lstatSync, readFileSync, readdirSync } from "node:fs";
import { isAbsolute, join, relative, resolve, sep } from "node:path";

export type PolicySourceEntry = Readonly<{ name: string; isFile: boolean; isDirectory: boolean; isSymbolicLink: boolean }>;

export type PolicySourceOperations = Readonly<{
  lstat: (path: string) => Readonly<{ isFile: boolean; isDirectory: boolean; isSymbolicLink: boolean }>;
  readFile: (path: string) => string;
  readdir: (path: string) => readonly PolicySourceEntry[];
}>;

export type PolicySourceDirectoryResult = Readonly<{ state: "directory"; entries: readonly PolicySourceEntry[] }> | Readonly<{ state: "missing" | "unreadable" | "symlink" | "not-directory"; entries: readonly [] }>;

export type PolicySourceTextResult = Readonly<{ state: "file"; text: string }> | Readonly<{ state: "missing" | "unreadable" | "symlink" | "not-file"; text: "" }>;

export const POLICY_SKIP_DIRS = new Set(["compose", "node_modules", ".git", ".🧬semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);

export const POLICY_SOURCE_OPERATIONS: PolicySourceOperations = {
  lstat: (path) => {
    const stat = lstatSync(path);
    return { isFile: stat.isFile(), isDirectory: stat.isDirectory(), isSymbolicLink: stat.isSymbolicLink() };
  },
  readFile: (path) => readFileSync(path, "utf8"),
  readdir: (path) => readdirSync(path, { withFileTypes: true }).map((entry) => ({ name: entry.name, isFile: entry.isFile(), isDirectory: entry.isDirectory(), isSymbolicLink: entry.isSymbolicLink() })),
};

export function policySourceUnavailableState(error: unknown): "missing" | "unreadable" {
  const code = error && typeof error === "object" && "code" in error ? String(error.code) : "";
  return code === "ENOENT" || code === "ENOTDIR" ? "missing" : "unreadable";
}

export function policySourceAncestry(repoRoot: string, relPath: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): "clear" | "missing" | "unreadable" | "symlink" | "not-directory" {
  const root = resolve(repoRoot),
    target = resolve(root, relPath),
    locator = relative(root, target);
  if (locator === ".." || locator.startsWith(`..${sep}`) || isAbsolute(locator)) return "unreadable";
  const segments = locator ? locator.split(sep) : [],
    ancestors = segments.length ? [root, ...segments.slice(0, -1).map((_, index) => join(root, ...segments.slice(0, index + 1)))] : [];
  for (const ancestor of ancestors) {
    let stat: ReturnType<PolicySourceOperations["lstat"]>;
    try {
      stat = operations.lstat(ancestor);
    } catch (error) {
      return policySourceUnavailableState(error);
    }
    if (stat.isSymbolicLink) return "symlink";
    if (!stat.isDirectory) return "not-directory";
  }
  return "clear";
}

export function policySourceDirectory(repoRoot: string, relDir: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicySourceDirectoryResult {
  const path = join(repoRoot, relDir);
  const ancestry = policySourceAncestry(repoRoot, relDir, operations);
  if (ancestry !== "clear") return { state: ancestry, entries: [] };
  let stat: ReturnType<PolicySourceOperations["lstat"]>;
  try {
    stat = operations.lstat(path);
  } catch (error) {
    return { state: policySourceUnavailableState(error), entries: [] };
  }
  if (stat.isSymbolicLink) return { state: "symlink", entries: [] };
  if (!stat.isDirectory) return { state: "not-directory", entries: [] };
  try {
    return { state: "directory", entries: operations.readdir(path) };
  } catch (error) {
    return { state: policySourceUnavailableState(error), entries: [] };
  }
}

export function policySourceText(repoRoot: string, relPath: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicySourceTextResult {
  const path = join(repoRoot, relPath);
  const ancestry = policySourceAncestry(repoRoot, relPath, operations);
  if (ancestry !== "clear") return { state: ancestry === "not-directory" ? "not-file" : ancestry, text: "" };
  let stat: ReturnType<PolicySourceOperations["lstat"]>;
  try {
    stat = operations.lstat(path);
  } catch (error) {
    return { state: policySourceUnavailableState(error), text: "" };
  }
  if (stat.isSymbolicLink) return { state: "symlink", text: "" };
  if (!stat.isFile) return { state: "not-file", text: "" };
  try {
    return { state: "file", text: operations.readFile(path) };
  } catch (error) {
    return { state: policySourceUnavailableState(error), text: "" };
  }
}

export function policyReaddirSafe(repoRoot: string, relDir: string): { name: string; isDirectory: boolean }[] {
  const source = policySourceDirectory(repoRoot, relDir);
  if (source.state === "missing") return [];
  if (source.state !== "directory") throw new Error(`Policy source directory ${join(repoRoot, relDir)} is ${source.state}.`);
  return source.entries.filter((entry) => !POLICY_SKIP_DIRS.has(entry.name)).map((entry) => ({ name: entry.name, isDirectory: entry.isDirectory }));
}

export function policyReadFileSafe(repoRoot: string, ...parts: string[]): string {
  const relPath = join(...parts),
    source = policySourceText(repoRoot, relPath);
  if (source.state === "missing") return "";
  if (source.state !== "file") throw new Error(`Policy source file ${join(repoRoot, relPath)} is ${source.state}.`);
  return source.text;
}
