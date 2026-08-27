import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { transformSync } from "esbuild";
import glob from "fast-glob";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const text = readFileSync(join(root, "📜️script.ts"), "utf8");
const source = ts.createSourceFile("📜️script.ts", text, ts.ScriptTarget.Latest, true);
const functions = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && !!node.body);

/** 🧬️ Executes the final effective declaration with independent Bun and esbuild compilers. */
function implementations(name: string): Function[] {
  const declaration = functions.filter((node) => node.name?.text === name).at(-1)!;
  const code = declaration.getText(source);
  return [new Bun.Transpiler({ loader: "ts" }).transformSync(code), transformSync(code, { loader: "ts", target: "es2022" }).code].map((compiled) => new Function("existsSync", "dirname", "readFileSync", "join", "resolve", `${compiled}\nreturn ${name};`)(existsSync, dirname, readFileSync, join, resolve));
}

test("the root task router has one implementation per top-level function", () => {
  const counts = new Map<string, number>();
  for (const node of functions) counts.set(node.name!.text, (counts.get(node.name!.text) ?? 0) + 1);
  expect([...counts].filter(([, count]) => count > 1)).toEqual([]);
  for (const name of vector.declarations) expect(counts.get(name)).toBe(1);
});

test("Bun and esbuild accept the complete actual root task router", () => {
  expect(new Bun.Transpiler({ loader: "ts" }).transformSync(text).length).toBeGreaterThan(0);
  expect(transformSync(text, { loader: "ts", format: "esm", target: "es2022" }).code.length).toBeGreaterThan(0);
});

test("field naming retains the actual Bun schema-extractor semantics", async () => {
  for (const implementation of implementations("policySnakeToCamel")) for (const row of vector.fieldNames) expect(implementation(row.input)).toBe(row.output);
  const declarations = functions.filter((node) => node.name?.text === "policySnakeToCamel").map((node) => node.getText(source)).join("\n");
  const compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(`${declarations}\nexport { policySnakeToCamel };`);
  const module = await import(`data:text/javascript;base64,${Buffer.from(compiled).toString("base64")}`);
  for (const row of vector.fieldNames) expect(module.policySnakeToCamel(row.input)).toBe(row.output);
  console.log("[DEBUG] Bun module field-name semantics", JSON.stringify(vector.fieldNames));
  const actual = await import(join(root, "📜️script.ts"));
  const rust = `pub struct Fixture { ${vector.schemaFields.map((name: string) => `pub ${name}: String,`).join(" ")} }`;
  const protobuf = `message Fixture { ${vector.schemaFields.map((name: string, index: number) => `string ${name} = ${index + 1};`).join("\n")} }`;
  const expected = vector.schemaFields.map((name: string) => vector.fieldNames.find((row: { input: string }) => row.input === name).output);
  expect(actual.policyExtractRustSchemaFields(rust, "Fixture").fields.map((field: { name: string }) => field.name)).toEqual(expected);
  expect(actual.policyExtractProtobufSchemaFields(protobuf, "Fixture").fields.map((field: { name: string }) => field.name)).toEqual(expected);
  console.log("[DEBUG] actual Rust and Protobuf policy field names", JSON.stringify(expected));
});

test("glue path discovery retains its declared targets with independent compiler parity", () => {
  const directory = mkdtempSync(join(ticket, "🧪️root-script-glue-"));
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
