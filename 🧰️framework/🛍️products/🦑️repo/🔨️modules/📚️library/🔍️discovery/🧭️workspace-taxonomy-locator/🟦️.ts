import { ephemeralBox } from "@semio-tech/framework/transient";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";

const __dirname = dirname(dirname(fileURLToPath(import.meta.url)));

export const cachedTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedTaxonomy", undefined);

export const cachedCatalogTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedCatalogTaxonomy", undefined);

/** 🧭️ No-follow workspace authority for the taxonomy named by root project metadata. */
export interface WorkspaceTaxonomyAuthority {
  readonly workspaceRoot: string;
  readonly manifestPath: string;
  readonly relativePath: string;
  readonly taxonomyPath: string;
}

function noFollowStat(path: string, expectation: "directory" | "file", subject: string): void {
  let stat: ReturnType<typeof lstatSync>;
  try {
    stat = lstatSync(path);
  } catch {
    throw new Error(`Workspace taxonomy ${subject} is missing at ${JSON.stringify(path)}.`);
  }
  if (stat.isSymbolicLink()) throw new Error(`Workspace taxonomy ${subject} must not be a symlink at ${JSON.stringify(path)}.`);
  if (expectation === "directory" ? !stat.isDirectory() : !stat.isFile()) throw new Error(`Workspace taxonomy ${subject} must be a ${expectation} at ${JSON.stringify(path)}.`);
}

function noFollowNodeKind(path: string): "missing" | "file" | "directory" | "symlink" | "other" {
  try {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink()) return "symlink";
    if (stat.isFile()) return "file";
    if (stat.isDirectory()) return "directory";
    return "other";
  } catch {
    return "missing";
  }
}

/** 🧭️ Lexically verifies one workspace authority path before resolution. */
export function workspaceAuthorityPath(path: string, subject: string): string {
  const segments = path.split(/[\\/]/u);
  if (path.includes("\0")) throw new Error(`Workspace taxonomy ${subject} contains a NUL path character at ${JSON.stringify(path)}.`);
  if (/^[A-Za-z]:(?:$|[^\\/])/u.test(path)) throw new Error(`Workspace taxonomy ${subject} contains a drive-relative path at ${JSON.stringify(path)}.`);
  if (segments.some((segment) => segment.toLowerCase() === "compose")) throw new Error(`Workspace taxonomy ${subject} enters excluded compose content at ${JSON.stringify(path)}.`);
  if (segments.includes(".") || segments.includes("..")) throw new Error(`Workspace taxonomy ${subject} contains a dot or parent path segment at ${JSON.stringify(path)}.`);
  return resolve(path);
}

/** 🛡️ Requires each directory from the filesystem root to be real and no-follow. */
export function noFollowDirectoryAncestry(path: string, subject: string): void {
  const ancestry: string[] = [];
  for (let candidate = path; ; candidate = dirname(candidate)) {
    ancestry.push(candidate);
    if (dirname(candidate) === candidate) break;
  }
  for (const candidate of ancestry.reverse()) noFollowStat(candidate, "directory", subject);
}

function workspaceTaxonomyLocator(manifest: unknown): string {
  const metadata = typeof manifest === "object" && manifest !== null && !Array.isArray(manifest) ? (manifest as { metadata?: unknown }).metadata : undefined;
  const semio = typeof metadata === "object" && metadata !== null && !Array.isArray(metadata) ? (metadata as { semio?: unknown }).semio : undefined;
  const locator = typeof semio === "object" && semio !== null && !Array.isArray(semio) ? (semio as { taxonomy?: unknown }).taxonomy : undefined;
  if (typeof locator !== "string") throw new Error("Workspace taxonomy locator metadata.semio.taxonomy must be a string.");
  if (locator.length === 0 || locator.includes("\0") || locator.includes("\\") || locator.startsWith("/") || /^[A-Za-z]:/u.test(locator)) throw new Error(`Workspace taxonomy locator ${JSON.stringify(locator)} must be a safe repository-relative slash path.`);
  const segments = locator.split("/");
  if (segments.some((segment) => segment.length === 0 || segment === "." || segment === "..")) throw new Error(`Workspace taxonomy locator ${JSON.stringify(locator)} contains an unsafe path segment.`);
  if (segments.some((segment) => segment.toLowerCase() === "compose")) throw new Error(`Workspace taxonomy locator ${JSON.stringify(locator)} enters excluded compose content.`);
  return locator;
}

