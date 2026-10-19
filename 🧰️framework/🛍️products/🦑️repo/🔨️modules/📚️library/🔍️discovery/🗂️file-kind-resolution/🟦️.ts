import { readFile as readFileAsync, readdir as readdirAsync } from "node:fs/promises";
import { basename, dirname, join } from "node:path";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { createTaxonomyPathMatcher } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { type TaxonomyImplementationCensusOptions, type TaxonomyImplementationCensusProgress, type TaxonomyImplementationFinding, cargoPackageRootBuildScriptPath, implementationPackageLocation } from "../📄️implementation-findings/🟦️.ts";
import { canonicalFilenamesForKind, canonicalPrimaryFilenameForKind, fileKindIdForSourcePath, loadCatalogTaxonomy, loadTaxonomy, scopedFileKindIdForSourcePath, semanticDirectoryKindId } from "../🔐️taxonomy-authority/🟦️.ts";
import { taxonomyRelativePathIsExcluded } from "../🔎️rust-source-identities/🟦️.ts";
import { createFixedContractResolver } from "../⚖️fixed-contract-resolution/🟦️.ts";

/** 🪆️ Resolves one logical subset id to its exact owner-scoped physical directory identity. */
export function subsetDirectoryNameForId(subsetsOwnerPath: string, subsetId: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const owner = subsetsOwnerPath.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC");
  const overrides = taxonomy.subsetDirectoryOverrides?.[owner];
  if (overrides) return overrides[subsetId] ?? null;
  const anyId = taxonomy.subsetAnyId ?? "*";
  if (subsetId === anyId) return taxonomy.subsetAnyDirName ?? "✳️any";
  const slug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
  return slug.test(subsetId) ? `${taxonomy.subsetDirPrefix ?? "✳️"}${subsetId}` : null;
}

/** 🪆️ Resolves one physical subset directory identity back to its logical owner-scoped id. */
export function subsetIdForDirectoryName(subsetsOwnerPath: string, directoryName: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const owner = subsetsOwnerPath.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC");
  const overrides = taxonomy.subsetDirectoryOverrides?.[owner];
  if (overrides) return Object.entries(overrides).find(([, name]) => name === directoryName)?.[0] ?? null;
  const anyId = taxonomy.subsetAnyId ?? "*";
  if (directoryName === (taxonomy.subsetAnyDirName ?? "✳️any")) return anyId;
  const prefix = taxonomy.subsetDirPrefix ?? "✳️";
  if (!directoryName.startsWith(prefix)) return null;
  const id = directoryName.slice(prefix.length);
  const slug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
  return slug.test(id) ? id : null;
}

/** 📇️ Resolves a semantic collection's manifest by exact owner, never by ancestor or suffix fallback. */
export function semanticManifestFilenameForCollection(collectionPath: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const owner = collectionPath.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC");
  return taxonomy.semanticManifestFilenameOverrides?.[owner] ?? canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId, taxonomy);
}

/** 🍃️ Renders a source's exact longest-chain format; this does not authorize its owner or move. */
export function canonicalLeafFilenameForSourcePath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const kindId = fileKindIdForSourcePath(path, taxonomy);
  if (!kindId) return null;
  const kind = taxonomy.fileKinds[kindId]!;
  const filename = path.replaceAll("\\", "/").split("/").pop()!.toLowerCase();
  const extension = [...kind.extensionChains].sort((left, right) => right.length - left.length).find((chain) => filename.endsWith(chain));
  return extension ? `${kind.emoji}${extension}` : null;
}

const implementationContractResolvers = new WeakMap<Taxonomy, ReturnType<typeof createFixedContractResolver>>();

function implementationContractResolver(taxonomy: Taxonomy): ReturnType<typeof createFixedContractResolver> {
  const retained = implementationContractResolvers.get(taxonomy);
  if (retained) return retained;
  const created = createFixedContractResolver(taxonomy);
  implementationContractResolvers.set(taxonomy, created);
  return created;
}

/** 🧫️ Reports whether a normalized path lies inside a fixtures directory: fixtures are data a test reads, never implementation, so the leaf gate does not apply. */
function pathIsFixtureData(normalizedPath: string, taxonomy: Taxonomy): boolean {
  const fixtures = taxonomy.testFixturesDirName.normalize(taxonomy.unicodeNormalization.form);
  return normalizedPath.split("/").slice(0, -1).includes(fixtures);
}

