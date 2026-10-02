import { describe, expect, it } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { createAssetBuildPluginsV1, parseAssetDeliveryDeclarationV1, type AssetDeliveryProviderV1 } from "../🟦️.ts";

describe("caller-owned asset dispatch", () => {
  it("matches every portable declaration against independent Ajv", async () => {
    const { default: Ajv } = await import("ajv");
    const admit = new Ajv({ strict: true }).compile(schema);
    for (const row of corpus.declarations) {
      expect(admit(row.value), row.id).toBe(row.accepted);
      if (row.accepted) expect(parseAssetDeliveryDeclarationV1(row.value), row.id).toEqual(row.value);
      else expect(() => parseAssetDeliveryDeclarationV1(row.value), row.id).toThrow();
    }
  });

  it("extends and deletes providers without changing the dispatcher", () => {
    for (const row of corpus.dispatch) {
      const providers: AssetDeliveryProviderV1[] = row.providers.map(kind => ({ kind, middleware: () => (_request, _response, next) => next(), plugins: (context, declaration) => [{ name: kind + ":" + declaration.route + ":" + context.mode }] }));
      const run = () => createAssetBuildPluginsV1(import.meta.dir, row.specs, providers, "bundle").map(plugin => plugin.name);
      if (row.plugins === null) expect(run, row.id).toThrow();
      else expect(run(), row.id).toEqual(row.plugins);
    }
  });

  it("runs actual HTTP with all products and built-in providers refused by an independent Node loader", async () => {
    const { build } = await import("esbuild");
    const framework = resolve(import.meta.dir, "../../../../..");
    const refused = [resolve(framework, "🛍️products"), resolve(framework, "🔨️modules/🖼️assets/🗺️tile-proxy"), resolve(framework, "🔨️modules/🖼️assets/🥽️mesh")].map(path => path.replaceAll("\\", "/") + "/");
    const wire = corpus.transport;
    const program = `import {createAssetHttpServerV1} from ${JSON.stringify(resolve(import.meta.dir, "../🟦️.ts"))};import {once} from "node:events";const wire=${JSON.stringify(wire)};const provider={kind:wire.kind,plugins:()=>[],middleware:()=>((request,response,next)=>{if(request.url!==wire.route)return next();response.end(Buffer.from(wire.bytes))})};const server=createAssetHttpServerV1(${JSON.stringify(import.meta.dir)},0,[{kind:wire.kind,route:wire.route}],[provider]);await once(server,"listening");try{const address=server.address();const url="http://127.0.0.1:"+address.port;const response=await fetch(url+wire.route);const missing=await fetch(url+"/missing");console.log(JSON.stringify({status:response.status,bytes:[...new Uint8Array(await response.arrayBuffer())],missingStatus:missing.status}));}finally{await new Promise((accept,reject)=>{server.close(error=>error?reject(error):accept());server.closeAllConnections()})}`;
    const bundle = await build({ stdin: { contents: program, resolveDir: import.meta.dir }, bundle: true, platform: "node", format: "esm", write: false, plugins: [{ name: "removed-asset-owners", setup(builder) { builder.onLoad({ filter: /.*/ }, input => refused.some(prefix => input.path.replaceAll("\\", "/").startsWith(prefix)) ? { errors: [{ text: "Neutral asset dispatcher loads a removed owner: " + input.path }] } : undefined); } }] });
    const native = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(bundle.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
    expect(native.exitCode, Buffer.from(native.stderr).toString()).toBe(0);
    const actual = JSON.parse(Buffer.from(native.stdout).toString());
    expect(actual).toEqual({ status: wire.status, bytes: wire.bytes, missingStatus: wire.missingStatus });
    expect(readFileSync(resolve(import.meta.dir, "../🟦️.ts"), "utf8")).not.toContain("🛍️products");
  });
});
