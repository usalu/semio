/** 🔌️ Law of provider-aware build inputs: a command whose script loads framework script providers hashes the designation, its schema and the import closure of every designated provider module, so editing a provider invalidates exactly those commands. The scenario lives in `./🔣️.json` as language-agnostic data; the real repository is checked against the independent esbuild import oracle. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { build } from "esbuild";
import { findWorkspaceRoot } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { cacheInternals, createScriptInputCache } from "../../🟨️.mjs";
import repositoryDesignation from "../../../../../🏃️process/🔌️providers/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const workspace = findWorkspaceRoot(import.meta.dir);
const library = resolve(import.meta.dir, "../../../../..");
const designationPath = join(library, "🏃️process/🔌️providers/🔣️.json");
const schemaPath = join(library, "🏃️process/🔌️providers/🧬️schema/🔣️.json");
const loaderPath = resolve(library, "../../../../🔨️modules/🏃️process/🔌️providers/🟦️.ts");
const prefix = "{workspaceRoot}/";

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
      expect(inputs.includes(relative(workspace, schemaPath)), `${row.id} designation schema`).toBe(row.loadsProviders);
      expect(inputs.includes(relative(workspace, loaderPath)), `${row.id} loader`).toBe(row.loadsProviders);
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

test("every real command that loads providers hashes the import closure of every designated repository provider module", async () => {
  const modules = Object.values(repositoryDesignation.providers).map(path => resolve(dirname(designationPath), path));
  expect(modules.length).toBe(Object.keys(repositoryDesignation.providers).length);
  const scripts = ["🔨️modules/🎭️actor/📦️packages/🦀️rust", "🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript", "📦️packages/🟦️typescript"].map(path => join(workspace, "🧰️framework", path));
  const cache = createScriptInputCache();
  for (const directory of scripts) {
    const inputs = await inputsOf(directory, cache);
    for (const file of [designationPath, schemaPath, loaderPath, ...modules]) expect(inputs, `${relative(workspace, directory)} -> ${relative(workspace, file)}`).toContain(relative(workspace, file));
  }
  for (const module of modules) {
    const bundle = await build({ absWorkingDir: workspace, entryPoints: [module], bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    const closure = Object.keys(bundle.metafile!.inputs).map(path => relative(workspace, resolve(workspace, path.replace(/ with \{.*\}$/u, ""))));
    const inputs = await inputsOf(scripts[0]!, cache);
    for (const file of closure) expect(inputs, `${relative(workspace, module)} closure member ${file}`).toContain(file);
  }
}, 120_000);
