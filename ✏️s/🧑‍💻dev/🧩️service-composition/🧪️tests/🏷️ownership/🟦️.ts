import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { resolve, join } from "node:path";
import { runInNewContext } from "node:vm";
import ts from "typescript";
import * as toml from "@iarna/toml";
import { build } from "esbuild";

/** 🏷 Checks current defining owners and independently bundles the neutral installed-inventory entries. */
export async function sDevCompositionOwnershipV1(repoRoot: string): Promise<void> {
  const neutral = "✏️s/🧑‍💻dev", specific = `${neutral}/🎭️variants/🌍️gis`;
  const schemaPath = `${neutral}/🚀️entry/🧬️schema`, discovery = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
  const inventory = discovery.inventorySchemaScopes(repoRoot), scope = inventory.catalog.scopes["s.dev.entry"];
  assert.equal(scope?.path, schemaPath);
  assert(scope.exports.SDevBrowserInventoryV1);
  assert.deepEqual(inventory.diagnostics.filter(row => row.path === schemaPath || row.path.startsWith(`${schemaPath}/`)), []);
  console.log("[DEBUG] actual s.dev.entry schema discovery admitted SDevBrowserInventoryV1 with zero scope diagnostics");
  const entries = [`${neutral}/🚀️entry/🟦️.ts`, `${neutral}/🧩️service-composition/👷️worker/🟦️.ts`];
  for (const path of entries) {
    const source = ts.createSourceFile(path, readFileSync(join(repoRoot, path), "utf8"), ts.ScriptTarget.Latest, true);
    const imports = source.statements.filter(ts.isImportDeclaration);
    assert(imports.every(node => node.importClause?.isTypeOnly || !/plugins|variants|gis|puzzle/u.test((node.moduleSpecifier as ts.StringLiteral).text)));
    const result = await build({ absWorkingDir: repoRoot, entryPoints: [path], bundle: true, write: false, metafile: true, platform: "browser", format: "cjs" });
    assert(Object.keys(result.metafile!.inputs).every(input => !/🔌️plugins|🎭️variants/u.test(input)));
    assert(!/GIS_INFERENCE|PUZZLE_BOARD|createGis/u.test(result.outputFiles[0].text));
    const sandbox = { module: { exports: {} as any } };
    runInNewContext(result.outputFiles[0].text, sandbox);
    const executable = sandbox.module.exports;
    const controller = new AbortController();
    let calls = 0, retired = 0;
    const operation = { signal: controller.signal, progress: () => {} };
    if (path === entries[0]) {
      const empty = { catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] }, plugins: [], documentServices: [], surfaceSessionFactories: [] };
      const mount = await executable.bootSDevV1(empty, { mount: async (received: unknown) => { assert.equal(received, empty); calls++; return { dispose: () => { retired++; } }; } }, operation);
      controller.abort();
      mount.dispose();
      assert.equal(calls, 1);
      assert.equal(retired, 1);
    } else {
      const mount = await executable.installSDevWorkersV1([], { install: async () => { calls++; return { dispose: () => { retired++; } }; } }, operation);
      controller.abort();
      mount.dispose();
      assert.equal(calls, 0);
      assert.equal(retired, 0);
    }
    console.log(`[DEBUG] neutral isolated bundle ${path}: host=${calls} retired=${retired}`);
  }
  const manifest = (path: string): any => toml.parse(readFileSync(join(repoRoot, path), "utf8"));
  const native = manifest(`${neutral}/💡️services/📦️packages/🦀️rust/Cargo.toml`);
  const nativeOwner = manifest(`${neutral}/💡️services/Cargo.toml`);
  assert.equal(native.package.workspace, "../..");
  assert.equal(native.package.name, "semio-s-dev-services");
  for (const providers of [native.dependencies, nativeOwner.workspace.dependencies]) assert(Object.keys(providers).every(key => !/gis|puzzle|hub/u.test(key)));
  assert.deepEqual(native.features["mcp-service"], ["dep:semio-framework-os-mcp"]);
  const child = manifest(`${specific}/💡️services/📦️packages/🦀️rust/Cargo.toml`);
  assert.equal(child.package.name, "semio-s-dev-gis-services");
  assert.equal(child.dependencies["semio-s-artifact-gis-gismap"].workspace, true);
  assert.equal(child.dependencies["semio-s-dev-services"].workspace, true);
  assert.equal(child.dependencies["semio-framework-os-renderer-wgpu"], undefined);
  assert.equal(child.dependencies["semio-framework-os-mcp"], undefined);
  assert.deepEqual(child.features["native-renderer"], ["semio-s-dev-services/native-renderer"]);
  assert(child.features["mcp-service"].includes("semio-s-dev-services/mcp-service"));
  assert(child.features["mcp-service"].includes("semio-s-artifact-gis-gismap/mcp-service"));
  assert(readFileSync(join(repoRoot, `${specific}/💡️services/⌨️entrypoint/🦀️.rs`), "utf8").includes("semio_s_dev_services::run_native_v1(semio_s_dev_gis_services::service_contributions_v1())"));
  assert(readFileSync(join(repoRoot, `${specific}/💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs`), "utf8").includes("semio_s_dev_services::run_mcp_v1(vec![semio_s_artifact_gis_gismap"));
  const foreign = (lock: any): string[] => lock.package.filter((row: any) => row.source).map((row: any) => JSON.stringify([row.name, row.version, row.source, row.checksum])).sort();
  const lockBodies = [`${neutral}/💡️services/Cargo.lock`, `${specific}/💡️services/Cargo.lock`].map(path => readFileSync(join(repoRoot, path), "utf8"));
  const locks = lockBodies.map(body => toml.parse(body) as any);
  for (let index = 0; index < locks.length; index++) assert.deepEqual(foreign(locks[index]), foreign(Bun.TOML.parse(lockBodies[index])));
  const childForeign = new Set(foreign(locks[1]));
  assert(foreign(locks[0]).every(identity => childForeign.has(identity)));
  assert(locks[0].package.every((row: any) => !/^semio-s-(artifact|plugin)/u.test(row.name)));
  const contribution = JSON.parse(readFileSync(join(repoRoot, `${specific}/🧬️schema/🔣️.json`), "utf8"));
  assert.equal(contribution.ownerRoot, specific);
  assert.equal(contribution.browserEntry, `${specific}/🚀️entry/🟦️.ts`);
  assert(existsSync(resolve(repoRoot, contribution.viteConfig)));
  const browser = readFileSync(join(repoRoot, contribution.browserEntry), "utf8");
  assert(browser.includes("GIS_INFERENCE_PRESENTATION_V1"));
  assert(browser.includes("PUZZLE_BOARD_SESSION_FACTORIES"));
  assert(browser.includes("mounted.dispose()"));
  const worker = readFileSync(join(repoRoot, `${specific}/🧩️service-composition/👷️worker/🟦️.ts`), "utf8");
  assert(worker.includes("createGisMapInferenceWorkerV1"));
  assert(worker.includes('import.meta.hot.dispose(retire)'));
  const receivingTests = ts.createSourceFile("gis-worker-tests.ts", readFileSync(join(repoRoot, `${specific}/🧩️service-composition/🧪️tests/🟦️.ts`), "utf8"), ts.ScriptTarget.Latest, true);
  const declarations: Array<{ name: string; value: string; position: number }> = [];
  let receivingFixtures = 0;
  const visit = (node: ts.Node): void => {
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && ts.isStringLiteral(node.initializer)) declarations.push({ name: node.name.text, value: node.initializer.text, position: node.pos });
    if (ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "URL" && node.arguments?.[1]?.getText(receivingTests) === "source.url") {
      const argument = node.arguments[0];
      let path: string | undefined;
      if (ts.isStringLiteral(argument)) path = argument.text;
      else if (ts.isBinaryExpression(argument) && ts.isIdentifier(argument.left) && ts.isStringLiteral(argument.right)) {
        const name = argument.left.text;
        const declaration = declarations.filter(row => row.name === name && row.position < node.pos).at(-1);
        if (declaration) path = declaration.value + argument.right.text;
      }
      if (path) { assert(existsSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os", path)), path); receivingFixtures++; }
    }
    ts.forEachChild(node, visit);
  };
  visit(receivingTests);
  assert(receivingFixtures > 40);
  console.log(`[DEBUG] GIS worker defining source authority: ${receivingFixtures} actual fixture/source URL readers resolve`);
  assert(!existsSync(join(repoRoot, `${neutral}/🧩️service-composition/🧪️tests/🟦️.ts`)));
  console.log(`s-dev-composition-source: neutral entries=${entries.length}, isolated neutral runtimes=2, independent AST/esbuild/Cargo owners, GIS/Puzzle deployment source passed`);
}
