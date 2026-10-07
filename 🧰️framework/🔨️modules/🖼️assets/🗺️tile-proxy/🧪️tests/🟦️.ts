import { describe, expect, it } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseTileProxyAssetSpecV1, TILE_PROXY_TRANSPORT_LIMITS_V1 } from "../🟦️.ts";

const authority = resolve(import.meta.dir, "..");
const schema = JSON.parse(readFileSync(resolve(authority, "🧬️schema/🔣️.json"), "utf8"));
const corpus = JSON.parse(readFileSync(resolve(authority, "🧫️fixtures/🔣️.json"), "utf8"));

describe("owner-neutral tile proxy contract", () => {
  it("matches the portable schema, finite transport limits and independent Ajv admission", async () => {
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(schema);
    const admit = ajv.getSchema(schema.$id)!;
    for (const row of corpus.cases) {
      expect(admit(row.value), row.id).toBe(row.accepted);
      if (row.accepted) expect(parseTileProxyAssetSpecV1(row.value), row.id).toEqual(row.value);
      else expect(() => parseTileProxyAssetSpecV1(row.value), row.id).toThrow();
    }
    expect(ajv.compile({ $ref: schema.$id + "#/definitions/TileProxyTransportLimitsV1" })(TILE_PROXY_TRANSPORT_LIMITS_V1)).toBe(true);
  });

  it("executes every portable case in independent Node while all concrete product loading is refused", async () => {
    const { build } = await import("esbuild");
    const products = resolve(import.meta.dir, "../../../../🛍️products").replaceAll("\\", "/") + "/";
    const program = `import { parseTileProxyAssetSpecV1 } from ${JSON.stringify(resolve(authority, "🟦️.ts"))};const cases=${JSON.stringify(corpus.cases)};console.log(JSON.stringify(cases.map(row=>{try{return{accepted:true,value:parseTileProxyAssetSpecV1(row.value)}}catch{return{accepted:false,value:null}}})));`;
    const bundle = await build({ stdin: { contents: program, resolveDir: import.meta.dir }, bundle: true, platform: "node", format: "esm", write: false, plugins: [{ name: "tile-proxy-product-removal", setup(builder) {
      builder.onLoad({ filter: /.*/ }, input => input.path.replaceAll("\\", "/").startsWith(products) ? { errors: [{ text: "General tile contract loads a concrete product: " + input.path }] } : undefined);
    } }] });
    const node = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(bundle.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
    expect(node.exitCode, Buffer.from(node.stderr).toString()).toBe(0);
    const actual = JSON.parse(Buffer.from(node.stdout).toString());
    expect(actual).toEqual(corpus.cases.map((row: { accepted: boolean; value: unknown }) => ({ accepted: row.accepted, value: row.accepted ? row.value : null })));
  });
});
