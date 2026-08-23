#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧪️ Router of the repository testing domain:
//   bun ./📜️script.ts <discover|contract|oracle|subject|parity|run|report|clean|dependency|nx|doctor> [args…]

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, join, relative, sep } from "node:path";
import { type BreachRecord, type TestLevel, Script, ScriptRouter, TEST_LEVELS, formatBreachReport, getRepoMetaDir, resolveTestLevel, runBundleScriptMain, runProbe, testLevelBudgetMs } from "../📚️library/📦️packages/🟦️typescript/📦️index.ts";
import {
  type ClassifiedDependency,
  type CoverageMetrics,
  type ImplementationCoverage,
  type ComparisonProfile,
  type DependencyEcosystem,
  type DiscoveredCase,
  type Implementation,
  type TestResult,
  type TestRole,
  agentCacheRoot,
  buildCasePlan,
  classifyLegacyKind,
  cleanTestOutputs,
  computeCoverageMetrics,
  discoverTestCases,
  dotnetPackageReferences,
  evaluateCrossSubjectParity,
  evaluateParity,
  enforceMetricGates,
  formatMetrics,
  formatCleanReport,
  isProductionClass,
  loadOracleRegistry,
  markOutputDir,
  markRunComplete,
  planExecution,
  pythonRuntimeImports,
  ratchetDependencies,
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
  const oracleByPackage = new Map(registry.oracles.map((entry) => [entry.package, entry]));
  const classified: ClassifiedDependency[] = baselineRaw.entries.map((entry) => {
    const oracle = oracleByPackage.get(entry.name);
    const oracleIds = oracle ? [oracle.id] : (entry.oracleIds ?? []);
    const kinds = [...new Set(entry.kinds.map((kind) => (["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"].includes(kind) ? (kind as ClassifiedDependency["kinds"][number]) : classifyLegacyKind(kind, oracleIds))))];
    return {
      ecosystem: entry.ecosystem as DependencyEcosystem,
      name: entry.name,
      version: entry.version,
      kinds,
      users: entry.users,
      productionReachable: entry.productionReachable ?? kinds.some(isProductionClass),
      oracleIds: oracleIds.length > 0 ? oracleIds : undefined,
      capabilities: oracle?.capabilities ?? entry.capabilities,
    };
  });
  for (const oracle of registry.oracles) {
    if (classified.some((entry) => entry.name === oracle.package)) continue;
    classified.push({ ecosystem: oracle.ecosystem === "javascript" ? "js" : (oracle.ecosystem as DependencyEcosystem), name: oracle.package, version: oracle.version ?? "*", kinds: ["test-oracle"], users: [oracle.hostPath ?? DOMAIN_REL], productionReachable: false, oracleIds: [oracle.id], capabilities: oracle.capabilities });
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
//#endregion 🎛️Selection

//#region 🏗️Hosts
/** 🏗️ One materialized native entrypoint: where it lives and how it is launched. */
type MaterializedHost = Readonly<{ command: string; args: readonly string[]; cwd: string; env: NodeJS.ProcessEnv; hostDir: string | null }>;

function hostDirFor(repoRoot: string, discovered: DiscoveredCase, role: TestRole, implementation: Implementation): string {
  const dir = join(testCacheDir(repoRoot, "hosts"), `${discovered.projectName}-${role}-${implementation}`);
  // 🧾️ A generated host is deletable state, so it carries the same ownership marker as every other
  // output root — an unmarked directory is never removed by `clean test`.
  markOutputDir(repoRoot, dir, { testId: `${discovered.owner}::${discovered.case}`, cacheKey: `${role}:${implementation}` });
  markRunComplete(dir);
  return dir;
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

/** 🔮️ Whether this case's registered oracle is a Rust one, which is what enables the host's `oracles` feature. */
function needsRustOracles(repoRoot: string, discovered: DiscoveredCase): boolean {
  const oracleId = readFileSync(join(repoRoot, discovered.featurePath), "utf8").match(/@oracle-([a-z0-9-]+)/)?.[1];
  if (oracleId === undefined) return false;
  return loadOracleRegistry(repoRoot).oracles.some((entry) => entry.id === oracleId && entry.ecosystem === "rust");
}

/** 🦀️ Materializes a standalone cache-local integration crate that links the adapter and the host support crate by path. */
function materializeRustHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "rust");
  const adapterAbs = join(repoRoot, discovered.adapters.rust!);
  const sut = rustSutCrate(repoRoot, discovered);
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
      `semio-repo-test-host = { path = ${JSON.stringify(join(repoRoot, RUST_PACKAGE_REL))}${needsRustOracles(repoRoot, discovered) ? ', features = ["oracles"]' : ""} }`,
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
  };
}

/** 🐹️ Materializes a cache-local Go module whose generated entrypoint delegates to the committed adapter. */
function materializeGoHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "go");
  const adapterAbs = join(repoRoot, discovered.adapters.go!);
  writeFileSync(join(dir, "go.mod"), ["// 🤖️ Generated — safe to delete, never commit.", "module semio.test/host", "", "go 1.23", "", "require semio.tech/repo/test v0.0.0", "", `replace semio.tech/repo/test => ${join(repoRoot, GO_PACKAGE_REL)}`, ""].join("\n"));
  writeFileSync(join(dir, "adapter.go"), readFileSync(adapterAbs, "utf8").replace(/^package\s+\w+/m, "package main"));
  writeFileSync(join(dir, "main.go"), ["// 🤖️ Generated native entrypoint.", "package main", "", 'import host "semio.tech/repo/test"', "", "func main() {", "\thost.RunMain(Adapter())", "}", ""].join("\n"));
  return { command: "go", args: ["run", ".", "--plan", planPath, "--out", outPath], cwd: dir, env: { ...process.env, GOFLAGS: "-mod=mod", GOWORK: "off" }, hostDir: dir };
}