/** 🧭️ Resolves root metadata.semio.taxonomy only after lexical and no-follow checks. */
export function resolveWorkspaceTaxonomyAuthority(workspaceRoot: string): WorkspaceTaxonomyAuthority {
  const root = workspaceAuthorityPath(workspaceRoot, "root");
  noFollowDirectoryAncestry(root, "workspace root ancestry");
  const manifestPath = join(root, "📋️project.json");
  noFollowStat(manifestPath, "file", "project manifest");
  let manifest: unknown;
  try {
    manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
  } catch (error) {
    throw new Error(`Workspace taxonomy project manifest is not valid JSON: ${error instanceof Error ? error.message : String(error)}`);
  }
  const relativePath = workspaceTaxonomyLocator(manifest);
  const segments = relativePath.split("/");
  let current = root;
  for (const [index, segment] of segments.entries()) {
    current = join(current, segment);
    noFollowStat(current, index === segments.length - 1 ? "file" : "directory", index === segments.length - 1 ? "taxonomy file" : "taxonomy path component");
  }
  return { workspaceRoot: root, manifestPath, relativePath, taxonomyPath: current };
}

export function readTaxonomyUnchecked(): Taxonomy {
  const authority = resolveWorkspaceTaxonomyAuthorityFromDirectory(__dirname);
  return JSON.parse(readFileSync(authority.taxonomyPath, "utf8")) as Taxonomy;
}

export function taxonomyWorkspaceRoot(): string | null {
  return taxonomyWorkspaceRootFromDirectory(__dirname);
}

function taxonomyWorkspaceRootFromDirectory(startDirectory: string): string | null {
  let start: string;
  try {
    start = workspaceAuthorityPath(startDirectory, "start directory");
    noFollowDirectoryAncestry(start, "workspace start directory ancestry");
  } catch {
    return null;
  }
  for (let candidate = start; dirname(candidate) !== candidate; candidate = dirname(candidate)) {
    const projectPath = join(candidate, "📋️project.json");
    const projectKind = noFollowNodeKind(projectPath);
    if (projectKind === "missing") continue;
    if (projectKind === "symlink") throw new Error(`Workspace taxonomy project manifest must not be a symlink at ${JSON.stringify(projectPath)}.`);
    if (projectKind !== "file") throw new Error(`Workspace taxonomy project manifest must be a file at ${JSON.stringify(projectPath)}.`);
    const project = JSON.parse(readFileSync(projectPath, "utf8")) as { name?: unknown; metadata?: { semio?: { taxonomy?: unknown } } };
    if (typeof project.name === "string" && project.name !== "workspace" && project.metadata?.semio?.taxonomy === undefined) continue;
    return candidate;
  }
  return null;
}

/** 🧭️ Finds the declared portable workspace from a no-follow, non-compose discovery directory. */
export function resolveWorkspaceTaxonomyAuthorityFromDirectory(startDirectory: string): WorkspaceTaxonomyAuthority {
  const start = workspaceAuthorityPath(startDirectory, "start directory");
  noFollowDirectoryAncestry(start, "workspace start directory ancestry");
  const workspaceRoot = taxonomyWorkspaceRootFromDirectory(start);
  if (!workspaceRoot) throw new Error("Workspace taxonomy root declaring metadata.semio.taxonomy could not be resolved.");
  return resolveWorkspaceTaxonomyAuthority(workspaceRoot);
}
