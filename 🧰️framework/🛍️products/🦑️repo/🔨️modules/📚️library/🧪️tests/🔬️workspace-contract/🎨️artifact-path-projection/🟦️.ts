import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import toml from "@iarna/toml";
import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { chmodSync, existsSync, lstatSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, posix, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, validateTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { fixedSourceDispositionDecision } from "../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { inspectRustModuleGraphFacts } from "../../../🔍️discovery/🌐️rust-module-graph/🟦️.ts";
import { semanticPathProjectionAuthority } from "../../../🔍️discovery/🪞️path-projection/🟦️.ts";
import { type SemanticProjectionAuthorityNode } from "../../../🔍️discovery/🧱️path-projection-contracts/🟦️.ts";
import { type Taxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { ownedFilePaths } from "../../🔍️filesystem/🟦️.ts";
import { normalizationEngineDeclarations } from "../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { ARTIFACT_PROJECTION_GOLDEN, ARTIFACT_PROJECTION_RUN_PARENT, type ArtifactProducerInput, type ArtifactProjectionGoldenEntry, CAD_LIVE_BINDINGS, DRAW_SOURCE_SCENARIO, DRAW_SOURCE_SCENARIO_ROOT, DRAW_SUBMITTED_PROOF, type DrawSourceScenario, NORMALIZATION_RUN_DIRECTORY, NORMALIZATION_RUN_ROOT, NORMALIZATION_SCHEMA_REL, artifactProjectionCaptureSubmittedPlan, artifactProjectionFixtureLocations, artifactProjectionFixturePath, artifactProjectionIgnoreResidue, artifactProjectionProducerInput, artifactProjectionProducerInputs, artifactProjectionProgressRecord, artifactProjectionProgressSampler, artifactProjectionRecordedStage, artifactProjectionSourceFiles, artifactProjectionSubmittedPlanBytes, assertArtifactProjectionSingleCaseRoute, cadProjectionLiveSources, drawScenarioTaxonomy, drawSourceScenarioContent, normalizationFixture, normalizationRetainedRunParent, projectionAuthorityNodes, projectionByteSort, projectionGolden, retainArtifactProjectionFixture } from "../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

/** 🗺️ Selects the authority a golden projection is planned against: the live taxonomy for CAD, the scenario overlay for the authored Draw bundle. */
function projectionTaxonomy(projection: ArtifactProjectionGoldenEntry): Taxonomy {
  return projection.contractId === "artifact-editor-command-bundle-v1" ? drawScenarioTaxonomy(loadTaxonomy()) : loadTaxonomy();
}

function projectionAuthority(projection: ArtifactProjectionGoldenEntry, nodes = projectionAuthorityNodes(projection), occupiedPaths: readonly string[] = []) {
  return semanticPathProjectionAuthority({
    artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")),
    contractId: projection.contractId,
    nodes,
    occupiedPaths,
    sourceRoot: projection.sourceRoot,
  }, projectionTaxonomy(projection));
}

describe.if(testLevelAtLeast("long"))("artifact path projection authority", () => {
  test("authored Draw preflight context preserves absent evidence and foreign ticket inputs", async () => {
    const root = join(DRAW_SOURCE_SCENARIO_ROOT, "🛫️preflight-context"), text = readFileSync(join(root, "🔣️.json"), "utf8"), vector = JSON.parse(text);
    
    expect(vector["schemaVersion"]).toEqual(1);expect(vector["contractId"]).toEqual("draw-preflight-evidence-context-v1");expect(vector["ticketDir"]).toEqual(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/DRAW-SOURCE-SCENARIOS");expect(vector["transactionDirectory"]).toEqual("🧾️taxonomy-transaction");expect(vector["planPath"]).toEqual("📊️taxonomy-plan/🔣️.json");expect(vector["sourcePath"]).toEqual("🧪️tests/🦀️fixture.rs");expect(vector["foreignPath"]).toEqual("📝️link-target.md");expect(vector["unrelatedPath"]).toEqual("📓️unrelated/📝️.md");expect(vector["cases"]).toEqual([{"id":"absent-transaction-and-plan","transactionExists":false,"planExists":false,"sourceDigestUnchanged":true},{"id":"existing-transaction-only","transactionExists":true,"planExists":false,"sourceDigestUnchanged":false},{"id":"existing-exact-plan-only","transactionExists":false,"planExists":true,"sourceDigestUnchanged":true},{"id":"existing-transaction-and-exact-plan","transactionExists":true,"planExists":true,"sourceDigestUnchanged":false}]);
    
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(text, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    const ts = await import("typescript"), source = normalizationEngineDeclarations(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization"));
    const syntax = ts.createSourceFile("🟦️.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const names = ["sourceTreeDigest", "inheritReferenceInventoryContext", "inventoryWithoutTransactionEvidence", "repositoryReferenceCandidatePaths"];
    const functions = names.map((name) => {
      const nodes = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === name);
      expect(nodes).toHaveLength(1);
      return nodes[0]!.getText(syntax);
    });
    const code = `${functions.join("\n")}\nreturn { sourceTreeDigest, inventoryWithoutTransactionEvidence, repositoryReferenceCandidatePaths };`;
    for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      for (const row of vector.cases) {
        const context = new WeakMap<object, unknown>(), transactionRoot = `${vector.ticketDir}/${vector.transactionDirectory}`, exactPlan = `${vector.ticketDir}/${vector.planPath}`;
        const foreign = `${vector.ticketDir}/${vector.foreignPath}`, unrelated = `${vector.ticketDir}/${vector.unrelatedPath}`, transactionFile = `${transactionRoot}/🔣️.json`;
        const ticketPaths = [foreign, unrelated, ...(row.transactionExists ? [transactionFile] : []), ...(row.planExists ? [exactPlan] : [])];
        const engine = new Function("referenceInventoryContexts", "canonicalJson", "sha256", "checkCancellation", "isExcluded", "historicalDocumentEvidence", "gitRows", "untrackedGitPaths", "explicitTicketRows", "generatorPathCompare", javascript)(context, canonicalJson, (bytes: string) => createHash("sha256").update(bytes).digest("hex"), () => {}, () => false, () => false, () => [{ path: vector.sourcePath }], () => [], (_root: string, ticket: string) => { expect(ticket).toBe(vector.ticketDir); return ticketPaths.map((path) => ({ path })); }, projectionByteSort);
        const entry = (sourcePath: string, referencesIn: string[] = []) => ({ sourcePath, nodeKind: "file", contentHash: "a".repeat(64), mode: 420, size: 1, referencesIn, referencesOut: [] });
        const original = { entries: [entry(vector.sourcePath, [foreign, ...(row.transactionExists ? [transactionFile] : [])]), ...(row.transactionExists ? [entry(transactionFile)] : [])], violations: [], sourceTreeDigest: "" };
        original.sourceTreeDigest = engine.sourceTreeDigest(original.entries);
        const originalContext = { ticketDir: vector.ticketDir, transactionRoots: [], exactEvidencePaths: [] };
        context.set(original, originalContext);
        const filtered = engine.inventoryWithoutTransactionEvidence(original, transactionRoot, row.planExists ? exactPlan : undefined);
        expect(filtered.entries.map(({ sourcePath }: { sourcePath: string }) => sourcePath)).toEqual([vector.sourcePath]);
        expect(filtered.entries[0].referencesIn).toEqual([foreign]);
        expect(filtered.sourceTreeDigest === original.sourceTreeDigest).toBe(row.sourceDigestUnchanged);
        expect(engine.repositoryReferenceCandidatePaths("/fixture", {}, context.get(filtered))).toEqual([vector.sourcePath, foreign, unrelated].sort(projectionByteSort));
        expect(context.get(original)).toEqual(originalContext);
        if (!row.transactionExists && !row.planExists) expect(filtered).toEqual(original);
      }
    }
  });

  test("authored Draw submitted proof has exact canonical bytes and a closed single-case budget", async () => {
    const root = join(DRAW_SOURCE_SCENARIO_ROOT, "📨️submitted-proof"), text = readFileSync(join(root, "🔣️.json"), "utf8"), vector = JSON.parse(text);
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json"), "utf8")));
    expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
    for (const changed of [{ ...vector, schemaVersion: 2 }, { ...vector, fileMode: 420 }, { ...vector, writePolicy: "overwrite" }, { ...vector, singleCase: { ...vector.singleCase, budgetMs: 120001 } }, { ...vector, singleCase: { ...vector.singleCase, extraArguments: true } }]) expect(validate(changed)).toBe(false);
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(text, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    const plan = { ...vector.unsignedPlan, planDigest: createHash("sha256").update(JSON.stringify(vector.unsignedPlan, Object.keys(vector.unsignedPlan).sort())).digest("hex") };
    const bytes = artifactProjectionSubmittedPlanBytes(plan);
    expect(bytes.toString()).toBe(`${JSON.stringify(plan, Object.keys(plan).sort())}\n`);
    expect(jsonc.parse(bytes.toString(), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(plan);
    expect(errors).toEqual([]);
    expect(() => artifactProjectionSubmittedPlanBytes({ ...plan, planDigest: "0".repeat(64) })).toThrow();
    expect(() => artifactProjectionSubmittedPlanBytes({ ...plan, unowned: true })).toThrow();
    let now = 0;
    const retained: unknown[] = [], report = artifactProjectionProgressSampler((row) => retained.push(row), () => now);
    for (const row of vector.progress.cases) {
      now = row.at;
      report({ operation: row.operation, phase: row.phase, current: row.current, total: row.total, path: "unretained/old-source.ts" });
    }
    expect(retained).toEqual(vector.progress.cases.filter((row: { retain: boolean }) => row.retain).map(({ at, operation, phase, current, total }: { at: number; operation: string; phase: string; current: number; total: number }) => ({ elapsedMs: at, operation, phase, current, total })));
    expect(JSON.stringify(retained)).not.toContain("old-source");
  });

  test("authored Draw submitted proof is exclusive, synced, and retained without following links", async () => {
    const fixture = normalizationFixture("artifact-projection-submitted-proof", {}, undefined, ARTIFACT_PROJECTION_RUN_PARENT);
    let passed = false;
    try {
      const vector = JSON.parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "📨️submitted-proof/🔣️.json"), "utf8"));
      const unsigned = { ...vector.unsignedPlan, baselineCommit: fixture.baselineCommit }, plan = { ...unsigned, planDigest: createHash("sha256").update(JSON.stringify(unsigned, Object.keys(unsigned).sort())).digest("hex") };
      const path = artifactProjectionCaptureSubmittedPlan(fixture, plan);
      expect(path).toBe(join(fixture.ticketDir, vector.directoryName, `🔖️${plan.planDigest}`, vector.filename));
      const bytes = readFileSync(path), node = lstatSync(path), jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
      expect(jsonc.parse(bytes.toString(), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(plan);
      expect(errors).toEqual([]);
      expect(node.isFile() && !node.isSymbolicLink()).toBe(true);
      if (process.platform !== "win32") expect(node.mode & 0o7777).toBe(vector.fileMode);
      expect(artifactProjectionCaptureSubmittedPlan(fixture, plan)).toBe(path);
      expect(lstatSync(path).ino).toBe(node.ino);
      expect(readFileSync(path)).toEqual(bytes);
      for (const row of vector.invalidContracts) {
        const mutable = DRAW_SUBMITTED_PROOF as unknown as Record<string, unknown>, before = mutable[row.field];
        try { mutable[row.field] = row.value; expect(() => artifactProjectionCaptureSubmittedPlan(fixture, plan)).toThrow(); }
        finally { mutable[row.field] = before; }
      }
      if (process.platform !== "win32") {
        chmodSync(path, 0o644);
        expect(() => artifactProjectionCaptureSubmittedPlan(fixture, plan)).toThrow();
        expect(lstatSync(path).mode & 0o7777).toBe(0o644);
        chmodSync(path, vector.fileMode);
      }
      const stage = artifactProjectionRecordedStage(fixture, "plan", (progress) => { progress({ operation: "plan", phase: "complete", current: 1, total: 1, path: "unretained/old-source.ts" }); return 7; });
      expect(stage).toBe(7);
      expect(() => artifactProjectionRecordedStage(fixture, "apply", () => { throw new Error("fixture boundary"); })).toThrow("fixture boundary");
      const progressRecord = artifactProjectionProgressRecord(fixture), progressText = readFileSync(progressRecord.path, "utf8");
      expect(progressRecord.path.startsWith(`${fixture.root}${sep}`)).toBe(false);
      expect(progressRecord.path).toBe(join(NORMALIZATION_RUN_ROOT, NORMALIZATION_RUN_DIRECTORY, ...vector.progress.externalParentSegments, basename(fixture.root), "📝️.md"));
      const records = progressText.split("\n").filter((line) => line.startsWith("- ")).map((line) => jsonc.parse(line.slice(2), errors, { disallowComments: true, allowTrailingComma: false }));
      expect(errors).toEqual([]);
      expect(records.filter((record) => record.state).map((record) => record.state)).toEqual(["started", "completed", "started", "failed"]);
      expect(progressText).not.toContain("old-source");
      for (const { kind } of vector.nodeCases.filter((row: { kind: string }) => row.kind.endsWith("-symlink"))) {
        const alternate = { ...unsigned, sourceTreeDigest: (kind === "file-symlink" ? "3" : "4").repeat(64) };
        const other = { ...alternate, planDigest: createHash("sha256").update(JSON.stringify(alternate, Object.keys(alternate).sort())).digest("hex") };
        const owner = join(fixture.ticketDir, vector.directoryName, `🔖️${other.planDigest}`);
        if (kind === "file-symlink") { mkdirSync(owner); symlinkSync(path, join(owner, vector.filename)); }
        else symlinkSync(dirname(path), owner, process.platform === "win32" ? "junction" : "dir");
        expect(() => artifactProjectionCaptureSubmittedPlan(fixture, other)).toThrow();
        expect(readFileSync(path)).toEqual(bytes);
      }
      writeFileSync(path, "unexpected retained evidence\n");
      expect(() => artifactProjectionCaptureSubmittedPlan(fixture, plan)).toThrow();
      expect(readFileSync(path, "utf8")).toBe("unexpected retained evidence\n");
      expect(() => artifactProjectionCaptureSubmittedPlan({ ...fixture, ticketDir: dirname(fixture.ticketDir) }, plan)).toThrow();
      writeFileSync(progressRecord.path, `${progressText}foreign retained diagnostic\n`);
      expect(() => artifactProjectionRecordedStage(fixture, "plan", () => 1)).toThrow();
      expect(readFileSync(progressRecord.path, "utf8")).toBe(`${progressText}foreign retained diagnostic\n`);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  });

  test("authored Draw submitted proof precedes every artifact apply", async () => {
    const ts = await import("typescript"), text = readFileSync(import.meta.filename, "utf8"), source = ts.createSourceFile(import.meta.filename, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const fixturePath = join(contractRoot, "🏗️shared-fixtures/🟦️.ts"), fixtureSource = ts.createSourceFile(fixturePath, readFileSync(fixturePath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const callsOf = (root: import("typescript").Node): import("typescript").CallExpression[] => {
      const found: import("typescript").CallExpression[] = [];
      const visit = (node: import("typescript").Node): void => { if (ts.isCallExpression(node)) found.push(node); ts.forEachChild(node, visit); };
      visit(root);
      return found;
    };
    const calls = callsOf(source), fixtureCalls = callsOf(fixtureSource);
    for (const title of ["rolls back and atomically applies CAD and Draw projections to an empty second plan", "rejects ignored and unplanned residual children in a projected source owner without following links"]) {
      const testCall = calls.find((call) => ts.isIdentifier(call.expression) && call.expression.text === "test" && ts.isStringLiteral(call.arguments[0]!) && call.arguments[0].text === title)!;
      expect(testCall).toBeDefined();
      const scoped = calls.filter((call) => call.pos > testCall.pos && call.end < testCall.end);
      expect(scoped.filter((call) => ts.isIdentifier(call.expression) && call.expression.text === "applyTaxonomyPlan").map((call) => source.getLineAndCharacterOfPosition(call.getStart(source)).line + 1)).toEqual([]);
      expect(scoped.filter((call) => ts.isIdentifier(call.expression) && call.expression.text === "artifactProjectionApply").length).toBeGreaterThan(0);
    }
    const planner = fixtureSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "normalizationPlan")!;
    expect(fixtureCalls.some((call) => call.pos > planner.pos && call.end < planner.end && ts.isIdentifier(call.expression) && call.expression.text === "artifactProjectionCaptureSubmittedPlan")).toBe(true);
    const wrapper = fixtureSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "artifactProjectionApply")!;
    const owned = fixtureCalls.filter((call) => call.pos > wrapper.pos && call.end < wrapper.end && ts.isIdentifier(call.expression));
    expect(owned.findIndex((call) => call.expression.getText(fixtureSource) === "artifactProjectionCaptureSubmittedPlan")).toBeLessThan(owned.findIndex((call) => call.expression.getText(fixtureSource) === "applyTaxonomyPlan"));
    expect(wrapper.getText(fixtureSource)).toContain("planArtifactPath,");
  });

  test("authored Draw residue route enforces its registered single-case parent budget", async () => {
    const vector = JSON.parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "📨️submitted-proof/🔣️.json"), "utf8")).singleCase;
    await assertArtifactProjectionSingleCaseRoute(vector);
  });

  test("authored Draw commit route enforces its registered single-case parent budget", async () => {
    const inputRoot = join(DRAW_SOURCE_SCENARIO_ROOT, "🛣️commit-route"), bytes = readFileSync(join(inputRoot, "🔣️.json"), "utf8"), vector = JSON.parse(bytes);
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🖍️draw-source-scenario/🛣️commit-route/🔣️.json"), "utf8")));
    expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
    for (const changed of [{ ...vector, budgetMs: 120001 }, { ...vector, bunTimeoutMs: 120001 }, { ...vector, testName: "other" }, { ...vector, extraArguments: true }, { ...vector, runner: "other" }]) expect(validate(changed)).toBe(false);
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    await assertArtifactProjectionSingleCaseRoute(vector);
  });

  test("authored Draw residue recovery checkpoints have closed language-neutral ownership", async () => {
    const root = join(DRAW_SOURCE_SCENARIO_ROOT, "🛟️residue-recovery"), bytes = readFileSync(join(root, "🔣️.json"), "utf8");
    const vector = JSON.parse(bytes);
    expect(vector["schemaVersion"]).toEqual(1);expect(vector["contractId"]).toEqual("draw-residue-durable-recovery-v1");expect(vector["childExitCode"]).toEqual(73);expect(vector["foreignInput"]).toEqual("ignored-source-directory-with-file-empty-directory-and-symlink");expect(vector["generatorStarts"]).toEqual(0);expect(vector["inverseState"]).toEqual("rolled-back");expect(vector["preservation"]).toEqual("all-owned-preimages-and-all-foreign-residue");expect(vector["cases"]).toEqual([{"id":"durable-resume","checkpoint":"durable","phase":"transaction-wal-prepared","canonicalState":"regenerating","candidateState":null,"installed":"all","cancel":false},{"id":"cancelled-resume","checkpoint":"durable","phase":"transaction-wal-prepared","canonicalState":"regenerating","candidateState":null,"installed":"all","cancel":true},{"id":"next-journal-wal","checkpoint":"candidate","phase":"transaction-journal-candidate-written","canonicalState":"editing","candidateState":"regenerating","installed":"all","cancel":false},{"id":"rename-before-journal","checkpoint":"rename","phase":"transaction-journal-write-mkdir","canonicalState":"installing","candidateState":null,"installed":"first-unrecorded","cancel":false}]);
    
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    expect(vector.cases.filter((row: { cancel: boolean }) => row.cancel)).toHaveLength(1);
    expect(new Set(vector.cases.map((row: { phase: string }) => row.phase)).size).toBe(3);
  });

  test("authored Draw residue ignore authority outranks inherited negation before injection", async () => {
    const inputRoot = join(DRAW_SOURCE_SCENARIO_ROOT, "🙈️residue-ignore"), bytes = readFileSync(join(inputRoot, "🔣️.json"), "utf8"), vector = JSON.parse(bytes);
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json"), "utf8")));
    expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
    for (const changed of [{ ...vector, authoritativeSource: vector.lowerPrioritySource }, { ...vector, absentIgnored: false }, { ...vector, writePolicy: "replace-policy" }]) expect(validate(changed)).toBe(false);
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    const fixture = normalizationFixture("artifact-projection-residue-ignore", {}, ({ repoRoot }) => writeFileSync(join(repoRoot, vector.authoritativeSource), vector.rootIgnore), ARTIFACT_PROJECTION_RUN_PARENT);
    let passed = false;
    try {
      const residue = join(fixture.workspace, ...vector.residueSegments), leaf = join(residue, vector.leaf), relativeLeaf = relative(fixture.root, leaf).replaceAll("\\", "/");
      const rule = `/${relative(fixture.root, residue).replaceAll("\\", "/")}/\n`, ignorePath = join(fixture.root, vector.authoritativeSource), beforeMode = lstatSync(ignorePath).mode;
      mkdirSync(dirname(residue), { recursive: true });
      writeFileSync(join(fixture.root, vector.lowerPrioritySource), rule);
      const ignored = (path: string): boolean => {
        const result = Bun.spawnSync(["git", "check-ignore", "--quiet", "--no-index", "--", path], { cwd: fixture.root, stdout: "pipe", stderr: "pipe" });
        if (result.exitCode !== 0 && result.exitCode !== 1) throw new Error(result.stderr.toString());
        return result.exitCode === 0;
      };
      expect(ignored(relativeLeaf)).toBe(vector.beforeIgnored);
      for (const invalid of [join(fixture.root, "🧪️unexpected"), join(fixture.workspace, "*", "🧪️unexpected"), join(fixture.workspace, "🧪️line\nbreak", "🧪️unexpected")]) expect(() => artifactProjectionIgnoreResidue(fixture, invalid)).toThrow();
      expect(readFileSync(ignorePath, "utf8")).toBe(vector.rootIgnore);
      artifactProjectionIgnoreResidue(fixture, residue);
      expect(existsSync(residue)).toBe(false);
      expect(ignored(relativeLeaf)).toBe(vector.absentIgnored);
      expect(readFileSync(ignorePath, "utf8")).toBe(vector.rootIgnore + rule);
      expect(lstatSync(ignorePath).mode).toBe(beforeMode);
      mkdirSync(residue, { recursive: true });
      writeFileSync(leaf, vector.content);
      expect(ignored(relativeLeaf)).toBe(vector.presentIgnored);
      expect(ignored(relative(fixture.root, join(dirname(residue), "🧪️neighbor", vector.leaf)).replaceAll("\\", "/"))).toBe(vector.siblingIgnored);
      const oracle = Bun.spawnSync(["git", "check-ignore", "--verbose", "--no-index", "-z", "--stdin"], { cwd: fixture.root, stdin: Buffer.from(`${relativeLeaf}\0`), stdout: "pipe", stderr: "pipe" });
      expect(oracle.exitCode, oracle.stderr.toString()).toBe(0);
      expect(oracle.stdout.toString().split("\0").slice(0, 4)).toEqual([vector.authoritativeSource, "3", rule.trim(), relativeLeaf]);
      expect(readFileSync(join(fixture.root, vector.lowerPrioritySource), "utf8")).toBe(rule);
      expect(() => artifactProjectionIgnoreResidue(fixture, residue)).toThrow("Residue must be absent");
      expect(readFileSync(ignorePath, "utf8")).toBe(vector.rootIgnore + rule);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  });

  test("authored Draw source schema and parser oracles retain the exact nine-member contract", async () => {
    
    
    expect(DRAW_SOURCE_SCENARIO["schemaVersion"]).toEqual(1);expect(DRAW_SOURCE_SCENARIO["contractId"]).toEqual("draw-source-scenario-input-v1");expect(DRAW_SOURCE_SCENARIO["producerContext"]).toEqual({"generatorId":"plugin-registry","compilerRoots":["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"],"runtimeModules":["🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs","🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs"],"runtimeData":["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"],"runtimeReceiptCatalogs":[{"manifestPath":"🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json","sourceRoot":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts","receiptPathField":"protocol_path"}],"workspaceInputs":["package.json","nx.json","📋️project.json",".gitignore"],"runtimePackages":["nx","@nx/js","@nxlv/python","typescript","@iarna/toml"],"authority":"current-exact-no-follow-static-closure","workspaceBindings":"fixture-local","initialOutputs":"actual-generator-before-baseline","registryNodeCount":16});expect(DRAW_SOURCE_SCENARIO["cadConsumerMount"]).toEqual({"path":"✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts","meaning":"unchanged-live-cad-bytes-in-authored-canonical-fixture-context"});expect(DRAW_SOURCE_SCENARIO["owner"]["artifactId"]).toEqual("🖍️drawing");expect(DRAW_SOURCE_SCENARIO["owner"]["standardVersion"]).toEqual("1");expect(DRAW_SOURCE_SCENARIO["owner"]["subsetId"]).toEqual("any");expect(DRAW_SOURCE_SCENARIO["owner"]["commandDirectoryName"]).toEqual("🖱️canvas-pointer-down");expect(DRAW_SOURCE_SCENARIO["commandBundle"]["directoryKinds"]["fsm"]).toEqual({"emoji":"🔄️","slugPattern":"^fsm$","allowEmojiOnly":false});expect(DRAW_SOURCE_SCENARIO["commandBundle"]["descendantContract"]["rootDirectoryKindId"]).toEqual("projected-editor-command");expect(DRAW_SOURCE_SCENARIO["commandBundle"]["descendantContract"]["exclusiveAlternatives"]).toEqual([]);expect(DRAW_SOURCE_SCENARIO["commandBundle"]["descendantContract"]["realizedNodeCount"]).toEqual(16);
    
    const projection = projectionGolden("artifact-editor-command-bundle-v1");
    expect(DRAW_SOURCE_SCENARIO.members.map(({ path }) => `${projection.sourceRoot}/${path}`)).toEqual(projection.mappings.map(({ sourcePath }) => sourcePath));
    expect(DRAW_SOURCE_SCENARIO.owner).toEqual({ artifactId: projection.artifactId, standardVersion: projection.standardVersion, subsetId: projection.subsetId, commandDirectoryName: basename(projection.sourceRoot) });
    expect(DRAW_SOURCE_SCENARIO.members.reduce((total, row) => total + Buffer.byteLength(row.content), 0)).toBeLessThan(10_000);
    const jsonc = await import("jsonc-parser"), ts = await import("typescript");
    const render = (content: string): string => content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
    let projectCwds = 0, workspaceGlobs = 0;
    for (const row of [...DRAW_SOURCE_SCENARIO.members, ...DRAW_SOURCE_SCENARIO.consumers]) {
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
        if (row.path.endsWith("/📦️packages/🦀️rust/Cargo.toml")) expect(DRAW_SOURCE_SCENARIO.members.map(({ path }) => path)).toContain(posix.join(dirname(row.path), (parsed.lib as { path: string }).path));
        if (row.path === "Cargo.toml") expect((parsed.workspace as { members: string[] }).members).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.workspaceMemberCount);
      } else if (row.format === "typescript") {
        const result = ts.transpileModule(content, { fileName: row.path, reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } });
        expect(result.diagnostics).toEqual([]);
        const source = ts.createSourceFile(row.path, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
        const imports = source.statements.filter(ts.isImportDeclaration).map((statement) => (statement.moduleSpecifier as import("typescript").StringLiteral).text);
        expect(imports).toEqual(row.path === "📜️script.ts" ? ["node:fs", "node:path", "@semio-tech/repo-lib"] : ["@semio-tech/repo-lib"]);
        expect(fixedSourceDispositionDecision("root-script", content)?.role).toBe("tool-metadata");
      }
    }
    expect(projectCwds).toBe(DRAW_SOURCE_SCENARIO.oracle.projectCwdCount);
    expect(workspaceGlobs).toBe(DRAW_SOURCE_SCENARIO.oracle.workspaceGlobCount);
    const host = DRAW_SOURCE_SCENARIO.catalogContext.find(({ format }) => format === "toml")!;
    const parsedHost = toml.parse(host.content);
    expect(Bun.TOML.parse(host.content)).toEqual(parsedHost);
    expect(parsedHost.package).toEqual({ name: "draw-fixture-host", version: "0.1.0", edition: "2021", metadata: { component: { package: "semio:draw-fixture-host" }, semio: { role: "hub", "component-kind": "plugin", "deployment-directory": "🧪️draw-fixture-host", host: { landing: "fixture-landing", shell: "fixture-shell" }, playground: [{ variant: "draw-fixture-host", ports: { react: 6197, wgpu: 6198 } }] } } });
    expect(DRAW_SOURCE_SCENARIO.catalogContext.find(({ path }) => path === `${dirname(host.path)}/${(parsedHost.lib as { path: string }).path}`)?.content).toBe("pub fn fixture_host() {}\n");
  });

  test("authored Draw source producer context captures actual implementations with independent import parity", async () => {
    const ts = await import("typescript"), { registryStaticImports } = await import("../../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts");
    const taxonomy = loadTaxonomy(), readPaths: string[] = [];
    const context = artifactProjectionProducerInputs(taxonomy, (path: string) => {
      readPaths.push(path);
      expect(path).not.toStartWith("✏️s/🔌️plugins/🖍️draw/");
      return artifactProjectionProducerInput(path);
    });
    const contract = taxonomy.generatorContracts[DRAW_SOURCE_SCENARIO.producerContext.generatorId]!;
    expect(context.files[contract.inputDiscovery!.implementationEntryPaths[0]!]?.content).toBe(readFileSync(join(getWorkspaceRoot(), contract.ownerPath!, "📜️script.ts"), "utf8"));
    expect(context.files[contract.inputDiscovery!.implementationEntryPaths[0]!]?.content).not.toBe("export {};\n");
    for (const path of [...contract.inputPatterns, ...DRAW_SOURCE_SCENARIO.producerContext.workspaceInputs, ...DRAW_SOURCE_SCENARIO.producerContext.runtimeData]) expect(context.files[path]?.content).toBe(readFileSync(join(getWorkspaceRoot(), path), "utf8"));
    for (const path of DRAW_SOURCE_SCENARIO.producerContext.runtimeModules) expect(context.modules.some((row) => row.path === path)).toBe(true);
    const nxPlugins = (JSON.parse(context.files["nx.json"]!.content) as { plugins: { plugin: string }[] }).plugins.filter(({ plugin }) => plugin.startsWith("./")).map(({ plugin }) => plugin.slice(2));
    for (const path of [...DRAW_SOURCE_SCENARIO.producerContext.runtimeModules, ...DRAW_SOURCE_SCENARIO.producerContext.runtimeData]) expect(nxPlugins.some((plugin) => context.files[plugin]!.content.includes(posix.relative(posix.dirname(plugin), path))), path).toBe(true);
    expect(readPaths).toEqual([...new Set(readPaths)]);
    const nx = JSON.parse(context.files["nx.json"]!.content);
    for (const row of nx.plugins.filter((entry: { plugin: string }) => entry.plugin.startsWith("./"))) expect(context.files[row.plugin.slice(2)]).toBeDefined();
    const nxScript = (JSON.parse(context.files["package.json"]!.content) as { scripts: Record<string, string> }).scripts.nx!.match(/^bun \.\/(\S+) nx$/u)?.[1];
    if (nxScript === undefined) throw new Error("package.json names no `bun ./<script> nx` runner");
    expect(DRAW_SOURCE_SCENARIO.producerContext.compilerRoots).toEqual([nxScript]);
    for (const row of context.modules) {
      if (row.kind === "json-data") {
        const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
        expect(jsonc.parse(context.files[row.path]!.content, errors, { disallowComments: true, allowTrailingComma: false }), row.path).toEqual(JSON.parse(context.files[row.path]!.content));
        expect(errors, row.path).toEqual([]);
        expect(row.imports, row.path).toEqual([]);
        continue;
      }
      const source = ts.createSourceFile(row.path, context.files[row.path]!.content, ts.ScriptTarget.Latest, true, row.path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
      const oracle = [...new Set(source.statements.flatMap((statement) => {
        if (!ts.isImportDeclaration(statement) && !ts.isExportDeclaration(statement) || ts.isImportDeclaration(statement) && statement.importClause?.isTypeOnly || ts.isExportDeclaration(statement) && statement.isTypeOnly) return [];
        return statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? [statement.moduleSpecifier.text] : [];
      }))].sort();
      expect(registryStaticImports(context.files[row.path]!.content, row.path), row.path).toEqual(oracle);
      expect(row.imports, row.path).toEqual(oracle);
    }
    for (const [name, binding] of Object.entries(contract.inputDiscovery!.workspaceImports)) {
      expect(JSON.parse(context.files[binding.manifestPath]!.content).name).toBe(name);
      expect(context.files[binding.entryPath]).toBeDefined();
    }
  });

  test("authored Draw source producer retains imported JSON bytes and rejects invalid data", () => {
    const vector = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🌐️registry-import-language/🧪️imported-data/🔣️.json"), "utf8")) as { graph: { entries: string[]; dataPath: string; files: { path: string; content: string; mode: number }[] } };
    const taxonomy = JSON.parse(readFileSync(join(getWorkspaceRoot(), NORMALIZATION_SCHEMA_REL), "utf8")) as Taxonomy;
    Object.assign(taxonomy.generatorContracts["plugin-registry"]!.inputDiscovery!, { implementationEntryPaths: vector.graph.entries, workspaceImports: {} });
    const source = new Map(vector.graph.files.map((row) => [row.path, row])), reads: string[] = [];
    const read = (path: string): ArtifactProducerInput => {
      reads.push(path);
      const row = source.get(path);
      if (!row && ![...DRAW_SOURCE_SCENARIO.producerContext.workspaceInputs, ...taxonomy.generatorContracts["plugin-registry"]!.inputPatterns].includes(path)) throw new Error("Missing authored compiler input: " + path);
      const content = row?.content ?? (path === "nx.json" ? '{"plugins":[]}' : "{}"), mode = row?.mode ?? 0o644;
      return { content, mode, sha256: createHash("sha256").update(content).digest("hex"), origin: "authored-scenario" };
    };
    const context = { ...DRAW_SOURCE_SCENARIO.producerContext, compilerRoots: [], runtimeModules: [], runtimeData: [], runtimeReceiptCatalogs: [] };
    const captured = artifactProjectionProducerInputs(taxonomy, read, context);
    const data = source.get(vector.graph.dataPath)!;
    expect(captured.files[data.path]).toEqual({ content: data.content, mode: data.mode, sha256: createHash("sha256").update(data.content).digest("hex"), origin: "authored-scenario" });
    expect(captured.modules.find(({ path }) => path === data.path)).toEqual({ path: data.path, kind: "json-data", imports: [] });
    expect(reads.filter((path) => path === data.path)).toHaveLength(1);
    expect(reads.some((path) => path.includes("never.ts"))).toBe(false);
    source.set(data.path, { ...data, content: "{" });
    expect(() => artifactProjectionProducerInputs(taxonomy, read, context)).toThrow("imported JSON is invalid");
    source.delete(data.path);
    expect(() => artifactProjectionProducerInputs(taxonomy, read, context)).toThrow("Missing authored compiler input");
    source.set(data.path.replace(".json", ".txt"), { ...data, path: data.path.replace(".json", ".txt") });
    const entry = source.get(vector.graph.entries[0]!)!;
    source.set(entry.path, { ...entry, content: entry.content.replaceAll(".json", ".txt") });
    expect(() => artifactProjectionProducerInputs(taxonomy, read, context)).toThrow("language is not supported");
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
    for (const manifest of DRAW_SOURCE_SCENARIO.members.filter(({ path }) => path.endsWith("/📦️packages/🦀️rust/Cargo.toml"))) {
      const entry = posix.join(dirname(manifest.path), (toml.parse(manifest.content).lib as { path: string }).path);
      expect(projection.mappings.map(({ sourcePath }) => sourcePath)).toContain(`${projection.sourceRoot}/${entry}`);
      expect(projection.mappings.map(({ destinationPath }) => destinationPath)).toContain(`${projection.destinationRoot}/${entry}`);
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
    const { inspectRustModuleGraph } = await import("../../../🔍️discovery/🌐️rust-module-graph/🟦️.ts"), { inspectRustJoinArgumentSpans, inspectRustManifestPathReferences } = await import("../../../🔍️discovery/🧷️rust-path-references/🟦️.ts");
    const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { conventionalRoots: false, strictManifests: true });
    expect([...new Set(graph.contexts.get(modulePath)?.map(({ manifestPath }) => manifestPath))]).toEqual([mount.manifestPath]);
    expect(graph.contexts.get(modulePath)?.filter(({ modulePath: segments }) => segments.join("::") === mount.moduleName).map(({ manifestPath }) => manifestPath)).toEqual([mount.manifestPath]);
    const cad = projectionGolden("artifact-example-model-catalog-v1"), testsPath = posix.join(posix.dirname(modulePath), "🧪️tests/🔬️unit/🦀️.rs");
    const expectedRoot = `../../${posix.relative(cad.sourceRoot.slice(0, cad.sourceRoot.indexOf("/🏅️standards/")), cad.sourceRoot)}`;
    expect(inspectRustManifestPathReferences(files[modulePath]!)).toEqual([]);
    expect([...new Set(graph.contexts.get(testsPath)?.map(({ manifestPath }) => manifestPath))]).toEqual([mount.manifestPath]);
    expect(inspectRustManifestPathReferences(files[testsPath]!).map(({ value, base }) => ({ value, base }))).toEqual([{ value: expectedRoot, base: [] }]);
    expect(inspectRustJoinArgumentSpans(files[testsPath]!).map(({ value }) => value)).toEqual([expectedRoot, "\\n"]);
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
    expect(reads).toHaveLength(projectionGolden("artifact-example-model-catalog-v1").sourceFileCount + 5);
    for (const row of DRAW_SOURCE_SCENARIO.members) expect(files[`${artifactProjectionFixturePath(projection.sourceRoot)}/${row.path}`]).toBe(drawSourceScenarioContent(row.content, projection));
    for (const row of DRAW_SOURCE_SCENARIO.consumers) expect(files[row.path]).toBe(drawSourceScenarioContent(row.content, projection));
    expect(files[DRAW_SOURCE_SCENARIO.cadConsumerMount.path]).toBeDefined();
    expect(createHash("sha256").update(files[DRAW_SOURCE_SCENARIO.cadConsumerMount.path]!).digest("hex")).toBe(createHash("sha256").update(readFileSync(join(getWorkspaceRoot(), DRAW_SOURCE_SCENARIO.cadConsumerMount.path))).digest("hex"));
  });

  test("authored Draw source run parent is exact and rejects unsafe ancestry before allocation", async () => {
    const expected = [NORMALIZATION_RUN_DIRECTORY, ...DRAW_SOURCE_SCENARIO.retention.parentSegments].join("/");
    expect(expected).toBe(ARTIFACT_PROJECTION_RUN_PARENT);
    const taxonomy = loadTaxonomy(), { registryCatalogPathMayAffect } = await import("../../../🔍️discovery/🧲️registry-catalog-projection/🟦️.ts");
    const fixtureOwner = DRAW_SOURCE_SCENARIO.retention.fixtureOwnerSegments.join("/");
    expect(fixtureOwner).toBe("🧪️tests");
    expect(registryCatalogPathMayAffect(`${fixtureOwner}/🧪️fixture/🗿️artifacts/🖍️draw/🦀️.rs`, taxonomy)).toBe(true);
    const parsed = (await import("jsonc-parser")).parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "🔣️.json"), "utf8")) as DrawSourceScenario;
    expect(parsed.retention).toEqual(DRAW_SOURCE_SCENARIO.retention);
    const virtualRoot = resolve("/draw-source-ownership"), locations = artifactProjectionFixtureLocations(virtualRoot);
    expect(relative(virtualRoot, locations.workspace).replaceAll("\\", "/")).toBe(`${parsed.retention.fixtureOwnerSegments.join("/")}/🧪️fixture`);
    expect(relative(virtualRoot, locations.ticketDir).replaceAll("\\", "/")).toBe(parsed.retention.transactionTicketSegments.join("/"));
    expect(registryCatalogPathMayAffect(`${parsed.retention.transactionTicketSegments.join("/")}/🧾️taxonomy-transaction`, taxonomy)).toBe(false);
    for (const vector of DRAW_SOURCE_SCENARIO.retention.cases) {
      let created = 0;
      const inspect = (path: string) => {
        const selected = path === resolve(NORMALIZATION_RUN_ROOT, expected);
        if (selected && (vector.kind === "unreadable" || vector.kind === "missing" && created === 0)) throw Object.assign(new Error(vector.kind), { code: vector.kind === "missing" ? "ENOENT" : "EACCES" });
        return { isDirectory: () => !selected || vector.kind === "directory" || vector.kind === "missing" && created === 1, isSymbolicLink: () => selected && vector.kind === "symlink" };
      };
      const run = () => normalizationRetainedRunParent(expected, inspect, (path) => { expect(path).toBe(resolve(NORMALIZATION_RUN_ROOT, expected)); created++; });
      if (vector.allowed) expect(run()).toBe(resolve(NORMALIZATION_RUN_ROOT, expected));
      else expect(run).toThrow();
      expect(created).toBe(vector.creates);
    }
    expect(() => normalizationRetainedRunParent(`${expected}/../🧪️elsewhere`, () => { throw new Error("must not inspect"); }, () => { throw new Error("must not create"); })).toThrow("exact artifact run parent");
  });

  test("canonical CAD and Draw authority retains the exact language-neutral mapping digests", () => {
    for (const projection of ARTIFACT_PROJECTION_GOLDEN.projections) {
      const source = projectionAuthorityNodes(projection, projection.contractId === "artifact-editor-command-bundle-v1" ? "authored-draw" : "live");
      const directories = new Set<string>([projection.destinationRoot]);
      const files = projection.mappings.map(({ sourcePath, destinationPath }) => {
        const content = source.find(({ path }) => path === sourcePath)!.content!;
        for (let path = dirname(destinationPath); path !== dirname(projection.destinationRoot); path = dirname(path)) directories.add(path);
        return { path: destinationPath, nodeKind: "file" as const, content };
      });
      const nodes = [...[...directories].map((path) => ({ path, nodeKind: "directory" as const })), ...files];
      const result = semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes, layout: "destination" }, projectionTaxonomy(projection));
      expect(result.problems).toEqual([]);
      expect(result.mappings).toEqual(projection.mappings);
      expect(result.mappingDigest).toBe(projection.mappingDigest);
      const invalid = nodes.map((node) => node.nodeKind === "file" && node.path === files[0]!.path ? { ...node, path: `${node.path}.unexpected` } : node);
      expect(semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes: invalid, layout: "destination" }, projectionTaxonomy(projection)).problems.length).toBeGreaterThan(0);
      const symlink = nodes.map((node) => node.path === files[0]!.path ? { path: node.path, nodeKind: "symlink" as const } : node);
      expect(semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes: symlink, layout: "destination" }, projectionTaxonomy(projection)).problems.some((problem) => problem.includes("forbidden symlink"))).toBe(true);
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
    expect(platformBoundary).toEqual([...cadProjectionLiveSources(projection).values()].sort(projectionByteSort));
    expect(result.problems).toEqual([]);
    expect(result.mappings).toEqual(projection.mappings);
    expect(result.destinationRoot).toBe(projection.destinationRoot);
    expect(result.mappingDigest).toBe("b47425503a8ebc2a60120e91a5fd6a7e8a54411bdfc122b87a2ef7d5a7cf2d8f");
    expect(result.destinationDirectoryCount).toBe(266);
    expect(result.destinationNodeCount).toBe(495);
    expect(result.maxPathUtf16Units).toBe(189);
    expect(projection.modelCatalog?.models).toHaveLength(9);
    expect(projection.modelCatalog?.categoryRules.map(({ count }) => count).reduce((sum, count) => sum + count, 0)).toBe(220);
  });

  test("CAD plain live binding examples retain exact mapping and reject escaping carriers", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    expect(CAD_LIVE_BINDINGS.bindings.map(({ mapping }) => mapping)).toEqual(projection.mappings.map((_, index) => index));
    expect(ARTIFACT_PROJECTION_GOLDEN.projections.every((entry) => !("liveBindings" in entry))).toBe(true);
    const tampered = { ...CAD_LIVE_BINDINGS, bindings: [...CAD_LIVE_BINDINGS.bindings.slice(1), { ...CAD_LIVE_BINDINGS.bindings[0]!, live: `../${CAD_LIVE_BINDINGS.bindings[0]!.live}` }] };
    expect(() => cadProjectionLiveSources(projection, undefined, tampered)).toThrow("outside its exact root");
  });

  test("CAD frozen scenario bindings read exact current bytes and reject missing, duplicate, or extra identities", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    const paths = ownedFilePaths(join(getWorkspaceRoot(), projection.sourceRoot)).filter((path) => path.endsWith(".json"));
    const bindings = cadProjectionLiveSources(projection, paths), reads: string[] = [];
    const nodes = projectionAuthorityNodes(projection, "live", (path) => {
      reads.push(path);
      return readFileSync(join(getWorkspaceRoot(), path), "utf8");
    }).filter(({ nodeKind }) => nodeKind === "file");
    expect(nodes).toHaveLength(229);
    expect(paths).toHaveLength(229);
    expect(reads.sort(projectionByteSort)).toEqual(paths.map((path) => `${projection.sourceRoot}/${path}`).sort(projectionByteSort));
    for (const node of nodes) expect(node.content).toBe(readFileSync(join(getWorkspaceRoot(), bindings.get(node.path)!), "utf8"));
    expect(() => cadProjectionLiveSources(projection, paths.slice(1))).toThrow("Missing");
    expect(() => cadProjectionLiveSources(projection, [...paths, paths[0]!])).toThrow("Duplicate");
    expect(() => cadProjectionLiveSources(projection, [...paths, "🧪️unknown/🔣️modelDefinition.json"])).toThrow("unbound");
    expect(createHash("sha256").update(projection.mappings.map(({ sourcePath, destinationPath }) => `${sourcePath}\0${destinationPath}`).join("\n")).digest("hex")).toBe(projection.mappingDigest);
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
    expect(projectionAuthority(projection, longNodes).problems.some((problem) => problem.includes("path-too-long"))).toBe(true);
    expect(projectionAuthority(projection, nodes, [projection.mappings[0]!.destinationPath]).problems.some((problem) => problem.includes("occupied"))).toBe(true);
  });

  test("artifact-editor-command-projection preserves the strict fixed-file union, the sealed scenario package bundle, and live command parity", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1"), live = loadTaxonomy(), scenario = drawScenarioTaxonomy(live);
    const options = { artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot };
    const nodes = projectionAuthorityNodes(projection, "authored-draw", () => { throw new Error("Live Draw source must not be read by the strict fixed-file union"); });
    const result = projectionAuthority(projection, nodes);
    expect(validateTaxonomy(scenario)).toEqual([]);
    expect(result.problems).toEqual([]);
    expect(result.mappings).toEqual(projection.mappings);
    expect(result.mappingDigest).toBe(projection.mappingDigest);
    expect(result.destinationDirectoryCount).toBe(projection.destinationDirectoryCount);
    expect(result.destinationNodeCount).toBe(projection.destinationNodeCount);
    expect(result.maxPathUtf16Units).toBe(projection.maxPathUtf16Units);
    expect(result.mappings.filter(({ destinationPath }) => destinationPath.endsWith("/📦️packages/🦀️rust/Cargo.toml"))).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.workspaceMemberCount);
    expect(semanticPathProjectionAuthority({ ...options, nodes }, live).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const destinationRoot = join(getWorkspaceRoot(), projection.destinationRoot), liveFiles = ownedFilePaths(destinationRoot).map((path) => `${projection.destinationRoot}/${path}`).sort(projectionByteSort);
    const liveDirectories = new Set<string>([projection.destinationRoot]);
    for (const path of liveFiles) for (let parent = dirname(path); parent !== projection.destinationRoot; parent = dirname(parent)) liveDirectories.add(parent);
    const liveNodes: SemanticProjectionAuthorityNode[] = [...[...liveDirectories].map((path) => ({ path, nodeKind: "directory" as const })), ...liveFiles.map((path) => ({ path, nodeKind: "file" as const, content: readFileSync(join(getWorkspaceRoot(), path), "utf8") }))];
    const current = semanticPathProjectionAuthority({ ...options, nodes: liveNodes, layout: "destination" }, live);
    expect(current.problems).toEqual([]);
    expect(current.mappings.map(({ destinationPath }) => destinationPath).sort(projectionByteSort)).toEqual(liveFiles);
    expect(semanticPathProjectionAuthority({ ...options, nodes: liveNodes, layout: "destination" }, scenario).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const partial = nodes.filter(({ path }) => !path.endsWith("/🔄️fsm/✨️macros/📦️packages/🦀️rust/📋️project.json"));
    expect(projectionAuthority(projection, partial).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);
    const extra = [...nodes, { path: `${projection.sourceRoot}/🔣️extra.json`, nodeKind: "file" as const, content: "{}" }];
    expect(projectionAuthority(projection, extra).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const invalid = structuredClone(live) as Taxonomy;
    const contract = invalid.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { requiredNodes: Record<string, unknown>[] };
    const sourceNode = contract.requiredNodes.find((node) => node.kindId === "rust-source" && !(node.pathSegments as unknown[]).length)!;
    sourceNode.kindId = "json";
    expect(validateTaxonomy(invalid).some((problem) => problem.includes("draw-editor-command-bundle-v1"))).toBe(true);
    const undeclared = structuredClone(scenario) as Taxonomy;
    const bundle = undeclared.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { requiredNodes: Record<string, unknown>[]; realizedNodeCount: number };
    bundle.requiredNodes = bundle.requiredNodes.filter((node) => node.fixedFilenameContractId !== "root-script" || (node.pathSegments as unknown[]).length !== 3);
    bundle.realizedNodeCount -= 1;
    expect(validateTaxonomy(undeclared).some((problem) => problem.includes("declaration-only nested Rust packages"))).toBe(true);
  });
});
