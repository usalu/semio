import { mutationRoot, prepareMutationFixtureRoot, mutationFixtureRoot } from "../../🧬️mutation-authority/🧫️fixture/🟦️.ts";
import Ajv, { type AnySchema } from "ajv";
import { describe, expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { newScaffoldMutationTree } from "../../../🏗️authoring/🧬️mutation-tree/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, canonicalPrimaryFilenameForKind, loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { createRustMutationCodecOwnershipInspector } from "../../../🔍️discovery/🛡️mutation-codec-ownership/🟦️.ts";
import { inspectRustMutationMetadataFacts, inspectRustStructure } from "../../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { taxonomyCliArtifactPath } from "../../../🧹️normalization/🎮️command-contract/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { mutationTaxonomySourceIndex } from "../../../🧹️normalization/🧬️mutation/📇️index/🟦️.ts";
import { MUTATION_STRUCTURAL_POLICY_KINDS, policyMutationStructuralBreaches, policyMutationStructuralBreachesView } from "../../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { mutationTaxonomyStructuralView } from "../../../🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts";
import { planMutationTaxonomy, runMutationTaxonomyCli } from "../../../🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts";
import { inventoryMutationTaxonomy } from "../../../🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

/** 🧷️ Admits one authored scenario file as the JSON Schema document its row declares. */
function schemaFile(files: Readonly<Record<string, unknown>>, path: string): AnySchema {
  const body = files[path];
  if (typeof body === "boolean" || (body !== null && typeof body === "object")) return body as AnySchema;
  throw Error(`scenario file ${path} is not a JSON Schema document`);
}

describe("direct mutation ownership", () => {
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

  test("recognizes a binary leaf tag only when its numeric identity equals the descriptor", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔭️mutation-scope/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      binaryCases: { source: string; accepted: boolean }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-binary-identity-");
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
  }, 30_000);

  test("rejects executable root aggregate codecs while preserving generic framing and unrelated syntax", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔐️mutation-codec-ownership/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      cases: { name: string; source: string; expected: string[] }[];
    };
    const root = mutationFixtureRoot("semio-mutation-codec-ownership-");
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
        expect(inspect(vector.source).map(({ kind }) => kind)).toEqual<readonly string[]>(vector.expected);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/codec-ownership").length > 0).toBe(vector.expected.length > 0);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }  }, 30000);

  test("rejects empty aggregates, inline variants, and orphaned direct folders", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔭️mutation-scope/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      aggregateCases: { source: string; kind: string; variantCount: number }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-aggregate-");
    try {
      mkdirSync(join(root, golden.mutationRoot, "➕️insert-page"), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, "➕️insert-page", "🦀️.rs"), "pub struct InsertPage;\n");
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
    }  }, 30000);

  test("rejects aggregate-state mutation inputs transitively while accepting semantic prior values", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/📥️mutation-input-carriers/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      compilerPrelude: string;
      compilerAssertion: string;
      cases: { name: string; source: string; aggregate?: string; forbidden: boolean }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-input-");
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
    }  }, 30000);

  test("reports legacy nesting, inline root behavior, sentinels, and snapshot fallbacks at high severity", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-policy-");
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
    const root = mutationFixtureRoot("semio-direct-mutation-policy-catalog-");
    try {
      const mutations = mutationRoot(root);
      mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation { InsertPage(InsertPage) }\n");
      expect(policyMutationStructuralBreaches(root).every(({ priority }) => priority === "high")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);

  test("requires one enabled reachable non-ignored leaf test proven by parsed Rust items", () => {
    const fixturePath = join(contractRoot, "../🧫️fixtures/✅️mutation-test-presence/🔣️.json");
    
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: 1; mutationRoot: string; leaf: string; cases: readonly { name: string; source: string; files: Readonly<Record<string, string>>; links: Readonly<Record<string, string>>; compilerAccepted: boolean; runnableTests: number; policyAccepted: boolean }[] };
    
    const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR?.trim();
    const base = artifactDirectory ? resolve(getWorkspaceRoot(), artifactDirectory) : realpathSync(tmpdir());
    const oracleTimeoutMs = 30_000;
    mkdirSync(base, { recursive: true });
    const root = mkdtempSync(join(base, "semio-mutation-test-presence-"));
    try {
      for (const vector of fixture.cases) {
        const caseRoot = join(root, vector.name);
        mkdirSync(caseRoot, { recursive: true });
        prepareMutationFixtureRoot(caseRoot);
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
    const root = mutationFixtureRoot("semio-direct-mutation-policy-");
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
  }, 30_000);

  test("scaffolds one direct mutation idempotently without overwriting implementation", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-scaffold-");
    try {
      const mutations = mutationRoot(root);
      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      const first = newScaffoldMutationTree(root, relative(root, mutations).replaceAll("\\", "/"), "➕️insert-page", { text: true, binary: true, jsonSchema: true }, false);
      expect(first.created).toContain(`${relative(root, mutations).replaceAll("\\", "/")}/➕️insert-page/🦀️.rs`);
      const descriptor = JSON.parse(readFileSync(join(mutations, "➕️insert-page", "🔣️.json"), "utf8"));
      expect(descriptor).toMatchObject({ schemaVersion: 1, owner: `${relative(root, mutations).replaceAll("\\", "/")}/➕️insert-page`, semanticKind: "insert-page", payloadSchema: "🧬️schema/🔣️.json", textOpcode: "insert-page", invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"] });
      const descriptorSchema = JSON.parse(readFileSync(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json"), "utf8"));
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
  }, 30_000);

  

  test("inventories direct and legacy records as stable language-neutral JSON validated by Ajv", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-inventory-");
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
  }, 30_000);

  

  

  

  test("requires a fresh clean terminal verification before mutation apply can commit", () => {
    const goldenPath = join(contractRoot, "../🧫️fixtures/📋️mutation-inventory/🔣️.json");
    const golden = JSON.parse(readFileSync(goldenPath, "utf8")) as { schemaVersion: number; baseline: string; cases: readonly string[] };
    const root = mutationFixtureRoot("semio-mutation-terminality-");
    const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️08", "☀️12", "TERMINALITY");
    const planPath = join(ticket, "plan.json");
    const applyPath = taxonomyCliArtifactPath(ticket, "apply", "json");
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
  }, 60_000);
});

describe("mutation metadata facts", () => {
  test("records exact declaration and alias syntax without decoy inference", () => {
    const fixturePath = join(contractRoot, "../🧫️fixtures/🪪️mutation-metadata/🔣️.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
    
    
    const facts = inspectRustMutationMetadataFacts(fixture.source);
    expect(facts.declarations.map(({ name, kind, visibility, modulePath, derives, mutationLeaf }) => ({ name, kind, visibility, modulePath, derives, state: mutationLeaf.state, contracts: mutationLeaf.contracts, detail: mutationLeaf.detail }))).toEqual(fixture.expected.declarations);
    expect(facts.crateAliases).toEqual(fixture.expected.aliases);
    expect(facts.manualMutationLeafImpls).toEqual(fixture.expected.manualMutationLeafImpls);
    expect(facts.declarations.some((item) => item.name === "Fake")).toBe(false);
  });
});

describe("schema scope catalog", () => {
    const casesPath = join(contractRoot, "../🧫️fixtures/🧬️schema-scope-catalog/🔣️.json");
    const cases = JSON.parse(readFileSync(casesPath, "utf8")) as { contract: string; cases: { id: string; files: Record<string, unknown>; expected: { scopes: Record<string, { path: string; level: string; exports: Record<string, { file: string; facet: string }>; dependsOn: string[] }>; diagnosticCodes: string[]; placementPaths: string[] }; schemaGrammar?: Record<string, boolean>; schemaCompile?: Record<string, boolean>; grantExamples?: { input: unknown; expected: boolean }[] }[] };
    const authority = JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🧬️schema-scope-catalog/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true });
    const admitScope = ajv.compile(authority);
    const grantAuthority = JSON.parse(readFileSync(join(contractRoot, "../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"), "utf8"));
    ajv.addSchema(grantAuthority);
    const admitGrant = ajv.compile({ $ref: grantAuthority.$id + "#/$defs/Grant" });
    
    for (const row of cases.cases) test("catalogues declared scopes and reports every retired placement and unresolved binding: "+row.id, async () => {
      const { inventorySchemaScopes, renderSchemaCatalog } = await import("../../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts");
      const taxonomy = loadCatalogTaxonomy();
      const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "s"));
      try {
        for (const [rel, body] of Object.entries(row.files)) {
          const abs = join(root, rel);
          expect(abs.length, row.id).toBeLessThanOrEqual(256);
          expect(Array.from(abs).length, row.id).toBeLessThanOrEqual(256);
          mkdirSync(dirname(abs), { recursive: true });
          writeFileSync(abs, typeof body === "string" ? body : `${JSON.stringify(body, null, 2)}\n`);
        }
        for (const [path, expected] of Object.entries(row.schemaGrammar ?? {})) expect(new Ajv({ strict: false }).validateSchema(schemaFile(row.files, path)), row.id).toBe(expected);
        for (const [path, expected] of Object.entries(row.schemaCompile ?? {})) {
          let accepted = true;
          try { new Ajv({ strict: true, strictTypes: false }).compile(schemaFile(row.files, path)); } catch { accepted = false; }
          expect(accepted, row.id).toBe(expected);
        }
        for (const example of row.grantExamples ?? []) expect(admitGrant(example.input), JSON.stringify(admitGrant.errors)).toBe(example.expected);
        const inventory = inventorySchemaScopes(root, taxonomy);
        const scopes = Object.fromEntries(Object.entries(inventory.catalog.scopes).map(([id, scope]) => [id, { path: scope.path, level: scope.level, exports: scope.exports, dependsOn: [...scope.dependsOn] }]));
        for (const scope of Object.values(scopes)) expect(admitScope(scope), JSON.stringify(admitScope.errors)).toBe(true);
        expect({ case: row.id, scopes }).toEqual({ case: row.id, scopes: row.expected.scopes });
        expect({ case: row.id, codes: inventory.diagnostics.map(({ code }) => code).sort() }).toEqual({ case: row.id, codes: [...row.expected.diagnosticCodes].sort() });
        expect({ case: row.id, paths: inventory.placement.map(({ path }) => path).sort() }).toEqual({ case: row.id, paths: [...row.expected.placementPaths].sort() });
        expect(renderSchemaCatalog(inventory.catalog)).toBe(renderSchemaCatalog(inventorySchemaScopes(root, taxonomy).catalog));
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
      if (row === cases.cases.at(-1)) {}
    });

  test("resolves scope ids and export bindings only from declared identity", async () => {
    const { schemaScopeIdFromDocumentId, parseSchemaExportUri, schemaScopeOwnerLevel } = await import("../../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts"), { resolveSchemaFacetKind } = await import("../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts");
    const taxonomy = loadCatalogTaxonomy();
    expect(schemaScopeIdFromDocumentId("https://json.schemas.assets.semio-tech.com/s/trinity/jack/artifact.json", taxonomy)).toBe("s.trinity.jack");
    expect(schemaScopeIdFromDocumentId("https://json.schemas.assets.semio-tech.com/hub/inference/contract.json", taxonomy)).toBe("hub.inference");
    expect(schemaScopeIdFromDocumentId("urn:semio:hub:inference", taxonomy)).toBe(null);
    expect(schemaScopeIdFromDocumentId("https://json.schemas.assets.semio-tech.com/contract.json", taxonomy)).toBe(null);
    expect(parseSchemaExportUri("schema://hub.inference/InferenceApproval", taxonomy)).toEqual({ scopeId: "hub.inference", exportId: "InferenceApproval" });
    expect(parseSchemaExportUri("schema://hub.inference/inferenceApproval", taxonomy)).toBe(null);
    expect(parseSchemaExportUri("local://hub.inference/InferenceApproval", taxonomy)).toBe(null);
    expect(schemaScopeOwnerLevel("✏️s/🔌️plugins/🔱️trinity", taxonomy)).toBe("plugin-root");
    expect(schemaScopeOwnerLevel("🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1", taxonomy)).toBe(null);
    expect(resolveSchemaFacetKind("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema", taxonomy)).toBe("📜️interface");
    expect(resolveSchemaFacetKind("🌎️hub/💡️inference/🧬️schema", taxonomy)).toBe(taxonomy.schemaDefaultFacetKind);
  });

  test("cross-checks the Rust export registry dump against the catalog on the resolution key", async () => {
    const { schemaRustEntryDiagnostics } = await import("../../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts");
    const taxonomy = loadCatalogTaxonomy();
    const casesPath = join(contractRoot, "../🧫️fixtures/🧬️schema-rust-entries/🔣️.json");
    const cases = JSON.parse(readFileSync(casesPath, "utf8")) as { contract: string; cases: { id: string; scopes: Record<string, { path: string; formats: Record<string, string>; exports: Record<string, { file: string; facet: string }> }>; dump: { contractId: string; generator: string; entries: { scope: string; export: string; format: string }[] }; complete: boolean; expectedCodes: string[] }[] };
    const authority = JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🧬️schema-rust-entries/🔣️.json"), "utf8"));
    
    for (const row of cases.cases) {
      const catalog = { contractId: taxonomy.schemaExportResolution.catalogContractId, catalogVersion: 1 as const, generator: taxonomy.schemaExportResolution.generator, taxonomySchemaVersion: taxonomy.schemaVersion, jsonSchemaDialect: taxonomy.schemaJsonDialect, scopes: Object.fromEntries(Object.entries(row.scopes).map(([id, scope]) => [id, { ...scope, level: "artifact-standard-subset", facetKind: taxonomy.schemaDefaultFacetKind, dependsOn: [], hashes: {} }])) };
      const findings = schemaRustEntryDiagnostics(catalog, row.dump, taxonomy, row.complete);
      expect({ case: row.id, codes: findings.map(({ code }) => code).sort() }).toEqual({ case: row.id, codes: [...row.expectedCodes].sort() });
      expect(findings.every(({ path, detail }) => path === taxonomy.schemaExportResolution.catalogPath && detail.length > 16)).toBe(true);
    }
  });

  test("declares one draft-07 dialect that an independent validator accepts", async () => {
    const library = await import("../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts");
    const taxonomy = library.loadCatalogTaxonomy();
    expect(taxonomy.schemaJsonDialect).toBe(taxonomy.mutationPayloadSchemaAuthority.jsonSchemaDialect);
    const document = { $schema: taxonomy.schemaJsonDialect, $id: "https://json.schemas.assets.semio-tech.com/repo/library/probe.json", title: "Probe", type: "object", additionalProperties: false, properties: { id: { type: "string" } }, required: ["id"] };
    const validate = new Ajv({ strict: true }).compile(document);
    expect(validate({ id: "a" })).toBe(true);
    expect(validate({ id: 1 })).toBe(false);
  });
});
