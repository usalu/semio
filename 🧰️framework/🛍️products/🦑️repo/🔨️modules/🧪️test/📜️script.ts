#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧪️ Router of the repository testing domain:
//   bun ./📜️script.ts <discover|contract|oracle|subject|parity|run|report|clean|dependency|nx|doctor> [args…]

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { basename, delimiter, join, relative, sep } from "node:path";
import { type BreachRecord, type TestLevel, Script, ScriptRouter, TEST_LEVELS, formatBreachReport, getRepoMetaDir, resolveTestLevel, runBundleScriptMain, runProbe, testLevelBudgetMs } from "../📚️library/📦️packages/🟦️typescript/📦️index.ts";
import {
  type ClassifiedDependency,
  type CoverageMetrics,
  type ImplementationCoverage,
  type ComparisonProfile,
  type DependencyEcosystem,
  type DiscoveredCase,
  type Implementation,
  type OracleEntry,
  type OracleHostPackage,
  type TestResult,
  type TestRole,
  agentCacheRoot,
  buildCasePlan,
  classifyLegacyKind,
  cleanTestOutputs,
  computeCoverageMetrics,
  dependencyEcosystemOfRegistryValue,
  digest,
  discoverTestCases,
  dotnetPackageReferences,
  evaluateCrossSubjectParity,
  evaluateParity,
  enforceMetricGates,
  externalOracleHostPackages,
  formatMetrics,
  formatCleanReport,
  isProductionClass,
  isExcludedTestPath,
  loadOracleRegistry,
  oracleHostModule,
  oracleHostPackagesFor,
  oracleLinkedPackages,
  profileTable,
  markOutputDir,
  type ComparisonPipeline,
  type CoverageRow,
  type FixtureManifest,
  type MutationManifest,
  type ProbeEntry,
  type RetentionClass,
  type RuntimeMutationInventory,
  buildCoverageMatrix,
  collectGarbage,
  compareInventories,
  contentDigestOf,
  engineFamilyId,
  enforceReleaseGates,
  fixtureManifestProblems,
  formatCoverageQuestions,
  formatGcReport,
  installFixtureFile,
  isQualifiedProbe,
  isWildcardSubset,
  measureCoverage,
  owningSubsetOf,
  publishFixtureManifest,
  readRuntimeInventory,
  subsetCoordinate,
  subsetCoordinatesOfOwner,
  leafDescriptorCoverage,
  manifestFromLeafDescriptors,
  scaffoldOwnerDescriptors,
  derivePayloadSchemas,
  testFilenameForKind,
  isQualifyingOracleKind,
  verifyFixture,
  writeRuntimeInventory,
  markRunComplete,
  planExecution,
  pythonRuntimeImports,
  ratchetDependencies,
  scanDeclaredDependencies,
  readResults,
  renderDiff,
  renderJUnit,
  summarizeRun,
  testCacheDir,
  testTaxonomy,
  validateAllContracts,
} from "./📦️packages/🟦️typescript/📦️index.ts";
//#endregion 🔌️Adapters

//#region 🗂️Paths
const DOMAIN_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";
const TS_PACKAGE_REL = `${DOMAIN_REL}/📦️packages/🟦️typescript`;
const RUST_PACKAGE_REL = `${DOMAIN_REL}/📦️packages/🦀️rust`;
const GO_PACKAGE_REL = `${DOMAIN_REL}/📦️packages/🐹️go`;
const PYTHON_PACKAGE_REL = `${DOMAIN_REL}/📦️packages/🐍️python`;
const DOTNET_PACKAGE_REL = `${DOMAIN_REL}/📦️packages/🔷️dotnet`;

/** 📊️ Where the latest run's reports land — a marked directory, so `clean test` can remove them. */
function reportsDir(repoRoot: string): string {
  const dir = join(testCacheDir(repoRoot, "reports"), "latest");
  markOutputDir(repoRoot, dir, { testId: "run::latest", cacheKey: "reports" });
  return dir;
}
//#endregion 🗂️Paths

//#region 📈️Metrics
/** 🔒️ Loads and phase-classifies the committed dependency baseline, adding any registry oracle it lacks. */
function loadClassifiedBaseline(repoRoot: string): ClassifiedDependency[] {
  const baselineRaw = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { ecosystem: string; name: string; version: string; kinds: string[]; users: string[]; productionReachable?: boolean; oracleIds?: string[]; capabilities?: string[] }[] };
  const registry = loadOracleRegistry(repoRoot);
  // 🔒️Keyed by EVERY package an oracle links and holding EVERY oracle that links it. Keying on the
  // primary package alone missed the composed halves; keeping only one oracle per package made the
  // committed record depend on manifest discovery order, so the same repository could produce two
  // different baselines. Ids are sorted, so the file is a function of the registry and nothing else.
  const oraclesByLinkedPackage = new Map<string, OracleEntry[]>();
  for (const oracle of registry.oracles) for (const linked of oracleLinkedPackages(oracle)) oraclesByLinkedPackage.set(linked.package, [...(oraclesByLinkedPackage.get(linked.package) ?? []), oracle]);
  const classified: ClassifiedDependency[] = baselineRaw.entries.map((entry) => {
    const linking = oraclesByLinkedPackage.get(entry.name) ?? [];
    const oracleIds = linking.length > 0 ? [...new Set(linking.map((oracle) => oracle.id))].sort() : (entry.oracleIds ?? []);
    const kinds = [...new Set(entry.kinds.map((kind) => (["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"].includes(kind) ? (kind as ClassifiedDependency["kinds"][number]) : classifyLegacyKind(kind, oracleIds))))];
    return {
      ecosystem: entry.ecosystem as DependencyEcosystem,
      name: entry.name,
      version: entry.version,
      kinds,
      users: entry.users,
      productionReachable: entry.productionReachable ?? kinds.some(isProductionClass),
      oracleIds: oracleIds.length > 0 ? oracleIds : undefined,
      capabilities: linking.length > 0 ? [...new Set(linking.flatMap((oracle) => oracle.capabilities))].sort() : entry.capabilities,
    };
  });
  // 🧩️EVERY package a registered oracle links, not only the one its id is named after. A composed
  // reference (reader + writer, archive + XML) links several, and a secondary package that never
  // reached this list would be linked into the host while the ratchet and the report both showed it
  // as absent — the exact blind spot registration exists to close.
  for (const oracle of registry.oracles) {
    for (const linked of oracleLinkedPackages(oracle)) {
      if (classified.some((entry) => entry.name === linked.package)) continue;
      const linking = oraclesByLinkedPackage.get(linked.package) ?? [oracle];
      classified.push({ ecosystem: dependencyEcosystemOfRegistryValue(oracle.ecosystem), name: linked.package, version: linked.version, kinds: ["test-oracle"], users: [oracle.hostPath ?? DOMAIN_REL], productionReachable: false, oracleIds: [...new Set(linking.map((entry) => entry.id))].sort(), capabilities: [...new Set(linking.flatMap((entry) => entry.capabilities))].sort() });
    }
  }
  // 🧩️An owner that puts an external distribution on a generated host's import path has added a
  // third-party test dependency, exactly as registering an oracle package does. Classifying it here
  // is what keeps the Python and npm hosts inside the same ratchet as the Rust one instead of
  // letting a manifest field become an unwatched channel for reference libraries.
  for (const host of externalOracleHostPackages(registry)) {
    const existing = classified.find((entry) => entry.ecosystem === host.ecosystem && entry.name === host.name);
    if (existing !== undefined) {
      classified[classified.indexOf(existing)] = { ...existing, users: [...new Set([...existing.users, ...host.users])] };
      continue;
    }
    classified.push({ ecosystem: host.ecosystem, name: host.name, version: host.version, kinds: ["test-oracle"], users: host.users, productionReachable: false });
  }
  return classified.sort((a, b) => a.ecosystem.localeCompare(b.ecosystem) || a.name.localeCompare(b.name));
}

/** 📈️ Per-implementation source coverage, read from each language's own report directory. An
 * implementation that reported nothing is ABSENT rather than 100% — a blended repository percentage
 * must never be able to stand in for a language that produced no coverage at all. */