function implementationLeafBasenameFindingResolved(
  path: string,
  taxonomy: Taxonomy,
  resolver: ReturnType<typeof createFixedContractResolver>,
  parentDirectoryKindId?: string,
  parentFixedDirectoryContractIds?: readonly string[],
  siblingFixedFilenameContractIds?: readonly string[],
  authorizedCargoBuildScriptPaths: ReadonlySet<string> = new Set(),
): TaxonomyImplementationFinding | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  if (pathIsFixtureData(normalized, taxonomy)) return null;
  const scopedKindId = scopedFileKindIdForSourcePath(normalized, taxonomy, { parentDirectoryKindId });
  const fileKindId = scopedKindId ? `scoped:${scopedKindId}` : fileKindIdForSourcePath(normalized, taxonomy);
  if (!taxonomyFileKindIsImplementation(fileKindId, taxonomy)) return null;
  const packageLocation = implementationPackageLocation(normalized, taxonomy);
  const fixed = resolver.filenameIdsForPath(normalized, {
    packageRoot: packageLocation?.packageRoot === dirname(normalized),
    ecosystemId: packageLocation?.ecosystemId,
    parentDirectoryKindId,
    parentFixedDirectoryContractIds,
    siblingFixedFilenameContractIds,
  });
  if (fixed.some((contractId) => contractId !== "cargo-build-script" || authorizedCargoBuildScriptPaths.has(normalized))) return null;
  const scopedKind = scopedKindId ? taxonomy.scopedFileKinds[scopedKindId] : undefined;
  const scopedExtension = scopedKind?.extensionChains.find((extension) => normalized.toLocaleLowerCase("und").endsWith(extension.toLocaleLowerCase("und")));
  const expectedBasename = scopedKind && scopedExtension ? `${scopedKind.emoji}${scopedExtension}` : canonicalLeafFilenameForSourcePath(normalized, taxonomy)!;
  const actualBasename = basename(normalized);
  return actualBasename === expectedBasename ? null : { breachId: "taxonomy/kind-only-basename", path: normalized, fileKindId, actualBasename, expectedBasename, exemptionAuthorityId: null };
}

/** 🪶️ Classifies implementation kinds from the closed taxonomy policy. */
export function taxonomyFileKindIsImplementation(fileKindId: string | null | undefined, taxonomy: Taxonomy = loadCatalogTaxonomy()): boolean {
  const scopedId = fileKindId?.startsWith("scoped:") ? fileKindId.slice("scoped:".length) : undefined;
  const fileKind = scopedId ? taxonomy.scopedFileKinds[scopedId] : fileKindId ? taxonomy.fileKinds[fileKindId] : undefined;
  return Boolean(fileKind && ((taxonomy.implementationLeafPolicy.roles as readonly string[]).includes(fileKind.role) || taxonomy.implementationLeafPolicy.fileKindIds.includes(fileKindId!)));
}

/** 🍂 Enforces one registered implementation leaf after deriving fixed-contract scope from its actual path. */
export function implementationLeafBasenameFinding(path: string, taxonomy: Taxonomy = loadCatalogTaxonomy()): TaxonomyImplementationFinding | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const parent = dirname(normalized);
  const resolver = implementationContractResolver(taxonomy);
  const parentDirectoryKindId = parent === "." ? undefined : semanticDirectoryKindId(basename(parent), taxonomy) ?? undefined;
  const grandparent = parent === "." ? "." : dirname(parent);
  const parentFixedDirectoryContractIds = parent === "." ? [] : resolver.directoryIdsForPath(parent, { parentDirectoryKindId: grandparent === "." ? undefined : semanticDirectoryKindId(basename(grandparent), taxonomy) ?? undefined });
  return implementationLeafBasenameFindingResolved(normalized, taxonomy, resolver, parentDirectoryKindId, parentFixedDirectoryContractIds);
}

