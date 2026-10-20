/** 🔶️ The mutation root discovery projects tests of the direct mutation ownership suite. */
import { mutationRoot, mutationFixtureRoot } from "../../../🧬️mutation-authority/🧫️fixture/🟦️.ts";
import { expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { mutationTaxonomySourceIndex } from "../../../../🧹️normalization/🧬️mutation/📇️index/🟦️.ts";
import { policyMutationStructuralBreaches, policyMutationStructuralBreachesView } from "../../../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { mutationTaxonomyStructuralView } from "../../../../🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔷 Registers the mutation root discovery projects tests of the direct mutation ownership suite. */
export function registerMutationRootDiscoveryProjectsTests(): void {
  test("mutation root discovery projects only admitted authored facets", async () => {
    const ts = await import("typescript");
    const inputRoot = join(contractRoot, "../🧫️fixtures/🌱️mutation-root-discovery");
    const vectorPath = join(inputRoot, "🔣️.json");
    
    const vector = JSON.parse(readFileSync(vectorPath, "utf8")) as { schemaVersion: number; includedRoots: string[]; filePaths: string[]; rejections: { name: string }[] };
    
    expect(vector["schemaVersion"]).toEqual(1);
    
    const sourcePath = join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts");
    const source = readFileSync(sourcePath, "utf8");
    const inputs = [sourcePath, vectorPath, import.meta.path].map((path) => ({ path, sha256: createHash("sha256").update(readFileSync(path)).digest("hex") }));
    const syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "policyFindAllMutationsDirs");
    expect(declaration).toBeDefined();
    const code = declaration!.getText(syntax).replace(/^export\s+/u, "") + "\nreturn policyFindAllMutationsDirs;";
    const compilers = [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText];
    const admitted = [...vector.includedRoots.map((sourcePath: string) => ({ sourcePath, observedKind: "directory", repositoryBoundary: "repository" })), ...vector.filePaths.map((sourcePath: string) => ({ sourcePath, observedKind: "file", repositoryBoundary: "repository", worktreeMode: "100644" }))];
    const scenarios = [{ name: "authored-facets", admission: { status: "complete", observations: admitted } }, ...vector.rejections.map((row) => ({ name: row.name, admission: { status: "rejected", diagnostics: [{ code: row.name }], observations: [] } }))];
    const observations: { compiler: number; scenario: string; roots: string[]; error: string | null; passed: boolean }[] = [];
    for (const [compiler, javascript] of compilers.entries()) for (const scenario of scenarios) {
      const operation = new Function("mutationTaxonomySourceAdmission", "POLICY_MUTATIONS_FACET", "mutationTaxonomyCompare", javascript)(() => { throw new Error("default source admission must not run"); }, "🧬️mutations", (left: string, right: string) => Buffer.from(left).compare(Buffer.from(right))) as (root: string, admission: unknown) => string[];
      let roots: string[] = [], error: string | null = null;
      try { roots = operation("/virtual-workspace", scenario.admission); } catch (failure) { error = String(failure); }
      const passed = scenario.name === "authored-facets" ? error === null && JSON.stringify(roots) === JSON.stringify([...vector.includedRoots].sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) : error !== null && /source admission is rejected/u.test(error);
      observations.push({ compiler, scenario: scenario.name, roots, error, passed });
    }
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for retained mutation discovery evidence.");
    mkdirSync(artifactRoot, { recursive: true });
    const run = mkdtempSync(join(artifactRoot, "🧫️discovery-virtual-"));
    const sourceStable = readFileSync(sourcePath, "utf8") === source;
    const inputsStable = inputs.every((entry) => createHash("sha256").update(readFileSync(entry.path)).digest("hex") === entry.sha256);
    writeFileSync(join(run, "🔣️.json"), JSON.stringify({ sourceHash: createHash("sha256").update(source).digest("hex"), helperHash: createHash("sha256").update(code).digest("hex"), sourceStable, inputs, inputsStable, observations }, null, 2) + "\n");
    expect(sourceStable).toBe(true);
    expect(inputsStable).toBe(true);
    expect(observations.filter((row) => !row.passed)).toEqual([]);
  });
  test("mutation root discovery actual filesystem agrees with independent directory enumeration", async () => {
    const ts = await import("typescript");
    const inputRoot = join(contractRoot, "../🧫️fixtures/🌱️mutation-root-discovery");
    const vector = JSON.parse(readFileSync(join(inputRoot, "🔣️.json"), "utf8")) as { physicalOwners: string[]; includedRoots: string[] };
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for retained mutation discovery evidence.");
    mkdirSync(artifactRoot, { recursive: true });
    const run = mkdtempSync(join(artifactRoot, "🧫️discovery-physical-"));
    const workspace = join(run, "workspace");
    const expected = vector.includedRoots.filter((path) => vector.physicalOwners.includes(path.split("/")[0])).sort();
    for (const path of expected) {
      mkdirSync(join(workspace, path), { recursive: true });
      writeFileSync(join(workspace, path, "🦀️.rs"), "pub enum ProbeMutation {}\n");
    }
    const sourcePath = join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts");
    const source = readFileSync(sourcePath, "utf8");
    const inputs = [sourcePath, join(inputRoot, "🔣️.json"), import.meta.path].map((path) => ({ path, sha256: createHash("sha256").update(readFileSync(path)).digest("hex") }));
    const syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "policyFindAllMutationsDirs");
    expect(declaration).toBeDefined();
    const code = declaration!.getText(syntax).replace(/^export\s+/u, "") + "\nreturn policyFindAllMutationsDirs;";
    const compilers = [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText];
    const oracle = fastGlob.sync("**/🧬️mutations", { cwd: workspace, onlyDirectories: true, followSymbolicLinks: false }).sort();
    const admission = { status: "complete", observations: oracle.map((sourcePath) => ({ sourcePath, observedKind: "directory", repositoryBoundary: "repository" })) };
    const observed = compilers.map((javascript) => new Function("mutationTaxonomySourceAdmission", "POLICY_MUTATIONS_FACET", "mutationTaxonomyCompare", javascript)(() => { throw new Error("default source admission must not run"); }, "🧬️mutations", (left: string, right: string) => Buffer.from(left).compare(Buffer.from(right)))(workspace, admission));
    const sourceStable = readFileSync(sourcePath, "utf8") === source;
    const inputsStable = inputs.every((entry) => createHash("sha256").update(readFileSync(entry.path)).digest("hex") === entry.sha256);
    writeFileSync(join(run, "🔣️.json"), JSON.stringify({ sourceHash: createHash("sha256").update(source).digest("hex"), sourceStable, inputs, inputsStable, expected, oracle, observed }, null, 2) + "\n");
    expect(sourceStable).toBe(true);
    expect(inputsStable).toBe(true);
    expect(oracle).toEqual(expected);
    for (const roots of observed) expect(roots).toEqual(expected);
  });
  test("captures empty structural mutation directories in the source admission epoch", () => {
    const root = mutationFixtureRoot("semio-empty-mutation-root-capture-");
    try {
      const mutations = mutationRoot(root);
      mkdirSync(mutations, { recursive: true });
      mkdirSync(join(root, "compose", "🧬️mutations"), { recursive: true });
      const index = mutationTaxonomySourceIndex(root, {});
      const observed = index.admission.observations.filter((observation) => observation.observedKind === "directory" && observation.sourcePath.endsWith("/🧬️mutations")).map((observation) => observation.sourcePath).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
      const oracle = fastGlob.sync("**/🧬️mutations", { cwd: root, onlyDirectories: true, followSymbolicLinks: false, dot: true }).filter((path) => !path.split("/").some((segment) => segment.toLocaleLowerCase("en-US") === "compose")).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
      expect(observed).toEqual(oracle);
      expect(index.roots).toEqual(oracle);
      rmSync(mutations, { recursive: true, force: true });
      expect(mutationTaxonomySourceIndex(root, {}).sourceTreeDigest).not.toBe(index.sourceTreeDigest);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);
  test("fails closed for missing, opaque, escaped, and symlinked explicit scopes", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔭️mutation-scope/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      acceptedSelections: string[];
      missingSelections: string[];
      rejectedSelections: string[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-scope-");
    try {
      expect(golden.schemaVersion).toBe(1);
      const aggregate = `${golden.mutationRoot}/🦀️.rs`;
      mkdirSync(join(root, golden.mutationRoot), { recursive: true });
      writeFileSync(join(root, aggregate), golden.aggregate);
      const oracleRoots = fastGlob.sync("**/🧬️mutations/🦀️.rs", { cwd: root, onlyFiles: true, followSymbolicLinks: false }).map((path) => path.slice(0, -"/🦀️.rs".length));
      expect(oracleRoots).toEqual([golden.mutationRoot]);
      const initialView = mutationTaxonomyStructuralView(mutationTaxonomySourceIndex(root, {}));
      for (const selection of golden.acceptedSelections) expect(policyMutationStructuralBreachesView(initialView, [selection])).toEqual([]);
      for (const selection of golden.missingSelections) {
        expect(oracleRoots).not.toContain(selection.replaceAll("\\", "/"));
        expect(() => policyMutationStructuralBreachesView(initialView, [selection])).toThrow(/mutation.*scope/iu);
      }
      for (const selection of golden.rejectedSelections) expect(() => policyMutationStructuralBreachesView(initialView, [selection])).toThrow(/mutation.*scope/iu);
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
  }, 30_000);
  test("recognizes exact leaf module identities in codecs without trusting comments or prefixes", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔭️mutation-scope/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      codecCases: { source: string; accepted: boolean }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-codec-identity-");
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
  }, 30_000);
}
