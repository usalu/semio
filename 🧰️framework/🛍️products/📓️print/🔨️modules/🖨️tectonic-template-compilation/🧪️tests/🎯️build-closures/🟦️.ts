/** 🎯️ Law: the command closure of every Print build command is the code it really reads. The command closure is what the build graph hashes for a task, so a closure that drags in tests, other commands or the repository tool makes every document build depend on edits that cannot change it. The scenario is language-agnostic data in `./🔣️.json`; the product closure is checked against an independent esbuild import oracle. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { build } from "esbuild";
import fixture from "./🔣️.json" with { type: "json" };
import fixtureSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import policy from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏗️build/🔣️policy/🔣️json/🟦️.ts";

const root = process.env.SEMIO_TEST_WORKSPACE_ROOT ?? (() => { let path = dirname(fileURLToPath(import.meta.url)); while (!existsSync(join(path, "bunfig.toml")) || !existsSync(join(path, "bun.lock"))) path = dirname(path); return path; })();
const inference = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference");
const prefix = "{workspaceRoot}/", presentation = new Set<string>(policy.presentationResources);

const closureOf = async (entry: string): Promise<readonly string[]> => {
  const owner = await import(join(inference, "🟨️.mjs")), scripts = await import(join(inference, "🧭️script-inputs/🟨️.mjs"));
  await owner.libraryBootstrap;
  const closure: string[] = await scripts.relativeScriptInputs([resolve(root, entry)], root, owner.createScriptInputCache({ workspaceRoot: root }), false);
  return closure.map(name => name.slice(prefix.length));
};

test("the fixture is closed under its schema and names existing files", () => {
  expect(new Ajv({ strict: true }).validate(fixtureSchema, fixture)).toBe(true);
  for (const file of [...fixture.entries, fixture.requiredProvider.module]) expect(existsSync(join(root, file)), file).toBe(true);
});

test("no Print build command closure holds a forbidden source or exceeds its declared size", async () => {
  for (const entry of fixture.entries) {
    const closure = await closureOf(entry);
    expect(closure, entry).toContain(entry);
    expect(closure.length, `${entry} size`).toBeLessThanOrEqual(fixture.maximumFiles);
    for (const fragment of fixture.forbidden) expect(closure.filter(file => file.includes(fragment)), `${entry} must not hold ${fragment}`).toEqual([]);
  }
});

test("the closure covers the one script provider port the documents are published through", async () => {
  expect(await closureOf(fixture.requiredProvider.entry)).toContain(fixture.requiredProvider.module);
});

test("every file the independent esbuild import oracle reaches without package imports is in the product closure", async () => {
  for (const entry of fixture.entries) {
    const closure = new Set(await closureOf(entry));
    const bundle = await build({ absWorkingDir: root, entryPoints: [join(root, entry)], bundle: true, write: false, platform: "node", format: "esm", packages: "external", external: ["bun"], metafile: true, logLevel: "silent" });
    const oracle = Object.keys(bundle.metafile!.inputs).map(path => relative(root, resolve(root, path.replace(/ with \{.*\}$/u, "")))).filter(path => !path.startsWith("..") && !path.includes("node_modules") && !presentation.has(path));
    expect(oracle.length).toBeGreaterThan(1);
    for (const file of oracle) expect(closure.has(file), `${entry} lacks ${file}`).toBe(true);
  }
}, 120_000);
