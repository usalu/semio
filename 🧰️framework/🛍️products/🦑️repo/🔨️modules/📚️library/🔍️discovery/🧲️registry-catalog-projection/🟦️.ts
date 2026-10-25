import { basename, dirname, relative, resolve } from "node:path";
import { renderOwnerPublications } from "../../📇️catalog/📣️publication/🟦️.ts";
import { declaredPlaygroundHostInputPathsV1, nativeHostTextSourceViewV1 } from "../../🎮️playground/🖥️native-host/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { type DiscoveredPackage, declaredComponentKind, isDiscoverySkipDirectory } from "../🪧️owner-markers/🟦️.ts";
import { type GeneratorInputProjection, type RegistryCatalogInputView, type RegistryCatalogProjection, type RegistryCompilerInputDependencies, type RegistryCompilerInputRole, parseGeneratorInputProjection, parseRegistryCatalogProjection, parseRegistrySourceRoots, registryCatalogOwnerIndex, registryCompilerInputDependencies } from "../📇️registry-catalog-inputs/🟦️.ts";
import { canonicalPrimaryFilenameForKind, loadTaxonomy, registryCatalogInputView } from "../🔐️taxonomy-authority/🟦️.ts";
import { scanRepo } from "../🔭️repository-scan/🟦️.ts";
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🌳️ Applies only authorized leaf operations and their required ancestor membership in memory. */
export function registryCatalogProjectedInputView(repoRoot: string, taxonomy: Taxonomy, projection: RegistryCatalogProjection, base: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): RegistryCatalogInputView {
  return generatorProjectedInputView(repoRoot, taxonomy, parseRegistryCatalogProjection(JSON.stringify(projection), taxonomy), base);
}

/** 🌲️ Shares no-follow source mapping and projected parent membership across exact preview producers. */
export function generatorProjectedInputView(repoRoot: string, taxonomy: Taxonomy, projection: GeneratorInputProjection, base: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): RegistryCatalogInputView {
  const checked = parseGeneratorInputProjection(JSON.stringify(projection), taxonomy, projection.contractId);
  const sources = new Set(checked.moves.map((move) => move.sourcePath));
  const removed = new Set(checked.removals);
  const moves = new Map(checked.moves.map((move) => [move.destinationPath, move]));
  const edits = new Map(checked.edits.map((edit) => [edit.path, Buffer.from(edit.bytesBase64, "base64")]));
  const createdDirectories = new Set<string>();
  for (const move of checked.moves) {
    if (base.kind(move.sourcePath) !== move.nodeKind) throw new Error(`Registry projected source drifted: ${move.sourcePath}`);
    if (base.kind(move.destinationPath) !== null && !sources.has(move.destinationPath)) throw new Error(`Registry projected destination is occupied: ${move.destinationPath}`);
    for (let parent = dirname(move.destinationPath); parent !== "."; parent = dirname(parent)) {
      const kind = base.kind(parent);
      if (kind && kind !== "directory") throw new Error(`Registry projected ancestor is not a directory: ${parent}`);
      if (!kind) createdDirectories.add(parent);
    }
  }
  for (const path of removed) if (base.kind(path) === null) throw new Error(`Registry projected removal drifted: ${path}`);
  const kind = (path: string): "file" | "directory" | "symlink" | null => {
    if (moves.has(path)) return moves.get(path)!.nodeKind;
    if (sources.has(path) || removed.has(path)) return null;
    if (createdDirectories.has(path)) return "directory";
    return base.kind(path);
  };
  for (const path of edits.keys()) if (kind(path) !== "file") throw new Error(`Registry projected edit is not a regular post-state file: ${path}`);
  return {
    kind,
    entries(path) {
      if (kind(path) !== "directory") return [];
      const rows = new Map<string, "file" | "directory" | "symlink">();
      if (!createdDirectories.has(path)) for (const entry of base.entries(path)) {
        const child = path ? `${path}/${entry.name}` : entry.name;
        const childKind = kind(child);
        if (childKind) rows.set(entry.name, childKind);
      }
      for (const child of [...moves.keys(), ...createdDirectories]) if ((dirname(child) === "." ? "" : dirname(child)) === path) rows.set(basename(child), kind(child)!);
      return [...rows].map(([name, nodeKind]) => ({ name, nodeKind }));
    },
    readText(path) {
      if (kind(path) !== "file") throw new Error(`Registry projected content is missing or a symlink: ${path}`);
      return edits.has(path) ? new TextDecoder("utf-8", { fatal: true }).decode(edits.get(path)) : base.readText(moves.get(path)?.sourcePath ?? path);
    },
    readBytes(path) {
      if (kind(path) !== "file") throw new Error(`Registry projected content is missing or a symlink: ${path}`);
      return edits.get(path)?.slice() ?? base.readBytes(moves.get(path)?.sourcePath ?? path);
    },
  };
}

