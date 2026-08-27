import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve, sep } from "node:path";
import { parseArgs } from "node:util";
import { nextestArtifactLocation, partitionNextestExecutionFilters } from "./📦️index.ts";
import { NEO4J_GRAPH_DATABASE_NAMES, getAllNeo4jGraphExportSpecs, joinNeo4jGraphDatabaseName, parseExtraNeo4jGraphDatabaseNamesFromEnv, partitionNeo4jGraphCliArgv, policyCanonicalArtifactKindBreaches, policyChildSlotKindDagBreaches, policyDissolvedKindRedefinitionBreaches, policyEmojiPrefixBreaches, policyModeCompletenessBreaches, policyPluginDependencyParityBreaches, policyWindowCompletenessBreaches } from "../../../../../../../📜️script.ts";
import { BundleScript, ScriptRouter, DAEMON_BUDGET_MS, ORCHESTRATOR_BUDGET_MS, budgetTimeoutHint, canReuseDevPort, capturedTestFailureDiagnostics, daemonBudgetMs, daemonBudgetOpts, describeDevPortOccupant, devServerUrl, devToolingEnv, dispatchSubcommand, findRepoRoot, gitSpawnEnv, goLevelTestArgs, isDevPortInUse, orchestratorBudgetMs, orchestratorBudgetOpts, resolveCargoPackageName, resolveCargoPackageNames, resolveDevPort, runCmd, runCmdStatus, runProbe, testLevelBudgetMs, vitestLevelArgs, wgpuDevPlayUrl } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { defineLint, type FileLinter } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { layeringBreaches, layeringCounts, layeringReferences, loadLayeringBaseline, policyDiscoveredAllowlist } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { dependencyBoundaryBreachesForBundleDir, dependencyBoundaryBreachesForFile, isAdapterBoundaryFile, parseTsImportSpecs } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import {
  PLAYGROUND_PORTS,
  PLAYGROUND_LOCKED_EXAMPLE_ENV,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  frameworkOsPlaygroundDevEnv,
  resolveFrameworkOsPlaygroundPlugin,
  loadFrameworkOsPlaygroundCatalog,
  playgroundPlayViteDefine,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { playgroundStaticSiteBuildOptions } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts";
import { areaOf, clearDiscoveryCache, discoverBurndown, discoverOwners, discoverPackageProblems, discoverPackages, getWorkspaceRoot, loadTaxonomy, readSemioMarker, resolveWorkspaceTaxonomyAuthority, resolveWorkspaceTaxonomyAuthorityFromDirectory, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { artifactFacetPathIsDeclared, buildSemanticCensus, canonicalPrimaryFilenameForKind, createRustMutationCodecOwnershipInspector, fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath, generatorNxPreviewCommand, inspectMutationMetadataSource, inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustMutationAggregateSpan, inspectRustMutationMetadataFacts, inspectRustStructure, inspectRustVirtualSources, projectCargoProviderManifest, renderRustStructuralFactsJson, renderSemanticCensusJson, resolveCargoProviderBinding, resolveRustPathAttributes, scopedFileKindIdForSourcePath, semanticPathProjectionAuthority, semanticProjectionCatalogProblems, taxonomyCliAttemptPreparationsProblems, taxonomyCliBackupPreparationProblems, taxonomyCliBackupWritePreparationProblems, taxonomyCliEditPreparationProblems, taxonomyCliEditWritePreparationProblems, taxonomyCliJsonWritePreparationProblems, taxonomyCliLeaseDirectoryProblems, taxonomyCliRestorePreparationProblems, validateGeneratorContractsAgainstWorkspace, type SemanticProjectionAuthorityNode, type SemanticProjectionCatalogRegistration, type Taxonomy } from "../../🔍️discovery/🟦️component.ts";
import { computeWorkspaces, diffWorkspaces } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { chmodSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readlinkSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { applyTaxonomyPlan, artifactProjectionTail, canonicalJson, inventoryTaxonomy, noFollowTreeDigest, opaqueTreeDigest, parseGeneratorPreviewManifest, parseTaxonomyPlan, planTaxonomy, repositoryLocalSymlinkTargetPath, taxonomyPlanDigest, taxonomyPlatformPathViolationCodes, taxonomyScopedGitPathspec, verifyTaxonomy, type OpaqueTreeDigest, type TaxonomyApplyResult, type TaxonomyInventory, type TaxonomyInventoryOptions, type TaxonomyPlan, type TaxonomyProgress } from "../../🧹️normalization/🟦️.ts";
import { ownedFilePaths, ownedFilesystemEntries, ownedPathByteSort, type OwnedFilesystemEntry } from "../../🧪️tests/🔍️filesystem/🟦️component.ts";
import Ajv from "ajv";
import toml from "@iarna/toml";
import fastGlob from "fast-glob";
import { MUTATION_STRUCTURAL_POLICY_KINDS, inspectMutationRootReachability, inventoryMutationTaxonomy, newScaffoldMutationTree, planMutationTaxonomy, policyMutationStructuralBreaches, runMutationTaxonomyCli, validateJsonSchemaSubset } from "../../../../../../../📜️script.ts";

//#region 🛡️ActiveTicketCleanProtection
describe("active ticket clean protection", () => {
  test("projects synthetic candidates with strict manifest and third-party oracle parity", async () => {
    const { cleanRemovalProtection, cleanProjectRemovals } = await import("../../../../../../../📜️script.ts");
    const fixturePath = join(import.meta.dir, "../../🧪️tests/🧹️clean");
    const fixture = JSON.parse(readFileSync(join(fixturePath, "🧪️fixture.json"), "utf8"));
    const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(fixturePath, "🧪️fixture.schema.json"), "utf8")));
    expect(validate(fixture)).toBe(true);
    for (const bad of [{ ...fixture, extra: true }, { ...fixture, version: 2 }, { ...fixture, cases: [] }]) expect(validate(bad)).toBe(false);
    const root = resolve("/synthetic-workspace");
    const nodes = new Map<string, { kind: "directory" | "file" | "symlink" | "unreadable"; text?: string }>([[root, { kind: "directory" }]]);
    const add = (path: string, kind: "directory" | "file" | "symlink" | "unreadable", text?: string): void => {
      const absolute = resolve(root, path); let parent = dirname(absolute);
      while (parent !== root && parent.startsWith(root + sep)) { if (!nodes.has(parent)) nodes.set(parent, { kind: "directory" }); parent = dirname(parent); }
      nodes.set(absolute, { kind, text });
    };
    for (const ticket of fixture.tickets) { add(ticket.path, "directory"); add(`${ticket.path}/log.txt`, "file", "evidence"); add(`${ticket.path}/target`, "directory"); if (ticket.manifest !== null) add(`${ticket.path}/🎫️ticket.json`, "file", ticket.manifest); }
    for (const entry of fixture.special) add(entry.path, entry.kind);
    const view = {
      kind: (path: string) => nodes.get(path)?.kind ?? "missing" as const,
      children: (path: string) => nodes.get(path)?.kind === "directory" ? [...nodes.keys()].filter((key) => key !== path && dirname(key) === path).map((key) => basename(key)) : undefined,
      read: (path: string) => nodes.get(path)?.text,
    };
    const validClosed = new Ajv({ strict: true }).compile({ type: "object", required: ["status"], properties: { status: { const: "closed" } }, additionalProperties: true });
    const intersects = (left: string, right: string) => left === right || left.startsWith(right + "/") || right.startsWith(left + "/");
    const protectedPaths = fixture.tickets.filter((ticket: { manifest: string | null }) => { try { return !validClosed(JSON.parse(ticket.manifest ?? "null")); } catch { return true; } }).map((ticket: { path: string }) => ticket.path);
    protectedPaths.push(...fixture.special.map((entry: { path: string }) => entry.path));
    const oracle = (path: string) => path !== "." && !path.startsWith("../") && !protectedPaths.some((prefix: string) => intersects(path, prefix));
    for (const vector of fixture.cases) {
      expect(oracle(vector.candidate)).toBe(vector.allowed);
      expect(cleanRemovalProtection(root, vector.candidate, view).length === 0).toBe(vector.allowed);
    }
    const parent = fixture.tickets.find((ticket: { path: string }) => ticket.path.endsWith("/NESTED")).path;
    const candidates = [{ kind: "gitignore" as const, path: parent, bytes: 99 }, { kind: "ticket-dir" as const, path: `${parent}/target`, bytes: 11 }];
    expect(cleanProjectRemovals(root, candidates, [], view).map((row) => row.path)).toEqual([`${parent}/target`]);
    const allowed = `${fixture.reopened}/target`;
    expect(cleanProjectRemovals(root, [{ kind: "ticket-dir", path: allowed, bytes: 11 }], [], view)).toHaveLength(1);
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"open"}');
    expect(cleanRemovalProtection(root, allowed, view)).toContain(resolve(root, fixture.reopened));
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"closed"}');
    let reads = 0;
    const changesDuringWalk = { ...view, read: (path: string) => path === resolve(root, `${fixture.reopened}/🎫️ticket.json`) && ++reads > 1 ? '{"status":"open"}' : view.read(path) };
    expect(cleanRemovalProtection(root, allowed, changesDuringWalk)).toContain(resolve(root, fixture.reopened));
    for (const manifest of ['{"status":null}', '{"status":0}', 'null', '"closed"']) { add(`${fixture.reopened}/🎫️ticket.json`, "file", manifest); expect(cleanRemovalProtection(root, allowed, view)).toContain(resolve(root, fixture.reopened)); }
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"closed"}');
    const unreadableChildren = { ...view, children: (path: string) => path === resolve(root, allowed) ? undefined : view.children(path) };
    expect(cleanRemovalProtection(root, allowed, unreadableChildren)).toContain(resolve(root, allowed));
    const changesAncestor = { ...view, children: (path: string) => { if (path === resolve(root, allowed)) add(fixture.reopened, "symlink"); return view.children(path); } };
    expect(cleanRemovalProtection(root, allowed, changesAncestor)).toContain(resolve(root, fixture.reopened));
    add(fixture.reopened, "directory");
    add(`${fixture.reopened}/🎫️ticket.json`, "symlink");
    expect(cleanRemovalProtection(root, allowed, view)).toContain(resolve(root, fixture.reopened));
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"closed"}');
    const blocked: string[] = [];
    expect(cleanProjectRemovals(root, candidates, [], view, (path) => blocked.push(path)).map((row) => row.path)).toEqual([`${parent}/target`]);
    expect(blocked).toContain(resolve(root, `${parent}/child`));
    expect(cleanProjectRemovals(root, [{ kind: "misplaced", path: ".🧬semio", bytes: 99 }], [resolve(root, ".🧬semio/🗺️map")], view)).toEqual([]);
  });
});
//#endregion 🛡️ActiveTicketCleanProtection

//#region 🔗️RegistryCompilerImports
describe("registryCompilerImports", () => {
  test("validates the language-neutral vectors against Bun and TypeScript", async () => {
    const { registryStaticImports } = await import("../../🔍️discovery/🟦️component.ts");
    const ts = await import("typescript");
    const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🔍️discovery/🧪️compiler-imports.json"), "utf8")) as { cases: { name: string; source: string; expected: string[] }[] };
    const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🔍️discovery/🧪️compiler-imports.schema.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    for (const vector of fixture.cases) {
      const source = ts.createSourceFile("fixture.tsx", vector.source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
      const oracle = [...new Set(source.statements.flatMap((statement) => {
        if (ts.isImportDeclaration(statement) && statement.importClause?.isTypeOnly || ts.isExportDeclaration(statement) && statement.isTypeOnly) return [];
        const specifier = ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement) ? statement.moduleSpecifier : undefined;
        return specifier && ts.isStringLiteral(specifier) ? [specifier.text] : [];
      }))].sort();
      expect(oracle).toEqual(vector.expected);
      expect(registryStaticImports(vector.source, "fixture.tsx")).toEqual(oracle);
    }
  });

  test("rejects malformed runtime compiler capabilities without trusting declarations", async () => {
    const { scanRegistryCompilerImports } = await import("../../🔍️discovery/🟦️component.ts");
    for (const platform of [null, {}, { Transpiler: 7 }, { Transpiler: () => ({}) }, { Transpiler: class {} }, { Transpiler: class { scanImports() { return {}; } } }, { Transpiler: class { scanImports() { return [{ path: 1, kind: "import-statement" }]; } } }, { Transpiler: class { scanImports() { return [{ path: "./a", kind: null }]; } } }]) {
      expect(() => scanRegistryCompilerImports("import './a'", "tsx", platform)).toThrow();
    }
    const row = { path: "./a", kind: "import-statement", ignored: true };
    const imports = scanRegistryCompilerImports("", "tsx", { Transpiler: class { scanImports() { return [row]; } } });
    row.path = "./rebound";
    expect(imports).toEqual([{ path: "./a", kind: "import-statement" }]);
  });
});
//#endregion 🔗️RegistryCompilerImports

//#region 📦️CargoProviderManifestProjection
const CARGO_PROVIDER_PROJECTION_FIXTURE = join(import.meta.dir, "../../🧪️tests/🧬️cargo-provider-projection/🔣️vectors.json");
const CARGO_PROVIDER_PROJECTION_SCHEMA = join(import.meta.dir, "../../🧪️tests/🧬️cargo-provider-projection/🛂️schema.json");

describe("cargo provider manifest projection", () => {
  test("matches independent TOML facts before preserving only bounded identity projections", () => {
    const fixture = JSON.parse(readFileSync(CARGO_PROVIDER_PROJECTION_FIXTURE, "utf8")) as {
      schemaVersion: number;
      accepted: readonly { id: string; locator: string; source: string; tomlOracle: unknown; expected: unknown }[];
      rejected: readonly { id: string; locator: string; source: string; reason: string; tomlOracle: unknown }[];
    };
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(CARGO_PROVIDER_PROJECTION_SCHEMA, "utf8")));
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    for (const vector of fixture.accepted) {
      expect(toml.parse(vector.source), vector.id).toEqual(vector.tomlOracle);
      expect(projectCargoProviderManifest({ locator: vector.locator, source: vector.source }), vector.id).toEqual(vector.expected);
    }
    for (const vector of fixture.rejected) {
      if (vector.tomlOracle === null) expect(() => toml.parse(vector.source), vector.id).toThrow();
      else expect(toml.parse(vector.source), vector.id).toEqual(vector.tomlOracle);
      expect(() => projectCargoProviderManifest({ locator: vector.locator, source: vector.source }), vector.reason).toThrow();
    }
  });
});
//#endregion 📦️CargoProviderManifestProjection

//#region 🔗️CargoProviderBinding
const CARGO_PROVIDER_BINDING_FIXTURE = join(import.meta.dir, "../../🧪️tests/🧬️cargo-provider-binding/🔣️vectors.json");
const CARGO_PROVIDER_BINDING_SCHEMA = join(import.meta.dir, "../../🧪️tests/🧬️cargo-provider-binding/🛂️schema.json");

describe("cargo provider binding", () => {
  test("resolves one selected local normal dependency without inferring a provider", () => {
    const fixture = JSON.parse(readFileSync(CARGO_PROVIDER_BINDING_FIXTURE, "utf8")) as { schemaVersion: 1; accepted: readonly { id: string; consumerManifest: string; consumerSource: string; providerManifest: string; providerSource: string; librarySource: string; dependencyKey: string; workspaceSource: string; workspaceRoot?: string; workspaceRootSymlink?: true; workspaceAncestorSymlink?: true; virtualFilesystem?: true; extraFiles?: readonly { path: string; source: string }[]; links?: readonly { path: string; target: string }[]; omitProviderManifest?: true; omitLibrarySource?: true; expected: { externName: string; packageName: string; libraryName: string; workspaceInherited: boolean } }[]; rejected: readonly { id: string; consumerManifest: string; consumerSource: string; providerManifest: string; providerSource: string; librarySource: string; dependencyKey: string; workspaceSource: string; workspaceRoot?: string; workspaceRootSymlink?: true; workspaceAncestorSymlink?: true; virtualFilesystem?: true; extraFiles?: readonly { path: string; source: string }[]; links?: readonly { path: string; target: string }[]; omitProviderManifest?: true; omitLibrarySource?: true; expected: null }[] };
    expect(new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(CARGO_PROVIDER_BINDING_SCHEMA, "utf8")))(fixture)).toBe(true);
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cargo provider binding artifacts.");
    mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot, "semio-cargo-provider-binding-"));
    try {
      for (const vector of [...fixture.accepted, ...fixture.rejected]) {
        const caseDirectory = join(root, vector.id), sourceRoot = vector.workspaceRootSymlink ? join(caseDirectory, "source") : vector.workspaceAncestorSymlink ? join(caseDirectory, "source", "workspace") : vector.workspaceRoot ?? caseDirectory;
        const caseRoot = vector.workspaceRootSymlink ? join(caseDirectory, "workspace") : vector.workspaceAncestorSymlink ? join(caseDirectory, "alias", "workspace") : sourceRoot, librarySource = join(sourceRoot, "provider/src/lib.rs");
        if (!vector.virtualFilesystem) {
          mkdirSync(dirname(librarySource), { recursive: true });
          writeFileSync(join(sourceRoot, "Cargo.toml"), vector.workspaceSource);
          mkdirSync(dirname(join(sourceRoot, vector.consumerManifest)), { recursive: true });
          writeFileSync(join(sourceRoot, vector.consumerManifest), vector.consumerSource);
          mkdirSync(dirname(join(sourceRoot, vector.providerManifest)), { recursive: true });
          if (!vector.omitProviderManifest) writeFileSync(join(sourceRoot, vector.providerManifest), vector.providerSource);
          if (!vector.omitLibrarySource) writeFileSync(librarySource, vector.librarySource);
          for (const file of vector.extraFiles ?? []) {
            const path = join(sourceRoot, file.path);
            mkdirSync(dirname(path), { recursive: true });
            writeFileSync(path, file.source);
          }
          for (const link of vector.links ?? []) {
            const path = join(sourceRoot, link.path);
            mkdirSync(dirname(path), { recursive: true });
            symlinkSync(process.platform === "win32" ? join(dirname(path), link.target) : link.target, path, "file");
          }
          if (vector.workspaceRootSymlink) symlinkSync(process.platform === "win32" ? sourceRoot : "source", caseRoot, process.platform === "win32" ? "junction" : "dir");
          if (vector.workspaceAncestorSymlink) symlinkSync(process.platform === "win32" ? join(caseDirectory, "source") : "source", join(caseDirectory, "alias"), process.platform === "win32" ? "junction" : "dir");
        }
        expect(toml.parse(vector.consumerSource), vector.id).toBeDefined();
        expect(toml.parse(vector.providerSource), vector.id).toBeDefined();
        const input = { workspaceRoot: caseRoot, consumerManifestLocator: vector.consumerManifest, dependencyKey: vector.dependencyKey };
        if (vector.expected === null) expect(() => resolveCargoProviderBinding(input), vector.id).toThrow();
        else expect(resolveCargoProviderBinding(input), vector.id).toMatchObject(vector.expected);
      }
    } finally {}
  });
});
//#endregion 🔗️CargoProviderBinding

//#region 🧬️MutationMetadataSourceProvider
const MUTATION_METADATA_SOURCE_FIXTURE = join(import.meta.dir, "../../🧪️tests/🧬️metadata-source-provider/🔣️vectors.json");
const MUTATION_METADATA_SOURCE_SCHEMA = join(import.meta.dir, "../../🧪️tests/🧬️metadata-source-provider/🛂️schema.json");

describe("mutation metadata source provider", () => {
  test("proves independent canonical derive and lower contract routes from one consumer declaration", () => {
    const fixture = JSON.parse(readFileSync(MUTATION_METADATA_SOURCE_FIXTURE, "utf8")) as { schemaVersion: 1; cases: readonly { id: string; derive: string; contract: string; facadeSource: string; manualImpl: string; binding?: "direct" | "inherited"; consumerPrefix?: string; consumerSuffix?: string; originModulePath?: readonly string[]; accepted: boolean }[] };
    expect(new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(MUTATION_METADATA_SOURCE_SCHEMA, "utf8")))(fixture)).toBe(true);
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for metadata source provider artifacts.");
    mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot, "semio-metadata-source-provider-"));
    try {
      const lower = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust", facade = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust", derive = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust", consumer = "consumer";
      const manifest = (path: string): string => `${path}/Cargo.toml`, source = (path: string): string => `${path}/📦️glue.rs`, relativePath = (from: string, to: string): string => relative(dirname(join(root, from)), join(root, to)).split(sep).join("/");
      for (const vector of fixture.cases) {
        const files = new Map<string, string>(), add = (path: string, text: string): void => { files.set(path, text); const absolute = join(root, path); mkdirSync(dirname(absolute), { recursive: true }); writeFileSync(absolute, text); };
        const dependencyRows = (from: string): string => `derive = { package = 'semio-framework-os-kernel-dsl-derive', path = '${relativePath(from, derive)}' }\nfacade = { package = 'semio-framework-os-kernel', path = '${relativePath(from, facade)}' }\nlower = { package = 'semio-framework-replication', path = '${relativePath(from, lower)}' }`;
        add("Cargo.toml", `[workspace]\n${vector.binding === "inherited" ? `\n[workspace.dependencies]\n${dependencyRows("Cargo.toml")}\n` : ""}`);
        add(manifest(lower), "[package]\nname = 'semio-framework-replication'\n\n[lib]\nname = 'protocol'\npath = '📦️glue.rs'\n");
        add(source(lower), "pub trait MutationLeaf {}\n");
        add(manifest(derive), "[package]\nname = 'semio-framework-os-kernel-dsl-derive'\n\n[lib]\nname = 'dsl_derive'\npath = '📦️glue.rs'\nproc-macro = true\n");
        add(source(derive), "pub fn marker() {}\n");
        add(manifest(facade), `[package]\nname = 'semio-framework-os-kernel'\n\n[lib]\nname = 'semio_framework_os_kernel'\npath = '📦️glue.rs'\n\n[dependencies]\nlower = { package = 'semio-framework-replication', path = '${relativePath(manifest(facade), lower)}' }\n`);
        add(source(facade), vector.facadeSource);
        add(manifest(consumer), `[package]\nname = 'consumer'\n\n[lib]\nname = 'consumer'\npath = '📦️glue.rs'\n\n[dependencies]\n${vector.binding === "inherited" ? "derive = { workspace = true }\nfacade = { workspace = true }\nlower = { workspace = true }" : dependencyRows(manifest(consumer))}\n`);
        add(source(consumer), `extern crate derive;\n${vector.consumerPrefix ?? ""}${vector.manualImpl}#[derive(${vector.derive})]\n#[mutation_leaf(contract = ${vector.contract})]\npub struct Payload;\n${vector.consumerSuffix ?? ""}`);
        const proof = inspectMutationMetadataSource({ repositoryRoot: root, consumerManifestLocator: manifest(consumer), origin: { sourcePath: source(consumer), declarationName: "Payload", modulePath: vector.originModulePath ?? [] }, files: [...files.keys()], readSource: (path) => files.get(path) });
        expect(proof.accepted, `${vector.id}: ${proof.diagnostics.join("; ")}`).toBe(vector.accepted);
        if (vector.accepted) expect(proof).toMatchObject({ deriveProvider: { role: "metadata-derive" }, contractProvider: { role: "lower-contract" }, facadeProvider: { role: "os-facade" } });
      }
      const compiler = join(root, "🧪️rustc"), lowerLibrary = join(compiler, "liblower.rlib"), facadeLibrary = join(compiler, "libfacade.rlib"), deriveLibrary = join(compiler, process.platform === "darwin" ? "libderive.dylib" : process.platform === "win32" ? "derive.dll" : "libderive.so"), consumerLibrary = join(compiler, "libconsumer.rlib");
      mkdirSync(compiler, { recursive: true });
      const compilerSource = (name: string, text: string): string => { const path = join(compiler, name); writeFileSync(path, text); return path; };
      const rustc = (args: string[]): void => { const result = spawnSync("rustc", args, { encoding: "utf8" }); expect({ status: result.status, signal: result.signal, stderr: result.stderr }).toMatchObject({ status: 0, signal: null }); };
      rustc(["--edition=2021", "--crate-name", "lower", "--crate-type", "rlib", compilerSource("lower.rs", "pub trait MutationLeaf {}\n"), "-o", lowerLibrary]);
      rustc(["--edition=2021", "--crate-name", "facade", "--crate-type", "rlib", compilerSource("facade.rs", "extern crate lower; pub use lower::MutationLeaf;\n"), "--extern", `lower=${lowerLibrary}`, "-L", `dependency=${compiler}`, "-o", facadeLibrary]);
      rustc(["--edition=2021", "--crate-name", "derive", "--crate-type", "proc-macro", compilerSource("derive.rs", "extern crate proc_macro; use proc_macro::TokenStream; #[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))] pub fn mutation_leaf(_: TokenStream) -> TokenStream { TokenStream::new() }\n"), "-o", deriveLibrary]);
      rustc(["--edition=2021", "--crate-name", "consumer", "--crate-type", "rlib", compilerSource("consumer.rs", "extern crate derive; extern crate facade; use self::facade::MutationLeaf as SelfContract; #[derive(derive::MutationLeaf)] #[mutation_leaf(contract = ::SelfContract)] pub struct SelfPayload; mod nested { use super::facade::MutationLeaf as SuperContract; #[derive(derive::MutationLeaf)] #[mutation_leaf(contract = ::SuperContract)] pub struct SuperPayload; }\n"), "--extern", `derive=${deriveLibrary}`, "--extern", `facade=${facadeLibrary}`, "--extern", `lower=${lowerLibrary}`, "-L", `dependency=${compiler}`, "-o", consumerLibrary]);
    } finally {}
  });

  test("proves current STDIO TXT and GLTF wrapped origins through kernel aliases", () => {
    const repositoryRoot = getWorkspaceRoot();
    const consumerManifest = "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml", consumerLibrary = "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs", facadeManifest = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml", facadeLibrary = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs", dslSource = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs";
    const leaves = [
      { root: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs", sourcePath: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🦀️.rs" },
      { root: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs", sourcePath: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🦀️.rs" },
    ] as const;
    const origins = leaves.map(({ sourcePath }) => {
      const declarations = inspectRustMutationMetadataFacts(readFileSync(join(repositoryRoot, sourcePath), "utf8")).declarations.filter((item) => item.visibility === "pub" && !item.conditional && item.mutationLeaf.state === "valid");
      expect(declarations, sourcePath).toHaveLength(1);
      return { sourcePath, declarationName: declarations[0]!.name, modulePath: declarations[0]!.modulePath };
    });
    const files = [consumerManifest, consumerLibrary, facadeManifest, facadeLibrary, dslSource, ...leaves.flatMap(({ root, sourcePath }) => [root, sourcePath])];
    expect(files.every((path) => !path.split("/").some((segment) => segment.toLocaleLowerCase("en-US") === "compose") && !lstatSync(join(repositoryRoot, path)).isSymbolicLink())).toBe(true);
    const source = new Map(files.map((path) => [path, readFileSync(join(repositoryRoot, path), "utf8")]));
    const graph = inspectRustModuleGraph([...source.keys()], (path) => source.get(path), { strictManifests: true });
    expect((graph.contexts.get(facadeLibrary) ?? []).filter((context) => context.manifestPath === facadeManifest && context.modulePath.length === 0)).toHaveLength(1);
    expect(graph.targets.get(`${facadeLibrary}\0os_dsl`)).toBe(facadeLibrary);
    expect(graph.targets.get(`${facadeLibrary}\0os_dsl::component`)).toBe(dslSource);
    expect((graph.contexts.get(facadeLibrary) ?? []).filter((context) => context.manifestPath === facadeManifest && context.modulePath.join("\0") === "os_dsl")).toHaveLength(1);
    expect((graph.contexts.get(dslSource) ?? []).filter((context) => context.manifestPath === facadeManifest && context.modulePath.join("\0") === "os_dsl\0component")).toHaveLength(1);
    expect(resolveCargoProviderBinding({ workspaceRoot: repositoryRoot, consumerManifestLocator: facadeManifest, dependencyKey: "dsl_derive" })).toMatchObject({ providerManifestLocator: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml", libraryName: "dsl_derive", procMacro: true });
    for (const origin of origins) {
      if (!origin) continue;
      expect((graph.contexts.get(origin.sourcePath) ?? []).filter((context) => context.manifestPath === consumerManifest && context.sourceScope.join("\0") === origin.modulePath.join("\0"))).toHaveLength(1);
      expect(inspectMutationMetadataSource({ repositoryRoot, consumerManifestLocator: consumerManifest, origin, files: [...source.keys()], readSource: (path) => source.get(path) })).toMatchObject({ accepted: true, deriveProvider: { role: "metadata-derive", manifestLocator: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml", libraryName: "dsl_derive", procMacro: true }, contractProvider: { role: "lower-contract", manifestLocator: "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml", libraryName: "protocol", procMacro: false }, facadeProvider: { role: "os-facade", manifestLocator: facadeManifest, libraryName: "semio_framework_os_kernel", procMacro: false } });
    }
  }, 30_000);
});
//#endregion 🧬️MutationMetadataSourceProvider

//#region 🧪️GitFixture
/** 🧬️Reads an isolated fixture's exact current commit without depending on subprocess pipe capture. */
function fixtureGitHead(root: string): string {
  const head = readFileSync(join(root, ".git", "HEAD"), "utf8").trim();
  return head.startsWith("ref: ") ? readFileSync(join(root, ".git", head.slice(5)), "utf8").trim() : head;
}
//#endregion 🧪️GitFixture

//#region 🔍️OwnedFilesystemDiscovery
const OWNED_FILESYSTEM_DISCOVERY_GOLDEN = join(import.meta.dir, "../../🧪️tests/🔍️filesystem/🧫️fixtures/🔣️.json");

describe("owned filesystem discovery", () => {
  test("sorts hostile Unicode and hidden nodes by bytes without following directory symlinks", () => {
    const golden = JSON.parse(readFileSync(OWNED_FILESYSTEM_DISCOVERY_GOLDEN, "utf8")) as {
      schemaVersion: number;
      directories: string[];
      files: { path: string; content: string }[];
      symlinks: { path: string; target: string }[];
      expected: OwnedFilesystemEntry[];
    };
    const root = mkdtempSync(join(tmpdir(), "owned-filesystem-discovery-"));
    try {
      expect(golden.schemaVersion).toBe(1);
      for (const path of golden.directories) mkdirSync(join(root, path), { recursive: true });
      for (const file of golden.files) {
        const target = join(root, file.path);
        mkdirSync(resolve(target, ".."), { recursive: true });
        writeFileSync(target, file.content);
      }
      for (const link of golden.symlinks) symlinkSync(process.platform === "win32" ? join(root, link.target) : link.target, join(root, link.path), process.platform === "win32" ? "junction" : "dir");
      const entries = ownedFilesystemEntries(root);
      expect(entries).toEqual(golden.expected);
      expect(entries.some(({ path }) => path.startsWith("alias/"))).toBe(false);
      expect(ownedFilesystemEntries(join(root, "absent"))).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🔍️OwnedFilesystemDiscovery

//#region 🧭️WorkspaceTaxonomyAuthority
const WORKSPACE_TAXONOMY_FIXTURE = join(import.meta.dir, "../../🧪️tests/🧬️workspace-taxonomy/🧫️fixtures/🔣️.json");
const WORKSPACE_TAXONOMY_SCHEMA = join(import.meta.dir, "../../🧪️tests/🧬️workspace-taxonomy/🛂️schema.json");

function workspaceTaxonomyFixtureRoot(prefix: string): { readonly root: string; readonly retained: boolean } {
  const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR?.trim();
  const base = artifactDirectory ? resolve(getWorkspaceRoot(), artifactDirectory) : realpathSync(tmpdir());
  mkdirSync(base, { recursive: true });
  return { root: mkdtempSync(join(base, prefix)), retained: Boolean(artifactDirectory) };
}

describe("workspace taxonomy authority", () => {
  test("resolves only a lexical, no-follow metadata.semio.taxonomy locator", () => {
    const fixture = JSON.parse(readFileSync(WORKSPACE_TAXONOMY_FIXTURE, "utf8")) as {
      canonicalLocator: string;
      childLocator: string;
      cases: readonly { name: string; manifest: object; target: "regular" | "file-symlink" | "directory-symlink" | "manifest-symlink"; expected: "resolve" | "reject"; reason: string }[];
      startCases: readonly { name: string; entry: "root" | "linked-child" | "child" | "compose"; nx: "regular" | "missing"; expected: "resolve" | "reject"; reason: string }[];
    };
    expect(new Ajv({ allErrors: true, strict: true }).compile(JSON.parse(readFileSync(WORKSPACE_TAXONOMY_SCHEMA, "utf8")))(fixture)).toBe(true);
    for (const vector of fixture.cases) {
      const fixtureRoot = workspaceTaxonomyFixtureRoot("semio-workspace-taxonomy-");
      const root = fixtureRoot.root;
      try {
        writeFileSync(join(root, "nx.json"), "{}\n");
        const manifestPath = join(root, "📋️project.json");
        const target = vector.name === "child-directory" || vector.name === "component-symlink" ? fixture.childLocator : fixture.canonicalLocator;
        const targetPath = join(root, target);
        if (vector.expected === "resolve" || vector.target !== "regular") {
          mkdirSync(dirname(targetPath), { recursive: true });
          writeFileSync(targetPath, "{}\n");
        }
        if (vector.target === "file-symlink") {
          const linked = `${targetPath}.linked`;
          writeFileSync(linked, "{}\n");
          rmSync(targetPath);
          symlinkSync(linked, targetPath, "file");
        }
        if (vector.target === "directory-symlink") {
          const component = join(root, "🧪️virtual");
          const linked = join(root, "🧪️linked");
          mkdirSync(join(linked, "📚️child"), { recursive: true });
          writeFileSync(join(linked, "📚️child", "🔣️taxonomy.json"), "{}\n");
          rmSync(component, { recursive: true, force: true });
          symlinkSync(linked, component, process.platform === "win32" ? "junction" : "dir");
        }
        if (vector.target === "manifest-symlink") {
          const linked = join(root, "📋️project-linked.json");
          writeFileSync(linked, JSON.stringify(vector.manifest));
          symlinkSync(linked, manifestPath, "file");
        } else writeFileSync(manifestPath, JSON.stringify(vector.manifest));
        if (vector.expected === "resolve") {
          const authority = resolveWorkspaceTaxonomyAuthority(root);
          expect(authority.relativePath).toBe(target);
          expect(relative(root, authority.taxonomyPath).replaceAll("\\", "/")).toBe(target);
          expect(lstatSync(authority.taxonomyPath).isSymbolicLink()).toBe(false);
        } else expect(() => resolveWorkspaceTaxonomyAuthority(root)).toThrow(vector.reason);
      } finally {
        if (!fixtureRoot.retained) rmSync(root, { recursive: true, force: true });
      }
    }
  });

  test("requires the exact marker pair and refuses symlink or compose ancestry before discovery", () => {
    const fixture = JSON.parse(readFileSync(WORKSPACE_TAXONOMY_FIXTURE, "utf8")) as {
      canonicalLocator: string;
      startCases: readonly { name: string; entry: "root" | "linked-child" | "child" | "compose" | "compose-parent-root" | "compose-parent-start" | "root-nul" | "root-drive-relative" | "symlink-parent"; nx: "regular" | "missing"; expected: "resolve" | "reject"; reason: string }[];
    };
    for (const vector of fixture.startCases) {
      const fixtureRoot = workspaceTaxonomyFixtureRoot("semio-workspace-taxonomy-start-");
      const root = fixtureRoot.root;
      try {
        const workspace = vector.entry === "compose" ? join(root, "compose", "workspace") : join(root, "workspace");
        const target = join(workspace, fixture.canonicalLocator);
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, "{}\n");
        writeFileSync(join(workspace, "📋️project.json"), JSON.stringify({ metadata: { semio: { taxonomy: fixture.canonicalLocator } } }));
        if (vector.nx === "regular") writeFileSync(join(workspace, "nx.json"), "{}\n");
        const start = vector.entry === "linked-child" ? join(root, "linked-workspace", "child") : vector.entry === "child" ? join(workspace, "child", "nested") : vector.entry === "symlink-parent" ? `${root}${sep}linked-workspace${sep}..${sep}workspace` : vector.entry === "compose-parent-start" ? `${root}${sep}COMPOSE${sep}..${sep}workspace` : workspace;
        if (vector.entry === "linked-child") {
          mkdirSync(join(workspace, "child"), { recursive: true });
          symlinkSync(workspace, join(root, "linked-workspace"), process.platform === "win32" ? "junction" : "dir");
        }
        if (vector.entry === "symlink-parent") symlinkSync(workspace, join(root, "linked-workspace"), process.platform === "win32" ? "junction" : "dir");
        if (vector.entry === "child") mkdirSync(start, { recursive: true });
        if (vector.expected === "resolve") {
          const authority = resolveWorkspaceTaxonomyAuthorityFromDirectory(start);
          expect(authority.workspaceRoot).toBe(workspace);
        } else {
          const resolveAuthority = vector.name === "marker-pair" || vector.entry === "compose" ? () => resolveWorkspaceTaxonomyAuthority(workspace) : vector.entry === "compose-parent-root" ? () => resolveWorkspaceTaxonomyAuthority(`${root}${sep}compose${sep}..${sep}workspace`) : vector.entry === "root-nul" ? () => resolveWorkspaceTaxonomyAuthority(`${workspace}\0`) : vector.entry === "root-drive-relative" ? () => resolveWorkspaceTaxonomyAuthority("C:workspace") : () => resolveWorkspaceTaxonomyAuthorityFromDirectory(start);
          expect(resolveAuthority).toThrow(vector.reason);
        }
      } finally {
        if (!fixtureRoot.retained) rmSync(root, { recursive: true, force: true });
      }
    }
  });

  test("does not borrow an outer taxonomy past a malformed nested nx workspace anchor", () => {
    const fixture = JSON.parse(readFileSync(WORKSPACE_TAXONOMY_FIXTURE, "utf8")) as {
      canonicalLocator: string;
      anchorCases: readonly { name: string; nx: "regular" | "symlink"; project: "regular" | "missing" | "symlink"; reason: string }[];
    };
    for (const vector of fixture.anchorCases) {
      const fixtureRoot = workspaceTaxonomyFixtureRoot("semio-workspace-taxonomy-anchor-");
      const root = fixtureRoot.root;
      try {
        const outer = join(root, "outer-workspace");
        const outerTaxonomy = join(outer, fixture.canonicalLocator);
        mkdirSync(dirname(outerTaxonomy), { recursive: true });
        writeFileSync(outerTaxonomy, "{}\n");
        writeFileSync(join(outer, "nx.json"), "{}\n");
        writeFileSync(join(outer, "📋️project.json"), JSON.stringify({ metadata: { semio: { taxonomy: fixture.canonicalLocator } } }));
        const inner = join(outer, "nested-workspace");
        mkdirSync(join(inner, "child"), { recursive: true });
        const nxPath = join(inner, "nx.json");
        const projectPath = join(inner, "📋️project.json");
        if (vector.nx === "regular") writeFileSync(nxPath, "{}\n");
        else symlinkSync(join(outer, "nx.json"), nxPath, "file");
        if (vector.project === "regular") writeFileSync(projectPath, "{}\n");
        if (vector.project === "symlink") symlinkSync(join(outer, "📋️project.json"), projectPath, "file");
        expect(() => resolveWorkspaceTaxonomyAuthorityFromDirectory(join(inner, "child"))).toThrow(vector.reason);
      } finally {
        if (!fixtureRoot.retained) rmSync(root, { recursive: true, force: true });
      }
    }
  });
});
//#endregion 🧭️WorkspaceTaxonomyAuthority

//#region 🧪️EmojiPrefixPolicy
describe("emoji-prefix policy", () => {
  test("requires prefixes on renamable entries and exempts ecosystem filenames only at schema-owned package roots", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "plain-dir"), { recursive: true });
      writeFileSync(join(owner, "plain.ts"), "");
      writeFileSync(join(owner, "Cargo.toml"), "");
      const rustPackage = join(owner, "📦️packages", "🦀️rust");
      mkdirSync(rustPackage, { recursive: true });
      writeFileSync(join(rustPackage, "Cargo.toml"), "");
      const scopes = policyEmojiPrefixBreaches(root).map((breach) => breach.scope);
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/plain-dir");
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/plain.ts");
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/Cargo.toml");
      expect(scopes).not.toContain("✏️s/🔌️plugins/🧪️probe/📦️packages/🦀️rust/Cargo.toml");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects duplicate sibling emoji identities after VS16 normalization", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(owner, { recursive: true });
      mkdirSync(join(owner, "🧭️first"), { recursive: true });
      mkdirSync(join(owner, "🧭️second"), { recursive: true });
      expect(policyEmojiPrefixBreaches(root).some((breach) => breach.kind === "taxonomy/emoji-prefix-uniqueness")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects duplicate identities shared by sibling files and directories", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "📄️folder"), { recursive: true });
      writeFileSync(join(owner, "📄️file.ts"), "");
      expect(policyEmojiPrefixBreaches(root).some((breach) => breach.kind === "taxonomy/emoji-prefix-uniqueness")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("accepts distinct VS16 emoji identities", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "🧭️first"), { recursive: true });
      writeFileSync(join(owner, "📄️second.ts"), "");
      expect(policyEmojiPrefixBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️EmojiPrefixPolicy

//#region 🪟️WindowCompletenessPolicy
describe("window completeness policy", () => {
  test("requires item-only capability facets and their language mirrors", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-window-policy-"));
    const ownerRel = "🧪️owner";
    // 👁️✏️ Windows now live under a subset's surface (👁️viewer/✏️editor), not 🎛️apps (W3 dissolution,
    // ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) — mirrors the on-disk shape every other
    // fixture in this file already uses for 🏅️standards/🪆️subsets.
    const window = join(root, ownerRel, "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "👁️viewer", "🎭️modes", "🧪️mode", "🪟️windows", "🧪️window");
    const crate = { shape: "taxonomy", ownerRel, pluginId: "fixture" } as const;
    try {
      mkdirSync(window, { recursive: true });
      // 🔣️ The required set is taxonomy-driven, never a literal here — adding a lane to
      // windowRequiredChildDirs must not require editing this assertion.
      const required = loadTaxonomy().windowRequiredChildDirs;
      const missingFacets = policyWindowCompletenessBreaches(root, [crate]);
      expect(missingFacets.filter((breach) => breach.kind === "taxonomy/window-completeness")).toHaveLength(required.length);
      expect(required).toContain("🎚️config");
      expect(required).toContain("🫧️transient");
      for (const facet of required) {
        const dir = join(window, facet);
        mkdirSync(dir, { recursive: true });
        writeFileSync(join(dir, "📝️.md"), "");
      }
      expect(policyWindowCompletenessBreaches(root, [crate])).toEqual([]);
      const action = join(window, "🎬️actions", "🧪️action");
      mkdirSync(action);
      expect(policyWindowCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/window-component")).toHaveLength(2);
      expect(policyWindowCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/window-empty-facet")).toHaveLength(1);
      rmSync(join(window, "🎬️actions", "📝️.md"));
      writeFileSync(join(action, "🦀️component.rs"), "");
      writeFileSync(join(action, "🟦️component.ts"), "");
      expect(policyWindowCompletenessBreaches(root, [crate])).toEqual([]);
      writeFileSync(join(window, "🎬️actions", "🦀️component.rs"), "");
      expect(policyWindowCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(["taxonomy/window-facet-component"]);
      rmSync(join(window, "🎬️actions", "🦀️component.rs"));
      rmSync(join(action, "🟦️component.ts"));
      const missingMirror = policyWindowCompletenessBreaches(root, [crate]);
      expect(missingMirror.map((breach) => breach.kind)).toEqual(["taxonomy/window-component"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("requires every mode to declare its windows collection and its three state lanes", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-mode-policy-"));
    const ownerRel = "🧪️owner";
    // 👁️✏️ Modes now live under a subset's surface, not 🎛️apps (W3 dissolution) — see the sibling
    // window-completeness test above for the same shape rationale.
    const mode = join(root, ownerRel, "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "👁️viewer", "🎭️modes", "🧪️mode");
    const crate = { shape: "taxonomy", ownerRel, pluginId: "fixture" } as const;
    try {
      mkdirSync(mode, { recursive: true });
      const required = loadTaxonomy().modeRequiredChildDirs;
      expect(required).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
      expect(policyModeCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/mode-completeness")).toHaveLength(required.length);
      for (const child of required) {
        mkdirSync(join(mode, child), { recursive: true });
      }
      expect(policyModeCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(required.map(() => "taxonomy/mode-empty-child"));
      for (const child of required) {
        writeFileSync(join(mode, child, "📝️.md"), "");
      }
      expect(policyModeCompletenessBreaches(root, [crate])).toEqual([]);
      mkdirSync(join(mode, "🪟️windows", "🧪️window"), { recursive: true });
      expect(policyModeCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(["taxonomy/mode-empty-child"]);
      rmSync(join(mode, "🪟️windows", "📝️.md"));
      expect(policyModeCompletenessBreaches(root, [crate])).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🪟️WindowCompletenessPolicy

//#region 🧪️CompositionPolicy
describe("composition policy (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W6)", () => {
  test("canonical-artifact-kind rejects legacy ArtifactKindSpec.id grammar but accepts s.<plugin>.<artifact>", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-kind-policy-"));
    const artifact = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe");
    try {
      mkdirSync(artifact, { recursive: true });
      writeFileSync(
        join(artifact, "🦀️component.rs"),
        ['pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {', '    semio_framework_plugin::ArtifactKindSpec {', '        id: "3d.mesh".into(),', "    }", "}"].join("\n"),
      );
      const legacy = policyCanonicalArtifactKindBreaches(root);
      expect(legacy.map((breach) => breach.kind)).toEqual(["composition/canonical-artifact-kind"]);
      expect(legacy[0]?.priority).toBe("medium");

      writeFileSync(
        join(artifact, "🦀️component.rs"),
        ['pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {', '    semio_framework_plugin::ArtifactKindSpec {', '        id: "s.probe.probe".into(),', "    }", "}"].join("\n"),
      );
      expect(policyCanonicalArtifactKindBreaches(root)).toEqual([]);

      // 🩹 Doc-comment prose quoting the pattern must not trip the rule (the exact false-positive
      // class flagged in the UCAS ticket's own 📌️important.md sweep-the-pattern lesson).
      writeFileSync(join(artifact, "🦀️component.rs"), '/// see `ArtifactKindSpec { id: "3d.mesh" }` for the old shape\n');
      expect(policyCanonicalArtifactKindBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("child-slot-kind-dag accepts an acyclic composition graph and rejects a cycle", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-dag-policy-"));
    const schemaA = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probea", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema");
    const schemaB = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probeb", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema");
    try {
      mkdirSync(schemaA, { recursive: true });
      mkdirSync(schemaB, { recursive: true });
      writeFileSync(join(schemaA, "🦀️component.rs"), '#[child(kind = "s.probe.probeb")] pub content: store::ArtifactChild<ProbeB>,\n');
      writeFileSync(join(schemaB, "🦀️component.rs"), "pub other: i32,\n");
      expect(policyChildSlotKindDagBreaches(root)).toEqual([]);

      writeFileSync(join(schemaB, "🦀️component.rs"), '#[child(kind = "s.probe.probea")] pub back: store::ArtifactChild<ProbeA>,\n');
      const cycles = policyChildSlotKindDagBreaches(root);
      expect(cycles).toHaveLength(1);
      expect(cycles[0]?.kind).toBe("composition/child-slot-kind-dag");
      expect(cycles[0]?.priority).toBe("high");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("dissolved-kind-redefinition bans redeclaring a frozen 🧿️semio subset type outside 🗄️stdio", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-dissolved-policy-"));
    const plugin = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe");
    const stdio = join(root, "✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio");
    try {
      mkdirSync(plugin, { recursive: true });
      mkdirSync(stdio, { recursive: true });
      writeFileSync(join(plugin, "🦀️component.rs"), "pub struct SemioMeshSnapshot { pub vertices: Vec<f64> }\n");
      writeFileSync(join(stdio, "🦀️component.rs"), "pub struct SemioMeshSnapshot { pub vertices: Vec<f64> }\n");
      const breaches = policyDissolvedKindRedefinitionBreaches(root);
      expect(breaches).toHaveLength(1);
      expect(breaches[0]?.kind).toBe("composition/dissolved-kind-redefinition");
      expect(breaches[0]?.scope).toBe("✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️probe/🦀️component.rs");

      rmSync(join(plugin, "🦀️component.rs"));
      writeFileSync(join(plugin, "🦀️component.rs"), "pub struct ProbeOwnType { pub vertices: Vec<f64> }\n");
      expect(policyDissolvedKindRedefinitionBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️CompositionPolicy

describe("Neo4j graph database registry", () => {
  test("joins name segments with hyphen", () => {
    expect(joinNeo4jGraphDatabaseName(["compose", "kit"])).toBe("compose-kit");
  });

  test("partitions argv into name segments and uvx passthrough", () => {
    expect(partitionNeo4jGraphCliArgv(["metabolism", "--verbose"])).toEqual({
      nameParts: ["metabolism"],
      passthrough: ["--verbose"],
    });
  });

  test("product graphs are the three current joined names", () => {
    expect(NEO4J_GRAPH_DATABASE_NAMES).toEqual(["elements", "coda", "reuse"]);
  });

  test("NEO4J_EXTRA_GRAPH_DATABASES extends export specs", () => {
    const env = { NEO4J_EXTRA_GRAPH_DATABASES: " foo , bar-baz " };
    expect(parseExtraNeo4jGraphDatabaseNamesFromEnv(env)).toEqual(["foo", "bar-baz"]);
    const names = getAllNeo4jGraphExportSpecs(env).map((s) => joinNeo4jGraphDatabaseName(s));
    expect(names).toContain("foo");
    expect(names).toContain("bar-baz");
  });
});

//#region 🧩️NxUnicodeTransport
describe("Nx Unicode project transport", () => {
  test("forces in-process plugin discovery even when the caller requests isolated workers", () => {
    const env = devToolingEnv({ NX_ISOLATE_PLUGINS: "true" });
    expect(env.NX_ISOLATE_PLUGINS).toBe("false");
  });

  test("builds the describe graph without a lossy duplicate repo-test root", () => {
    const root = getWorkspaceRoot();
    const result = spawnSync("bun", ["📜️script.ts", "nx", "show", "projects", "--with-target", "describe"], { cwd: root, encoding: "utf8", env: { ...process.env, NX_ISOLATE_PLUGINS: "true" } });
    expect(result.status, result.stderr).toBe(0);
  }, 20_000);
});
//#endregion 🧩️NxUnicodeTransport

describe("isDevPortInUse", () => {
  test("returns false for a high ephemeral port", () => {
    expect(isDevPortInUse("127.0.0.1", 59_999)).toBe(false);
  });

  test("returns true when puzzle 3d play port is listening", () => {
    if (!isDevPortInUse("127.0.0.1", 6013) && !isDevPortInUse("127.0.0.1", 6014)) return;
    expect(isDevPortInUse("127.0.0.1", 6013) || isDevPortInUse("127.0.0.1", 6014)).toBe(true);
  });
});

describe("resolveDevPort", () => {
  test("returns preferred port when free", () => {
    expect(resolveDevPort("127.0.0.1", 59_990)).toBe(59_990);
  });

  test("skips occupied ports", () => {
    if (!isDevPortInUse("127.0.0.1", 6013)) return;
    expect(resolveDevPort("127.0.0.1", 6013)).toBeGreaterThan(6013);
  });

  test("honors skipPorts", () => {
    expect(resolveDevPort("127.0.0.1", 59_991, 5, new Set([59_991, 59_992]))).toBe(59_993);
  });
});

describe("devServerUrl", () => {
  test("maps 0.0.0.0 to loopback", () => {
    expect(devServerUrl("0.0.0.0", 6019)).toBe("http://127.0.0.1:6019/");
  });
});

describe("wgpuDevPlayUrl", () => {
  test("builds root and legacy play urls", () => {
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly")).toBe("http://127.0.0.1:6178/?plugin=lowpoly");
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly", "/renderer-modules/wgpu/")).toBe("http://127.0.0.1:6178/renderer-modules/wgpu/?plugin=lowpoly");
  });
});

describe("canReuseDevPort", () => {
  test("returns true for an active dev HTTP server", () => {
    if (!isDevPortInUse("127.0.0.1", 6019)) return;
    expect(canReuseDevPort("127.0.0.1", 6019)).toBe(true);
  });
});

describe("describeDevPortOccupant", () => {
  test("returns undefined for a free port", () => {
    expect(describeDevPortOccupant(59_998)).toBeUndefined();
  });
});

describe("bundle-script", () => {
  test("ScriptRouter usage lists registered commands", () => {
    class A extends BundleScript {
      run(): void {}
    }
    const router = new ScriptRouter(import.meta.dir).register("a", A).register("b", A);
    expect(router.hasCommands()).toBe(true);
    expect(router.usage()).toContain("a");
    expect(router.usage()).toContain("b");
  });

  test("gitRepoRoot uses monorepo toplevel from repo/lib/js", async () => {
    const { gitRepoRoot } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const top = gitRepoRoot(import.meta.dir);
    expect(top).toBe(getWorkspaceRoot());
    expect(existsSync(join(top, ".git"))).toBe(true);
    expect(existsSync(join(top, "nx.json"))).toBe(true);
  });

  test("findRepoRoot reaches monorepo from repo/lib/js", () => {
    const root = findRepoRoot(import.meta.dir);
    expect(existsSync(join(root, "nx.json"))).toBe(true);
  });

  test("dispatchSubcommand invokes handler for first segment", () => {
    let ran = "";
    dispatchSubcommand(
      ["go", "x"],
      {
        go: (rest) => {
          ran = rest.join(",");
        },
      },
      "unused",
    );
    expect(ran).toBe("x");
  });
});

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});

describe("dependency-boundary", () => {
  test("detects adapter region marker", () => {
    expect(isAdapterBoundaryFile("pkg/foo.ts", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/main.py", "# #region 🔌️Adapters\nimport fastapi")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/index.ts", "//#region 🌐️RsWasmTransport\nexport async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/kit-store.🟦️worker.ts", "export async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/bin/assistant/mcp-app.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("framework/platform/renderer/react/index.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/foo.ts", "import x from 'react'")).toBe(false);
  });

  test("parseTsImportSpecs extracts module", () => {
    expect(parseTsImportSpecs(`import { z } from "zod";`)).toEqual(["zod"]);
  });

  test("flags direct third-party import outside adapter", () => {
    const content = `import React from "react";\nexport const a = React.createElement("div");\n`;
    const file = "🧰️framework/🧪️boundary-probe.ts";
    const breachs = dependencyBoundaryBreachesForFile(findRepoRoot(import.meta.dir), file, content, file);
    expect(breachs.length).toBeGreaterThan(0);
    expect(breachs[0]?.kind).toBe("dependency-boundary/import/direct-third-party");
  });

  test("dependencyBoundaryBreachesForBundleDir walks nested tsx", () => {
    const repoRoot = new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1");
    const dir = "framework/playground/renderer/react/puzzle";
    const breachs = dependencyBoundaryBreachesForBundleDir(repoRoot, dir);
    expect(breachs.every((b) => b.scope.startsWith("framework/playground/renderer/react/puzzle"))).toBe(true);
  });

  test("allows third-party import inside adapter region", () => {
    const content = `// #region 🔌️Adapters\nimport { NextResponse } from "next/server";\n// #endregion 🔌️Adapters\nexport async function GET() { return NextResponse.json({}); }\n`;
    const file = "repo/server/coordinator/app/api/v1/health/route.ts";
    const breachs = dependencyBoundaryBreachesForFile(new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"), file, content, file);
    expect(breachs).toEqual([]);
  });
});

describe("ui scrollbar styling", () => {
  test("🎨️ui.css defines scrollbar tokens and native plus Scrollable rules", () => {
    const repoRoot = findRepoRoot(import.meta.dir);
    const css = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️ui.css"), "utf8");
    expect(css).toContain("--scrollbar-size:");
    expect(css).toContain("--scrollbar-thumb:");
    expect(css).toContain("scrollbar-color:");
    expect(css).toContain("*::-webkit-scrollbar-thumb");
    expect(css).toContain('[data-slot="scroll-area-thumb"]');
  });
});

describe("micro-commit", () => {
  function gitDirFor(root: string): string {
    const rel = spawnSync("git", ["rev-parse", "--git-dir"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim() ?? ".git";
    return rel.startsWith("/") ? rel : join(root, rel);
  }

  test("safeGitEnv replaces an unusable configured global Git config", async () => {
    const { safeGitEnv } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { devNull } = await import("node:os");
    const env = safeGitEnv({ GIT_CONFIG_GLOBAL: join(process.cwd(), ".missing-global-gitconfig") });
    expect(env.GIT_CONFIG_GLOBAL).toBe(devNull);
  });

  test("branchValidationError distinguishes lookup failure, detached HEAD, and a wrong branch", async () => {
    const { branchValidationError } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(branchValidationError("micro-commit", { ok: false, error: "permission denied" })).toBe("micro-commit: cannot read current branch: permission denied");
    expect(branchValidationError("micro-commit", { ok: true, name: "" })).toContain("detached HEAD");
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/feature" })).toContain('current branch "🐙️ueli/feature"');
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
  });

  test("extractCounterFromSubject reads formatted subject lines", async () => {
    const { extractCounterFromSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(extractCounterFromSubject("🧑️ueli🎆️26🌙️06☀️02🚩️009")).toEqual({ nnn: 9, line1Base: "🧑️ueli🎆️26🌙️06☀️02" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️397")).toEqual({ nnn: 397, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️396")).toEqual({ nnn: 396, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("33")).toBeNull();
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });

  test("extractNumericCounterFromSubject reads GitKraken numeric subjects", async () => {
    const { extractNumericCounterFromSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(extractNumericCounterFromSubject("299")).toBe(299);
    expect(extractNumericCounterFromSubject("001")).toBe(1);
    expect(extractNumericCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });

  test("line1BaseFromBundleTag reads WIP epoch from squash tag", async () => {
    const { line1BaseFromBundleTag } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });

  test("bumpCounterFromHistory uses max across formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["🐙️ueli🎆️26🌙️06☀️02🚩️033", "🐙️ueli🎆️26🌙️06☀️02🚩️032", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });

  test("bumpCounterFromHistory preserves selector-free history with canonical output", async () => {
    const { bumpCounterFromHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const bumped = bumpCounterFromHistory(["🦢️other🎆️26🌙️07☀️31🚩️999", "🐙️ueli🎆️26🌙️06☀️04🚩️397", "🐙️ueli🎆️26🌙️06☀️04🚩️396"], contributor, new Date("2026-07-31T12:00:00"));
    expect(bumped).toEqual({ line1Base: "🐙️ueli🎆️26🌙️06☀️04", nnn: "398" });
    expect(() => bumpCounterFromHistory(["🐙️ueli🎆️26🌙️6☀️04🚩️397"], contributor, new Date("2026-07-31T12:00:00"))).toThrow("refusing to reset counter to 001");
  });

  test("bumpCounterFromHistory continues numeric GitKraken subjects with WIP epoch", async () => {
    const { bumpCounterFromHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["299", "298", "297"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-07-17T12:00:00"), "🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.nnn).toBe("300");
  });

  test("normalizeBulletLines strips uloc block lines", async () => {
    const { normalizeBulletLines } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bullets = normalizeBulletLines("🎆️Summary\n📊️metric📃uloc💯️65k➕️1✏️1🟰️2\n📊️metric🟦️typescript📃uloc💯️65k➕️1✏️1🟰️2\n🐛️Fix bug");
    expect(bullets).toEqual(["🎆️Summary", "🐛️Fix bug"]);
  });

  test("bulletEmojiValidationError rejects fireworks emoji on bullets", async () => {
    const { bulletLeadEmoji, bulletEmojiValidationError } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(bulletLeadEmoji("🎆️Drop stacked intro")).toBe("🎆️");
    expect(bulletEmojiValidationError(["🎆️All bullets wrongly use fireworks"])).toContain("🎆️");
    expect(bulletEmojiValidationError(["🐛️Fix real bug"])).toBeNull();
    expect(bulletEmojiValidationError(["🧬️Tune WASM flush timing"])).toBeNull();
    expect(bulletEmojiValidationError(["📊️uloc block"])).toContain("📊️");
  });

  test("normalizeBulletLines enforces compact {emoji}{description} format", async () => {
    const { normalizeBulletLines, formatMicroCommitBulletLine } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(formatMicroCommitBulletLine("- 🐛️ Fix PDF")).toBe("🐛️Fix PDF");
    expect(normalizeBulletLines("🐛️ Fix PDF\n- 🖼️ Tweak UI")).toEqual(["🐛️Fix PDF", "🖼️Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `🎆️item ${i}`).join("\n"))).toHaveLength(8);
  });

  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-message-"));
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msg = buildMicroCommitMessage(root, contributor, ["🎆️LLM-authored change summary"], {
        countRepoByLanguage: () => ({ TypeScript: 1000, Rust: 500 }),
      });
      const lines = msg.trimEnd().split("\n");
      expect(lines[0]).toMatch(/🚩️\d{3}$/);
      expect(lines[1]).toMatch(/^🎆️/);
      expect(lines.some((l) => l.includes("LLM-authored"))).toBe(true);
      expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
      expect(lines.at(-2)).toBe("");
      const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
      const metricsIdx = lines.findIndex((l) => l.startsWith(ulocHeader));
      expect(metricsIdx).toBeGreaterThan(2);
      expect(lines[metricsIdx - 1]).toBe("");
      expect(lines[metricsIdx]?.startsWith(ulocHeader)).toBe(true);
      expect(lines[metricsIdx + 1]).toMatch(/^📊️metric/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildMicroCommitMessage rejects output without the required uloc footer", async () => {
    const { buildMicroCommitMessage } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-empty-uloc-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      expect(() =>
        buildMicroCommitMessage(root, contributor, ["🐛️Reject incomplete commit messages"], {
          countRepoByLanguage: () => ({}),
        }),
      ).toThrow("required 📊️metric footer");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("formatMicroCommitMetricsLines uses compact loc and delta counts", async () => {
    const {
      formatMicroCommitMetricLine,
      formatMicroCommitMetricsLines,
      formatMetricLocCount,
      formatMetricSizeCount,
      formatMetricRatio,
      COMMIT_METRIC_HEADER,
    } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(COMMIT_METRIC_HEADER).toBe("📊️metric");
    expect(formatMetricSizeCount(1024)).toBe("1.02KB");
    expect(formatMetricSizeCount(10_400_000_000)).toBe("10.4GB");
    expect(formatMetricLocCount(200_000)).toBe("200k");
    expect(formatMetricLocCount(1_300_000)).toBe("1.3M");
    expect(formatMetricLocCount(769_000)).toBe("769k");
    expect(formatMetricLocCount(422_377)).toBe("422k");
    expect(formatMetricLocCount(52_759)).toBe("52.8k");
    expect(formatMetricLocCount(500)).toBe("500");
    expect(formatMetricRatio(0.001)).toBe("0.1");
    expect(formatMetricRatio(0.0000854)).toBe("0.009");
    expect(formatMetricRatio(0.0305)).toBe("3.05");
    expect(formatMetricRatio(0.00264)).toBe("0.264");
    expect(formatMetricRatio(0.10061)).toBe("10.1");
    expect(formatMetricRatio(1.21001)).toBe("121");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 0, added: 0, removed: 0 })).toBe("📊️metric🐚️shell📃uloc💯️2k");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 2, removed: 0 })).toBe("📊️metric🐚️shell📃uloc💯️2k📈️2➗️0.1➕️2✏️2🟰️4");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 0, removed: 2 })).toBe("📊️metric🐚️shell📃uloc💯️2k📉️2➗️0.1✏️2➖️2🟰️4");
    const lines = formatMicroCommitMetricsLines([{ lang: "Rust", emoji: "🦀️", code: 200_000, edited: 2220, added: 2000, removed: 500 }]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️200k📈️1.5k➗️0.756➕️2k✏️2.22k➖️500🟰️4.72k");
    expect(lines[1]).toBe("📊️metric🦀️rust📃uloc💯️200k📈️1.5k➗️0.756➕️2k✏️2.22k➖️500🟰️4.72k");
  });

  test("formatMicroCommitMetricsLines totals all languages on the first row", async () => {
    const { formatMicroCommitMetricsLines } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const lines = formatMicroCommitMetricsLines([
      { lang: "TypeScript", emoji: "🟦️", code: 3000, edited: 10, added: 8, removed: 0 },
      { lang: "Markdown", emoji: "📝️", code: 44, edited: 0, added: 0, removed: 0 },
    ]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️3.04k📈️8➗️0.264➕️8✏️10🟰️18");
    expect(lines[1]).toBe("📊️metric🟦️typescript📃uloc💯️3k📈️8➗️0.267➕️8✏️10🟰️18");
    expect(lines[2]).toBe("📊️metric📝️markdown📃uloc💯️44");
  });

  test("buildMicroCommitMetrics merges uloc and git numstat by language", async () => {
    const { buildMicroCommitMetrics } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = process.cwd();
    const metrics = buildMicroCommitMetrics(root, {
      countRepoByLanguage: () => ({ Rust: 100, TypeScript: 50, JSON: 20 }),
    });
    expect(metrics.some((m) => m.lang === "Rust" && m.code === 100)).toBe(true);
  });

  test("isUlocCachePlausible rejects partial caches", async () => {
    const { isUlocCachePlausible, gitRepoRoot } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    const root = gitRepoRoot(import.meta.dir);
    const tracked = spawnSync("git", ["ls-files"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.split("\n").filter(Boolean).length ?? 0;
    try {
      expect(isUlocCachePlausible(root, {})).toBe(false);
      if (tracked <= 1000) return;
      expect(isUlocCachePlausible(root, { TypeScript: 3000, JavaScript: 78, Markdown: 44, JSON: 43 })).toBe(false);
      expect(
        isUlocCachePlausible(root, {
          JSON: 400_000,
          TypeScript: 150_000,
          Go: 90_000,
          Rust: 44_000,
          Markdown: 10_000,
          Python: 30_000,
        }),
      ).toBe(true);
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
    }
  });

  test("shouldSkipPathForUloc skips dot paths license templates and .🦑️repo", async () => {
    const { shouldSkipPathForUloc } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = process.cwd();
    expect(shouldSkipPathForUloc(root, ".cursor/plans/foo.plan.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, ".agents/skills/micro-commit/SKILL.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "compose/client/ui/LICENSE.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "repo/AGENTS.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "repo/CHANGELOG.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, ".🦑️repo/⚡️cache/x")).toBe(true);
    expect(shouldSkipPathForUloc(root, "framework/README.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "puzzle/3d/src/foo.ts")).toBe(false);
  });

  test("countJsonKeys counts nested object keys", async () => {
    const { countJsonKeys } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(countJsonKeys('{"a":1,"b":{"c":2}}')).toBe(3);
  });

  test("appendGitDeltaSuffix formats legacy delta-only suffixes", async () => {
    const { appendGitDeltaSuffix, formatBundleUlocSuffix } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(appendGitDeltaSuffix("🟦️65k", { added: 700, edited: 200, removed: 10 })).toBe("🟦️65k➕️700✏️200➖️10🟰️910");
    expect(formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 }, 65_000)).toBe("📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910");
  });

  test("splitGitNumstatDelta separates replaced lines from net added and removed", async () => {
    const { splitGitNumstatDelta } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(splitGitNumstatDelta(4, 2)).toEqual({ edited: 2, added: 2, removed: 0 });
    expect(splitGitNumstatDelta(2, 4)).toEqual({ edited: 2, added: 0, removed: 2 });
    expect(splitGitNumstatDelta(5, 5)).toEqual({ edited: 5, added: 0, removed: 0 });
    expect(splitGitNumstatDelta(10, 0)).toEqual({ edited: 0, added: 10, removed: 0 });
    expect(splitGitNumstatDelta(0, 7)).toEqual({ edited: 0, added: 0, removed: 7 });
  });

  test("countUnifiedLocForFile uses physical lines for code and keys for json", async () => {
    const { countUnifiedLocForFile } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(countUnifiedLocForFile("x.rs", "// c\nfn main() {}\n")).toBe(3);
    expect(countUnifiedLocForFile("x.json", '{"k":1}')).toBe(1);
  });

  test("classifyPathForMetrics maps TeX ecosystem extensions", async () => {
    const { classifyPathForMetrics, langMetricsEmoji } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(classifyPathForMetrics("mit-bestand/bericht/zwischenbericht/🖋️zwischenbericht.tex")).toBe("TeX");
    expect(classifyPathForMetrics("print/semio.sty")).toBe("TeX");
    expect(classifyPathForMetrics("print/🖋️semio.cls")).toBe("TeX");
    expect(classifyPathForMetrics("report/📚️references.bib")).toBe("TeX");
    expect(classifyPathForMetrics("doc/sample.ltx")).toBe("TeX");
    expect(langMetricsEmoji("TeX")).toBe("📐️");
  });

  test("uncoveredStagedAreas flags missing cursor-plans and product coverage", async () => {
    const { uncoveredStagedAreas } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const staged = [".cursor/plans/brush_fix_cfd8a931.plan.md", "framework/product/playground/renderer/react/index.tsx"];
    expect(uncoveredStagedAreas(["🫡️Only micro-commit skill wording"], staged)).toContain(".cursor/plans");
    expect(uncoveredStagedAreas(["🫡️Only micro-commit skill wording"], staged)).toContain("product");
    const ok = uncoveredStagedAreas(["📋️Plan brush edge resurrection guard and sync", "🖌️Playground renderer restores brush placement after structural deletes"], staged);
    expect(ok).toEqual([]);
  });

  test("validateBulletsAgainstStaged rejects bullets that ignore staged product code", async () => {
    const root = getWorkspaceRoot();
    const stagedHasPresentation = spawnSync("git", ["diff", "--cached", "--name-only"], { cwd: root, encoding: "utf8" }).stdout?.includes("presentation/");
    if (!stagedHasPresentation) return;
    const prev = process.env.REPO_ROOT;
    process.env.REPO_ROOT = root;
    const stdin = ["🫡️Only micro-commit skill docs"].join("\n");
    const r = spawnSync(process.execPath, ["./📜️script.ts", "micro-commit", "prepare"], {
      cwd: root,
      input: stdin,
      encoding: "utf8",
      timeout: 120_000,
    });
    if (prev === undefined) delete process.env.REPO_ROOT;
    else process.env.REPO_ROOT = prev;
    expect(r.status).not.toBe(0);
  }, 120_000);

  test("installMicroCommitGitHooks writes portable hooks and bun pin", async () => {
    const { installMicroCommitGitHooks, renderMicroCommitGitHook } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      const init = spawnSync("git", ["init"], { cwd: root, encoding: "utf8" });
      expect(init.status).toBe(0);
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      expect(hook).toContain("compose_micro_commit_wipe");
      expect(hook).not.toContain("\r");
      expect(existsSync(join(root, ".🧬semio/🦑️repo/compose-micro-commit-bun"))).toBe(true);
      expect(renderMicroCommitGitHook("post-commit")).toContain("#!/usr/bin/env sh");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("handlePrepareCommitMsg inactive does not clear commit message file", async () => {
    const { handlePrepareCommitMsg } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msgFile = join(root, ".git", "COMMIT_EDITMSG");
      writeFileSync(msgFile, "🐙️ueli manual subject\n", "utf8");
      handlePrepareCommitMsg(root, msgFile, "template");
      expect(readFileSync(msgFile, "utf8")).toContain("manual subject");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("wipeAfterCommit clears all GK templates and prepare state", async () => {
    const { wipeAfterCommit, writeMicroCommitTemplates } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync, existsSync, readdirSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-wipe-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "u@example.com"], { cwd: root, env: gitSpawnEnv() });
      spawnSync("git", ["config", "user.name", "U"], { cwd: root, env: gitSpawnEnv() });
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️test reset\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      writeFileSync(join(gitDir, "gkcommittemplate-099.txt"), "stale numbered", "utf8");
      wipeAfterCommit(root);
      const gkTemplate = join(gitDir, "gkcommittemplate.txt");
      expect(existsSync(gkTemplate)).toBe(true);
      expect(readFileSync(gkTemplate, "utf8")).toBe("");
      const gkLeft = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gkLeft).toEqual(["gkcommittemplate.txt"]);
      expect(existsSync(join(gitDir, "compose-micro-commit-active"))).toBe(false);
      expect(readFileSync(join(gitDir, "COMMIT_EDITMSG"), "utf8")).toBe("");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("writeMicroCommitTemplates uses single gkcommittemplate.txt", async () => {
    const { writeMicroCommitTemplates } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, readdirSync, rmSync, readFileSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-tpl-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️bullet\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      const gk = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gk).toEqual(["gkcommittemplate.txt"]);
      expect(readFileSync(join(gitDir, "gkcommittemplate.txt"), "utf8")).toContain("🚩️");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("shouldRefreshPreparedCommitMessage keeps user edits", async () => {
    const { digestMicroCommitMessage, shouldRefreshPreparedCommitMessage } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const prepared = "line1\nline2\n";
    const digest = digestMicroCommitMessage(prepared);
    expect(shouldRefreshPreparedCommitMessage(prepared, digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage("", digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage(`${prepared}\nmy edit`, digest)).toBe(false);
  });
});

describe("playground static sites", () => {
  test("PLAYGROUND_PORTS keeps cad and dag on distinct reserved dev ports", () => {
    expect(playgroundDevPort("cad")).toBe(6020);
    expect(playgroundDevPort("dag")).toBe(6017);
    expect(allPlaygroundReservedPorts().size).toBeGreaterThanOrEqual(Object.keys(PLAYGROUND_PORTS).length);
  });

  test("resolveFrameworkOsPlaygroundPlugin maps CLI segments to OS plugin ids", () => {
    const catalog = loadFrameworkOsPlaygroundCatalog();
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["dag"])).toEqual({ plugin: "dag", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["gis", "2d"])).toEqual({ plugin: "gis2d", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["procedural", "3d", "fixture", "hexagonal-column"])).toEqual({
      plugin: "procedural3d",
      rest: ["fixture", "hexagonal-column"],
    });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["trinity", "jack"])).toEqual({ plugin: "trinity-jack", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["unknown"])).toBeNull();
    const resolvableSegments = catalog.reduce((sum, row) => sum + 1 + row.aliases.length, 0);
    expect(resolvableSegments).toBeGreaterThan(20);
  });

  test("frameworkOsPlaygroundDevEnv defaults wgpu renderer and resolves catalog dev port", () => {
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const dagEnv = frameworkOsPlaygroundDevEnv(catalog, "dag", {}, {});
    expect(dagEnv.SEMIO_RENDERER).toBe("wgpu");
    expect(dagEnv.SEMIO_PLUGIN).toBe("dag");
    expect(dagEnv.S_OS_PORT).toBe("6117");

    const cadEnv = frameworkOsPlaygroundDevEnv(catalog, "cad", {}, { S_OS_PORT: "6020" });
    expect(cadEnv.SEMIO_RENDERER).toBe("wgpu");
    expect(cadEnv.SEMIO_PLUGIN).toBe("cad");
    expect(cadEnv.S_OS_PORT).toBe("6020");
  });

  test("assigns a unique port per dev and test slot", () => {
    const seen = new Set<number>();
    for (const spec of Object.values(PLAYGROUND_PORTS)) {
      expect(seen.has(spec.dev)).toBe(false);
      seen.add(spec.dev);
      if (spec.test !== undefined) {
        expect(seen.has(spec.test)).toBe(false);
        seen.add(spec.test);
      }
    }
  }, 30_000);

  test("playgroundPlayViteDefine embeds locked fixture env", () => {
    const prev = process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
    try {
      delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('""');
      process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = "concrete-forest";
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('"concrete-forest"');
    } finally {
      if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      else process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = prev;
    }
  });

  test("playgroundStaticSiteBuildOptions uses relative-base dist output", () => {
    expect(playgroundStaticSiteBuildOptions()).toEqual({
      target: "esnext",
      outDir: "dist",
      emptyOutDir: true,
    });
    expect(playgroundStaticSiteBuildOptions({ sourcemap: true }).sourcemap).toBe(true);
  });
});

describe("package boundary guards", () => {
  const repoRoot = findRepoRoot();

  test("vite and vitest configs avoid cross-package @semio-tech aliases", () => {
    const offenders: string[] = [];
    const walk = (dir: string) => {
      for (const entry of require("node:fs").readdirSync(dir, { withFileTypes: true })) {
        if (entry.name === "node_modules" || entry.name === ".git" || entry.name === ".🧬semio" || entry.name === "dist" || entry.name === "target" || entry.name === ".claude") continue;
        const full = join(dir, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (!/vite.*\.config\.ts$/.test(entry.name) && entry.name !== "🧪️vitest.config.ts") continue;
        const pkgDir = findNearestPackageDir(full, repoRoot);
        const text = readFileSync(full, "utf8");
        for (const match of text.matchAll(/find:\s*(?:\/\^)?["']@semio-tech\/[^"']+["']/g)) {
          const line = text.slice(0, match.index).split("\n").length;
          const snippet = text.split("\n")[line - 1] ?? "";
          if (/path\.resolve\(__dirname,\s*["']\.\./.test(snippet) && pkgDir && !snippet.includes("node_modules")) {
            offenders.push(`${full.replace(repoRoot + "/", "")}:${line}`);
          }
        }
      }
    };
    walk(repoRoot);
    expect(offenders).toEqual([]);
  });

  test("legacy package aliases are absent from source imports", () => {
    const result = spawnSync("rg", ["-l", "@compose/ui|@ui/react|@elements/", "--glob", "*.{ts,tsx}", "--glob", "!**/.🧬semio/**", "--glob", "!**/🧪️index.test.ts"], {
      cwd: repoRoot,
      encoding: "utf8",
    });
    const files = (result.stdout ?? "")
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    expect(files).toEqual([]);
  });

  test("framework renderer host has no per-technology registerUi surface host APIs", () => {
    const indexPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx");
    const indexSource = readFileSync(indexPath, "utf8");
    expect(indexSource).not.toMatch(/registerUi(?:Draw|Flow|Layout|Note|Puzzle2d|Puzzle3d|Puzzle5d|Sequence|Writer|Raster|Forms|Trinity|Procedural|Shooting|Gis|Cad|Dag|Lowpoly|Imperative|S)SurfaceHost/);
    expect(indexSource).toContain("bootFrameworkOs");
  });
});

function findNearestPackageDir(filePath: string, repoRoot: string): string | undefined {
  let dir = join(filePath, "..");
  while (dir.startsWith(repoRoot)) {
    if (existsSync(join(dir, "package.json"))) return dir;
    const parent = join(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  return undefined;
}

describe("commit", () => {
  test("parseCommitBundleBody reads emoji scopes dates and bullets", async () => {
    const { parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = parseCommitBundleBody("🏘️compose✍️sketchpad\n🎆️26🌙️06☀️04\n🗺️Map work\n🎆️26🌙️06☀️03\n🧪️Playground\n\n🖱️ui⚛️react\n🎆️26🌙️06☀️02\n🖥️Shell");
    expect(bundles).toHaveLength(2);
    expect(bundles[0]?.label).toBe("🏘️compose✍️sketchpad");
    expect(bundles[0]?.dates).toHaveLength(2);
    expect(bundles[0]?.dates[0]?.bullets[0]).toBe("🗺️Map work");
  });

  test("parseCommitBundleBody rejects path prefixes and reserved emojis", async () => {
    const { parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(() => parseCommitBundleBody("compose/foo|🏘️compose\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🏘️compose🔀️📊️metric\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🗺️🧩️🕸️\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
  });

  test("normalizeBundleScopeLabel strips reserved and uloc suffix", async () => {
    const { normalizeBundleScopeLabel } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(normalizeBundleScopeLabel("🏘️compose🔀️📊️metric📃uloc➕️1")).toBe("🏘️compose");
  });

  test("isBundleScopeLine accepts area and technology root labels", async () => {
    const { isBundleScopeLine } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(isBundleScopeLine("🌐️gis📍️map")).toBe(true);
    expect(isBundleScopeLine("🖱️ui⚛️react")).toBe(true);
    expect(isBundleScopeLine("🥅️framework")).toBe(true);
    expect(isBundleScopeLine("🧪️Playground")).toBe(false);
    expect(isBundleScopeLine("🗺️Single emoji line")).toBe(false);
  });

  test("extractBundleDateLineFromSubject reads calendar day from micro-commit subject", async () => {
    const { extractBundleDateLineFromSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(extractBundleDateLineFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️012")).toBe("🎆️26🌙️06☀️04");
    expect(extractBundleDateLineFromSubject("unrelated")).toBeNull();
  });

  test("extractBundleDateLineFromCommit prefers body timestamp over subject checkpoint day", async () => {
    const { extractBundleDateLineFromCommit, extractBundleDateLineFromCommitBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const body = "🎆️26🌙️06☀️04⏰️02⌚️38⏱️38\n🗺️Map work\n";
    expect(extractBundleDateLineFromCommitBody(body)).toBe("🎆️26🌙️06☀️04");
    expect(extractBundleDateLineFromCommit("🐙️ueli🎆️26🌙️06☀️02🚩️084", body)).toBe("🎆️26🌙️06☀️04");
  });

  test("pathsFromNumstatRow expands rename paths", async () => {
    const { pathsFromNumstatRow } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(pathsFromNumstatRow("old/a.ts\tnew/b.ts")).toEqual(["old/a.ts", "new/b.ts"]);
    expect(pathsFromNumstatRow("dir/{old.ts => new.ts}")).toEqual(["old.ts", "new.ts"]);
  });

  test("pathMatchesBundleIndex does not treat empty prefix set as match-all", async () => {
    const { pathMatchesBundleIndex } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = [
      { label: "🏘️compose✍️sketchpad", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["✍️x"] }] },
      { label: "🏘️compose🗃️fixtures", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗃️y"] }] },
    ];
    const prefixSets = [[], ["compose/fixture"]];
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 0, prefixSets, bundles)).toBe(false);
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 1, prefixSets, bundles)).toBe(true);
  });

  test("formatBundleDateLine appends per-day uloc suffix", async () => {
    const { formatBundleDateLine } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(formatBundleDateLine("🎆️26🌙️06☀️04", { added: 700, edited: 200, removed: 10 }, { added: 788_000, edited: 0, removed: 0 }, 65_000, 10_400_000_000)).toBe(
      "🎆️26🌙️06☀️04📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910📊️metric💾size💯️10.4GB📈️788KB➗️0.008➕️788KB🟰️788KB",
    );
  });

  test("commitBundleBodyError rejects per-day uloc on stdin", async () => {
    const { commitBundleBodyError } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(commitBundleBodyError("🏘️compose\n🎆️26🌙️06☀️04📊️metric📃uloc➕️1\n🗺️Work")).toMatch(/per-day/);
  });

  test("validateMicroCommitLangMetricsDeltaSum passes when language rows sum to footer", async () => {
    const { validateMicroCommitLangMetricsDeltaSum } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(() =>
      validateMicroCommitLangMetricsDeltaSum([
        { lang: "TypeScript", emoji: "🟦️", code: 100, edited: 5, added: 3, removed: 0 },
        { lang: "Rust", emoji: "🦀️", code: 50, edited: 2, added: 1, removed: 1 },
      ]),
    ).not.toThrow();
  });

  test("validateBundleCommitAttribution requires bundle headers to sum to range total", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { validateBundleCommitAttribution, parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-check-sum-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      mkdirSync(join(root, "other"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙️ueli🎆️26🌙️06☀️01🔀️"], { cwd: root });
      const wip = fixtureGitHead(root);
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\nc\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      const msg = join(root, "mc.txt");
      writeFileSync(msg, "🐙️ueli🎆️26🌙️06☀️01🚩️001\n\n🎆️26🌙️06☀️04⏰️12⌚️00⏱️00\n🔧️Work\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg], { cwd: root });
      const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Only repo");
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(/not attributed to any bundle|do not add up/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleDayDeltasAttribution rejects unlisted micro-commit day", async () => {
    const { validateBundleDayDeltasAttribution } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = [{ label: "📚️repo🔧️js", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🔧️Net"] }] }];
    const prefixSets = [["repo/js/"]];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆️26🌙️06☀️03", { added: 4, removed: 0, edited: 0 }],
          ["🎆️26🌙️06☀️04", { added: 0, removed: 4, edited: 0 }],
        ]),
      ],
    ]);
    const bundleTotals = [{ added: 0, removed: 0, edited: 0 }];
    expect(() => validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, bundleTotals)).toThrow(/missing from your bundle body/);
  });

  test("validateBundleDayDeltasAttribution rejects when listed days do not sum to bundle total", async () => {
    const { validateBundleDayDeltasAttribution } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = [
      {
        label: "📚️repo🔧️js",
        dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🔧️Only day four listed"] }],
      },
    ];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆️26🌙️06☀️04", { added: 2, edited: 0, removed: 0 }],
          ["🎆️26🌙️06☀️03", { added: 5, edited: 0, removed: 0 }],
        ]),
      ],
    ]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltas, [{ added: 2, edited: 0, removed: 0 }])).toThrow(/missing from your bundle body/);
    const dateDeltasOneDay = new Map([[0, new Map([["🎆️26🌙️06☀️04", { added: 2, edited: 0, removed: 0 }]])]]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltasOneDay, [{ added: 7, edited: 0, removed: 0 }])).toThrow(/does not add up/);
  });

  test("buildCommitMessage appends per-day uloc from micro-commit dates", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { buildCommitMessage, parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-day-uloc-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙️ueli🎆️26🌙️06☀️01🔀️"], { cwd: root });
      const wip = fixtureGitHead(root);
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg1 = join(tmpdir(), `compose-mc1-${Date.now()}.txt`);
      writeFileSync(msg1, "🐙️ueli🎆️26🌙️06☀️01🚩️001\n\n🎆️26🌙️06☀️03⏰️12⌚️00⏱️00\n🔧️Day three\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg1], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\nd\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg2 = join(tmpdir(), `compose-mc2-${Date.now()}.txt`);
      writeFileSync(msg2, "🐙️ueli🎆️26🌙️06☀️01🚩️002\n\n🎆️26🌙️06☀️04⏰️12⌚️01⏱️00\n🔧️Day four\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg2], { cwd: root });
      const contributor = { alias: "ueli", emoji: "🐙️", name: "U", email: "u@e.com" };
      const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Day four\n🎆️26🌙️06☀️03\n🔧️Day three");
      const msg = buildCommitMessage(root, contributor, bundles, wip, "HEAD");
      expect(msg).toMatch(/🎆️26🌙️06☀️04📊️metric/);
      expect(msg).toMatch(/🎆️26🌙️06☀️03📊️metric/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("sortCommitBundlesByEditTotal orders bundles by descending gitDeltaLineTotal", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { sortCommitBundlesByEditTotal } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const mk = (label: string) => ({
      label,
      dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗺️change"] }],
    });
    const root = mkdtempSync(join(tmpdir(), "compose-commit-sort-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "compose"), { recursive: true });
      mkdirSync(join(root, "ui"), { recursive: true });
      mkdirSync(join(root, "framework"), { recursive: true });
      writeFileSync(join(root, "compose/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "ui/b.ts"), "b\n", "utf8");
      writeFileSync(join(root, "framework/c.ts"), "c\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "init"], { cwd: root });
      const base = fixtureGitHead(root);
      writeFileSync(join(root, "compose/a.ts"), `${"a\n".repeat(11)}`, "utf8");
      writeFileSync(join(root, "ui/b.ts"), `${"b\n".repeat(151)}`, "utf8");
      writeFileSync(join(root, "framework/c.ts"), `${"c\n".repeat(5)}`, "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "delta"], { cwd: root });
      const head = fixtureGitHead(root);
      const bundles = [mk("🏘️compose"), mk("🖱️ui"), mk("🥅️framework")];
      const paths = [["compose/a.ts"], ["ui/b.ts"], ["framework/c.ts"]];
      const sorted = sortCommitBundlesByEditTotal(root, base, head, bundles, paths);
      expect(sorted.bundles.map((b) => b.label)).toEqual(["🖱️ui", "🏘️compose", "🥅️framework"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildCommitMessage renders bundle subject and footer", async () => {
    const { buildCommitMessage, parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Tooling");
    const root = mkdtempSync(join(tmpdir(), "semio-bundle-message-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo", "js", "a.ts"), "export const a = 1;\n");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "fixture"], { cwd: root });
      const head = fixtureGitHead(root);
      const msg = buildCommitMessage(root, contributor, bundles, head, head, { countRepoByLanguage: () => ({ TypeScript: 1000 }) });
      const lines = msg.trimEnd().split("\n");
      expect(lines[0]).toMatch(/🔀️$/);
      expect(lines.some((l) => l.includes("📊️metric"))).toBe(true);
      expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("formatBundleTagName and formatBundleSubject use contributor date emojis", async () => {
    const { formatBundleTagName, formatBundleSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const c = { alias: "ueli", emoji: "🐙️", name: "U", email: "u@e.com" };
    const now = new Date("2026-06-04T12:00:00");
    expect(formatBundleTagName(c, now)).toBe("🐙️ueli🎆️26🌙️06☀️04🚩️");
    expect(formatBundleSubject(c, now)).toBe("🐙️ueli🎆️26🌙️06☀️04🔀️");
  });

  test("formatCommitPrepareCommands emits four fenced git blocks", async () => {
    const { formatCommitPrepareCommands } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const out = formatCommitPrepareCommands({
      tagName: "🐙️ueli🎆️26🌙️06☀️04🚩️",
      wipSha: "abc123def456",
      messageFile: ".git/compose-commit-message",
    });
    const blocks = out.trimEnd().split("\n\n");
    expect(blocks).toHaveLength(4);
    expect(blocks[0]).toBe("```\ngit tag -s -m '🐙️ueli🎆️26🌙️06☀️04🚩️' '🐙️ueli🎆️26🌙️06☀️04🚩️' HEAD\n```");
  });

  test("formatCommitPrepareAgentReply ends with tag name and commit message blocks", async () => {
    const { formatCommitPrepareAgentReply } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const commitMessage = "🐙️ueli🎆️26🌙️06☀️04🔀️\n\n🏘️compose✍️sketchpad📊️metric\n🎆️26🌙️06☀️04\n🗺️Work\n\n📊️metric📃uloc➕️1🟰️1\n\nSigned-off-by: U <u@e.com>\n";
    const out = formatCommitPrepareAgentReply({
      tagName: "🐙️ueli🎆️26🌙️06☀️04🚩️",
      wipSha: "abc",
      commitMessage,
    });
    const fenceBodies = [...out.matchAll(/```\n([\s\S]*?)\n```/g)].map((m) => m[1]!);
    expect(fenceBodies).toHaveLength(6);
    expect(fenceBodies[4]).toBe("🐙️ueli🎆️26🌙️06☀️04🚩️");
    expect(fenceBodies[5]).toBe(commitMessage.trimEnd());
  });

  test("parseCommitSteps treats cs as squash without tag", async () => {
    const { parseCommitSteps } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(parseCommitSteps(["cs"])).toEqual({ tag: false, squash: true, push: false });
    expect(parseCommitSteps(["ct", "cs", "cp"])).toEqual({ tag: true, squash: true, push: true });
  });

  test("bulletMatchesCommitHistory detects verbatim prior commit lines", async () => {
    const { bulletMatchesCommitHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const history = new Set(["🗺️copied line from an old micro-commit"]);
    expect(bulletMatchesCommitHistory("🗺️copied line from an old micro-commit", history)).toBe(true);
    expect(bulletMatchesCommitHistory("🗺️fresh summary written from git diff", history)).toBe(false);
  });
});

describe("command budgets", () => {
  const indexPath = join(import.meta.dir, "📦️index.ts");

  /** ⏱️Runs `runCmd` in a fresh subprocess (its budget-exceeded path calls `process.exit`, which would kill the test runner in-process) against a child that sleeps far longer than its budget. */
  function spawnBudgetedSleep(budgetMs: number, envOverride?: Record<string, string>): ReturnType<typeof runProbe> {
    const script = `const { runCmd } = await import(${JSON.stringify(indexPath)}); runCmd("bun", ["-e", "await Bun.sleep(10000)"], ${budgetMs === -1 ? "{}" : `{ budgetMs: ${budgetMs} }`});`;
    return runProbe("bun", ["-e", script], { budgetMs: 5_000, env: { ...process.env, ...envOverride } });
  }

  test("captured Cargo failures replay structured compiler errors without warning floods", () => {
    const warning = JSON.stringify({ reason: "compiler-message", message: { level: "warning", rendered: "warning: noisy dependency" } });
    const error = JSON.stringify({ reason: "compiler-message", message: { level: "error", rendered: "error[E0609]: exact field failure\n" } });
    expect(capturedTestFailureDiagnostics(`${warning}\n${error}\n`, "warning: stderr noise")).toBe("error[E0609]: exact field failure\n");
  });

  test("captured Cargo failures retain unrendered structured errors and stderr causes", () => {
    const error = JSON.stringify({
      reason: "compiler-message",
      message: {
        level: "error",
        message: "missing schema attribute",
        spans: [{ file_name: "src/schema.rs", line_start: 17, column_start: 4, is_primary: true }],
      },
    });
    expect(capturedTestFailureDiagnostics(error, "warning: noise\nCaused by: fixture failed")).toBe(
      "missing schema attribute\n  --> src/schema.rs:17:4\nCaused by: fixture failed",
    );
  });

  test("kills a command that exceeds its explicit budget", () => {
    const start = Date.now();
    const result = spawnBudgetedSleep(250);
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
    expect(Date.now() - start).toBeLessThan(5000);
  });

  test("SEMIO_CMD_BUDGET_MS env override kills a command with no explicit budget", () => {
    const result = spawnBudgetedSleep(-1, { SEMIO_CMD_BUDGET_MS: "250" });
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
  });

  test("orchestratorBudgetOpts supplies a bounded orchestrator budget", () => {
    expect(orchestratorBudgetOpts()).toEqual({ budgetMs: orchestratorBudgetMs() });
    expect(() => runCmd(process.execPath, ["-e", "1"], orchestratorBudgetOpts())).not.toThrow();
  });

  test("daemonBudgetOpts returns the daemon budget class", () => {
    expect(daemonBudgetOpts()).toEqual({ budgetMs: daemonBudgetMs() });
    expect(daemonBudgetMs()).toBe(DAEMON_BUDGET_MS);
    expect(orchestratorBudgetMs()).toBe(ORCHESTRATOR_BUDGET_MS);
  });

  test("goLevelTestArgs includes -timeout derived from the level budget", () => {
    const args = goLevelTestArgs("fundamental");
    expect(args[0]).toBe("-timeout");
    expect(args[1]).toMatch(/^\d+s$/);
    expect(args[1]).toBe(`${Math.ceil(testLevelBudgetMs("fundamental") / 1000)}s`);
  });

  test("vitestLevelArgs returns per-test timeout flags", () => {
    const ms = String(testLevelBudgetMs("quick"));
    expect(vitestLevelArgs("quick")).toEqual(["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms]);
  });

  test("orchestratorBudgetMs defaults to 4h and honors SEMIO_ORCHESTRATOR_BUDGET_MS", () => {
    expect(orchestratorBudgetMs()).toBe(4 * 60 * 60 * 1000);
    const prev = process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
    process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = "12345";
    try {
      expect(orchestratorBudgetMs()).toBe(12345);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
      else process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = prev;
    }
  });

  test("daemonBudgetMs defaults to 24h and honors SEMIO_DAEMON_BUDGET_MS", () => {
    expect(daemonBudgetMs()).toBe(24 * 60 * 60 * 1000);
    const prev = process.env.SEMIO_DAEMON_BUDGET_MS;
    process.env.SEMIO_DAEMON_BUDGET_MS = "67890";
    try {
      expect(daemonBudgetMs()).toBe(67890);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_DAEMON_BUDGET_MS;
      else process.env.SEMIO_DAEMON_BUDGET_MS = prev;
    }
  });

  test("testLevelBudgetMs maps levels and honors SEMIO_TEST_BUDGET_MS", async () => {
    const { TEST_LEVEL_BUDGET_MS } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(testLevelBudgetMs("fundamental")).toBe(TEST_LEVEL_BUDGET_MS.fundamental);
    expect(testLevelBudgetMs("exhaustive")).toBe(TEST_LEVEL_BUDGET_MS.exhaustive);
    const prev = process.env.SEMIO_TEST_BUDGET_MS;
    process.env.SEMIO_TEST_BUDGET_MS = "42000";
    try {
      expect(testLevelBudgetMs("quick")).toBe(42000);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_TEST_BUDGET_MS;
      else process.env.SEMIO_TEST_BUDGET_MS = prev;
    }
  });

  test("runProbe captures stdout under budget", () => {
    const result = runProbe("bun", ["-e", "console.log('probe-ok')"]);
    const { status, stdout } = result;
    expect(status).toBe(0);
    expect(stdout.trim()).toBe("probe-ok");
  });

  test("runCmdStatus returns the exit status instead of throwing", () => {
    expect(runCmdStatus(process.execPath, ["-e", "process.exit(3)"])).toBe(3);
  });

  test("budgetTimeoutHint defaults cargo to the target-dir lock-contention hint", () => {
    expect(budgetTimeoutHint("cargo")).toContain("target-dir lock contention");
  });

  test("budgetTimeoutHint honors an explicit override", () => {
    expect(budgetTimeoutHint("git", "custom hint")).toBe("custom hint");
  });

  test("budgetTimeoutHint gives non-cargo commands a generic hint", () => {
    expect(budgetTimeoutHint("git")).not.toContain("target-dir lock contention");
  });
});

//#region 🦀️NextestExecutionFilters
describe("nextest execution filters", () => {
  test("retains explicit task artifacts and preserves the default temporary location", () => {
    const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🦀️nextest/🔣️artifact-location.json"), "utf8")) as { schema: string; cases: Array<{ value: string; relative: string | null; retain: boolean }> };
    const validate = new Ajv({ strict: true }).compile({
      type: "object", additionalProperties: false, required: ["schema", "cases"],
      properties: {
        schema: { const: "nextest-artifact-location/v1" },
        cases: { type: "array", minItems: 4, items: { type: "object", additionalProperties: false, required: ["value", "relative", "retain"], properties: { value: { type: "string" }, relative: { type: ["string", "null"] }, retain: { type: "boolean" } } } },
      },
    });
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of fixture.cases) {
      expect(nextestArtifactLocation(import.meta.dir, { SEMIO_TEST_ARTIFACT_DIR: row.value })).toEqual({ directory: row.relative === null ? tmpdir() : resolve(import.meta.dir, row.relative), retain: row.retain });
    }
  });

  test("preserves language-neutral build and execution vectors with an independent Node parser", () => {
    const fixtureRoot = join(import.meta.dir, "../../🧪️tests/🦀️nextest");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️command-vectors.json"), "utf8")) as {
      cases: Array<{ name: string; input: string[]; build: string[]; execution: string[]; libtest: string[] }>;
      rejected: string[][];
    };
    const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "🔣️schema.json"), "utf8")));
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const runtimeKeys = ["filter-expr", "partition", "run-ignored", "ignore-default-filter"];
    for (const row of fixture.cases) {
      const options = {
        lib: { type: "boolean" }, release: { type: "boolean" }, "no-default-features": { type: "boolean" }, package: { type: "string", short: "p", multiple: true }, features: { type: "string", short: "F", multiple: true }, target: { type: "string" }, "target-dir": { type: "string" }, "build-jobs": { type: "string" }, "cargo-profile": { type: "string" }, "cargo-message-format": { type: "string", multiple: true }, timings: { type: row.input.some((arg) => arg.startsWith("--timings=")) ? "string" : "boolean" }, config: { type: "string", multiple: true }, Z: { type: "string", short: "Z", multiple: true }, test: { type: "string", multiple: true }, bin: { type: "string", multiple: true }, bench: { type: "string", multiple: true }, example: { type: "string", multiple: true },
        "filter-expr": { type: "string", short: "E", multiple: true }, partition: { type: "string" }, "run-ignored": { type: "string" }, "ignore-default-filter": { type: "boolean" },
      } as const;
      const actual = partitionNextestExecutionFilters(row.input);
      expect(actual, row.name).toEqual({ buildArgs: row.build, executionArgs: row.execution, libtestArgs: row.libtest });
      const separator = row.input.indexOf("--");
      const original = parseArgs({ args: separator < 0 ? row.input : row.input.slice(0, separator), options, strict: true, allowPositionals: true });
      const runtime = parseArgs({ args: actual.executionArgs, options, strict: true, allowPositionals: true });
      const build = parseArgs({ args: actual.buildArgs, options, strict: true, allowPositionals: true });
      expect(runtime.values, row.name).toEqual(Object.fromEntries(Object.entries(original.values).filter(([key]) => runtimeKeys.includes(key))));
      expect(build.values, row.name).toEqual(Object.fromEntries(Object.entries(original.values).filter(([key]) => !runtimeKeys.includes(key))));
      expect(runtime.positionals, row.name).toEqual(original.positionals);
      expect(build.positionals, row.name).toEqual([]);
    }
    for (const args of fixture.rejected) expect(() => partitionNextestExecutionFilters(args)).toThrow();
  });
});
//#endregion 🦀️NextestExecutionFilters

describe("resolveCargoPackageName", () => {
  test("resolves short lib names to full package names", () => {
    const root = process.cwd();
    expect(resolveCargoPackageName("db", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust"))).toBe("semio-framework-os-kernel-db");
    expect(resolveCargoPackageName("semio-s-plugin-architect", join(root, "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust"))).toBe("semio-s-plugin-architect");
    expect(resolveCargoPackageName("semio-s-plugin-energy", join(root, "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust"))).toBe("semio-s-plugin-energy");
  });

  test("resolves empty package list to local Cargo.toml package name", () => {
    const root = process.cwd();
    const dbDir = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust");
    expect(resolveCargoPackageNames([], dbDir)).toEqual(["semio-framework-os-kernel-db"]);
  });
});

describe("loadTaxonomy", () => {
  test("parses 🔣️taxonomy.json into the expected shape", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.artifactComponentDirs).toEqual(["🧬️schema", "🚪️io"]);
    expect(taxonomy.mutationComponentFileKindId).toBe("rust-source");
    expect(taxonomy.mutationOptionalFacetDirs).toEqual(["🔺️diff", "↩️inverse", "🧩️plan", "📝️text", "💾️binary", "🧬️schema"]);
    expect(taxonomy.schemaChildDirs).toEqual(["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"]);
    expect(taxonomy.representationDirs).toEqual(["📝️text", "💾️binary"]);
    expect(taxonomy.schemaFormats).toEqual({
      "🦀️rust": { fileKindId: "rust-source", fieldCasing: "snake" },
      "🟦️typescript": { fileKindId: "typescript-source", fieldCasing: "camel" },
      "🔗️graphql": { fileKindId: "graphql", fieldCasing: "camel" },
      "🔣️jsonschema": { fileKindId: "json", fieldCasing: "camel" },
      "🛰️protobuf": { fileKindId: "protobuf", fieldCasing: "snake" },
      "📜️wit": { fileKindId: "wit", fieldCasing: "kebab" },
    });
    expect(taxonomy.artifactSchemaSpecFileKinds["🧬️schema"]).toBe("json");
    expect(taxonomy.artifactSchemaSpecFileKinds["🧬️schema/📸️snapshot"]).toBe("json");
    expect(taxonomy.artifactSchemaSpecFileKinds["🧬️schema/🔺️diff"]).toBe("json");
    expect(taxonomy.artifactSpecFileKinds["🧬️schema/📸️snapshot/💾️binary"]).toBe("protocol-semio");
    expect("🎒️pack" in taxonomy.artifactSpecFileKinds).toBe(false);
    expect(taxonomy.windowChildDirs).toEqual(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.windowRequiredChildDirs).toEqual(["🎬️actions", "🪛️utilities", "🎚️options", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.modeChildDirs).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.modeRequiredChildDirs).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.pluginRequiredChildDirs).toEqual(["🎮️commands"]);
    expect(taxonomy.osRequiredChildDirs).toEqual(["🎮️commands"]);
    expect(taxonomy.surfaceRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.transientChildDirs).toEqual(["🧬️schema"]);
    expect(taxonomy.surfaceRequiredChildDirs).toContain("🫧️transient");
    expect(taxonomy.surfaceChildDirs).toContain("🫧️transient");
    expect(taxonomy.surfaceSchemaSpecFileKinds["🫧️transient/🧬️schema"]).toBe("json");
    expect(taxonomy.windowComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowEmptyFacetFileKindId).toBe("markdown");
    expect(taxonomy.componentFileKinds["🦀️rust"]).toBe("rust-source");
    expect(taxonomy.componentFileKinds["🟦️typescript"]).toBe("typescript-source");
    expect(taxonomy.artifactSpecFileKinds["🧬️schema/📸️snapshot/📝️text"]).toBe("grammar-semio");
    expect(taxonomy.artifactSpecFileKinds["🧬️schema/📸️snapshot/💾️binary"]).toBe("protocol-semio");
    expect(taxonomy.libWiringLineBudget).toBe(150);
    expect(taxonomy.packagesDirName).toBe("📦️packages");
    expect(Object.keys(taxonomy.areas).length).toBeGreaterThan(0);
  });

  test("closes generator ownership and covers the exact platform-inventoried Ralph surface", () => {
    const taxonomy = loadTaxonomy();
    const tracked = ownedFilePaths(join(getWorkspaceRoot(), ".ralph-tui")).map((path) => `.ralph-tui/${path}`);
    const setup = taxonomy.generatorContracts["setup-wizard-config"]!;
    expect(Object.values(taxonomy.generatorContracts).every((contract) => contract.ownership === "owned" || contract.ownership === "external")).toBe(true);
    expect(taxonomy.generatorContracts["ownerless-ui-icons"]).toBeUndefined();
    expect(taxonomy.generatorContracts["root-layering-declarations"]).toBeUndefined();
    expect(setup.ownership).toBe("external");
    expect(setup.outputRoots.map((output) => output.path)).toEqual(tracked);
    expect(setup.outputRoots.every((output) => output.inclusion === "tracked")).toBe(true);
    for (const path of tracked) expect(fixedFilenameContractIdsForPath(path, taxonomy)[0]?.startsWith("ralph-")).toBe(true);
    expect(fixedDirectoryContractIdsForPath(".ralph-tui", taxonomy)[0]).toBe("ralph-metadata");
    expect(fixedDirectoryContractIdsForPath(".ralph-tui/prd", taxonomy)[0]).toBe("ralph-prd-root");
    expect(fixedDirectoryContractIdsForPath(".ralph-tui/prd/dynamic-prd-id", taxonomy)[0]).toBe("ralph-prd-identifier");
  });

  test("scopes Cargo target triples and adjacent Nx manifests through exact owner identities", () => {
    const taxonomy = loadTaxonomy();
    const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/TICKET";
    for (const triple of ["wasm32-unknown-unknown", "wasm32-wasip2"] as const) {
      const directoryId = `cargo-target-triple-${triple}`;
      const cacheId = `cargo-cache-tag-${triple}`;
      const target = `${ticket}/🧪️target-probe/${triple}`;
      expect(fixedDirectoryContractIdsForPath(target, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([directoryId]);
      expect(fixedFilenameContractIdsForPath(`${target}/CACHEDIR.TAG`, taxonomy, { parentFixedDirectoryContractIds: [directoryId] })).toEqual([cacheId]);
      expect(fixedDirectoryContractIdsForPath(`tmp/🧪️target-probe/${triple}`, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([]);
    }
    expect(fixedDirectoryContractIdsForPath(`${ticket}/🧪️target-probe/wasm32-wasi`, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([]);
    const nxRoot = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react";
    expect(fixedFilenameContractIdsForPath(`${nxRoot}/package.json`, taxonomy, { siblingFixedFilenameContractIds: ["nx-project-manifest"] })).toEqual(["nx-owned-node-package-manifest"]);
    expect(fixedFilenameContractIdsForPath(`${nxRoot}/tsconfig.json`, taxonomy, { siblingFixedFilenameContractIds: ["nx-project-manifest"] })).toEqual(["nx-owned-typescript-config"]);
    expect(fixedFilenameContractIdsForPath(`${nxRoot}/package.json`, taxonomy)).toEqual([]);
    expect(fixedFilenameContractIdsForPath(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️15/BOARD-REACT-RECONCILER/_tmp/package.json", taxonomy)).toEqual([]);
    const broadCargo = structuredClone(taxonomy);
    broadCargo.fixedDirectoryContracts["cargo-target-triple-wasm32-wasip2"]!.pathPattern = "**/wasm32-wasip2";
    expect(validateTaxonomy(broadCargo).some((problem) => problem.includes("cargo-target-triple-wasm32-wasip2") && problem.includes("exact governed ticket"))).toBe(true);
    const unscopedNx = structuredClone(taxonomy);
    unscopedNx.fixedFilenameContracts["nx-owned-node-package-manifest"]!.scope = { kind: "path-pattern" };
    expect(validateTaxonomy(unscopedNx).some((problem) => problem.includes("nx-owned-node-package-manifest") && problem.includes("adjacent exact Nx"))).toBe(true);
  });

  test("validates the exact transaction edit preparation directory and sole rendered candidate", () => {
    const taxonomy = loadTaxonomy();
    const directory = "🚧️edit-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    for (const leafNames of [[], ["0123456789abcdef01234567.edit"], ["0123456789abcdef01234567.edit", "0123456789abcdef01234567.pre"], ["0123456789abcdef01234567.pre"]]) expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames, writePreparations: [] }, taxonomy)).toEqual([]);
    expect(scopedFileKindIdForSourcePath(`🚧️stage/${directory}/0123456789abcdef01234567.edit`, taxonomy, { parentDirectoryKindId: "transaction-edit-preparation" })).toBe("transaction-edit-candidate");
    expect(scopedFileKindIdForSourcePath(`🚧️stage/${directory}/0123456789abcdef01234567.pre`, taxonomy, { parentDirectoryKindId: "transaction-edit-preparation" })).toBe("transaction-edit-preimage");
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-backup", directoryName: directory, leafNames: ["0123456789abcdef01234567.edit"], writePreparations: [] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames: ["ffffffffffffffffffffffff.edit"], writePreparations: [] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames: ["0123456789abcdef01234567.pre", "ffffffffffffffffffffffff.pre"], writePreparations: [] }, taxonomy)).not.toEqual([]);
    const write = "🚧️write-42-123e4567-e89b-42d3-a456-426614174000";
    expect(taxonomyCliEditWritePreparationProblems({ parentKindId: "transaction-edit-preparation", directoryName: write, leafNames: ["🚧️.edit"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames: [], writePreparations: [{ directoryName: write, leafNames: ["🚧️.edit"] }] }, taxonomy)).toEqual([]);
    const backup = "🚧️backup-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    expect(taxonomyCliBackupWritePreparationProblems({ parentKindId: "transaction-backup-preparation", directoryName: write, leafNames: ["🚧️.backup"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliBackupPreparationProblems({ parentKindId: "transaction-backup", directoryName: backup, leafNames: ["0123456789abcdef01234567.backup"], writePreparations: [{ directoryName: write, leafNames: [] }] }, taxonomy)).toEqual([]);
  });

  test("validates every attempt preparation sibling before recovery", () => {
    const taxonomy = loadTaxonomy();
    const first = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", children: [{ name: "🚧️stage", nodeKind: "directory" }, { name: "💾️backup", nodeKind: "directory" }, { name: "🔒️lease", nodeKind: "directory" }, { name: "🔣️.json", nodeKind: "file" }] } as const;
    const second = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000002-43-223e4567-e89b-42d3-a456-426614174000", children: [] } as const;
    expect(taxonomyCliAttemptPreparationsProblems([first, second], taxonomy)).toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([first, first], taxonomy)).not.toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([{ ...second, directoryName: "🚧️prepare-malformed" }], taxonomy)).not.toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([{ ...second, children: [{ name: "foreign", nodeKind: "directory" }] }], taxonomy)).not.toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([{ ...second, children: [{ name: "🚧️write-43-223e4567-e89b-42d3-a456-426614174000", nodeKind: "directory" }] }], taxonomy)).not.toEqual([]);
  });

  test("validates restore exchange states and exact JSON write preparations", () => {
    const taxonomy = loadTaxonomy();
    const restore = "🚧️restore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    for (const leaves of [[], ["0123456789abcdef01234567.backup"], ["0123456789abcdef01234567.backup", "0123456789abcdef01234567.post"], ["0123456789abcdef01234567.post"]]) {
      expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-backup", directoryName: restore, leafNames: leaves }, taxonomy)).toEqual([]);
    }
    expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-backup", directoryName: restore, leafNames: ["ffffffffffffffffffffffff.backup"] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-stage", directoryName: restore, leafNames: [] }, taxonomy)).not.toEqual([]);
    const write = "🚧️write-42-123e4567-e89b-42d3-a456-426614174000";
    for (const leafNames of [[], ["🔣️.json"], ["🔣️.json", "⏮️.json"], ["⏮️.json"]]) expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-journal-write", directoryName: write, leafNames }, taxonomy)).toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-lease-preparation", directoryName: write, leafNames: ["🔣️.json"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-lease", directoryName: write, leafNames: ["🔣️.json"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliLeaseDirectoryProblems({ parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", leafNames: ["🔣️.json"], writePreparations: [] }, taxonomy)).toEqual([]);
    expect(taxonomyCliLeaseDirectoryProblems({ parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-stale", leafNames: [], writePreparations: [{ directoryName: write, leafNames: ["🔣️.json", "⏮️.json"] }] }, taxonomy)).toEqual([]);
    expect(taxonomyCliLeaseDirectoryProblems({ parentKindId: "transaction-attempt", directoryName: "🔒️lease", leafNames: [], writePreparations: [] }, taxonomy)).not.toEqual([]);
    expect(scopedFileKindIdForSourcePath(`🚧️journal/${write}/⏮️.json`, taxonomy, { parentDirectoryKindId: "transaction-json-write-preparation" })).toBe("transaction-json-previous");
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-stage", directoryName: write, leafNames: ["🔣️.json"] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-journal-write", directoryName: write, leafNames: ["🔣️.json", "partial.json"] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-journal-write", directoryName: write, leafNames: ["⏮️.json", "⏮️.json"] }, taxonomy)).not.toEqual([]);
  });

  test("requires every owned generator to expose one exact read-only preview target", () => {
    const taxonomy = loadTaxonomy();
    const owned = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => contract.ownership === "owned");
    const external = Object.values(taxonomy.generatorContracts).filter((contract) => contract.ownership === "external");
    expect(owned).toHaveLength(14);
    for (const [, contract] of owned) {
      const project = contract.target!.slice(0, contract.target!.lastIndexOf(":"));
      expect(contract.previewTarget).toBe(`${project}:preview-generated`);
      expect(generatorNxPreviewCommand(contract)).toEqual(["bun", "nx", "run", `${project}:preview-generated`]);
    }
    expect(external.every((contract) => contract.previewTarget === undefined)).toBe(true);
    expect(validateGeneratorContractsAgainstWorkspace(getWorkspaceRoot(), taxonomy)).toEqual([]);
  });

  test("rejects missing, external, and non-canonical generator preview targets", () => {
    const taxonomy = loadTaxonomy();
    const actor = taxonomy.generatorContracts["actor-typegen"]!;
    const setup = taxonomy.generatorContracts["setup-wizard-config"]!;
    const missing = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewTarget: undefined } } } as unknown as Taxonomy;
    const external = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "setup-wizard-config": { ...setup, previewTarget: "workspace:preview-generated" } } } as unknown as Taxonomy;
    const nonCanonical = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewTarget: "@semio-tech/framework-actor-rs:preview" } } } as unknown as Taxonomy;
    expect(validateTaxonomy(missing).some((problem) => problem.includes("previewTarget is required"))).toBe(true);
    expect(validateTaxonomy(external).some((problem) => problem.includes("previewTarget is forbidden"))).toBe(true);
    expect(validateTaxonomy(nonCanonical).some((problem) => problem.includes("must be the exact owner preview-generated target"))).toBe(true);
  });

  test("rejects unsettled, broad Ralph, incomplete Ralph, and false root generation contracts", () => {
    const taxonomy = loadTaxonomy();
    const setup = taxonomy.generatorContracts["setup-wizard-config"]!;
    const unsettled = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "setup-wizard-config": { ...setup, ownership: "unknown" } } } as unknown as Taxonomy;
    expect(validateTaxonomy(unsettled).some((problem) => problem.includes("zero unknown or unsafe"))).toBe(true);
    const broad = { ...taxonomy, fixedDirectoryContracts: { ...taxonomy.fixedDirectoryContracts, "ralph-prd-identifier": { ...taxonomy.fixedDirectoryContracts["ralph-prd-identifier"]!, pathPattern: ".ralph-tui/**" } } };
    expect(validateTaxonomy(broad).some((problem) => problem.includes("recursive wildcard"))).toBe(true);
    const incomplete = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "setup-wizard-config": { ...setup, outputRoots: setup.outputRoots.slice(1) } } };
    expect(validateTaxonomy(incomplete).some((problem) => problem.includes("exactly the seven tracked Ralph files"))).toBe(true);
    const generatedRoot = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "setup-wizard-config": { ...setup, outputRoots: [{ path: "package.json", inclusion: "tracked" as const }] } } };
    expect(validateTaxonomy(generatedRoot).some((problem) => problem.includes("authored fixed contracts"))).toBe(true);
  });

  test("declares direct plugin-root facets without a nested directory taxonomy field", () => {
    const taxonomy = loadTaxonomy();
    expect("pluginDirName" in taxonomy).toBe(false);
    expect(taxonomy.pluginChildDirs).toEqual(["🎮️commands", "🔨️modules"]);
    expect(taxonomy.osChildDirs).toEqual(["🎮️commands", "🎚️config"]);
  });

  test("keeps the artifact completeness set and the artifact structural set as two separate lists", () => {
    const taxonomy = loadTaxonomy();
    // 🗿️ Completeness is 🧬️schema + 🚪️io — never ⚙️engine; structural-only extra is 📚️examples.
    expect(taxonomy.artifactChildDirs).toEqual(["🧬️schema", "🚪️io", "📚️examples", "🔨️modules"]);
    expect(taxonomy.artifactChildDirs.filter((dir) => !taxonomy.artifactComponentDirs.includes(dir))).toEqual(["📚️examples", "🔨️modules"]);
  });

  test("derives lifecycle capabilities instead of declaring artifact facet directories", () => {
    const taxonomy = loadTaxonomy();
    const lifecycleDirs = ["🏗️builder", "🧐️analyzer", "🎹️composer"];
    expect(taxonomy.newArtifactComponentDirs).toEqual(["🏅️standards"]);
    expect(taxonomy.newArtifactChildDirs).toEqual(["🏅️standards"]);
    expect(taxonomy.standardComponentDirs).toEqual(["🪆️subsets"]);
    expect(taxonomy.standardChildDirs).toEqual(["🪆️subsets", "🔨️modules"]);
    expect(taxonomy.subsetComponentDirs).toEqual(["🧬️schema", "🚪️io"]);
    expect(taxonomy.subsetChildDirs).toEqual(["🧬️schema", "🚪️io", "📚️examples", "👁️viewer", "✏️editor"]);
    expect(taxonomy.subsetArchetypes).toEqual(["owning", "derived"]);
    expect(taxonomy.ioFidelityClasses).toEqual(["exact", "canonical", "semantic", "lossy"]);
    expect([
      ...taxonomy.newArtifactChildDirs,
      ...taxonomy.standardChildDirs,
      ...taxonomy.subsetChildDirs,
    ].filter((dir) => lifecycleDirs.includes(dir))).toEqual([]);
  });

  test("describes the per-example assets/tests shape instead of plural facet dirs", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.exampleAssetsDirName).toBe("🖼️assets");
    expect(taxonomy.exampleTestsDirName).toBe("🧪️tests");
    expect(taxonomy.exampleSlugPattern).toBe("^.+\\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$");
    expect(taxonomy.exampleAssetKindPrefixes).toEqual({
      "snapshot-text": "🗣️",
      "snapshot-binary": "🎒️",
      "mutations-text": "🔧️",
      "mutations-binary": "📡️",
      "diff-text": "🔺️",
      cmd: "🎮️",
    });
    expect(taxonomy.exampleMediaKindPrefixes).toEqual({
      image: "🖼️",
      mesh: "🧊️",
      document: "📄️",
      video: "🎬️",
    });
    expect(taxonomy.exampleFileKinds).toEqual({
      "🦀️rust": "rust-source",
      "🟦️typescript": "typescript-source",
    });
    expect(taxonomy.exampleTestFileKinds).toEqual({
      "🦀️rust": "rust-source",
      "🟦️typescript": "typescript-source",
    });
    expect(taxonomy.forbiddenExampleSlugs).toEqual(["♻️reuse", "♻️default", "📕️default", "♻️semio"]);
    expect(taxonomy.forbiddenExamplePluralDirs).toEqual(["🎒️packs", "🗣️dsls", "🔧️ops", "📡️sprs"]);
    expect(taxonomy.surfaceChildDirs).toContain("📚️examples");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🎒️packs");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🗣️dsls");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🔧️ops");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("📡️sprs");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🖼️assets");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🧪️tests");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🧬️mutations");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🦠️mutation");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("↩️inverse");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🔺️diff");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🧬️schema");
    expect("exampleComponentDirs" in taxonomy).toBe(false);
  });

  test("forbids both the plural and singular implementations spellings", () => {
    expect(loadTaxonomy().forbiddenPathSegments).toEqual(["⚡️implementations", "⚡️implementation"]);
  });

  test("describes every declared lang with a manifest/marker/leaf contract", () => {
    const taxonomy = loadTaxonomy();
    for (const lang of taxonomy.langs) {
      const ecosystem = taxonomy.ecosystems[lang];
      expect(ecosystem).toBeDefined();
      expect(taxonomy.fileKinds[ecosystem.componentFileKindId]).toBeDefined();
    }
    expect(taxonomy.ecosystems["🦀️rust"].marker).toEqual({ in: "manifest", format: "toml", table: "package.metadata.semio", roleKey: "role", idKey: "id" });
    expect(taxonomy.ecosystems["🟦️typescript"].marker).toEqual({ in: "manifest", format: "json", table: "semio", roleKey: "role", idKey: "id" });
    expect(taxonomy.ecosystems["🐍️python"].marker?.table).toBe("tool.semio");
    expect(taxonomy.ecosystems["🐹️go"].manifestContractId).toBe("nx-project-manifest");
    expect(taxonomy.ecosystems["🐹️go"].moduleRootContractId).toBe("go-module");
    expect(taxonomy.ecosystems["🐹️go"].marker?.table).toBe("metadata.semio");
  });

  test("encodes Shape V2 entry location and both valid rust #[path] base conventions", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.entryLocation).toBe("packages");
    expect(taxonomy.rustEntryPathRules.entryDirFromOwner).toBe("📦️packages/🦀️rust");
    expect(taxonomy.rustEntryPathRules.resolution).toBe("cumulative");
    expect(taxonomy.rustEntryPathRules.conventions.map((convention) => convention.id)).toEqual(["leaf-prefixed", "once-reset"]);
    // 🥞️ A grouping `#[path = "."]` reset is never prefixed under either convention — the bug that only a real compile catches.
    expect(taxonomy.rustEntryPathRules.conventions.every((convention) => convention.groupingReset === ".")).toBe(true);
  });

  test("enforces clean declared and undeclared areas while keeping recovered compose trees opaque", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.areaEnforcement).toEqual({ requiredState: "clean", undeclaredAreas: "enforce", opaquePathExclusionIds: ["compose", "temp-compose"] });
    expect(new Set(Object.values(taxonomy.areas))).toEqual(new Set(["clean"]));
    expect(taxonomy.migratedMarker).toBe("packages-dir-exists");
  });
});

describe("validateTaxonomy", () => {
  test("the shipped vocabulary is internally consistent", () => {
    expect(validateTaxonomy()).toEqual([]);
  });

  test("discriminates catalog descendants from exact bundles and diagnoses missing exact alternatives", () => {
    const taxonomy = structuredClone(loadTaxonomy()) as Taxonomy;
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    const malformed = structuredClone(taxonomy) as Taxonomy;
    delete (malformed.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { exclusiveAlternatives?: unknown }).exclusiveAlternatives;
    expect(validateTaxonomy(malformed).some((problem) => problem.includes("draw-editor-command-bundle-v1") && problem.includes("exclusiveAlternatives must be an array"))).toBe(true);
    const wrongCatalog = structuredClone(taxonomy) as Taxonomy;
    (wrongCatalog.semanticDescendantContracts["cad-model-catalog-bundle-v1"] as unknown as { catalogContractId: string }).catalogContractId = "draw-editor-command-vectors-v1";
    expect(validateTaxonomy(wrongCatalog).some((problem) => problem.includes("cad-model-catalog-bundle-v1") && problem.includes("distributed JSON manifest catalog"))).toBe(true);
  });

  test("declares schema facet kinds partitioning data vs interface formats", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaFacetKinds?.["🧬️data"]?.normativeFormat).toBe("🔣️jsonschema");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.normativeFormat).toBe("📜️wit");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.formats).toEqual(["📜️wit"]);
    expect(taxonomy.schemaFormats?.["📜️wit"]?.fieldCasing).toBe("kebab");
    expect(taxonomy.packagingDirectoryKindIds).toEqual(["targets", "fixtures", "apps"]);
    expect(taxonomy.ecosystems["🦀️rust"]?.packagingDirectoryKindIds).toEqual(["benchmarks", "typescript-language"]);
  });

  test("declares canonical semantic collection and module ownership contracts", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.semanticManifestFileKindId).toBe("json");
    expect(taxonomy.semanticExtensionKey).toBe("x-semio");
    expect(taxonomy.semanticConsumerMinimum).toBe(2);
    expect(taxonomy.semanticCollections["🔨️modules"]?.kind).toBe("module");
    expect(taxonomy.semanticCollections["💡️inferences"]?.kind).toBe("inference");
    expect(taxonomy.semanticCollections["🚪️io/🧬️mutations"]).toEqual({ kind: "io", direction: "transport" });
    expect(taxonomy.ioSemanticCollectionDirNames).toEqual(["📸️snapshot", "🔺️diff", "💡️inferences", "🧬️mutations"]);
    expect(artifactFacetPathIsDeclared("🚪️io/🧬️mutations/📝️text", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/🧬️mutations/💾️binary", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/📝️text", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/💾️binary", taxonomy)).toBe(false);
    expect(taxonomy.ioSemanticCollectionDirNames).toContain("🧬️mutations");
  });

  test("declares the surface vocabulary ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET froze", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaVersion).toBe(7);
    expect(taxonomy.viewerDirName).toBe("👁️viewer");
    expect(taxonomy.editorDirName).toBe("✏️editor");
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: "👁️viewer", editor: "✏️editor" });
    expect(taxonomy.subsetChildDirs).toContain("👁️viewer");
    expect(taxonomy.subsetChildDirs).toContain("✏️editor");
    expect(taxonomy.subsetRequiredSurfaceDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.contributedSubsetChildDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.surfaceChildDirs).toContain(taxonomy.modesDirName);
    expect(taxonomy.surfaceComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowLeafLangs).toEqual(["🦀️rust", "🟦️typescript"]);
  });

  test("surfaceRoles preserves the AppRole declaration and wire-tag order", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
  });

  test("surface role mappings name their registered v7 directories", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: taxonomy.viewerDirName, editor: taxonomy.editorDirName });
  });

  test("required surface children belong to the structural set and cover every state lane", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceRequiredChildDirs.every((dir) => taxonomy.surfaceChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.surfaceRequiredChildDirs).toEqual(expect.arrayContaining(["🎚️config", "👥️presence", "🫧️transient"]));
  });

  test("every surface directory belongs to the subset structural set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.subsetSurfaceDirs.every((dir) => taxonomy.subsetChildDirs.includes(dir))).toBe(true);
  });

  test("rejects any area outside the required clean state", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, "🧰️framework": "taxonomy" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('must be "clean"'))).toBe(true);
  });

  test("keeps plugin roots inside the clean area registry", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginAreas.every((area) => taxonomy.areas[area] === "clean")).toBe(true);
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, [taxonomy.pluginAreas[0]!]: "mixed" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('must be "clean"'))).toBe(true);
  });

  test("reports a completeness dir missing from the structural set", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, artifactChildDirs: taxonomy.artifactChildDirs.filter((dir) => dir !== "🚪️io") };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🚪️io"))).toBe(true);
  });

  test("keeps derived lifecycle directories out of the subset structural set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.subsetChildDirs.filter((dir) => ["🏗️builder", "🧐️analyzer", "🎹️composer"].includes(dir))).toEqual([]);
  });

  test("registers every optional mutation facet as a taxonomy leaf parent", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.mutationOptionalFacetDirs.every((dir) => taxonomy.taxonomyLeafParentDirs.includes(dir))).toBe(true);
  });

  test("declares direct mutation ownership and optional subfacets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.mutationComponentFileKindId).toBe("rust-source");
    expect(taxonomy.mutationDescriptorFileKindId).toBe("json");
    expect(taxonomy.mutationOptionalFacetDirs).toEqual(["🔺️diff", "↩️inverse", "🧩️plan", "📝️text", "💾️binary", "🧬️schema"]);
    expect(new RegExp(taxonomy.mutationDirectoryPattern, "u").test("➕️add-node")).toBe(true);
    expect(new RegExp(taxonomy.mutationDirectoryPattern, "u").test("➕️node")).toBe(false);
    expect("mutationChildDirs" in taxonomy).toBe(false);
    expect("compositeMutationChildDirs" in taxonomy).toBe(false);
    expect(taxonomy.schemaChildDirs).toContain("🧬️mutations");
  });

  test("keeps required window capabilities non-empty and structural", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.windowRequiredChildDirs.length).toBeGreaterThan(0);
    expect(taxonomy.windowRequiredChildDirs.every((dir) => taxonomy.windowChildDirs.includes(dir))).toBe(true);
  });

  test("declares command facets at plugin and OS ownership boundaries", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.osRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.pluginRequiredChildDirs.every((dir) => taxonomy.pluginChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.osRequiredChildDirs.every((dir) => taxonomy.osChildDirs.includes(dir))).toBe(true);
  });

  test("declares every state lane in each state-owning scope", () => {
    const taxonomy = loadTaxonomy();
    const stateLanes = ["🎚️config", "👥️presence", "🫧️transient"];
    expect(stateLanes.every((dir) => taxonomy.modeChildDirs.includes(dir))).toBe(true);
    expect(stateLanes.every((dir) => taxonomy.windowRequiredChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.modeChildDirs).toContain(taxonomy.windowsDirName);
  });

  test("reports an invalid window component language set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.windowComponentLangs.every((lang) => Boolean(taxonomy.componentFileKinds[lang]))).toBe(true);
    const missingMarkerKind = { ...taxonomy, windowEmptyFacetFileKindId: "unknown" };
    expect(validateTaxonomy(missingMarkerKind).some((problem) => problem.includes("windowEmptyFacetFileKindId references a missing file kind"))).toBe(true);
  });

  test("declares direct plugin-root facets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginChildDirs).toEqual(["🎮️commands", "🔨️modules"]);
  });

  test("includes commands at every command-owning scope", () => {
    const taxonomy = loadTaxonomy();
    for (const key of ["surfaceChildDirs", "modeChildDirs", "pluginChildDirs", "osChildDirs"] as const) {
      expect(taxonomy[key]).toContain("🎮️commands");
    }
  });

  test("keeps direct plugin-root facet declarations non-empty", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginChildDirs.every((dir) => dir.length > 0)).toBe(true);
  });

  test("rejects plural example component dirs in taxonomyLeafParentDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      taxonomyLeafParentDirs: [...taxonomy.taxonomyLeafParentDirs, "🎒️packs"],
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🎒️packs"))).toBe(true);
  });

  test("declares a valid example slug pattern", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.exampleSlugPattern.length).toBeGreaterThan(0);
    expect(() => new RegExp(taxonomy.exampleSlugPattern, "u")).not.toThrow();
  });

  test("declares the schema lifecycle collections", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaChildDirs).toEqual(["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"]);
  });

  test("declares the schema representation kinds", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.representationDirs).toEqual(["📝️text", "💾️binary"]);
  });

  test("keeps bare pack out of artifact structural and completeness sets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.artifactComponentDirs).not.toContain("🎒️pack");
    expect(taxonomy.artifactChildDirs).not.toContain("🎒️pack");
  });

  test("keeps artifact specification mappings on fully qualified v7 facet paths", () => {
    const taxonomy = loadTaxonomy();
    expect("🎒️pack" in taxonomy.artifactSpecFileKinds).toBe(false);
    expect(taxonomy.artifactSpecFileKinds["🧬️schema/📸️snapshot/💾️binary"]).toBe("protocol-semio");
  });

  test("reports a schema format referencing an unknown file kind", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      schemaFormats: {
        ...taxonomy.schemaFormats,
        "🦀️rust": { ...taxonomy.schemaFormats["🦀️rust"], fileKindId: "unknown" },
      },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('schemaFormats["🦀️rust"].fileKindId is missing'))).toBe(true);
  });

  test("reports artifact schema mappings referencing an unknown file kind", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      artifactSchemaSpecFileKinds: { ...taxonomy.artifactSchemaSpecFileKinds, "🧬️schema": "unknown" },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("artifactSchemaSpecFileKinds") && problem.includes("unknown"))).toBe(true);
  });
});

//#region 🧬️DirectMutationTaxonomyTests
const DIRECT_MUTATION_RUST_FIXTURE = `
#![allow(dead_code)]

pub struct Snapshot;
pub struct AddNode {
    pub id: u64,
    pub label: String,
}

#[path = "➕️add-node/🦀️component.rs"]
pub mod add_node;

#[cfg(test)]
#[path = "🧪️tests/add-node/🦀️component.rs"]
mod tests;

pub enum Mutation {
    AddNode(add_node::AddNode),
    RemoveNode { id: u64 },
    Noop,
}

impl protocol::MutationKind<Snapshot, Mutation> for AddNode {
    const ID: &'static str = "add-node";

    fn diff(&self, _base: &Snapshot) -> u64 {
        self.id
    }

    fn inverse(&self, _base: &Snapshot) -> Vec<Mutation> {
        Vec::new()
    }
}

pub const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
    verb: "add",
    entity: "node",
    kind: "add-node",
    record: "AddNode",
};
pub const WIRE_KIND: &str = "add-node";
const GENERATED: &str = include!(concat!(env!("OUT_DIR"), "/mutations.rs"));
const FAKE_RUST: &str = r#"pub enum Fake { Bad(fake::Bad) }"#;
// pub enum CommentFake { Bad(comment_fake::Bad) }

pub fn dispatch(value: &Mutation) -> u64 {
    match value {
        Mutation::AddNode(payload) => payload.id,
        Mutation::RemoveNode { id } => *id,
        Mutation::Noop => 0,
    }
}
`;

describe("direct mutation taxonomy", () => {
  test("models direct ownership and only optional child facets", () => {
    const taxonomy = loadTaxonomy();
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/➕️add-node", taxonomy)).toBe(true);
    for (const facet of taxonomy.mutationOptionalFacetDirs) expect(artifactFacetPathIsDeclared(`🧬️schema/🧬️mutations/➕️add-node/${facet}`, taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/➕️add-node/🦠️mutation", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/➕️node", taxonomy)).toBe(false);
  });

  test("extracts stable Rust structural facts without treating comments or strings as syntax", () => {
    const facts = inspectRustStructure(DIRECT_MUTATION_RUST_FIXTURE);
    expect(facts.schemaVersion).toBe(1);
    expect(facts.modules).toEqual([
      { name: "add_node", visibility: "pub", inline: false, pathTarget: "➕️add-node/🦀️component.rs", cfgTest: false },
      { name: "tests", visibility: "private", inline: false, pathTarget: "🧪️tests/add-node/🦀️component.rs", cfgTest: true },
    ]);
    expect(facts.enums).toEqual([{
      name: "Mutation",
      visibility: "pub",
      variants: [
        { name: "AddNode", fieldStyle: "tuple", fieldTypes: ["add_node::AddNode"], wrappedTupleLeafType: "add_node::AddNode" },
        { name: "RemoveNode", fieldStyle: "struct", fieldTypes: ["u64"], wrappedTupleLeafType: null },
        { name: "Noop", fieldStyle: "unit", fieldTypes: [], wrappedTupleLeafType: null },
      ],
    }]);
    expect(facts.inlinePayloads.find((payload) => payload.name === "AddNode")?.fields).toEqual([
      { name: "id", type: "u64" },
      { name: "label", type: "String" },
    ]);
    expect(facts.impls).toContainEqual({
      traitPath: "protocol::MutationKind<Snapshot,Mutation>",
      selfType: "AddNode",
      methods: ["diff", "inverse"],
      associatedConstants: ["ID"],
    });
    expect(facts.matchArms.map((arm) => arm.variantPath)).toEqual(["Mutation::AddNode", "Mutation::RemoveNode", "Mutation::Noop"]);
    expect(facts.constants.find((constant) => constant.name === "SEMANTICS")?.identityFields).toEqual({ entity: "node", kind: "add-node", record: "AddNode", verb: "add" });
    expect(facts.constants.find((constant) => constant.name === "WIRE_KIND")?.stringValue).toBe("add-node");
    expect(facts.includes).toEqual([{ macro: "include", expression: 'concat!(env!("OUT_DIR"),"/mutations.rs")', usesOutDir: true }]);
    expect(facts.testModules).toEqual([facts.modules[1]]);
    expect(renderRustStructuralFactsJson(facts)).toBe(renderRustStructuralFactsJson(inspectRustStructure(DIRECT_MUTATION_RUST_FIXTURE)));
  });

  test("filters virtual opaque paths before reading and normalizes Windows separators", () => {
    const outside = "🧰️framework/🧬️schema/🧬️mutations/➕️add-node/🦀️component.rs";
    const opaque = "compose/🧬️schema/🧬️mutations/➕️add-node/🦀️component.rs";
    const recoveredOpaque = "temp\\compose\\🧬️schema\\🧬️mutations\\➕️add-node\\🦀️component.rs";
    const reads: string[] = [];
    const inspected = inspectRustVirtualSources([opaque, outside, recoveredOpaque], (path) => {
      reads.push(path);
      if (path !== outside) throw new Error(`opaque virtual source was read: ${path}`);
      return DIRECT_MUTATION_RUST_FIXTURE;
    });
    expect(reads).toEqual([outside]);
    expect(inspected.map((entry) => entry.path)).toEqual([outside]);
    expect(inspected[0]?.facts.enums[0]?.name).toBe("Mutation");
  });

  test("agrees with the nightly Rust parser on every declared structural identity", () => {
    const facts = inspectRustStructure(DIRECT_MUTATION_RUST_FIXTURE);
    const parsed = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-type=lib", "-"], { input: DIRECT_MUTATION_RUST_FIXTURE, encoding: "utf8" });
    expect(parsed.status).toBe(0);
    const ast = parsed.stdout;
    for (const identity of [...facts.modules.map((fact) => fact.name), ...facts.enums.map((fact) => fact.name), ...facts.inlinePayloads.map((fact) => fact.name), ...facts.constants.map((fact) => fact.name)]) expect(ast).toContain(identity);
  });
});
//#endregion 🧬️DirectMutationTaxonomyTests

//#region 🧩️SemanticCollections
function semanticFixture(options: { readonly secondProductionConsumer?: boolean; readonly rootBehavior?: boolean; readonly rustRelativeConsumers?: boolean; readonly glueConsumer?: boolean } = {}): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-semantic-census-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🔨️modules/📏measure/🟦️.ts", "export const measure = (value: number) => value;\n");
  if (options.rustRelativeConsumers || options.glueConsumer) write("🧰️framework/🔨️modules/📏measure/🦀️.rs", "pub fn measure(value: u32) -> u32 { value }\n");
  write("🧰️framework/🔨️modules/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: "📏measure", id: "measure", kind: "module", responsibility: "exact and stable numeric measurement", module: { productionConsumers: options.secondProductionConsumer ? ["height", "width"] : ["width"] } }] } }));
  write("🧰️framework/💡️inferences/📏width/🟦️.ts", 'import { measure } from "../../🔨️modules/📏measure/🟦️.ts";\nexport const width = measure(1);\n');
  if (options.secondProductionConsumer) write("🧰️framework/💡️inferences/↕️height/🟦️.ts", 'import { measure } from "../../🔨️modules/📏measure/🟦️.ts";\nexport const height = measure(1);\n');
  if (options.rustRelativeConsumers) {
    write("🧰️framework/💡️inferences/📏width/🦀️.rs", "use super::super::modules::measure::measure;\npub fn width() -> u32 { measure(1) }\n");
    if (options.secondProductionConsumer) write("🧰️framework/💡️inferences/↕️height/🦀️.rs", "use super::super::modules::measure::measure;\npub fn height() -> u32 { measure(1) }\n");
  }
  if (options.glueConsumer) write("🧰️framework/💡️inferences/📏width/📦️packages/🦀️rust/📦️glue.rs", '#[path = "../../../../🔨️modules/📏measure/🦀️.rs"]\npub mod measure;\n');
  write("🧰️framework/💡️inferences/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📏width", id: "width", kind: "inference", responsibility: "derived width", inference: { inputs: ["value"], target: "width" } },
    ...(options.secondProductionConsumer ? [{ directory: "↕️height", id: "height", kind: "inference", responsibility: "derived height", inference: { inputs: ["value"], target: "height" } }] : []),
  ] } }));
  if (options.rootBehavior) write("🧰️framework/💡️inferences/🟦️.ts", "export function analyze() { return 1; }\n");
  return { root, taxonomy };
}

function schemaMountedRustModulesFixture(): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-schema-mounted-rust-modules-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🔨️modules/📏measure/🦀️.rs", "pub fn measure(value: u32) -> u32 { value }\n");
  write("🧰️framework/🔨️modules/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: "📏measure", id: "measure", kind: "module", responsibility: "exact and stable numeric measurement", module: { productionConsumers: ["height", "width"] } }] } }));
  write("🧰️framework/🧬️schema/💡️inferences/📏width/🦀️.rs", "use super::super::modules::measure::measure;\npub fn width() -> u32 { measure(1) }\n");
  write("🧰️framework/🧬️schema/💡️inferences/↕️height/🦀️.rs", "use super::super::modules::measure::measure;\npub fn height() -> u32 { measure(1) }\n");
  write("🧰️framework/🧬️schema/💡️inferences/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📏width", id: "width", kind: "inference", responsibility: "derived width", inference: { inputs: ["value"], target: "width" } },
    { directory: "↕️height", id: "height", kind: "inference", responsibility: "derived height", inference: { inputs: ["value"], target: "height" } },
  ] } }));
  return { root, taxonomy };
}

function mutationTransportFixture(): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-mutation-transport-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🚪️io/🧬️mutations/📝️text/🟦️.ts", "export const encodeMutationText = (value: string) => value;\n");
  write("🧰️framework/🚪️io/🧬️mutations/💾️binary/🟦️.ts", "export const encodeMutationBinary = (value: Uint8Array) => value;\n");
  write("🧰️framework/🚪️io/🧬️mutations/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📝️text", id: "framework.io.mutation-text", kind: "io", responsibility: "frozen mutation text transport", io: { format: "Semio mutation text", direction: "transport" } },
    { directory: "💾️binary", id: "framework.io.mutation-binary", kind: "io", responsibility: "frozen mutation binary transport", io: { format: "Semio mutation binary", direction: "transport" } },
  ] } }));
  return { root, taxonomy };
}

describe("semantic collection census", () => {
  test("accepts a genuine two-production-component module at its lowest common owner", () => {
    const fixture = semanticFixture({ secondProductionConsumer: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
      expect(census.records.find((record) => record.id === "measure")?.computedLowestCommonOwner).toBe("🧰️framework");
      expect(renderSemanticCensusJson(census)).toBe(renderSemanticCensusJson(buildSemanticCensus(fixture.root, {}, fixture.taxonomy)));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves relative Rust and TypeScript imports to terminal production components", () => {
    const fixture = semanticFixture({ secondProductionConsumer: true, rustRelativeConsumers: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      const imports = census.graph.edges.filter((edge) => edge.to === "measure" && edge.mechanism === "static-import");
      expect(imports.map((edge) => edge.from)).toEqual(["height", "height", "width", "width"]);
      expect(imports.map((edge) => edge.target.endsWith("🦀️.rs") || edge.target.endsWith("🟦️.ts"))).toEqual([true, true, true, true]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves logical schema modules to their closest physical module collection", () => {
    const fixture = schemaMountedRustModulesFixture();
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.graph.edges.filter((edge) => edge.to === "measure" && edge.mechanism === "static-import").map((edge) => edge.from)).toEqual(["height", "width"]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("rejects a one-consumer module and authored list-root behavior", () => {
    const fixture = semanticFixture({ rootBehavior: true });
    try {
      const codes = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code);
      expect(codes).toContain("module-consumer-minimum");
      expect(codes).toContain("module-production-consumer-minimum");
      expect(codes).toContain("collection-authored-behavior");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("includes unregistered collection-path findings beneath a semantic owner scope", () => {
    const fixture = semanticFixture({ rootBehavior: true });
    try {
      const manifestPath = join(fixture.root, "🧰️framework/💡️inferences/🔣️.json");
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      manifest["x-semio"].members[0].id = "framework.inference.width";
      writeFileSync(manifestPath, JSON.stringify(manifest));
      const scoped = buildSemanticCensus(fixture.root, { scope: "framework" }, fixture.taxonomy);
      expect(scoped.records.map((record) => record.id)).toEqual(["framework.inference.width"]);
      expect(scoped.problems.some((problem) => problem.code === "collection-authored-behavior" && problem.path.endsWith("💡️inferences/🟦️.ts"))).toBe(true);
      expect(scoped.problems.some((problem) => problem.code === "module-production-consumer-minimum" && problem.path.endsWith("🔨️modules/📏measure"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("accepts mutation codecs only as explicit bidirectional I/O transport boundaries", () => {
    const fixture = mutationTransportFixture();
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.records.map((record) => record.id)).toEqual(["framework.io.mutation-binary", "framework.io.mutation-text"]);
      const manifestPath = join(fixture.root, "🧰️framework/🚪️io/🧬️mutations/🔣️.json");
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      manifest["x-semio"].members[0].io.direction = "import";
      writeFileSync(manifestPath, JSON.stringify(manifest));
      expect(buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code)).toContain("io-contract-missing");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("does not count a test call site as a second production component", () => {
    const fixture = semanticFixture();
    try {
      const testPath = join(fixture.root, "🧰️framework/💡️inferences/📏width/🧪️tests/🟦️.ts");
      mkdirSync(join(testPath, ".."), { recursive: true });
      writeFileSync(testPath, 'import { measure } from "../../../🔨️modules/📏measure/🟦️.ts";\nexport const checked = measure(1);\n');
      const module = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).records.find((record) => record.id === "measure");
      expect(module?.productionConsumers).toEqual(["width"]);
      expect(module?.excludedConsumers).toEqual(["width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("does not count a package glue mount as a production consumer", () => {
    const fixture = semanticFixture({ glueConsumer: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      const glueEdge = census.graph.edges.find((edge) => edge.source.endsWith("📦️glue.rs") && edge.to === "measure");
      expect(glueEdge?.production).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("rejects missing manifest members, stale declarations, and generic stems", () => {
    const fixture = semanticFixture();
    try {
      const collection = join(fixture.root, "🧰️framework/💡️inferences");
      mkdirSync(join(collection, "🧪️orphan"), { recursive: true });
      writeFileSync(join(collection, "🧪️orphan/🟦️.ts"), "export const orphan = true;\n");
      const manifest = JSON.parse(readFileSync(join(collection, "🔣️.json"), "utf8"));
      manifest["x-semio"].members.push({ directory: "🧹️stale", id: "stale", kind: "inference", responsibility: "stale result", inference: { inputs: ["x"], target: "stale" } });
      manifest["x-semio"].members[0].directory = "🧪️shared";
      writeFileSync(join(collection, "🔣️.json"), JSON.stringify(manifest));
      const codes = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code);
      expect(codes).toContain("manifest-child-missing");
      expect(codes).toContain("manifest-child-extra");
      expect(codes).toContain("member-generic-stem");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves cumulative nested Rust path attributes", () => {
    const root = join(tmpdir(), "semantic-rust-path");
    const source = [
      '#[path = "../../."]',
      "pub mod gltf {",
      '  #[path = "."]',
      "  pub mod schema {",
      '    #[path = "metrics/🦀️.rs"]',
      "    pub mod metric;",
      "  }",
      "}",
    ].join("\n");
    expect(resolveRustPathAttributes(join(root, "📦️glue.rs"), source).at(-1)?.target).toBe(resolve(root, "../../metrics/🦀️.rs"));
  });

  test("places inference codecs below I/O rather than the inference result collection", () => {
    const taxonomy = loadTaxonomy();
    expect(artifactFacetPathIsDeclared("🚪️io/💡️inferences/📝️text", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/💡️inferences/💾️binary", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/💡️inferences/📝️text", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🧬️schema/💡️inferences/💾️binary", taxonomy)).toBe(false);
  });
});
//#endregion 🧩️SemanticCollections

describe("areaOf", () => {
  test("longest-prefix matches a plugin path to its declared area", () => {
    // 🕵️ "✏️s/🔌️plugins" graduated "legacy" -> "clean" in an earlier wave of
    // 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT; this assertion was never updated to match.
    expect(areaOf("✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust")).toBe("clean");
  });

  test("longest-prefix matches framework paths to clean", () => {
    expect(areaOf("🧰️framework/🛍️products/💻️os")).toBe("clean");
  });

  test("returns undefined outside every declared area", () => {
    expect(areaOf("some/unknown/path")).toBeUndefined();
  });
});

describe("readSemioMarker", () => {
  test("reads role = \"plugin\" from the writer plugin's migrated Cargo.toml", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml");
    expect(readSemioMarker(manifestPath, "🦀️rust")).toEqual({ role: "plugin" });
  });

  test("returns undefined for a non-existent manifest", () => {
    const root = getWorkspaceRoot();
    expect(readSemioMarker(join(root, "does/not/exist/Cargo.toml"), "🦀️rust")).toBeUndefined();
  });

  test("reads role + id from a typescript package.json's \"semio\" key", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json");
    expect(readSemioMarker(manifestPath, "🟦️typescript")).toEqual({ role: "framework", id: "ui-styling-ts" });
  });

  test("returns undefined for a non-existent manifest", () => {
    const root = getWorkspaceRoot();
    expect(readSemioMarker(join(root, "does/not/exist/Cargo.toml"), "🦀️rust")).toBeUndefined();
  });

  test("does not mistake one ecosystem's manifest for another's", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml");
    expect(readSemioMarker(manifestPath, "🐹️go")).toBeUndefined();
    expect(readSemioMarker(manifestPath, "🐍️python")).toBeUndefined();
  });

  test("reads the go and python marker contracts those ecosystems will carry", () => {
    // 🌱️ No go/python package declares a marker on disk yet (W8 restructures them); this pins the contract
    // the vocabulary promises so the reader is written and proven before the first real manifest lands.
    const dir = mkdtempSync(join(tmpdir(), "semio-marker-"));
    try {
      const goManifest = join(dir, "📋️project.json");
      writeFileSync(goManifest, JSON.stringify({ name: "@semio-tech/repo-cli", metadata: { semio: { role: "product", id: "repo-cli" } } }));
      expect(readSemioMarker(goManifest, "🐹️go")).toEqual({ role: "product", id: "repo-cli" });
      const pyManifest = join(dir, "pyproject.toml");
      writeFileSync(pyManifest, '[project]\nname = "x"\n\n[tool.semio]\nrole = "framework"\nid = "ui-styling-py"\n');
      expect(readSemioMarker(pyManifest, "🐍️python")).toEqual({ role: "framework", id: "ui-styling-py" });
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });
});

describe("discoverPackages", () => {
  /** 🧭️ Every plugin dir that carries a rust package manifest — derived from disk, never a hand-maintained list (`taxonomy.migratedMarker`). */
  const migratedPluginDirs = (root: string): string[] =>
    readdirSync(join(root, "✏️s/🔌️plugins"), { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && existsSync(join(root, "✏️s/🔌️plugins", entry.name, "📦️packages/🦀️rust/Cargo.toml")))
      .map((entry) => `✏️s/🔌️plugins/${entry.name}`)
      .sort();

  test("finds every migrated plugin under the real repo root", () => {
    const root = getWorkspaceRoot();
    const catalog = discoverPackages(root);
    const pluginOwners = [...new Set(catalog.filter((pkg) => pkg.role === "plugin").map((pkg) => pkg.ownerRel))].sort();
    expect(pluginOwners).toEqual(migratedPluginDirs(root));
    const writerEntry = catalog.find((pkg) => pkg.ownerRel === "✏️s/🔌️plugins/✒️writer");
    expect(writerEntry?.area).toBe("clean");
    expect(writerEntry?.lang).toBe("🦀️rust");
    expect(writerEntry?.id).toBe("semio-s-plugin-writer");
  });

  test("only ever reports roles from the declared vocabulary", () => {
    const taxonomy = loadTaxonomy();
    const catalog = discoverPackages(getWorkspaceRoot());
    expect(catalog.filter((pkg) => !taxonomy.roles.includes(pkg.role))).toEqual([]);
    expect(catalog.filter((pkg) => !taxonomy.langs.includes(pkg.lang))).toEqual([]);
  });

  test("resolves the installed three-level 🎯️target shape for framework ui", () => {
    const catalog = discoverPackages(getWorkspaceRoot());
    const uiTargets = catalog.filter((pkg) => pkg.ownerRel === "🧰️framework/🔨️modules/🖱️ui").map((pkg) => pkg.target).sort();
    expect(uiTargets).toEqual(["⚛️react"]);
    expect(catalog.filter((pkg) => pkg.ownerRel === "🧰️framework/🔨️modules/🖱️ui").every((pkg) => pkg.role === "framework")).toBe(true);
  });

  test("an in-flight plugin is discovered as mixed, never as a problem", () => {
    const root = getWorkspaceRoot();
    // 🏗️ fem is the last plugin still mid-migration: a marked new-contract package PLUS residual
    // ⚡️implementations sandwiches and a Shape V1 owner-root entry file. Tolerating that state (instead of
    // erroring on it) is the whole point of deriving maturity from disk. Once fem lands this flips to clean,
    // which the assertions below allow for without going vacuous.
    const owners = discoverOwners(root);
    const fem = owners.find((owner) => owner.ownerRel === "✏️s/🔌️plugins/🏗️fem");
    expect(fem).toBeDefined();
    expect(fem?.roles).toEqual(["plugin"]);
    expect(fem?.maturity).toBe(fem!.residualImplDirs === 0 && fem!.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed");
    expect(discoverPackageProblems(root).filter((problem) => problem.path.startsWith("✏️s/🔌️plugins/🏗️fem"))).toEqual([]);
  });

  test("the real repo raises no discovery problems", () => {
    expect(discoverPackageProblems(getWorkspaceRoot())).toEqual([]);
  });
});

describe("discoverOwners", () => {
  test("every owner groups its own packages and derives maturity from its residuals", () => {
    const owners = discoverOwners(getWorkspaceRoot());
    expect(owners.length).toBeGreaterThan(0);
    for (const owner of owners) {
      expect(owner.packages.every((pkg) => pkg.ownerRel === owner.ownerRel)).toBe(true);
      expect(owner.packages.every((pkg) => pkg.maturity === owner.maturity)).toBe(true);
      expect(owner.maturity).toBe(owner.residualImplDirs === 0 && owner.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed");
      expect(owner.langs).toEqual([...new Set(owner.packages.map((pkg) => pkg.lang))]);
    }
  });

  test("nested owners are their own rows, not folded into the enclosing plugin", () => {
    const owners = discoverOwners(getWorkspaceRoot()).map((owner) => owner.ownerRel);
    // 🖍️ draw ships a proc-macro sibling crate; a nested 📦️packages dir is an owner in its own right.
    expect(owners).toContain("✏️s/🔌️plugins/🖍️draw");
    expect(owners.some((owner) => owner.startsWith("✏️s/🔌️plugins/🖍️draw/"))).toBe(true);
  });
});

describe("discoverBurndown", () => {
  /** 🔥️ Independent recursive count of forbidden-segment dirs under one path — deliberately not sharing the discovery walk. */
  const countImplDirs = (absDir: string): number => {
    let total = 0;
    for (const entry of readdirSync(absDir, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || entry.name === "node_modules" || entry.name === "target" || entry.name === "pkg") continue;
      if (loadTaxonomy().forbiddenPathSegments.includes(entry.name)) {
        total += 1;
        continue;
      }
      total += countImplDirs(join(absDir, entry.name));
    }
    return total;
  };

  test("counts residual ⚡️implementations dirs the same way an independent walk does", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const perPluginArea = burndown.implDirsByArea["clean"] ?? 0;
    expect(perPluginArea).toBe(countImplDirs(join(root, "✏️s/🔌️plugins")));
    expect(burndown.implDirsTotal).toBeGreaterThanOrEqual(perPluginArea);
  });

  test("owner residual counts add up to their area total", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const owners = discoverOwners(root).filter((owner) => owner.area === "mixed");
    const summed = owners.reduce((total, owner) => total + owner.residualImplDirs, 0);
    expect(summed).toBe(burndown.implDirsByArea["mixed"] ?? 0);
    expect(burndown.mixedOwners.every((owner) => owner.residualImplDirs > 0 || owner.entryFilesAtOwnerRoot.length > 0)).toBe(true);
    expect(burndown.cleanOwners + burndown.mixedOwners.length).toBe(burndown.ownersTotal);
  });

  test("markerless package manifests stay visible even where they are a silent skip", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const catalogPaths = new Set(discoverPackages(root).map((pkg) => pkg.manifestPath));
    expect(burndown.unmarkedManifests.every((manifest) => !catalogPaths.has(manifest.path))).toBe(true);
    expect(burndown.packagingViolations.every((violation) => violation.path.includes("📦️packages/"))).toBe(true);
  });

  test("discoverPackageProblems promotes packaging violations into census problems", () => {
    const root = getWorkspaceRoot();
    clearDiscoveryCache();
    const problems = discoverPackageProblems(root);
    expect(problems.some((problem) => problem.kind === "packaging-violation")).toBe(true);
    expect(problems.every((problem) => problem.path.length > 0 && problem.message.includes(problem.path))).toBe(true);
    const censusProblems = buildSemanticCensus(root).problems.filter((problem) => problem.kind === "packaging-violation");
    expect(censusProblems.length).toBe(problems.filter((problem) => problem.kind === "packaging-violation").length);
  });

  test("clearDiscoveryCache forces a fresh walk", () => {
    const root = getWorkspaceRoot();
    const first = discoverPackages(root);
    clearDiscoveryCache();
    expect(discoverPackages(root).map((pkg) => pkg.manifestPath)).toEqual(first.map((pkg) => pkg.manifestPath));
  });
});

describe("computeWorkspaces", () => {
  /** 🧪️ Builds a synthetic repo tree covering every hazard `🗂️workspaces.ts` was written to handle:
   * a plain Shape V1 package, a nested Shape V2-ish package, a skip-dir (`node_modules`) and a dot-dir
   * that must never surface, a wasm `pkg/` shadowed by its outer wrapper's identical name, a wasm `pkg/`
   * with a genuinely different name (included regardless of whether anything depends on it via
   * `workspace:*` — a real `bun install` only requires a listed workspace dir to exist on disk, and
   * 🌊️flow's real `flow-extension-bim` has no such dependent at all since it's loaded by path at runtime),
   * and a stray `pkg/` with no sibling `Cargo.toml` (must be skipped and never descended into). */
  function buildFixture(): string {
    const root = mkdtempSync(join(tmpdir(), "workspaces-fixture-"));
    const write = (relPath: string, content: string) => {
      const absPath = join(root, relPath);
      mkdirSync(join(absPath, ".."), { recursive: true });
      writeFileSync(absPath, content);
    };
    write("a/package.json", JSON.stringify({ name: "@t/a" }));
    write("b/nested/package.json", JSON.stringify({ name: "@t/b-nested" }));
    write("node_modules/pkgx/package.json", JSON.stringify({ name: "@t/should-be-skipped" }));
    write(".hidden/package.json", JSON.stringify({ name: "@t/also-should-be-skipped" }));
    // c: wasm pkg/ shares its outer wrapper's exact name -> shadowed, pkg/ excluded.
    write("c/📦️packages/🦀️rust/package.json", JSON.stringify({ name: "@t/c-rs" }));
    write("c/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"c\"\n");
    write("c/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/c-rs" }));
    // d: wasm pkg/ has its OWN name and no outer wrapper at all -> included (flow-extension-bim's real shape).
    write("d/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"d\"\n");
    write("d/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/d-wasm" }));
    // f: wasm pkg/ differs from its outer wrapper's name too -> included the same way (flow-core's real shape).
    write("f/📦️packages/🦀️rust/package.json", JSON.stringify({ name: "@t/f-rs" }));
    write("f/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"f\"\n");
    write("f/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/f-wasm" }));
    // e: a pkg/ dir with no sibling Cargo.toml -- a broken/stray wasm-pack emission, must never surface
    // and must never be descended into (its own nested junk must not surface either).
    write("e/strayroot/pkg/junk/package.json", JSON.stringify({ name: "@t/e-stray-nested" }));
    write("e/strayroot/pkg/package.json", JSON.stringify({ name: "@t/e-stray" }));
    return root;
  }

  test("includes plain and nested packages, excludes node_modules/dot-dirs", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("a");
      expect(result).toContain("b/nested");
      expect(result.some((entry) => entry.includes("node_modules"))).toBe(false);
      expect(result.some((entry) => entry.includes(".hidden"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a wasm pkg/ shadowed by its outer wrapper's identical name is excluded", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("c/📦️packages/🦀️rust");
      expect(result).not.toContain("c/📦️packages/🦀️rust/pkg");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a differently-named wasm pkg/ is included whether or not it has an outer wrapper", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("d/📦️packages/🦀️rust/pkg"); // no outer wrapper at all
      expect(result).toContain("f/📦️packages/🦀️rust");
      expect(result).toContain("f/📦️packages/🦀️rust/pkg"); // differs from its outer wrapper's name
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a pkg/ dir with no sibling Cargo.toml is skipped and never descended into", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result.some((entry) => entry.startsWith("e/"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("throws on a genuine duplicate package name (never silently produces an ambiguous array)", () => {
    const root = mkdtempSync(join(tmpdir(), "workspaces-collision-"));
    try {
      mkdirSync(join(root, "x"), { recursive: true });
      mkdirSync(join(root, "y"), { recursive: true });
      writeFileSync(join(root, "x/package.json"), JSON.stringify({ name: "@t/dup" }));
      writeFileSync(join(root, "y/package.json"), JSON.stringify({ name: "@t/dup" }));
      expect(() => computeWorkspaces(root)).toThrow(/duplicate package name/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("output is de-duplicated and lexicographically sorted", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(new Set(result).size).toBe(result.length);
      expect(result).toEqual([...result].sort((a, b) => a.localeCompare(b)));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("diffWorkspaces reports exactly what's missing/stale against a given current array", () => {
    const root = buildFixture();
    try {
      const expected = computeWorkspaces(root);
      expect(diffWorkspaces(root, []).missing).toEqual(expected);
      expect(diffWorkspaces(root, expected).missing).toEqual([]);
      expect(diffWorkspaces(root, expected).stale).toEqual([]);
      const withBogus = [...expected, "bogus/nonexistent"];
      expect(diffWorkspaces(root, withBogus).stale).toEqual(["bogus/nonexistent"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("against the real repo: every entry is a real dir with its own package.json, no duplicates, and known Shape V1/V2 + math drift cases resolve as expected", () => {
    const root = getWorkspaceRoot();
    const result = computeWorkspaces(root);
    for (const entry of result) expect(existsSync(join(root, entry, "package.json"))).toBe(true);
    expect(new Set(result).size).toBe(result.length);
    // Shape V1 (not yet migrated) framework core must still resolve — dropping it would break bun install repo-wide.
    expect(result).toContain("🧰️framework/📦️packages/🟦️typescript");
    // Shape V2 (already migrated) flow plugin TS residual.
    expect(result).toContain("✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript");
    expect(result).toContain("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust");
    // editor's wasm pkg/ shares its outer wrapper's name -> must stay excluded (would collide otherwise).
    expect(result).not.toContain("🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg");
  });
});

//#region 🧪️PluginDependencyParityPolicy
describe("policyPluginDependencyParityBreaches", () => {
  test("excludes nested extensions from parent plugin walk while checking extensions independently", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const pluginDir = join(root, "✏️s", "🔌️plugins", "📐️cad");
      const extDir = join(pluginDir, "🧩️extensions", "🏢️aec-building");
      mkdirSync(join(pluginDir, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(extDir, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(pluginDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad"\n`);
      writeFileSync(join(pluginDir, "🦀️component.rs"), `pub struct CadPlugin;\n`);
      writeFileSync(join(extDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad-aec-building"\n[dependencies]\nsemio-s-plugin-cad = { path = "../../../📦️packages/🦀️rust" }\n`);
      writeFileSync(join(extDir, "🦀️component.rs"), `pub fn configure() { b.depends_on("cad", "^1.0.0"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad" && b.priority === "high")).toEqual([]);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building")).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("flags declared runtime dependency with missing Cargo dependency at extension scope", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const pluginDir = join(root, "✏️s", "🔌️plugins", "📐️cad");
      const extDir = join(pluginDir, "🧩️extensions", "🏢️aec-building");
      mkdirSync(join(pluginDir, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(extDir, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(pluginDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad"\n`);
      writeFileSync(join(pluginDir, "🦀️component.rs"), `pub struct CadPlugin;\n`);
      writeFileSync(join(extDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad-aec-building"\n`);
      writeFileSync(join(extDir, "🦀️component.rs"), `pub fn configure() { b.depends_on("cad", "^1.0.0"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad")).toEqual([]);
      const extBreaches = breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building");
      expect(extBreaches.length).toBe(1);
      expect(extBreaches[0]!.priority).toBe("high");
      expect(extBreaches[0]!.id).toBe("plugin-dependency-missing-cargo-✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building-cad");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️PluginDependencyParityPolicy

describe("🏛️ layering", () => {
  const repoRoot = getWorkspaceRoot();

  test("the ratchet never allows a file to grow past its baseline", () => {
    expect(layeringBreaches(repoRoot)).toEqual([]);
  }, 60_000);

  test("no repo-wide or framework file exceeds what the baseline records", () => {
    const baseline = loadLayeringBaseline(repoRoot);
    for (const [file, count] of Object.entries(layeringCounts(layeringReferences(repoRoot)))) {
      expect(count, `${file} references more implementation paths than its baseline`).toBeLessThanOrEqual(baseline.allowed[file] ?? 0);
    }
  }, 60_000);

  test("the baseline lists no file that is already clean", () => {
    const counts = layeringCounts(layeringReferences(repoRoot));
    for (const [file, allowed] of Object.entries(loadLayeringBaseline(repoRoot).allowed)) {
      if (allowed === 0) expect(counts[file] ?? 0, `${file} is baselined at 0 and should be dropped from the list`).toBe(0);
    }
  }, 60_000);

  test("an area contributes its own policy exemptions, and the router merges them by discovery", () => {
    const merged = policyDiscoveredAllowlist(repoRoot, "semantic-vocabulary");
    expect(merged.size).toBeGreaterThan(0);
    // 🧭️Every entry must live under the area that contributed it — an exemption never outlives the
    // code it exempts, which is the whole point of moving it out of the root router.
    for (const entry of merged) expect(entry.length).toBeGreaterThan(0);
    expect(policyDiscoveredAllowlist(repoRoot, "no-such-rule-key").size).toBe(0);
  }, 60_000);
});

//#region 🛤️ArtifactPathProjectionAuthority
type ArtifactProjectionGolden = Readonly<{
  schemaVersion: 1;
  mappingDigestAlgorithm: "sha256-source-nul-destination-lines-v1";
  projections: readonly ArtifactProjectionGoldenEntry[];
}>;

type ArtifactProjectionGoldenEntry = Readonly<{
  contractId: "artifact-example-model-catalog-v1" | "artifact-editor-command-bundle-v1";
  rationaleRule: string;
  artifactId: string;
  standardVersion: string;
  subsetId: string;
  profileDirectoryName: string;
  sourceRoot: string;
  destinationRoot: string;
  sourceFileCount: number;
  destinationDirectoryCount: number;
  destinationNodeCount: number;
  maxPathBytes: number;
  mappingDigest: string;
  mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[];
  referenceEdits?: readonly Readonly<{ path: string; adapter: "json" | "toml"; structuredLocation: string; oldValue: string; newValue: string; preimageHash: string }>[];
  modelCatalog?: Readonly<{
    models: readonly Readonly<{ directoryName: string; id: string; schema: string; version: string }>[];
    categoryRules: readonly Readonly<{ sourceDirectoryName: string; sourceShape: string; manifestSchema: string; count: number }>[];
  }>;
}>;

const ARTIFACT_PROJECTION_GOLDEN = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures", "🧪️cad-draw-path-projection", "🔣️.json"), "utf8")) as ArtifactProjectionGolden;

type DrawSourceScenario = Readonly<{
  schemaVersion: 1;
  contractId: "draw-source-scenario-input-v1";
  registryImplementation: Readonly<{ format: "typescript"; content: string }>;
  cargoModuleRoot: DrawSourceScenarioInput;
  owner: Readonly<{ artifactId: string; standardVersion: string; subsetId: string; commandDirectoryName: string }>;
  members: readonly DrawSourceScenarioInput[];
  consumers: readonly DrawSourceScenarioInput[];
  retention: Readonly<{ parentSegments: readonly string[]; recordFilename: string; disposition: string; cases: readonly Readonly<{ kind: "directory" | "symlink" | "file" | "missing" | "unreadable"; allowed: boolean; creates: number }>[] }>;
  oracle: Readonly<{
    sourceDirectoryCount: number;
    destinationDirectoryCount: number;
    destinationNodeCount: number;
    configuration: readonly Readonly<{ path: string; sourceEntry: string; destinationEntry: string }>[];
    workspaceMemberCount: number;
    dependencyUserCount: number;
    projectCwdCount: number;
    workspaceGlobCount: number;
    rootCollectionCount: number;
    externalModuleCount: number;
    pluginRegistryRegenerationCount: number;
    cargoModuleMount: Readonly<{ manifestPath: string; packageName: string; libraryEntry: string; moduleName: string; moduleTarget: string }>;
  }>;
}>;

type DrawSourceScenarioInput = Readonly<{ path: string; format: "json" | "toml" | "typescript" | "rust"; content: string }>;

const DRAW_SOURCE_SCENARIO_ROOT = join(import.meta.dir, "../../🧪️tests/🧪️draw-source-scenario");
const DRAW_SOURCE_SCENARIO = JSON.parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "🔣️.json"), "utf8")) as DrawSourceScenario;

function projectionByteSort(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

function projectionGolden(contractId: ArtifactProjectionGoldenEntry["contractId"]): ArtifactProjectionGoldenEntry {
  const projection = ARTIFACT_PROJECTION_GOLDEN.projections.find((candidate) => candidate.contractId === contractId);
  if (!projection) throw new Error(`Missing projection golden ${contractId}.`);
  return projection;
}

function drawSourceScenarioContent(content: string, projection: ArtifactProjectionGoldenEntry): string {
  if (projection.contractId !== "artifact-editor-command-bundle-v1") throw new Error("Authored Draw inputs require the exact Draw projection");
  const rendered = content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
  if (rendered.includes("{{")) throw new Error("Authored Draw input has an unknown binding");
  return rendered;
}

function drawSourceScenarioPreimage(destinationPath: string): string {
  const projection = projectionGolden("artifact-editor-command-bundle-v1");
  const mapping = projection.mappings.find((row) => row.destinationPath === destinationPath);
  const member = mapping && DRAW_SOURCE_SCENARIO.members.find(({ path }) => `${projection.sourceRoot}/${path}` === mapping.sourcePath);
  if (!member) throw new Error(`Authored Draw preimage does not own ${destinationPath}`);
  return createHash("sha256").update(drawSourceScenarioContent(member.content, projection)).digest("hex");
}

function projectionAuthorityNodes(projection: ArtifactProjectionGoldenEntry, source: "live" | "authored-draw" = "live", readLive: (path: string) => string = (path) => readFileSync(join(getWorkspaceRoot(), path), "utf8")): SemanticProjectionAuthorityNode[] {
  if (source === "authored-draw" && projection.contractId !== "artifact-editor-command-bundle-v1") throw new Error("Authored Draw inputs cannot supply another projection");
  const directories = new Set<string>([projection.sourceRoot]);
  const files = projection.mappings.map(({ sourcePath }) => {
    let owner = sourcePath.slice(0, sourcePath.lastIndexOf("/"));
    while (owner.length >= projection.sourceRoot.length) {
      directories.add(owner);
      if (owner === projection.sourceRoot) break;
      owner = owner.slice(0, owner.lastIndexOf("/"));
    }
    const member = source === "authored-draw" ? DRAW_SOURCE_SCENARIO.members.find(({ path }) => `${projection.sourceRoot}/${path}` === sourcePath) : undefined;
    if (source === "authored-draw" && !member) throw new Error(`Authored Draw input does not own ${sourcePath}`);
    return { path: sourcePath, nodeKind: "file" as const, content: member ? drawSourceScenarioContent(member.content, projection) : readLive(sourcePath) };
  });
  return [...[...directories].sort(projectionByteSort).map((path) => ({ path, nodeKind: "directory" as const })), ...files];
}

function projectionAuthority(projection: ArtifactProjectionGoldenEntry, nodes = projectionAuthorityNodes(projection), occupiedPaths: readonly string[] = []) {
  return semanticPathProjectionAuthority({
    artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")),
    contractId: projection.contractId,
    nodes,
    occupiedPaths,
    sourceRoot: projection.sourceRoot,
  });
}

describe("artifact path projection authority", () => {
  test("authored Draw source schema and parser oracles retain the exact eleven-member contract", async () => {
    const schema = JSON.parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(DRAW_SOURCE_SCENARIO)).toBe(true);
    for (const changed of [
      { ...DRAW_SOURCE_SCENARIO, schemaVersion: 2 },
      { ...DRAW_SOURCE_SCENARIO, extra: true },
      { ...DRAW_SOURCE_SCENARIO, members: DRAW_SOURCE_SCENARIO.members.slice(1) },
      { ...DRAW_SOURCE_SCENARIO, members: [...DRAW_SOURCE_SCENARIO.members, DRAW_SOURCE_SCENARIO.members[0]] },
      { ...DRAW_SOURCE_SCENARIO, members: DRAW_SOURCE_SCENARIO.members.map((row, index) => index === 1 ? { ...row, path: DRAW_SOURCE_SCENARIO.members[0]!.path } : row) },
      { ...DRAW_SOURCE_SCENARIO, consumers: DRAW_SOURCE_SCENARIO.consumers.map((row, index) => index === 0 ? { ...row, path: "../Cargo.toml" } : row) },
      { ...DRAW_SOURCE_SCENARIO, owner: { ...DRAW_SOURCE_SCENARIO.owner, artifactId: "🧪️counterfeit" } },
      { ...DRAW_SOURCE_SCENARIO, registryImplementation: { format: "typescript", content: 'import "./🧪️undeclared.ts";\n' } },
      { ...DRAW_SOURCE_SCENARIO, oracle: { ...DRAW_SOURCE_SCENARIO.oracle, pluginRegistryRegenerationCount: 1 } },
      { ...DRAW_SOURCE_SCENARIO, cargoModuleRoot: { ...DRAW_SOURCE_SCENARIO.cargoModuleRoot, path: "../🦀️.rs" } },
    ]) expect(validate(changed)).toBe(false);
    const projection = projectionGolden("artifact-editor-command-bundle-v1");
    expect(DRAW_SOURCE_SCENARIO.members.map(({ path }) => `${projection.sourceRoot}/${path}`)).toEqual(projection.mappings.map(({ sourcePath }) => sourcePath));
    expect(DRAW_SOURCE_SCENARIO.owner).toEqual({ artifactId: projection.artifactId, standardVersion: projection.standardVersion, subsetId: projection.subsetId, commandDirectoryName: basename(projection.sourceRoot) });
    expect(DRAW_SOURCE_SCENARIO.members.reduce((total, row) => total + Buffer.byteLength(row.content), 0)).toBeLessThan(10_000);
    const jsonc = await import("jsonc-parser"), ts = await import("typescript");
    const render = (content: string): string => content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
    let projectCwds = 0, workspaceGlobs = 0;
    for (const row of [...DRAW_SOURCE_SCENARIO.members, ...DRAW_SOURCE_SCENARIO.consumers, { path: "📜️script.ts", ...DRAW_SOURCE_SCENARIO.registryImplementation }]) {
      const content = render(row.content);
      expect(content).not.toContain("{{");
      if (row.format === "json") {
        const errors: import("jsonc-parser").ParseError[] = [];
        const parsed = jsonc.parse(content, errors, { disallowComments: true, allowTrailingComma: false });
        expect(errors).toEqual([]);
        expect(parsed).toEqual(JSON.parse(content));
        if (row.path.endsWith("📋️project.json")) {
          projectCwds += Object.values(parsed.targets as Record<string, { options: { cwd: string; command: string } }>).filter(({ options }) => options.cwd.startsWith(projection.sourceRoot) && options.command.startsWith("bun ./📜️script.ts test")).length;
          workspaceGlobs += parsed.namedInputs.default.filter((value: string) => value.startsWith("{workspaceRoot}/") && value.endsWith("/**/*.rs")).length;
        } else expect(parsed.entries.flatMap((entry: { users: string[] }) => entry.users)).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.dependencyUserCount);
      } else if (row.format === "toml") {
        const parsed = toml.parse(content);
        const configuration = DRAW_SOURCE_SCENARIO.oracle.configuration.find(({ path }) => path === row.path);
        if (configuration) expect((parsed.lib as { path: string }).path).toBe(configuration.sourceEntry);
        if (row.path === "Cargo.toml") expect((parsed.workspace as { members: string[] }).members).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.workspaceMemberCount);
      } else if (row.format === "typescript") {
        const result = ts.transpileModule(content, { fileName: row.path, reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } });
        expect(result.diagnostics).toEqual([]);
        const source = ts.createSourceFile(row.path, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
        const imports = source.statements.filter(ts.isImportDeclaration).map((statement) => (statement.moduleSpecifier as import("typescript").StringLiteral).text);
        expect(imports).toEqual(row.path === "📜️script.ts" ? [] : ["node:path"]);
      }
    }
    expect(projectCwds).toBe(DRAW_SOURCE_SCENARIO.oracle.projectCwdCount);
    expect(workspaceGlobs).toBe(DRAW_SOURCE_SCENARIO.oracle.workspaceGlobCount);
    const { registryStaticImports } = await import("../../🔍️discovery/🟦️component.ts");
    expect(registryStaticImports(DRAW_SOURCE_SCENARIO.registryImplementation.content, "📜️script.ts")).toEqual([]);
  });

  test("authored Draw source authority never reads live Draw leaves or reconstructs canonical input", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1");
    const nodes = projectionAuthorityNodes(projection, "authored-draw", () => { throw new Error("Live Draw input must not be read by a source scenario"); });
    expect(nodes.filter(({ nodeKind }) => nodeKind === "directory")).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.sourceDirectoryCount);
    for (const row of DRAW_SOURCE_SCENARIO.members) {
      const expected = row.content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
      expect(nodes.find(({ path }) => path === `${projection.sourceRoot}/${row.path}`)?.content).toBe(expected);
    }
    const authority = projectionAuthority(projection, nodes);
    expect(authority.problems).toEqual([]);
    expect(authority.mappings).toEqual(projection.mappings);
    expect(authority.mappingDigest).toBe(projection.mappingDigest);
    expect(authority.destinationDirectoryCount).toBe(DRAW_SOURCE_SCENARIO.oracle.destinationDirectoryCount);
    expect(authority.destinationNodeCount).toBe(DRAW_SOURCE_SCENARIO.oracle.destinationNodeCount);
    expect(authority.referenceEdits).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.configuration.length);
    for (const configuration of DRAW_SOURCE_SCENARIO.oracle.configuration) {
      const mapping = projection.mappings.find(({ sourcePath }) => sourcePath === `${projection.sourceRoot}/${configuration.path}`)!;
      const content = DRAW_SOURCE_SCENARIO.members.find(({ path }) => path === configuration.path)!.content;
      expect((toml.parse(content).lib as { path: string }).path).toBe(configuration.sourceEntry);
      expect(authority.referenceEdits.find(({ path }) => path === mapping.destinationPath)).toEqual({ path: mapping.destinationPath, adapter: "toml", structuredLocation: "lib.path", oldValue: configuration.sourceEntry, newValue: configuration.destinationEntry, preimageHash: createHash("sha256").update(content).digest("hex") });
    }
    expect(() => projectionAuthorityNodes(projectionGolden("artifact-example-model-catalog-v1"), "authored-draw")).toThrow("cannot supply another projection");
    expect(() => drawSourceScenarioContent("{{unknown}}", projection)).toThrow("unknown binding");
    expect(() => projectionAuthorityNodes({ ...projection, mappings: [{ sourcePath: `${projection.sourceRoot}/🧪️unknown`, destinationPath: `${projection.destinationRoot}/🧪️unknown` }] }, "authored-draw")).toThrow("does not own");
    expect(() => projectionAuthorityNodes(projection, "live", () => { throw new Error("Missing live source input"); })).toThrow("Missing live source input");
  });

  test("authored Draw source Cargo mount agrees with independent TOML and Rust parser ownership", async () => {
    const { cargoModuleRoot: root, oracle: { cargoModuleMount: mount } } = DRAW_SOURCE_SCENARIO;
    const files = artifactProjectionSourceFiles(true);
    expect(files[root.path]).toBe(root.content);
    const manifest = toml.parse(files[mount.manifestPath]!);
    expect((manifest.package as { name: string }).name).toBe(mount.packageName);
    expect((manifest.lib as { path: string }).path).toBe(mount.libraryEntry);
    const module = inspectRustModuleGraphFacts(root.content).modules;
    expect(module.map(({ name, pathTarget }) => ({ name, pathTarget }))).toEqual([{ name: mount.moduleName, pathTarget: mount.moduleTarget }]);
    const virtualRoot = resolve("/draw-source-fixture");
    expect(resolve(virtualRoot, dirname(mount.manifestPath), mount.libraryEntry)).toBe(resolve(virtualRoot, root.path));
    const modulePath = relative(virtualRoot, resolve(virtualRoot, dirname(root.path), mount.moduleTarget)).replaceAll("\\", "/");
    const { inspectRustModuleGraph } = await import("../../🔍️discovery/🟦️component.ts");
    const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { conventionalRoots: false, strictManifests: true });
    expect([...new Set(graph.contexts.get(modulePath)?.map(({ manifestPath }) => manifestPath))]).toEqual([mount.manifestPath]);
    expect(graph.contexts.get(modulePath)?.filter(({ modulePath: segments }) => segments.join("::") === mount.moduleName).map(({ manifestPath }) => manifestPath)).toEqual([mount.manifestPath]);
    const parser = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "draw_source_mount", "--edition=2021", "--crate-type=lib", "-"], { input: root.content, encoding: "utf8", timeout: 10_000 });
    expect({ status: parser.status, signal: parser.signal, stderr: parser.stderr }).toEqual({ status: 0, signal: null, stderr: "" });
    expect(parser.stdout.match(/kind: Mod\(/gu)).toHaveLength(1);
    const name = parser.stdout.match(/kind: Mod\(\s*Default,\s*([a-z_][a-z_0-9]*)#[0-9]+,\s*Unloaded,/u)?.[1];
    const literal = parser.stdout.match(/symbol: ("(?:[^"\\]|\\.)*")/u)?.[1];
    const pathTarget = literal === undefined ? undefined : JSON.parse(literal.replace(/\\u\{([0-9a-f]+)\}/gu, (_, code: string) => String.fromCodePoint(Number.parseInt(code, 16))));
    expect([{ name, pathTarget }]).toEqual(module.map(({ name, pathTarget }) => ({ name, pathTarget })));
  });

  test("authored Draw source scenarios isolate every member and external consumer from live Draw reads", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1"), reads: string[] = [];
    const files = artifactProjectionSourceFiles(true, (path) => {
      if (path.startsWith("✏️s/🔌️plugins/🖍️draw/") || path === "Cargo.toml" || path === "🔒️dependencies.json") throw new Error(`Unexpected live Draw source-scenario read: ${path}`);
      reads.push(path);
      return readFileSync(join(getWorkspaceRoot(), path), "utf8");
    });
    expect(reads).toHaveLength(projectionGolden("artifact-example-model-catalog-v1").sourceFileCount + 4);
    for (const row of DRAW_SOURCE_SCENARIO.members) expect(files[`${artifactProjectionFixturePath(projection.sourceRoot)}/${row.path}`]).toBe(drawSourceScenarioContent(row.content, projection));
    for (const row of DRAW_SOURCE_SCENARIO.consumers) expect(files[row.path]).toBe(drawSourceScenarioContent(row.content, projection));
  });

  test("authored Draw source run parent is exact and rejects unsafe ancestry before allocation", () => {
    const expected = [NORMALIZATION_TICKET_REL, ...DRAW_SOURCE_SCENARIO.retention.parentSegments].join("/");
    expect(expected).toBe(ARTIFACT_PROJECTION_RUN_PARENT);
    for (const vector of DRAW_SOURCE_SCENARIO.retention.cases) {
      let created = 0;
      const inspect = (path: string) => {
        const selected = path === resolve(getWorkspaceRoot(), expected);
        if (selected && (vector.kind === "unreadable" || vector.kind === "missing" && created === 0)) throw Object.assign(new Error(vector.kind), { code: vector.kind === "missing" ? "ENOENT" : "EACCES" });
        return { isDirectory: () => !selected || vector.kind === "directory" || vector.kind === "missing" && created === 1, isSymbolicLink: () => selected && vector.kind === "symlink" };
      };
      const run = () => normalizationRetainedRunParent(expected, inspect, (path) => { expect(path).toBe(resolve(getWorkspaceRoot(), expected)); created++; });
      if (vector.allowed) expect(run()).toBe(resolve(getWorkspaceRoot(), expected));
      else expect(run).toThrow();
      expect(created).toBe(vector.creates);
    }
    expect(() => normalizationRetainedRunParent(`${expected}/../🧪️elsewhere`, () => { throw new Error("must not inspect"); }, () => { throw new Error("must not create"); })).toThrow("exact artifact run parent");
  });

  test("canonical CAD and Draw authority retains the exact language-neutral mapping digests", () => {
    for (const projection of ARTIFACT_PROJECTION_GOLDEN.projections) {
      const source = projectionAuthorityNodes(projection, projection.contractId === "artifact-editor-command-bundle-v1" ? "authored-draw" : "live");
      const sourceAuthority = projectionAuthority(projection, source);
      const directories = new Set<string>([projection.destinationRoot]);
      const files = projection.mappings.map(({ sourcePath, destinationPath }) => {
        let content = source.find(({ path }) => path === sourcePath)!.content!;
        for (const edit of sourceAuthority.referenceEdits.filter(({ path }) => path === destinationPath)) content = content.replace(JSON.stringify(edit.oldValue), JSON.stringify(edit.newValue));
        for (let path = dirname(destinationPath); path !== dirname(projection.destinationRoot); path = dirname(path)) directories.add(path);
        return { path: destinationPath, nodeKind: "file" as const, content };
      });
      const nodes = [...[...directories].map((path) => ({ path, nodeKind: "directory" as const })), ...files];
      const result = semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes, layout: "destination" });
      expect(result.problems).toEqual([]);
      expect(result.mappings).toEqual(projection.mappings);
      expect(result.mappingDigest).toBe(projection.mappingDigest);
      expect(result.referenceEdits).toEqual([]);
      const invalid = nodes.map((node) => node.nodeKind === "file" && node.path === files[0]!.path ? { ...node, path: `${node.path}.unexpected` } : node);
      expect(semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes: invalid, layout: "destination" }).problems.length).toBeGreaterThan(0);
      const symlink = nodes.map((node) => node.path === files[0]!.path ? { path: node.path, nodeKind: "symlink" as const } : node);
      expect(semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes: symlink, layout: "destination" }).problems.some((problem) => problem.includes("forbidden symlink"))).toBe(true);
    }
  });

  test("CAD profile vectors reject missing, duplicate, and ambiguous forward authority", () => {
    const taxonomy = loadTaxonomy();
    const catalog = taxonomy.semanticPathProjectionCatalogContracts["cad-model-catalog-v1"]!;
    expect("profileVectors" in catalog && catalog.profileVectors).toEqual([{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }]);
    for (const vectors of [[], [{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }, { artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }], [{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any/nested" }]]) {
      const changed = structuredClone(taxonomy);
      (changed.semanticPathProjectionCatalogContracts["cad-model-catalog-v1"] as { profileVectors: unknown }).profileVectors = vectors;
      expect(validateTaxonomy(changed).some((problem) => /profile|tuple/u.test(problem))).toBe(true);
    }
  });

  test("artifact-example-model-catalog-projection is schema-owned and agrees with owned filesystem discovery", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    const result = projectionAuthority(projection);
    const platformBoundary = ownedFilePaths(join(getWorkspaceRoot(), projection.sourceRoot)).filter((path) => path.endsWith(".json"))
      .map((path) => `${projection.sourceRoot}/${path}`)
      .sort(projectionByteSort);
    expect(platformBoundary).toEqual(projection.mappings.map(({ sourcePath }) => sourcePath));
    expect(result.problems).toEqual([]);
    expect(result.mappings).toEqual(projection.mappings);
    expect(result.destinationRoot).toBe(projection.destinationRoot);
    expect(result.mappingDigest).toBe("a09f60c5de5718394ddb856052444b306de7443b2d4ecd546e1e911dc44d40a6");
    expect(result.destinationDirectoryCount).toBe(244);
    expect(result.destinationNodeCount).toBe(453);
    expect(result.maxPathBytes).toBe(237);
    expect(projection.modelCatalog?.models).toHaveLength(9);
    expect(projection.modelCatalog?.categoryRules.map(({ count }) => count).reduce((sum, count) => sum + count, 0)).toBe(200);
  });

  test("artifact-example-model-catalog-projection fails closed for invalid authority and path states", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    const nodes = projectionAuthorityNodes(projection);
    const concreteManifest = nodes.find(({ path }) => path.endsWith("/🧱️aec.building.concrete/🔣️modelDefinition.json"))!;
    expect(projectionAuthority(projection, nodes.filter((node) => node !== concreteManifest)).problems.some((problem) => problem.toLocaleLowerCase("und").includes("model manifest"))).toBe(true);

    const action = nodes.find((node) => node.nodeKind === "file" && node.path.includes("/🎬️actions/"))!;
    const unknownSchema = nodes.map((node) => node === action ? { ...node, content: JSON.stringify({ id: "invalid", schema: "spatial.unknown", version: "1.0.0" }) } : node);
    expect(projectionAuthority(projection, unknownSchema).problems.some((problem) => problem.includes("manifest schema"))).toBe(true);

    const shapeManifest = nodes.find(({ path }) => path.endsWith("/📐️spatial.shape/🔣️modelDefinition.json"))!;
    const duplicateModel = nodes.map((node) => node === concreteManifest ? { ...node, content: shapeManifest.content } : node);
    expect(projectionAuthority(projection, duplicateModel).problems.some((problem) => problem.includes("duplicated"))).toBe(true);

    const unknownCategoryRoot = `${projection.sourceRoot}/📐️spatial.shape/🧪️unknown`;
    const unknownCategory: SemanticProjectionAuthorityNode[] = [...nodes, { path: unknownCategoryRoot, nodeKind: "directory" }, { path: `${unknownCategoryRoot}/🔣️member.json`, nodeKind: "file", content: JSON.stringify({ id: "unknown", schema: "spatial.unknown", version: "1.0.0" }) }];
    expect(projectionAuthority(projection, unknownCategory).problems.some((problem) => problem.includes("Unknown CAD catalog category"))).toBe(true);

    const symlink = [...nodes, { path: `${projection.sourceRoot}/🧪️symlink`, nodeKind: "symlink" as const }];
    expect(projectionAuthority(projection, symlink).problems.some((problem) => problem.includes("symlink"))).toBe(true);
    const vs15 = [...nodes, { path: `${projection.sourceRoot}/🧪︎vs15`, nodeKind: "symlink" as const }];
    expect(projectionAuthority(projection, vs15).problems.some((problem) => problem.includes("VS15"))).toBe(true);

    const reverse = semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.destinationRoot, nodes });
    expect(reverse.problems.some((problem) => problem.includes("exact projection grammar"))).toBe(true);

    const modelRoot = `${projection.sourceRoot}/📐️spatial.shape`;
    const collisionStem = "CaseCollision";
    const collisionNodes: SemanticProjectionAuthorityNode[] = [
      ...nodes,
      { path: `${modelRoot}/🎬️actions/🔣️${collisionStem}.json`, nodeKind: "file", content: JSON.stringify({ id: "case-a", schema: "spatial.action", version: "1.0.0" }) },
      { path: `${modelRoot}/🎬️actions/🔣️${collisionStem.toLocaleLowerCase("und")}.json`, nodeKind: "file", content: JSON.stringify({ id: "case-b", schema: "spatial.action", version: "1.0.0" }) },
    ];
    expect(projectionAuthority(projection, collisionNodes).problems.some((problem) => problem.includes("case-fold"))).toBe(true);

    const longStem = "path".repeat(40);
    const longNodes: SemanticProjectionAuthorityNode[] = [...nodes, { path: `${modelRoot}/🎬️actions/🔣️${longStem}.json`, nodeKind: "file", content: JSON.stringify({ id: "long", schema: "spatial.action", version: "1.0.0" }) }];
    expect(projectionAuthority(projection, longNodes).problems.some((problem) => problem.includes("maxPathBytes"))).toBe(true);
    expect(projectionAuthority(projection, nodes, [projection.mappings[0]!.destinationPath]).problems.some((problem) => problem.includes("occupied"))).toBe(true);
  });

  test("artifact-editor-command-projection preserves the strict fixed-file union and owned filesystem parity", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1");
    const nodes = projectionAuthorityNodes(projection);
    const result = projectionAuthority(projection, nodes);
    const platformBoundary = ownedFilePaths(join(getWorkspaceRoot(), projection.sourceRoot))
      .map((path) => `${projection.sourceRoot}/${path}`)
      .sort(projectionByteSort);
    expect(platformBoundary).toEqual(projection.mappings.map(({ sourcePath }) => sourcePath));
    expect(result.problems).toEqual([]);
    expect(result.mappings).toEqual(projection.mappings);
    expect(result.mappingDigest).toBe(projection.mappingDigest);
    expect(result.destinationDirectoryCount).toBe(projection.destinationDirectoryCount);
    expect(result.destinationNodeCount).toBe(projection.destinationNodeCount);
    expect(result.maxPathBytes).toBe(projection.maxPathBytes);
    expect(result.referenceEdits).toEqual(projection.referenceEdits ?? []);

    const partial = nodes.filter(({ path }) => !path.endsWith("/🔄️fsm/✨️macros/📦️packages/🦀️rust/📋️project.json"));
    expect(projectionAuthority(projection, partial).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);
    const extra = [...nodes, { path: `${projection.sourceRoot}/🔣️extra.json`, nodeKind: "file" as const, content: "{}" }];
    expect(projectionAuthority(projection, extra).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const invalid = structuredClone(loadTaxonomy()) as Taxonomy;
    const contract = invalid.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { requiredNodes: Record<string, unknown>[] };
    const fixedNode = contract.requiredNodes.find((node) => node.fixedFilenameContractId === "nx-project-manifest")!;
    delete fixedNode.fixedFilenameContractId;
    fixedNode.kindId = "json";
    expect(validateTaxonomy(invalid).some((problem) => problem.includes("draw-editor-command-bundle-v1"))).toBe(true);
  });
});
//#endregion 🛤️ArtifactPathProjectionAuthority

//#region 🧹️TaxonomyNormalization
type NormalizationFixture = {
  baselineCommit: string;
  opaqueDigest: OpaqueTreeDigest;
  options: TaxonomyInventoryOptions;
  repoRoot: string;
  root: string;
  scope: string;
  ticketDir: string;
  workspace: string;
};

type MutationProjectionGolden = Readonly<{
  schemaVersion: 1;
  contract: "artifact-mutation-test-projection-v1";
  standardDirectoryName: string;
  subsetDirectoryName: string;
  profileDirectoryName: string;
  registryCounts: Readonly<{ catalogs: number; vectors: number; scenarios: number; changedMutationRows: number; changedMutationSources: number }>;
  sourceGlob: string;
  bundle: readonly Readonly<{ source: string; destination: string }>[];
  cases: readonly Readonly<{ mutationId: string; sourceMutationDirectoryName: string; mutationDirectoryName: string; sourceScenarioId: string; scenarioId: string }>[];
}>;

const NORMALIZATION_TICKET_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION";
const ARTIFACT_PROJECTION_RUN_PARENT = `${NORMALIZATION_TICKET_REL}/📓️draw-source-scenarios/🧪️runs`;
const ARTIFACT_PROJECTION_RETAINED_RUNS = new Map<string, Readonly<{ device: number; inode: number; reportHash: string; name: string; startedAt: string }>>();
const NORMALIZATION_SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const TICKET_IMPORTANT_HISTORY_ROOT_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/SCOPED-HISTORY-APPLY";
const TICKET_IMPORTANT_HISTORY_SOURCE_REL = `${TICKET_IMPORTANT_HISTORY_ROOT_REL}/📌️important.md`;
const TICKET_IMPORTANT_HISTORY_DESTINATION_REL = `${TICKET_IMPORTANT_HISTORY_ROOT_REL}/📓️important/📝️.md`;
const MUTATION_PROJECTION_GOLDEN = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures", "🧪️mutation-path-projection", "🔣️.json"), "utf8")) as MutationProjectionGolden;
const SCOPED_INVENTORY_PATHSPEC_GOLDEN = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures", "🧪️scoped-inventory-pathspec", "🔣️.json"), "utf8")) as Readonly<{
  schemaVersion: 1;
  opaqueExclusions: readonly string[];
  cases: readonly Readonly<{ id: string; inputScope: string | null; normalizedScope: string | null; conservativePrefix: string; positivePathspec: string; exclusionPathspecs: readonly string[] }>[];
}>;

/** 🧭️ Discovers mutation catalog files without third-party glob semantics. */
function nativeMutationCatalogPaths(workspace: string): string[] {
  const root = join(workspace, "✏️s", "🔌️plugins");
  const paths: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const absolute = join(directory, entry.name);
      if (entry.isDirectory()) walk(absolute);
      else if (absolute.replaceAll("\\", "/").endsWith("/🧪️oracle/🔣️.json")) paths.push(relative(workspace, absolute).replaceAll("\\", "/"));
    }
  };
  walk(root);
  return paths.sort();
}

function mutationGoldenSource(entry: MutationProjectionGolden["cases"][number], leaf: string): string {
  return `🏅️standards/${MUTATION_PROJECTION_GOLDEN.standardDirectoryName}/🪆️subsets/${MUTATION_PROJECTION_GOLDEN.subsetDirectoryName}/🧬️schema/🧬️mutations/${entry.sourceMutationDirectoryName}/🧪️tests/${entry.sourceScenarioId}/${leaf}`;
}

function mutationGoldenDestination(entry: MutationProjectionGolden["cases"][number], leaf: string): string {
  return `🧪️tests/${MUTATION_PROJECTION_GOLDEN.profileDirectoryName}/${entry.mutationDirectoryName}/🧪️${entry.scenarioId}/${leaf}`;
}

function normalizationGit(root: string, args: readonly string[]): string {
  const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const stdout = result.stdout.toString();
  const stderr = result.stderr.toString();
  if (result.exitCode !== 0) throw new Error(`fixture git ${args.join(" ")} failed: ${stderr || stdout}`);
  return stdout.trim();
}

function normalizationWriteFiles(root: string, files: Readonly<Record<string, string>>): void {
  for (const [path, content] of Object.entries(files)) {
    const absolute = join(root, path);
    mkdirSync(resolve(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  }
}

function normalizationRetainedRunParent(parent: string, inspect: (path: string) => { isDirectory(): boolean; isSymbolicLink(): boolean } = lstatSync, create: (path: string) => void = (path) => { mkdirSync(path); }): string {
  if (parent !== ARTIFACT_PROJECTION_RUN_PARENT) throw new Error("Retained normalization requires the exact artifact run parent");
  let current = getWorkspaceRoot();
  const workspace = inspect(current);
  if (!workspace.isDirectory() || workspace.isSymbolicLink()) throw new Error("Retained fixture workspace is not a no-follow directory");
  for (const segment of parent.split("/")) {
    current = join(current, segment);
    let node: ReturnType<typeof inspect>;
    try { node = inspect(current); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      create(current);
      node = inspect(current);
    }
    if (!node.isDirectory() || node.isSymbolicLink()) throw new Error(`Retained fixture ancestor is not a no-follow directory: ${current}`);
  }
  return current;
}

function retainArtifactProjectionFixture(fixture: NormalizationFixture, passed: boolean): void {
  const record = ARTIFACT_PROJECTION_RETAINED_RUNS.get(fixture.root);
  const parent = normalizationRetainedRunParent(ARTIFACT_PROJECTION_RUN_PARENT);
  if (!record || dirname(fixture.root) !== parent) throw new Error("Artifact fixture run has no exact retention ownership");
  const root = lstatSync(fixture.root), reportPath = join(fixture.root, "📝️.md"), report = lstatSync(reportPath);
  if (!root.isDirectory() || root.isSymbolicLink() || root.dev !== record.device || root.ino !== record.inode || !report.isFile() || report.isSymbolicLink()) throw new Error("Artifact fixture retention node identity changed");
  if (createHash("sha256").update(readFileSync(reportPath)).digest("hex") !== record.reportHash) throw new Error("Artifact fixture retention report changed");
  writeFileSync(reportPath, `# Retained Artifact Projection Run\n\nScenario: ${record.name}\n\nStarted: ${record.startedAt}\n\nFinished: ${new Date().toISOString()}\n\nOutcome: ${passed ? "passed" : "failed or interrupted before completion"}\n\nAll authored inputs and generated/recovery evidence remain retained pending exact review. No output or fixture input was deleted by the finalizer.\n`);
}

function normalizationFixture(name: string, files: Readonly<Record<string, string>>, configure?: (fixture: { repoRoot: string; ticketDir: string; workspace: string }) => void, retainedRunParent?: string): NormalizationFixture {
  const owner = retainedRunParent === undefined ? resolve(getWorkspaceRoot(), NORMALIZATION_TICKET_REL) : normalizationRetainedRunParent(retainedRunParent);
  mkdirSync(owner, { recursive: true });
  const root = mkdtempSync(join(owner, `🧪️s-test-${name}-`));
  if (retainedRunParent !== undefined) {
    const node = lstatSync(root), startedAt = new Date().toISOString();
    if (!node.isDirectory() || node.isSymbolicLink() || dirname(root) !== owner) throw new Error("Retained fixture allocation lost its unique directory ownership");
    const report = `# Retained Artifact Projection Run\n\nScenario: ${name}\n\nStarted: ${startedAt}\n\nOutcome: prepared; final completion not yet recorded\n\nAll authored inputs and generated/recovery evidence remain retained pending exact review.\n`;
    writeFileSync(join(root, "📝️.md"), report, { flag: "wx" });
    ARTIFACT_PROJECTION_RETAINED_RUNS.set(root, { device: node.dev, inode: node.ino, reportHash: createHash("sha256").update(report).digest("hex"), name, startedAt });
  }
  const ticketDir = join(root, "🧪️tests");
  const workspace = join(ticketDir, "🧪️fixture");
  const schemaPath = join(root, NORMALIZATION_SCHEMA_REL);
  mkdirSync(resolve(schemaPath, ".."), { recursive: true });
  mkdirSync(join(root, "compose"), { recursive: true });
  mkdirSync(workspace, { recursive: true });
  writeFileSync(schemaPath, readFileSync(resolve(getWorkspaceRoot(), NORMALIZATION_SCHEMA_REL), "utf8"));
  writeFileSync(join(root, "compose", "keep.txt"), "opaque\n");
  normalizationWriteFiles(workspace, files);
  configure?.({ repoRoot: root, ticketDir, workspace });
  normalizationGit(root, ["init", "--quiet"]);
  normalizationGit(root, ["config", "user.name", "Semio Taxonomy Fixture"]);
  normalizationGit(root, ["config", "user.email", "taxonomy-fixture@invalid.example"]);
  normalizationGit(root, ["config", "commit.gpgsign", "false"]);
  normalizationGit(root, ["add", "--all"]);
  normalizationGit(root, ["commit", "--quiet", "-m", "taxonomy fixture"]);
  const head = readFileSync(join(root, ".git", "HEAD"), "utf8").trim();
  const baselineCommit = head.startsWith("ref: ") ? readFileSync(join(root, ".git", head.slice(5)), "utf8").trim() : head;
  const scope = relative(root, workspace).replaceAll("\\", "/");
  return {
    baselineCommit,
    opaqueDigest: opaqueTreeDigest(root, "compose"),
    options: { repoRoot: root, scope, ticketDir, workers: 1 },
    repoRoot: root,
    root,
    scope,
    ticketDir,
    workspace,
  };
}

function normalizationPlan(fixture: NormalizationFixture): { inventory: TaxonomyInventory; plan: TaxonomyPlan } {
  const inventory = inventoryTaxonomy(fixture.options);
  const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
  if (ARTIFACT_PROJECTION_RETAINED_RUNS.has(fixture.root)) expect(plan.regenerations.filter(({ contractId }) => contractId === "plugin-registry")).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.pluginRegistryRegenerationCount);
  return {
    inventory,
    plan,
  };
}

function normalizationWorkspaceSnapshot(workspace: string): Readonly<Record<string, string>> {
  const rows: [string, string][] = [];
  const visit = (relativePath: string): void => {
    const path = relativePath ? join(workspace, relativePath) : workspace;
    const stat = lstatSync(path), mode = (stat.mode & 0o7777).toString(8);
    if (stat.isSymbolicLink()) rows.push([relativePath || ".", `symlink|${mode}|${Buffer.from(readlinkSync(path)).toString("base64")}`]);
    else if (stat.isFile()) rows.push([relativePath || ".", `file|${mode}|${readFileSync(path).toString("base64")}`]);
    else if (stat.isDirectory()) {
      rows.push([relativePath || ".", `directory|${mode}`]);
      for (const name of readdirSync(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) visit(relativePath ? `${relativePath}/${name}` : name);
    } else rows.push([relativePath || ".", `other|${mode}`]);
  };
  visit("");
  return Object.fromEntries(rows);
}

function normalizationMove(plan: TaxonomyPlan, sourceSuffix: string): TaxonomyPlan["moves"][number] {
  const move = plan.moves.find((candidate) => candidate.sourcePath.endsWith(sourceSuffix));
  if (!move) throw new Error(`normalization plan has no move for ${sourceSuffix}: ${canonicalJson(plan)}`);
  return move;
}

function inScopeForTest(path: string, scope: string): boolean {
  const normalizedPath = path.normalize("NFC"), normalizedScope = scope.normalize("NFC");
  return normalizedPath === normalizedScope || normalizedPath.startsWith(`${normalizedScope}/`) || normalizedScope.startsWith(`${normalizedPath}/`);
}

/** 🧾️ Reduces inventory entries to the physical census fields governed by scoped admission. */
function normalizationInventoryCensus(inventory: TaxonomyInventory): readonly Readonly<Record<string, unknown>>[] {
  return inventory.entries.filter(({ nodeKind }) => nodeKind !== "directory").map(({ sourcePath, nodeKind, mode, size, contentHash, symlinkTarget }) => ({ sourcePath, nodeKind, mode, size, contentHash, ...(symlinkTarget === undefined ? {} : { symlinkTarget }) }));
}

/** 📡️ Preserves the complete, duplicate-free inventory phase transition sequence. */
function normalizationPhaseSequence(events: readonly TaxonomyProgress[]): readonly string[] {
  return events.map(({ phase }) => phase).filter((phase, index, rows) => index === 0 || phase !== rows[index - 1]);
}

function artifactProjectionFixturePath(path: string): string {
  const marker = "/🗿️artifacts/";
  const index = path.indexOf(marker);
  if (index < 0) throw new Error(`Artifact projection path has no artifact marker: ${path}`);
  return path.slice(index + 1);
}

function artifactProjectionSourceFiles(references: boolean, readLive: (path: string) => string = (path) => readFileSync(join(getWorkspaceRoot(), path), "utf8")): Record<string, string> {
  const files: Record<string, string> = {};
  for (const projection of ARTIFACT_PROJECTION_GOLDEN.projections) for (const node of projectionAuthorityNodes(projection, projection.contractId === "artifact-editor-command-bundle-v1" ? "authored-draw" : "live", readLive)) if (node.nodeKind === "file") files[artifactProjectionFixturePath(node.path)] = node.content!;
  if (references) {
    const cadRuntime = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts";
    const cadInteraction = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🕹️interaction/🦀️component.rs";
    const interactionSpec = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs";
    const spatialKernel = "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts";
    files[artifactProjectionFixturePath(cadRuntime)] = readLive(cadRuntime);
    files[artifactProjectionFixturePath(cadInteraction)] = readLive(cadInteraction);
    files[artifactProjectionFixturePath(interactionSpec)] = readLive(interactionSpec);
    files[spatialKernel] = readLive(spatialKernel);
    const draw = projectionGolden("artifact-editor-command-bundle-v1");
    for (const row of DRAW_SOURCE_SCENARIO.consumers) files[row.path] = drawSourceScenarioContent(row.content, draw);
    files[DRAW_SOURCE_SCENARIO.cargoModuleRoot.path] = DRAW_SOURCE_SCENARIO.cargoModuleRoot.content;
  }
  return files;
}

/** 🏗️ Materializes the permanent language-neutral CAD/Draw authority under one isolated normalization scope. */
function artifactProjectionNormalizationFixture(name: string, references = false): NormalizationFixture {
  const files = artifactProjectionSourceFiles(references);
  return normalizationFixture(`artifact-projection-${name}`, files, references ? ({ repoRoot, workspace }) => {
    const scope = relative(repoRoot, workspace).replaceAll("\\", "/");
    const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
    const schema = JSON.parse(readFileSync(schemaPath, "utf8")) as { generatorContracts: Record<string, { inputDiscovery?: { implementationEntryPaths: readonly string[] } }>; fixedFilenameContracts: Record<string, unknown>; semanticPathProjectionReferenceConsumerContracts: Record<string, { sourcePathIdentities: string[]; sourcePathPattern: string }> };
    const entries = schema.generatorContracts["plugin-registry"]?.inputDiscovery?.implementationEntryPaths;
    if (!entries || entries.length !== 1) throw new Error("Authored artifact fixture requires one declared registry implementation entry");
    normalizationWriteFiles(repoRoot, { [entries[0]!]: DRAW_SOURCE_SCENARIO.registryImplementation.content });
    const fixtureConsumerPaths: Readonly<Record<string, readonly string[]>> = {
      "draw-workspace-cargo": [`${scope}/Cargo.toml`],
      "draw-dependency-registry": [`${scope}/🔣️.json`],
      "draw-workspace-script": [`${scope}/📜️script.ts`],
      "cad-spatial-kernel-geometry": schema.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"]!.sourcePathIdentities.map((path) => `${scope}/${path}`),
    };
    const escaped = (value: string): string => value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
    for (const [id, paths] of Object.entries(fixtureConsumerPaths)) {
      const contract = schema.semanticPathProjectionReferenceConsumerContracts[id]!;
      contract.sourcePathIdentities = [...paths];
      contract.sourcePathPattern = `^(?:${paths.map(escaped).join("|")})$`;
    }
    schema.fixedFilenameContracts["fixture-workspace-cargo"] = { pathPattern: `${scope}/Cargo.toml`, authority: "normalization fixture", reason: "The isolated fixture relocates the schema-owned workspace Cargo consumer without changing its fixed identity.", configurability: "unconfigurable", scope: { kind: "exact-path", path: `${scope}/Cargo.toml` }, verification: "normalization fixture inventory", expires: null };
    writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
    const cad = projectionGolden("artifact-example-model-catalog-v1");
    const draw = projectionGolden("artifact-editor-command-bundle-v1");
    const target = join(workspace, artifactProjectionFixturePath(cad.mappings[0]!.sourcePath));
    const manifest = JSON.parse(readFileSync(target, "utf8")) as Record<string, unknown>;
    manifest.projectionReferences = {
      cadSourceRoot: `${scope}/${artifactProjectionFixturePath(cad.sourceRoot)}`,
      drawSourceFile: `${scope}/${artifactProjectionFixturePath(draw.mappings[0]!.sourcePath)}`,
    };
    writeFileSync(target, `${JSON.stringify(manifest, null, 2)}\n`);
    const drawSourceRoot = artifactProjectionFixturePath(draw.sourceRoot);
    for (const mapping of draw.mappings.filter(({ sourcePath }) => sourcePath.endsWith("📋️project.json"))) {
      const project = join(workspace, artifactProjectionFixturePath(mapping.sourcePath));
      writeFileSync(project, readFileSync(project, "utf8").replaceAll(`{workspaceRoot}/${draw.sourceRoot}`, `{workspaceRoot}/${scope}/${drawSourceRoot}`).replaceAll(draw.sourceRoot, `${scope}/${drawSourceRoot}`));
    }
    for (const relativePath of ["🔣️.json", "Cargo.toml"]) {
      const target = join(workspace, relativePath);
      writeFileSync(target, readFileSync(target, "utf8").replaceAll(draw.sourceRoot, `${scope}/${drawSourceRoot}`));
    }
    const rootScript = join(workspace, "📜️script.ts");
    writeFileSync(rootScript, readFileSync(rootScript, "utf8").replaceAll("__SCOPE__", scope));
  } : undefined, ARTIFACT_PROJECTION_RUN_PARENT);
}

function scopedUnrelatedProjectionStaleFixture(name: string): Readonly<{ fixture: NormalizationFixture; externalPath: string; staleToken: string }> {
  const externalPath = `🧪️external-${name}/🗿️artifacts/📐️cad/📝️.md`;
  let staleToken = "";
  const fixture = normalizationFixture(`scoped-stale-${name}`, { "🟦️component.ts": "export const scoped = true;\n" }, ({ repoRoot }) => {
    const schema = JSON.parse(readFileSync(join(repoRoot, NORMALIZATION_SCHEMA_REL), "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { projectionContractId: string; staleMarkers: string[] }> };
    staleToken = Object.values(schema.semanticPathProjectionReferenceConsumerContracts).find((contract) => contract.projectionContractId === "artifact-example-model-catalog-v1")!.staleMarkers[0]!;
    normalizationWriteFiles(repoRoot, { [externalPath]: "# Unrelated clean owner\n" });
  });
  return { fixture, externalPath, staleToken };
}

function expectNormalizationApplyFailure(run: () => TaxonomyApplyResult): void {
  let failed = false;
  try {
    const result = run();
    failed = !/^(?:applied|committed|complete)$/u.test(String(result.state));
  } catch {
    failed = true;
  }
  expect(failed).toBe(true);
}

describe("taxonomy normalization", () => {
  test("a minimal CAD catalog commits and re-inventories without source hierarchy authority", () => {
    const cad = projectionGolden("artifact-example-model-catalog-v1");
    const member = cad.mappings.find(({ sourcePath }) => sourcePath.includes("/🎬️actions/") && sourcePath.slice(cad.sourceRoot.length + 1).split("/").length === 3)!;
    const model = member.sourcePath.slice(cad.sourceRoot.length + 1).split("/")[0]!;
    const manifest = cad.mappings.find(({ sourcePath }) => sourcePath === `${cad.sourceRoot}/${model}/🔣️modelDefinition.json`)!;
    const fixture = normalizationFixture("minimal-cad-canonical", Object.fromEntries([manifest, member].map(({ sourcePath }) => [artifactProjectionFixturePath(sourcePath), readFileSync(join(getWorkspaceRoot(), sourcePath), "utf8")])));
    try {
      const retainedDirectory = join(fixture.workspace, dirname(artifactProjectionFixturePath(cad.sourceRoot)), "🔤️fonts");
      mkdirSync(retainedDirectory);
      const { plan } = normalizationPlan(fixture);
      expect(plan.moves).toHaveLength(2);
      expect(plan.unresolved).toEqual([]);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.root, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "after-edits" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const committed = applyTaxonomyPlan(plan, { repoRoot: fixture.root, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect({ state: committed.state, error: JSON.parse(readFileSync(committed.journalPath, "utf8")).error ?? null }).toEqual({ state: "committed", error: null });
      expect(existsSync(join(fixture.workspace, artifactProjectionFixturePath(cad.sourceRoot)))).toBe(false);
      expect(lstatSync(retainedDirectory).isDirectory()).toBe(true);
      const second = normalizationPlan(fixture).plan;
      expect(second.moves).toEqual([]);
      expect(second.edits).toEqual([]);
      expect(second.unresolved).toEqual([]);
      const leaf = join(fixture.workspace, artifactProjectionFixturePath(member.destinationPath));
      const original = readFileSync(leaf, "utf8");
      writeFileSync(leaf, JSON.stringify({ id: "counterfeit", schema: "spatial.unknown", version: "1.0.0" }));
      expect(normalizationPlan(fixture).plan.unresolved.some(({ code }) => code === "projection-authority-invalid")).toBe(true);
      writeFileSync(leaf, original);
      const profilePath = artifactProjectionFixturePath(cad.destinationRoot).replace("🪆️1-any", "🪆️2-any");
      normalizationWriteFiles(fixture.workspace, { [`${profilePath}/${model}/🔣️.json`]: readFileSync(join(fixture.workspace, artifactProjectionFixturePath(manifest.destinationPath)), "utf8") });
      expect(normalizationPlan(fixture).plan.unresolved.some(({ code }) => code === "projection-destination-unregistered")).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);

  test("all physical mutation catalogs close the strict source-to-canonical registry", () => {
    const workspace = getWorkspaceRoot();
    const paths = ownedFilePaths(join(workspace, "✏️s", "🔌️plugins"))
      .filter((path) => path.endsWith("/🧪️oracle/🔣️.json"))
      .map((path) => `✏️s/🔌️plugins/${path}`);
    expect(paths).toEqual(nativeMutationCatalogPaths(workspace));
    const registrations: SemanticProjectionCatalogRegistration[] = [];
    for (const path of paths) {
      const parsed = JSON.parse(readFileSync(join(workspace, path), "utf8")) as { mutationCatalogs?: readonly { id: string; vectors: SemanticProjectionCatalogRegistration["vectors"] }[] };
      const ownerPath = path.slice(0, -"/🧪️oracle/🔣️.json".length);
      for (const catalog of parsed.mutationCatalogs ?? []) registrations.push({ ownerPath, catalogId: catalog.id, vectors: catalog.vectors });
    }
    const vectors = registrations.flatMap((catalog) => catalog.vectors);
    const changed = vectors.filter((vector) => vector.sourceMutationDirectoryName !== vector.mutationDirectoryName);
    const sourceTuples = registrations.flatMap((catalog) => catalog.vectors.flatMap((vector) => vector.scenarios.map((scenario) => `${catalog.catalogId}\0${vector.mutationId}\0${vector.sourceMutationDirectoryName}\0${scenario.id}`)));
    const canonicalTuples = registrations.flatMap((catalog) => catalog.vectors.flatMap((vector) => vector.scenarios.map((scenario) => `${catalog.catalogId}\0${vector.mutationId}\0${vector.mutationDirectoryName}\0${scenario.id}`)));
    const taxonomy = loadTaxonomy();
    const reserve = taxonomy.semanticDescendantContracts[taxonomy.mutationCatalogProjection.descendantContractId].pathBudgetReserve.bytes;
    const projectedBytes = registrations.flatMap((catalog) => catalog.vectors.flatMap((vector) => vector.scenarios.map((scenario) => new TextEncoder().encode(`${catalog.ownerPath.replace(/\/🏅️standards\/.*$/u, "")}/🧪️tests/🪆️${catalog.ownerPath.match(/\/🏅️standards\/🔖️([^/]+)\/🪆️subsets\/✳️([^/]+)$/u)![1]}-${catalog.ownerPath.match(/\/🏅️standards\/🔖️([^/]+)\/🪆️subsets\/✳️([^/]+)$/u)![2]}/${vector.mutationDirectoryName}/${scenario.directoryName}`).length + reserve)));
    expect(registrations).toHaveLength(MUTATION_PROJECTION_GOLDEN.registryCounts.catalogs);
    expect(vectors).toHaveLength(MUTATION_PROJECTION_GOLDEN.registryCounts.vectors);
    expect(vectors.flatMap((vector) => vector.scenarios)).toHaveLength(MUTATION_PROJECTION_GOLDEN.registryCounts.scenarios);
    expect(changed).toHaveLength(MUTATION_PROJECTION_GOLDEN.registryCounts.changedMutationRows);
    expect(new Set(changed.map((vector) => vector.sourceMutationDirectoryName)).size).toBe(MUTATION_PROJECTION_GOLDEN.registryCounts.changedMutationSources);
    expect(new Set(sourceTuples).size).toBe(sourceTuples.length);
    expect(new Set(canonicalTuples).size).toBe(canonicalTuples.length);
    expect(Math.max(...projectedBytes)).toBeLessThanOrEqual(taxonomy.collisionPolicy.maxPathBytes);
    expect(semanticProjectionCatalogProblems(registrations, taxonomy)).toEqual([]);
  }, 15_000);

  test("the strict catalog helper rejects missing sources, unknown canonical members, duplicate bundles, and excess path bytes", () => {
    const taxonomy = loadTaxonomy();
    const ownerPath = "✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️probe/🏅️standards/🔖️1/🪆️subsets/✳️any";
    const vector = { mutationId: "change-annex", sourceMutationDirectoryName: "change-annex", mutationDirectoryName: "🏷️change-annex", scenarios: [{ id: "switches-to-national-annex-a", directoryName: "🧪️switches-to-national-annex-a" }] };
    const problems = (vectors: SemanticProjectionCatalogRegistration["vectors"]) => semanticProjectionCatalogProblems([{ ownerPath, catalogId: "probe", vectors }], taxonomy);
    expect(problems([{ mutationId: vector.mutationId, mutationDirectoryName: vector.mutationDirectoryName, scenarios: vector.scenarios } as SemanticProjectionCatalogRegistration["vectors"][number]]).some((problem) => problem.includes("exactly mutationId, sourceMutationDirectoryName"))).toBe(true);
    expect(problems([{ ...vector, mutationDirectoryName: "🫥️change-annex" }]).some((problem) => problem.includes("no exact canonical schema membership"))).toBe(true);
    expect(problems([vector, vector]).some((problem) => problem.includes("duplicates a source bundle tuple"))).toBe(true);
    const id = `switches-${"very-long-".repeat(30)}annex`;
    expect(problems([{ ...vector, scenarios: [{ id, directoryName: `🧪️${id}` }] }]).some((problem) => problem.includes("exceeds maxPathBytes"))).toBe(true);
  });

  test("the language-agnostic mutation projection golden agrees with owned filesystem discovery", () => {
    const root = mkdtempSync(join(tmpdir(), "mutation-projection-golden-"));
    try {
      expect(MUTATION_PROJECTION_GOLDEN.sourceGlob).toBe(`🏅️standards/${MUTATION_PROJECTION_GOLDEN.standardDirectoryName}/🪆️subsets/${MUTATION_PROJECTION_GOLDEN.subsetDirectoryName}/🧬️schema/🧬️mutations/*/🧪️tests/*/🦀️component.rs`);
      for (const entry of MUTATION_PROJECTION_GOLDEN.cases) {
        const source = join(root, mutationGoldenSource(entry, "🦀️component.rs"));
        mkdirSync(resolve(source, ".."), { recursive: true });
        writeFileSync(source, "fixture\n");
      }
      const platformBoundary = ownedFilePaths(root).filter((path) => /^🏅️standards\/[^/]+\/🪆️subsets\/[^/]+\/🧬️schema\/🧬️mutations\/[^/]+\/🧪️tests\/[^/]+\/🦀️component\.rs$/u.test(path));
      const languageAgnostic = MUTATION_PROJECTION_GOLDEN.cases.map((entry) => mutationGoldenSource(entry, "🦀️component.rs")).sort(ownedPathByteSort);
      expect(platformBoundary).toEqual(languageAgnostic);
      expect(MUTATION_PROJECTION_GOLDEN.cases.map((entry) => mutationGoldenDestination(entry, "🦀️.rs"))).toEqual([
        "🧪️tests/🪆️1-any/🏷️change-annex/🧪️switches-to-national-annex-a/🦀️.rs",
        "🧪️tests/🪆️1-any/🌾️change-humidification-required-kg-h/🧪️required-humidification-becomes-3-point-5-kg-per-hour/🦀️.rs",
        "🧪️tests/🪆️1-any/🍀️change-humidification-provided-kg-h/🧪️provided-humidification-becomes-1-point-25-kg-per-hour/🦀️.rs",
        "🧪️tests/🪆️1-any/🌴️change-infiltration-allowance-m3-h/🧪️raises-infiltration-allowance-to-52-point-5-m3-per-hour/🦀️.rs",
        "🧪️tests/🪆️1-any/🔗️🎬️bind-default-scene/🧪️binds-first-scene-as-default/🦀️.rs",
        "🧪️tests/🪆️1-any/➖️delete-generation/🧪️removes-generation-2-and-selects-generation-1/🦀️.rs",
        "🧪️tests/🪆️1-any/✏️📦️change-asset-descriptive-metadata/🧪️restamps-generator-copyright-and-min-version/🦀️.rs",
        "🧪️tests/🪆️1-any/✏️📄️change-document-extension-data/🧪️attaches-punctual-lights-extension-to-document-root/🦀️.rs",
        "🧪️tests/🪆️1-any/✏️🔺️change-primitive-topology-mode/🧪️switches-primitive-from-triangles-to-triangle-strip/🦀️.rs",
        "🧪️tests/🪆️1-any/🚚️🧩️move-required-extension/🧪️moves-unlit-requirement-behind-transform-requirement/🦀️.rs",
        "🧪️tests/🪆️1-any/🔀️🧬️reorder-morph-target-attributes/🧪️orders-normal-before-position-in-morph-target/🦀️.rs",
      ]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("projects every registered golden bundle into artifact profile storage", () => {
    const files: Record<string, string> = {};
    for (const entry of MUTATION_PROJECTION_GOLDEN.cases) {
      for (const leaf of MUTATION_PROJECTION_GOLDEN.bundle) files[`🧪️artifact/${mutationGoldenSource(entry, leaf.source)}`] = "{}\n";
    }
    files[`🧪️artifact/🏅️standards/${MUTATION_PROJECTION_GOLDEN.standardDirectoryName}/🪆️subsets/${MUTATION_PROJECTION_GOLDEN.subsetDirectoryName}/🧪️oracle/🔣️.json`] = `${JSON.stringify({
      schemaVersion: 1,
      oracles: [],
      noOracleDecisions: [],
      mutationCatalogs: [{
        id: "golden-1-any",
        capability: "golden-mutate",
        standardDirectoryName: MUTATION_PROJECTION_GOLDEN.standardDirectoryName,
        subsetDirectoryName: MUTATION_PROJECTION_GOLDEN.subsetDirectoryName,
        kinds: ["runtime-only-operation"],
        vectors: MUTATION_PROJECTION_GOLDEN.cases.map((entry) => ({ mutationId: entry.mutationId, sourceMutationDirectoryName: entry.sourceMutationDirectoryName, mutationDirectoryName: entry.mutationDirectoryName, scenarios: [{ id: entry.scenarioId, directoryName: `🧪️${entry.scenarioId}` }] })),
      }],
    }, null, 2)}\n`;
    const fixture = normalizationFixture("mutation-projection", files);
    try {
      const { plan } = normalizationPlan(fixture);
      for (const entry of MUTATION_PROJECTION_GOLDEN.cases) {
        for (const leaf of MUTATION_PROJECTION_GOLDEN.bundle) {
          const move = normalizationMove(plan, mutationGoldenSource(entry, leaf.source));
          expect(move.destinationPath).toBe(`${fixture.scope}/🧪️artifact/${mutationGoldenDestination(entry, leaf.destination)}`);
          expect(move.rationaleRule).toBe("artifact-mutation-test-projection-v1");
        }
      }
      expect(plan.unresolved.filter((violation) => violation.code.includes("projection"))).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("blocks a reintroduced mutation source token after an empty converged plan", () => {
    const entry = MUTATION_PROJECTION_GOLDEN.cases[0]!;
    const files: Record<string, string> = {};
    for (const leaf of MUTATION_PROJECTION_GOLDEN.bundle) files[`🧪️artifact/${mutationGoldenSource(entry, leaf.source)}`] = "{}\n";
    files[`🧪️artifact/🏅️standards/${MUTATION_PROJECTION_GOLDEN.standardDirectoryName}/🪆️subsets/${MUTATION_PROJECTION_GOLDEN.subsetDirectoryName}/🧪️oracle/🔣️.json`] = `${JSON.stringify({
      schemaVersion: 1,
      oracles: [],
      noOracleDecisions: [],
      mutationCatalogs: [{ id: "stale-1-any", capability: "stale-mutate", standardDirectoryName: MUTATION_PROJECTION_GOLDEN.standardDirectoryName, subsetDirectoryName: MUTATION_PROJECTION_GOLDEN.subsetDirectoryName, kinds: [], vectors: [{ mutationId: entry.mutationId, sourceMutationDirectoryName: entry.sourceMutationDirectoryName, mutationDirectoryName: entry.mutationDirectoryName, scenarios: [{ id: entry.scenarioId, directoryName: `🧪️${entry.scenarioId}` }] }] }],
    }, null, 2)}\n`;
    const fixture = normalizationFixture("mutation-stale", files);
    try {
      const first = normalizationPlan(fixture).plan;
      expect(first.unresolved.filter((violation) => violation.severity === "error")).toEqual([]);
      expect(applyTaxonomyPlan(first, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: first.baselineCommit, expectedPlanDigest: first.planDigest }).state).toBe("committed");
      const clean = normalizationPlan(fixture).plan;
      expect(clean.moves.filter((move) => move.rationaleRule === "artifact-mutation-test-projection-v1")).toEqual([]);
      expect(clean.unresolved.filter((violation) => violation.code === "projection-old-token-stale")).toEqual([]);
      const destination = join(fixture.workspace, "🧪️artifact", mutationGoldenDestination(entry, "🦀️.rs"));
      writeFileSync(destination, `${readFileSync(destination, "utf8")}\n// ${mutationGoldenSource(entry, "")}\n`);
      const stale = normalizationPlan(fixture).plan;
      expect(stale.moves.filter((move) => move.rationaleRule === "artifact-mutation-test-projection-v1")).toEqual([]);
      expect(stale.unresolved.filter((violation) => violation.code === "projection-old-token-stale")).toHaveLength(1);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("plans the exact CAD and Draw authority mappings with structured cross-profile references", () => {
    const fixture = artifactProjectionNormalizationFixture("plan", true);
    let passed = false;
    try {
      const { plan } = normalizationPlan(fixture);
      const cad = projectionGolden("artifact-example-model-catalog-v1");
      const draw = projectionGolden("artifact-editor-command-bundle-v1");
      expect(artifactProjectionTail(`${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}`)).toStartWith("🗿️artifacts/");
      const cadMoves = plan.moves.filter((move) => move.rationaleRule === cad.rationaleRule);
      const drawMoves = plan.moves.filter((move) => move.rationaleRule === draw.rationaleRule);
      expect(cadMoves).toHaveLength(cad.sourceFileCount);
      expect(drawMoves).toHaveLength(draw.sourceFileCount);
      expect(cadMoves.map(({ sourcePath, destinationPath }) => ({ sourcePath: sourcePath.slice(fixture.scope.length + 1), destinationPath: destinationPath.slice(fixture.scope.length + 1) }))).toEqual(cad.mappings.map(({ sourcePath, destinationPath }) => ({ sourcePath: artifactProjectionFixturePath(sourcePath), destinationPath: artifactProjectionFixturePath(destinationPath) })));
      expect(drawMoves.map(({ sourcePath, destinationPath }) => ({ sourcePath: sourcePath.slice(fixture.scope.length + 1), destinationPath: destinationPath.slice(fixture.scope.length + 1) }))).toEqual(draw.mappings.map(({ sourcePath, destinationPath }) => ({ sourcePath: artifactProjectionFixturePath(sourcePath), destinationPath: artifactProjectionFixturePath(destinationPath) })));
      expect(plan.unresolved.filter((entry) => /projection|collision/u.test(entry.code))).toEqual([]);
      const referenceEdits = plan.edits.filter((edit) => edit.path.includes(`/${artifactProjectionFixturePath(cad.destinationRoot)}/`) && (edit.oldValue === `${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}` || edit.oldValue === `${fixture.scope}/${artifactProjectionFixturePath(draw.mappings[0]!.sourcePath)}`));
      expect(referenceEdits).toHaveLength(2);
      expect(referenceEdits.map((edit) => edit.newValue).sort(projectionByteSort)).toEqual([
        `${fixture.scope}/${artifactProjectionFixturePath(cad.destinationRoot)}`,
        `${fixture.scope}/${artifactProjectionFixturePath(draw.mappings[0]!.destinationPath)}`,
      ].sort(projectionByteSort));
      const locations = plan.edits.map((edit) => edit.structuredLocation);
      expect(locations.filter((location) => location.startsWith("artifact-catalog-glob:"))).toHaveLength(10);
      expect(locations.filter((location) => location.startsWith("artifact-catalog-comment:"))).toHaveLength(2);
      expect(locations.filter((location) => location.startsWith("artifact-catalog-marker:"))).toHaveLength(1);
      expect(locations.filter((location) => location.includes("/workspace-glob@"))).toHaveLength(2);
      expect(locations.filter((location) => location.startsWith("path-collection:"))).toHaveLength(1);
      expect(plan.edits.some((edit) => edit.structuredLocation.startsWith("artifact-catalog-root-join:") && edit.oldValue.endsWith("🖼️assets/🏗️modelDefinitions") && edit.newValue.endsWith("📚️examples/🪆️1-any/🏗️models"))).toBe(true);
      const removedSelectors = plan.edits.filter((edit) => edit.structuredLocation.startsWith("artifact-catalog-glob:") && edit.newValue === "");
      expect(removedSelectors).toHaveLength(2);
      expect(removedSelectors.reduce((count, edit) => count + (edit.oldValue.match(/modelDefinitions/g)?.length ?? 0), 0)).toBe(3);
      const cadExact = plan.edits.filter((edit) => edit.adapter === "rust" && edit.structuredLocation.startsWith("rust-string-path:") && /modelDefinitions\/.+\.json$/u.test(edit.oldValue));
      expect(cadExact).toHaveLength(61);
      for (const edit of cadExact) {
        const matches = cad.mappings.filter((mapping) => edit.oldValue.endsWith(mapping.sourcePath.slice(cad.sourceRoot.length + 1)) && edit.newValue.endsWith(mapping.destinationPath.slice(cad.destinationRoot.length + 1)));
        expect(matches).toHaveLength(1);
      }
      const drawExact = plan.edits.filter((edit) => (edit.oldValue.includes(`${fixture.scope}/${artifactProjectionFixturePath(draw.sourceRoot)}`) || edit.path.startsWith(`${fixture.scope}/📦️packages/🦀️rust/`) || edit.path.includes("/🧪️tests/🧪️reference/")) && edit.oldValue !== "📦️glue.rs" && !edit.oldValue.includes("workspaceRoot") && !edit.structuredLocation.startsWith("path-collection:") && !edit.path.includes(`/${artifactProjectionFixturePath(cad.destinationRoot)}/`));
      expect(drawExact).toHaveLength(20);
      expect(drawExact.some((edit) => edit.path.endsWith("📦️packages/🦀️rust/Cargo.toml") && edit.newValue === "../../🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/📦️packages/🦀️rust")).toBe(true);
      expect(drawExact.some((edit) => edit.path.endsWith("🧪️tests/🧪️reference/🦀️.rs") && edit.newValue === "../../🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs")).toBe(true);
      const configurableEntries = plan.edits.filter((edit) => edit.oldValue === "📦️glue.rs" && edit.newValue === "📚️library/🦀️.rs");
      expect(configurableEntries).toHaveLength(2);
      expect(configurableEntries.every((edit) => edit.adapter === "toml" && edit.structuredLocation.startsWith("lib.path:"))).toBe(true);
      expect(configurableEntries.map((edit) => ({ path: edit.path.slice(fixture.scope.length + 1), adapter: edit.adapter, structuredLocation: edit.structuredLocation.slice(0, "lib.path".length), oldValue: edit.oldValue, newValue: edit.newValue, preimageHash: edit.preimage.contentHash }))).toEqual((draw.referenceEdits ?? []).map((edit) => ({ ...edit, path: artifactProjectionFixturePath(edit.path), preimageHash: drawSourceScenarioPreimage(edit.path) })));
      expect(plan.unresolved.filter((entry) => /projection|reference/u.test(entry.code))).toEqual([]);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 60_000);

  test("rejects unowned artifact prose, unmatched selectors, escaped placeholders, and counterfeit owners", () => {
    const fixture = artifactProjectionNormalizationFixture("negative", true);
    let passed = false;
    try {
      const cad = projectionGolden("artifact-example-model-catalog-v1");
      const draw = projectionGolden("artifact-editor-command-bundle-v1");
      const unowned = join(fixture.workspace, "✏️s", "🔨️modules", "🧪️unowned", "🟦️.ts");
      mkdirSync(resolve(unowned, ".."), { recursive: true });
      const copiedSelector = `${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}/**/🗂️typologies/**/🔣️typology.json`;
      writeFileSync(unowned, `export const unrelated = "🖼️assets/🏗️modelDefinitions/";\nexport const counterfeit = import.meta.glob(${JSON.stringify(copiedSelector)});\n`);
      const schemaPath = join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL);
      const schema = JSON.parse(readFileSync(schemaPath, "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { sourcePathPattern: string }> };
      schema.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"]!.sourcePathPattern = "^.*\\.ts$";
      writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
      const runtime = join(fixture.workspace, artifactProjectionFixturePath("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts"));
      writeFileSync(runtime, readFileSync(runtime, "utf8").replace("/🗂️typologies/**/🔣️typology.json", "/🧪️unregistered/**/*.json"));
      const projectMapping = draw.mappings.find(({ sourcePath }) => sourcePath.endsWith("📋️project.json"))!;
      const project = join(fixture.workspace, artifactProjectionFixturePath(projectMapping.sourcePath));
      writeFileSync(project, readFileSync(project, "utf8").replace("{workspaceRoot}", "\\u007bworkspaceRoot\\u007d"));
      const counterfeit = join(fixture.workspace, "🧪️counterfeit", cad.sourceRoot.slice(cad.sourceRoot.lastIndexOf("/📐️cad/") + 1), "🧪️model", "🔣️modelDefinition.json");
      mkdirSync(resolve(counterfeit, ".."), { recursive: true });
      writeFileSync(counterfeit, JSON.stringify({ id: "counterfeit", schema: "model-definition", version: "1.0.0" }));
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("outside an authorized owner"))).toBe(true);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("selector or reference file"))).toBe(true);
      expect(plan.edits.some((entry) => entry.path.endsWith("🧪️unowned/🟦️.ts"))).toBe(false);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("Nonempty artifact selector"))).toBe(true);
      expect(plan.unresolved.some((entry) => entry.code === "reference-syntax-unsupported" && entry.message.includes("Escaped workspace projection glob"))).toBe(true);
      expect(plan.moves.some((move) => move.sourcePath.includes("🧪️counterfeit") && /artifact-(?:example|editor)/u.test(move.rationaleRule))).toBe(false);
      expect(plan.unresolved.some((entry) => entry.path.includes("🧪️counterfeit") && entry.code === "projection-authority-invalid")).toBe(false);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 60_000);

  test("rolls back and atomically applies CAD and Draw projections to an empty second plan", () => {
    const fixture = artifactProjectionNormalizationFixture("apply", true);
    let passed = false;
    try {
      const sourceRoots = ARTIFACT_PROJECTION_GOLDEN.projections.map(({ sourceRoot }) => artifactProjectionFixturePath(sourceRoot));
      const retainedRelative = `${dirname(sourceRoots[0]!)}/🔤️fonts`;
      const retainedDirectory = join(fixture.workspace, retainedRelative);
      mkdirSync(retainedDirectory);
      if (process.platform !== "win32") for (const path of [...sourceRoots.map((path) => join(fixture.workspace, path)), retainedDirectory]) chmodSync(path, 0o750);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const plan = normalizationPlan(fixture).plan;
      expect(plan.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
      const projectedNodes = new Set(Object.keys(before));
      const candidateDirectories = new Set<string>();
      for (const move of plan.moves) {
        const source = move.sourcePath.slice(fixture.scope.length + 1), destination = move.destinationPath.slice(fixture.scope.length + 1);
        projectedNodes.delete(source);
        projectedNodes.add(destination);
        for (let path = dirname(source); path !== "."; path = dirname(path)) candidateDirectories.add(path);
        for (let path = dirname(destination); path !== "."; path = dirname(path)) projectedNodes.add(path);
      }
      const expectedPruned: string[] = [];
      for (const path of [...candidateDirectories].sort((left, right) => right.split("/").length - left.split("/").length)) if (projectedNodes.has(path) && ![...projectedNodes].some((candidate) => candidate.startsWith(`${path}/`))) {
        projectedNodes.delete(path);
        expectedPruned.push(path);
      }
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "after-edits" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const cancelFile = join(fixture.ticketDir, "🛑️cancel");
      writeFileSync(cancelFile, "cancel\n");
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, cancelFile }));
      rmSync(cancelFile);
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const applied = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect({ state: applied.state, error: JSON.parse(readFileSync(applied.journalPath, "utf8")).error ?? null }).toEqual({ state: "committed", error: null });
      expect(JSON.parse(readFileSync(applied.journalPath, "utf8")).attemptOrdinal).toBe("000002");
      for (const path of sourceRoots) expect(existsSync(join(fixture.workspace, path))).toBe(false);
      const after = normalizationWorkspaceSnapshot(fixture.workspace);
      expect(Object.entries(before).filter(([path, entry]) => entry.startsWith("directory|") && !after[path]).map(([path]) => path).sort(projectionByteSort)).toEqual(expectedPruned.sort(projectionByteSort));
      expect(fastGlob.sync("**/*", { cwd: fixture.workspace, onlyDirectories: true, dot: true, followSymbolicLinks: false }).sort(projectionByteSort)).toEqual(Object.entries(after).filter(([path, entry]) => path !== "." && entry.startsWith("directory|")).map(([path]) => path).sort(projectionByteSort));
      expect(after[retainedRelative]).toBe(before[retainedRelative]);
      for (const [path, entry] of Object.entries(before).filter(([, entry]) => entry.startsWith("directory|"))) if (after[path]) expect(after[path]).toBe(entry);
      expect(lstatSync(fixture.ticketDir).isDirectory()).toBe(true);
      expect(applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, resumeJournal: applied.journalPath }).state).toBe("committed");
      for (const edit of plan.edits.filter((candidate) => /artifact-catalog|workspace-glob|path-collection/u.test(candidate.structuredLocation))) expect(readFileSync(join(fixture.repoRoot, edit.path), "utf8")).not.toContain(edit.oldValue);
      const second = normalizationPlan(fixture).plan;
      expect(second.moves.filter((move) => move.rationaleRule === "artifact-example-model-catalog-projection-v1" || move.rationaleRule === "artifact-editor-command-projection-v1")).toEqual([]);
      expect(second.edits.filter((edit) => /artifact-catalog|workspace-glob|path-collection/u.test(edit.structuredLocation))).toEqual([]);
      expect(second.unresolved.filter((entry) => /projection/u.test(entry.code))).toEqual([]);
      const taxonomy = JSON.parse(readFileSync(join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL), "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { projectionContractId: string; staleMarkers: string[] }> };
      const marker = (projectionContractId: string): string => Object.values(taxonomy.semanticPathProjectionReferenceConsumerContracts).find((contract) => contract.projectionContractId === projectionContractId)!.staleMarkers[0]!;
      const cadConsumer = plan.edits.find((edit) => edit.structuredLocation.startsWith("artifact-catalog-marker:"))!.path;
      writeFileSync(join(fixture.repoRoot, cadConsumer), `${readFileSync(join(fixture.repoRoot, cadConsumer), "utf8")}\nexport const staleCadRoot = ${JSON.stringify(marker("artifact-example-model-catalog-v1"))};\n`);
      const drawPackageConsumers = plan.edits.filter((edit) => edit.oldValue === "📦️glue.rs" && edit.newValue === "📚️library/🦀️.rs").map((edit) => edit.path);
      expect(drawPackageConsumers).toHaveLength(2);
      for (const path of drawPackageConsumers) writeFileSync(join(fixture.repoRoot, path), `${readFileSync(join(fixture.repoRoot, path), "utf8")}\n# ${marker("artifact-editor-command-bundle-v1")}\n`);
      const stale = normalizationPlan(fixture).plan;
      expect(stale.moves.filter((move) => move.rationaleRule === "artifact-example-model-catalog-projection-v1" || move.rationaleRule === "artifact-editor-command-projection-v1")).toEqual([]);
      expect(stale.unresolved.filter((entry) => entry.code === "projection-old-token-stale")).toHaveLength(3);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 120_000);

  test("rejects ignored and unplanned residual children in a projected source owner without following links", () => {
    const fixture = artifactProjectionNormalizationFixture("source-residue", true);
    let passed = false;
    try {
      const sourceRoot = join(fixture.workspace, artifactProjectionFixturePath(projectionGolden("artifact-example-model-catalog-v1").sourceRoot));
      const residue = join(sourceRoot, "🧪️unexpected");
      const linkTarget = join(fixture.ticketDir, "🧪️link-target");
      mkdirSync(linkTarget);
      writeFileSync(join(linkTarget, "📝️.md"), "not followed\n");
      writeFileSync(join(fixture.root, ".git", "info", "exclude"), `/${relative(fixture.root, residue).replaceAll("\\", "/")}/\n`);
      const { plan } = normalizationPlan(fixture);
      expect(plan.moves.length).toBeGreaterThan(0);
      expect(plan.unresolved.filter(({ severity }) => severity === "error")).toEqual([]);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const targetBefore = normalizationWorkspaceSnapshot(linkTarget);
      let injected = false;
      const applied = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, progress: (event) => {
        if (event.phase !== "moves" || event.current !== event.total || injected) return;
        injected = true;
        mkdirSync(join(residue, "🧪️empty"), { recursive: true });
        writeFileSync(join(residue, "📝️.md"), "ignored residual source\n");
        symlinkSync(process.platform === "win32" ? linkTarget : relative(residue, linkTarget), join(residue, "🧪️link"), process.platform === "win32" ? "junction" : "dir");
      } });
      expect(injected).toBe(true);
      normalizationGit(fixture.root, ["check-ignore", "--quiet", "--", relative(fixture.root, join(residue, "📝️.md"))]);
      expect({ state: applied.state, error: JSON.parse(readFileSync(applied.journalPath, "utf8")).error }).toEqual({ state: "rolled-back", error: expect.stringContaining("projection-source-directory-stale") });
      expect(readFileSync(join(residue, "📝️.md"), "utf8")).toBe("ignored residual source\n");
      expect(lstatSync(join(residue, "🧪️empty")).isDirectory()).toBe(true);
      expect(lstatSync(join(residue, "🧪️link")).isSymbolicLink()).toBe(true);
      expect(normalizationWorkspaceSnapshot(linkTarget)).toEqual(targetBefore);
      const after = normalizationWorkspaceSnapshot(fixture.workspace);
      for (const [path, entry] of Object.entries(before)) expect(after[path]).toBe(entry);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 120_000);

  test("rederives repeatable CAD and Draw source-tree authority before apply", () => {
    const fixture = artifactProjectionNormalizationFixture("source-tree-repeatability", true);
    let passed = false;
    try {
      const first = normalizationPlan(fixture);
      const second = normalizationPlan(fixture);
      const sourceRows = (inventory: TaxonomyInventory) => inventory.entries.map(({ sourcePath, nodeKind, contentHash, mode, size, symlinkTarget }) => ({ sourcePath, nodeKind, contentHash, mode, size, symlinkTarget }));
      expect(sourceRows(second.inventory)).toEqual(sourceRows(first.inventory));
      expect(second.plan.sourceTreeDigest).toBe(first.plan.sourceTreeDigest);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 120_000);

  test("rejects a changed explicit source-admission ticket before journal mutation", () => {
    const fixture = normalizationFixture("source-authority-ticket", { "🧪️subject/🦀️component.rs": "pub const VALUE: i32 = 1;\n" }, ({ repoRoot, workspace }) => writeFileSync(join(repoRoot, ".gitignore"), `${relative(repoRoot, workspace).replaceAll("\\", "/")}/\n`));
    try {
      const { inventory, plan } = normalizationPlan(fixture);
      expect(plan.unresolved.filter(({ severity }) => severity === "error")).toEqual([]);
      const transactionTicket = join(fixture.ticketDir, "🧪️transaction-evidence");
      mkdirSync(transactionTicket, { recursive: true });
      const changedAdmission = inventoryTaxonomy({ ...fixture.options, ticketDir: transactionTicket });
      expect(inventory.entries.map(({ sourcePath }) => sourcePath)).toEqual(["🧪️tests", fixture.scope, `${fixture.scope}/🧪️subject`, `${fixture.scope}/🧪️subject/🦀️component.rs`]);
      expect(changedAdmission.entries).toEqual([]);
      expect(changedAdmission.sourceTreeDigest).not.toBe(plan.sourceTreeDigest);
      const before = normalizationWorkspaceSnapshot(fixture.ticketDir);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: transactionTicket, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("Plan source-tree digest cannot be rederived exactly");
      expect(normalizationWorkspaceSnapshot(fixture.ticketDir)).toEqual(before);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("rejects changed and newly admitted in-scope sources before journal mutation", () => {
    const stableRelative = "🧪️stable/📝️.md";
    const fixture = normalizationFixture("source-authority-drift", { "🧪️subject/🦀️component.rs": "pub const VALUE: i32 = 1;\n", [stableRelative]: "original\n" }, ({ repoRoot, workspace }) => writeFileSync(join(repoRoot, ".gitignore"), `${relative(repoRoot, workspace).replaceAll("\\", "/")}/\n`));
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved.filter(({ severity }) => severity === "error")).toEqual([]);
      expect(plan.moves.some(({ sourcePath }) => sourcePath === `${fixture.scope}/${stableRelative}`)).toBe(false);
      writeFileSync(join(fixture.workspace, stableRelative), "changed physical source\n");
      const changedBefore = normalizationWorkspaceSnapshot(fixture.ticketDir);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("Plan source-tree digest cannot be rederived exactly");
      expect(normalizationWorkspaceSnapshot(fixture.ticketDir)).toEqual(changedBefore);
      writeFileSync(join(fixture.workspace, stableRelative), "original\n");
      normalizationWriteFiles(fixture.workspace, { "🧪️in-scope-extra/📝️.md": "new ignored but explicitly admitted source\n" });
      const extraBefore = normalizationWorkspaceSnapshot(fixture.ticketDir);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("Plan source-tree digest cannot be rederived exactly");
      expect(normalizationWorkspaceSnapshot(fixture.ticketDir)).toEqual(extraBefore);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("canonical JSON recursively sorts keys and contract-identified arrays", () => {
    const left = { z: 1, rows: [{ id: "b", value: 2 }, { id: "a", value: 1 }], nested: { z: false, a: true } };
    const right = { nested: { a: true, z: false }, rows: [{ value: 1, id: "a" }, { value: 2, id: "b" }], z: 1 };
    expect(canonicalJson(left)).toBe(canonicalJson(right));
    expect(canonicalJson(left)).toBe(canonicalJson(JSON.parse(canonicalJson(left))));
  });

  test("normalization rejects malformed projection consumers and configurable descendants at its own schema boundary", () => {
    const fixture = normalizationFixture("projection-schema-boundary", { "🟦️component.ts": "export const value = 1;\n" });
    try {
      const schemaPath = join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL);
      const original = JSON.parse(readFileSync(schemaPath, "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, Record<string, unknown>>; semanticDescendantContracts: Record<string, { requiredNodes: Record<string, unknown>[] }> };
      const consumer = structuredClone(original);
      consumer.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"]!.consumerIdentity = "counterfeit";
      writeFileSync(schemaPath, `${JSON.stringify(consumer, null, 2)}\n`);
      expect(() => inventoryTaxonomy(fixture.options)).toThrow("discovery contract validation failed");
      const descendant = structuredClone(original);
      const configurable = descendant.semanticDescendantContracts["draw-editor-command-bundle-v1"]!.requiredNodes.find((node) => "configurableEntry" in node)!;
      configurable.compatibilityAlias = true;
      writeFileSync(schemaPath, `${JSON.stringify(descendant, null, 2)}\n`);
      expect(() => inventoryTaxonomy(fixture.options)).toThrow("discovery contract validation failed");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("opaque-tree digest hashes symlink identity without reading its target", () => {
    const fixture = normalizationFixture(
      "opaque",
      { "🟦️component.ts": "export const value = 1;\n" },
      ({ repoRoot }) => {
        mkdirSync(join(repoRoot, "outside"), { recursive: true });
        writeFileSync(join(repoRoot, "outside", "target.txt"), "first\n");
        symlinkSync("../outside/target.txt", join(repoRoot, "compose", "target-link"), "file");
      },
    );
    try {
      const before = opaqueTreeDigest(fixture.repoRoot, "compose");
      writeFileSync(join(fixture.repoRoot, "outside", "target.txt"), "second and deliberately different\n");
      const after = opaqueTreeDigest(fixture.repoRoot, "compose");
      expect(after).toEqual(before);
      expect(before.algorithm).toBe("sha256-merkle-v1");
      expect(before.relativeRoot).toBe("compose");
      expect(before.files).toBe(1);
      expect(before.symlinks).toBe(1);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("an absent opaque prefix remains lexically excluded without requiring an impossible digest", () => {
    const fixture = normalizationFixture("absent-opaque", { "🟦️component.ts": "export const value = 1;\n" });
    try {
      rmSync(join(fixture.repoRoot, "compose"), { recursive: true, force: true });
      const inventory = inventoryTaxonomy(fixture.options);
      expect(inventory.pathExclusions).toContain("compose");
      expect(inventory.entries.some((entry) => entry.sourcePath === "compose" || entry.sourcePath.startsWith("compose/"))).toBe(false);
      const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(plan.unresolved.some((violation) => violation.code === "opaque-digest-missing")).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("scoped Git pathspec rendering matches the language-neutral golden", () => {
    expect(SCOPED_INVENTORY_PATHSPEC_GOLDEN.schemaVersion).toBe(1);
    for (const row of SCOPED_INVENTORY_PATHSPEC_GOLDEN.cases) {
      expect(taxonomyScopedGitPathspec(row.inputScope, SCOPED_INVENTORY_PATHSPEC_GOLDEN.opaqueExclusions)).toEqual({
        normalizedScope: row.normalizedScope,
        conservativePrefix: row.conservativePrefix,
        positivePathspec: row.positivePathspec,
        exclusionPathspecs: row.exclusionPathspecs,
      });
    }
  });

  test("scoped inventory preserves literal Unicode scope, third-party census, and closed phases", () => {
    const literalRoot = "🧪️literal [x]*?!:";
    const decomposedRoot = "🧪️cafe\u0301";
    const fixture = normalizationFixture("scoped-pathspec", {
      ".gitignore": "ignored/\n",
      "🟦️component.ts": "export const tracked = true;\n",
      [`${literalRoot}/🟦️component.ts`]: "export const literal = true;\n",
      [`${decomposedRoot}/🟦️component.ts`]: "export const unicode = true;\n",
    });
    try {
      normalizationWriteFiles(fixture.workspace, {
        "🧪️untracked/🟦️component.ts": "export const untracked = true;\n",
        "ignored/🟦️component.ts": "export const ignored = true;\n",
      });
      normalizationWriteFiles(fixture.repoRoot, { "🧪️unrelated/🟦️.ts": "export const unrelated = true;\n" });
      const events: TaxonomyProgress[] = [];
      const scopedOptions: TaxonomyInventoryOptions = fixture.options;
      const inventory = inventoryTaxonomy({ ...scopedOptions, progress: (event) => events.push(event) });
      const files = inventory.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath);
      expect(files).toContain(`${fixture.scope}/🧪️untracked/🟦️component.ts`);
      expect(files.some((path) => path.startsWith("🧪️unrelated/"))).toBe(false);
      const thirdParty = fastGlob.sync("**/*", { cwd: fixture.workspace, dot: true, followSymbolicLinks: false, onlyFiles: true })
        .map((path) => `${fixture.scope}/${path.replaceAll("\\", "/")}`)
        .sort(ownedPathByteSort);
      expect([...files].sort(ownedPathByteSort)).toEqual(thirdParty);
      const phases = events.map((event) => event.phase).filter((phase, index, rows) => index === 0 || phase !== rows[index - 1]);
      expect(phases).toEqual(["setup", "tracked-enumeration", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission", "directories", "files", "references", "complete"]);
      for (const phase of ["tracked-enumeration", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission"]) expect(events.filter((event) => event.phase === phase).map((event) => [event.current, event.total])).toEqual([[0, 1], [1, 1]]);
      for (const phase of ["directories", "files", "references"]) expect(events.find((event) => event.phase === phase)?.current).toBe(0);
      const unicode = inventoryTaxonomy({ ...scopedOptions, scope: `${fixture.scope}/🧪️café` });
      expect(unicode.entries.some((entry) => entry.sourcePath === `${fixture.scope}/${decomposedRoot}/🟦️component.ts`)).toBe(true);
      const literal = inventoryTaxonomy({ ...scopedOptions, scope: `${fixture.scope}/${literalRoot}` });
      expect(literal.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath)).toEqual([`${fixture.scope}/${literalRoot}/🟦️component.ts`]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("an exact symlink leaf is inventoried without following target content", () => {
    const fixture = normalizationFixture("scoped-symlink-leaf", { "🧪️target/📝️.md": "first\n" }, ({ workspace }) => symlinkSync("🧪️target/📝️.md", join(workspace, "🧪️link"), "file"));
    try {
      const scope = `${fixture.scope}/🧪️link`;
      normalizationGit(fixture.repoRoot, ["add", "-f", "--", scope]);
      expect(lstatSync(join(fixture.workspace, "🧪️link")).isSymbolicLink()).toBe(true);
      const first = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const link = first.entries.find(({ sourcePath }) => sourcePath === scope);
      expect(link?.nodeKind).toBe("symlink");
      expect(link?.symlinkTarget).toBe("🧪️target/📝️.md");
      writeFileSync(join(fixture.workspace, "🧪️target", "📝️.md"), "second and deliberately different\n");
      const second = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      expect(second.entries.find(({ sourcePath }) => sourcePath === scope)?.contentHash).toBe(link?.contentHash);
      expect(second.entries.some(({ sourcePath }) => sourcePath.startsWith(`${scope}/`))).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("a scope below a symlink ancestor falls back to full Git enumeration and preserves the ancestor-only census", () => {
    const fixture = normalizationFixture("scoped-symlink-ancestor", { "🧪️seed/📝️.md": "seed\n" }, ({ repoRoot, workspace }) => {
      const target = join(repoRoot, "🧪️outside-target");
      normalizationWriteFiles(target, { "🧪️child/📝️.md": "must not be followed\n" });
      symlinkSync(process.platform === "win32" ? target : relative(workspace, target), join(workspace, "🧪️alias"), process.platform === "win32" ? "junction" : "dir");
    });
    try {
      const scope = `${fixture.scope}/🧪️alias/🧪️child`;
      normalizationGit(fixture.repoRoot, ["add", "-f", "--", `${fixture.scope}/🧪️alias`]);
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const full = inventoryTaxonomy({ repoRoot: fixture.repoRoot, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.filter(({ nodeKind }) => nodeKind !== "directory").map(({ sourcePath, nodeKind }) => ({ sourcePath, nodeKind }))).toEqual([{ sourcePath: `${fixture.scope}/🧪️alias`, nodeKind: "symlink" }]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("an exact mode-160000 ancestor falls back without admitting nested-repository content", () => {
    const fixture = normalizationFixture("scoped-gitlink-ancestor", { "🧪️seed/📝️.md": "seed\n" });
    const gitlinkRelative = `${fixture.scope}/🧪️gitlink`;
    const gitlink = join(fixture.workspace, "🧪️gitlink");
    try {
      mkdirSync(gitlink, { recursive: true });
      normalizationGit(gitlink, ["init", "--quiet"]);
      normalizationWriteFiles(gitlink, { "🧪️child/📝️.md": "nested sentinel\n" });
      normalizationGit(fixture.repoRoot, ["update-index", "--add", "--cacheinfo", `160000,${fixture.baselineCommit},${gitlinkRelative}`]);
      const scope = `${gitlinkRelative}/🧪️child`;
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === gitlinkRelative)).toBe(true);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath.startsWith(`${gitlinkRelative}/`))).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("standalone ignored-generator admission preserves scoped versus full census parity", () => {
    const fixture = normalizationFixture("scoped-generator-admission", { "🧪️generator/🟦️.ts": "export const input = true;\n" }, ({ repoRoot, workspace }) => {
      const owner = relative(repoRoot, join(workspace, "🧪️generator")).replaceAll("\\", "/");
      const outputRoot = `${owner}/🤖️generated`;
      const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
      const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as { generatorContracts: Record<string, unknown> };
      taxonomy.generatorContracts = Object.fromEntries(Object.entries({ ...taxonomy.generatorContracts, "fixture-scoped-generator": { ownership: "owned", ownerPath: owner, target: "@fixture/scoped:generate", previewTarget: "@fixture/scoped:preview-generated", checkTarget: "@fixture/scoped:check", inputPatterns: [`${owner}/🟦️.ts`], outputRoots: [{ path: outputRoot, inclusion: "ignored" }], reason: "Scoped inventory admission fixture" } }).sort(([left], [right]) => left.localeCompare(right)));
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
      writeFileSync(join(repoRoot, ".gitignore"), `${outputRoot}\n`);
      normalizationWriteFiles(repoRoot, { [`${outputRoot}/📝️.md`]: "owned ignored output\n" });
    });
    try {
      const scope = `${fixture.scope}/🧪️generator/🤖️generated`;
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const full = inventoryTaxonomy({ repoRoot: fixture.repoRoot, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("standalone explicit-ticket admission preserves scoped versus full census parity", () => {
    const fixture = normalizationFixture("scoped-ticket-admission", { ".gitignore": "🧪️explicit/\n", "🟦️.ts": "export const input = true;\n" }, ({ workspace }) => normalizationWriteFiles(workspace, { "🧪️explicit/📝️.md": "explicit ignored evidence\n" }));
    try {
      const scope = `${fixture.scope}/🧪️explicit`;
      const scoped = inventoryTaxonomy({ ...fixture.options, scope });
      const full = inventoryTaxonomy({ ...fixture.options, scope: undefined });
      const withoutTicketAuthority = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(true);
      expect(withoutTicketAuthority.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("inventory cancellation is observed from every nonterminal frozen phase boundary", () => {
    const fixture = normalizationFixture("scoped-phase-cancellation", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    const cancelFile = join(fixture.root, "🛑️cancel");
    try {
      for (const phase of ["setup", "tracked-enumeration", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission", "directories", "files", "references"]) {
        rmSync(cancelFile, { force: true });
        expect(() => inventoryTaxonomy({ ...fixture.options, cancelFile, progress: (event) => { if (event.phase === phase && event.current === 0) writeFileSync(cancelFile, `${phase}\n`); } })).toThrow("Taxonomy operation cancelled");
      }
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("unscoped inventory retains the frozen Git pathspec and canonical inventory bytes", () => {
    const fixture = normalizationFixture("unscoped-regression", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    try {
      const options: TaxonomyInventoryOptions = { repoRoot: fixture.repoRoot, workers: 1 };
      const first = inventoryTaxonomy(options);
      const second = inventoryTaxonomy(options);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(taxonomyScopedGitPathspec(undefined, SCOPED_INVENTORY_PATHSPEC_GOLDEN.opaqueExclusions)).toEqual({ normalizedScope: null, conservativePrefix: ".", positivePathspec: ".", exclusionPathspecs: [":(exclude,top,literal)compose", ":(exclude,top,literal)temp/compose"] });
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("reversed creation order preserves scoped canonical bytes and phase sequence", () => {
    const fixture = normalizationFixture("scoped-order", { "🧪️seed/🟦️.ts": "export const seed = true;\n" });
    const orderRoot = join(fixture.workspace, "🧪️order");
    try {
      normalizationWriteFiles(orderRoot, { "🧪️a/📝️.md": "a\n", "🧪️b/📝️.md": "b\n" });
      const firstEvents: TaxonomyProgress[] = [];
      const first = inventoryTaxonomy({ ...fixture.options, scope: `${fixture.scope}/🧪️order`, progress: (event) => firstEvents.push(event) });
      rmSync(orderRoot, { recursive: true, force: true });
      normalizationWriteFiles(orderRoot, { "🧪️b/📝️.md": "b\n", "🧪️a/📝️.md": "a\n" });
      const secondEvents: TaxonomyProgress[] = [];
      const second = inventoryTaxonomy({ ...fixture.options, scope: `${fixture.scope}/🧪️order`, progress: (event) => secondEvents.push(event) });
      expect(canonicalJson(second)).toBe(canonicalJson(first));
      expect(canonicalJson(secondEvents)).toBe(canonicalJson(firstEvents));
      expect(normalizationPhaseSequence(secondEvents)).toEqual(normalizationPhaseSequence(firstEvents));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("root taxonomy inventory CLI writes phases only to stderr and canonical JSON only to stdout", () => {
    const fixture = normalizationFixture("scoped-cli-purity", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    try {
      const scriptPath = join(getWorkspaceRoot(), "📜️script.ts");
      const child = 'const [scriptPath,repoRoot,scope]=process.argv.slice(1);const {pathToFileURL}=await import("node:url");const {CleanScript}=await import(pathToFileURL(scriptPath).href);new CleanScript(repoRoot,repoRoot).run(["taxonomy","inventory","--scope",scope,"--workers","1","--format","json"]);';
      const result = spawnSync("bun", ["-e", child, scriptPath, fixture.repoRoot, fixture.scope], { encoding: "utf8" });
      expect(result.status).toBe(0);
      const parsed = JSON.parse(result.stdout);
      expect(result.stdout).toBe(`${canonicalJson(parsed)}\n`);
      expect(result.stdout).not.toContain("[clean taxonomy progress]");
      expect(result.stderr).toContain("[clean taxonomy progress] inventory setup 0/1");
      expect(result.stderr).toContain("[clean taxonomy progress] inventory complete");
      expect(result.stderr.split(/\r?\n/u).filter(Boolean).every((line) => line.startsWith("[clean taxonomy progress]"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("a scoped apply ignores unrelated stale projection tokens", () => {
    const { fixture, externalPath, staleToken } = scopedUnrelatedProjectionStaleFixture("unrelated-stale");
    try {
      const plan = normalizationPlan(fixture).plan;
      expect(plan.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
      writeFileSync(join(fixture.repoRoot, externalPath), `# Unrelated\n\n${staleToken}\n`);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.state).toBe("committed");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);

  test("an exact ticket-important leaf scope ignores unrelated ticket evidence, rolls back, and commits its history projection", () => {
    const content = "Historical ticket evidence.\n";
    const fixture = normalizationFixture("ticket-important-history-apply", { "🟦️component.ts": "export const fixture = true;\n" }, ({ repoRoot, ticketDir }) => {
      const source = join(repoRoot, TICKET_IMPORTANT_HISTORY_SOURCE_REL);
      mkdirSync(dirname(source), { recursive: true });
      writeFileSync(source, content);
      normalizationWriteFiles(ticketDir, { "📓️unrelated/📝️.md": `Retained evidence names ${TICKET_IMPORTANT_HISTORY_SOURCE_REL}.\n` });
    });
    const options: TaxonomyInventoryOptions = { repoRoot: fixture.repoRoot, scope: TICKET_IMPORTANT_HISTORY_SOURCE_REL, ticketDir: fixture.ticketDir, workers: 1 };
    try {
      const inventory = inventoryTaxonomy(options);
      const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(plan.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
      expect(plan.moves.map((move) => [move.sourcePath, move.destinationPath, move.rationaleRule])).toEqual([[TICKET_IMPORTANT_HISTORY_SOURCE_REL, TICKET_IMPORTANT_HISTORY_DESTINATION_REL, "ticket-important-history-markdown-v1"]]);
      expect(plan.evidenceRemovals).toEqual([]);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "before-verify" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(readFileSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_SOURCE_REL), "utf8")).toBe(content);
      expect(existsSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_DESTINATION_REL))).toBe(false);
      const applied = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(applied.state).toBe("committed");
      expect(existsSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_SOURCE_REL))).toBe(false);
      expect(readFileSync(join(fixture.repoRoot, TICKET_IMPORTANT_HISTORY_DESTINATION_REL), "utf8")).toBe(content);
      const second = planTaxonomy(inventoryTaxonomy(options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(second.moves).toEqual([]);
      expect(second.evidenceRemovals).toEqual([]);
      expect(second.unresolved.filter((entry) => entry.severity === "error")).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);

  test("exact CAD source scope plans declared external edits and rejects declared stale consumers before mutation", () => {
    const fixture = artifactProjectionNormalizationFixture("declared-external", true);
    let passed = false;
    try {
      const cad = projectionGolden("artifact-example-model-catalog-v1");
      const exactScope = `${fixture.scope}/${artifactProjectionFixturePath(cad.sourceRoot)}`;
      const schemaPath = join(fixture.repoRoot, NORMALIZATION_SCHEMA_REL);
      const schema = JSON.parse(readFileSync(schemaPath, "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { projectionContractId: string; sourcePathIdentities: string[]; sourcePathPattern: string; staleMarkers: string[] }> };
      const contract = Object.values(schema.semanticPathProjectionReferenceConsumerContracts).find((entry) => entry.projectionContractId === cad.contractId)!;
      const declaredStalePath = `${fixture.scope}/🧪️declared-external/🟦️.ts`;
      contract.sourcePathIdentities.push(declaredStalePath);
      contract.sourcePathPattern = `^(?:${contract.sourcePathIdentities.map((path) => path.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|")})$`;
      normalizationWriteFiles(fixture.repoRoot, { [declaredStalePath]: `export const stale = ${JSON.stringify(contract.staleMarkers[0])};\n` });
      writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
      const inventory = inventoryTaxonomy({ ...fixture.options, scope: exactScope });
      const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
      expect(plan.regenerations.filter(({ contractId }) => contractId === "plugin-registry")).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.pluginRegistryRegenerationCount);
      expect(plan.unresolved.filter((entry) => entry.code === "projection-old-token-stale" && entry.path === declaredStalePath)).toHaveLength(1);
      expect(plan.moves.filter((move) => move.rationaleRule === cad.rationaleRule)).toHaveLength(cad.sourceFileCount);
      expect(plan.edits.some((edit) => !inScopeForTest(edit.path, exactScope))).toBe(true);
      expect(plan.edits.some((edit) => edit.path === declaredStalePath)).toBe(false);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest }));
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  }, 60_000);

  test("inventory bytes are deterministic and its language-agnostic file census agrees with owned filesystem discovery", () => {
    const fixture = normalizationFixture("inventory", {
      ".hidden/🔣️config.json": "{}\n",
      "Cargo.toml": "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\n",
      "🦀️component.rs": "pub struct Fixture;\n",
      "🧪️component/🟦️component.ts": "export const fixture = true;\n",
      "🤖️generated/🟦️component.ts": "export const generated = true;\n",
    });
    try {
      let progressEvents = 0;
      const options: TaxonomyInventoryOptions = { ...fixture.options, progress: () => { progressEvents += 1; } };
      const first = inventoryTaxonomy(options);
      const second = inventoryTaxonomy(options);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(progressEvents).toBeGreaterThan(0);
      const expected = ownedFilePaths(fixture.workspace)
        .map((path) => `${fixture.scope}/${path.replaceAll("\\", "/")}`)
        .sort();
      const actual = first.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath).sort();
      expect(actual).toEqual(expected);
      expect(first.entries.every((entry) => !entry.sourcePath.startsWith("compose/"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("plans physical-format leaves, compound extensions, generic-stem drops, parent ownership, assets, generated output, and exact fixed/configurable entries", () => {
    const fixture = normalizationFixture("kinds", {
      "🧪️case/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"fixture-rust\"\nversion = \"0.0.0\"\n",
      "🧪️case/📦️packages/🟦️typescript/package.json": "{\"name\":\"fixture-typescript\",\"exports\":{\".\":\"./🟦️.ts\"}}\n",
      "🧪️case/📦️packages/🟦️typescript/🟦️.ts": "export {};\n",
      "🧪️case/🦀️component.rs": "pub struct Component;\n",
      "🧪️case/🟦️component.ts": "export const component = true;\n",
      "🧪️case/🧪️react-component/🟦️react-component.tsx": "export const ReactComponent = () => null;\n",
      "🧪️case/🧪️types/🟦️types.d.ts": "export interface FixtureType { readonly value: number }\n",
      "🧪️case/🧪️contract/🔣️contract.json": "{\"valid\":true}\n",
      "🧪️case/🧪️guide/📝️guide.md": "# Fixture guide\n",
      "🧪️case/🖼️asset.png": "not-a-decoded-image\n",
      "🧪️case/🤖️generated/🟦️component.ts": "export const generated = true;\n",
    });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(normalizationMove(plan, "🧪️case/🦀️component.rs").destinationPath).toMatch(/\/🧪️case\/🦀️\.rs$/u);
      expect(normalizationMove(plan, "🧪️case/🟦️component.ts").destinationPath).toMatch(/\/🧪️case\/🟦️\.ts$/u);
      expect(normalizationMove(plan, "🧪️react-component/🟦️react-component.tsx").destinationPath).toMatch(/\/🧪️react-component\/🟦️\.tsx$/u);
      expect(normalizationMove(plan, "🧪️types/🟦️types.d.ts").destinationPath).toMatch(/\/🧪️types\/🟦️\.d\.ts$/u);
      expect(normalizationMove(plan, "🧪️contract/🔣️contract.json").destinationPath).toMatch(/\/🧪️contract\/🔣️\.json$/u);
      expect(normalizationMove(plan, "🧪️guide/📝️guide.md").destinationPath).toMatch(/\/🧪️guide\/📝️\.md$/u);
      const asset = normalizationMove(plan, "🧪️case/🖼️asset.png").destinationPath;
      expect(asset.endsWith("/🖼️.png")).toBe(true);
      expect(asset).not.toContain("asset.png");
      expect(normalizationMove(plan, "🧪️case/🤖️generated/🟦️component.ts").destinationPath).toContain("/🤖️generated/");
      expect(plan.moves.some((move) => move.sourcePath.endsWith("/Cargo.toml"))).toBe(false);
      expect(plan.moves.some((move) => move.sourcePath.endsWith("/package.json"))).toBe(false);
      expect(plan.moves.some((move) => move.sourcePath.endsWith("/🟦️.ts"))).toBe(false);
      expect(plan.planDigest).toBe(taxonomyPlanDigest(plan));
      expect(plan.sourceTreeDigest.length).toBeGreaterThan(0);
      expect(plan.expectedPostStateDigest.length).toBeGreaterThan(0);
      expect(canonicalJson(plan)).toBe(canonicalJson(planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] })));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("preserves the D3D12, PDF mutation, and BMP pilot destinations exactly", () => {
    const fixture = normalizationFixture("named-pilot", {
      "🧪️golden/🪟️d3d12/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"d3d12-fixture\"\nversion = \"0.0.0\"\n",
      "🧪️golden/🪟️d3d12/📦️packages/🦀️rust/🦀️backend.rs": "pub struct Backend { pub device: usize }\n",
      "🧪️golden/🧪️tests/🧪️mutate-pdf-1-7/🦀️component.rs": "pub fn mutate() {}\n",
      "🧪️golden/🧪️tests/🧪️mutate-pdf-1-7/component.feature": "Feature: mutate PDF 1.7\n",
      "🧪️golden/🖼️rathaus-ahlen-grundriss.bmp": "BMfixture\n",
    });
    try {
      const { plan } = normalizationPlan(fixture);
      const prefix = fixture.scope;
      expect(normalizationMove(plan, "🪟️d3d12/📦️packages/🦀️rust/🦀️backend.rs").destinationPath).toBe(`${prefix}/🧪️golden/🪟️d3d12/⚙️backend/🦀️.rs`);
      const rustCase = normalizationMove(plan, "🧪️tests/🧪️mutate-pdf-1-7/🦀️component.rs").destinationPath;
      const featureCase = normalizationMove(plan, "🧪️tests/🧪️mutate-pdf-1-7/component.feature").destinationPath;
      expect(rustCase).toMatch(/\/🧪️mutate-pdf-1-7\/🦀️\.rs$/u);
      expect(featureCase).toBe(rustCase.replace(/🦀️\.rs$/u, "🥒️.feature"));
      expect(normalizationMove(plan, "🖼️rathaus-ahlen-grundriss.bmp").destinationPath).toBe(`${prefix}/🧪️golden/🖼️rathaus-ahlen-grundriss/🖼️.bmp`);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("normalizes NFC and VS16 and blocks same-kind, platform, and path-budget hazards", () => {
    const longPath = Array.from({ length: 18 }, (_, index) => `🧩️segment-${String(index).padStart(2, "0")}`).join("/");
    const fixture = normalizationFixture("collisions", {
      "🧪️collision/🟦️component.ts": "export const component = 1;\n",
      "🧪️collision/🟦️index.ts": "export const index = 2;\n",
      "🧪️unicode/🦀component.rs": "pub struct MissingVs16;\n",
      "🧪️unicode/🧪️cafe\u0301/🟦️component.ts": "export const nfc = true;\n",
      "🧪️platform/CON.ts": "export const reserved = true;\n",
      [`🧪️length/${longPath}/🟦️component.ts`]: "export const long = true;\n",
    });
    try {
      const { inventory, plan } = normalizationPlan(fixture);
      const rendered = canonicalJson(plan);
      expect(normalizationMove(plan, "🧪️unicode/🦀component.rs").destinationPath).toContain("🦀️.rs");
      expect(inventory.entries.some((entry) => entry.sourcePath.endsWith("🧪️unicode/🧪️cafe\u0301/🟦️component.ts"))).toBe(true);
      expect(normalizationMove(plan, "🧪️unicode/🧪️cafe\u0301/🟦️component.ts").destinationPath.normalize("NFC")).toBe(normalizationMove(plan, "🧪️unicode/🧪️cafe\u0301/🟦️component.ts").destinationPath);
      expect(plan.moves.filter((move) => move.destinationPath.endsWith("/🟦️.ts")).length).toBeGreaterThanOrEqual(2);
      expect(rendered).toMatch(/collision/i);
      expect(rendered).toMatch(/(?:reserved|platform|windows)/i);
      expect(rendered).toMatch(/(?:path.{0,20}length|length.{0,20}path|240)/i);
      expect(plan.unresolved.length + inventory.violations.length).toBeGreaterThan(0);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("structured reference adapters emit locations, old/new values, and preimage hashes", () => {
    const fixture = normalizationFixture("references", {
      "package.json": "{\"name\":\"fixture\",\"exports\":{\".\":\"./🧪️subject/🟦️component.ts\"}}\n",
      "📋️project.json": "{\"targets\":{\"build\":{\"options\":{\"entryFile\":\"🧪️subject/🟦️component.ts\"}}}}\n",
      "🧪️subject/🦀️component.rs": "pub const VALUE: i32 = 1;\n",
      "🧪️subject/🟦️component.ts": "export const value = 1;\n",
      "🧪️subject/component.cpp": "int value() { return 1; }\n",
      "fixture/subject/component.py": "value = 1\n",
      "🧪️consumer/🦀️component.rs": "#[path = \"../🧪️subject/🦀️component.rs\"]\nmod subject;\n",
      "🧪️consumer/🟦️component.ts": "export { value } from \"../🧪️subject/🟦️component.ts\";\n",
      "🧪️consumer/🐹️component.go": "package consumer\n//go:embed ../🧪️subject/🟦️component.ts\n",
      "🧪️consumer/🐍️component.py": "from fixture.subject.component import value\n",
      "🧪️consumer/fixture.csproj": "<Project><ItemGroup><Compile Include=\"../🧪️subject/🟦️component.ts\" /></ItemGroup></Project>\n",
      "🧪️consumer/🔣️paths.json": "{\"source\":\"../🧪️subject/🟦️component.ts\"}\n",
      "🧪️consumer/🔣️paths.jsonc": "{\"source\":\"../🧪️subject/🟦️component.ts\"}\n",
      "🧪️consumer/🔣️paths.toml": "source = \"../🧪️subject/🟦️component.ts\"\n",
      "🧪️consumer/🔣️paths.yaml": "source: ../🧪️subject/🟦️component.ts\n",
      "🧪️consumer/🔣️paths.xml": "<Source path=\"../🧪️subject/🟦️component.ts\" />\n",
      "🧪️consumer/📖️references.md": "[source](../🧪️subject/🟦️component.ts)\n",
      "🧪️consumer/CMakeLists.txt": "target_sources(fixture PRIVATE \"../🧪️subject/component.cpp\")\n",
    });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.edits.length).toBeGreaterThanOrEqual(5);
      for (const edit of plan.edits) {
        expect(edit.path.length).toBeGreaterThan(0);
        expect(edit.adapter.length).toBeGreaterThan(0);
        expect(edit.structuredLocation.length).toBeGreaterThan(0);
        expect(edit.oldValue.length).toBeGreaterThan(0);
        expect(edit.newValue.length).toBeGreaterThan(0);
        expect(edit.preimage.contentHash.length).toBeGreaterThan(0);
      }
      const adapters = plan.edits.map((edit) => edit.adapter.toLowerCase()).join(" ");
      expect(adapters).toMatch(/rust/);
      expect(adapters).toMatch(/(?:typescript|javascript|ts)/);
      expect(adapters).toMatch(/go/);
      expect(adapters).toMatch(/python/);
      expect(adapters).toMatch(/dotnet/);
      expect(adapters).toMatch(/json/);
      expect(adapters).toMatch(/jsonc/);
      expect(adapters).toMatch(/toml/);
      expect(adapters).toMatch(/yaml/);
      expect(adapters).toMatch(/xml/);
      expect(adapters).toMatch(/markdown/);
      expect(adapters).toMatch(/(?:native|cmake)/);
      expect(plan.moves.some((move) => move.referenceEdits.length > 0)).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("package boundaries accept thin Rust/TypeScript glue and reject domain implementation", () => {
    const fixture = normalizationFixture("package-glue", {
      "🧪️thin/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"thin-rust\"\nversion = \"0.0.0\"\n[lib]\npath = \"📦️glue.rs\"\n",
      "🧪️thin/📦️packages/🦀️rust/📦️glue.rs": "pub use crate::component::*;\n",
      "🧪️thin/📦️packages/🟦️typescript/package.json": "{\"name\":\"thin-ts\",\"exports\":{\".\":\"./📦️index.ts\"}}\n",
      "🧪️thin/📦️packages/🟦️typescript/📦️index.ts": "export {};\n",
      "🧪️thick/📦️packages/🦀️rust/Cargo.toml": "[package]\nname = \"thick-rust\"\nversion = \"0.0.0\"\n[lib]\npath = \"📦️glue.rs\"\n",
      "🧪️thick/📦️packages/🦀️rust/📦️glue.rs": "pub struct Domain { pub value: i32 }\nimpl Domain { pub fn calculate(&self) -> i32 { self.value * 2 + 1 } }\n",
      "🧪️thick/📦️packages/🟦️typescript/package.json": "{\"name\":\"thick-ts\",\"exports\":{\".\":\"./📦️index.ts\"}}\n",
      "🧪️thick/📦️packages/🟦️typescript/📦️index.ts": "export function calculate(values: number[]): number { return values.reduce((sum, value) => sum + value, 0); }\n",
    });
    try {
      const inventory = inventoryTaxonomy(fixture.options);
      const thin = inventory.entries.filter((entry) => entry.sourcePath.includes("/🧪️thin/📦️packages/") && /(?:glue|index)\.(?:rs|ts)$/u.test(entry.sourcePath));
      const thick = inventory.entries.filter((entry) => entry.sourcePath.includes("/🧪️thick/📦️packages/") && /(?:glue|index)\.(?:rs|ts)$/u.test(entry.sourcePath));
      expect(thin).toHaveLength(2);
      expect(thick).toHaveLength(2);
      const packageViolation = (violation: { readonly code: string; readonly message: string }): boolean => /(?:package|glue|implementation)/i.test(`${violation.code} ${violation.message}`);
      expect(thin.flatMap((entry) => entry.violations).filter(packageViolation)).toEqual([]);
      expect(thin.map((entry) => entry.packageRole)).toEqual(["declaration", "declaration"]);
      expect(thick.map((entry) => entry.packageRole)).toEqual(["implementation", "implementation"]);
      expect(thick.every((entry) => entry.violations.some(packageViolation))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("stale preimages block apply without changing source bytes", () => {
    const fixture = normalizationFixture("stale", { "🦀️component.rs": "pub const VALUE: i32 = 1;\n" });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(plan.moves.length).toBeGreaterThan(0);
      writeFileSync(join(fixture.workspace, "🦀️component.rs"), "pub const VALUE: i32 = 2;\n");
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest }));
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  for (const failureStage of ["after-staging", "after-moves", "after-edits", "before-verify"] as const) {
    test(`rolls back byte-for-byte when failure is injected at ${failureStage}`, () => {
      const fixture = normalizationFixture(`failure-${failureStage}`, {
        "🧪️subject/🟦️component.ts": "export const value = 1;\n",
        "🧪️consumer/🟦️component.ts": "export { value } from \"🧪️tests/🧪️fixture/🧪️subject/🟦️component.ts\";\n",
      });
      try {
        const { plan } = normalizationPlan(fixture);
        expect(plan.moves.length).toBeGreaterThan(0);
        const before = normalizationWorkspaceSnapshot(fixture.workspace);
        expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: failureStage }));
        expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
        expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
      } finally {
        rmSync(fixture.root, { recursive: true, force: true });
      }
    });
  }

  test("cancellation rolls back and a successful retry converges to an empty second plan", () => {
    const cancelled = normalizationFixture("cancel", { "🟦️component.ts": "export const value = 1;\n" });
    try {
      const { plan } = normalizationPlan(cancelled);
      const before = normalizationWorkspaceSnapshot(cancelled.workspace);
      const cancelFile = join(cancelled.ticketDir, "cancel");
      writeFileSync(cancelFile, "cancel\n");
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, cancelFile }));
      expect(normalizationWorkspaceSnapshot(cancelled.workspace)).toEqual(before);
      expect(opaqueTreeDigest(cancelled.repoRoot, "compose")).toEqual(cancelled.opaqueDigest);
    } finally {
      rmSync(cancelled.root, { recursive: true, force: true });
    }

    const fixture = normalizationFixture("convergence", { "🦀️component.rs": "pub const VALUE: i32 = 1;\n" });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.planDigest).toBe(plan.planDigest);
      expect(result.appliedMoves).toBe(plan.moves.length);
      expect(result.appliedEdits).toBe(plan.edits.length);
      expect(String(result.state)).toMatch(/^(?:applied|committed|complete)$/u);
      const verification = verifyTaxonomy({ ...fixture.options, baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(canonicalJson(verification)).not.toMatch(/"severity":"error"/u);
      const secondInventory = inventoryTaxonomy(fixture.options);
      const secondPlan = planTaxonomy(secondInventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(secondPlan.moves).toEqual([]);
      expect(secondPlan.edits).toEqual([]);
      expect(secondPlan.regenerations).toEqual([]);
      expect(secondPlan.unresolved).toEqual([]);
      expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);
});
//#endregion 🧹️TaxonomyNormalization

//#region 🧾️GeneratorPreviewProtocol
const GENERATOR_PREVIEW_GOLDEN = join(import.meta.dir, "🧫️fixtures", "🧪️generator-preview", "🔣️.json");

describe("generator preview protocol", () => {
  test("accepts the canonical language-neutral manifest and agrees with owned filesystem inventory", () => {
    const canonical = `${canonicalJson(JSON.parse(readFileSync(GENERATOR_PREVIEW_GOLDEN, "utf8")))}\n`;
    const manifest = parseGeneratorPreviewManifest(canonical, "fixture-generator", ["🧪️workspace/🤖️generated"], ["compose"]);
    const root = mkdtempSync(join(tmpdir(), "generator-preview-parity-"));
    try {
      for (const node of manifest.nodes) {
        const target = join(root, node.path);
        if (node.nodeKind === "directory") mkdirSync(target, { recursive: true, mode: node.mode });
        else {
          mkdirSync(resolve(target, ".."), { recursive: true });
          writeFileSync(target, Buffer.from(node.bytesBase64, "base64"), { mode: node.mode });
        }
      }
      const platformBoundary = ownedFilesystemEntries(join(root, "🧪️workspace", "🤖️generated"), true).map(({ path }) => path === "." ? "🧪️workspace/🤖️generated" : `🧪️workspace/🤖️generated/${path}`);
      expect(platformBoundary).toEqual(manifest.nodes.map((node) => node.path));
      expect(manifest.staleRemovals).toEqual(["🧪️workspace/🤖️generated/old.txt"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects noncanonical, noisy, unsafe, duplicate, and malformed manifests", () => {
    const valid = JSON.parse(readFileSync(GENERATOR_PREVIEW_GOLDEN, "utf8")) as Record<string, unknown>;
    const render = (value: unknown): string => `${canonicalJson(value)}\n`;
    const cases: readonly string[] = [
      `noise\n${render(valid)}`,
      JSON.stringify(valid),
      `${JSON.stringify({ schemaVersion: 1, contractId: valid.contractId, nodes: valid.nodes, staleRemovals: valid.staleRemovals })}\n`,
      render({ ...valid, schemaVersion: 2 }),
      render({ ...valid, contractId: "different-generator" }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }, { bytesBase64: "%%%", mode: 420, nodeKind: "file", path: "🧪️workspace/🤖️generated/🔤️.txt" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493.5, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "symlink", path: "🧪️workspace/🤖️generated" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "../outside" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "compose/generated" }] }),
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }, { bytesBase64: "Z2VuZXJhdGVkXG4=", mode: 420, nodeKind: "file", path: "🧪️workspace/🤖️generated/cafe\u0301.txt" }] }),
      `${JSON.stringify({ ...valid, nodes: [...(valid.nodes as unknown[])].reverse() })}\n`,
      render({ ...valid, nodes: [{ bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }, { bytesBase64: "", mode: 493, nodeKind: "directory", path: "🧪️workspace/🤖️generated" }] }),
      render({ ...valid, staleRemovals: ["🧪️workspace/🤖️generated/old.txt", "🧪️workspace/🤖️generated/old.txt"] }),
      render({ ...valid, staleRemovals: ["🧪️workspace/🤖️generated/🔤️.txt"] }),
    ];
    for (const content of cases) expect(() => parseGeneratorPreviewManifest(content, "fixture-generator", ["🧪️workspace/🤖️generated"], ["compose"])).toThrow();
  });

  test("plans, applies, verifies, and converges an exact Nx-owned preview", () => {
    const fixture = normalizationFixture(
      "generator-preview",
      {
        "🧪️generator/🟦️.ts": "export const input = true;\n",
        "🧪️generator/🤖️generated/old.txt": "stale\n",
      },
      ({ repoRoot, workspace }) => {
        const owner = relative(repoRoot, join(workspace, "🧪️generator")).replaceAll("\\", "/");
        const outputRoot = `${owner}/🤖️generated`;
        const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
        const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
        taxonomy.generatorContracts = Object.fromEntries(Object.entries({
          ...(taxonomy.generatorContracts as Record<string, unknown>),
          "fixture-generator": {
            ownership: "owned",
            ownerPath: owner,
            target: "@fixture/generator:generate",
            previewTarget: "@fixture/generator:preview-generated",
            checkTarget: "@fixture/generator:check",
            inputPatterns: [`${owner}/🟦️.ts`],
            outputRoots: [{ path: outputRoot, inclusion: "ignored" }],
            reason: "Language-neutral generator preview fixture",
          },
        }).sort(([left], [right]) => left.localeCompare(right)));
        writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
        writeFileSync(join(repoRoot, ".gitignore"), `${outputRoot}\n`);
        writeFileSync(join(repoRoot, "nx.json"), "{\"defaultBase\":\"main\"}\n");
        writeFileSync(join(repoRoot, "package.json"), "{\"name\":\"generator-preview-fixture\",\"private\":true}\n");
        const projectManifest = `${JSON.stringify({
          name: "@fixture/generator",
          root: owner,
          targets: {
            generate: { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts generate" } },
            "preview-generated": { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts preview-generated" } },
            check: { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts check" } },
          },
        }, null, 2)}\n`;
        writeFileSync(join(repoRoot, "project.json"), projectManifest);
        normalizationWriteFiles(join(workspace, "🧪️generator"), {
          "📋️project.json": projectManifest,
          "📜️script.ts": [
            'import { existsSync, mkdirSync, readdirSync, rmSync, writeFileSync, readFileSync } from "node:fs";',
            'import { join, relative } from "node:path";',
            `const repoRoot = ${JSON.stringify(repoRoot)};`,
            `const outputRoot = ${JSON.stringify(join(repoRoot, outputRoot))};`,
            `const outputRelative = ${JSON.stringify(outputRoot)};`,
            'const outputFile = join(outputRoot, "🔤️.txt");',
            'const bytes = Buffer.from("generated\\n");',
            'const nodes = [{ bytesBase64: "", mode: 0o755, nodeKind: "directory", path: outputRelative }, { bytesBase64: bytes.toString("base64"), mode: 0o644, nodeKind: "file", path: `${outputRelative}/🔤️.txt` }];',
            'const staleRemovals = (existsSync(outputRoot) ? readdirSync(outputRoot) : []).filter((name) => name !== "🔤️.txt").map((name) => `${outputRelative}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));',
            'const command = process.argv[2];',
            'if (command === "preview-generated") process.stdout.write(`${JSON.stringify({ contractId: "fixture-generator", nodes, schemaVersion: 1, staleRemovals })}\\n`);',
            'else if (command === "generate") { rmSync(outputRoot, { recursive: true, force: true }); mkdirSync(outputRoot, { recursive: true, mode: 0o755 }); writeFileSync(outputFile, bytes, { mode: 0o644 }); }',
            'else if (command === "check") { if (!existsSync(outputFile) || !readFileSync(outputFile).equals(bytes) || readdirSync(outputRoot).join("\\0") !== "🔤️.txt") throw new Error("generated output is stale"); }',
            'else throw new Error(`unknown command ${command}`);',
            '',
          ].join("\n"),
        });
      },
    );
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(plan.regenerations).toHaveLength(1);
      expect(plan.regenerations[0]?.contractId).toBe("fixture-generator");
      expect(plan.regenerations[0]?.preview.nodes.map((node) => node.path)).toEqual(plan.regenerations[0]?.outputs.map((node) => node.path));
      expect(plan.regenerations[0]?.staleRemovals).toEqual([`${fixture.scope}/🧪️generator/🤖️generated/old.txt`]);
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: "after-edits" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.state).toBe("committed");
      expect(JSON.parse(readFileSync(result.journalPath, "utf8")).attemptOrdinal).toBe("000002");
      const second = planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(second.regenerations).toEqual([]);
      expect(second.unresolved).toEqual([]);
    } finally {
      if (process.env.KEEP_GENERATOR_FIXTURE !== "1") rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);
});
//#endregion 🧾️GeneratorPreviewProtocol

//#region 🧾️TransactionDispositionsV2
describe("taxonomy transaction dispositions v2", () => {
  const goldenPath = resolve(import.meta.dir, "🧫️fixtures/🧪️transaction-dispositions/🔣️.json");

  test("keeps checkout-hostile sentinels virtual and language-neutral", () => {
    const golden = JSON.parse(readFileSync(goldenPath, "utf8")) as {
      expectedDispositionOperations: { embeddedTicketRoots: number; embeddedTicketRootRelocations: number; evidenceRemovals: number; operationFamilies: string[] };
      virtualPreimageNodes: { path: string; state: "absent" | "directory" | "file" | "symlink"; contentHash?: string; size?: number; target?: string }[];
      affectedStateCases: { id: string; pre: unknown[]; post: unknown[] }[];
      failureStages: string[];
      journalStates: string[];
      negativeDispositionCases: { id: string; expectedCode: string }[];
      virtualPathPolicyCases: { inputPath: string; expectedViolationCode: string }[];
      symlinkFlavorCases: { repositoryRoot: string; target: string; owned: boolean }[];
    };
    for (const row of golden.virtualPathPolicyCases) expect(taxonomyPlatformPathViolationCodes(row.inputPath)).toContain(row.expectedViolationCode);
    for (const row of golden.symlinkFlavorCases) expect(repositoryLocalSymlinkTargetPath(row.repositoryRoot, row.target) !== null).toBe(row.owned);
    expect(golden.expectedDispositionOperations).toEqual({ embeddedTicketRoots: 3, embeddedTicketRootRelocations: 4, evidenceRemovals: 2, operationFamilies: ["moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "edits", "regenerations"] });
    expect(golden.failureStages).toEqual(["after-staging", "after-embedded-root-staging", "after-moves", "after-relocations", "after-symlink-retargeting", "after-edits", "after-regenerations", "before-verify"]);
    expect(golden.journalStates).toHaveLength(11);
    expect(golden.affectedStateCases.map((row) => row.id)).toEqual(["remove-redundant-evidence", "retarget-no-follow-symlink"]);
    expect(new Set(golden.negativeDispositionCases.map((row) => row.expectedCode)).size).toBe(golden.negativeDispositionCases.length);
    expect(ownedFilePaths(resolve(goldenPath, ".."))).toEqual(["🔣️.json"]);
    const root = mkdtempSync(join(tmpdir(), "transaction-golden-nofollow-"));
    try {
      mkdirSync(join(root, "evidence", "directory"), { recursive: true });
      writeFileSync(join(root, "evidence", "file.txt"), "sentinel\n");
      symlinkSync("../file.txt", join(root, "evidence", "link"));
      const native = noFollowTreeDigest(root, "evidence");
      const platformBoundary = ownedFilesystemEntries(join(root, "evidence"), true).map(({ path }) => path === "." ? "evidence" : `evidence/${path}`);
      expect(native.files).toBe(1);
      expect(native.directories).toBe(2);
      expect(native.symlinks).toBe(1);
      expect(platformBoundary).toEqual(["evidence", "evidence/directory", "evidence/file.txt", "evidence/link"]);
      expect(golden.virtualPreimageNodes.find((row) => row.state === "file")?.contentHash).toBe("b5f7e7d285029324d9b3acae19cc05099271454ac98bfc059a92b0581625cd51");
      expect(golden.virtualPreimageNodes.find((row) => row.state === "symlink")?.target).toBe("../file.txt");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects v1 and incomplete v2 plans without compatibility defaults", () => {
    expect(() => parseTaxonomyPlan({ schemaVersion: 1 })).toThrow();
    expect(() => parseTaxonomyPlan({ schemaVersion: 2, taxonomySchemaVersion: 7 })).toThrow();
  });

  test("hashes symlink target text without following its target", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-transaction-no-follow-"));
    try {
      const linkRoot = join(root, "links");
      mkdirSync(linkRoot);
      writeFileSync(join(root, "target.txt"), "first");
      symlinkSync("../target.txt", join(linkRoot, "link"));
      const before = noFollowTreeDigest(linkRoot, ".");
      writeFileSync(join(root, "target.txt"), "second");
      const afterTarget = noFollowTreeDigest(linkRoot, ".");
      expect(afterTarget.digest).toBe(before.digest);
      expect(afterTarget.symlinks).toBe(1);
      rmSync(join(linkRoot, "link"));
      symlinkSync("../missing.txt", join(linkRoot, "link"));
      expect(noFollowTreeDigest(linkRoot, ".").digest).not.toBe(afterTarget.digest);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("retargets a repository-local broken absolute symlink atomically and converges", () => {
    const fixture = normalizationFixture("transaction-symlink-v2", {}, ({ workspace }) => symlinkSync(join(workspace, "target.ts"), join(workspace, "🟦️.ts")));
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.symlinkTargetEdits).toHaveLength(1);
      expect(plan.unresolved).toEqual([]);
      expect(parseTaxonomyPlan(JSON.parse(JSON.stringify(plan)))).toEqual(plan);
      const tamperedTarget = JSON.parse(JSON.stringify(plan)) as TaxonomyPlan;
      (tamperedTarget.symlinkTargetEdits[0] as { oldTarget: string }).oldTarget = `${tamperedTarget.symlinkTargetEdits[0].oldTarget}-drift`;
      (tamperedTarget as { planDigest: string }).planDigest = taxonomyPlanDigest(tamperedTarget);
      expect(() => parseTaxonomyPlan(tamperedTarget)).toThrow();
      expect(() => parseTaxonomyPlan({ ...plan, unexpected: true })).toThrow();
      const deadPreparation = join(fixture.ticketDir, "🧾️taxonomy-transaction", `🔖️${plan.planDigest}`, "🔂️attempts", "🚧️prepare-000001-999999-00000000-0000-4000-8000-000000000001");
      mkdirSync(deadPreparation, { recursive: true });
      const before = noFollowTreeDigest(fixture.repoRoot, fixture.scope);
      const rolledBack = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, injectFailureAt: "after-symlink-retargeting" });
      expect(rolledBack.state).toBe("rolled-back");
      expect(existsSync(deadPreparation)).toBe(false);
      expect(noFollowTreeDigest(fixture.repoRoot, fixture.scope)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const applied = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit });
      expect(applied.appliedSymlinkTargetEdits).toBe(1);
      expect(applied.journalPath).toMatch(/🔢️000002\/🔣️\.json$/u);
      const terminalAttempt = resolve(applied.journalPath, "..");
      const deadLease = join(terminalAttempt, "🔒️lease");
      mkdirSync(deadLease);
      writeFileSync(join(deadLease, "🔣️.json"), `${canonicalJson({ schemaVersion: 1, planDigest: plan.planDigest, attemptOrdinal: "000002", token: "00000000-0000-4000-8000-000000000002", pid: 999999 })}\n`);
      const resumedCommitted = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, resumeJournal: applied.journalPath });
      expect(resumedCommitted.state).toBe("committed");
      expect(readdirSync(terminalAttempt)).toEqual(["🔣️.json"]);
      const second = planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(second.symlinkTargetEdits).toEqual([]);
      expect(second.unresolved).toEqual([]);
    } finally {
      if (process.env.KEEP_TRANSACTION_FIXTURE !== "1") rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 12_000);

  test("resumes an installed symlink retarget from its durable journal marker", () => {
    const fixture = normalizationFixture("transaction-symlink-resume-v2", {}, ({ workspace }) => symlinkSync(join(workspace, "target.ts"), join(workspace, "🟦️.ts")));
    try {
      const { plan } = normalizationPlan(fixture);
      const planPath = join(fixture.ticketDir, "plan-v2.json");
      writeFileSync(planPath, `${canonicalJson(plan)}\n`);
      const modulePath = resolve(import.meta.dir, "../../🧹️normalization/🟦️.ts");
      const child = `const [modulePath,planPath,repoRoot,ticketDir,baseline]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(modulePath);const plan=JSON.parse(await Bun.file(planPath).text());applyTaxonomyPlan(plan,{repoRoot,ticketDir,expectedBaselineCommit:baseline,progress:(row)=>{if(row.phase==="retargeting-symlinks")process.exit(73)}});`;
      const interrupted = spawnSync("bun", ["-e", child, modulePath, planPath, fixture.repoRoot, fixture.ticketDir, plan.baselineCommit], { encoding: "utf8" });
      expect(interrupted.status).toBe(73);
      const journals = ownedFilePaths(fixture.ticketDir)
        .filter((path) => (path.startsWith("🧾️taxonomy-transaction/") || path.includes("/🧾️taxonomy-transaction/")) && /\/🔂️attempts\/🔢️[^/]+\/🔣️\.json$/u.test(path))
        .map((path) => join(fixture.ticketDir, path));
      expect(journals).toHaveLength(1);
      const leasePath = join(resolve(journals[0], ".."), "🔒️lease", "🔣️.json");
      expect(existsSync(leasePath)).toBe(true);
      const leaseBytes = readFileSync(leasePath, "utf8");
      const lease = JSON.parse(leaseBytes) as { pid: number };
      writeFileSync(leasePath, `${canonicalJson({ ...lease, pid: process.pid })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/leased by active pid/u);
      writeFileSync(leasePath, leaseBytes);
      let journalBytes = readFileSync(journals[0], "utf8");
      let journal = JSON.parse(journalBytes) as { revision: number; state: string; stagingRoot: string; installedSymlinkTargetEditIds: string[] };
      expect(journal.state).toBe("retargeting");
      expect(journal.installedSymlinkTargetEditIds).toEqual([plan.symlinkTargetEdits[0].operationId]);
      const stageRoot = join(fixture.repoRoot, journal.stagingRoot);
      const walRoot = join(stageRoot, "🚧️journal");
      mkdirSync(walRoot);
      writeFileSync(join(walRoot, "🔣️.json"), `${canonicalJson({ ...journal, revision: journal.revision + 1 })}\n`);
      writeFileSync(join(stageRoot, "unexpected"), "drift");
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/(?:unexpected|unauthorized) evidence/u);
      expect(existsSync(walRoot)).toBe(true);
      expect(readFileSync(journals[0], "utf8")).toBe(journalBytes);
      rmSync(walRoot, { recursive: true });
      expect(existsSync(leasePath)).toBe(true);
      rmSync(join(stageRoot, "unexpected"));
      journalBytes = readFileSync(journals[0], "utf8");
      journal = JSON.parse(journalBytes) as typeof journal;
      mkdirSync(walRoot);
      writeFileSync(join(stageRoot, "unexpected"), "drift");
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/(?:unexpected|unauthorized) evidence/u);
      expect(existsSync(walRoot)).toBe(true);
      rmSync(join(stageRoot, "unexpected"));
      writeFileSync(journals[0], JSON.stringify(journal, null, 2));
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/canonical JSON/u);
      writeFileSync(journals[0], `${canonicalJson({ ...journal, state: "staging" })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/impossible phase/u);
      writeFileSync(journals[0], `${canonicalJson({ ...journal, stagingRoot: `${journal.stagingRoot}-forged` })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] })).toThrow(/(?:attempt|transaction) roots/u);
      writeFileSync(journals[0], journalBytes);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, resumeJournal: journals[0] });
      expect(result.state).toBe("committed");
      expect(readdirSync(resolve(journals[0], ".."))).toEqual(["🔣️.json"]);
      expect(planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] }).symlinkTargetEdits).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 12_000);

  test("relocates three complete embedded ticket roots with exact many-to-one evidence", () => {
    const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL";
    const files: Record<string, string> = {};
    for (const owner of ["pkg-a", "pkg-b", "pkg-c"]) {
      files[`${owner}/${ticket}/🧪️target-os-errors/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
      files[`${owner}/${ticket}/🧪️unique-${owner.slice(-1)}/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
    }
    const fixture = normalizationFixture("transaction-embedded-v2", files, ({ repoRoot, workspace }) => {
      const canonicalManifest = join(repoRoot, ticket, "🎫️ticket.json");
      mkdirSync(resolve(canonicalManifest, ".."), { recursive: true });
      writeFileSync(canonicalManifest, '{"id":"26/08/17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL"}\n');
      const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
      const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
      const contracts = taxonomy.fixedFilenameContracts as Record<string, unknown>;
      const directoryContracts = taxonomy.fixedDirectoryContracts as Record<string, unknown>;
      const scope = relative(repoRoot, workspace).replaceAll("\\", "/");
      taxonomy.fixedFilenameContracts = Object.fromEntries(Object.entries({ ...contracts, "fixture-cache-tag": { pathPattern: `${scope}/**/CACHEDIR.TAG`, authority: "Transaction disposition golden", reason: "Exact cache marker fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
      taxonomy.fixedDirectoryContracts = Object.fromEntries(Object.entries({ ...directoryContracts, "fixture-package-prefix": { pathPattern: `${scope}/pkg-*`, authority: "Transaction disposition golden", reason: "Exact embedded-root owner fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
    });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.embeddedTicketRoots).toHaveLength(3);
      expect(new Set(plan.embeddedTicketRoots.map((root) => root.ticketId))).toEqual(new Set(["26/08/17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL"]));
      expect(plan.embeddedTicketRootRelocations).toHaveLength(4);
      expect(plan.evidenceRemovals).toHaveLength(2);
      expect(plan.unresolved).toEqual([]);
      const rootTamper = JSON.parse(JSON.stringify(plan)) as TaxonomyPlan;
      (rootTamper.embeddedTicketRoots[0].relocationOperationIds as string[]).reverse();
      (rootTamper as { planDigest: string }).planDigest = taxonomyPlanDigest(rootTamper);
      expect(() => parseTaxonomyPlan(rootTamper)).toThrow();
      const removalTamper = JSON.parse(JSON.stringify(plan)) as TaxonomyPlan;
      (removalTamper.evidenceRemovals[0].authority as { evidenceSetDigest: string }).evidenceSetDigest = "0".repeat(64);
      (removalTamper as { planDigest: string }).planDigest = taxonomyPlanDigest(removalTamper);
      expect(() => parseTaxonomyPlan(removalTamper)).toThrow();
      const before = noFollowTreeDigest(fixture.repoRoot, fixture.scope);
      const cancellationTicket = fixture.ticketDir;
      const cancelFile = join(cancellationTicket, "cancel");
      const cancelled = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: cancellationTicket, expectedBaselineCommit: plan.baselineCommit, cancelFile, progress: (row) => { if (row.phase === "staging-evidence-removals") writeFileSync(cancelFile, "cancel\n"); } });
      expect(cancelled.state).toBe("rolled-back");
      rmSync(cancelFile, { force: true });
      expect(noFollowTreeDigest(fixture.repoRoot, fixture.scope)).toEqual(before);
      const retryTicket = fixture.ticketDir;
      const planArtifact = join(retryTicket, "📊️taxonomy-plan", "🔣️.json");
      mkdirSync(resolve(planArtifact, ".."), { recursive: true });
      writeFileSync(planArtifact, `${canonicalJson(plan)}\n`);
      const copiedConsumer = join(retryTicket, "🧪️consumer", "🔣️.json");
      mkdirSync(resolve(copiedConsumer, ".."), { recursive: true });
      writeFileSync(copiedConsumer, `${JSON.stringify({ path: plan.embeddedTicketRoots[0].sourceMetadataRoot })}\n`);
      expect(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit })).toThrow(/structured incoming reference/u);
      const priorAttempts = ownedFilePaths(retryTicket)
        .filter((path) => (path.startsWith("🧾️taxonomy-transaction/") || path.includes("/🧾️taxonomy-transaction/")) && path.endsWith("/🔣️.json"))
        .map((path) => join(retryTicket, path));
      expect(priorAttempts).toHaveLength(1);
      expect(JSON.parse(readFileSync(priorAttempts[0], "utf8")).state).toBe("rolled-back");
      rmSync(copiedConsumer);
      expect(applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit, injectFailureAt: "after-relocations" }).state).toBe("rolled-back");
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: retryTicket, expectedBaselineCommit: plan.baselineCommit });
      expect(result.appliedEmbeddedTicketRootRelocations).toBe(4);
      expect(result.appliedEvidenceRemovals).toBe(2);
      expect(result.journalPath).toMatch(/🔢️000003\/🔣️\.json$/u);
      const attempts = ownedFilePaths(retryTicket)
        .filter((path) => (path.startsWith("🧾️taxonomy-transaction/") || path.includes("/🧾️taxonomy-transaction/")) && /\/🔂️attempts\/🔢️[^/]+\/🔣️\.json$/u.test(path))
        .map((path) => join(retryTicket, path))
        .sort(ownedPathByteSort);
      expect(attempts).toHaveLength(3);
      expect(attempts.map((path) => JSON.parse(readFileSync(path, "utf8")).state)).toEqual(["rolled-back", "rolled-back", "committed"]);
      const second = planTaxonomy(inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(second.embeddedTicketRoots).toEqual([]);
      expect(second.embeddedTicketRootRelocations).toEqual([]);
      expect(second.evidenceRemovals).toEqual([]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 60_000);
});
//#endregion 🧾️TransactionDispositionsV2

//#region 🧬️DirectMutationOwnership
describe("direct mutation ownership", () => {
  const mutationRoot = (root: string, area = "✏️s") => join(root, area, "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema", "🧬️mutations");

  test("fails closed for missing, opaque, escaped, and symlinked explicit scopes", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-scope/🧫️fixtures/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      acceptedSelections: string[];
      missingSelections: string[];
      rejectedSelections: string[];
    };
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-scope-"));
    try {
      expect(golden.schemaVersion).toBe(1);
      const aggregate = `${golden.mutationRoot}/🦀️.rs`;
      mkdirSync(join(root, golden.mutationRoot), { recursive: true });
      writeFileSync(join(root, aggregate), golden.aggregate);
      const oracleRoots = fastGlob.sync("**/🧬️mutations/🦀️.rs", { cwd: root, onlyFiles: true, followSymbolicLinks: false }).map((path) => path.slice(0, -"/🦀️.rs".length));
      expect(oracleRoots).toEqual([golden.mutationRoot]);
      for (const selection of golden.acceptedSelections) expect(policyMutationStructuralBreaches(root, [selection])).toEqual([]);
      for (const selection of golden.missingSelections) {
        expect(oracleRoots).not.toContain(selection.replaceAll("\\", "/"));
        expect(() => policyMutationStructuralBreaches(root, [selection])).toThrow(/mutation.*scope/iu);
      }
      for (const selection of golden.rejectedSelections) expect(() => policyMutationStructuralBreaches(root, [selection])).toThrow(/mutation.*scope/iu);
      const alias = join(root, "✏️s", "🧪️alias");
      symlinkSync(process.platform === "win32" ? join(root, "✏️s", "🧪️probe") : "🧪️probe", alias, process.platform === "win32" ? "junction" : "dir");
      expect(() => policyMutationStructuralBreaches(root, ["✏️s/🧪️alias/🧬️mutations"])).toThrow(/mutation.*scope/iu);
      rmSync(join(root, aggregate));
      expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).some(({ kind, priority }) => kind === "mutation/reachability" && priority === "high")).toBe(true);
      mkdirSync(join(root, aggregate));
      expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).some(({ kind, priority }) => kind === "mutation/reachability" && priority === "high")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("recognizes exact leaf module identities in codecs without trusting comments or prefixes", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-scope/🧫️fixtures/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      codecCases: { source: string; accepted: boolean }[];
    };
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-codec-identity-"));
    try {
      const leaf = `${golden.mutationRoot}/➕️insert-page`;
      mkdirSync(join(root, leaf, "📝️text"), { recursive: true });
      mkdirSync(join(root, golden.mutationRoot, "📝️text"), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, "🦀️.rs"), "pub enum PageMutation { InsertPage(insert_page::InsertPage) }\n");
      writeFileSync(join(root, leaf, "🦀️.rs"), "pub struct InsertPage; pub struct InsertPagePayload; impl MutationKind for InsertPage {}\n");
      writeFileSync(join(root, leaf, "📝️text", "🦀️.rs"), "pub const OPCODE: &str = \"insert-page\";\n");
      writeFileSync(join(root, leaf, "🔣️.json"), JSON.stringify({ schemaVersion: 1, owner: leaf, semanticKind: "insert-page", displayName: "Insert Page", emoji: "➕️", aggregateVariant: "InsertPage", payloadSchema: "🦀️.rs#InsertPage", textOpcode: "insert-page", binaryTag: null, invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust", "text"] }));
      const codec = join(root, golden.mutationRoot, "📝️text", "🦀️.rs");
      for (const vector of golden.codecCases) {
        writeFileSync(codec, vector.source);
        const oracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_codec_probe", "--edition", "2021", codec], { encoding: "utf8" });
        expect(oracle.status).toBe(0);
        expect(oracle.stdout.includes("ident: insert_page#") || oracle.stdout.includes("ident: InsertPagePayload#") || oracle.stdout.includes('symbol: "insert-page"')).toBe(vector.accepted);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/language-parity").length === 0).toBe(vector.accepted);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("recognizes a binary leaf tag only when its numeric identity equals the descriptor", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-scope/🧫️fixtures/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      binaryCases: { source: string; accepted: boolean }[];
    };
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-binary-identity-"));
    try {
      const leaf = `${golden.mutationRoot}/➕️insert-page`;
      mkdirSync(join(root, leaf, "💾️binary"), { recursive: true });
      mkdirSync(join(root, golden.mutationRoot, "💾️binary"), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, "🦀️.rs"), "pub enum PageMutation { InsertPage(insert_page::InsertPage) }\n");
      writeFileSync(join(root, golden.mutationRoot, "💾️binary", "🦀️.rs"), "pub const REGISTRY: &[u8] = &[insert_page::binary::BINARY_TAG];\n");
      writeFileSync(join(root, leaf, "🦀️.rs"), "pub struct InsertPage; impl MutationKind for InsertPage {}\n");
      writeFileSync(join(root, leaf, "🔣️.json"), JSON.stringify({ schemaVersion: 1, owner: leaf, semanticKind: "insert-page", displayName: "Insert Page", emoji: "➕️", aggregateVariant: "InsertPage", payloadSchema: "🦀️.rs#InsertPage", textOpcode: null, binaryTag: 3, invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust", "binary"] }));
      const codec = join(root, leaf, "💾️binary", "🦀️.rs");
      for (const vector of golden.binaryCases) {
        writeFileSync(codec, vector.source);
        const oracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_binary_probe", "--edition", "2021", codec], { encoding: "utf8" });
        expect(oracle.status).toBe(0);
        expect(oracle.stdout.includes('symbol: "3"')).toBe(vector.accepted);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/language-parity").length === 0).toBe(vector.accepted);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects executable root aggregate codecs while preserving generic framing and unrelated syntax", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-codec-ownership/🧫️fixtures/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      cases: { name: string; source: string; expected: string[] }[];
    };
    const root = mkdtempSync(join(tmpdir(), "semio-mutation-codec-ownership-"));
    try {
      expect(golden.schemaVersion).toBe(1);
      const taxonomy = loadTaxonomy();
      const rustComponent = canonicalPrimaryFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
      const codec = join(root, golden.mutationRoot, "📝️text", rustComponent);
      mkdirSync(dirname(codec), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, rustComponent), golden.aggregate);
      const compilerOutput = join(root, "📦️compiler-output");
      mkdirSync(compilerOutput, { recursive: true });
      const inspect = createRustMutationCodecOwnershipInspector(golden.aggregate);
      for (const vector of golden.cases) {
        writeFileSync(codec, vector.source);
        const source = `${golden.aggregate}\n${vector.source}`;
        const parserOracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_codec_ownership_probe", "--edition", "2021", "-"], { encoding: "utf8", input: source });
        const compilerOracle = spawnSync("rustc", ["--crate-type=lib", "--crate-name", "mutation_codec_ownership_probe", "--edition", "2021", "--out-dir", compilerOutput, "-"], { encoding: "utf8", input: source });
        expect(parserOracle.status).toBe(0);
        expect(compilerOracle.status === 0).toBe(vector.expected.length === 0);
        expect(inspect(vector.source).map(({ kind }) => kind)).toEqual(vector.expected);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/codec-ownership").length > 0).toBe(vector.expected.length > 0);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects empty aggregates, inline variants, and orphaned direct folders", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-scope/🧫️fixtures/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      aggregateCases: { source: string; kind: string; variantCount: number }[];
    };
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-aggregate-"));
    try {
      mkdirSync(join(root, golden.mutationRoot, "➕️insert-page"), { recursive: true });
      const aggregate = join(root, golden.mutationRoot, "🦀️.rs");
      for (const vector of golden.aggregateCases) {
        writeFileSync(aggregate, vector.source);
        const oracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_aggregate_probe", "--edition", "2021", aggregate], { encoding: "utf8" });
        expect(oracle.status).toBe(0);
        expect((oracle.stdout.match(/\bVariant \{/gu) ?? []).length).toBe(vector.variantCount);
        const facts = inspectRustStructure(vector.source);
        expect(facts.enums.flatMap((item) => item.variants)).toHaveLength(vector.variantCount);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).some(({ kind }) => kind === vector.kind)).toBe(true);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects aggregate-state mutation inputs transitively while accepting semantic prior values", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-input-carriers/🧫️fixtures/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      compilerPrelude: string;
      compilerAssertion: string;
      cases: { name: string; source: string; aggregate?: string; forbidden: boolean }[];
    };
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-input-"));
    try {
      expect(golden.schemaVersion).toBe(1);
      const leaf = join(root, golden.mutationRoot, "➕️insert-page");
      mkdirSync(leaf, { recursive: true });
      for (const vector of golden.cases) {
        writeFileSync(join(root, golden.mutationRoot, "🦀️.rs"), vector.aggregate ?? golden.aggregate);
        writeFileSync(join(leaf, "🦀️.rs"), vector.source);
        const oracle = spawnSync("rustc", ["--crate-name", "mutation_input_probe", "--edition", "2021", "--crate-type", "lib", "--emit=metadata", "-o", join(root, "oracle.rmeta"), "-"], { input: golden.compilerPrelude + vector.source + golden.compilerAssertion, encoding: "utf8" });
        expect(oracle.status === 0).toBe(!vector.forbidden);
        if (vector.forbidden) expect(oracle.stderr).toContain("E0277");
        const breaches = policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/no-generic-snapshot-fallback");
        expect({ name: vector.name, forbidden: breaches.length > 0 }).toEqual({ name: vector.name, forbidden: vector.forbidden });
        expect(breaches.every(({ priority }) => priority === "high")).toBe(true);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("reports legacy nesting, inline root behavior, sentinels, and snapshot fallbacks at high severity", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-policy-"));
    try {
      const mutations = mutationRoot(root);
      const leaf = join(mutations, "➕️insert-page");
      mkdirSync(join(leaf, "🦠️mutation"), { recursive: true });
      writeFileSync(join(leaf, "🦠️mutation", "🦀️.rs"), "pub struct InsertPage;\n");
      writeFileSync(join(mutations, "🦀️.rs"), "pub const KINDS: &[&str] = &[\"insert-page\"]; pub enum PdfMutation { NoMutation, SetSnapshot(Vec<u8>), InsertPage(InsertPage) } pub fn apply(m: PdfMutation) { match m { _ => {} } }\n");
      const findings = policyMutationStructuralBreaches(root);
      const kinds = new Set(findings.map(({ kind }) => kind));
      for (const kind of ["mutation/direct-owner", "mutation/root-purity", "mutation/no-sentinel", "mutation/no-generic-snapshot-fallback"]) expect(kinds.has(kind)).toBe(true);
      expect(findings.every(({ priority }) => priority === "high")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("registers every architectural invariant as a high-severity structural policy", () => {
    expect(MUTATION_STRUCTURAL_POLICY_KINDS).toEqual([
      "mutation/direct-owner", "mutation/root-purity", "mutation/folder-variant-bijection", "mutation/descriptor-bijection", "mutation/reachability", "mutation/behavior-ownership", "mutation/codec-ownership", "mutation/wire-identity", "mutation/schema-parity", "mutation/language-parity", "mutation/catalog-parity", "mutation/no-hidden-generation", "mutation/no-sentinel", "mutation/no-generic-snapshot-fallback", "mutation/shared-helper-purity", "mutation/test-presence", "mutation/compose-exclusion",
    ]);
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-policy-catalog-"));
    try {
      const mutations = mutationRoot(root);
      mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation { InsertPage(InsertPage) }\n");
      expect(policyMutationStructuralBreaches(root).every(({ priority }) => priority === "high")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("requires one enabled reachable non-ignored leaf test proven by parsed Rust items", () => {
    const fixturePath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-test-presence/🧫️fixtures/🔣️.json");
    const schemaPath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-test-presence/🛂️schema.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: 1; mutationRoot: string; leaf: string; cases: readonly { name: string; source: string; files: Readonly<Record<string, string>>; links: Readonly<Record<string, string>>; compilerAccepted: boolean; runnableTests: number; policyAccepted: boolean }[] };
    expect(new Ajv({ allErrors: true, strict: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")))(fixture)).toBe(true);
    const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR?.trim();
    const base = artifactDirectory ? resolve(getWorkspaceRoot(), artifactDirectory) : realpathSync(tmpdir());
    const oracleTimeoutMs = 30_000;
    mkdirSync(base, { recursive: true });
    const root = mkdtempSync(join(base, "semio-mutation-test-presence-"));
    try {
      for (const vector of fixture.cases) {
        const caseRoot = join(root, vector.name);
        const mutations = join(caseRoot, fixture.mutationRoot);
        const leaf = join(mutations, fixture.leaf);
        mkdirSync(leaf, { recursive: true });
        writeFileSync(join(leaf, "🦀️.rs"), vector.source);
        for (const [path, source] of Object.entries(vector.files)) {
          const target = resolve(leaf, path);
          mkdirSync(dirname(target), { recursive: true });
          writeFileSync(target, source);
        }
        for (const [path, target] of Object.entries(vector.links)) {
          const link = resolve(leaf, path);
          mkdirSync(dirname(link), { recursive: true });
          symlinkSync(resolve(leaf, target), link, process.platform === "win32" ? "junction" : "dir");
        }
        const aggregate = join(mutations, "🦀️.rs");
        writeFileSync(aggregate, `#[path = "${fixture.leaf}/🦀️.rs"] pub mod insert_page; pub enum ProbeMutation { InsertPage(insert_page::InsertPage) }\n`);
        const executable = join(caseRoot, process.platform === "win32" ? "oracle.exe" : "oracle");
        const compile = spawnSync("rustc", ["--test", "--edition", "2021", "--crate-name", "mutation_test_presence_oracle", aggregate, "-o", executable], { encoding: "utf8", timeout: oracleTimeoutMs });
        writeFileSync(join(caseRoot, "compiler.stdout.log"), compile.stdout ?? "");
        writeFileSync(join(caseRoot, "compiler.stderr.log"), compile.stderr ?? "");
        expect(compile.signal).toBeNull();
        expect(compile.status === 0).toBe(vector.compilerAccepted);
        if (!vector.compilerAccepted) {
          expect(policyMutationStructuralBreaches(caseRoot, [fixture.mutationRoot]).some(({ kind }) => kind === "mutation/test-presence")).toBe(true);
          continue;
        }
        const execution = spawnSync(executable, ["--nocapture"], { encoding: "utf8", timeout: oracleTimeoutMs });
        writeFileSync(join(caseRoot, "runtime.stdout.log"), execution.stdout ?? "");
        writeFileSync(join(caseRoot, "runtime.stderr.log"), execution.stderr ?? "");
        expect(execution.signal).toBeNull();
        expect(execution.status).toBe(0);
        expect(Number(/test result: ok\. (\d+) passed;/u.exec(execution.stdout)?.[1])).toBe(vector.runnableTests);
        const accepted = policyMutationStructuralBreaches(caseRoot, [fixture.mutationRoot]).every(({ kind }) => kind !== "mutation/test-presence");
        expect({ name: vector.name, runnable: vector.runnableTests, accepted }).toEqual({ name: vector.name, runnable: vector.runnableTests, accepted: vector.policyAccepted });
      }
    } finally {
      if (!artifactDirectory) rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("accepts a direct authoritative leaf with optional facets and ignores the same violation under compose", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-policy-"));
    try {
      const mutations = mutationRoot(root);
      const leaf = join(mutations, "➕️insert-page");
      mkdirSync(join(leaf, "🔺️diff"), { recursive: true });
      mkdirSync(join(leaf, "↩️inverse"), { recursive: true });
      mkdirSync(join(leaf, "🧪️tests"), { recursive: true });
      writeFileSync(join(leaf, "🦀️.rs"), "pub struct InsertPage; pub const DESCRIPTOR: MutationDescriptor = descriptor(); impl MutationKind for InsertPage {}\n");
      writeFileSync(join(leaf, "🔣️.json"), `${JSON.stringify({ schemaVersion: 1, owner: relative(root, leaf).replaceAll("\\", "/"), semanticKind: "insert-page", displayName: "Insert Page", emoji: "➕️", aggregateVariant: "InsertPage", payloadSchema: "🦀️.rs#InsertPage", textOpcode: null, binaryTag: null, invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust"] }, null, 2)}\n`);
      writeFileSync(join(leaf, "🔺️diff", "🦀️.rs"), "pub fn diff() {}\n");
      writeFileSync(join(leaf, "↩️inverse", "🦀️.rs"), "pub fn inverse() {}\n");
      writeFileSync(join(leaf, "🧪️tests", "🦀️.rs"), "#[test] fn applies() {}\n");
      writeFileSync(join(mutations, "🦀️.rs"), "pub use super::insert_page::*; #[derive(Mutations)] pub enum PdfMutation { InsertPage(insert_page::InsertPage) }\n");
      const opaque = mutationRoot(root, "compose");
      mkdirSync(join(opaque, "❌️broken", "🦠️mutation"), { recursive: true });
      writeFileSync(join(opaque, "❌️broken", "🦠️mutation", "🦀️.rs"), "pub struct Broken;\n");
      expect(policyMutationStructuralBreaches(root).filter(({ kind }) => ["mutation/direct-owner", "mutation/root-purity", "mutation/descriptor-bijection", "mutation/no-sentinel", "mutation/no-generic-snapshot-fallback"].includes(kind))).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("scaffolds one direct mutation idempotently without overwriting implementation", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-scaffold-"));
    try {
      const mutations = mutationRoot(root);
      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      const first = newScaffoldMutationTree(root, relative(root, mutations).replaceAll("\\", "/"), "➕️insert-page", { text: true, binary: true, jsonSchema: true }, false);
      expect(first.created).toContain(`${relative(root, mutations).replaceAll("\\", "/")}/➕️insert-page/🦀️.rs`);
      const descriptor = JSON.parse(readFileSync(join(mutations, "➕️insert-page", "🔣️.json"), "utf8"));
      expect(descriptor).toMatchObject({ schemaVersion: 1, owner: `${relative(root, mutations).replaceAll("\\", "/")}/➕️insert-page`, semanticKind: "insert-page", payloadSchema: "🧬️schema/🔣️.json", textOpcode: "insert-page", invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"] });
      const descriptorSchema = JSON.parse(readFileSync(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json"), "utf8"));
      const validateDescriptor = new Ajv({ allErrors: true, strict: true }).compile(descriptorSchema);
      expect(validateDescriptor(descriptor)).toBe(true);
      expect(validateJsonSchemaSubset(descriptorSchema, descriptor)).toEqual([]);
      for (const candidate of [
        { ...descriptor, binaryTag: -1 },
        { ...descriptor, requiredLanguageSurfaces: ["typescript"] },
        { ...descriptor, outcomeClasses: [] },
        { ...descriptor, invertibility: "unclassified" },
        { ...descriptor, compatibilityAlias: "legacy" },
      ]) {
        expect(validateJsonSchemaSubset(descriptorSchema, candidate).length === 0).toBe(validateDescriptor(candidate));
      }
      const payloadSchema = JSON.parse(readFileSync(join(mutations, "➕️insert-page", "🧬️schema/🔣️.json"), "utf8"));
      expect(new Ajv({ allErrors: true, strict: true }).compile(payloadSchema)({})).toBe(true);
      writeFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), "pub struct Handwritten;\n");
      const second = newScaffoldMutationTree(root, relative(root, mutations).replaceAll("\\", "/"), "➕️insert-page", { text: true, binary: true, jsonSchema: true }, false);
      expect(second.created).toEqual([]);
      expect(readFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), "utf8")).toBe("pub struct Handwritten;\n");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("prepares AST-safe direct mutation scaffolds before one guarded publication", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-scaffolding/🧫️fixtures/🔣️.json"), "utf8")) as { schemaVersion: number; mutationRoot: string; name: string; attributedAggregate: string; malformedAggregate: string; ambiguousAggregate: string; wrongMountAggregate: string; privateMountAggregate: string; wrongVariantAggregate: string; scopedAggregate: string; unrelatedDocAggregate: string; nestedAggregateDecoy: string; nestedAggregateScopes: string; unmatchedAggregate: string };
    const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-scaffolding/🛂️schema.json"), "utf8"));
    const root = mkdtempSync(join(tmpdir(), "semio-mutation-scaffold-transaction-"));
    const snapshot = (): string => fastGlob.sync("**/*", { cwd: root, onlyFiles: true, followSymbolicLinks: false }).sort().map((path) => `${path}\0${readFileSync(join(root, path), "utf8")}`).join("\0");
    const mutationRoot = join(root, golden.mutationRoot);
    const aggregate = join(mutationRoot, "🦀️.rs");
    const scaffold = (source: string, options: Record<string, unknown> = {}, dryRun = false, name = golden.name) => {
      mkdirSync(mutationRoot, { recursive: true });
      writeFileSync(aggregate, source);
      return newScaffoldMutationTree(root, golden.mutationRoot, name, options as never, dryRun);
    };
    try {
      expect(new Ajv({ allErrors: true, strict: true }).compile(schema)(golden)).toBe(true);
      expect(inspectRustMutationAggregateSpan(golden.attributedAggregate)).toMatchObject({ enumName: "ProbeMutation" });
      expect(inspectRustMutationAggregateSpan(golden.scopedAggregate)).toMatchObject({ declarationStart: 0, enumName: "ProbeMutation" });
      expect(inspectRustMutationAggregateSpan(golden.unrelatedDocAggregate)).toMatchObject({ declarationStart: golden.unrelatedDocAggregate.indexOf("#[derive") });
      expect(inspectRustMutationAggregateSpan(golden.nestedAggregateScopes)).toBeNull();
      for (const source of [golden.malformedAggregate, golden.ambiguousAggregate, golden.nestedAggregateDecoy, golden.nestedAggregateScopes, golden.unmatchedAggregate]) {
        mkdirSync(mutationRoot, { recursive: true });
        writeFileSync(aggregate, source);
        const before = snapshot();
        expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/aggregate/i);
        expect(snapshot()).toBe(before);
      }
      for (const source of [golden.wrongMountAggregate, golden.privateMountAggregate, golden.wrongVariantAggregate]) {
        writeFileSync(aggregate, source);
        const before = snapshot();
        expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/existing (mount|variant)/i);
        expect(snapshot()).toBe(before);
      }
      writeFileSync(aggregate, golden.attributedAggregate);
      const beforeDryRun = snapshot();
      const preview = newScaffoldMutationTree(root, golden.mutationRoot, golden.name, {}, true);
      expect(preview.created).toContain(`${golden.mutationRoot}/${golden.name}/🦀️.rs`);
      expect(snapshot()).toBe(beforeDryRun);
      mkdirSync(join(mutationRoot, golden.name, "🦀️.rs"), { recursive: true });
      const beforeBlockedTarget = snapshot();
      expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/target is not a regular file/i);
      expect(snapshot()).toBe(beforeBlockedTarget);
      rmSync(join(mutationRoot, golden.name), { recursive: true, force: true });
      newScaffoldMutationTree(root, golden.mutationRoot, golden.name);
      const mounted = readFileSync(aggregate, "utf8");
      expect(mounted.indexOf("pub mod insert_page;")).toBeLessThan(mounted.indexOf("#[derive(Clone, Debug)]"));
      expect(mounted).toContain("InsertPage(insert_page::Mutation)");
      const parser = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_scaffold_probe", "--edition", "2021", "-"], { encoding: "utf8", input: mounted });
      expect(parser.status).toBe(0);
      writeFileSync(join(mutationRoot, golden.name, "🦀️.rs"), "pub struct Handwritten;\n");
      newScaffoldMutationTree(root, golden.mutationRoot, golden.name, {}, false);
      expect(readFileSync(join(mutationRoot, golden.name, "🦀️.rs"), "utf8")).toBe("pub struct Handwritten;\n");
      const unrelated = join(root, "unrelated.txt");
      expect(() => scaffold(golden.attributedAggregate, { cancelled: () => { writeFileSync(unrelated, "kept\n"); return true; } }, false, "➖️remove-page")).toThrow(/cancel/i);
      expect(readFileSync(unrelated, "utf8")).toBe("kept\n");
      expect(existsSync(join(mutationRoot, "➖️remove-page"))).toBe(false);
      writeFileSync(aggregate, golden.attributedAggregate);
      const beforeFailure = snapshot();
      let checks = 0;
      expect(() => newScaffoldMutationTree(root, golden.mutationRoot, "✏️edit-page", { cancelled: () => {
        checks += 1;
        if (checks === 5) writeFileSync(aggregate, "pub enum ConcurrentMutation {}\n");
        return false;
      } }, false)).toThrow(/changed during publication/i);
      expect(readFileSync(aggregate, "utf8")).toBe("pub enum ConcurrentMutation {}\n");
      expect(existsSync(join(mutationRoot, "✏️edit-page"))).toBe(false);
      expect(snapshot()).not.toBe(beforeFailure);
      expect(snapshot()).toContain("🦀️.rs\u0000pub enum ConcurrentMutation {}\n");
      writeFileSync(aggregate, golden.attributedAggregate);
      rmSync(join(mutationRoot, golden.name), { recursive: true, force: true });
      symlinkSync(join(root, "missing-leaf"), join(mutationRoot, golden.name), process.platform === "win32" ? "junction" : "file");
      const beforeDanglingLink = snapshot();
      expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/symlink/i);
      expect(snapshot()).toBe(beforeDanglingLink);
      rmSync(join(mutationRoot, golden.name));
      const linkedRoot = `${root}-linked`;
      symlinkSync(root, linkedRoot, process.platform === "win32" ? "junction" : "dir");
      expect(() => newScaffoldMutationTree(linkedRoot, golden.mutationRoot, golden.name)).toThrow(/repository root.*regular directory/i);
      rmSync(linkedRoot);
      expect(() => newScaffoldMutationTree(root, "../escape/🧬️mutations", golden.name)).toThrow(/scope/i);
      expect(() => newScaffoldMutationTree(root, `compose/${golden.mutationRoot}`, golden.name)).toThrow(/scope.*excluded/i);
      expect(() => newScaffoldMutationTree(root, "✏️s/🧪️scaffold/not-a-mutation-owner", golden.name)).toThrow(/scope/i);
      const linked = join(root, "✏️s", "🧪️linked");
      mkdirSync(dirname(linked), { recursive: true });
      symlinkSync(mutationRoot, linked, process.platform === "win32" ? "junction" : "dir");
      expect(() => newScaffoldMutationTree(root, "✏️s/🧪️linked/🧬️mutations", golden.name)).toThrow(/scope/i);
    } finally {
      try { chmodSync(mutationRoot, 0o755); } catch {}
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("inventories direct and legacy records as stable language-neutral JSON validated by Ajv", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-direct-mutation-inventory-"));
    try {
      const mutations = mutationRoot(root);
      mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
      mkdirSync(join(mutations, "➖️remove-page", "🦠️mutation"), { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation { InsertPage(InsertPage), RemovePage(RemovePage) }\n");
      writeFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), 'pub struct InsertPage; impl MutationKind for InsertPage { const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "InsertedPage" }; }\n');
      writeFileSync(join(mutations, "➖️remove-page", "🦠️mutation", "🦀️.rs"), "pub struct RemovePage;\n");
      const inventory = inventoryMutationTaxonomy(root);
      expect(inventory.records.map(({ state }) => state).sort()).toEqual(["direct", "legacy"]);
      const validate = new Ajv({ strict: true }).compile({ type: "object", required: ["schemaVersion", "kind", "sourceTreeDigest", "roots", "sourceRoster", "records", "unresolved", "violations"], properties: { schemaVersion: { const: 2 }, kind: { const: "mutation" }, sourceTreeDigest: { type: "string", pattern: "^[a-f0-9]{64}$" }, roots: { type: "array" }, sourceRoster: { type: "array" }, records: { type: "array" }, unresolved: { type: "array" }, violations: { type: "array" } }, additionalProperties: false });
      expect(validate(JSON.parse(JSON.stringify(inventory)))).toBe(true);
      const plan = planMutationTaxonomy(inventory, "d03b1fdb6da7c4ea97043e5618d8f4098a43dff7");
      expect(plan.moves).toEqual([{ source: `${relative(root, mutations).replaceAll("\\", "/")}/➖️remove-page/🦠️mutation/🦀️.rs`, destination: `${relative(root, mutations).replaceAll("\\", "/")}/➖️remove-page/🦀️.rs` }]);
      expect(plan.unresolved).toEqual(expect.arrayContaining([expect.objectContaining({ path: `${relative(root, mutations).replaceAll("\\", "/")}/➖️remove-page` })]));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("resolves mutation consumers and schema-validated assignment evidence from a stable source index", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-mutation-inventory-consumers-"));
    const fixturePath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️consumers.json");
    const inventorySchemaPath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-inventory/🛂️schema/🔣️inventory.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: number; assignmentLedger: unknown; files: readonly { path: string; content: string }[] };
    const fixtureSchema = { type: "object", required: ["schemaVersion", "assignmentLedger", "files"], properties: { schemaVersion: { const: 1 }, assignmentLedger: { type: "object" }, files: { type: "array", minItems: 2, items: { type: "object", required: ["path", "content"], properties: { path: { type: "string" }, content: { type: "string" } }, additionalProperties: false } } }, additionalProperties: false };
    const taxonomy = loadTaxonomy();
    const rustFilename = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
    const typescriptFilename = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🟦️typescript"]!, taxonomy);
    const jsonFilename = canonicalPrimaryFilenameForKind("json", taxonomy);
    const taxonomyPath = (path: string) => path.replaceAll("🦀️.rs", rustFilename).replaceAll("🟦️.ts", typescriptFilename).replaceAll("🔣️.json", jsonFilename);
    try {
      expect(new Ajv({ strict: true }).compile(fixtureSchema)(fixture)).toBe(true);
      for (const { path, content } of fixture.files) {
        const target = join(root, taxonomyPath(path));
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, content.replaceAll("🦀️.rs", rustFilename).replaceAll("🟦️.ts", typescriptFilename).replaceAll("🔣️.json", jsonFilename));
      }
      const inventory = inventoryMutationTaxonomy(root, { assignmentLedger: fixture.assignmentLedger, assignmentLedgerPath: "ticket/📋️mutation-assignments.json" });
      const inventorySchema = JSON.parse(readFileSync(inventorySchemaPath, "utf8"));
      expect(new Ajv({ strict: true }).compile(inventorySchema)(JSON.parse(JSON.stringify(inventory)))).toBe(true);
      const sourceRoster = fastGlob.sync("**/*", { cwd: root, onlyFiles: true, followSymbolicLinks: false, dot: true }).filter((path) => !path.startsWith("compose/")).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
      expect(inventory.sourceRoster.filter(({ role }) => role === "source").map(({ path }) => path)).toEqual(sourceRoster);
      expect(inventory.sourceRoster.some(({ path }) => path.startsWith("compose/"))).toBe(false);
      const alpha = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "➕️insert-page")!;
      const beta = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "✏️change-page")!;
      const remove = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "➖️remove-page")!;
      expect(alpha.structuralState).toBe("direct");
      expect(alpha.executionState).toBe("assigned");
      expect(beta.executionState).toBe("conflicting");
      expect(remove.executionState).toBe("unassigned");
      expect(alpha.assignmentEvidence).toMatchObject({ status: "resolved", ledgerPath: null, rows: [{ terraExecutor: "TERRA-ALPHA-INSERT" }] });
      expect(beta.assignmentEvidence).toMatchObject({ status: "conflicting" });
      expect(remove.assignmentEvidence).toMatchObject({ status: "missing" });
      expect(alpha.consumerEdges).toEqual(expect.arrayContaining([
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🎮️command/🦀️.rs"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), kind: "command", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/✏️editor/🟦️.ts"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🟦️.ts"), kind: "editor", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/👁️viewer/🟦️.ts"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🟦️.ts"), kind: "viewer", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📚️catalog/🦀️.rs"), kind: "catalog", relation: "reexport" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📋️registry/🦀️.rs"), kind: "registry", relation: "reexport" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️operations/🦀️.rs"), kind: "sibling-operation" }),
        expect.objectContaining({ sourcePath: "✏️s/🔌️plugins/🅰️alpha/🧪️tests/two-file/command.rs", targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), kind: "test", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📦️crate/command.rs"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), kind: "leaf", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), targetPath: taxonomyPath("🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page/🦀️.rs"), kind: "cross-owner", relation: "import" }),
      ]));
      expect(alpha.consumerEdges.some(({ sourcePath }) => sourcePath.endsWith("🧪️identity-controls.rs"))).toBe(false);
      expect(alpha.schemaAndLanguageSurfaces).toEqual([taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🟦️.ts")]);
      expect(beta.schemaAndLanguageSurfaces).toEqual([taxonomyPath("🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page/🔣️.json")]);
      expect(alpha.sharedHelpers).toEqual(["✏️s/🔌️plugins/🅰️alpha/🧰️helpers/🦀️page.rs"]);
      expect(alpha.crossOwnerDependencies).toEqual([taxonomyPath("🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page/🦀️.rs")]);
      expect(beta.consumerEdges).toEqual(expect.arrayContaining([expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📦️crate/🎮️command/🦀️.rs"), kind: "command", relation: "import" })]));
      expect(inventory.unresolved).toEqual(expect.arrayContaining([
        expect.objectContaining({ path: "✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➖️remove-page", reason: expect.stringMatching(/assignment/i) }),
        expect.objectContaining({ path: "🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page", reason: expect.stringMatching(/conflicting/i) }),
      ]));
      const digest = inventory.sourceTreeDigest;
      writeFileSync(join(root, taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🎮️command/🦀️.rs")), `#[path = "../🧬️mutations/➕️insert-page/${rustFilename}"] mod insert_page;\nuse insert_page::Mutation;\n// byte-only external consumer edit\n`);
      expect(inventoryMutationTaxonomy(root, { assignmentLedger: fixture.assignmentLedger, assignmentLedgerPath: "ticket/📋️mutation-assignments.json" }).sourceTreeDigest).not.toBe(digest);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("proves direct leaf reachability through exact public canonical mounts and wrapped types", () => {
    const fixturePath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-reachability/🧫️fixtures/🔣️cases.json");
    const schemaPath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-reachability/🛂️schema/🔣️cases.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { cases: readonly { name: string; source: string; leafSource?: string; extraFiles?: readonly { path: string; source: string }[]; accepted: boolean }[] };
    expect(new Ajv({ strict: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")))(fixture)).toBe(true);
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (artifactRoot) mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot ?? tmpdir(), "semio-mutation-reachability-"));
    const taxonomy = loadTaxonomy();
    const rust = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
    try {
      for (const vector of fixture.cases) {
        const caseRoot = join(root, vector.name);
        const mutations = mutationRoot(caseRoot);
        mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
        writeFileSync(join(mutations, "➕️insert-page", rust), (vector.leafSource ?? "pub struct Mutation;\n").replaceAll("🦀️.rs", rust));
        for (const file of vector.extraFiles ?? []) { const target = join(mutations, "➕️insert-page", file.path.replaceAll("🦀️.rs", rust)); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, file.source); }
        writeFileSync(join(mutations, rust), vector.source.replaceAll("🦀️.rs", rust));
        if (["public-canonical", "semantic-type-alias", "child-facet-reexport"].includes(vector.name)) {
          const out = join(caseRoot, "🧪️compiler-artifacts");
          mkdirSync(out, { recursive: true });
          const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", "reachability_probe", "--crate-type", "lib", "--out-dir", out, join(mutations, rust)], { encoding: "utf8", timeout: 30_000 });
          writeFileSync(join(out, "compiler.log"), `${compiled.stdout}\n${compiled.stderr}`);
          if (compiled.status !== 0) throw new Error(`${vector.name}: ${compiled.stderr || compiled.error}`);
        }
        const relativeRoot = relative(caseRoot, mutations).replaceAll("\\", "/");
        const reaches = policyMutationStructuralBreaches(caseRoot, [relativeRoot]).some(({ kind, scope }) => (kind === "mutation/reachability" && scope === `${relativeRoot}/➕️insert-page/${rust}`) || kind === "mutation/folder-variant-bijection");
        expect(reaches).toBe(!vector.accepted);
      }
    } finally { if (!artifactRoot) rmSync(root, { recursive: true, force: true }); }
  });

  test("projects the actual wrapped mutation declaration origin through public aliases only", () => {
    const fixturePath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-type-origin/🔣️vectors.json");
    const schemaPath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-type-origin/🛂️schema.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: 1; mutationRoot: string; leaf: string; rustFilename: "🦀️.rs"; cases: readonly { id: string; mutationRoot?: string; leaf?: string; rustFilename?: string; virtualFilesystem?: true; repoRoot?: string; repositoryRootSymlink?: true; repositoryAncestorSymlink?: true; rootSource: string; leafSource: string; extraFiles?: readonly { path: string; source: string }[]; links?: readonly { path: string; target: string }[]; compileAccepted: boolean; expected: { sourcePath: string; declarationName: string; modulePath: string[] } | null }[] };
    expect(new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")))(fixture)).toBe(true);
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (artifactRoot) mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot ?? tmpdir(), "semio-mutation-type-origin-"));
    try {
      for (const vector of fixture.cases) {
        const rust = vector.rustFilename ?? fixture.rustFilename, mutationRoot = vector.mutationRoot ?? fixture.mutationRoot, leafName = vector.leaf ?? fixture.leaf, caseDirectory = join(root, vector.id);
        const sourceRoot = vector.repositoryRootSymlink ? join(caseDirectory, "source") : vector.repositoryAncestorSymlink ? join(caseDirectory, "source", "workspace") : vector.repoRoot ?? caseDirectory;
        const caseRoot = vector.repositoryRootSymlink ? join(caseDirectory, "repo") : vector.repositoryAncestorSymlink ? join(caseDirectory, "alias", "workspace") : sourceRoot;
        const mutations = join(sourceRoot, mutationRoot), leaf = join(mutations, leafName);
        if (!vector.virtualFilesystem) {
          mkdirSync(leaf, { recursive: true });
          writeFileSync(join(mutations, rust), vector.rootSource.replaceAll("🦀️.rs", rust));
          writeFileSync(join(leaf, rust), vector.leafSource.replaceAll("🦀️.rs", rust));
          for (const file of vector.extraFiles ?? []) {
            const target = join(leaf, file.path.replaceAll("🦀️.rs", rust));
            mkdirSync(dirname(target), { recursive: true });
            writeFileSync(target, file.source.replaceAll("🦀️.rs", rust));
          }
          for (const link of vector.links ?? []) {
            const path = join(leaf, link.path.replaceAll("🦀️.rs", rust));
            mkdirSync(dirname(path), { recursive: true });
            symlinkSync(process.platform === "win32" ? join(dirname(path), link.target.replaceAll("🦀️.rs", rust)) : link.target.replaceAll("🦀️.rs", rust), path, "file");
          }
          if (vector.repositoryRootSymlink) symlinkSync(process.platform === "win32" ? sourceRoot : "source", caseRoot, process.platform === "win32" ? "junction" : "dir");
          if (vector.repositoryAncestorSymlink) symlinkSync(process.platform === "win32" ? join(caseDirectory, "source") : "source", join(caseDirectory, "alias"), process.platform === "win32" ? "junction" : "dir");
        }
        const graph = inspectRustModuleGraphFacts(vector.rootSource.replaceAll("🦀️.rs", rust));
        expect(graph.modules.find((module) => module.name === "insert_page")?.conditional === true, vector.id).toBe(["conditional-mount", "inner-conditional-mount", "cfg-attr-conditional-mount", "root-inner-cfg"].includes(vector.id));
        const structure = inspectRustStructure(vector.rootSource.replaceAll("🦀️.rs", rust)), aggregate = structure.enums.filter((item) => item.name === "ProbeMutation"), variant = aggregate.flatMap((item) => item.variants).filter((item) => item.name === "InsertPage");
        expect(aggregate.some((item) => item.conditional === true), vector.id).toBe(["inner-conditional-mount", "disabled-aggregate-cfg", "conditional-aggregate-cfg-attr", "root-inner-cfg", "conditional-ancestor-inline-enum"].includes(vector.id));
        expect(variant.some((item) => item.conditional === true), vector.id).toBe(["inner-conditional-mount", "disabled-aggregate-cfg", "conditional-aggregate-cfg-attr", "root-inner-cfg", "disabled-variant-cfg", "conditional-variant-cfg-attr", "conditional-ancestor-inline-enum"].includes(vector.id));
        if (vector.compileAccepted) {
          const out = join(caseRoot, "🧪️rustc");
          mkdirSync(out, { recursive: true });
          const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", "wrapped_type_origin_probe", "--crate-type", "lib", "--out-dir", out, join(mutations, rust)], { encoding: "utf8", timeout: 30_000 });
          writeFileSync(join(out, "📓️compiler.md"), `# Rustc ${vector.id}\n\n\`\`\`text\n${compiled.stdout}${compiled.stderr}\n\`\`\`\n`);
          expect(compiled.status, vector.id).toBe(0);
        }
        const relativeMutations = mutationRoot;
        const proof = inspectMutationRootReachability(caseRoot, relativeMutations, vector.rootSource.replaceAll("🦀️.rs", rust), [leafName], rust);
        expect(proof).toHaveLength(1);
        expect(proof[0]!.origin, vector.id).toEqual(vector.expected === null ? null : { ...vector.expected, sourcePath: vector.expected.sourcePath.replaceAll("🦀️.rs", rust) });
        expect(proof[0]!.wrapped, vector.id).toBe(vector.expected !== null);
      }
    } finally { if (!artifactRoot) rmSync(root, { recursive: true, force: true }); }
  });

  test("requires a fresh clean terminal verification before mutation apply can commit", () => {
    const goldenPath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️.json");
    const golden = JSON.parse(readFileSync(goldenPath, "utf8")) as { schemaVersion: number; baseline: string; cases: readonly string[] };
    const validateGolden = new Ajv({ strict: true }).compile({ type: "object", required: ["schemaVersion", "baseline", "cases"], properties: { schemaVersion: { const: 1 }, baseline: { type: "string", pattern: "^[a-f0-9]{40}$" }, cases: { type: "array", items: { type: "string" }, minItems: 10 } }, additionalProperties: false });
    expect(validateGolden(golden)).toBe(true);
    expect(golden.cases).toEqual(["direct-violation-unresolved", "source-byte-change-invalidates-plan", "baseline-mismatch-rejects", "cancellation-never-commits", "current-violation-zero-move-rejects", "fresh-clean-terminal-verification-commits", "virtual-compose-excluded", "symlink-source-rejected", "mid-inventory-source-change-retries", "empty-root-invalidates-plan"]);
    const root = mkdtempSync(join(tmpdir(), "semio-mutation-terminality-"));
    const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️08", "☀️12", "TERMINALITY");
    const planPath = join(ticket, "plan.json");
    const applyPath = join(ticket, "📊️taxonomy-apply", "🔣️.json");
    const options = (plan: string, cancelFile?: string) => ({ baseline: golden.baseline, cancelFile, failOnWarning: false, format: "json" as const, kind: "mutation" as const, plan });
    const writePlan = (plan: unknown) => {
      mkdirSync(dirname(planPath), { recursive: true });
      writeFileSync(planPath, `${canonicalJson(plan)}\n`);
    };
    try {
      const directViolation = {
        schemaVersion: 1 as const,
        kind: "mutation" as const,
        sourceTreeDigest: "a".repeat(64),
        roots: ["✏️s/🧪️probe/🧬️mutations"],
        records: [{ mutationRootPath: "✏️s/🧪️probe/🧬️mutations", targetMutationDirectoryName: "➕️insert-page", state: "direct" as const, violationClasses: ["mutation/behavior-ownership"], evidence: [] }],
        violations: [{ id: "behavior", kind: "mutation/behavior-ownership", scope: "✏️s/🧪️probe/🧬️mutations/➕️insert-page", summary: "behavior remains at root", priority: "high" as const, reason: "direct leaves own behavior", solution: "move behavior" }],
      };
      expect(planMutationTaxonomy(directViolation as any, golden.baseline).unresolved).toEqual(expect.arrayContaining([expect.objectContaining({ path: "✏️s/🧪️probe/🧬️mutations/➕️insert-page" })]));

      const mutations = mutationRoot(root);
      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      const sourceRoster = fastGlob.sync("**/🧬️mutations/🦀️.rs", { cwd: root, onlyFiles: true, followSymbolicLinks: false });
      expect(sourceRoster).toEqual([`${relative(root, mutations).replaceAll("\\", "/")}/🦀️.rs`]);
      const binarySource = join(mutations, "🔢source.bin");
      writeFileSync(binarySource, Buffer.from([0, 1, 0]));
      const nulDigest = inventoryMutationTaxonomy(root).sourceTreeDigest;
      writeFileSync(binarySource, Buffer.from([0, 2, 0]));
      expect(inventoryMutationTaxonomy(root).sourceTreeDigest).not.toBe(nulDigest);
      const staleInventory = inventoryMutationTaxonomy(root);
      const stalePlan = planMutationTaxonomy(staleInventory, golden.baseline);
      writePlan(stalePlan);
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n// source-byte-only change\n");
      expect(inventoryMutationTaxonomy(root).sourceTreeDigest).not.toBe(staleInventory.sourceTreeDigest);
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath), ticket)).toThrow(/inventory digest/u);
      expect(existsSync(applyPath)).toBe(false);
      let mutateDuringInventory = true;
      const stableInventory = inventoryMutationTaxonomy(root, { afterFactsCollected: () => {
        if (!mutateDuringInventory) return;
        mutateDuringInventory = false;
        writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n// source-byte-only change during facts\n");
      } });
      expect(stableInventory.sourceTreeDigest).toBe(inventoryMutationTaxonomy(root).sourceTreeDigest);

      rmSync(mutations, { recursive: true, force: true });
      const opaqueMutations = join(root, "compose", "🧬️mutations");
      mkdirSync(opaqueMutations, { recursive: true });
      writeFileSync(join(opaqueMutations, "🦀️.rs"), "pub enum OpaqueMutation {}\n");
      expect(inventoryMutationTaxonomy(root).roots).toEqual([]);
      rmSync(join(root, "compose"), { recursive: true, force: true });

      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      symlinkSync(process.platform === "win32" ? mutations : "🦀️.rs", join(mutations, process.platform === "win32" ? "🧬️alias" : "🦀️alias.rs"), process.platform === "win32" ? "junction" : "file");
      expect(() => inventoryMutationTaxonomy(root)).toThrow(/symlink/u);
      rmSync(mutations, { recursive: true, force: true });

      const cleanInventory = inventoryMutationTaxonomy(root);
      const cleanPlan = planMutationTaxonomy(cleanInventory, golden.baseline);
      writePlan(cleanPlan);
      expect(() => runMutationTaxonomyCli(root, "apply", { ...options(planPath), baseline: "b".repeat(40) }, ticket)).toThrow(/baseline/u);
      expect(existsSync(applyPath)).toBe(false);
      mkdirSync(mutations, { recursive: true });
      expect(inventoryMutationTaxonomy(root).sourceTreeDigest).not.toBe(cleanInventory.sourceTreeDigest);
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath), ticket)).toThrow(/inventory digest/u);
      expect(existsSync(applyPath)).toBe(false);
      rmSync(mutations, { recursive: true, force: true });

      const cancelFile = join(ticket, "cancel");
      writeFileSync(cancelFile, "cancel\n");
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath, cancelFile), ticket)).toThrow(/cancelled/u);
      expect(existsSync(applyPath)).toBe(false);
      rmSync(cancelFile);

      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      mkdirSync(join(mutations, "➕️insert-page", "🦠️mutation"), { recursive: true });
      writeFileSync(join(mutations, "➕️insert-page", "🦠️mutation", "🦀️.rs"), "pub struct InsertPage;\n");
      const violatingInventory = inventoryMutationTaxonomy(root);
      const unsigned = { schemaVersion: 1 as const, kind: "mutation" as const, baselineCommit: golden.baseline, inventoryDigest: violatingInventory.sourceTreeDigest, moves: [], unresolved: [] };
      writePlan({ ...unsigned, planDigest: createHash("sha256").update(canonicalJson(unsigned)).digest("hex") });
      expect(() => runMutationTaxonomyCli(root, "apply", options(planPath), ticket)).toThrow(/terminal verification/u);
      expect(existsSync(applyPath)).toBe(false);

      rmSync(mutations, { recursive: true, force: true });
      const terminalInventory = inventoryMutationTaxonomy(root);
      writePlan(planMutationTaxonomy(terminalInventory, golden.baseline));
      runMutationTaxonomyCli(root, "apply", options(planPath), ticket);
      expect(JSON.parse(readFileSync(applyPath, "utf8"))).toMatchObject({ schemaVersion: 1, kind: "mutation", state: "committed", inventoryDigest: terminalInventory.sourceTreeDigest, terminalVerification: { clean: true, violations: [] } });
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧬️DirectMutationOwnership

//#region 🧬️MutationMetadataFacts
describe("mutation metadata facts", () => {
  test("records exact declaration and alias syntax without decoy inference", () => {
    const fixturePath = join(import.meta.dir, "../../🧪️tests/🧬️mutation-metadata/🧫️fixtures/🔣️cases.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
    const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️tests/🧬️mutation-metadata/🛂️schema/🔣️cases.json"), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const facts = inspectRustMutationMetadataFacts(fixture.source);
    expect(facts.declarations.map(({ name, kind, visibility, modulePath, derives, mutationLeaf }) => ({ name, kind, visibility, modulePath, derives, state: mutationLeaf.state, contracts: mutationLeaf.contracts, detail: mutationLeaf.detail }))).toEqual(fixture.expected.declarations);
    expect(facts.crateAliases).toEqual(fixture.expected.aliases);
    expect(facts.manualMutationLeafImpls).toEqual(fixture.expected.manualMutationLeafImpls);
    expect(facts.declarations.some((item) => item.name === "Fake")).toBe(false);
  });
});
//#endregion 🧬️MutationMetadataFacts
