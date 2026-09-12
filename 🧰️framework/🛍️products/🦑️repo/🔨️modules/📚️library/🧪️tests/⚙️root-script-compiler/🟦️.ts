import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { transformSync } from "esbuild";
import glob from "fast-glob";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";
import { policySnakeToCamel } from "../../🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";
import { policyExtractProtobufSchemaFields } from "../../🧬️schema/🔍️field-discovery/🛰️protobuf/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const defaultArtifactRoot = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/sol-root-schema-field-extraction/root-script-compiler");
const artifactRoot = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? defaultArtifactRoot);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/⚙️root-script-compiler/🔣️.json"), "utf8"));
const text = readFileSync(join(root, "📜️script.ts"), "utf8");
const source = ts.createSourceFile("📜️script.ts", text, ts.ScriptTarget.Latest, true);
const functions = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && !!node.body);

/** 🧬️ Executes the final effective declaration with independent Bun and esbuild compilers. */
function implementations(name: string, implementationText = text): Function[] {
  const implementationSource = ts.createSourceFile(`${name}.ts`, implementationText, ts.ScriptTarget.Latest, true);
  const declaration = implementationSource.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && !!node.body && node.name?.text === name).at(-1)!;
  const code = declaration.getText(implementationSource).replace(/^export /u, "");
  return [new Bun.Transpiler({ loader: "ts" }).transformSync(code), transformSync(code, { loader: "ts", target: "es2022" }).code].map((compiled) => new Function("existsSync", "dirname", "readFileSync", "join", "resolve", `${compiled}\nreturn ${name};`)(existsSync, dirname, readFileSync, join, resolve));
}

test("the root task router has one implementation per top-level function", () => {
  const counts = new Map<string, number>();
  for (const node of functions) counts.set(node.name!.text, (counts.get(node.name!.text) ?? 0) + 1);
  expect([...counts].filter(([, count]) => count > 1)).toEqual([]);
  for (const name of vector.declarations) expect(counts.get(name)).toBe(1);
  const moved = new Set<string>();
  for (const owner of vector.fieldOwners) {
    const ownerText = readFileSync(join(root, owner.path), "utf8");
    const ownerSource = ts.createSourceFile(owner.path, ownerText, ts.ScriptTarget.Latest, true);
    const declarations = ownerSource.statements.flatMap((node) => (ts.isFunctionDeclaration(node) || ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node)) && node.name ? [node.name.text] : []);
    expect(declarations).toEqual(owner.declarations);
    for (const name of declarations) moved.add(name);
  }
  expect(functions.map((node) => node.name!.text).filter((name) => moved.has(name))).toEqual([]);
});

test("Bun and esbuild accept the complete actual root task router", () => {
  expect(new Bun.Transpiler({ loader: "ts" }).transformSync(text).length).toBeGreaterThan(0);
  expect(transformSync(text, { loader: "ts", format: "esm", target: "es2022" }).code.length).toBeGreaterThan(0);
});