/** 🎯️ Conservative lexical admission before the expensive shared catalog membership scan. */
export function registryCatalogPathMayAffect(path: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const authority = taxonomy.generatorContracts["plugin-registry"]?.inputDiscovery;
  if (!authority || Object.values(taxonomy.pathExclusions).some((entry) => { const prefix = entry.path.replace(/\/+$/u, ""); return path === prefix || path.startsWith(prefix + "/"); })) return false;
  const roots = [...authority.implementationEntryPaths, ...Object.values(authority.workspaceImports).flatMap((entry) => [entry.entryPath, entry.manifestPath])].map((path) => path.split("/")[0]);
  if (roots.includes(path.split("/")[0])) return true;
  return !path.split("/").some((segment) => segment.startsWith(".") || isDiscoverySkipDirectory(segment));
}

/** 📦️ Catalog discovery deliberately omits source-role, cache-tag and package-purity diagnostics. */
export function discoverCatalogPackages(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy(), view?: RegistryCatalogInputView): DiscoveredPackage[] {
  const ownerIndex = view === undefined ? registryCatalogOwnerIndex(repoRoot, taxonomy.packagesDirName) : undefined;
  return [...scanRepo(repoRoot, taxonomy, { view: view ?? registryCatalogInputView(repoRoot, taxonomy), inputs: new Set<string>(), ...(ownerIndex ? { ownerIndex } : {}) }).packages];
}

/** 🛂️ Admits explicit physical source directories within the workspace without following any symlink. */
export function registryArtifactSourceRoots(repoRoot: string, cratePath: string, view: RegistryCatalogInputView = registryCatalogInputView(repoRoot), inputs = new Set<string>()): readonly string[] {
  const manifest = `${cratePath}/Cargo.toml`;
  if (view.kind(manifest) !== "file") throw new Error(`Registry source manifest is not a regular file: ${manifest}`);
  inputs.add(manifest);
  return parseRegistrySourceRoots(view.readText(manifest)).map((path) => {
    const root = relative(resolve(repoRoot), resolve(repoRoot, cratePath, path)).replaceAll("\\", "/");
    if (!root || root === ".." || root.startsWith("../")) throw new Error(`Registry source root escapes the workspace: ${path}`);
    const parts = root.split("/");
    for (let length = 1; length <= parts.length; length += 1) {
      const ancestor = parts.slice(0, length).join("/"), kind = view.kind(ancestor);
      if (kind !== "directory") throw new Error(`Registry source root requires nofollow directories: ${ancestor}`);
      inputs.add(ancestor);
    }
    return root;
  });
}

/** 📚️ Example IDs are directory membership with one schema-owned primary Rust leaf, scanned at artifact, subset and surface level. */
export function registryExampleCatalog(repoRoot: string, cratePath: string, taxonomy: Taxonomy = loadTaxonomy(), view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy), inputs = new Set<string>()): string[] {
  const authority = taxonomy.generatorContracts["plugin-registry"]?.inputDiscovery;
  if (!authority || authority.kind !== "registry-catalog") throw new Error("Registry catalog input discovery is not declared.");
  const leaf = canonicalPrimaryFilenameForKind(authority.exampleFileKindId, taxonomy);
  const exists = (path: string): boolean => {
    const kind = view.kind(path);
    if (kind === "symlink") throw new Error(`Registry catalog example input is a symlink: ${path}`);
    if (kind) inputs.add(path);
    return kind !== null;
  };
  const dirs = (path: string): string[] => exists(path) ? view.entries(path).filter((entry) => entry.nodeKind === "directory").map((entry) => entry.name) : [];
  const ids = new Set<string>();
  const slugPattern = new RegExp(taxonomy.exampleSlugPattern, "u");
  const slugs = (path: string): void => {
    for (const slug of dirs(path)) if (slugPattern.test(slug) && !taxonomy.forbiddenExampleSlugs.includes(slug) && exists(`${path}/${slug}/${leaf}`)) ids.add(slug);
  };
  for (const artifacts of registryArtifactSourceRoots(repoRoot, cratePath, view, inputs)) for (const artifact of dirs(artifacts)) {
    const artifactPath = `${artifacts}/${artifact}`;
    slugs(`${artifactPath}/${authority.exampleDirectoryName}`);
    const standards = `${artifactPath}/${taxonomy.standardsDirName}`;
    for (const standard of dirs(standards)) {
      const subsets = `${standards}/${standard}/${taxonomy.subsetsDirName}`;
      for (const subset of dirs(subsets)) {
        slugs(`${subsets}/${subset}/${authority.exampleDirectoryName}`);
        for (const role of taxonomy.surfaceRoles) {
          const surface = `${subsets}/${subset}/${taxonomy.surfaceDirNames[role]}`;
          if (exists(surface)) slugs(`${surface}/${authority.exampleDirectoryName}`);
        }
      }
    }
  }
  return [...ids].sort();
}

