/** 🔌️ Law of provider-aware build inputs: a command whose closure defines framework script provider ports hashes the designation, its schema and the import closure of the designated module of every port it defines (every designated module when a port is defined by a name no literal states), never the modules of other ports, so editing a provider invalidates exactly the commands that can load it. The scenario lives in `./🔣️.json` as language-agnostic data; the real repository is checked against independent TypeScript-compiler and esbuild oracles. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { build } from "esbuild";
import ts from "typescript";
import { findWorkspaceRoot } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { cacheInternals, createScriptInputCache } from "../../🟨️.mjs";
import repositoryDesignation from "../../../../../🏃️process/🔌️providers/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { buildSchedulingPrefixesV1 } from "../../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";

const workspace = findWorkspaceRoot(import.meta.dir);
const library = resolve(import.meta.dir, "../../../../..");
const designationPath = join(library, "🏃️process/🔌️providers/🔣️.json");
const schemaPath = join(library, "🏃️process/🔌️providers/🧬️schema/🔣️.json");
const loaderPath = resolve(library, "../../../../🔨️modules/🏃️process/🔌️providers/🟦️.ts");
const prefix = "{workspaceRoot}/";
const presentation = new Set<string>(JSON.parse(readFileSync(join(library, "🏗️build/🔣️policy.json"), "utf8")).presentationResources);
const scheduling = buildSchedulingPrefixesV1();

const inputsOf = async (directory: string, cache = createScriptInputCache()): Promise<readonly string[]> => {
  const target = { options: { cwd: relative(workspace, directory), command: "repo ./📜️script.ts run" } };
  const declarations: unknown[] = await cacheInternals.genericTargetCommandInputs(target, workspace, [], cache);
  return declarations.filter((input): input is string => typeof input === "string").map(input => input.slice(prefix.length));
};

const hashOf = (inputs: readonly string[]): string => {
  const hash = createHash("sha256");
  for (const input of [...inputs].sort()) { const file = join(workspace, input); hash.update(input + "\0").update(existsSync(file) ? readFileSync(file) : "absent"); }
  return hash.digest("hex");
};

test("the provider input corpus satisfies its schema", () => {
  const validate = new Ajv({ strict: false }).compile(corpusSchema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
});

test("editing a designated provider module changes the input hash of exactly the commands that load providers", async () => {
  const scratch = process.env.SEMIO_TEST_SCRATCH_DIR, base = scratch && !relative(workspace, scratch).startsWith("..") ? scratch : workspace;
  const root = mkdtempSync(join(base, "provider-inputs-")), relativeRoot = relative(workspace, root);
  const write = (name: string, content: string) => { mkdirSync(dirname(join(root, name)), { recursive: true }); writeFileSync(join(root, name), content); };
  const designation = (value: unknown) => write("designation.json", JSON.stringify(value));
  const cache = () => createScriptInputCache({ scriptProviderDesignation: join(root, "designation.json") });
  const measure = async () => Object.fromEntries(await Promise.all(corpus.cases.map(async row => [row.id, await inputsOf(join(root, row.directory), cache())] as const)));
  try {
    const specifier = relative(join(root, "loads"), loaderPath);
    for (const [name, content] of Object.entries(corpus.files)) write(name, content.replace("{loader}", specifier));
    designation(corpus.designation);
    const baseline = await measure(), baselineHashes = Object.fromEntries(Object.entries(baseline).map(([id, inputs]) => [id, hashOf(inputs)]));
    for (const row of corpus.cases) {
      const inputs = baseline[row.id]!;
      for (const name of row.includes) expect(inputs, `${row.id} includes ${name}`).toContain(`${relativeRoot}/${name}`);
      for (const name of row.excludes) expect(inputs, `${row.id} excludes ${name}`).not.toContain(`${relativeRoot}/${name}`);
      expect(inputs.includes(relative(workspace, schemaPath)), `${row.id} designation schema`).toBe(row.definesPorts);
      expect(inputs.includes(relative(workspace, loaderPath)), `${row.id} loader`).toBe(row.importsLoader);
    }
    for (const edit of corpus.edits) {
      const original = readFileSync(join(root, edit.file), "utf8");
      write(edit.file, "designation" in edit ? JSON.stringify(edit.designation) : edit.content!);
      try {
        const edited = await measure();
        for (const row of corpus.cases) {
          expect(hashOf(edited[row.id]!) !== baselineHashes[row.id], `${edit.id} changes ${row.id}`).toBe(edit.changes.includes(row.id));
          for (const name of (edit.gains as Record<string, string[]> | undefined)?.[row.id] ?? []) expect(edited[row.id], `${edit.id} gains ${name}`).toContain(`${relativeRoot}/${name}`);
        }
      } finally { write(edit.file, original); }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 120_000);

const definedPorts = (file: string): readonly string[] => {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true), names = new Set<string>();
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "defineScriptProviderV1") names.add(node.arguments[0] !== undefined && ts.isStringLiteralLike(node.arguments[0]) ? node.arguments[0].text : "*");
    ts.forEachChild(node, visit);
  };
  visit(source);
  return [...names].sort();
};

test("the provider port scan never misses a port an independent TypeScript compiler scan finds and equals it in every designated provider-port module", async () => {
  const { scriptProviderPortsV1 } = await import("../../🧭️script-inputs/🟨️.mjs");
  const owners = [...new Bun.Glob("🧰️framework/🔨️modules/**/*.ts").scanSync({ cwd: workspace, absolute: true })].filter(path => !path.includes("node_modules") && readFileSync(path, "utf8").includes("defineScriptProviderV1"));
  const defining = owners.filter(path => definedPorts(path).length > 0);
  expect(defining.length).toBeGreaterThanOrEqual(Object.keys(repositoryDesignation.providers).length - 1);
  for (const file of defining) {
    const scanned = [...scriptProviderPortsV1(readFileSync(file, "utf8"))], oracle = definedPorts(file);
    expect(scanned.includes("*") || oracle.every(name => scanned.includes(name)), relative(workspace, file)).toBe(true);
  }
  for (const file of owners.filter(path => /\/🔌️provider\/🟦️\.ts$/u.test(path))) expect([...scriptProviderPortsV1(readFileSync(file, "utf8"))], relative(workspace, file)).toEqual([...definedPorts(file)]);
});

