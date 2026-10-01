import { defaultPlaygroundVariant } from "../🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import TOML from "@iarna/toml";
import { build } from "esbuild";
import { resolve } from "node:path";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import { parsePlaygroundBlock, type PlaygroundEntry } from "../../🔎️discovery/🟦️.ts";
import { emitPlaygroundsTypeScript, renderCatalogFiles, validatePlaygroundSessions } from "../../../📽️projection/🟦️.ts";
import { renderPlaygroundSessionTypeScript } from "../../🧭️session/🟦️.ts";
import type { RegistryCatalogInputView } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🏃️Executes one emitted catalog in native Bun and independently compiled Node. */
async function executeCatalogV1(source: string, directory: string, assertion: string): Promise<void> {
  const resolveDir = resolve(import.meta.dirname, "../../../🤖️generated", directory);
  const program = `${source}\n${assertion}\nprocess.stdout.write("Catalog executed");`;
  const executable = program.replace('"../../📦️deployment/🟦️.ts"', JSON.stringify(resolve(resolveDir, "../../📦️deployment/🟦️.ts"))).replace('"../../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts"', JSON.stringify(resolve(resolveDir, "../../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts")));
  const bun = Bun.spawnSync(["bun", "run", "-"], { stdin: Buffer.from(executable), stdout: "pipe", stderr: "pipe" });
  assert.equal(bun.exitCode, 0, Buffer.from(bun.stderr).toString());
  assert.equal(Buffer.from(bun.stdout).toString(), "Catalog executed");
  const independent = await build({ stdin: { contents: program, resolveDir, loader: "ts" }, bundle: true, platform: "node", format: "esm", write: false });
  const node = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(independent.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
  assert.equal(node.exitCode, 0, Buffer.from(node.stderr).toString());
  assert.equal(Buffer.from(node.stdout).toString(), "Catalog executed");
}

/** ⭐️Checks explicit default policy and complete empty-catalog rendering without concrete owners. */
export async function provePlaygroundDefaultContractV1(): Promise<number> {
  const ajv = new Ajv({ strict: true }).addSchema(schema);
  assert.equal(ajv.getSchema(`${schema.$id}#/$defs/CasesV1`)!(corpus), true);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/SelectionV1`)!;
  const failures: string[] = [];
  for (const row of corpus.cases) {
    const accepted = validate(row.input), chosen = row.input.find(input => input.catalogDefault === true);
    assert.deepEqual({ accepted, variant: accepted ? chosen?.variant ?? null : null }, row.expected, `${row.id}: AJV oracle`);
    let actual: typeof row.expected;
    try { actual = { accepted: true, variant: defaultPlaygroundVariant(row.input) ?? null }; }
    catch { actual = { accepted: false, variant: null }; }
    if (JSON.stringify(actual) !== JSON.stringify(row.expected)) failures.push(row.id);
    if (accepted) {
      const entries = row.input.map((owner, index) => parsePlaygroundBlock(`variant = ${JSON.stringify(owner.variant)}\nports = { react = ${6001 + index * 2}, wgpu = ${6002 + index * 2} }\n${owner.catalogDefault === undefined ? "" : `catalog-default = ${owner.catalogDefault}`}`, `owner-${index}`, `future/owner-${index}`)!);
      const assertion = `if(DEFAULT_PLAYGROUND_VARIANT!==${row.expected.variant === null ? "undefined" : JSON.stringify(row.expected.variant)}||JSON.stringify(PLAYGROUND_BUILD_TARGETS.map(row=>row.variant))!==${JSON.stringify(JSON.stringify(row.input.map(owner => owner.variant)))})throw Error('Owner selection drift');`;
      await executeCatalogV1(emitPlaygroundsTypeScript(entries), "🎮️playgrounds", assertion);
    }
  }
  for (const row of corpus.declarations) {
    const block = `variant = "future"\nports = { react = 6001, wgpu = 6002 }\n${row.block}`;
    const observe = (read: () => unknown): boolean | "refused" => {
      try { const value = read(); if (value !== undefined && typeof value !== "boolean") throw Error("Invalid default policy"); return value === true; }
      catch { return "refused"; }
    };
    assert.equal(observe(() => TOML.parse(block)["catalog-default"]), row.expected, `${row.id}: Iarna oracle`);
    if (observe(() => (parsePlaygroundBlock(block, "future", "future/owner") as PlaygroundEntry & { catalogDefault?: boolean }).catalogDefault) !== row.expected) failures.push(row.id);
  }
  const empty: RegistryCatalogInputView = { entries: () => [], kind: path => path === "" ? "directory" : null, readText: () => { throw Error("No source files"); }, readBytes: () => { throw Error("No source files"); } };
  try {
    const rendered = renderCatalogFiles("/removed-specific-owners", empty);
    assert.deepEqual(rendered.entries, []);
    assert.deepEqual(rendered.playgrounds, []);
    assert.deepEqual(JSON.parse(rendered.files["🔌️plugins.json"]!), []);
    assert.deepEqual(validatePlaygroundSessions("/removed-specific-owners", rendered), []);
    const emitted = [
      { source: rendered.files["🧩️plugins/🟦️.ts"]!, directory: "🧩️plugins", assertion: "if(COMPONENT_MODULE_DIRECTORIES.length||PLUGIN_BUILD_TARGETS.length||EXTENSION_TARGETS.length)throw Error('Empty inventory drift');" },
      { source: rendered.files["🎮️playgrounds/🟦️.ts"]!, directory: "🎮️playgrounds", assertion: "if(PLAYGROUND_BUILD_TARGETS.length||DEFAULT_PLAYGROUND_VARIANT!==undefined)throw Error('Empty default drift');" },
      { source: renderPlaygroundSessionTypeScript(undefined, rendered), directory: "🎮️playgrounds", assertion: "if(PLAYGROUND_SESSION!==undefined)throw Error('Empty session drift');" },
    ];
    for (const row of emitted) {
      await executeCatalogV1(row.source, row.directory, row.assertion);
    }
  } catch (error) { failures.push(`complete-empty-catalog: ${String(error)}`); }
  assert.deepEqual(failures, [], "Default policy differs from the portable corpus");
  return corpus.cases.length + corpus.declarations.length + 1;
}
