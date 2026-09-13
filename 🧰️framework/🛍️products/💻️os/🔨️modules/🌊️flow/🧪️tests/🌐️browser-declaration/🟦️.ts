/** 🧪️ Verifies the Flow browser declaration against its schema, runtime and package projection. */
import assert from "node:assert/strict";
import { readFileSync, realpathSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { flowBrowserDeclaration } from "../../🕸️wasm/🌐️browser/📝️declaration/📤️projection/🟦️.ts";
import { flowWasmContract } from "../../🕸️wasm/🧪️tests/🧬️schema-oracle/🛂️admission/🟦️.ts";

const testDirectory = fileURLToPath(new URL(".", import.meta.url));
const browserTypesFixturePath = join(testDirectory, "../../🕸️wasm/🧫️fixtures/📝️browser-types/🔣️.json");
const declarationPath = join(testDirectory, "../../🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts");

export async function testFlowBrowserDeclaration(packageRoot: string): Promise<void> {
  const ts = await import("typescript");
  const { FlowSession } = await import("../../🕸️wasm/🌐️browser/🏃️runtime/🟨️.js");
  const fixture = JSON.parse(readFileSync(browserTypesFixturePath, "utf8"));
  const validate = flowWasmContract("FlowBrowserTypesV1");
  assert.equal(validate(fixture), true);
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8"));
  assert.equal(manifest.name, fixture.package.name);
  assert.deepEqual(manifest.files.slice().sort(), fixture.package.files.slice().sort());
  assert.deepEqual(manifest.exports, fixture.package.exports);
  for (const name of fixture.package.files) assert.ok(ts.sys.fileExists(join(packageRoot, name)), name);
  for (const [subpath, entry] of Object.entries(fixture.package.exports)) {
    if (typeof entry === "string") continue;
    const binding = entry as { types: string; import: string };
    const name = fixture.package.name + (subpath === "." ? "" : subpath.slice(1));
    const resolved = ts.resolveModuleName(name, join(testDirectory, "consumer.ts"), { moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext }, ts.sys).resolvedModule;
    assert.ok(resolved, name);
    assert.equal(realpathSync(resolved.resolvedFileName), realpathSync(join(packageRoot, binding.types)));
  }
  const text = flowBrowserDeclaration();
  const declarationTextPath = declarationPath;
  const program = ts.createProgram([declarationTextPath], { noLib: true, noResolve: true });
  assert.equal(program.getSyntacticDiagnostics().length, 0);
  const parsed = program.getSourceFile(declarationTextPath);
  assert.ok(parsed);
  const session = parsed.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "FlowSession");
  assert.ok(session && ts.isClassDeclaration(session));
  const methods = session.members.filter(ts.isMethodDeclaration);
  const names = [...new Set(methods.map((method) => method.name.getText(parsed)).filter((name) => !name.startsWith("[")))];
  const runtime = Object.getOwnPropertyNames(FlowSession.prototype).filter((name) => name !== "constructor").sort();
  assert.deepEqual(names.slice().sort(), runtime);
  assert.equal(names.filter((name) => ![...fixture.canvasMethods, "close", "free"].includes(name)).length, fixture.operationMethods);
  for (const sample of fixture.samples) {
    const method = methods.find((value) => value.name.getText(parsed) === sample.name);
    assert.ok(method);
    assert.deepEqual(method.parameters.map((parameter) => parameter.getText(parsed)), sample.parameters);
    assert.equal(method.type?.getText(parsed), fixture.result);
  }
  for (const name of fixture.excluded) assert.equal(names.includes(name), false);
  for (const mutate of [(value: typeof fixture) => { value.operationMethods = 111; }, (value: typeof fixture) => { value.result = "void"; }, (value: typeof fixture) => { value.extra = true; }, (value: typeof fixture) => { delete value.package.exports["."]; }, (value: typeof fixture) => { value.package.files.push(value.package.files[0]); }]) { const bad = structuredClone(fixture); mutate(bad); assert.equal(validate(bad), false); }
  assert.equal(readFileSync(declarationPath, "utf8"), text);
  console.log(`[DEBUG] Flow browser declarations: ${fixture.operationMethods} schema methods, runtime prototype and TypeScript parser parity; 3 package exports and 2 TypeScript resolutions; 5 hostile fixtures rejected`);
}