/** 🎯 Rejects the inverse `<packages>/<language>/<targets>` ownership relation at its boundary. */
export function targetInsidePackageBoundaryFinding(path: string, taxonomy: Taxonomy = loadCatalogTaxonomy()): TaxonomyImplementationFinding | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "").normalize(taxonomy.unicodeNormalization.form);
  const parts = normalized.split("/");
  const packageIndex = parts.indexOf(taxonomy.packagesDirName);
  if (packageIndex < 0 || !taxonomy.ecosystems[parts[packageIndex + 1] ?? ""] || parts[packageIndex + 2] !== taxonomy.targetsDirName) return null;
  return { breachId: "taxonomy/target-inside-package-boundary", path: parts.slice(0, packageIndex + 3).join("/"), fileKindId: null, actualBasename: null, expectedBasename: null, exemptionAuthorityId: null };
}

/** 🌲 Performs the focused cancellable no-follow implementation-leaf and package-topology census. */
export async function taxonomyImplementationFilesystemFindings(repoRoot: string, taxonomy: Taxonomy = loadCatalogTaxonomy(), options: TaxonomyImplementationCensusOptions = {}): Promise<TaxonomyImplementationFinding[]> {
  const paths: { readonly path: string; readonly nodeKind: "directory" | "file" }[] = [];
  const yieldEvery = options.yieldEvery ?? 256;
  if (!Number.isSafeInteger(yieldEvery) || yieldEvery < 1) throw new TypeError("Taxonomy implementation census yieldEvery must be a positive safe integer.");
  let pathsVisited = 0, pathsClassified = 0, vanishedDirectories = 0, findingCount = 0, checkpoints = 0;
  const pathMatcher = createTaxonomyPathMatcher();
  const ignored = (path: string): boolean => taxonomy.implementationLeafPolicy.ignoredPathPatterns.some((pattern) => pathMatcher.matches(path, pattern));
  const abort = (): void => {
    if (!options.signal?.aborted) return;
    const error = new Error("Taxonomy implementation census was cancelled.");
    error.name = "AbortError";
    throw error;
  };
  const progress = async (phase: TaxonomyImplementationCensusProgress["phase"], force = false): Promise<void> => {
    abort();
    checkpoints += 1;
    if (force || checkpoints % yieldEvery === 0) {
      options.onProgress?.({ phase, pathsVisited, pathsClassified, findings: findingCount, vanishedDirectories });
      await new Promise<void>((resolve) => setImmediate(resolve));
      abort();
    }
  };
  const entries = async (absolute: string) => {
    for (let attempt = 0; ; attempt += 1) try {
      return await readdirAsync(absolute, { withFileTypes: true });
    } catch (error) {
      const code = (error as NodeJS.ErrnoException).code;
      if (code === "EINTR" && attempt < 2) continue;
      if (code === "ENOENT") {
        vanishedDirectories += 1;
        await progress("walk", true);
        return [];
      }
      throw error;
    }
  };
  const walk = async (absolute: string, relativePath: string): Promise<void> => {
    if (relativePath && taxonomyRelativePathIsExcluded(relativePath, taxonomy)) return;
    for (const entry of (await entries(absolute)).sort((left, right) => Buffer.from(left.name).compare(Buffer.from(right.name)))) {
      const path = relativePath ? `${relativePath}/${entry.name}` : entry.name;
      if (taxonomyRelativePathIsExcluded(path, taxonomy)) continue;
      if (entry.isDirectory()) {
        if (ignored(path) || options.admitted?.(path, "directory") === false) continue;
        paths.push({ path, nodeKind: "directory" });
        pathsVisited += 1;
        await progress("walk");
        await walk(join(absolute, entry.name), path);
      } else if (entry.isFile()) {
        if (options.admitted?.(path, "file") === false) continue;
        paths.push({ path, nodeKind: "file" });
        pathsVisited += 1;
        await progress("walk");
      }
    }
  };
  await walk(repoRoot, "");
  await progress("walk", true);
  const resolver = implementationContractResolver(taxonomy);
  const files = new Set(paths.filter((entry) => entry.nodeKind === "file").map((entry) => entry.path));
  const authorizedCargoBuildScriptPaths = new Set<string>();
  for (const manifest of [...files].filter((path) => basename(path) === "Cargo.toml")) {
    const location = implementationPackageLocation(manifest, taxonomy);
    if (!location || location.ecosystemId !== "🦀️rust" || dirname(manifest) !== location.packageRoot) continue;
    let source: string;
    try {
      source = await readFileAsync(join(repoRoot, manifest), "utf8");
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") continue;
      throw error;
    }
    if (cargoPackageRootBuildScriptPath(source) !== "build.rs") continue;
    const buildScript = `${location.packageRoot}/build.rs`;
    if (files.has(buildScript)) authorizedCargoBuildScriptPaths.add(buildScript);
  }
  const parentFixed = new Map<string, readonly string[]>();
  for (const { path } of paths) {
    const parent = dirname(path) === "." ? "" : dirname(path);
    if (parentFixed.has(parent)) continue;
    const grandparent = dirname(parent);
    const parentDirectoryKindId = parent && grandparent !== "." ? semanticDirectoryKindId(basename(grandparent), taxonomy) ?? undefined : undefined;
    parentFixed.set(parent, resolver.directoryIdsForPath(parent, { parentDirectoryKindId }));
  }
  const preliminaryFixed = new Map<string, readonly string[]>();
  for (const { path, nodeKind } of paths) {
    if (nodeKind !== "file") continue;
    const parent = dirname(path) === "." ? "" : dirname(path);
    const packageLocation = implementationPackageLocation(path, taxonomy);
    preliminaryFixed.set(path, resolver.filenameIdsForPath(path, {
      packageRoot: packageLocation?.packageRoot === parent,
      ecosystemId: packageLocation?.ecosystemId,
      parentDirectoryKindId: parent ? semanticDirectoryKindId(basename(parent), taxonomy) ?? undefined : undefined,
      parentFixedDirectoryContractIds: parentFixed.get(parent),
    }));
  }
  const siblingFixed = new Map<string, readonly string[]>();
  for (const { path, nodeKind } of paths) {
    if (nodeKind !== "file") continue;
    const parent = dirname(path) === "." ? "" : dirname(path);
    siblingFixed.set(parent, [...new Set([...(siblingFixed.get(parent) ?? []), ...(preliminaryFixed.get(path) ?? [])])]);
  }
  const findings: TaxonomyImplementationFinding[] = [];
  for (const { path, nodeKind } of paths) {
    pathsClassified += 1;
    const topology = targetInsidePackageBoundaryFinding(path, taxonomy);
    if (topology) { findings.push(topology); findingCount += 1; }
    if (nodeKind !== "file") { await progress("classify"); continue; }
    const parent = dirname(path) === "." ? "" : dirname(path);
    const finding = implementationLeafBasenameFindingResolved(path, taxonomy, resolver, parent ? semanticDirectoryKindId(basename(parent), taxonomy) ?? undefined : undefined, parentFixed.get(parent), siblingFixed.get(parent), authorizedCargoBuildScriptPaths);
    if (finding) { findings.push(finding); findingCount += 1; }
    await progress("classify");
  }
  const unique = [...new Map(findings.map((finding) => [`${finding.path}\0${finding.breachId}`, finding])).values()].sort((left, right) => Buffer.from(`${left.path}\0${left.breachId}`).compare(Buffer.from(`${right.path}\0${right.breachId}`)));
  findingCount = unique.length;
  await progress("complete", true);
  return unique;
}