function readImplementationCoverage(repoRoot: string): ImplementationCoverage[] {
  const roots: [string, string][] = [
    ["rust", "rust"],
    ["typescript", "js"],
    ["go", "go"],
    ["python", "py"],
    ["dotnet", "dotnet"],
  ];
  const out: ImplementationCoverage[] = [];
  for (const [implementation, dirName] of roots) {
    const dir = join(getRepoMetaDir(repoRoot), "📊️metrics", "coverage", dirName);
    if (!existsSync(dir)) continue;
    let linesFound = 0;
    let linesHit = 0;
    let branchesFound = 0;
    let branchesHit = 0;
    let sawBranches = false;
    const walk = (current: string): void => {
      for (const entry of readdirSync(current, { withFileTypes: true })) {
        const full = join(current, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (!/\.(lcov|info|cover)$/.test(entry.name)) continue;
        for (const line of readFileSync(full, "utf8").split(/\r?\n/)) {
          if (line.startsWith("LF:")) linesFound += Number(line.slice(3)) || 0;
          else if (line.startsWith("LH:")) linesHit += Number(line.slice(3)) || 0;
          else if (line.startsWith("BRF:")) {
            branchesFound += Number(line.slice(4)) || 0;
            sawBranches = true;
          } else if (line.startsWith("BRH:")) branchesHit += Number(line.slice(4)) || 0;
        }
      }
    };
    walk(dir);
    if (linesFound === 0) continue;
    out.push({
      implementation,
      lines: { covered: linesHit, total: linesFound, ratio: linesHit / linesFound },
      branches: sawBranches && branchesFound > 0 ? { covered: branchesHit, total: branchesFound, ratio: branchesHit / branchesFound } : null,
    });
  }
  return out;
}
//#endregion 📈️Metrics

//#region 🎛️Selection
/** 🎛️ Narrows discovery to the cases a command was pointed at (`--case`, `--owner`, `--project`). */
function selectCases(repoRoot: string, segments: readonly string[]): DiscoveredCase[] {
  const all = discoverTestCases(repoRoot);
  const value = (flag: string): string | null => {
    const index = segments.indexOf(flag);
    return index === -1 ? null : (segments[index + 1] ?? null);
  };
  const owner = value("--owner");
  const caseSlug = value("--case");
  const project = value("--project");
  // 🎛️`--owner` matches the exact owner path, a trailing segment of it, OR any ancestor segment, so
  // `--owner 🗄️stdio` selects every artifact owned beneath that plugin rather than nothing.
  const matchesOwner = (entry: DiscoveredCase): boolean => owner === null || entry.owner === owner || entry.owner.endsWith(`/${owner}`) || entry.owner.split("/").includes(owner) || entry.owner.includes(`${owner}/`);
  return all.filter((entry) => matchesOwner(entry) && (caseSlug === null || entry.case === caseSlug) && (project === null || entry.projectName === project));
}

/** 🎛️ Implementations a command should exercise: every claimed adapter unless `--implementation` narrows it. */
function selectImplementations(discovered: DiscoveredCase, segments: readonly string[]): Implementation[] {
  const index = segments.indexOf("--implementation");
  const requested = index === -1 ? null : segments[index + 1];
  const claimed = Object.keys(discovered.adapters) as Implementation[];
  return requested ? claimed.filter((impl) => impl === requested) : claimed;
}

/**
 * 🎛️ The full v2 selector set. Every phase accepts every selector, so a CI shard is expressible at
 * the exact coordinate a report row is keyed by — never merely at artifact level.
 */
type Selectors = Readonly<{
  artifact: string | null;
  standard: string | null;
  subset: string | null;
  mutation: string | null;
  outcome: string | null;
  case: string | null;
  fixtureClass: string | null;
  fixtureFamily: string | null;
  oracle: string | null;
  probe: string | null;
  implementation: string | null;
  platform: string | null;
  agent: string | null;
  run: string | null;
  status: string | null;
}>;

function readSelectors(segments: readonly string[]): Selectors {
  const value = (flag: string): string | null => {
    const index = segments.indexOf(flag);
    return index === -1 ? null : (segments[index + 1] ?? null);
  };
  return {
    artifact: value("--artifact"),
    standard: value("--standard"),
    subset: value("--subset"),
    mutation: value("--mutation"),
    outcome: value("--outcome"),
    case: value("--case"),
    fixtureClass: value("--fixture-class"),
    fixtureFamily: value("--fixture-family"),
    oracle: value("--oracle"),
    probe: value("--probe"),
    implementation: value("--implementation"),
    platform: value("--platform"),
    agent: value("--agent"),
    run: value("--run"),
    status: value("--status"),
  };
}

function matchesTarget(manifest: MutationManifest, selectors: Selectors): boolean {
  return (
    (selectors.artifact === null || manifest.artifact === selectors.artifact || manifest.artifact.endsWith(`.${selectors.artifact}`)) &&
    (selectors.standard === null || manifest.standard === selectors.standard) &&
    (selectors.subset === null || manifest.subset === selectors.subset || manifest.mutations.some((mutation) => mutation.subset === selectors.subset)) &&
    (selectors.mutation === null || manifest.mutations.some((mutation) => mutation.id === selectors.mutation))
  );
}

function matchesFixture(fixture: FixtureManifest, selectors: Selectors): boolean {
  return (
    (selectors.artifact === null || fixture.target.artifact === selectors.artifact || fixture.target.artifact.endsWith(`.${selectors.artifact}`)) &&
    (selectors.standard === null || fixture.target.standard === selectors.standard) &&
    (selectors.subset === null || fixture.target.subset === selectors.subset) &&
    (selectors.mutation === null || fixture.mutation === selectors.mutation) &&
    (selectors.outcome === null || fixture.outcome === selectors.outcome) &&
    (selectors.fixtureClass === null || fixture.class === selectors.fixtureClass) &&
    (selectors.fixtureFamily === null || fixture.family === selectors.fixtureFamily)
  );
}

function matchesRow(row: CoverageRow, selectors: Selectors): boolean {
  return (
    (selectors.artifact === null || row.artifact === selectors.artifact || row.artifact.endsWith(`.${selectors.artifact}`)) &&
    (selectors.standard === null || row.standard === selectors.standard) &&
    (selectors.subset === null || row.subset === selectors.subset) &&
    (selectors.mutation === null || row.mutation === selectors.mutation) &&
    (selectors.outcome === null || row.outcome === selectors.outcome) &&
    (selectors.oracle === null || row.oracle === selectors.oracle) &&
    (selectors.implementation === null || row.implementation === selectors.implementation) &&
    (selectors.platform === null || row.platform === selectors.platform) &&
    (selectors.fixtureClass === null || row.fixtureClass === selectors.fixtureClass) &&
    (selectors.status === null || row.status === selectors.status)
  );
}

/**
 * 🏭️ The language-neutral production mutation bridge an owner exposes. It is a plain executable
 * beside the owner that answers `listMutations(artifact, standard, subset)` on stdout. Keeping it a
 * process rather than a linked entry point is what lets one gate cover Rust, TypeScript and every
 * other implementation without the framework knowing which language an artifact is written in.
 */
const MUTATION_BRIDGE_REL = "🏭️bridge/📜️script.ts";

function mutationBridgeFor(repoRoot: string, owner: string, manifest: MutationManifest): { command: string; args: string[] } | null {
  // 🧭️The bridge is looked up at the owner and then at each ancestor, so a subset inherits the one its
  // artifact publishes instead of every subset needing its own copy.
  let candidate = owner;
  for (;;) {
    const abs = join(repoRoot, candidate, MUTATION_BRIDGE_REL);
    if (existsSync(abs)) return { command: "bun", args: [abs, "list-mutations", manifest.artifact, manifest.standard, manifest.subset] };
    const parent = candidate.split("/").slice(0, -1).join("/");
    if (parent === "" || parent === candidate) return null;
    candidate = parent;
  }
}
//#endregion 🎛️Selection

//#region 🏗️Hosts
/** 🏗️ One materialized native entrypoint: where it lives and how it is launched. */
type MaterializedHost = Readonly<{ command: string; args: readonly string[]; cwd: string; env: NodeJS.ProcessEnv; hostDir: string | null; problems: readonly string[] }>;

function hostDirFor(repoRoot: string, discovered: DiscoveredCase, role: TestRole, implementation: Implementation): string {
  const dir = join(testCacheDir(repoRoot, "hosts"), `${discovered.projectName}-${role}-${implementation}`);
  // 🧾️ A generated host is deletable state, so it carries the same ownership marker as every other
  // output root — an unmarked directory is never removed by `clean test`.
  markOutputDir(repoRoot, dir, { testId: `${discovered.owner}::${discovered.case}`, cacheKey: `${role}:${implementation}` });
  markRunComplete(dir);
  return dir;
}

/**
 * 🔬️ Whether this repository actually SHIPS an implementation of the case's owner in
 * `implementation`'s language — the owner root, or the nearest ancestor of it, carrying a package
 * directory for that language.
 *
 * The subject role means "this repository's own implementation, on the same inputs". An adapter file
 * in a language the owner ships no package in exists to HOST a reference library (that is what every
 * `🐍️component.py` in this repository is), so there is no subject there to dispatch — and asking for
 * one produces an `adapter has no subject registration` error for every scenario of the case, which
 * then enters the parity ratio as a null projection and drags a fully-passing case to zero. That is
 * not evidence of anything; it is the coordinator asking a question the taxonomy already answers.
 *
 * This is deliberately NOT "skip an implementation whose adapter registers no subject handlers":
 * an owner that ships a package in a language and whose adapter then forgets a subject registration
 * must still fail, loudly, per scenario. The language dir names come from the taxonomy's own
 * `testImplementationIds`, so no language is named here.
 */
function ownerShipsImplementation(repoRoot: string, discovered: DiscoveredCase, implementation: Implementation): boolean {
  const languageDir = Object.entries(testTaxonomy(repoRoot).testImplementationIds).find(([, id]) => id === implementation)?.[0];
  if (languageDir === undefined) return false;
  let dir = discovered.owner;
  for (let depth = 0; depth < 16; depth += 1) {
    if (existsSync(join(repoRoot, dir, "📦️packages", languageDir))) return true;
    const parent = dir.split("/").slice(0, -1).join("/");
    if (parent === "" || parent === dir) break;
    dir = parent;
  }
  return false;
}

/** 🦀️ The owner's own Rust package, discovered by walking up from the owner root — the subject under test. */
function rustSutCrate(repoRoot: string, discovered: DiscoveredCase): { name: string; path: string } | null {
  let dir = discovered.owner;
  for (let depth = 0; depth < 16; depth += 1) {
    const manifest = join(repoRoot, dir, "📦️packages", "🦀️rust", "Cargo.toml");
    if (existsSync(manifest)) {
      const name = readFileSync(manifest, "utf8").match(/^\s*name\s*=\s*"([^"]+)"/m)?.[1];
      // 🧭️ The host crate is already a dependency of every generated host; a case owned by the
      // testing domain itself must not declare it twice.
      if (name !== undefined && name !== "semio-repo-test-host") return { name, path: join(repoRoot, dir, "📦️packages", "🦀️rust") };
      if (name === "semio-repo-test-host") return null;
    }
    const parent = dir.split("/").slice(0, -1).join("/");
    if (parent === "" || parent === dir) break;
    dir = parent;
  }
  return null;
}

/**
 * 🧩️ The native oracle packages this case's OWNER contributes, resolved from the discovered
 * contribution manifests. The framework links whatever an owner declares; it never names a package,
 * a plugin or a format, so a new artifact family needs no edit here.
 */
function contributedOraclePackages(repoRoot: string, discovered: DiscoveredCase, implementation: Implementation): OracleHostPackage[] {
  return oracleHostPackagesFor(loadOracleRegistry(repoRoot), discovered.owner, implementation);
}

/** 🦀️ Materializes a standalone cache-local integration crate that links the adapter and the host support crate by path. */
function materializeRustHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "rust");
  const adapterAbs = join(repoRoot, discovered.adapters.rust!);
  const sut = rustSutCrate(repoRoot, discovered);
  const declared = contributedOraclePackages(repoRoot, discovered, "rust");
  // 🦀️A Cargo dependency is linked by path or it is not linked at all; a crates.io coordinate would
  // be an unreviewed third-party dependency of the generated host, which is what the local-crate
  // rule exists to prevent.
  const oraclePackages = declared.filter((entry) => entry.path !== undefined);
  const problems = declared.filter((entry) => entry.path === undefined).map((entry) => `${discovered.caseDir}: rust oracle host package ${entry.package} declares no path — a Rust host links contributed crates by path`);
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(
    join(dir, "Cargo.toml"),
    [
      "# 🤖️ Generated by `bun ./📜️script.ts <phase>` — safe to delete, never commit.",
      "[workspace]",
      "",
      "[package]",
      `name = "semio-test-host-${discovered.case.replace(/[^a-z0-9]+/g, "-")}"`,
      'version = "0.0.0"',
      'edition = "2021"',
      "",
      "[[bin]]",
      'name = "host"',
      'path = "src/main.rs"',
      "",
      // 🔮️The subject crate is OPTIONAL and reached only through the `sut` feature. §5.3 of the
      // frozen plan requires the oracle-only test to pass WITHOUT invoking the local implementation,
      // so the oracle role must not link — or even compile — the subject. An adapter therefore gates
      // its subject half with `#[cfg(feature = "sut")]`, and this crate turns that feature on for
      // the subject role only.
      ...(sut === null ? [] : ["[features]", `sut = ["dep:${sut.name}"]`, ""]),
      "[dependencies]",
      `semio-repo-test-host = { path = ${JSON.stringify(join(repoRoot, RUST_PACKAGE_REL))} }`,
      // 🧩️Whatever the owner contributed, exactly as the owner declared it.
      ...oraclePackages.map((entry) => `${entry.package} = { path = ${JSON.stringify(join(repoRoot, entry.path!))}${(entry.features ?? []).length > 0 ? `, features = [${(entry.features ?? []).map((feature) => JSON.stringify(feature)).join(", ")}]` : ""} }`),
      ...(sut === null ? [] : [`${sut.name} = { path = ${JSON.stringify(sut.path)}, default-features = false, optional = true }`]),
      "",
    ].join("\n"),
  );
  writeFileSync(
    join(dir, "src", "main.rs"),
    [
      "// 🤖️ Generated native entrypoint. The adapter below is the committed, taxonomy-named source.",
      `#[path = ${JSON.stringify(adapterAbs)}]`,
      "mod adapter;",
      "",
      "fn main() -> std::process::ExitCode {",
      "    semio_repo_test_host::run_main(adapter::adapter())",
      "}",
      "",
    ].join("\n"),
  );
  return {
    command: "cargo",
    args: ["run", "--quiet", "--manifest-path", join(dir, "Cargo.toml"), ...(sut !== null && role === "subject" ? ["--features", "sut"] : []), "--", "--plan", planPath, "--out", outPath],
    cwd: repoRoot,
    env: { ...process.env, CARGO_TARGET_DIR: join(agentCacheRoot(repoRoot), "cargo-test-hosts") },
    hostDir: dir,
    problems,
  };
}

