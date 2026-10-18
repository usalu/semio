import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import type { AreaState, PackageGlueGrammarSpec, PackageMaturity } from "../📜️taxonomy-contracts/🟦️.ts";
import type { SemanticPackageProjectionCase } from "../📦️package-projection-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { type FixedSourceDispositionDecision, classifyPackageSource, classifyPackageSourceDisposition } from "../🏷️package-source-classification/🟦️.ts";
import { type DiscoveredOwner, type DiscoveredPackage, type DiscoveryBurndown, type DiscoveryProblem, type DiscoveryScan, type OwnerAccumulator, type PackageLang, type PackageRole, type PackageTarget, fallbackPackageId, isDiscoverySkipDirectory, jsonTable, parseSemioMarker, readdirSafe, scanCache, tomlTableBody, tomlTableValues } from "../🪧️owner-markers/🟦️.ts";
import { type RegistryCatalogInputView, gitlinkBoundaryCache } from "../📇️registry-catalog-inputs/🟦️.ts";
import { exactContractFilename, fixedContractFilename } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { projectionByteCompare } from "../🧱️path-projection-contracts/🟦️.ts";
import { semanticPackageGenerationAuthority } from "../🎁️package-projection/🟦️.ts";
import { fileKindIdForSourcePath, loadCatalogTaxonomy, loadTaxonomy, pathIsExcluded, semanticDirectoryKindId } from "../🔐️taxonomy-authority/🟦️.ts";

/** 🏗️ Renders only new generated leaves, excluding authored mappings and their source splices. */
export function semanticPackageGeneratedLeafPreview(repoRoot: string, packageId: SemanticPackageProjectionCase["id"], taxonomy: Taxonomy = loadTaxonomy()): readonly Readonly<{ path: string; content: string }>[] {
  return semanticPackageGenerationAuthority(repoRoot, packageId, taxonomy).generated.map(({ path, content }) => ({ path, content })).sort((left, right) => projectionByteCompare(left.path, right.path));
}

/** 📖️ Normative specification file-kind identifier for an artifact facet, if any. */
export function artifactSpecFileKindId(facetDirName: string, taxonomy: Taxonomy = loadTaxonomy()): string | undefined {
  return taxonomy.artifactSpecFileKinds?.[facetDirName];
}

/** 🧭️ Longest-prefix match of a repo-relative path against `taxonomy.areas` keys — `undefined` outside every declared area. */
export function areaOf(repoRelPath: string, taxonomy: Taxonomy = loadTaxonomy()): AreaState | undefined {
  const norm = repoRelPath.replaceAll("\\", "/").replace(/^\.\//, "");
  let bestKey: string | undefined;
  for (const key of Object.keys(taxonomy.areas)) {
    if (norm !== key && !norm.startsWith(`${key}/`)) continue;
    if (!bestKey || key.length > bestKey.length) bestKey = key;
  }
  return bestKey ? taxonomy.areas[bestKey] : undefined;
}

/**
 * 🏷️ Reads a package's role marker as declared by its ecosystem (`taxonomy.ecosystems.<lang>.marker`):
 * rust `[package.metadata.semio]`, typescript `package.json` `"semio"`, go `📋️project.json`
 * `metadata.semio`, python `pyproject.toml` `[tool.semio]`. `undefined` when the manifest is missing, the
 * ecosystem is discovery-opaque (no marker spec, e.g. dotnet), or no `role` is declared.
 */
export function readSemioMarker(manifestPath: string, lang: PackageLang, taxonomy: Taxonomy = loadTaxonomy()): { role: PackageRole; id?: string } | undefined {
  return existsSync(manifestPath) ? parseSemioMarker(readFileSync(manifestPath, "utf8"), lang, taxonomy) : undefined;
}

/**
 * 🏷️ Reads an arbitrary OPT-IN sub-table nested under a package's own semio marker table — rust
 * `[package.metadata.semio.<subKey>]`, TS `package.json`'s `"semio": {"<subKey>": {...}}` — the generic
 * mechanism per-concern consumers use to opt a package INTO extra behavior beyond the bare role/id every
 * marked package already declares via `readSemioMarker` (first consumer: Storybook coverage, see
 * `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`). `undefined` when the
 * manifest/table/sub-key is absent — silence here means "not opted in", never an error. Only ONE table
 * per sub-key is read (no TOML array-of-tables support); a package needing more than one entry, or
 * fields this decoder can't express, is expected to stay hand-curated by its consumer instead.
 */
export function readSemioMarkerSubTable(manifestPath: string, lang: PackageLang, subKey: string, taxonomy: Taxonomy = loadTaxonomy()): Record<string, unknown> | undefined {
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec || !existsSync(manifestPath)) return undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(readFileSync(manifestPath, "utf8"), `${spec.table}.${subKey}`);
    return body === undefined ? undefined : tomlTableValues(body);
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(readFileSync(manifestPath, "utf8"));
  } catch {
    return undefined;
  }
  return jsonTable(parsed, `${spec.table}.${subKey}`);
}

