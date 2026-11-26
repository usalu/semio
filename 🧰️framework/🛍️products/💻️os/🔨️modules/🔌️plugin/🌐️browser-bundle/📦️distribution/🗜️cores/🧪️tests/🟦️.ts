import assert from "node:assert/strict";
import { readFile, writeFile, mkdtemp, mkdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { createServer } from "node:http";
import { once } from "node:events";
import { pathToFileURL } from "node:url";
import { gunzipSync, gzipSync } from "node:zlib";
import { tmpdir } from "node:os";
import { test } from "bun:test";
import { copyBrowserArtifacts } from "../../🟦️.ts";
import { compressBrowserCore, gzipCoreLoader } from "../🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };
import { evaluateGeneratedSourceV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";

test("the encoded delivery compiles to the same WebAssembly function", async () => {
  const root = await mkdtemp(join(process.env.SEMIO_TICKET_DIR ?? tmpdir(), "core-"));
  try {
    const core = Buffer.from(fixture.moduleHex, "hex");
    const binding = `async function fetchCompile (url) {
      if (url.protocol === "file:") {
        const _fs = await import('node:fs/promises');
        return WebAssembly.compile(await _fs.readFile(url));
      }
      return fetch(url).then(WebAssembly.compileStreaming);
    }
    export function load(base) { return fetchCompile(new URL('./${fixture.source}', base)); }`;
    const input = join(root, "staged"), outputRoot = join(root, "site");
    await mkdir(input);
    await writeFile(join(input, fixture.source), core);
    await writeFile(join(input, "answer.js"), binding);
    await writeFile(join(input, ".semio-artifact.json"), JSON.stringify({version: 1, owner: "fixture", files: ["answer.js", fixture.source]}));
    await copyBrowserArtifacts(outputRoot, [{root: input, owner: "fixture", destination: "modules"}], {gzipComponentCores: true});
    const output = join(outputRoot, "modules", "answer.js");
    const encoded = await readFile(join(outputRoot, "modules", fixture.encoded));
    assert.deepEqual(gunzipSync(encoded), core);
    const failed = join(root, "failed.bin");
    await assert.rejects(compressBrowserCore(join(root, "absent"), failed), {code: "ENOENT"});
    await assert.rejects(readFile(failed), {code: "ENOENT"});
    await assert.rejects(compressBrowserCore(join(input, fixture.source), failed, AbortSignal.abort()));
    await assert.rejects(readFile(failed), {code: "ENOENT"});
    await assert.rejects(compressBrowserCore(join(input, fixture.source), output), {code: "EEXIST"});
    await assert.rejects(readFile(join(outputRoot, "modules", fixture.source)), {code: "ENOENT"});
    const { load } = await import(pathToFileURL(output).href);
    const server = createServer((request, response) => {
      if (request.url === "/" + fixture.encoded) response.end(encoded);
      else response.writeHead(404).end();
    });
    server.listen(0, "127.0.0.1");
    try {
      await once(server, "listening");
      const address = server.address();
      assert.ok(address && typeof address === "object");
      const origin = new URL("http://127.0.0.1:" + address.port);
      for (const base of [origin, pathToFileURL(output)]) {
        const instance = await WebAssembly.instantiate(await load(base));
        assert.equal((instance.exports.ans as () => number)(), fixture.result);
      }
      await assert.rejects(load(new URL("/missing/", origin)), /404/);
    } finally {
      server.close();
      server.closeAllConnections();
      await once(server, "close");
    }
    assert.throws(() => gzipCoreLoader("export default 1", [fixture.source]));
  } finally { await rm(root, { recursive: true, force: true }); }
});

test("an activation factory requests encoded cores through the generated shard loader", async () => {
  const source = `export function instantiate(getCoreModule, imports, instantiateCore, options) {
    return getCoreModule('${fixture.source}');
  }`;
  assert.throws(() => gzipCoreLoader(source, [fixture.source, "answer.core2.wasm"]), /Missing factory request/);
  assert.throws(() => gzipCoreLoader(source, ["answer.core2.wasm"]), /Missing staged core/);
  const factory = gzipCoreLoader(source, [fixture.source]);
  const { instantiate } = await import("data:text/javascript;base64," + Buffer.from(factory).toString("base64"));
  const { shardWorkerActorLoadingSource } = await import("../../../🏗️materialization/🧵️shard-worker-source/🔌️actor-loading-and-replies/🟦️.ts");
  const worker = shardWorkerActorLoadingSource();
  const start = worker.indexOf("const coreModules = new ShardCoreModules(");
  const end = worker.indexOf("const coreModuleHolder", start);
  assert.ok(start >= 0 && end > start);
  const compile = evaluateGeneratedSourceV1(worker.slice(start, end), "coreModules.compile", { ShardCoreModules: class {
    constructor(readonly compile: (url: string) => Promise<WebAssembly.Module>) {}
  } });
  const core = Buffer.from(fixture.moduleHex, "hex");
  const requests: string[] = [];
  const server = createServer((request, response) => {
    requests.push(request.url!);
    if (request.url === "/" + fixture.encoded + "?v=test") response.end(gzipSync(core));
    else if (request.url === "/" + fixture.source) response.writeHead(200, {"Content-Type": "application/wasm"}).end(core);
    else response.writeHead(404).end();
  });
  server.listen(0, "127.0.0.1");
  try {
    await once(server, "listening");
    const address = server.address();
    assert.ok(address && typeof address === "object");
    const origin = "http://127.0.0.1:" + address.port + "/";
    const module = await instantiate((name: string) => compile(origin + name + "?v=test"));
    for (const compiled of [module, await compile(origin + fixture.source)]) {
      const instance = await WebAssembly.instantiate(compiled as WebAssembly.Module);
      assert.equal((instance.exports.ans as () => number)(), fixture.result);
    }
    assert.deepEqual(requests, ["/" + fixture.encoded + "?v=test", "/" + fixture.source]);
    await assert.rejects(compile(origin + "missing.core.wasm.gz.bin"), /404/);
  } finally {
    server.close();
    server.closeAllConnections();
    await once(server, "close");
  }
});
