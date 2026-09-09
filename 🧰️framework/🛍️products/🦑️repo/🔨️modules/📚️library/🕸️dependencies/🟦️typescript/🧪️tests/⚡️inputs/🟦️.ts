import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdtempSync, unlinkSync } from "node:fs";
import { join, relative } from "node:path";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";

/** 🧬️ Compares declared source closures with native bundlers and validates missing generated boundaries. */
export async function testTypeScriptSourceInputs(generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/⚡️inputs/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🔣️schema.json"), "utf8"));
  const validate = new (require("ajv"))().compile(schema);
  assert.equal(validate(fixture.contract), true, JSON.stringify(validate.errors));
  const api = await import("../../🟨️.mjs");
  const root = mkdtempSync(join(generated, "typescript-source-inputs-"));
  for (const [path, source] of Object.entries(fixture.files)) writeFileSync(join(root, path), source as string);
  const files = () => api.relativeSourceInputs(fixture.contract, root).files.map((path: string) => relative(root, path).replaceAll("\\", "/"));
  assert.deepEqual(files(), fixture.expected);
  const esbuild = await require("esbuild").build({ entryPoints: [join(root, "entry.ts")], absWorkingDir: root, bundle: true, write: false, format: "cjs", platform: "node", metafile: true, logLevel: "silent" });
  assert.deepEqual(Object.keys(esbuild.metafile.inputs).filter(path => !fixture.contract.generated.includes(path)).sort(), files());
  const bun = await Bun.build({ entrypoints: [join(root, "entry.ts")], format: "cjs", target: "node" });
  assert.equal(bun.success, true, bun.logs.map(String).join("\n"));
  for (const source of [esbuild.outputFiles[0].text, await bun.outputs[0].text()]) {
    const node = spawnSync("node", ["-e", `${source}\nconsole.log(JSON.stringify(module.exports.result))`], { encoding: "utf8", timeout: 10000 });
    assert.equal(node.status, 0, node.stderr); assert.equal(Number(node.stdout.trim().split("\n").at(-1)), fixture.result);
  }
  unlinkSync(join(root, "generated.ts")); assert.deepEqual(files(), fixture.expected);
  assert.throws(() => api.assertGeneratedSources(fixture.contract, root), /missing/i);
  writeFileSync(join(root, "generated.ts"), fixture.generatedImport);
  assert.throws(files, /generated.*import/i); assert.throws(() => api.assertGeneratedSources(fixture.contract, root), /generated.*import/i);
  writeFileSync(join(root, "generated.ts"), fixture.files["generated.ts"]);
  for (const row of fixture.rejected) {
    writeFileSync(join(root, "entry.ts"), row.source);
    if (row.reason) assert.throws(files, new RegExp(row.reason, "i")); else assert.ok(files().includes("leaf.ts"));
  }
  writeFileSync(join(root, "entry.ts"), fixture.files["entry.ts"]);
  writeFileSync(join(root, "branch.ts"), 'export { value } from "./new.ts";'); writeFileSync(join(root, "new.ts"), "export const value = 21;");
  assert.deepEqual(files(), ["branch.ts", "entry.ts", "inline.ts", "new.ts"]);
  for (const malformed of [{ ...fixture.contract, unknown: true }, { ...fixture.contract, entries: ["../entry.ts"] }, { ...fixture.contract, entries: [] }]) {
    assert.equal(validate(malformed), false); assert.throws(() => api.relativeSourceInputs(malformed, root));
  }
  console.log("[DEBUG] Source inputs match esbuild, Bun/Node runtime values, changed imports and absent generated boundaries PASS");
}