test("eager policy vocabulary avoids workspace output reads while default lookup stays strict", () => {
  const expected = vector.eagerVocabulary, names = Object.keys(expected.values);
  const vocabularyText = readFileSync(join(root, expected.sourceOwner.path), "utf8");
  const vocabularySource = ts.createSourceFile(expected.sourceOwner.path, vocabularyText, ts.ScriptTarget.Latest, true);
  const variableDeclarations = (syntax: ts.SourceFile) => syntax.statements.filter(ts.isVariableStatement).flatMap((statement) => statement.declarationList.declarations).filter((node) => ts.isIdentifier(node.name));
  const rootDeclarations = variableDeclarations(source), ownerDeclarations = variableDeclarations(vocabularySource);
  expect(ownerDeclarations.filter((node) => expected.sourceOwner.declarations.includes(node.name.getText(vocabularySource))).map((node) => node.name.getText(vocabularySource))).toEqual(expected.sourceOwner.declarations);
  const declarations = names.map((name: string) => {
    const owner = ownerDeclarations.find((node) => node.name.getText(vocabularySource) === name);
    if (owner) return { node: owner, syntax: vocabularySource };
    const local = rootDeclarations.find((node) => node.name.getText(source) === name);
    if (!local) throw new Error(`Missing eager policy declaration ${name}`);
    return { node: local, syntax: source };
  });
  const discovery = ts.createSourceFile("discovery.ts", readFileSync(join(root, library, "🔍️discovery/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const helpers = discovery.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && ["canonicalFilenamesForKind", "canonicalPrimaryFilenameForKind"].includes(node.name?.text ?? ""));
  expect(helpers).toHaveLength(2);
  const input = [...helpers.map((node) => node.getText(discovery).replace(/^export /u, "")), ...declarations.map(({ node, syntax }) => `const ${node.getText(syntax)};`)].join("\n");
  const taxonomy = JSON.parse(readFileSync(join(root, library, "🔣️taxonomy.json"), "utf8"));
  for (const compiled of [new Bun.Transpiler({ loader: "ts" }).transformSync(input), transformSync(input, { loader: "ts", target: "es2022" }).code]) {
    let workspaceReads = 0, lookups = 0;
    const catalog = { ...taxonomy, fileKinds: new Proxy(taxonomy.fileKinds, { get: (target, key) => { lookups++; return Reflect.get(target, key); } }) };
    const actual = new Function("loadCatalogTaxonomy", "loadTaxonomy", `${compiled}\nreturn { values: {${names.join(",")}}, strictLookup: () => canonicalPrimaryFilenameForKind("json") };`)(() => catalog, () => { workspaceReads++; throw new Error(expected.missingActiveOutput); });
    expect(actual.values).toEqual(expected.values);
    expect(lookups).toBe(expected.lookupCount);
    expect(workspaceReads).toBe(0);
    expect(() => actual.strictLookup()).toThrow(expected.missingActiveOutput);
    expect(workspaceReads).toBe(1);
  }
  console.log("[DEBUG] root policy vocabulary matched both compilers; implicit workspace lookup remained strict");
});

test("field naming retains the actual Bun schema-extractor semantics", async () => {
  const contract = vector.fieldOwners.find((owner: { declarations: string[] }) => owner.declarations.includes("policySnakeToCamel"));
  const contractText = readFileSync(join(root, contract.path), "utf8");
  for (const implementation of implementations("policySnakeToCamel", contractText)) for (const row of vector.fieldNames) expect(implementation(row.input)).toBe(row.output);
  const contractSource = ts.createSourceFile(contract.path, contractText, ts.ScriptTarget.Latest, true);
  const declaration = contractSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "policySnakeToCamel")!;
  const compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(declaration.getText(contractSource));
  const module = await import(`data:text/javascript;base64,${Buffer.from(compiled).toString("base64")}`);
  for (const row of vector.fieldNames) expect(module.policySnakeToCamel(row.input)).toBe(row.output);
  for (const row of vector.fieldNames) expect(policySnakeToCamel(row.input)).toBe(row.output);
  console.log("[DEBUG] Bun module field-name semantics", JSON.stringify(vector.fieldNames));
  const rust = `pub struct Fixture { ${vector.schemaFields.map((name: string) => `pub ${name}: String,`).join(" ")} }`;
  const protobuf = `message Fixture { ${vector.schemaFields.map((name: string, index: number) => `string ${name} = ${index + 1};`).join("\n")} }`;
  const expected = vector.schemaFields.map((name: string) => vector.fieldNames.find((row: { input: string }) => row.input === name).output);
  expect(policyExtractRustSchemaFields(rust, "Fixture").fields.map((field: { name: string }) => field.name)).toEqual(expected);
  expect(policyExtractProtobufSchemaFields(protobuf, "Fixture").fields.map((field: { name: string }) => field.name)).toEqual(expected);
  console.log("[DEBUG] actual Rust and Protobuf policy field names", JSON.stringify(expected));
});

test("glue path discovery retains its declared targets with independent compiler parity", () => {
  expect(vector.fixtureRetention).toEqual({ ownerPath: "📓️root-script-compiler/🧾️runs", prefix: "🧪️glue-" });
  const parent = join(artifactRoot, vector.fixtureRetention.ownerPath);
  let ancestor = root;
  for (const part of relative(root, parent).split(/[\\/]/u)) {
    ancestor = join(ancestor, part);
    try { lstatSync(ancestor); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; mkdirSync(ancestor); }
    const state = lstatSync(ancestor);
    expect(state.isDirectory() && !state.isSymbolicLink()).toBe(true);
  }
  const directory = mkdtempSync(join(parent, vector.fixtureRetention.prefix));
  writeFileSync(join(directory, "📝️.md"), "# Root Script Compiler Fixture\n\nFresh authored inputs and active test evidence are retained; no historical fixture is reconstructed.\n", { flag: "wx" });
  for (const [path, content] of Object.entries(vector.glue.inputs)) { mkdirSync(dirname(join(directory, path)), { recursive: true }); writeFileSync(join(directory, path), content as string); }
  const oracle = glob.sync("**/*.rs", { cwd: directory, onlyFiles: true, dot: true }).filter((path) => path !== vector.glue.entry).sort();
  expect(oracle).toEqual(vector.glue.targets);
  for (const implementation of implementations("policyCollectGluePathTargets")) expect([...implementation(join(directory, vector.glue.entry))].map((path) => relative(directory, path as string).replaceAll("\\", "/")).sort()).toEqual(vector.glue.targets);
});

test("registers the root compiler gate through Nx and both launch catalogs", () => {
  const expected = vector.execution;
  const project = JSON.parse(readFileSync(join(root, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});
