import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdtempSync, unlinkSync, symlinkSync } from "node:fs";
import { join, relative } from "node:path";
import { createRequire } from "node:module";
import {withBuildTestInvocationV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

/** 🧬️ Compares declared source closures with native bundlers and validates missing generated boundaries. */
export async function testTypeScriptSourceInputs(generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/⚡️inputs/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🔣️schema.json"), "utf8"));
  const validate = new (require("ajv"))().compile(schema);
  assert.equal(validate(fixture.contract), true, JSON.stringify(validate.errors));
  const api = await import("../../🟨️.mjs");
  const root = mkdtempSync(join(generated, "typescript-source-inputs-"));
  for (const [path, source] of Object.entries(fixture.files)) writeFileSync(join(root, path), source as string);
  const files = () => api.relativeSourceInputs(fixture.contract, root).files.map((path: string) => relative(root, path).replaceAll("\\", "/"));
  assert.deepEqual(files(), fixture.expected);
  const observation = await import("../../../../🏗️build/👁️observation/🟨️.mjs");
  const asynchronous = await observation.captureObservedSourceInputsV1(() => api.relativeSourceInputsAsyncV1(fixture.contract, root), undefined, false);
  assert.deepEqual(asynchronous.value.files, api.relativeSourceInputs(fixture.contract, root).files);
  const analyses = asynchronous.observations.filter((row: any) => row.kind === "analysis" && row.fact.provider === fixture.observation.provider);
  assert.equal(analyses.length, fixture.expected.length + fixture.contract.generated.length);
  const validateImports = new (require("ajv"))().compile(schema.$defs.runtimeImports);
  for (const row of analyses) assert.equal(validateImports(api.sourceRuntimeImportsV1(row.path)), true, JSON.stringify(validateImports.errors));
  assert.deepEqual(api.sourceRuntimeImportsV1(join(root,"branch.ts")),require("typescript").preProcessFile(fixture.files["branch.ts"],true,true).importedFiles.map((row: any)=>row.fileName));
  writeFileSync(join(root, "leaf.ts"), fixture.observation.bodyChanged);
  assert.equal(await observation.observationsHold(asynchronous.observations), true);
  writeFileSync(join(root, "leaf.ts"), fixture.observation.importChanged);
  assert.equal(await observation.observationsHold(asynchronous.observations), false);
  writeFileSync(join(root, "leaf.ts"), fixture.files["leaf.ts"]);
  const cancelled = Error("source traversal cancelled");
  await assert.rejects(api.relativeSourceInputsAsyncV1(fixture.contract, root, async () => { throw cancelled; }), error => error === cancelled);
  const progress: number[]=[];
  for(let index=0;index<fixture.observation.controlFiles;index++)writeFileSync(join(root,`control${index}.ts`),index+1<fixture.observation.controlFiles?`export * from "./control${index+1}.ts";`:"export const result = 1;");
  const controlled={entries:["control0.ts"],generated:[],externalDependencies:[]};
  const controlledInputs=await api.relativeSourceInputsAsyncV1(controlled,root,async (index:number)=>{progress.push(index);});
  assert.equal(controlledInputs.files.length,fixture.observation.controlFiles);assert.deepEqual(progress,fixture.observation.controlProgress);
  const esbuild = await require("esbuild").build({ entryPoints: [join(root, "entry.ts")], absWorkingDir: root, bundle: true, write: false, format: "cjs", platform: "node", metafile: true, logLevel: "silent" });
  assert.deepEqual(Object.keys(esbuild.metafile.inputs).filter(path => !fixture.contract.generated.includes(path)).sort(), files());
  const bun = await Bun.build({ entrypoints: [join(root, "entry.ts")], format: "cjs", target: "node" });
  assert.equal(bun.success, true, bun.logs.map(String).join("\n"));
  const bunOutput = bun.outputs[0];
  assert.ok(bunOutput, "The successful Bun build must expose its authored output");
  for (const source of [esbuild.outputFiles[0].text, await bunOutput.text()]) {
    await withBuildTestInvocationV1(async original => {
      const output = await captureOwnedCommandV1("node", ["-e", `${source}\nconsole.log(JSON.stringify(module.exports.result))`], root, { control: scriptCompilerAdmissionControlV1(original, "build:declared-source:oracle", 10000) });
      assert.equal(Number(output.trim().split("\n").at(-1)), fixture.result);
    });
  }
  unlinkSync(join(root, "generated.ts")); assert.deepEqual(files(), fixture.expected);
  assert.equal(await observation.observationsHold(asynchronous.observations), false);
  assert.deepEqual((await api.relativeSourceInputsAsyncV1(fixture.contract, root)).files.map((path: string) => relative(root, path)), fixture.expected);
  assert.throws(() => api.assertGeneratedSources(fixture.contract, root), /missing/i);
  writeFileSync(join(root, "generated.ts"), fixture.generatedImport);
  assert.throws(files, /generated.*import/i); assert.throws(() => api.assertGeneratedSources(fixture.contract, root), /generated.*import/i);
  await assert.rejects(api.relativeSourceInputsAsyncV1(fixture.contract,root),/generated.*import/i);
  writeFileSync(join(root, "generated.ts"), fixture.files["generated.ts"]);
  const outside = join(generated, "outside.ts"); writeFileSync(outside, "export const value = 1;");
  symlinkSync(outside, join(root, "escaped.ts"));
  writeFileSync(join(root, "entry.ts"), 'export { value } from "./escaped.ts";');
  assert.throws(files, /link/); await assert.rejects(api.relativeSourceInputsAsyncV1(fixture.contract, root), /link/);
  for (const row of fixture.rejected) {
    writeFileSync(join(root, "entry.ts"), row.source);
    if (row.reason) {assert.throws(files, new RegExp(row.reason, "i"));await assert.rejects(api.relativeSourceInputsAsyncV1(fixture.contract,root),new RegExp(row.reason,"i"));} else assert.ok(files().includes("leaf.ts"));
  }
  writeFileSync(join(root, "entry.ts"), fixture.files["entry.ts"]);
  writeFileSync(join(root, "branch.ts"), 'export { value } from "./new.ts";'); writeFileSync(join(root, "new.ts"), "export const value = 21;");
  assert.deepEqual(files(), ["branch.ts", "entry.ts", "inline.ts", "new.ts"]);
  for (const malformed of [{ ...fixture.contract, unknown: true }, { ...fixture.contract, entries: ["../entry.ts"] }, { ...fixture.contract, entries: [] }]) {
    assert.equal(validate(malformed), false); assert.throws(() => api.relativeSourceInputs(malformed, root));
  }
}