/** 🧹️ Drops the memoized repo scan — call after mutating the tree inside one process (tests, generators). */
export function clearDiscoveryCache(): void {
  scanCache.clear();
  gitlinkBoundaryCache.clear();
}

/** 🧾️ Separates an exact fixed filename's body proof from package membership and filename authority. */
export function fixedSourceDispositionDecision(contractId: string | undefined, content: string | null, taxonomy: Taxonomy = loadCatalogTaxonomy()): FixedSourceDispositionDecision | null {
  const disposition = contractId ? taxonomy.packageSourceDispositions[contractId] : undefined;
  if (!disposition || disposition.contractKind !== "fixed" || !disposition.grammarId) return null;
  const grammar = taxonomy.packageGlueGrammar[disposition.grammarId];
  if (!grammar) throw new Error(`Fixed source disposition ${contractId} references unknown grammar ${disposition.grammarId}`);
  if (content === null) return { contractId: contractId!, role: null, finding: "fixed-source-content-unreadable" };
  const role = classifyPackageSourceDisposition(content, disposition, grammar);
  const accepted = disposition.disposition === "tool-metadata" ? role === "tool-metadata" : grammar.allowedRoles.includes(role as PackageGlueGrammarSpec["allowedRoles"][number]);
  return { contractId: contractId!, role, finding: accepted ? null : "fixed-source-disposition-unresolved" };
}

/**
 * 🗺️ ONE repo walk answering every discovery question. For each `<owner>/📦️packages/<lang>/` it resolves one
 * direct manifest. An owner at `<domain>/🎯️targets/<target>` contributes the registered target identity; the
 * inverse `<owner>/📦️packages/<lang>/🎯️targets/<target>` relation is only reported and never discovered. It reads its role marker via
 * `readSemioMarker`, and — in the same pass — derives each owner's migration state from disk: residual
 * `⚡️implementations`/`⚡️implementation` dirs and Shape V1 entry files still at the owner root. A package's
 * existence IS the migration marker (`taxonomy.migratedMarker`), so no hand-maintained "already migrated" list
 * can drift. A markerless manifest is skipped silently in a `legacy`/`mixed`/`exempt`/undeclared area (not yet
 * migrated) but always shows up in `discoverBurndown`, so nothing vanishes unnoticed.
 */
