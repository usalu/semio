import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { parse, type ParseError } from "jsonc-parser";
import * as ts from "typescript";
import { loadCatalogTaxonomy, semanticArtifactEmptyFacetProjectionAuthority, type Taxonomy } from "../../🔍️discovery/🟦️component.ts";

const libraryRoot = resolve(import.meta.dir, "../..");
const goldenPath = join(libraryRoot, "📦️packages/🟦️typescript/🧫️fixtures/🧪️artifact-empty-facet-authority/🔣️.json");
const goldenBytes = readFileSync(goldenPath);
const golden = JSON.parse(goldenBytes.toString()) as Readonly<{
  schemaVersion: number; contractId: string; sourceRoot: string; sourceFilename: string; destinationFilename: string;
  cases: readonly Readonly<{ id: string; owner: string; form: string | null; root?: string; filename?: string; fileKindId?: string }>[];
}>;
const oracleBytes = readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8");
const oracle = JSON.parse(oracleBytes);
const validateOwner = new Ajv({ strict: true, allErrors: true }).compile(oracle);
const forms = oracle.oneOf.map((branch: { properties: { ownerForm: { const: string } } }) => branch.properties.ownerForm.const) as string[];
const taxonomy = loadCatalogTaxonomy();
const functionNames = ["canonicalFilenamesForKind", "canonicalFilenameForKind", "canonicalSemanticDirectoryName", "semanticDirectoryKindId", "semanticArtifactEmptyFacetProjectionAuthority"] as const;

/** 🔬️ Compiles the exact pure authority closure without granting it filesystem access. */
function independentlyCompiledAuthorities(): readonly (typeof semanticArtifactEmptyFacetProjectionAuthority)[] {
  const path = join(libraryRoot, "🔍️discovery/🟦️component.ts"), source = readFileSync(path, "utf8");
  const syntax = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const code = functionNames.map((name) => {
    const matches = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === name);
    expect(matches.length, name).toBe(1);
    return matches[0]!.getText(syntax).replace(/^export\s+/u, "");
  }).join("\n");
  return [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText].map((javascript) => new Function("basename", "dirname", "loadTaxonomy", `${javascript}\nreturn semanticArtifactEmptyFacetProjectionAuthority;`)(basename, dirname, () => { throw new Error("Pure authority unexpectedly requested workspace discovery"); }) as typeof semanticArtifactEmptyFacetProjectionAuthority);
}