/** 🧬️ Renders the default schema location for a newly authored mutation, not existing descriptor authority. */
export function mutationPayloadSchemaRelativePath(taxonomy: Taxonomy = loadTaxonomy()): string {
  const location = taxonomy.mutationPayloadSchemaLocation;
  return `${location.directoryName}/${canonicalPrimaryFilenameForKind(location.fileKindId, taxonomy)}`;
}

/** 🧷️ Returns a semantic leaf filename using the file kind's schema-ordered primary extension. */
export function canonicalStemmedFilenameForKind(kindId: string, stem: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const kind = taxonomy.fileKinds[kindId];
  const extension = kind?.extensionChains[0];
  if (!kind || !extension) throw new Error(`File kind ${JSON.stringify(kindId)} must declare a primary extension chain.`);
  if (!stem || /[\\/]/u.test(stem)) throw new Error(`Filename stem ${JSON.stringify(stem)} must be one non-empty path segment.`);
  return `${kind.emoji}${stem}${extension}`;
}

/** 🧭️ Resolves an exact canonical kind-only filename to its registered file-kind identifier. */
export function fileKindIdForFilename(filename: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const normalized = filename.normalize(taxonomy.unicodeNormalization.form);
  const matches = Object.entries(taxonomy.fileKinds).filter(([kindId]) => canonicalFilenamesForKind(kindId, taxonomy).includes(normalized));
  return matches.length === 1 ? matches[0]![0] : null;
}
