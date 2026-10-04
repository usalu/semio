import { describe, expect, it } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseMeshDeliveryCatalog, resolveMeshAsset, meshAssetTransportUrl } from "../../🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🔣️.json"), "utf8"));

describe("explicit mesh delivery authority", () => {
  it("keeps concrete product transport out of the general mesh dependency closure", async () => {
    const { build } = await import("esbuild");
    const { default: Ajv } = await import("ajv");
    const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }).addSchema(schema);
    expect(ajv.compile({ $ref: schema.$id + "#/$defs/MeshTransportCasesV1" })(fixture.transport)).toBe(true);
    const catalog = parseMeshDeliveryCatalog(fixture.delivery, path => fixture.catalogs[path]);
    for (const row of fixture.transport) {
      if (!row.valid) expect(() => meshAssetTransportUrl(row.url, catalog), row.id).toThrow();
      else expect(new URL(meshAssetTransportUrl(row.url, catalog), "https://owner.test").href, row.id).toBe(new URL(row.output, "https://owner.test").href);
    }
    const products = resolve(import.meta.dir, "../../../../../🛍️products") + "/";
    const program = `import { parseMeshDeliveryCatalog, meshAssetTransportUrl } from ${JSON.stringify(resolve(import.meta.dir, "../../🟦️.ts"))}; const fixture=${JSON.stringify(fixture)}; const catalog=parseMeshDeliveryCatalog(fixture.delivery,path=>fixture.catalogs[path]); console.log(JSON.stringify(fixture.transport.map(row=>{try{return {valid:true,output:meshAssetTransportUrl(row.url,catalog)}}catch{return {valid:false,output:null}}})));`;
    const bundle = await build({ stdin: { contents: program, resolveDir: import.meta.dir }, bundle: true, platform: "node", format: "esm", write: false, plugins: [{ name: "mesh-product-removal", setup(builder) {
      builder.onLoad({ filter: /.*/ }, input => {
        const path = input.path.replaceAll("\\", "/");
        if (path.startsWith(products.replaceAll("\\", "/")) || path.includes("/🖼️assets/🌱️metabolism/") || path.endsWith("/🥽️mesh/📇️catalog.json")) return { errors: [{ text: "General mesh loads a concrete product or asset collection: " + input.path }] };
      });
    } }] });
    const node = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(bundle.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
    expect(node.exitCode, Buffer.from(node.stderr).toString()).toBe(0);
    expect(JSON.parse(Buffer.from(node.stdout).toString())).toEqual(fixture.transport.map((row: { valid: boolean; output: string | null }) => ({ valid: row.valid, output: row.output })));
  });

  it("agrees with independent JSON Schema admission and the neutral source/output map", async () => {
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true });
    const deliverySchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
    const sourceSchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌱️metabolism/🎨️representation/🧬️schema/🔣️.json"), "utf8"));
    expect(ajv.compile(deliverySchema)(fixture.delivery)).toBe(true);
    const validateSource = ajv.compile(sourceSchema);
    for (const value of Object.values(fixture.catalogs)) expect(validateSource(value)).toBe(true);
    const catalog = parseMeshDeliveryCatalog(fixture.delivery, path => fixture.catalogs[path]);
    expect(catalog).toEqual(fixture.expected);
    for (const expected of fixture.expected) {
      expect(resolveMeshAsset(expected.url, catalog)).toEqual(expected);
      expect(meshAssetTransportUrl(expected.url, catalog)).toBe(`/mesh/${expected.path}`);
    }
    expect(meshAssetTransportUrl("https://external.test/model.glb", catalog)).toBe("https://external.test/model.glb");
    for (const url of fixture.unknown) expect(() => resolveMeshAsset(url, catalog)).toThrow();
  });

  it("rejects duplicate identities, duplicate destinations, duplicate sources, traversal and unknown fields", () => {
    for (const key of ["url", "source", "path"]) {
      const input = structuredClone(fixture.delivery);
      input.entries.push({ url: "/mesh/🛖️hut.glb", source: "🛖️hut/🧊️shape.glb", path: "🛖️hut/🧊️shape.glb", [key]: input.entries[0][key] });
      expect(() => parseMeshDeliveryCatalog(input, path => fixture.catalogs[path])).toThrow();
    }
    for (const path of ["../../🧊️shape.glb", "/🧊️shape.glb", "🏠️house//🧊️shape.glb", "🏠️house/%2e%2e/🧊️shape.glb", "🏠️house\\🧊️shape.glb", "🏠️house/./🧊️shape.glb"]) {
      const input = structuredClone(fixture.delivery);
      input.entries[0].path = path;
      expect(() => parseMeshDeliveryCatalog(input, path => fixture.catalogs[path])).toThrow();
    }
    const extra = structuredClone(fixture.delivery);
    extra.entries[0].alias = "/mesh/old.glb";
    expect(() => parseMeshDeliveryCatalog(extra, path => fixture.catalogs[path])).toThrow();
    expect(() => parseMeshDeliveryCatalog(fixture.delivery, () => undefined)).toThrow();
    const sources = structuredClone(fixture.catalogs);
    sources["📦️sources/📇️catalog.json"].entries[1].url = sources["📦️sources/📇️catalog.json"].entries[0].url;
    expect(() => parseMeshDeliveryCatalog(fixture.delivery, path => sources[path])).toThrow();
  });
});
