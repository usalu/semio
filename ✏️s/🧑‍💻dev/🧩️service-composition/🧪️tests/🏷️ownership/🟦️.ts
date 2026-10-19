import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { runInNewContext } from "node:vm";
import ts from "typescript";
import * as toml from "@iarna/toml";
import { build } from "esbuild";

/** 🏷 Checks current defining owners and independently bundles the neutral installed-inventory entries. */
export async function sDevCompositionOwnershipV1(repoRoot: string): Promise<void> {
  const neutral = "✏️s/🧑‍💻dev", specific = `${neutral}/🎭️variants/🌍️gis`;
  const schemaPath = `${neutral}/🚀️entry/🧬️schema`, discovery = await import("@semio-tech/repo-lib/inventory");
  const inventory = discovery.inventorySchemaScopes(repoRoot), scope = inventory.catalog.scopes["s.dev.entry"];
  assert.equal(scope?.path, schemaPath);
  assert(scope);
  assert(scope.exports.SDevBrowserInventoryV1);
  assert.deepEqual(inventory.diagnostics.filter(row => row.path === schemaPath || row.path.startsWith(`${schemaPath}/`)), []);
  await sDevCompositionSourceOwnershipV1(repoRoot);
}

/** 🧩️ Checks the finite entry owners, bundles and manifest/lock projections without repeating global schema discovery. */
export async function sDevCompositionSourceOwnershipV1(repoRoot:string):Promise<void>{
  const neutral = "✏️s/🧑‍💻dev", specific = `${neutral}/🎭️variants/🌍️gis`;
  const entries = [`${neutral}/🚀️entry/🟦️.ts`, `${neutral}/🧩️service-composition/👷️worker/🟦️.ts`];
  for (const path of entries) {
    const source = ts.createSourceFile(path, readFileSync(join(repoRoot, path), "utf8"), ts.ScriptTarget.Latest, true);
    const imports = source.statements.filter(ts.isImportDeclaration);
    assert(imports.every(node => node.importClause?.isTypeOnly || ts.isStringLiteral(node.moduleSpecifier) && !/(?:^|[^A-Za-z])(?:plugins|variants|gis|puzzle)(?:$|[^A-Za-z])/u.test(node.moduleSpecifier.text)));
    const result = await build({ absWorkingDir: repoRoot, entryPoints: [path], bundle: true, write: false, metafile: true, platform: "browser", format: "cjs" });
    const metafile=result.metafile,output=result.outputFiles[0];
    assert(metafile&&output);
    assert(Object.keys(metafile.inputs).every(input => !/🔌️plugins|🎭️variants/u.test(input)));
    assert(!/GIS_INFERENCE|PUZZLE_BOARD|createGis/u.test(output.text));
    const sandbox:{module:{exports:unknown};TextEncoder:typeof TextEncoder;URL:typeof URL} = { module: { exports: {} },TextEncoder,URL };
    runInNewContext(output.text, sandbox);
    const observeMount=async(name:string,args:readonly unknown[]):Promise<()=>void>=>{const executable=sandbox.module.exports;assert(executable&&typeof executable==="object");const execute:unknown=Reflect.get(executable,name);assert(typeof execute==="function");const mount:unknown=await Reflect.apply(execute,undefined,args);assert(mount&&typeof mount==="object");const dispose:unknown=Reflect.get(mount,"dispose");assert(typeof dispose==="function");return()=>{Reflect.apply(dispose,mount,[]);};};
    const controller = new AbortController();
    let calls = 0, retired = 0;
    const operation = { signal: controller.signal, progress: () => {} };
    if (path === entries[0]) {
      const empty = { catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] }, plugins: [], documentServices: [], surfaceSessionFactories: [] };
      const dispose = await observeMount("bootSDevV1",[empty, { mount: async (received: unknown) => { assert.equal(received, empty); calls++; return { dispose: () => { retired++; } }; } }, operation]);
      controller.abort();
      dispose();
      assert.equal(calls, 1);
      assert.equal(retired, 1);
    } else {
      const dispose = await observeMount("installSDevWorkersV1",[[], { install: async () => { calls++; return { dispose: () => { retired++; } }; } }, operation]);
      controller.abort();
      dispose();
      assert.equal(calls, 0);
      assert.equal(retired, 0);
    }
  }
  const isRecord=(value:unknown):value is Record<string,unknown>=>!!value&&typeof value==="object"&&!Array.isArray(value),record=(value:unknown):Record<string,unknown>=>{assert(isRecord(value));return value;};
  const manifest = (path: string): Record<string,unknown> => record(toml.parse(readFileSync(join(repoRoot, path), "utf8")));
  const native = manifest(`${neutral}/💡️services/📦️packages/🦀️rust/Cargo.toml`);
  const nativeOwner = manifest(`${neutral}/💡️services/Cargo.toml`);
  assert.equal(record(native.package).workspace, "../..");
  assert.equal(record(native.package).name, "semio-s-dev-services");
  for (const providers of [record(native.dependencies), record(record(nativeOwner.workspace).dependencies)]) assert(Object.keys(providers).every(key => !/gis|puzzle|hub/u.test(key)));
  assert.equal(native.features, undefined);
  assert.equal(record(record(native.dependencies)["semio-framework-os-mcp"]).workspace, true);
  assert.equal(record(record(native.dependencies)["semio-framework-os-renderer-wgpu"]).workspace, true);
  const child = manifest(`${specific}/💡️services/📦️packages/🦀️rust/Cargo.toml`);
  assert.equal(record(child.package).name, "semio-s-dev-gis-services");
  assert.equal(record(record(child.dependencies)["semio-s-artifact-gis-gismap"]).workspace, true);
  assert.equal(record(record(child.dependencies)["semio-s-dev-services"]).workspace, true);
  assert.equal(record(child.dependencies)["semio-framework-os-renderer-wgpu"], undefined);
  assert.equal(record(child.dependencies)["semio-framework-os-mcp"], undefined);
  assert.equal(child.features, undefined);
  assert.equal(record(record(child.dependencies)["semio-s-artifact-gis-gismap-mcp"]).workspace, true);
  assert(readFileSync(join(repoRoot, `${specific}/💡️services/⌨️entrypoint/🦀️.rs`), "utf8").includes("semio_s_dev_services::run_native_v1(semio_s_dev_gis_services::service_contributions_v1())"));
  assert(readFileSync(join(repoRoot, `${specific}/💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs`), "utf8").includes("semio_s_dev_services::run_mcp_v1(vec![semio_s_artifact_gis_gismap_mcp::gis_map_mcp_protocol_v1()"));
  const foreign = (lock:unknown):string[]=>{const packages=record(lock).package;assert(Array.isArray(packages));const result:string[]=[];for(const value of packages){const row=record(value);if(row.source===undefined)continue;assert(typeof row.name==="string"&&typeof row.version==="string"&&typeof row.source==="string"&&(row.checksum===undefined||typeof row.checksum==="string"));result.push(JSON.stringify([row.name,row.version,row.source,row.checksum]));}return result.sort();};
  const lockBodies = [`${neutral}/💡️services/Cargo.lock`, `${specific}/💡️services/Cargo.lock`].map(path => readFileSync(join(repoRoot, path), "utf8"));
  const locks = lockBodies.map(body => record(toml.parse(body)));
  for (const [index,lock] of locks.entries()){const body=lockBodies[index];assert(body!==undefined);assert.deepEqual(foreign(lock), foreign(Bun.TOML.parse(body)));}
  const parentLock=locks[0],childLock=locks[1];assert(parentLock&&childLock);
  const childForeign = new Set(foreign(childLock));
  assert(foreign(parentLock).every(identity => childForeign.has(identity)));
  const parentPackages=parentLock.package;assert(Array.isArray(parentPackages));
  assert(parentPackages.every(value=>{const row=record(value);assert(typeof row.name==="string");return !/^semio-s-(artifact|plugin)/u.test(row.name);}));
  const contribution = record(JSON.parse(readFileSync(join(repoRoot, `${specific}/🧬️schema/🔣️.json`), "utf8")));
  assert(typeof contribution.viteConfig==="string"&&typeof contribution.browserEntry==="string");
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
  const receivingDirectory = join(repoRoot, `${specific}/🧩️service-composition/🧪️tests`);
  let receivingFixtures = 0;
  for (const entry of readdirSync(receivingDirectory, { withFileTypes: true }).filter(row => row.isDirectory())) {
    const receivingTests = ts.createSourceFile(`gis-worker-tests-${entry.name}.ts`, readFileSync(join(receivingDirectory, entry.name, "🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const declarations: Array<{ name: string; value: string; position: number }> = [];
    const visit = (node: ts.Node): void => {
      if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && ts.isStringLiteral(node.initializer)) declarations.push({ name: node.name.text, value: node.initializer.text, position: node.pos });
      if (ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "URL" && node.arguments?.[1]?.getText(receivingTests) === "source.url") {
        const argument = node.arguments[0];
        assert(argument);
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
  }
  assert(receivingFixtures > 8);
  const receiverConfiguration = readFileSync(join(repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🎚️config/🟦️.ts"), "utf8");
  assert(receiverConfiguration.includes("../👷️worker/🟦️.ts"));
  assert(!existsSync(join(repoRoot, `${neutral}/🧩️service-composition/🧪️tests/🟦️.ts`)));
  console.log(`s-dev-composition-source: neutral entries=${entries.length}, isolated neutral runtimes=2, independent AST/esbuild/Cargo owners, GIS/Puzzle deployment source passed`);
}
