import assert from "node:assert/strict";
import { readFileSync, mkdtempSync, statSync, utimesSync, readdirSync, mkdirSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";

/** 🔏️ Validates deterministic digest publication, failure preservation and explicit Nx ownership. */
export async function testGeneratorInputReceipt(workspace: string, generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔏️receipt/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(schema), stable = require("fast-json-stable-stringify");
  const receipt = { version: 1, kind: fixture.kind, digest: fixture.initialDigest };
  assert.equal(validate(receipt), true, JSON.stringify(validate.errors));
  const { publishGeneratorInputReceipt } = await import("../../🟦️.ts");
  const root = mkdtempSync(join(generated, "generator-input-receipt-"));
  const first = publishGeneratorInputReceipt(root, fixture.output, fixture.kind, fixture.initialDigest);
  assert.equal(first.changed, true); assert.equal(first.path, join(root, fixture.output));
  assert.equal(readFileSync(first.path, "utf8"), stable(receipt) + "\n");
  utimesSync(first.path, fixture.mtime, fixture.mtime);
  assert.equal(publishGeneratorInputReceipt(root, fixture.output, fixture.kind, fixture.initialDigest).changed, false);
  assert.equal(statSync(first.path).mtimeMs, fixture.mtime * 1000);
  for (const digest of fixture.invalidDigests) {
    assert.equal(validate({ ...receipt, digest }), false);
    assert.throws(() => publishGeneratorInputReceipt(root, fixture.output, fixture.kind, digest), /digest/i);
    assert.equal(readFileSync(first.path, "utf8"), stable(receipt) + "\n");
  }
  assert.throws(() => publishGeneratorInputReceipt(root, "../outside.json", fixture.kind, fixture.initialDigest), /workspace/i);
  assert.equal(publishGeneratorInputReceipt(root, fixture.output, fixture.kind, fixture.changedDigest).changed, true);
  assert.equal(readFileSync(first.path, "utf8"), stable({ ...receipt, digest: fixture.changedDigest }) + "\n");
  assert.deepEqual(readdirSync(dirname(first.path)), ["🔣️.json"]);
  mkdirSync(join(root, "foreign")); writeFileSync(join(root, "foreign/source.json"), "foreign");
  symlinkSync(join(root, "foreign"), join(root, "alias"), process.platform === "win32" ? "junction" : "dir");
  assert.throws(() => publishGeneratorInputReceipt(root, "alias/source.json", fixture.kind, fixture.initialDigest), /owner/i);
  assert.equal(readFileSync(join(root, "foreign/source.json"), "utf8"), "foreign");
  const caching = resolve(import.meta.dir, "../../.."), policy = JSON.parse(readFileSync(join(caching, "🔣️policy.json"), "utf8"));
  const contract = policy.generatorInputs[fixture.kind], project = JSON.parse(readFileSync(join(caching, "📋️project.json"), "utf8"));
  assert.equal(contract.target, "repo:generator-inputs");
  assert.equal(project.targets["generator-inputs"].cache, false);
  assert.deepEqual(project.targets["generator-inputs"].outputs, [`{workspaceRoot}/${contract.output}`]);
  const { isDiscoverySkipDirectory } = await import(resolve(caching, "../🔍️discovery/🟦️.ts"));
  assert.equal(isDiscoverySkipDirectory(contract.output.split("/")[0]), true, "The digest must not become its own catalog input");
  const plugin = (await import(resolve(caching, "../🟨️.mjs"))).default;
  const registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
  const nodes = await plugin.createNodesV2[1]([`${registry}/📋️project.json`], {}, { workspaceRoot: workspace });
  const owner: any = nodes.flatMap(([, result]: any) => Object.values(result.projects)).find((project: any) => project.name === "@semio-tech/plugin-registry");
  assert.ok(owner); const target = owner.targets.generate;
  assert.ok(target.dependsOn.includes(contract.target));
  assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === contract.output));
  assert.ok(!target.inputs.some((input: any) => input.runtime?.includes("generator-inputs")));
  console.log("[DEBUG] Digest publication matches stable JSON/Ajv, preserves unchanged bytes and mtime, rejects invalid writes and has an uncached Nx producer PASS");
}
