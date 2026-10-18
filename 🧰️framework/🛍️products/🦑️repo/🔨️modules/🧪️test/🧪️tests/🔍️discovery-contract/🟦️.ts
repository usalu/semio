/** 🔍️ Test platform: discovery, layout and openness laws
 *
 * The frozen vocabulary, discovery, clean safety, subject routing and selection, contribution ownership, projected vector storage and open/closed laws of the test platform.
 */

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { withBuildTestInvocationV1 } from "../../../📚️library/🏗️build/🧪️tests/📥️invocation/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { minimatch } from "minimatch";
import { clearContributionCache, isTestOraclePath, scanDeclaredDependencies } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { isBlockingBreach } from "../../../📚️library/🚨️policy/🏃️runner/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import oracleDirectoryCases from "../🧫️fixtures/🧭️contribution-directory-ownership/🔣️.json";
import { type TestCasePlan, type FeatureScenario } from "@semio-tech/framework/test/adapter";
import { CORE_COMPARISON_PROFILES, mutationVocabularyRequiresCatalog, discoverTestContributions, oracleImportsInProduction, cleanTestOutputs, discoverTestCases, isExcludedTestPath, markOutputDir, readOutputMarker, testCacheDir, testFilenameForKind, testLocationPath, testProjectName, testTaxonomy, caseContractBreaches, subjectRawInputsByScenario, makeAdapterContext } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exemptAreas, HOST_PROTOCOL_PARITY_CASE_DIR, implementationAreas, repoCases, repoContributions, repoRegistry, repoRoot, repoWideBreaches } from "../🧰️repository-scan/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧪️Tests
describe("🔣️ contract", () => {
  test("the taxonomy exposes every frozen test vocabulary key", () => {
    const taxonomy = testTaxonomy(repoRoot);
    expect(taxonomy.testsDirName).toBe("🧪️tests");
    expect(taxonomy.testFeatureFileKindId).toBe("gherkin-feature");
    expect(testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId)).toBe("🥒️.feature");
    expect(taxonomy.testAdapterFileKinds).toEqual({ "🦀️rust": "rust-source", "🟦️typescript": "typescript-source", "🐹️go": "go-source", "🐍️python": "python-source", "🔷️dotnet": "dotnet-source" });
    expect(Object.values(taxonomy.testAdapterFileKinds).map((kindId) => testFilenameForKind(taxonomy, kindId)).sort()).toEqual(["🐍️.py", "🐹️.go", "🔷️.cs", "🟦️.ts", "🦀️.rs"].sort());
    expect(taxonomy.testOutputChildDirs).toEqual(["work", "hosts", "oracles", "results", "diffs", "reports"]);
  });

  test("every exempt area is excluded by the discovery library itself, not by a caller's filter", () => {
    const areas = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areas as Record<string, string>;
    const exempt = Object.entries(areas).filter(([, state]) => state === "exempt").map(([area]) => area);
    expect(exempt.length).toBeGreaterThan(0);
    for (const area of exempt) {
      expect(isExcludedTestPath(repoRoot, area)).toBe(true);
      expect(isExcludedTestPath(repoRoot, `${area}/anything/below/it`)).toBe(true);
    }
  });

  test("no area is excluded merely for being legacy or mixed — only an exempt one is", () => {
    const areas = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areas as Record<string, string>;
    for (const [area, state] of Object.entries(areas)) {
      if (state === "exempt") continue;
      expect(isExcludedTestPath(repoRoot, area), `${area} (${state}) must not be excluded`).toBe(false);
    }
  });

  test("project names are deterministic and CLI-safe", () => {
    const owner = "🧪️synthetic/🔬️owner/📐️with-emoji";
    expect(testProjectName(owner, "a-case")).toBe(testProjectName(owner, "a-case"));
    expect(testProjectName(owner, "a-case")).toMatch(/^[a-z0-9-]+$/);
    expect(testProjectName(owner, "a-case")).not.toBe(testProjectName(`${owner}-other`, "a-case"));
  });
});