/** 🐹️ Materializes a cache-local Go module whose generated entrypoint delegates to the committed adapter. */
function materializeGoHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "go");
  const adapterAbs = join(repoRoot, discovered.adapters.go!);
  writeFileSync(join(dir, "go.mod"), ["// 🤖️ Generated — safe to delete, never commit.", "module semio.test/host", "", "go 1.23", "", "require semio.tech/repo/test v0.0.0", "", `replace semio.tech/repo/test => ${join(repoRoot, GO_PACKAGE_REL)}`, ""].join("\n"));
  writeFileSync(join(dir, "adapter.go"), readFileSync(adapterAbs, "utf8").replace(/^package\s+\w+/m, "package main"));
  writeFileSync(join(dir, "main.go"), ["// 🤖️ Generated native entrypoint.", "package main", "", 'import host "semio.tech/repo/test"', "", "func main() {", "\thost.RunMain(Adapter())", "}", ""].join("\n"));
  return { command: "go", args: ["run", ".", "--plan", planPath, "--out", outPath], cwd: dir, env: { ...process.env, GOFLAGS: "-mod=mod", GOWORK: "off" }, hostDir: dir, problems: [] };
}

/**
 * 🐍️ The cache-local interpreter the Python host runs under, carrying exactly the external
 * distributions the owners declared.
 *
 * A virtual environment, never the system interpreter: a test host may not mutate the machine it
 * runs on. It is created with `--system-site-packages` so a distribution the machine already
 * provides is REUSED rather than downloaded, which is what keeps a zero-touch checkout working
 * offline; anything still missing is installed INTO the environment, where it stays isolated. The
 * environment is keyed by the declared package set, so it is built once and reused by every run and
 * every case that declares the same set, and rebuilt the moment the declaration changes.
 */
function provisionPythonInterpreter(repoRoot: string, base: string, declared: readonly OracleHostPackage[]): { interpreter: string; problems: string[] } {
  const external = declared.filter((entry) => entry.path === undefined);
  if (external.length === 0) return { interpreter: base, problems: [] };
  const specs = [...external].map((entry) => ({ spec: entry.version === undefined ? entry.package : `${entry.package}==${entry.version}`, module: oracleHostModule(entry), package: entry.package, version: entry.version })).sort((a, b) => a.spec.localeCompare(b.spec));
  const signature = specs.map((entry) => entry.spec).join(" ");
  const dir = join(testCacheDir(repoRoot, "hosts"), `python-env-${digest(`${base}\n${signature}`)}`);
  const interpreter = join(dir, process.platform === "win32" ? "Scripts" : "bin", process.platform === "win32" ? "python.exe" : "python3");
  const stampPath = join(dir, "🧾️packages.json");
  const stamp = existsSync(stampPath) ? (JSON.parse(readFileSync(stampPath, "utf8")) as { signature?: string }) : null;
  if (stamp?.signature === signature && existsSync(interpreter)) return { interpreter, problems: [] };

  markOutputDir(repoRoot, dir, { testId: "hosts::python-env", cacheKey: `python-env:${signature}` });
  const problems: string[] = [];
  if (!existsSync(interpreter)) {
    const created = runProbe(base, ["-m", "venv", "--system-site-packages", dir], { cwd: repoRoot, budgetMs: testLevelBudgetMs("long") });
    if ((created.status ?? 1) !== 0 || !existsSync(interpreter)) {
      problems.push(`python oracle host: cannot create the cache-local environment at ${relative(repoRoot, dir).split(sep).join("/")} with \`${base} -m venv\` — ${created.stderr.trim() || `exit ${created.status}`}`);
      return { interpreter: base, problems };
    }
  }
  // 🔎️Importable AND at the declared version. Checking only importability would let a declared pin
  // be silently satisfied by whatever the machine happened to have, which is the same as not
  // declaring one.
  const present = (entry: { spec: string; module: string; package: string; version?: string }): boolean => {
    const probe = runProbe(interpreter, ["-c", `import ${entry.module}, importlib.metadata as meta; print(meta.version(${JSON.stringify(entry.package)}))`], { cwd: repoRoot, budgetMs: testLevelBudgetMs("quick") });
    return (probe.status ?? 1) === 0 && (entry.version === undefined || probe.stdout.trim() === entry.version);
  };
  for (const entry of specs) {
    if (present(entry)) continue;
    const installed = runProbe(interpreter, ["-m", "pip", "install", "--disable-pip-version-check", entry.spec], { cwd: repoRoot, budgetMs: testLevelBudgetMs("exhaustive") });
    if ((installed.status ?? 1) !== 0) {
      problems.push(`python oracle host: ${entry.spec} is neither importable nor installable into ${relative(repoRoot, dir).split(sep).join("/")} — ${installed.stderr.trim().split("\n").slice(-3).join(" ") || `pip exited ${installed.status}`}`);
      continue;
    }
    if (!present(entry)) problems.push(`python oracle host: ${entry.spec} installed but \`import ${entry.module}\` at that version still fails — declare the import name with "module" if it differs from the distribution name`);
  }
  if (problems.length === 0) {
    writeFileSync(stampPath, `${JSON.stringify({ interpreter: base, signature, packages: specs }, null, 2)}\n`);
    markRunComplete(dir);
  }
  return { interpreter, problems };
}

/** 🐍️ Runs the committed adapter through the owned Python host — never through the compose-scoped root discovery config. */
function materializePythonHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "python");
  const declared = contributedOraclePackages(repoRoot, discovered, "python");
  const { interpreter, problems } = provisionPythonInterpreter(repoRoot, process.env.SEMIO_PYTHON ?? "python3", declared);
  // 🧩️A contributed package that DOES carry a path is in-repo source, reached the way Python reaches
  // any source tree: on the import path, never installed.
  const localPaths = declared.filter((entry) => entry.path !== undefined).map((entry) => join(repoRoot, entry.path!));
  return {
    command: interpreter,
    args: [join(repoRoot, PYTHON_PACKAGE_REL, "🐍️host.py"), "--plan", planPath, "--out", outPath, "--adapter", join(repoRoot, discovered.adapters.python!)],
    cwd: repoRoot,
    env: {
      ...process.env,
      PYTHONDONTWRITEBYTECODE: "1",
      PYTHONPYCACHEPREFIX: join(agentCacheRoot(repoRoot), "pycache"),
      ...(localPaths.length > 0 ? { PYTHONPATH: [...localPaths, process.env.PYTHONPATH ?? ""].filter((value) => value !== "").join(delimiter) } : {}),
    },
    hostDir: dir,
    problems,
  };
}

/**
 * 🟦️ Runs the committed adapter through the owned TypeScript host. Nothing is generated: bun resolves
 * a bare specifier by walking up from the repository root, so a declared npm package is RESOLVED
 * from the checkout's existing `node_modules` rather than installed into a private tree — one
 * install, one lockfile, one version of every library in the repository. What this does add is the
 * check that the declaration is true: an unresolvable package is reported here instead of surfacing
 * as an adapter import error with no mention of the manifest that promised it.
 */
function materializeTypescriptHost(repoRoot: string, discovered: DiscoveredCase, planPath: string, outPath: string): MaterializedHost {
  const problems = contributedOraclePackages(repoRoot, discovered, "typescript")
    .filter((entry) => entry.path === undefined && !resolvesFromRepoRoot(repoRoot, entry.package))
    .map((entry) => `${discovered.caseDir}: declared typescript oracle package ${entry.package} does not resolve from the repository's node_modules — add it to the root manifest and install it`);
  return { command: "bun", args: [join(repoRoot, TS_PACKAGE_REL, "🏃️host.ts"), "--plan", planPath, "--out", outPath, "--adapter", join(repoRoot, discovered.adapters.typescript!)], cwd: repoRoot, env: process.env, hostDir: null, problems };
}

/** 🟦️ Whether a bare specifier resolves from the repository root — the same lookup the host will do. */
function resolvesFromRepoRoot(repoRoot: string, specifier: string): boolean {
  try {
    createRequire(join(repoRoot, "package.json")).resolve(specifier);
    return true;
  } catch {
    // 🧭️A package whose manifest declares no resolvable entry point still counts as present; the
    // adapter may be reaching a subpath export the root resolver alone cannot answer for.
    return existsSync(join(repoRoot, "node_modules", ...specifier.split("/"), "package.json"));
  }
}

/** 🔷️ Materializes a cache-local .NET test project that links the committed adapter and the host support project. */
function materializeDotnetHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "dotnet");
  const adapterAbs = join(repoRoot, discovered.adapters.dotnet!);
  writeFileSync(
    join(dir, "host.csproj"),
    [
      "<!-- 🤖️ Generated — safe to delete, never commit. -->",
      '<Project Sdk="Microsoft.NET.Sdk">',
      "  <PropertyGroup>",
      "    <OutputType>Exe</OutputType>",
      "    <TargetFramework>net8.0</TargetFramework>",
      "    <Nullable>enable</Nullable>",
      "    <ImplicitUsings>enable</ImplicitUsings>",
      "    <LangVersion>latest</LangVersion>",
      "    <EnableDefaultCompileItems>false</EnableDefaultCompileItems>",
      "    <AssemblyName>host</AssemblyName>",
      "    <RootNamespace>Semio.Repo.Test.Host</RootNamespace>",
      "  </PropertyGroup>",
      "  <ItemGroup>",
      `    <Compile Include="${adapterAbs}" />`,
      '    <Compile Include="Program.cs" />',
      `    <ProjectReference Include="${join(repoRoot, DOTNET_PACKAGE_REL, "Semio.Repo.Test.csproj")}" />`,
      "  </ItemGroup>",
      "</Project>",
      "",
    ].join("\n"),
  );
  writeFileSync(join(dir, "Program.cs"), ["// 🤖️ Generated native entrypoint.", "using Semio.Repo.Test;", "", "internal static class GeneratedHost", "{", "    private static int Main(string[] args) => TestHost.RunMain(Adapter.Create(), args);", "}", ""].join("\n"));
  return {
    command: "dotnet",
    args: ["run", "--project", join(dir, "host.csproj"), "--", "--plan", planPath, "--out", outPath],
    cwd: dir,
    env: { ...process.env, DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1" },
    hostDir: dir,
    problems: [],
  };
}

