/** 🏗️ Canonical 📁️input source service. */
import { resolve, relative, sep, join, isAbsolute, posix, parse } from "node:path";
import { type Stats, lstatSync } from "node:fs";
import { createHash } from "node:crypto";

export const LEXICAL_OPAQUE_ROOTS = ["compose", "temp/compose"] as const;

export function sha256(value: string | Uint8Array): string {
  return createHash("sha256").update(value).digest("hex");
}

export function assertNoFollowAncestors(repoRoot: string, target: string, label: string, rejectLeafSymlink = false): void {
  const root = resolve(repoRoot);
  const relativeTarget = relative(root, target);
  const segments = relativeTarget.split(sep).filter(Boolean);
  let current = root;
  const end = segments.length - (rejectLeafSymlink ? 0 : 1);
  for (let index = 0; index < end; index++) {
    current = join(current, segments[index]);
    const stat = lstatOrNull(current);
    const leaf = rejectLeafSymlink && index === segments.length - 1;
    if (stat?.isSymbolicLink() || (!leaf && stat && !stat.isDirectory())) throw new Error(`${label} has a non-directory or symlink ancestor: ${segments.slice(0, index + 1).join("/")}`);
  }
}

export function assertLexicalInputOutsideOpaque(repoRoot: string, path: string, label: string, rejectLeafSymlink = false): string {
  if ([repoRoot, path].some((value) => value.replaceAll("\\", "/").split("/").includes(".."))) throw new Error(`${label} must not contain parent traversal`);
  const root = resolve(repoRoot);
  const target = isAbsolute(path) ? resolve(path) : resolve(root, path);
  const nativeRelative = relative(root, target);
  if (nativeRelative === ".." || nativeRelative.startsWith(`..${sep}`) || nativeRelative.startsWith("..") || nativeRelative.startsWith("..\\") || isAbsolute(nativeRelative)) throw new Error(`${label} must be repository-local`);
  const repositoryRelative = posix.normalize(nativeRelative.replaceAll("\\", "/"));
  if (LEXICAL_OPAQUE_ROOTS.some((opaque) => repositoryRelative === opaque || repositoryRelative.startsWith(`${opaque}/`))) throw new Error(`${label} is inside an opaque path: ${repositoryRelative}`);
  assertNoFollowAncestors(root, target, label, rejectLeafSymlink);
  return target;
}

export function lstatOrNull(path: string): Stats | null {
  try {
    return lstatSync(path);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
}

/** 🧱️ Signals a linked or non-directory component in an input root. */
export class UnsafeDirectoryAncestorError extends Error {}

/** 🔗️ Observes every physical directory component of an absolute input root without following links. */
export function noFollowDirectoryChain(repoRoot: string): readonly { readonly path: string; readonly stat: Stats }[] {
  const root = parse(repoRoot).root;
  const paths = [root];
  for (const segment of repoRoot.slice(root.length).split(sep).filter(Boolean)) paths.push(join(paths[paths.length - 1], segment));
  return paths.map((path) => {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new UnsafeDirectoryAncestorError(`Input root has unsafe ancestry: ${path}`);
    return { path, stat };
  });
}

/** 📸️ Verifies captured directory identities without treating unrelated child writes as input changes. */
export function verifyNoFollowDirectoryChain(ancestors: ReturnType<typeof noFollowDirectoryChain>): void {
  for (const ancestor of ancestors) {
    const current = lstatOrNull(ancestor.path);
    if (!current || current.isSymbolicLink() || !current.isDirectory() || current.dev !== ancestor.stat.dev || current.ino !== ancestor.stat.ino || current.mode !== ancestor.stat.mode) throw new Error(`Input directory ancestry changed during observation: ${ancestor.path}`);
  }
}
