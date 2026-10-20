/** 🔶️ The authored draw preflight context tests of the artifact path projection authority suite. */
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { chmodSync, lstatSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, sep } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { normalizationEngineDeclarations } from "../../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { ARTIFACT_PROJECTION_RUN_PARENT, DRAW_SOURCE_SCENARIO_ROOT, DRAW_SUBMITTED_PROOF, NORMALIZATION_RUN_DIRECTORY, NORMALIZATION_RUN_ROOT, artifactProjectionCaptureSubmittedPlan, artifactProjectionProgressRecord, artifactProjectionProgressSampler, artifactProjectionRecordedStage, artifactProjectionSubmittedPlanBytes, assertArtifactProjectionSingleCaseRoute, normalizationFixture, projectionByteSort, retainArtifactProjectionFixture } from "../../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔷 Registers the authored draw preflight context tests of the artifact path projection authority suite. */
export function registerAuthoredDrawPreflightContextTests(): void {
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
}
