import { describe, expect, it } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseMeshDeliveryCatalog, resolveMeshAsset, meshAssetTransportUrl } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🔣️.json"), "utf8"));

describe("explicit mesh delivery authority", () => {
  it("agrees with independent JSON Schema admission and the neutral source/output map", async () => {
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true });
    const deliverySchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️catalog.schema.json"), "utf8"));
    const sourceSchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🌱️metabolism/🎨️representation/🧬️catalog.schema.json"), "utf8"));
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
    for (const path of ["../🧊️shape.glb", "/🧊️shape.glb", "🏠️house//🧊️shape.glb", "🏠️house/%2e%2e/🧊️shape.glb", "🏠️house\\🧊️shape.glb", "🏠️house/./🧊️shape.glb"]) {
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