/** 🏗️ Resolves the launch recipe for one implementation, materializing a cache-local entrypoint when the native framework needs one. */
function materializeHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, implementation: Implementation, planPath: string, outPath: string): MaterializedHost {
  switch (implementation) {
    case "typescript":
      return materializeTypescriptHost(repoRoot, discovered, planPath, outPath);
    case "rust":
      return materializeRustHost(repoRoot, discovered, role, planPath, outPath);
    case "go":
      return materializeGoHost(repoRoot, discovered, role, planPath, outPath);
    case "python":
      return materializePythonHost(repoRoot, discovered, role, planPath, outPath);
    case "dotnet":
      return materializeDotnetHost(repoRoot, discovered, role, planPath, outPath);
  }
}
//#endregion 🏗️Hosts

//#region 🏃️Run
type PhaseOutcome = Readonly<{ results: TestResult[]; problems: string[] }>;

/** 🏃️ Executes one `(case, level, role, implementation)` triple and reads back its owned result stream. */
function executeOne(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, role: TestRole, implementation: Implementation, subjectRawInputs?: Readonly<Partial<Record<Implementation, string>>>): PhaseOutcome {
  const planned = planExecution(repoRoot, discovered, level, role, implementation);
  const plan = subjectRawInputs === undefined ? planned.plan : { ...planned.plan, subjectRawInputs };
  const { missingFixtures, planPath } = planned;
  if (subjectRawInputs !== undefined) writeFileSync(planPath, `${JSON.stringify(plan, null, 2)}\n`);
  const problems = missingFixtures.map((uri) => `${discovered.caseDir}: unresolved fixture ${uri}`);
  if (plan.scenarios.length === 0) return { results: [], problems };
  rmSync(plan.resultsPath, { force: true });
  const host = materializeHost(repoRoot, discovered, role, implementation, planPath, plan.resultsPath);
  // 🧩️A host that could not be provisioned has not run, and an unprovisioned host must never look
  // like a case with nothing to do — the declaration is reported before anything is executed.
  if (host.problems.length > 0) return { results: [], problems: [...problems, ...host.problems] };
  const probe = runProbe(host.command, [...host.args], { cwd: host.cwd, env: host.env, budgetMs: testLevelBudgetMs(level) });
  if (probe.stdout.trim() !== "") console.log(probe.stdout.trimEnd());
  const { results, problems: readProblems } = readResults(plan.resultsPath);
  if ((probe.status ?? 1) !== 0 && results.length === 0) {
    problems.push(`${discovered.caseDir}: ${implementation} ${role} host exited ${probe.status} without emitting results`);
    if (probe.stderr.trim() !== "") problems.push(probe.stderr.trimEnd());
  }
  // 🏁️Both generated directories are marked, not just the results one. An unmarked work directory
  // reads as permanently interrupted, and `clean test --stale` would then delete the work directory
  // of a run that is still executing in a parallel session.
  markRunComplete(plan.workDir);
  markRunComplete(plan.outputDir);
  return { results, problems: [...problems, ...readProblems.map((problem) => `${discovered.caseDir}: ${problem}`)] };
}

/** 🎭️ The case's oracle decision: which implementation serves the oracle role, or the recorded no-oracle decision. */
function oracleDecision(repoRoot: string, discovered: DiscoveredCase, level: TestLevel): { implementation: Implementation | null; noOracleDecision: string | null; comparison: ComparisonProfile; problem: string | null } {
  const registry = loadOracleRegistry(repoRoot);
  const { plan } = buildCasePlan(repoRoot, discovered, level);
  if (plan.oracle === null) return { implementation: null, noOracleDecision: plan.noOracleDecision, comparison: plan.comparison, problem: plan.noOracleDecision === null ? `${discovered.caseDir}: feature declares neither an oracle nor a no-oracle decision` : null };
  const entry = registry.oracles.find((candidate) => candidate.id === plan.oracle);
  if (entry === undefined) return { implementation: null, noOracleDecision: null, comparison: plan.comparison, problem: `${discovered.caseDir}: unknown oracle id ${plan.oracle}` };
  const mapped = (entry.ecosystem === "javascript" ? "typescript" : entry.ecosystem) as Implementation;
  if ((discovered.adapters as Record<string, string | undefined>)[mapped] === undefined) return { implementation: null, noOracleDecision: null, comparison: plan.comparison, problem: `${discovered.caseDir}: oracle ${entry.id} needs a ${mapped} adapter to run in` };
  return { implementation: mapped, noOracleDecision: null, comparison: plan.comparison, problem: null };
}

/** 🎯️ The declared execution mode of one planned scenario — what a no-oracle substitute must match. */
function planModeOf(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, scenarioId: string): string {
  return buildCasePlan(repoRoot, discovered, level).plan.scenarios.find((scenario) => scenario.id === scenarioId)?.mode ?? "differential";
}

/** 🏃️ Runs the requested phases for every selected case and writes the run report. */
function runPhases(repoRoot: string, segments: readonly string[], phases: readonly ("oracle" | "subject")[]): number {
  const { level, rest } = resolveTestLevel([...segments]);
  const cases = selectCases(repoRoot, rest);
  const allResults: TestResult[] = [];
  const problems: string[] = [];
  const parity: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number }[] = [];
  // ⚖️One effective profile table for the whole run: the framework's domain-neutral profiles plus
  // every profile the discovered owners contribute.
  const profiles = profileTable(loadOracleRegistry(repoRoot));
  let scenarioCount = 0;

  for (const discovered of cases) {
    const decision = oracleDecision(repoRoot, discovered, level);
    if (decision.problem !== null) problems.push(decision.problem);
    const caseResults: TestResult[] = [];
    const rawInputOracle = buildCasePlan(repoRoot, discovered, level).plan.oracleInput === "subject-raw";

    const runSubjects = (): void => {
      if (!phases.includes("subject") && !rawInputOracle) return;
      // 🔬️Only the languages this repository actually implements the owner in are dispatched as
      // subjects; see `ownerShipsImplementation`.
      const subjects = selectImplementations(discovered, rest).filter((candidate) => ownerShipsImplementation(repoRoot, discovered, candidate));
      // 🚫️A case every one of whose adapters is a reference HOST has no subject half at all. That is
      // a real gap and it must stay visible — reported the same way `not-exercised` is, rather than
      // as a per-scenario `errored` result whose null projection would enter the parity ratio and
      // read as a subject that ran and disagreed.
      if (subjects.length === 0 && selectImplementations(discovered, rest).length > 0) console.error(`[test] no-subject-implementation ${discovered.caseDir} (adapters ${Object.keys(discovered.adapters).join(", ")} host references only; this repository ships no implementation of the owner in any of those languages)`);
      for (const implementation of subjects) {
        const outcome = executeOne(repoRoot, discovered, level, "subject", implementation);
        caseResults.push(...outcome.results);
        problems.push(...outcome.problems);
      }
    };
    const subjectRawInputs = (): Readonly<Partial<Record<Implementation, string>>> => Object.fromEntries(
      caseResults
        .filter((result) => result.role === "subject" && result.status === "passed" && result.output.rawPath !== undefined)
        .map((result) => [result.implementation, result.output.rawPath!] as const),
    );
    if (rawInputOracle) runSubjects();
    if (phases.includes("oracle") && decision.implementation !== null) {
      const outcome = executeOne(repoRoot, discovered, level, "oracle", decision.implementation, rawInputOracle ? subjectRawInputs() : undefined);
      caseResults.push(...outcome.results);
      problems.push(...outcome.problems);
    }
    if (!rawInputOracle) runSubjects();
    scenarioCount += new Set(caseResults.map((result) => result.scenario)).size;
    allResults.push(...caseResults);

    if (!phases.includes("oracle") || !phases.includes("subject") || caseResults.length === 0) continue;

    const diffDir = testCacheDir(repoRoot, "diffs");
    mkdirSync(diffDir, { recursive: true });
    if (decision.implementation !== null) {
      const { verdicts, unmatched } = evaluateParity(decision.comparison, caseResults, profiles);
      for (const verdict of verdicts) {
        parity.push({ testId: verdict.testId, profile: verdict.profile, equal: verdict.equal, diffs: verdict.diffs });
        if (verdict.equal) continue;
        writeFileSync(join(diffDir, `${verdict.testId.replace(/[^A-Za-z0-9]+/g, "_")}.diff.txt`), renderDiff(verdict.testId, verdict.verdict));
        problems.push(`parity failed: ${verdict.testId} (${verdict.diffs} differences)`);
      }
      for (const orphan of unmatched) problems.push(`no oracle result to compare against: ${orphan}`);
    }

    // 🧮️ Pairwise subject equivalence keeps two implementations from exploiting different oracle
    // ambiguities, and is the ONLY parity evidence a recorded no-oracle case can offer — so it must
    // then involve at least two independently written implementations to mean anything at all.
    const crossPairs = evaluateCrossSubjectParity(decision.comparison, caseResults, profiles);
    for (const pair of crossPairs) {
      parity.push({ testId: pair.pair, profile: decision.comparison, equal: pair.equal, diffs: pair.diffs });
      if (!pair.equal) problems.push(`cross-subject parity failed: ${pair.pair} (${pair.diffs} differences)`);
    }
    // 🧭️A recorded no-oracle decision names the substitutes it relies on, and only the
    // `independent-implementations` substitute is discharged by pairwise parity. A decision resting
    // on specification vectors or metamorphic laws discharges itself inside the scenarios, so
    // demanding a second implementation there would be a rule the decision never claimed to meet.
    if (decision.implementation === null && decision.noOracleDecision !== null && crossPairs.length === 0) {
      const substitutes = loadOracleRegistry(repoRoot).noOracleDecisions.find((entry) => entry.id === decision.noOracleDecision)?.substitutes ?? [];
      if (substitutes.includes("independent-implementations")) problems.push(`${discovered.caseDir}: no-oracle decision ${decision.noOracleDecision} claims the independent-implementations substitute but only one implementation ran`);
      else if (!caseResults.every((result) => ["conformance", "property", "round-trip", "error"].includes(planModeOf(repoRoot, discovered, level, result.scenario)))) {
        problems.push(`${discovered.caseDir}: no-oracle decision ${decision.noOracleDecision} rests on ${substitutes.join(", ")}, which only discharge a conformance, property, round-trip or error scenario — a differential scenario needs an oracle or a second implementation`);
      }
    }
  }

  // 🚫️A selected case that produced no result at all is not a pass — it is an absence of evidence,
  // and the two must never look the same. The commonest cause is legitimate (asking for the oracle
  // phase of a recorded no-oracle case, which by definition has no oracle to run), so this reports
  // rather than fails; what it must not do is stay silent while the run prints a green summary.
  const exercised = new Set(allResults.map((result) => `${result.owner}::${result.case}`));
  const unexercised = cases.filter((discovered) => !exercised.has(`${discovered.owner}::${discovered.case}`));
  for (const discovered of unexercised) {
    const decision = oracleDecision(repoRoot, discovered, level);
    const why = decision.implementation === null && decision.noOracleDecision !== null ? `recorded no-oracle decision ${decision.noOracleDecision} — its evidence is discharged by the subject phase` : `no implementation served the requested phase(s) ${phases.join(", ")}`;
    console.error(`[test] not-exercised ${discovered.caseDir} (${why})`);
  }

  const summary = summarizeRun(level, cases.length, scenarioCount, allResults, parity, problems);
  const dir = reportsDir(repoRoot);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "📊️summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  writeFileSync(join(dir, "📤️results.jsonl"), allResults.map((result) => JSON.stringify(result)).join("\n") + (allResults.length > 0 ? "\n" : ""));
  writeFileSync(join(dir, "📋️junit.xml"), renderJUnit(allResults));
  const metrics = computeCoverageMetrics(repoRoot, cases, allResults, parity, loadClassifiedBaseline(repoRoot));
  writeFileSync(join(dir, "📈️metrics.json"), `${JSON.stringify(metrics, null, 2)}\n`);
  markRunComplete(dir);

  console.log(`[test] level=${level} cases=${summary.cases} executed=${summary.executed} passed=${summary.passed} failed=${summary.failed} errored=${summary.errored} parity=${parity.filter((row) => row.equal).length}/${parity.length}${unexercised.length > 0 ? ` not-exercised=${unexercised.length}` : ""}`);
  if (segments.includes("--metrics")) console.log(formatMetrics(metrics, readImplementationCoverage(repoRoot)));
  for (const problem of problems) console.error(`[test] ${problem}`);
  return summary.failed + summary.errored + problems.length === 0 ? 0 : 1;
}
//#endregion 🏃️Run