/** 🐍️ Runs the committed adapter through the owned Python host — never through the compose-scoped root discovery config. */
function materializePythonHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "python");
  return {
    command: process.env.SEMIO_PYTHON ?? "python3",
    args: [join(repoRoot, PYTHON_PACKAGE_REL, "🐍️host.py"), "--plan", planPath, "--out", outPath, "--adapter", join(repoRoot, discovered.adapters.python!)],
    cwd: repoRoot,
    env: { ...process.env, PYTHONDONTWRITEBYTECODE: "1", PYTHONPYCACHEPREFIX: join(agentCacheRoot(repoRoot), "pycache") },
    hostDir: dir,
  };
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
  };
}

/** 🏗️ Resolves the launch recipe for one implementation, materializing a cache-local entrypoint when the native framework needs one. */
function materializeHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, implementation: Implementation, planPath: string, outPath: string): MaterializedHost {
  switch (implementation) {
    case "typescript":
      return { command: "bun", args: [join(repoRoot, TS_PACKAGE_REL, "🏃️host.ts"), "--plan", planPath, "--out", outPath, "--adapter", join(repoRoot, discovered.adapters.typescript!)], cwd: repoRoot, env: process.env, hostDir: null };
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
function executeOne(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, role: TestRole, implementation: Implementation): PhaseOutcome {
  const { plan, missingFixtures, planPath } = planExecution(repoRoot, discovered, level, role, implementation);
  const problems = missingFixtures.map((uri) => `${discovered.caseDir}: unresolved fixture ${uri}`);
  if (plan.scenarios.length === 0) return { results: [], problems };
  rmSync(plan.resultsPath, { force: true });
  const host = materializeHost(repoRoot, discovered, role, implementation, planPath, plan.resultsPath);
  const probe = runProbe(host.command, [...host.args], { cwd: host.cwd, env: host.env, budgetMs: testLevelBudgetMs(level) });
  if (probe.stdout.trim() !== "") console.log(probe.stdout.trimEnd());
  const { results, problems: readProblems } = readResults(plan.resultsPath);
  if ((probe.status ?? 1) !== 0 && results.length === 0) {
    problems.push(`${discovered.caseDir}: ${implementation} ${role} host exited ${probe.status} without emitting results`);
    if (probe.stderr.trim() !== "") problems.push(probe.stderr.trimEnd());
  }
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
  let scenarioCount = 0;

  for (const discovered of cases) {
    const decision = oracleDecision(repoRoot, discovered, level);
    if (decision.problem !== null) problems.push(decision.problem);
    const caseResults: TestResult[] = [];

    if (phases.includes("oracle") && decision.implementation !== null) {
      const outcome = executeOne(repoRoot, discovered, level, "oracle", decision.implementation);
      caseResults.push(...outcome.results);
      problems.push(...outcome.problems);
    }
    if (phases.includes("subject")) {
      for (const implementation of selectImplementations(discovered, rest)) {
        const outcome = executeOne(repoRoot, discovered, level, "subject", implementation);
        caseResults.push(...outcome.results);
        problems.push(...outcome.problems);
      }
    }
    scenarioCount += new Set(caseResults.map((result) => result.scenario)).size;
    allResults.push(...caseResults);

    if (!phases.includes("oracle") || !phases.includes("subject") || caseResults.length === 0) continue;

    const diffDir = testCacheDir(repoRoot, "diffs");
    mkdirSync(diffDir, { recursive: true });
    if (decision.implementation !== null) {
      const { verdicts, unmatched } = evaluateParity(decision.comparison, caseResults);
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
    const crossPairs = evaluateCrossSubjectParity(decision.comparison, caseResults);
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

  const summary = summarizeRun(level, cases.length, scenarioCount, allResults, parity, problems);
  const dir = reportsDir(repoRoot);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "📊️summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  writeFileSync(join(dir, "📤️results.jsonl"), allResults.map((result) => JSON.stringify(result)).join("\n") + (allResults.length > 0 ? "\n" : ""));
  writeFileSync(join(dir, "📋️junit.xml"), renderJUnit(allResults));
  const metrics = computeCoverageMetrics(repoRoot, cases, allResults, parity, loadClassifiedBaseline(repoRoot));
  writeFileSync(join(dir, "📈️metrics.json"), `${JSON.stringify(metrics, null, 2)}\n`);
  markRunComplete(dir);

  console.log(`[test] level=${level} cases=${summary.cases} executed=${summary.executed} passed=${summary.passed} failed=${summary.failed} errored=${summary.errored} parity=${parity.filter((row) => row.equal).length}/${parity.length}`);
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

/** 🔮️ The oracle phase — proves the reference library actually supports the case before any local code exists. */
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

/** 🧹️ Marker-guarded removal of generated test state. Never descends into `compose/`. */
class CleanScript extends Script {
  run(segments: string[]): void {
    const dry = segments.includes("--dry");
    const stale = segments.includes("--stale");
    const liveTestIds = new Set(discoverTestCases(this.repoRoot).map((entry) => `${entry.owner}::${entry.case}`));
    const report = cleanTestOutputs(this.repoRoot, { dry, stale, liveTestIds: stale ? liveTestIds : undefined });
    console.log(formatCleanReport(this.repoRoot, report));
    const leaked = report.removals.filter((row) => row.path.includes("compose/"));
    if (leaked.length > 0) {
      console.error(`[clean test] refusing: ${leaked.length} candidate(s) resolved inside compose/`);
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

    const verdict = ratchetDependencies(sorted, sorted, registry);
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
  .register("doctor", DoctorScript);

await runBundleScriptMain(router, import.meta.url);
//#endregion 🚪️Entry