test("retains the original nineteen-case empty-facet input with independent JSON parsing", () => {
  expect(goldenBytes.length).toBe(3154);
  expect(createHash("sha256").update(goldenBytes).digest("hex")).toBe("d03f52fd16ef6e87f6916925d6ebadc13c1e6b3a0b1083624f5cb32764672dbd");
  const errors: ParseError[] = [];
  expect(parse(goldenBytes.toString(), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(golden);
  expect(parse(oracleBytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(oracle);
  expect(errors).toEqual([]);
  expect(golden.cases).toHaveLength(19);
  expect(golden.cases.filter((row) => row.form !== null)).toHaveLength(8);
  expect(new Set(golden.cases.map((row) => row.id)).size).toBe(19);
  expect(new Set(forms).size).toBe(7);
});

test("matches all empty-facet decisions with independent owner schemas and both compilers", () => {
  const implementations = [semanticArtifactEmptyFacetProjectionAuthority, ...independentlyCompiledAuthorities()];
  for (const row of golden.cases) {
    const root = row.root ?? golden.sourceRoot, sourceFilename = row.filename ?? golden.sourceFilename, fileKindId = row.fileKindId ?? "markdown";
    const sourcePath = `${root}/${row.owner}/${sourceFilename}`;
    const matches = forms.filter((ownerForm) => validateOwner({ root, sourceFilename, fileKindId, ownerSegments: row.owner.split("/"), ownerForm }));
    expect(matches, row.id).toEqual(row.form === null ? [] : [row.form]);
    const core = { contractId: golden.contractId, sourcePath };
    const expected = row.form === null ? { ...core, disposition: "unclaimed" } : { ...core, ownerForm: row.form, destinationPath: `${root}/${row.owner}/${golden.destinationFilename}`, disposition: "project" };
    for (const implementation of implementations) expect(implementation({ sourcePath, sourceFileKindId: fileKindId }, taxonomy), row.id).toEqual(expected);
  }
});

test("keeps exact structural ownership distinct from a basename-only match", () => {
  const matchingNames = golden.cases.filter((row) => (row.filename ?? golden.sourceFilename) === golden.sourceFilename && (row.fileKindId ?? "markdown") === "markdown");
  expect(matchingNames.some((row) => row.form === null)).toBe(true);
  const sourcePath = `${golden.sourceRoot}/${golden.cases[0]!.owner}/${golden.sourceFilename}`;
  const missing: Taxonomy = { ...taxonomy, semanticOwnedFileProjectionContracts: Object.fromEntries(Object.entries(taxonomy.semanticOwnedFileProjectionContracts).filter(([id]) => id !== golden.contractId)) };
  const contract = taxonomy.semanticOwnedFileProjectionContracts[golden.contractId];
  if (contract?.contractKind !== "semantic-facet-primary-file") throw new Error("Expected the existing authored empty-facet contract");
  const ambiguous: Taxonomy = { ...taxonomy, semanticOwnedFileProjectionContracts: { ...taxonomy.semanticOwnedFileProjectionContracts, [golden.contractId]: { ...contract, ownerPathPatterns: { ...contract.ownerPathPatterns, "duplicate-test-owner": contract.ownerPathPatterns["plugin-commands"]! } } } };
  for (const implementation of [semanticArtifactEmptyFacetProjectionAuthority, ...independentlyCompiledAuthorities()]) {
    expect(() => implementation({ sourcePath, sourceFileKindId: "markdown" }, missing)).toThrow("empty-facet primary-leaf contract");
    expect(implementation({ sourcePath, sourceFileKindId: "markdown" }, ambiguous).disposition).toBe("unclaimed");
  }
});

test("registers the empty-facet authority through its closed canonical route", async () => {
  const directory = join(import.meta.dir, "🧪️registration"), bytes = readFileSync(join(directory, "🔣️.json"), "utf8"), vector = JSON.parse(bytes);
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...vector, source: "🟦️component.ts" }, { ...vector, budget: 120000 }, { ...vector, budgetMs: 120000 }, { ...vector, runner: "other" }, { ...vector, launchOrder: 410.198 }]) expect(validate(changed)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  const repoRoot = resolve(libraryRoot, "../../../../.."), packageRelative = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript", packageRoot = join(repoRoot, packageRelative);
  expect(join(repoRoot, vector.source)).toBe(import.meta.filename);
  const project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
  expect(project.targets[vector.target]).toBeDefined();
  expect(project.targets[vector.target]).toEqual({ executor: "nx:run-commands", options: { cwd: packageRelative, command: `bun ./📜️script.ts test ${vector.command}` } });
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8"));
  expect(manifest.scripts[vector.target]).toBe(`nx run @semio-tech/repo-lib:${vector.target}`);
  const path = join(packageRoot, "📜️script.ts"), source = readFileSync(path, "utf8"), syntax = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declarations = syntax.statements.filter((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
  expect(declarations.length).toBe(1);
  const code = `${declarations[0]!.getText(syntax)}\nreturn new TestScript();`;
  for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const invocations: { executable: string; args: string[]; options: { cwd: string } }[] = [];
    class FixtureBundle { root = packageRoot; repoRoot = repoRoot; }
    const router = new Function("BundleScript", "join", "runTestBudgeted", "resolveTestLevel", javascript)(FixtureBundle, join, async (executable: string, args: string[], options: { cwd: string }) => { invocations.push({ executable, args, options }); }, () => { throw new Error("Empty-facet authority fell through to generic routing"); });
    await router.run([vector.command]);
    expect(invocations).toEqual([{ executable: process.execPath, args: ["test", join(repoRoot, vector.source)], options: { cwd: repoRoot } }]);
  }
  for (const filename of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const parseErrors: ParseError[] = [], document = parse(readFileSync(join(repoRoot, filename), "utf8"), parseErrors);
    expect(parseErrors).toEqual([]);
    const entries = document.configurations.filter((row: { name: string }) => row.name === vector.launchName);
    expect(entries).toEqual([{ name: vector.launchName, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/repo-lib:${vector.target} --skip-nx-cache`, cwd: "${workspaceFolder}", presentation: { group: vector.launchGroup, order: vector.launchOrder } }]);
    expect(document.configurations.filter((row: { presentation?: { group: string; order: number } }) => row.presentation?.group === vector.launchGroup && row.presentation.order === vector.launchOrder)).toHaveLength(1);
  }
});