//#region 🧭️Commands
/** 🔍️ Lists every discovered case as JSON — the same list Nx generates projects from. */
class DiscoverScript extends Script {
  run(segments: string[]): void {
    const cases = discoverTestCases(this.repoRoot);
    if (segments.includes("--json")) {
      console.log(JSON.stringify(cases, null, 2));
      return;
    }
    for (const entry of cases) console.log(`${entry.projectName}\t${entry.caseDir}\t[${Object.keys(entry.adapters).join(",") || "no-adapter"}]`);
    console.log(`[discover] ${cases.length} test case(s)`);
  }
}

/** 🧾️ The contract phase — everything provable without executing a test. */
class ContractScript extends Script {
  run(segments: string[]): void {
    const cases = selectCases(this.repoRoot, segments);
    const breaches: BreachRecord[] = validateAllContracts(this.repoRoot, cases);
    const cachePath = join(getRepoMetaDir(this.repoRoot), "⚡️cache", "breaches", "testing.json");
    mkdirSync(join(getRepoMetaDir(this.repoRoot), "⚡️cache", "breaches"), { recursive: true });
    writeFileSync(cachePath, `${JSON.stringify(breaches, null, 2)}\n`);
    console.log(formatBreachReport(breaches, cachePath));
    if (breaches.length > 0) process.exit(1);
  }
}

/** 🔮️ The oracle phase — proves the reference library actually supports the case before any local code exists, except an explicit byte-decoder oracle, which first receives its subject artifact. */
class OracleScript extends Script {
  run(segments: string[]): void {
    process.exit(runPhases(this.repoRoot, segments, ["oracle"]));
  }
}

/** 🎯️ The subject phase — this repository's implementations, on the same inputs. */
class SubjectScript extends Script {
  run(segments: string[]): void {
    process.exit(runPhases(this.repoRoot, segments, ["subject"]));
  }
}

/** ⚖️ Oracle + subject + semantic comparison + pairwise subject equivalence. */
class ParityScript extends Script {
  run(segments: string[]): void {
    process.exit(runPhases(this.repoRoot, segments, ["oracle", "subject"]));
  }
}

/** ▶️ The default phase chain for a level: contract, then full parity. */
class RunScript extends Script {
  run(segments: string[]): void {
    const cases = selectCases(this.repoRoot, segments);
    const breaches = validateAllContracts(this.repoRoot, cases);
    if (breaches.length > 0) {
      console.error(formatBreachReport(breaches, "(not cached — contract phase failed inside run)"));
      process.exit(1);
    }
    process.exit(runPhases(this.repoRoot, segments, ["oracle", "subject"]));
  }
}

/** 📊️ Re-renders the report artifacts from the last run's result stream. */
class ReportScript extends Script {
  run(): void {
    const dir = reportsDir(this.repoRoot);
    const stream = join(dir, "📤️results.jsonl");
    const { results, problems } = readResults(stream);
    writeFileSync(join(dir, "📋️junit.xml"), renderJUnit(results));
    console.log(`[report] ${results.length} result(s) from ${relative(this.repoRoot, stream).split(sep).join("/")}`);
    for (const problem of problems) console.error(`[report] ${problem}`);
  }
}

/** 🧹️ Marker-guarded removal of generated test state. Never descends into an excluded area. */
class CleanScript extends Script {
  run(segments: string[]): void {
    const dry = segments.includes("--dry");
    const stale = segments.includes("--stale");
    const overIndex = segments.indexOf("--over");
    const over = overIndex === -1 ? undefined : Number(segments[overIndex + 1]);
    if (over !== undefined && !Number.isFinite(over)) {
      console.error("[clean test] --over needs a byte count, for example `--over 104857600`");
      process.exit(1);
    }
    const liveTestIds = new Set(discoverTestCases(this.repoRoot).map((entry) => `${entry.owner}::${entry.case}`));
    const report = cleanTestOutputs(this.repoRoot, { dry, stale, over, liveTestIds: stale ? liveTestIds : undefined });
    console.log(formatCleanReport(this.repoRoot, report));
    // 🚫️Which areas may never be touched is taxonomy vocabulary; this router names none of them.
    const leaked = report.removals.filter((row) => isExcludedTestPath(this.repoRoot, row.path));
    if (leaked.length > 0) {
      console.error(`[clean test] refusing: ${leaked.length} candidate(s) resolved inside an excluded area`);
      process.exit(1);
    }
  }
}

/** 🔒️ Multi-ecosystem dependency classification with production reachability and a shrink-only ratchet. */
class DependencyScript extends Script {
  run(segments: string[]): void {
    const sorted = loadClassifiedBaseline(this.repoRoot);
    const registry = loadOracleRegistry(this.repoRoot);
    if (segments.includes("write-baseline")) {
      const baselinePath = join(this.repoRoot, "🔒️dependencies.json");
      const baselineRaw = JSON.parse(readFileSync(baselinePath, "utf8")) as Record<string, unknown>;
      writeFileSync(baselinePath, `${JSON.stringify({ ...baselineRaw, schemaVersion: 2, entries: sorted }, null, 2)}\n`);
      console.log(`[dependency] baseline rewritten with ${sorted.length} classified entries`);
      return;
    }

    // 🔎️The ratchet is fed the COMMITTED baseline against a FRESH SCAN of the live tree. It used to be
    // handed the same array twice, which made `newProduction` and `unregisteredTestDeps` provably
    // always empty however many production dependencies a change added — a shrink-only gate that
    // could not see growth. `--scan` prints what the scan found, for when the two disagree.
    const scanned = scanDeclaredDependencies(this.repoRoot, registry);
    const verdict = ratchetDependencies(sorted, scanned, registry);
    if (segments.includes("--scan")) {
      console.log(`[dependency] live scan: ${scanned.length} declared external dependenc(ies), ${scanned.filter((entry) => entry.productionReachable).length} production-reachable`);
      for (const entry of scanned.filter((candidate) => !sorted.some((baseline) => baseline.ecosystem === candidate.ecosystem && baseline.name === candidate.name))) {
        console.log(`[dependency] scan-only ${entry.ecosystem}:${entry.name}@${entry.version} kinds=${entry.kinds.join(",")} users=${entry.users.slice(0, 2).join(",")}`);
      }
    }
    const production = sorted.filter((entry) => entry.productionReachable);
    const oracleDeps = sorted.filter((entry) => entry.kinds.includes("test-oracle"));
    console.log(`[dependency] ecosystems=${new Set(sorted.map((entry) => entry.ecosystem)).size} entries=${sorted.length} production-reachable=${production.length} test-oracle=${oracleDeps.length}`);
    for (const entry of oracleDeps) console.log(`[dependency] test-oracle ${entry.ecosystem}:${entry.name}@${entry.version} (${(entry.oracleIds ?? []).join(",")})`);
    // 🔒️Recorded debt is printed every run so it can never quietly become permanent; an UNRECORDED
    // production-reachable oracle is still a hard failure.
    const recorded = new Map(registry.oracles.filter((entry) => entry.productionDebt !== undefined).map((entry) => [entry.package, entry]));
    for (const [, entry] of recorded) console.log(`[dependency] production-debt ${entry.package} (oracle ${entry.id}) reachable from ${entry.productionDebt!.reachableFrom.join(", ")} — owner ${entry.productionDebt!.owner}`);
    const leaked = oracleDeps.filter((entry) => entry.productionReachable && !recorded.has(entry.name));
    for (const entry of leaked) console.error(`[dependency] oracle package ${entry.name} is production-reachable and is NOT recorded as debt — oracles must be test-only`);
    for (const name of verdict.newProduction) console.error(`[dependency] NEW production-reachable dependency ${name} is declared in the tree and absent from the committed baseline — the ratchet is shrink-only`);
    for (const name of verdict.unregisteredTestDeps) console.error(`[dependency] ${name} is declared as a test dependency but no oracle or probe registers it`);
    if (!verdict.ok || leaked.length > 0) process.exit(1);
  }
}

/** 📈️ The non-aggregate gates: scenario, implementation, parity, oracle and per-implementation source
 * coverage. Reads the last run's metrics rather than re-running it, so a gate can never be satisfied
 * by a run that never happened. */
class MetricsScript extends Script {
  run(segments: string[]): void {
    const metricsPath = join(reportsDir(this.repoRoot), "📈️metrics.json");
    if (!existsSync(metricsPath)) {
      console.error(`[metrics] no run to report on — run \`bun ./📜️script.ts parity <level>\` first`);
      process.exit(1);
    }
    const metrics = JSON.parse(readFileSync(metricsPath, "utf8")) as CoverageMetrics;
    const perImplementation = readImplementationCoverage(this.repoRoot);
    console.log(formatMetrics(metrics, perImplementation));
    if (!segments.includes("--enforce")) return;
    const threshold = Number(segments[segments.indexOf("--threshold") + 1] ?? 95);
    const failures = enforceMetricGates(metrics, perImplementation, Number.isFinite(threshold) ? threshold : 95);
    for (const failure of failures) console.error(`[metrics] ${failure}`);
    if (failures.length > 0) process.exit(1);
  }
}

/** 🕸️ Emits the per-case Nx project graph the plugin generates, for inspection and for CI. */
class NxScript extends Script {
  run(): void {
    console.log(JSON.stringify(discoverTestCases(this.repoRoot).map((entry) => ({ name: entry.projectName, root: entry.caseDir, owner: entry.owner, implementations: Object.keys(entry.adapters) })), null, 2));
  }
}