test("every real command hashes the designation and the closure of exactly the designated repository provider modules whose ports its closure defines", async () => {
  const designated = Object.entries(repositoryDesignation.providers).map(([name, path]) => [name, resolve(dirname(designationPath), path)] as const);
  expect(designated.length).toBe(Object.keys(repositoryDesignation.providers).length);
  const scripts = ["🔨️modules/🎭️actor/📦️packages/🦀️rust", "🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript", "📦️packages/🟦️typescript", "🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation"].map(path => join(workspace, "🧰️framework", path));
  const cache = createScriptInputCache();
  let requestedSomewhere = false, skippedSomewhere = false;
  for (const directory of scripts) {
    const inputs = await inputsOf(directory, cache), requested = new Set(inputs.filter(path => /\.[cm]?[jt]sx?$/u.test(path)).flatMap(path => definedPorts(join(workspace, path))));
    for (const [name, module] of designated) {
      const wanted = requested.has("*") || requested.has(name), included = inputs.includes(relative(workspace, module));
      if (wanted) expect(included, `${relative(workspace, directory)} -> ${name}`).toBe(true);
      requestedSomewhere ||= wanted; skippedSomewhere ||= !included;
    }
    if (requested.size) for (const file of [designationPath, schemaPath]) expect(inputs, `${relative(workspace, directory)} -> ${relative(workspace, file)}`).toContain(relative(workspace, file));
  }
  expect(requestedSomewhere && skippedSomewhere).toBe(true);
  for (const [name, module] of designated) {
    const bundle = await build({ absWorkingDir: workspace, entryPoints: [module], bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    const closure = Object.keys(bundle.metafile!.inputs).map(path => relative(workspace, resolve(workspace, path.replace(/ with \{.*\}$/u, ""))));
    for (const directory of scripts) {
      const inputs = await inputsOf(directory, cache);
      if (inputs.includes(relative(workspace, module))) for (const file of closure.filter(path => !presentation.has(path) && !scheduling.some(prefix => path.startsWith(prefix)))) expect(inputs, `${name} closure member ${file}`).toContain(file);
    }
  }
}, 240_000);