export function scanRepo(repoRoot: string, taxonomy: Taxonomy, catalog?: { readonly view: RegistryCatalogInputView; readonly inputs: Set<string> }): DiscoveryScan {
  if (typeof taxonomy.packagesDirName !== "string" || typeof taxonomy.targetsDirName !== "string" || !taxonomy.roles || !taxonomy.areas || !taxonomy.configurableEntryContracts) throw new Error("Catalog discovery requires the complete validated taxonomy vocabulary.");
  const packagesDirName = taxonomy.packagesDirName;
  const targetsDirName = taxonomy.targetsDirName;
  const forbiddenSegments = new Set(taxonomy.forbiddenPathSegments);
  const owners = new Map<string, OwnerAccumulator>();
  const problems: DiscoveryProblem[] = [];
  const unmarkedManifests: { path: string; area: string }[] = [];
  const packagingViolations: { path: string; ownerRel: string }[] = [];
  const implDirsByArea: Record<string, number> = {};
  let implDirsTotal = 0;
  const rel = (abs: string): string => relative(repoRoot, abs).replaceAll("\\", "/");
  const catalogExists = (abs: string): boolean => {
    if (!catalog) return existsSync(abs);
    const path = rel(abs);
    const kind = catalog.view.kind(path);
    if (kind === "symlink") throw new Error(`Registry catalog input is a symlink: ${path}`);
    if (kind) catalog.inputs.add(path);
    return kind !== null;
  };
  const catalogEntries = (abs: string) => {
    if (!catalog) return readdirSafe(abs);
    const path = rel(abs);
    if (path) catalog.inputs.add(path);
    return catalog.view.entries(path).map((entry) => ({ name: entry.name, isDirectory: () => entry.nodeKind === "directory", isFile: () => entry.nodeKind === "file" }));
  };

  const addPackageProblem = (owner: OwnerAccumulator, path: string, kind: DiscoveryProblem["kind"], detail: string): void => {
    const repoPath = rel(path);
    packagingViolations.push({ path: repoPath, ownerRel: owner.ownerRel });
    problems.push({ kind, path: repoPath, message: `"${repoPath}" ${detail}` });
  };

  const collectPackageRoles = (packageRoot: string, lang: PackageLang, owner: OwnerAccumulator, entryContractIds: readonly string[]): void => {
    if (catalog) return;
    const rule = taxonomy.packageBoundaryRules[lang];
    const grammar = rule && taxonomy.packageGlueGrammar[rule.glueGrammarId];
    if (!rule || !grammar) {
      addPackageProblem(owner, packageRoot, "package-role-unresolved", `has no package boundary rule or glue grammar for ${lang}.`);
      return;
    }
    const fixedNames = new Map<string, string[]>();
    for (const id of rule.allowedFixedContractIds) {
      const contract = taxonomy.fixedFilenameContracts[id];
      if (!contract) continue;
      const name = fixedContractFilename(contract);
      fixedNames.set(name, [...(fixedNames.get(name) ?? []), id]);
    }
    const entryNames = new Map<string, string[]>();
    for (const id of entryContractIds) {
      const contract = taxonomy.configurableEntryContracts[id];
      if (!contract) continue;
      entryNames.set(contract.filename, [...(entryNames.get(contract.filename) ?? []), id]);
    }
    const allowedKinds = new Set(rule.allowedFileKindIds);
    const allowedDirectories = new Set(rule.allowedDirectoryKindIds);
    const visit = (dir: string): void => {
      if (pathIsExcluded(repoRoot, dir, taxonomy)) return;
      for (const entry of readdirSafe(dir).sort((a, b) => a.name.localeCompare(b.name))) {
        const path = join(dir, entry.name);
        if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
        if (entry.isDirectory()) {
          if (isDiscoverySkipDirectory(entry.name)) continue;
          if (existsSync(join(path, "CACHEDIR.TAG")) && /cache directory tag created by cargo/iu.test(readFileSync(join(path, "CACHEDIR.TAG"), "utf8"))) continue;
          const directoryKindId = semanticDirectoryKindId(entry.name, taxonomy);
          if (!directoryKindId || !allowedDirectories.has(directoryKindId)) addPackageProblem(owner, path, "packaging-violation", "is not an allowed semantic package directory.");
          visit(path);
          continue;
        }
        if (!entry.isFile()) {
          addPackageProblem(owner, path, "package-role-unresolved", "is neither a regular file nor a declared package directory.");
          continue;
        }
        const fixedContractIds = fixedNames.get(entry.name) ?? [];
        const configurableContractIds = entryNames.get(entry.name) ?? [];
        const entryContract = configurableContractIds.length > 0;
        const kindId = fileKindIdForSourcePath(entry.name, taxonomy);
        if (!kindId || (fixedContractIds.length === 0 && !entryContract && !allowedKinds.has(kindId))) {
          addPackageProblem(owner, path, "packaging-violation", "has no exact fixed/configurable contract or allowed file-kind identity.");
          continue;
        }
        const kind = taxonomy.fileKinds[kindId]!;
        if (kind.role !== "source") continue;
        let content: string;
        try {
          content = readFileSync(path, "utf8");
        } catch {
          addPackageProblem(owner, path, "package-role-unresolved", "could not be decoded as source.");
          continue;
        }
        const contractIds = [...fixedContractIds, ...configurableContractIds];
        const disposition = contractIds.map((id) => taxonomy.packageSourceDispositions[id]).find((value) => value !== undefined);
        if (contractIds.length > 0 && !disposition) {
          addPackageProblem(owner, path, "package-role-unresolved", "has a source-format fixed/configurable contract without a package source disposition.");
          continue;
        }
        const sourceGrammar = disposition?.grammarId ? taxonomy.packageGlueGrammar[disposition.grammarId] : grammar;
        if (!sourceGrammar) { addPackageProblem(owner, path, "package-role-unresolved", "references a missing package source grammar."); continue; }
        const dispositionRole = disposition && disposition.validator !== "package-glue" ? classifyPackageSourceDisposition(content, disposition, sourceGrammar) : undefined;
        const generalDecision = classifyPackageSource(content, sourceGrammar);
        const decision = dispositionRole === "tool-metadata"
          ? { role: dispositionRole, evidence: `fixed/configurable ${disposition!.validator} validator` }
          : dispositionRole === "unresolved" && generalDecision.role !== "implementation"
            ? { role: dispositionRole, evidence: `fixed/configurable ${disposition!.validator} validator rejected the source body` }
            : generalDecision;
        if (decision.role === "implementation") addPackageProblem(owner, path, "package-implementation", `contains authored implementation inside a package boundary: ${decision.evidence}.`);
        else if (decision.role !== "tool-metadata" && (decision.role === "unresolved" || !sourceGrammar.allowedRoles.includes(decision.role))) addPackageProblem(owner, path, "package-role-unresolved", `has uncertain or disallowed package role ${JSON.stringify(decision.role)}: ${decision.evidence}.`);
      }
    };
    visit(packageRoot);
  };

  const resolveOne = (manifestAbs: string, lang: PackageLang, owner: OwnerAccumulator, target: PackageTarget | undefined): void => {
    const manifestPath = rel(manifestAbs);
    const content = catalog?.view.readText(manifestPath);
    const marker = content === undefined ? readSemioMarker(manifestAbs, lang, taxonomy) : parseSemioMarker(content, lang, taxonomy);
    if (catalog) catalog.inputs.add(manifestPath);
    if (!marker) {
      unmarkedManifests.push({ path: manifestPath, area: owner.area });
      problems.push({ kind: "manifest-without-marker", path: manifestPath, message: `"${manifestPath}" has no resolvable semio role marker; all non-opaque areas require one.` });
      return;
    }
    if (!taxonomy.roles.includes(marker.role)) {
      problems.push({ kind: "unknown-role", path: manifestPath, message: `"${manifestPath}" declares unknown role "${marker.role}".` });
      return;
    }
    owner.packages.push({
      ownerRel: owner.ownerRel,
      lang,
      target,
      packageRel: rel(dirname(manifestAbs)),
      manifestPath,
      role: marker.role,
      id: marker.id ?? fallbackPackageId(manifestAbs, lang, owner.ownerRel, content),
      area: owner.area,
      maturity: "clean",
    });
  };

  const ownerTarget = (ownerRel: string): PackageTarget | undefined => {
    const parts = ownerRel.split("/");
    const target = parts.at(-1);
    return parts.at(-2) === targetsDirName && target && taxonomy.targets[target] ? target : undefined;
  };

  const scanPackagesDir = (packagesAbs: string, owner: OwnerAccumulator): void => {
    if (pathIsExcluded(repoRoot, packagesAbs, taxonomy)) return;
    for (const langEntry of catalogEntries(packagesAbs)) {
      if (!langEntry.isDirectory() || langEntry.name.startsWith(".") || isDiscoverySkipDirectory(langEntry.name)) continue;
      const lang = langEntry.name as PackageLang;
      const ecosystem = taxonomy.ecosystems[lang];
      if (!ecosystem) {
        const path = rel(join(packagesAbs, langEntry.name));
        problems.push({ kind: "unknown-lang", path, message: `"${path}" is not a declared language directory.` });
        continue;
      }
      const manifestFilename = exactContractFilename(ecosystem.manifestContractId, taxonomy);
      const langAbs = join(packagesAbs, langEntry.name);
      if (!manifestFilename) {
        collectPackageRoles(langAbs, lang, owner, ecosystem.entryContractIds);
        continue;
      }
      const directManifestAbs = join(langAbs, manifestFilename);
      const targetsAbs = join(langAbs, targetsDirName);
      const hasDirect = catalogExists(directManifestAbs);
      const hasTargets = catalogExists(targetsAbs);
      const target = ownerTarget(owner.ownerRel);
      if (hasTargets) addPackageProblem(owner, targetsAbs, "target-inside-package-boundary", "places a target subtree inside a package boundary.");
      collectPackageRoles(langAbs, lang, owner, target ? taxonomy.targets[target]!.entryContractIds : ecosystem.entryContractIds);
      if (hasDirect) {
        resolveOne(directManifestAbs, lang, owner, target);
        continue;
      }
      if (target) problems.push({ kind: "target-without-manifest", path: rel(langAbs), message: `"${rel(langAbs)}" has no exact manifest contract ${JSON.stringify(manifestFilename)}.` });
    }
  };

  const ownerRootEntryFiles = (entries: readonly { name: string; isDirectory: () => boolean }[]): string[] => {
    const names = new Set(Object.values(taxonomy.configurableEntryContracts).map((contract) => contract.filename));
    return entries.filter((entry) => !entry.isDirectory() && names.has(entry.name)).map((entry) => entry.name);
  };

  const walk = (absDir: string, ownerStack: readonly OwnerAccumulator[]): void => {
    if (pathIsExcluded(repoRoot, absDir, taxonomy)) return;
    const entries = catalogEntries(absDir);
    let stack = ownerStack;
    if (entries.some((entry) => entry.isDirectory() && entry.name === packagesDirName)) {
      const ownerRel = rel(absDir);
      const owner: OwnerAccumulator = { ownerRel, area: areaOf(ownerRel, taxonomy) ?? "", packages: [], residualImplDirs: 0, entryFilesAtOwnerRoot: ownerRootEntryFiles(entries) };
      if (catalog) for (const name of owner.entryFilesAtOwnerRoot) catalogExists(join(absDir, name));
      owners.set(ownerRel, owner);
      stack = [...ownerStack, owner];
      scanPackagesDir(join(absDir, packagesDirName), owner);
    }
    for (const entry of entries) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || isDiscoverySkipDirectory(entry.name) || entry.name === packagesDirName) continue;
      const path = join(absDir, entry.name);
      if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
      if (forbiddenSegments.has(entry.name)) {
        if (catalog) catalog.inputs.add(rel(path));
        implDirsTotal += 1;
        const area = stack.at(-1)?.area ?? areaOf(rel(absDir), taxonomy) ?? "";
        implDirsByArea[area] = (implDirsByArea[area] ?? 0) + 1;
        if (stack.length > 0) stack[stack.length - 1]!.residualImplDirs += 1;
        problems.push({ kind: "package-implementation", path: rel(path), message: `"${rel(path)}" is a forbidden implementation boundary in a clean-enforced area.` });
        continue;
      }
      walk(path, stack);
    }
  };
  walk(repoRoot, []);

  const discoveredOwners: DiscoveredOwner[] = [...owners.values()].map((owner) => {
    const maturity: PackageMaturity = owner.residualImplDirs === 0 && owner.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed";
    const packages = owner.packages.map((pkg) => ({ ...pkg, maturity }));
    return {
      ownerRel: owner.ownerRel,
      area: owner.area,
      maturity,
      langs: [...new Set(packages.map((pkg) => pkg.lang))],
      targets: [...new Set(packages.flatMap((pkg) => pkg.target ? [pkg.target] : []))],
      roles: [...new Set(packages.map((pkg) => pkg.role))],
      packages,
      residualImplDirs: owner.residualImplDirs,
      entryFilesAtOwnerRoot: owner.entryFilesAtOwnerRoot,
    };
  }).sort((a, b) => a.ownerRel.localeCompare(b.ownerRel));
  const packages = discoveredOwners.flatMap((owner) => owner.packages).sort((a, b) => a.ownerRel.localeCompare(b.ownerRel) || (a.target ?? "").localeCompare(b.target ?? ""));
  return {
    packages,
    owners: discoveredOwners,
    problems,
    burndown: {
      ownersTotal: discoveredOwners.length,
      packagesTotal: packages.length,
      cleanOwners: discoveredOwners.filter((owner) => owner.maturity === "clean").length,
      mixedOwners: discoveredOwners.filter((owner) => owner.maturity === "mixed"),
      implDirsTotal,
      implDirsByArea,
      unmarkedManifests,
      packagingViolations,
    },
  };
}

function scan(repoRoot: string, taxonomy: Taxonomy): DiscoveryScan {
  const cached = scanCache.get(repoRoot);
  if (cached) return cached;
  const result = scanRepo(repoRoot, taxonomy);
  scanCache.set(repoRoot, result);
  return result;
}

/** 📦️ Flat catalog of every marked package in the repo — see `scanRepo`. */
export function discoverPackages(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredPackage[] {
  return [...scan(repoRoot, taxonomy).packages];
}

/** 🏠️ Owner-level view of the same scan: one row per `📦️packages`-carrying dir, with its langs/targets/roles and derived maturity. */
export function discoverOwners(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredOwner[] {
  return [...scan(repoRoot, taxonomy).owners];
}

/** ⚠️ Diagnostics half of the scan (inverse target boundaries, missing target manifests, unknown langs/roles, and unmarked manifests). */
export function discoverPackageProblems(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveryProblem[] {
  return [...scan(repoRoot, taxonomy).problems];
}

/** 🔥️ Burn-down half of the scan: everything that must shrink to zero before the finalization flip. */
export function discoverBurndown(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveryBurndown {
  return scan(repoRoot, taxonomy).burndown;
}