/** 🩺️ Reports which toolchains are present. A missing tool fails setup — it never becomes a silent skip. */
class DoctorScript extends Script {
  run(): void {
    const taxonomy = testTaxonomy(this.repoRoot);
    const checks: [Implementation, string, string[]][] = [
      ["typescript", "bun", ["--version"]],
      ["rust", "cargo", ["--version"]],
      ["go", "go", ["version"]],
      ["python", process.env.SEMIO_PYTHON ?? "python3", ["--version"]],
      ["dotnet", "dotnet", ["--version"]],
    ];
    const claimed = new Set(discoverTestCases(this.repoRoot).flatMap((entry) => Object.keys(entry.adapters)));
    let missing = 0;
    for (const [implementation, command, args] of checks) {
      const probe = runProbe(command, args, { cwd: this.repoRoot, budgetMs: 30_000 });
      const ok = (probe.status ?? 1) === 0;
      const required = claimed.has(implementation);
      console.log(`[doctor] ${implementation}: ${ok ? probe.stdout.trim().split("\n")[0] : "MISSING"}${required ? " (required by a discovered case)" : ""}`);
      if (!ok && required) missing += 1;
    }
    console.log(`[doctor] cache root: ${relative(this.repoRoot, testCacheDir(this.repoRoot, taxonomy.testOutputChildDirs[0]!)).split(sep).join("/")}`);
    if (missing > 0) process.exit(1);
  }
}

/**
 * 🏭️ The runtime mutation inventory phase. Runs each owner's PRODUCTION dispatch bridge and records
 * what it actually offers, so completeness becomes a measurement rather than a claim. This is the
 * phase v1 had no equivalent of: its mutation audit compared a catalog with checked-in evidence and
 * never consulted dispatch, so a mutation reachable in production and missing from the catalog left
 * no trace anywhere.
 */
class InventoryScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const manifests = registry.contributions.flatMap((contribution) => contribution.mutationManifests.map((manifest) => ({ contribution, manifest }))).filter(({ manifest }) => matchesTarget(manifest, selectors));
    if (manifests.length === 0) {
      console.error("[inventory] no mutation manifest matches the selection — declare one in the owner's 🧪️oracle contribution");
      process.exit(1);
    }
    let failed = 0;
    for (const { contribution, manifest } of manifests) {
      const coordinate = subsetCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset: manifest.subset });
      const bridge = mutationBridgeFor(this.repoRoot, contribution.owner, manifest);
      if (bridge === null) {
        console.error(`[inventory] ${coordinate}: no production mutation bridge — expected an executable at ${MUTATION_BRIDGE_REL} beside the owner`);
        failed += 1;
        continue;
      }
      const probe = runProbe(bridge.command, bridge.args, { cwd: this.repoRoot, budgetMs: testLevelBudgetMs("long"), env: { ...process.env, SEMIO_MUTATION_ARTIFACT: manifest.artifact, SEMIO_MUTATION_STANDARD: manifest.standard, SEMIO_MUTATION_SUBSET: manifest.subset } });
      if ((probe.status ?? 1) !== 0) {
        console.error(`[inventory] ${coordinate}: bridge exited ${probe.status} — ${probe.stderr.trim().split("\n").slice(-3).join(" | ")}`);
        failed += 1;
        continue;
      }
      let inventory: RuntimeMutationInventory;
      try {
        inventory = JSON.parse(probe.stdout) as RuntimeMutationInventory;
      } catch (error) {
        console.error(`[inventory] ${coordinate}: bridge did not emit a runtime inventory (${(error as Error).message})`);
        failed += 1;
        continue;
      }
      if (inventory.schema !== "semio.repository-test.runtime-inventory/v2") {
        console.error(`[inventory] ${coordinate}: bridge emitted schema ${JSON.stringify(inventory.schema)}`);
        failed += 1;
        continue;
      }
      const path = writeRuntimeInventory(this.repoRoot, inventory);
      const equality = compareInventories(manifest, inventory, []);
      const drift = equality.runtimeOnly.length + equality.manifestOnly.length + equality.outcomeMismatches.length + equality.variantMismatches.length;
      console.log(`[inventory] ${coordinate}: ${inventory.mutations.length} runtime mutation(s), ${manifest.mutations.length} declared, ${drift} difference(s) → ${relative(this.repoRoot, path).split(sep).join("/")}`);
      for (const id of equality.runtimeOnly) console.error(`[inventory] ${coordinate}: runtime-only ${id}`);
      for (const id of equality.manifestOnly) console.error(`[inventory] ${coordinate}: manifest-only ${id}`);
      if (drift > 0) failed += 1;
    }
    if (failed > 0) process.exit(1);
  }
}

/** 🧫️ Fixture generation, reproduction, verification and audit — the four halves of provenance. */
class FixtureScript extends Script {
  run(segments: string[]): void {
    const [subcommand = "verify"] = segments;
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const fixtures = registry.contributions.flatMap((contribution) => contribution.fixtureManifests).filter((fixture) => matchesFixture(fixture, selectors));
    switch (subcommand) {
      case "verify": {
        let bad = 0;
        for (const fixture of fixtures) {
          for (const verification of verifyFixture(this.repoRoot, fixture)) {
            if (verification.ok) continue;
            bad += 1;
            console.error(`[fixture verify] ${fixture.id}/${verification.role}: ${verification.missing ? "missing" : `${verification.actual} ≠ ${verification.expected}`} (${verification.path})`);
          }
        }
        console.log(`[fixture verify] ${fixtures.length} fixture(s), ${bad} file problem(s)`);
        if (bad > 0) process.exit(1);
        return;
      }
      case "audit": {
        const rows = fixtures.map((fixture) => ({
          id: fixture.id,
          class: fixture.class,
          target: subsetCoordinate(fixture.target),
          mutation: fixture.mutation ?? "",
          outcome: fixture.outcome ?? "",
          license: fixture.provenance.license,
          reproducible: fixture.reproducible,
          generator: fixture.generator?.oracle ?? "",
          engine: fixture.generator?.engineFamily ?? "",
          // 🪆️Resolved, not spelled: `fixtureManifestProblems` judges `✳️any` by what sits BESIDE it
          // when it is handed the repository, exactly as the contract phase and the coverage gate
          // already do (both other call sites pass it). Omitting it here made `fixture audit` the
          // one command that read the bare spelling, so every single-subset owner's fixture — gif,
          // las, obj — audited as a wildcard breach while the release gate it feeds reported it clean.
          problems: fixtureManifestProblems(fixture, this.repoRoot),
        }));
        if (segments.includes("--json")) {
          console.log(JSON.stringify(rows, null, 2));
          return;
        }
        for (const row of rows) console.log(`[fixture audit] ${row.class.padEnd(24)} ${row.target} ${row.mutation}/${row.outcome} licence=${row.license} reproducible=${row.reproducible} generator=${row.generator}(${row.engine})${row.problems.length > 0 ? ` PROBLEMS: ${row.problems.join("; ")}` : ""}`);
        const bad = rows.filter((row) => row.problems.length > 0).length;
        console.log(`[fixture audit] ${rows.length} fixture(s), ${bad} with contract problems`);
        if (bad > 0) process.exit(1);
        return;
      }
      case "reproduce": {
        // 🏭️Reproduction re-runs the RECORDED generator command and compares the bytes it produces with
        // the committed ones. It never writes into the committed fixture: a "reproduce" that overwrote
        // its own expectation would pass unconditionally, which is the whole failure mode it guards.
        let failed = 0;
        for (const fixture of fixtures.filter((entry) => entry.class === "third-party-generated")) {
          if (fixture.generator === undefined) {
            console.error(`[fixture reproduce] ${fixture.id}: no generator record`);
            failed += 1;
            continue;
          }
          const outDir = join(testCacheDir(this.repoRoot, "work"), "🧫️reproduce", fixture.id);
          rmSync(outDir, { recursive: true, force: true });
          mkdirSync(outDir, { recursive: true });
          const [command = "", ...args] = fixture.generator.command.split(/\s+/);
          const probe = runProbe(command, args, { cwd: this.repoRoot, budgetMs: testLevelBudgetMs("long"), env: { ...process.env, SEMIO_FIXTURE_OUT: outDir, SEMIO_FIXTURE_SEED: String(fixture.generator.seed ?? "") } });
          if ((probe.status ?? 1) !== 0) {
            console.error(`[fixture reproduce] ${fixture.id}: generator exited ${probe.status}`);
            failed += 1;
            continue;
          }
          for (const file of fixture.files) {
            const produced = join(outDir, fixture.id, basename(file.path));
            if (!existsSync(produced)) {
              console.error(`[fixture reproduce] ${fixture.id}/${file.role}: generator produced no ${basename(file.path)}`);
              failed += 1;
              continue;
            }
            const actual = contentDigestOf(produced);
            if (actual !== file.sha256) {
              console.error(`[fixture reproduce] ${fixture.id}/${file.role}: ${actual} ≠ ${file.sha256}`);
              failed += 1;
            }
          }
        }
        console.log(`[fixture reproduce] ${fixtures.filter((entry) => entry.class === "third-party-generated").length} generated fixture(s), ${failed} problem(s)`);
        if (failed > 0) process.exit(1);
        return;
      }
      case "generate": {
        // 🏭️Generation and execution are separate operations on purpose: a normal test run must never
        // be able to rewrite the expectation it is being measured against.
        let generated = 0;
        for (const fixture of fixtures.filter((entry) => entry.class === "third-party-generated" && entry.generator !== undefined)) {
          const outDir = join(testCacheDir(this.repoRoot, "work"), "🧫️generate", fixture.id);
          mkdirSync(outDir, { recursive: true });
          const [command = "", ...args] = fixture.generator!.command.split(/\s+/);
          const probe = runProbe(command, args, { cwd: this.repoRoot, budgetMs: testLevelBudgetMs("long"), env: { ...process.env, SEMIO_FIXTURE_OUT: outDir, SEMIO_FIXTURE_SEED: String(fixture.generator!.seed ?? "") } });
          if ((probe.status ?? 1) !== 0) {
            console.error(`[fixture generate] ${fixture.id}: generator exited ${probe.status} — ${probe.stderr.trim().split("\n").slice(-3).join(" | ")}`);
            continue;
          }
          for (const file of fixture.files) {
            const produced = join(outDir, fixture.id, basename(file.path));
            if (!existsSync(produced)) continue;
            installFixtureFile(this.repoRoot, produced);
          }
          publishFixtureManifest(this.repoRoot, fixture);
          generated += 1;
          console.log(`[fixture generate] ${fixture.id}: ${fixture.files.length} file(s) into the content-addressed store`);
        }
        console.log(`[fixture generate] ${generated} fixture bundle(s) generated — commit review is a separate, human step`);
        return;
      }
      default:
        console.error(`[fixture] unknown subcommand ${JSON.stringify(subcommand)} — expected generate | reproduce | verify | audit`);
        process.exit(1);
    }
  }
}

/** 🔬️ Lists and qualifies the external measurement probes the comparison pipeline invokes. */
class ProbeScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const probes = registry.probes.filter((probe) => selectors.probe === null || probe.id === selectors.probe);
    if (segments.includes("--json")) {
      console.log(JSON.stringify(probes, null, 2));
      return;
    }
    for (const probe of probes) {
      const status = probe.qualification?.status ?? "unqualified";
      console.log(`[probe] ${probe.id.padEnd(28)} ${probe.kind.padEnd(17)} engine=${engineFamilyId(probe.engine)}@${probe.engine?.version ?? "*"} deterministic=${probe.deterministic} qualification=${status}`);
      for (const criterion of probe.qualification?.criteria ?? []) console.log(`[probe]   ${criterion.met ? "✔" : "✘"} ${criterion.id}${criterion.detail === undefined ? "" : ` — ${criterion.detail}`}`);
    }
    const unqualified = probes.filter((probe) => !isQualifiedProbe(probe));
    console.log(`[probe] ${probes.length} probe(s), ${unqualified.length} not yet qualified`);
    for (const probe of unqualified) console.log(`[probe] ${probe.id}: RUNS and REPORTS; no release gate may claim its strongest guarantee until the qualification spike passes`);
  }
}