describe("🔍️ discovery and contract", () => {
  test("only standards or exact Gherkin owners require mutation catalogs", () => {
    const featureOwners = new Set(["framework/component"]);
    expect(mutationVocabularyRequiresCatalog("artifact/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations", new Set())).toBe(true);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧬️schema/🧬️mutations", featureOwners)).toBe(true);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧬️schema/🧬️mutations", new Set())).toBe(false);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧪️fixtures/demo/🧬️mutations", featureOwners)).toBe(false);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧪️fixtures/demo/🧬️mutations", new Set())).toBe(false);
  });

  test("discovery finds the committed cases and never returns a compose path", () => {
    const cases = repoCases;
    expect(cases.length).toBeGreaterThan(0);
    for (const area of exemptAreas()) expect(cases.every((entry) => !entry.owner.startsWith(`${area}/`) && !entry.caseDir.includes(`${area}/`)), `an exempt area leaked into discovery: ${area}`).toBe(true);
    expect(cases.some((entry) => entry.case === basename(HOST_PROTOCOL_PARITY_CASE_DIR))).toBe(true);
  });

  // ⏱️ One fresh repository walk compared with the file's own discovery: the second walk the law needs, not three.
  test(
    "discovery is idempotent",
    () => {
      expect(JSON.stringify(discoverTestCases(repoRoot))).toBe(JSON.stringify(repoCases));
    },
    30_000,
  );

  test("every committed case satisfies the frozen contract's blocking rules", () => {
    expect([...caseContractBreaches(repoRoot, repoCases, repoRegistry), ...repoWideBreaches].filter(isBlockingBreach).map((breach) => `${breach.kind}:${breach.scope}:${breach.summary}`)).toEqual([]);
  }, 30_000);

});

describe("🧹️ clean safety", () => {
  test("removes marked outputs, never unmarked directories, and reports identically in dry mode", () => {
    const work = testCacheDir(repoRoot, "work");
    const marked = join(work, "🧪️clean-self-test-marked");
    const unmarked = join(work, "🧪️clean-self-test-unmarked");
    rmSync(marked, { recursive: true, force: true });
    rmSync(unmarked, { recursive: true, force: true });
    markOutputDir(repoRoot, marked, { testId: "self-test::clean", cacheKey: "self-test" });
    writeFileSync(join(marked, "artifact.bin"), "x");
    mkdirSync(unmarked, { recursive: true });
    writeFileSync(join(unmarked, "🚨️sentinel-do-not-delete"), "sentinel");

    expect(readOutputMarker(repoRoot, marked)?.testId).toBe("self-test::clean");
    expect(readOutputMarker(repoRoot, unmarked)).toBeNull();

    const dry = cleanTestOutputs(repoRoot, { dry: true });
    expect(dry.removals.some((row) => row.path.endsWith("🧪️clean-self-test-marked"))).toBe(true);
    expect(dry.skippedUnmarked.some((path) => path.endsWith("🧪️clean-self-test-unmarked"))).toBe(true);
    expect(existsSync(marked)).toBe(true);

    const applied = cleanTestOutputs(repoRoot, { dry: false });
    expect(applied.removals.map((row) => row.path).sort()).toEqual(dry.removals.map((row) => row.path).sort());
    expect(existsSync(marked)).toBe(false);
    expect(existsSync(join(unmarked, "🚨️sentinel-do-not-delete"))).toBe(true);

    rmSync(unmarked, { recursive: true, force: true });
  }, 60_000);

  test("no tracked fixture, source file or compose path is ever a clean candidate", () => {
    const sentinelFixture = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧫️fixtures/📡️protocol-vector.txt");
    const before = readFileSync(sentinelFixture, "utf8");
    const report = cleanTestOutputs(repoRoot, { dry: true });
    for (const row of report.removals) {
      expect(row.path.startsWith(".🧬semio/🦑️repo/⚡️cache/tests")).toBe(true);
      for (const area of exemptAreas()) expect(row.path.includes(`${area}/`), `clean candidate inside the exempt area ${area}`).toBe(false);
    }
    expect(readFileSync(sentinelFixture, "utf8")).toBe(before);
    expect(report.removals.some((row) => row.path.endsWith("compose.yaml"))).toBe(false);
    expect(existsSync(join(repoRoot, "🌎️hub/compose.yaml"))).toBe(true);
  });

  test("marking a directory outside the test cache root is refused", () => {
    expect(() => markOutputDir(repoRoot, join(repoRoot, "✏️s"), { testId: "x", cacheKey: "y" })).toThrow();
  });
});