/** 📇️ Exact content inputs plus positive membership witnesses; ignored files are not excluded. */
export function registryCatalogInputPaths(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy(), view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): readonly string[] {
  const authority = taxonomy.generatorContracts["plugin-registry"]?.inputDiscovery;
  if (!authority || authority.kind !== "registry-catalog") throw new Error("Registry catalog input discovery is not declared.");
  const inputs = new Set<string>();
  const packages = scanRepo(repoRoot, taxonomy, { view, inputs }).packages;
  for (const pkg of packages) if (pkg.lang === "🦀️rust") {
    for (const path of declaredPlaygroundHostInputPathsV1(view.readText(pkg.manifestPath), nativeHostTextSourceViewV1(view))) inputs.add(path);
  }
  for (const pkg of packages) if (pkg.lang === "🦀️rust" && declaredComponentKind(view.readText(pkg.manifestPath)) !== undefined) {
    const descriptor = relative(repoRoot, resolve(repoRoot, pkg.packageRel, authority.descriptorRelativePath)).replaceAll("\\", "/");
    const kind = view.kind(descriptor);
    if (kind === "symlink") throw new Error(`Registry descriptor is a symlink: ${descriptor}`);
    if (kind) inputs.add(descriptor);
    registryExampleCatalog(repoRoot, pkg.packageRel, taxonomy, view, inputs);
  }
  for (const path of renderOwnerPublications(repoRoot, packages, view).inputs) inputs.add(path);
  const visited = new Map<string, { source: string; kind: RegistryCompilerInputDependencies["kind"] }>(), importers = new Map<string, string>();
  const visit = (path: string, role: RegistryCompilerInputRole, importer?: string): void => {
    if (importer !== undefined && !importers.has(path)) importers.set(path, importer);
    const previous = visited.get(path);
    if (previous) {
      if (role === "implementation-entry" && previous.kind === "json-data") registryCompilerInputDependencies(previous.source, path, role);
      return;
    }
    const source = view.readText(path);
    const dependencies = registryCompilerInputDependencies(source, path, role);
    visited.set(path, { source, kind: dependencies.kind });
    inputs.add(path);
    for (const specifier of dependencies.imports) {
      if (specifier.startsWith("node:") || isRuntimeBuiltinSpecifierV1(specifier)) continue;
      if (specifier.startsWith(".")) {
        visit(relative(repoRoot, resolve(repoRoot, dirname(path), specifier)).replaceAll("\\", "/"), "static-import", path);
        continue;
      }
      const binding = authority.workspaceImports[specifier];
      if (!binding) { const chain = [path]; for (let at = importers.get(path); at !== undefined && chain.length < 32; at = importers.get(at)) chain.push(at); throw new Error(`Registry implementation import is not schema-owned: ${specifier} in ${path} (imported through ${chain.reverse().join(" -> ")})`); }
      const manifest = JSON.parse(view.readText(binding.manifestPath)) as { name?: string; exports?: Record<string, unknown> };
      if (manifest.name !== specifier || manifest.exports?.["."] !== "./" + relative(dirname(binding.manifestPath), binding.entryPath).replaceAll("\\", "/")) throw new Error(`Registry workspace import binding drifted: ${specifier}`);
      inputs.add(binding.manifestPath);
      visit(binding.entryPath, "implementation-entry", path);
    }
  };
  for (const path of authority.implementationEntryPaths) visit(path, "implementation-entry");
  return [...inputs].filter(Boolean).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
}