/** 📊️ The full subset-scoped coverage matrix and its release gates. Never an artifact-level aggregate. */
class MatrixScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const { results } = readResults(join(reportsDir(this.repoRoot), "📤️results.jsonl"));
    const baselineSha = process.env.SEMIO_BASELINE_SHA ?? "";
    const rows = buildCoverageMatrix(this.repoRoot, registry, results, baselineSha).filter((row) => matchesRow(row, selectors));
    const inventories = registry.mutationManifests.map((manifest) => readRuntimeInventory(this.repoRoot, manifest)).filter((inventory): inventory is RuntimeMutationInventory => inventory !== null);
    const measurements = measureCoverage(registry, rows, results, inventories);
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ baselineSha, rows, measurements }, null, 2));
      return;
    }
    for (const measurement of measurements) console.log(`[matrix] ${measurement.dimension.padEnd(32)} ${(measurement.ratio * 100).toFixed(2).padStart(6)}%  ${measurement.covered}/${measurement.total}`);
    console.log(formatCoverageQuestions(registry, rows, measurements));
    if (!segments.includes("--enforce")) return;
    const wildcardOwners = registry.mutationManifests.flatMap((manifest) => manifest.mutations.filter((mutation) => isWildcardSubset(owningSubsetOf(manifest, mutation)))).length;
    const deferred = registry.mutationCatalogs.reduce((total, catalog) => total + (catalog.deferredKinds ?? []).length, 0);
    const unregistered = measurements.find((measurement) => measurement.dimension === "runtimeMutationCoverage")?.missing.length ?? 0;
    const failures = enforceReleaseGates(measurements, { deferredMutations: deferred, skipped: 0, wildcardOwners, unregisteredRuntimeMutations: unregistered });
    for (const failure of failures) console.error(`[matrix] ${failure}`);
    if (failures.length > 0) process.exit(1);
  }
}

/** 🧹️ Mark-and-sweep over the fixture store and the run directories. Dry by default, everywhere. */
class GcScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const value = (flag: string): string | null => {
      const index = segments.indexOf(flag);
      return index === -1 ? null : (segments[index + 1] ?? null);
    };
    const olderThan = value("--older-than");
    const overSize = value("--over-size");
    const report = collectGarbage(this.repoRoot, registry, {
      dry: !segments.includes("--apply"),
      olderThanMs: olderThan === null ? undefined : Number(olderThan) * 1000,
      overBytes: overSize === null ? undefined : Number(overSize),
      agent: value("--agent") ?? undefined,
      retention: value("--retention") === null ? undefined : [value("--retention") as RetentionClass],
    });
    console.log(formatGcReport(report));
  }
}

/**
 * 🕳️ What each owner still owes before its mutations are externally oracled at subset level.
 *
 * The target is that EVERY mutation of every artifact is predicted by a third-party library, scoped to
 * the smallest semantic subset. `matrix` measures what is covered; this answers the complementary and
 * more actionable question — what is missing, per owner, and which of four things it is. Without it the
 * shortfall is a single percentage, and a percentage tells nobody what to do on Monday.
 */
/**
 * 🧬️ Derives each mutation leaf's payload schema from the Rust payload struct it declares.
 *
 * The payload schema is the DEEPEST blocker in the chain to subset-scoped external-oracle coverage:
 * you cannot author a fixture for a mutation whose payload has no contract, so every leaf without one
 * is unreachable by any amount of testing effort. The struct IS the contract — it is what serde puts
 * on the wire — so this is a projection of it, not a second declaration to keep in sync.
 */
function payloadSchemaCommand(script: Script, registry: OracleRegistry, selectors: ReturnType<typeof readSelectors>, write: boolean): void {
  const owners = [...new Set(registry.contributions.map((entry) => entry.owner))].filter((owner) => selectors.subset === null || subsetCoordinatesOfOwner(owner)?.subset === selectors.subset);
  let leaves = 0;
  let derived = 0;
  let written = 0;
  let present = 0;
  const refusals = new Map<string, number>();
  for (const owner of owners) {
    for (const row of derivePayloadSchemas(script.repoRoot, owner)) {
      leaves += 1;
      if (row.schema === null) {
        for (const why of row.refused) {
          const type = why.replace(/^field [a-z_]+: /, "").replace(/ is not a shape.*/, "");
          refusals.set(type, (refusals.get(type) ?? 0) + 1);
        }
        continue;
      }
      derived += 1;
      const path = join(script.repoRoot, row.leaf, "🔣️payload.schema.json");
      if (existsSync(path)) {
        present += 1;
        continue;
      }
      if (!write) continue;
      writeFileSync(path, `${JSON.stringify(row.schema, null, 2)}\n`);
      written += 1;
    }
  }
  console.log(`[manifest payload-schema] ${derived}/${leaves} leaves derivable from their Rust payload struct (${((derived / Math.max(leaves, 1)) * 100).toFixed(1)}%)`);
  console.log(`[manifest payload-schema] ${present} already carry a schema; ${derived - present} would be new`);
  console.log(`[manifest payload-schema] ${leaves - derived} refused, by the Rust type that defeated them:`);
  for (const [type, count] of [...refusals].sort((a, b) => b[1] - a[1]).slice(0, 15)) console.log(`[manifest payload-schema]   ${String(count).padStart(4)} × ${type.slice(0, 72)}`);
  console.log(write ? `[manifest payload-schema] ${written} schema(s) written` : "[manifest payload-schema] dry run — pass --write to emit them");
}

/** 🏗️ Derives leaf descriptors from the leaves themselves, refusing every field it cannot cite. */
function scaffoldCommand(script: Script, registry: OracleRegistry, selectors: ReturnType<typeof readSelectors>, write: boolean): void {
  const owners = [...new Set(registry.contributions.map((entry) => entry.owner))].filter((owner) => selectors.subset === null || subsetCoordinatesOfOwner(owner)?.subset === selectors.subset);
  let leaves = 0;
  let derived = 0;
  let written = 0;
  const refusals = new Map<string, number>();
  const ready: string[] = [];
  const taxonomy = testTaxonomy(script.repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  for (const owner of owners) {
    const rows = scaffoldOwnerDescriptors(script.repoRoot, owner);
    if (rows.length === 0) continue;
    leaves += rows.length;
    derived += rows.filter((row) => row.descriptor !== null).length;
    for (const row of rows) for (const why of row.refused) refusals.set(why.split(":")[0]!, (refusals.get(why.split(":")[0]!) ?? 0) + 1);
    // 🏗️An owner is written ALL-OR-NOTHING. A partial descriptor set would let a manifest be generated
    // over a denominator that silently omits the undescribed leaves, which reads as coverage of a
    // smaller vocabulary rather than as the gap it is.
    if (rows.some((row) => row.descriptor === null)) continue;
    ready.push(`${owner} (${rows.length} leaves)`);
    if (!write) continue;
    for (const row of rows) {
      const path = join(script.repoRoot, row.leaf, filename);
      if (existsSync(path)) continue;
      writeFileSync(path, `${JSON.stringify(row.descriptor, null, 2)}\n`);
      written += 1;
    }
  }
  console.log(`[manifest scaffold] ${derived}/${leaves} leaves derivable with full evidence (${((derived / Math.max(leaves, 1)) * 100).toFixed(1)}%)`);
  for (const [field, count] of [...refusals].sort((a, b) => b[1] - a[1])) console.log(`[manifest scaffold] refused ${String(count).padStart(5)} × ${field}`);
  console.log(`[manifest scaffold] ${ready.length} owner(s) fully derivable:`);
  for (const owner of ready) console.log(`[manifest scaffold]   ${owner}`);
  console.log(write ? `[manifest scaffold] ${written} descriptor(s) written` : "[manifest scaffold] dry run — pass --write to emit descriptors for the fully derivable owners");
}

class GapScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const manifested = new Set(registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => mutation.capability)));
    // 🧫️A capability is only genuinely covered once fixtures exist for it too. Counting a manifest as
    // sufficient would repeat the error this whole protocol exists to remove.
    const fixtured = new Set(registry.contributions.flatMap((contribution) => contribution.fixtureManifests.map((fixture) => `${fixture.target.artifact}@${fixture.target.standard}/${fixture.target.subset}`)));
    const fixtureCountFor = (artifact: string, standard: string, subset: string): number =>
      registry.contributions.flatMap((contribution) => contribution.fixtureManifests).filter((fixture) => fixture.target.artifact === artifact && fixture.target.standard === standard && fixture.target.subset === subset).length;

    type Row = { owner: string; catalog: string; capability: string; kinds: number; subset: string; state: string; oracles: string; owed: string; fixtures: number };
    const rows: Row[] = [];
    for (const contribution of registry.contributions) {
      for (const catalog of contribution.mutationCatalogs) {
        const coordinates = subsetCoordinatesOfOwner(contribution.owner);
        const subset = coordinates?.subset ?? "";
        if (selectors.subset !== null && subset !== selectors.subset) continue;
        const supplying = registry.oracles.filter((oracle) => oracle.capabilities.includes(catalog.capability));
        const qualifying = supplying.filter((oracle) => isQualifyingOracleKind(oracle.kind));
        const state = qualifying.length > 0 ? (manifested.has(catalog.capability) ? "covered" : "manifestable") : supplying.length > 0 ? "supplemental-only" : "un-oracled";
        if (selectors.status !== null && state !== selectors.status) continue;
        // 🕳️What is OWED, stated in full. An earlier version said "only a manifest" for the qualifying
        // group, and that was an understatement of exactly the kind this protocol exists to remove: a
        // manifest also needs the OUTCOME CLASSES each mutation can reach, which nothing can state
        // honestly until the production bridge has been run, and every mutation needs a fixture whose
        // expected result that oracle actually produced. A qualifying oracle is the PREREQUISITE, not
        // the remaining work.
        const owed =
          state === "covered"
            ? "—"
            : state === "manifestable"
              ? "a manifest (needs each mutation's OUTCOME CLASSES, which only the production bridge can state), a runtime inventory, and a fixture per mutation × outcome — the oracle is in place"
              : state === "supplemental-only"
                ? `a QUALIFYING third-party oracle before anything else; today only ${supplying.map((oracle) => `${oracle.id}(${oracle.kind ?? "unclassified"})`).join(", ")}`
                : "a qualifying third-party oracle, and nothing supplies this capability at all";
        const fixtures = coordinates === null ? 0 : fixtureCountFor(catalog.capability.startsWith("step-") ? "s.stdio.step" : "", coordinates.standard, coordinates.subset);
        rows.push({ owner: contribution.owner, catalog: catalog.id, capability: catalog.capability, kinds: catalog.kinds.length, subset, state, oracles: qualifying.map((oracle) => oracle.id).join(",") || "—", owed, fixtures });
      }
    }

    if (segments.includes("--json")) {
      console.log(JSON.stringify(rows, null, 2));
      return;
    }

    const byState = new Map<string, Row[]>();
    for (const row of rows) byState.set(row.state, [...(byState.get(row.state) ?? []), row]);
    const mutationsIn = (state: string): number => (byState.get(state) ?? []).reduce((total, row) => total + row.kinds, 0);
    for (const state of ["covered", "manifestable", "supplemental-only", "un-oracled"]) {
      const group = byState.get(state) ?? [];
      console.log(`\n[gap] ${state.toUpperCase()} — ${group.length} catalog(s), ${mutationsIn(state)} mutation kind(s)`);
      for (const row of group.slice(0, segments.includes("--all") ? group.length : 12)) {
        console.log(`[gap]   ${row.capability.padEnd(34)} ${String(row.kinds).padStart(4)} kinds  ${String(row.fixtures).padStart(4)} fixtures  ${row.subset.padEnd(10)} ${row.owed}`);
      }
      if (!segments.includes("--all") && group.length > 12) console.log(`[gap]   … and ${group.length - 12} more (--all to list, --json for the full record)`);
    }

    const total = rows.reduce((sum, row) => sum + row.kinds, 0);
    const covered = mutationsIn("covered");
    console.log(`\n[gap] ${covered}/${total} mutation kind(s) are externally oracled and manifested — ${((covered / Math.max(total, 1)) * 100).toFixed(1)}%`);
    console.log(`[gap] ${mutationsIn("manifestable")} kind(s) have a qualifying oracle and still need a manifest, a runtime inventory and fixtures`);
    console.log(`[gap] ${mutationsIn("supplemental-only") + mutationsIn("un-oracled")} kind(s) need a qualifying third-party oracle BEFORE any of that`);
    // 🏭️Nothing above can be completed while the production bridge cannot run: a manifest must declare
    // the outcome classes each mutation reaches, and only dispatch can state them.
    const inventories = registry.mutationManifests.filter((manifest) => readRuntimeInventory(this.repoRoot, manifest) !== null).length;
    console.log(`[gap] ${inventories}/${registry.mutationManifests.length} manifest(s) have a runtime inventory — a manifest's outcome classes cannot be stated honestly without one`);
    // 🕳️A cross-semio implementation is the single largest category, and naming it as such is the point:
    // it reads as an oracle in the registry and discharges nothing.
    const semioDerived = registry.oracles.filter((oracle) => oracle.kind === "cross-semio-implementation").length;
    console.log(`[gap] ${semioDerived} of ${registry.oracles.length} registered oracles are second implementations written inside this repository, and none of them discharges a mutation's requirement`);
  }
}