describe("📥️ subject raw routing", () => {
  const subject = (scenario: string, implementation: "rust" | "typescript", status: "passed" | "failed", rawPath?: string) =>
    ({ testId: `o::c::${scenario}::${implementation}::subject`, owner: "o", case: "c", scenario, implementation, role: "subject" as const, level: "quick" as const, status, durationMs: 1, output: { rawHash: "", projectionHash: "", rawPath }, diagnostics: [] });

  test("every passed subject raw output is routed to its own scenario, never folded into one map per case", () => {
    const routed = subjectRawInputsByScenario([
      subject("s1", "rust", "passed", "/r/s1.rust"),
      subject("s1", "typescript", "passed", "/r/s1.ts"),
      subject("s2", "rust", "passed", "/r/s2.rust"),
      subject("s3", "rust", "failed", "/r/s3.rust"),
      subject("s4", "rust", "passed"),
      { ...subject("s5", "rust", "passed", "/r/s5.oracle"), role: "oracle" as const },
    ]);
    expect(routed).toEqual({ s1: { rust: "/r/s1.rust", typescript: "/r/s1.ts" }, s2: { rust: "/r/s2.rust" } });
  });

  test("an oracle scenario reads exactly its own subject scenario's bytes and refuses a scenario without any", ()=>withBuildTestInvocationV1(async invocation=>{
    const dir = mkdtempSync(join(tmpdir(), "semio-raw-routing-"));
    try {
      writeFileSync(join(dir, "s1.raw"), "first");
      writeFileSync(join(dir, "s2.raw"), "second");
      const plan = { workDir: join(dir, "work"), artifactDir: join(dir, "artifacts"), inputs: [], subjectRawInputs: { s1: { rust: join(dir, "s1.raw") }, s2: { rust: join(dir, "s2.raw") } } } as unknown as TestCasePlan;
      const phaseControl=createIssuedResourceControlV1(invocation,"test:subject-raw-fixture",0),context = (id: string) => makeAdapterContext(dir, plan, { id, name: id, steps: [] } as unknown as FeatureScenario, "oracle",invocation,phaseControl);
      expect(new TextDecoder().decode(context("s1").subjectRawBytes("rust"))).toBe("first");
      expect(new TextDecoder().decode(context("s2").subjectRawBytes("rust"))).toBe("second");
      expect(() => context("s3").subjectRawBytes("rust")).toThrow("scenario s3 has no raw subject output from rust");
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }));
});

describe("🧭️ contribution directory ownership", () => {
  test("the canonical oracle collection matches neutral cases and the minimatch oracle", async () => {
    const taxonomy = testTaxonomy(repoRoot);
    expect(oracleDirectoryCases.directoryName).toBe(taxonomy.testOraclesDirName);
    expect(oracleDirectoryCases.cases.map(({ path }) => isTestOraclePath(taxonomy, path))).toEqual(oracleDirectoryCases.cases.map(({ owned }) => owned));
    expect(oracleDirectoryCases.cases.map(({ path }) => minimatch(path, `**/${oracleDirectoryCases.directoryName}/**`, { dot: true }))).toEqual(oracleDirectoryCases.cases.map(({ owned }) => owned));
  });

  test("discovery and dependency ownership use the canonical collection without aliases", () => {
    const root = mkdtempSync(join(tmpdir(), "contribution-directory-ownership-"));
    const vocabulary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
    const candidates = [
      { owner: "🎠️kernel", directory: "🔮️oracles", dependency: "selected-reference", owned: true },
      { owner: "🎠️kernel", directory: "🔮️oracle", dependency: "singular-reference", owned: false },
      { owner: "🧩️other", directory: "🔮️oracles", dependency: "other-reference", owned: true },
      { owner: "🧩️other", directory: "🧪️oracle", dependency: "test-emoji-reference", owned: false },
    ];
    try {
      const taxonomy = JSON.parse(readFileSync(join(repoRoot, vocabulary), "utf8"));
      mkdirSync(join(root, vocabulary, ".."), { recursive: true });
      writeFileSync(join(root, vocabulary), JSON.stringify(taxonomy));
      for (const candidate of candidates) {
        const dir = join(root, candidate.owner, candidate.directory);
        mkdirSync(dir, { recursive: true });
        writeFileSync(join(dir, "🔣️.json"), JSON.stringify({ schemaVersion: 2, oracles: [{ id: candidate.dependency, package: "semver", ecosystem: "javascript", capabilities: [], comparisonProfiles: [], license: "ISC", testOnly: true }] }));
        writeFileSync(join(dir, "package.json"), JSON.stringify({ dependencies: { [candidate.dependency]: "1.0.0" } }));
        writeFileSync(join(dir, "🟦️.ts"), "import semver from \"semver\";\n");
      }
      const contributions = discoverTestContributions(root);
      expect(contributions.map(({ manifestPath }) => manifestPath).sort()).toEqual(candidates.filter(({ owned }) => owned).map(({ owner, directory }) => `${owner}/${directory}/🔣️.json`).sort());
      const dependencies = scanDeclaredDependencies(root);
      for (const candidate of candidates) expect(dependencies.find(({ name }) => name === candidate.dependency)?.productionReachable).toBe(!candidate.owned);
      expect([...new Set(oracleImportsInProduction(root).map(({ path }) => path))].sort()).toEqual(candidates.filter(({ owned }) => !owned).map(({ owner, directory }) => `${owner}/${directory}/🟦️.ts`).sort());
      writeFileSync(join(root, "🎠️kernel/🔮️oracles/🔣️.json"), "{ malformed");
      clearContributionCache(root);
      expect(discoverTestContributions(root).map(({ owner }) => owner)).toEqual(["🧩️other"]);
      expect(scanDeclaredDependencies(root).find(({ name }) => name === "selected-reference")?.productionReachable).toBe(false);
      expect(oracleImportsInProduction(root).some(({ path }) => path.startsWith("🎠️kernel/🔮️oracles/"))).toBe(false);
    } finally {
      clearContributionCache(root);
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("root dependency discovery and classification honor the same neutral owner contract", async () => {
    const inventory = await import("../../../📚️library/🕸️dependencies/📇️inventory/🟦️.ts");
    const api = { discover: inventory.dependencyDiscoverContributionManifests, classify: inventory.dependencyClassifyOracleEntry };
    const oracleOnlyDomain = { directoryNames: [oracleDirectoryCases.directoryName], domainPath: String(testTaxonomy(repoRoot).testDomainPath) };
    const root = mkdtempSync(join(tmpdir(), "root-contribution-directory-"));
    try {
      for (const row of oracleDirectoryCases.cases) {
        const directory = row.path.slice(0, row.path.lastIndexOf("/"));
        mkdirSync(join(root, directory), { recursive: true });
        writeFileSync(join(root, directory, "🔣️.json"), "{}\n");
        const entry: Parameters<typeof api.classify>[0] = { ecosystem: "js", name: "reference", version: "1.0.0", productionReachable: true, kinds: ["production-runtime"], users: [row.path], declarations: [{ user: row.path, version: "1.0.0", kind: "production-runtime" }] };
        api.classify(entry, ["reference"], oracleOnlyDomain);
        expect(entry.kinds, row.path).toEqual([row.owned ? "test-oracle" : "production-runtime"]);
      }
      const expected = oracleDirectoryCases.cases.filter(({ owned }) => owned).map(({ path }) => `${path.slice(0, path.lastIndexOf("/"))}/🔣️.json`).sort();
      expect(api.discover(root, oracleDirectoryCases.directoryName, "🔣️.json").sort()).toEqual(expected);
      rmSync(join(root, "🎠️kernel/🔮️oracles/🔣️.json"));
      expect(api.discover(root, oracleDirectoryCases.directoryName, "🔣️.json").sort()).toEqual(expected.filter((path) => path !== "🎠️kernel/🔮️oracles/🔣️.json"));
      const absent: Parameters<typeof api.classify>[0] = { ecosystem: "js", productionReachable: true, name: "reference", version: "1.0.0", kinds: ["production-runtime"], users: ["🎠️kernel/🔮️oracles/package.json"], declarations: [{ user: "🎠️kernel/🔮️oracles/package.json", version: "1.0.0", kind: "production-runtime" }] };
      api.classify(absent, ["reference"], oracleOnlyDomain);
      expect(absent.kinds).toEqual(["test-oracle"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("the handpicked kernel oracle remains discoverable at runtime", () => {
    const contribution = repoContributions.find(({ owner }) => owner === "🧰️framework/🔨️modules/🎠️kernel");
    expect(contribution?.manifestPath).toBe("🧰️framework/🔨️modules/🎠️kernel/🔮️oracles/🔣️.json");
    expect(contribution?.oracles.map(({ id }) => id)).toContain("semver");
  }, 60_000);
});

describe("🧩️ open/closed", () => {
  test("the framework's own registry names no plugin, product or file format", () => {
    const taxonomy = testTaxonomy(repoRoot);
    const core = JSON.parse(readFileSync(join(repoRoot, testLocationPath(taxonomy, taxonomy.testOracleRegistryLocation)), "utf8")) as { oracles?: unknown[]; comparisonProfiles?: unknown[]; oracleHostPackages?: unknown[] };
    expect(core.oracles ?? []).toEqual([]);
    expect(core.comparisonProfiles ?? []).toEqual([]);
    expect(core.oracleHostPackages ?? []).toEqual([]);
  });

  test("every oracle and every format-specific profile arrives as an owner contribution", () => {
    const contributions = repoContributions;
    expect(contributions.length).toBeGreaterThan(0);
    const registry = repoRegistry;
    const contributedOracleIds = new Set(contributions.flatMap((entry) => entry.oracles.map((oracle) => oracle.id)));
    for (const oracle of registry.oracles) expect(contributedOracleIds.has(oracle.id), `${oracle.id} is not contributed by any owner`).toBe(true);
    const coreIds = new Set(CORE_COMPARISON_PROFILES.map((spec) => spec.id));
    for (const spec of registry.comparisonProfiles) {
      if (coreIds.has(spec.id)) continue;
      expect(contributions.some((entry) => entry.comparisonProfiles.some((candidate) => candidate.id === spec.id)), `${spec.id} is not contributed by any owner`).toBe(true);
    }
  }, 30_000);

  test("the framework's own comparison profiles are domain-neutral", () => {
    for (const spec of CORE_COMPARISON_PROFILES) {
      expect(spec.id).not.toMatch(/pdf|png|gif|zip|wav|csv|raster|archive|audio|mesh|tabular/);
    }
  });

  test("the framework Rust host declares no dependency at all", () => {
    const manifest = readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml"), "utf8");
    const dependencyTable = manifest.split(/^\[dependencies\]$/m)[1] ?? "";
    expect(dependencyTable.split(/\r?\n/).filter((line) => /^[a-z0-9_-]+\s*=/.test(line.trim()))).toEqual([]);
  });

  test("a Rust host build resolves Cargo's executable before the finite scenario budget starts", async () => {
    const ts = await import("typescript");
    const hostPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts");
    const executionPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts");
    const hostSource = ts.createSourceFile(hostPath, readFileSync(hostPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const executionSource = ts.createSourceFile(executionPath, readFileSync(executionPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const parser = hostSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "rustHostExecutableFromCargo");
    const execute = executionSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "executeOne");
    expect(parser).toBeDefined();
    expect(execute).toBeDefined();
    const compiled = ts.transpileModule(parser!.getText(hostSource).replace(/^export\s+/u, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
    const resolveExecutable = new Function(`${compiled}\nreturn rustHostExecutableFromCargo;`)() as (stdout: string) => string | null;
    const cargoOutput = [
      JSON.stringify({ reason: "compiler-artifact", target: { name: "dependency", kind: ["lib"] }, executable: null }),
      JSON.stringify({ reason: "compiler-artifact", target: { name: "host", kind: ["bin"] }, executable: "/tmp/target/aarch64/debug/host" }),
    ].join("\n");
    expect(resolveExecutable(cargoOutput)).toBe("/tmp/target/aarch64/debug/host");
    const executeText = execute!.getText(executionSource);
    expect(executeText).toContain("budgetMs: buildBudgetMs()");
    expect(executeText).toContain("budgetMs: testLevelBudgetMs(level)");
    expect(executeText.indexOf("budgetMs: buildBudgetMs()")).toBeLessThan(executeText.indexOf("budgetMs: testLevelBudgetMs(level)"));
  });

  test("the framework test domain's sources name no implementation area", () => {
    const domain = join(repoRoot, testTaxonomy(repoRoot).testDomainPath);
    for (const file of ["📜️script.ts", "📦️packages/🟦️typescript/🟦️.ts", "📡️protocol/🦀️.rs", "🏃️runner/🦀️.rs"]) {
      const content = readFileSync(join(domain, file), "utf8");
      // 🧭️Which areas are implementations is taxonomy vocabulary; this test names none of them.
      for (const area of implementationAreas()) expect(content.includes(`${area}/`), `${file} names the implementation area ${area}`).toBe(false);
    }
  });

  test("the root script names neither the test module's location nor its phase vocabulary", () => {
    const root = readFileSync(join(repoRoot, "📜️script.ts"), "utf8");
    expect(root.includes(testTaxonomy(repoRoot).testDomainPath)).toBe(false);
    expect(root).not.toMatch(/LEVELLESS_PHASES|composeProjectNames/);
  });
});
//#endregion 🧪️Tests

describe("🧪️ projected vector storage", () => {
  test("profile storage never becomes an executable Nx test project", async () => {
    const artifacts = repoTestArtifactEnvironment(repoRoot, "projected-vector-storage").SEMIO_TEST_ARTIFACT_DIR!;
    const root = mkdtempSync(join(artifacts, "projected-nx-"));
    const taxonomy = testTaxonomy(repoRoot);
    const featureFilename = testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId);
    try {
      const taxonomyPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
      mkdirSync(join(taxonomyPath, ".."), { recursive: true });
      writeFileSync(taxonomyPath, readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")));
      const executable = join(root, "artifact", taxonomy.testsDirName, "🦠️mutate-thing");
      const projected = join(root, "artifact", taxonomy.testsDirName, "🪆️1-any", "🪄️change-value", "🧪️changes-the-value");
      mkdirSync(executable, { recursive: true });
      mkdirSync(projected, { recursive: true });
      writeFileSync(join(executable, featureFilename), "Feature: executable\n");
      writeFileSync(join(projected, featureFilename), "Feature: must stay storage\n");
      const { discoverCaseDirs } = await import("../../🟨️.mjs");
      expect(discoverCaseDirs(root)).toEqual([`artifact/${taxonomy.testsDirName}/🦠️mutate-thing`]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("🔬️ subject selection", () => {
  test("an oracle implemented in the case's own adapter is never also dispatched as that case's subject", async () => {
    const { oracleDecision, subjectImplementations } = await import("../../⚖️parity/📋️orchestration/🟦️.ts");
    const cases = new Map(repoCases.map((entry) => [entry.caseDir, entry] as const));
    const hosting = repoRegistry.oracles.filter((oracle) => oracle.hostPath !== undefined && cases.has(oracle.hostPath));
    expect(hosting.length).toBeGreaterThan(0);
    for (const oracle of hosting) {
      const discovered = cases.get(oracle.hostPath!)!;
      const decision = oracleDecision(repoRoot, discovered, "exhaustive");
      if (decision.implementation === null) continue;
      expect(decision.hostedByCase, `${oracle.id} is hosted by ${discovered.caseDir}`).toBe(true);
      expect(subjectImplementations(repoRoot, discovered, decision, ["rust", "typescript", "python", "go", "dotnet"])).not.toContain(decision.implementation);
    }
  }, 300_000);

  test("a package of an enclosing framework never makes a nested product's reference adapter a subject", async () => {
    const { oracleDecision, subjectImplementations } = await import("../../⚖️parity/📋️orchestration/🟦️.ts");
    const product = "🧰️framework/🛍️products/🦑️repo";
    const hosted = repoCases.filter((entry) => entry.owner.startsWith(`${product}/`) && entry.adapters.typescript !== undefined && entry.adapters.rust !== undefined);
    let referenceOnly = 0;
    for (const discovered of hosted) {
      const decision = oracleDecision(repoRoot, discovered, "exhaustive");
      if (decision.implementation !== "typescript") continue;
      const segments = discovered.owner.split("/");
      const within = Array.from({ length: segments.length - product.split("/").length + 1 }, (_, index) => segments.slice(0, segments.length - index).join("/"));
      const ships = within.some((dir) => existsSync(join(repoRoot, dir, "📦️packages", "🟦️typescript")));
      if (!ships) referenceOnly += 1;
      expect(subjectImplementations(repoRoot, discovered, decision, ["rust", "typescript"]), discovered.caseDir).toEqual(ships ? ["rust", "typescript"] : ["rust"]);
    }
    expect(referenceOnly).toBeGreaterThan(0);
  }, 300_000);
});
