import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { decodeRegistryDescriptorV1, REGISTRY_HOST_APP_CHANNEL_VERSION } from "../../🧬️schema/🟦️.ts";

const contract = JSON.parse(readFileSync(join(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8"));
const cases = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { id: string; valid: boolean; value: unknown }[];
const validate = new Ajv({ strict: true }).compile(contract);

describe("current package descriptor admission", () => {
  it("uses the installed locked independent oracle and current producer channel", () => {
    const require = createRequire(import.meta.url), version = require("ajv/package.json").version;
    let root = import.meta.dir;
    while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
    expect(readFileSync(join(root, "bun.lock"), "utf8")).toContain(`ajv@${version}`);
    const producer = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs"), "utf8");
    expect(Number(producer.match(/pub const CHANNEL_VERSION: u32 = (\d+);/)![1])).toBe(REGISTRY_HOST_APP_CHANNEL_VERSION);
    expect(contract.$defs.PackageDescriptorV1.properties.executionProtocol.properties.appChannelVersion.const).toBe(REGISTRY_HOST_APP_CHANNEL_VERSION);
  });
  it("admits the production owner through the current canonical scope inventory", async () => {
    let root = import.meta.dir;
    while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
    const library = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
    const inventory = library.inventorySchemaScopes(root), id = "os.plugin.registry.descriptor-verification";
    const scope = inventory.catalog.scopes[id];
    expect(scope).toBeDefined();
    expect(scope.path).toBe("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema");
    expect(scope.exports.PackageDescriptorV1).toBeDefined();
    const diagnostics = inventory.diagnostics.filter((row: { path: string }) => row.path.includes("📇️registry/🛂️descriptor-verification"));
    expect(diagnostics).toEqual([]);
    console.log("[DEBUG] Current genuine descriptor scope admitted", id, "exports", Object.keys(scope.exports).length, "owned diagnostics", diagnostics.length);
  });
  for (const row of cases) it(row.id, () => {
    expect(Boolean(validate(row.value))).toBe(row.valid);
    const bytes = new TextEncoder().encode(JSON.stringify(row.value));
    if (row.valid) expect(decodeRegistryDescriptorV1(bytes)).toEqual(row.value);
    else expect(() => decodeRegistryDescriptorV1(bytes)).toThrow();
  });
  it("refuses duplicate decoded members and malformed UTF-8 independently of value admission", () => {
    const source = JSON.stringify(cases[0]!.value).replace('"role":"plugin"', '"role":"plugin","\\u0072ole":"plugin"');
    expect(() => decodeRegistryDescriptorV1(new TextEncoder().encode(source))).toThrow(/repeats/);
    expect(() => decodeRegistryDescriptorV1(new Uint8Array([255]))).toThrow();
  });
});