/**
 * 🧬️ Generates each owner's v2 mutation manifest FROM ITS LEAF DESCRIPTORS, and reports who cannot yet
 * have one.
 *
 * A manifest must state the OUTCOME CLASSES each mutation can reach, and that is the one field nobody
 * can honestly invent from outside the implementation. The `dsl::Mutations` derive already reads it
 * from a declarative per-leaf JSON descriptor at expansion time — so a manifest built from the same
 * file is generated from production's own record rather than restated beside it, and it needs no
 * compiler, no running bridge and no guess.
 *
 *   bun 📜️script.ts manifest --dry            # who is ready, who is blocked, and on what
 *   bun 📜️script.ts manifest --write          # write manifests for every ready owner
 */
class ManifestScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const write = segments.includes("--write");
    if (segments.includes("scaffold")) return scaffoldCommand(this, registry, selectors, write);
    if (segments.includes("payload-schema")) return payloadSchemaCommand(this, registry, selectors, write);

    type Row = { owner: string; capability: string; leaves: number; described: number; ready: boolean; reason: string; manifest: MutationManifest | null };
    const rows: Row[] = [];
    for (const contribution of registry.contributions) {
      for (const catalog of contribution.mutationCatalogs) {
        if (selectors.subset !== null && subsetCoordinatesOfOwner(contribution.owner)?.subset !== selectors.subset) continue;
        const coverage = leafDescriptorCoverage(this.repoRoot, contribution.owner);
        const manifest = manifestFromLeafDescriptors(this.repoRoot, contribution.owner, catalog.capability);
        const qualifying = registry.oracles.filter((oracle) => oracle.capabilities.includes(catalog.capability) && isQualifyingOracleKind(oracle.kind));
        const reason =
          coverage.leaves === 0
            ? "no mutation leaves on disk"
            : coverage.missing.length > 0
              ? `${coverage.missing.length}/${coverage.leaves} leaves carry no descriptor — a manifest whose outcome classes were guessed for even one mutation is worse than none`
              : manifest === null
                ? "leaves are described but the owner path or artifact id could not be resolved"
                : qualifying.length === 0
                  ? "described, but no QUALIFYING third-party oracle supplies this capability — the manifest would declare a requirement nothing can discharge"
                  : "ready";
        rows.push({ owner: contribution.owner, capability: catalog.capability, leaves: coverage.leaves, described: coverage.described, ready: reason === "ready", reason, manifest });
      }
    }

    if (segments.includes("--json")) {
      console.log(JSON.stringify(rows.map(({ manifest, ...rest }) => ({ ...rest, mutations: manifest?.mutations.length ?? 0 })), null, 2));
      return;
    }

    const ready = rows.filter((row) => row.ready);
    const described = rows.filter((row) => row.leaves > 0 && row.described === row.leaves);
    const totalLeaves = rows.reduce((sum, row) => sum + row.leaves, 0);
    const totalDescribed = rows.reduce((sum, row) => sum + row.described, 0);

    console.log(`[manifest] ${totalDescribed}/${totalLeaves} mutation leaves carry a descriptor across ${rows.length} catalog(s)`);
    console.log(`[manifest] ${described.length} owner(s) fully described; ${ready.length} of those also have a qualifying oracle and are READY`);
    for (const row of ready) console.log(`[manifest] READY   ${row.capability.padEnd(32)} ${String(row.manifest?.mutations.length).padStart(4)} mutations  ${row.owner}`);
    const blocked = new Map<string, number>();
    for (const row of rows.filter((candidate) => !candidate.ready)) blocked.set(row.reason.split("—")[0]!.trim(), (blocked.get(row.reason.split("—")[0]!.trim()) ?? 0) + 1);
    for (const [reason, count] of [...blocked].sort((a, b) => b[1] - a[1])) console.log(`[manifest] BLOCKED ${String(count).padStart(4)} catalog(s): ${reason}`);

    if (!write) {
      console.log(`[manifest] dry run — pass --write to emit manifests for the ${ready.length} ready owner(s)`);
      return;
    }
    let written = 0;
    for (const row of ready) {
      const contribution = registry.contributions.find((entry) => entry.owner === row.owner);
      if (contribution === undefined || row.manifest === null) continue;
      const path = join(this.repoRoot, contribution.manifestPath);
      const parsed = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
      const all = (parsed.mutationManifests as MutationManifest[] | undefined) ?? [];
      const prior = all.find((entry) => entry.artifact === row.manifest!.artifact && entry.standard === row.manifest!.standard && entry.subset === row.manifest!.subset);
      const existing = all.filter((entry) => entry !== prior);
      // 🤝️MERGE, NEVER REPLACE. The generator derives STRUCTURE from the leaf descriptors — payload
      // schema, outcome classes, dispatch variant — but it knows nothing about SCOPE: which specific
      // oracle discharges a mutation, and which mutations the carrier provably cannot witness. That is
      // registration work, and a wholesale replace silently undid it: re-running this command flattened
      // `sequence`'s hand-scoped 4-carried/4-uncarried split back to eight undifferentiated mutations,
      // turning an honest partial into a claim of blanket coverage. Refined fields win over derived ones.
      const carried = new Map((prior?.mutations ?? []).map((mutation) => [mutation.id, mutation] as const));
      const merged = { ...row.manifest, mutations: row.manifest.mutations.map((mutation) => {
        const before = carried.get(mutation.id);
        if (before === undefined) return mutation;
        return {
          ...mutation,
          ...(before.oracleRequirements !== undefined ? { oracleRequirements: before.oracleRequirements } : {}),
          ...((before as { invariants?: unknown }).invariants !== undefined ? { invariants: (before as { invariants?: unknown }).invariants } : {}),
          ...((before as { carriers?: unknown }).carriers !== undefined ? { carriers: (before as { carriers?: unknown }).carriers } : {}),
          ...((before as { comparisonPipeline?: unknown }).comparisonPipeline !== undefined ? { comparisonPipeline: (before as { comparisonPipeline?: unknown }).comparisonPipeline } : {}),
        };
      }) };
      parsed.mutationManifests = [...existing, merged];
      parsed.schemaVersion = 2;
      writeFileSync(path, `${JSON.stringify(parsed, null, 2)}\n`);
      written += 1;
      console.log(`[manifest] wrote ${row.manifest.mutations.length} mutation(s) into ${contribution.manifestPath}`);
    }
    console.log(`[manifest] ${written} manifest(s) written`);
  }
}
//#endregion 🧭️Commands

//#region 🧹️Policy
/** 🧹️ Folder policy for this domain: the six generated output roots must never be committed. */
export function policy(): BreachRecord[] {
  const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..");
  const breaches: BreachRecord[] = [];
  for (const name of readdirSync(import.meta.dir)) {
    if (!["🧬️schema", "📇️registry", "📦️packages", "🧫️fixtures", "🧪️tests", "🧬️protocol", "🏃️runner", "🔮️oracle", "📜️script.ts", "📋️project.json", "🔌️nx-plugin.mjs", "AGENTS.md", "README.md", "node_modules"].includes(name)) {
      breaches.push({ id: "unknown-domain-child", kind: "testing/taxonomy", scope: `${DOMAIN_REL}/${name}`, summary: `Unexpected child ${name} in the testing domain root`, priority: "medium", reason: "The testing domain root holds its schema, registry, packages, fixtures, self-tests and routers — nothing else.", solution: "Move it into the owning child directory, or delete it." });
    }
  }
  const cachedInTree = existsSync(join(repoRoot, DOMAIN_REL, ".🧬semio"));
  if (cachedInTree) breaches.push({ id: "nested-cache", kind: "testing/taxonomy", scope: DOMAIN_REL, summary: "A nested .🧬semio cache exists inside the testing domain", priority: "high", reason: "Generated test state belongs only under the repository cache root.", solution: "Delete the nested cache." });
  return breaches;
}

export const policyFile = "📦️packages/🟦️typescript/📦️index.ts";
//#endregion 🧹️Policy

//#region 🚪️Entry
const router = new ScriptRouter(import.meta.dir)
  .register("discover", DiscoverScript)
  .register("contract", ContractScript)
  .register("oracle", OracleScript)
  .register("subject", SubjectScript)
  .register("parity", ParityScript)
  .register("run", RunScript)
  .register("test", RunScript)
  .register("report", ReportScript)
  .register("clean", CleanScript)
  .register("dependency", DependencyScript)
  .register("metrics", MetricsScript)
  .register("nx", NxScript)
  .register("doctor", DoctorScript)
  .register("inventory", InventoryScript)
  .register("fixture", FixtureScript)
  .register("probe", ProbeScript)
  .register("matrix", MatrixScript)
  .register("gc", GcScript)
  .register("gap", GapScript)
  .register("manifest", ManifestScript);

await runBundleScriptMain(router, import.meta.url);
//#endregion 🚪️Entry
