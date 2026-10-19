import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { SCRIPT_PROVIDERS_ENV, defineScriptProviderV1, loadScriptProviderSyncV1, loadScriptProviderV1, readScriptProviderMapV1, writeScriptProviderMapV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

interface Probe { ping(value: number): number }
const probe = defineScriptProviderV1<Probe>("probe-port", ["ping"]);

test("provider designation corpus agrees between the framework validator and Ajv", () => {
  const ajv = new Ajv({ strict: false }), validate = ajv.compile(schema);
  for (const row of fixture.designations) {
    expect(validate(row.value), row.id).toBe(row.valid);
    expect(validateJsonSchemaSubset(schema, row.value).length === 0, row.id).toBe(row.valid);
  }
  for (const row of fixture.names) {
    const accepts = (() => { try { defineScriptProviderV1(row.name, ["ping"]); return true; } catch { return false; } })();
    expect(accepts, JSON.stringify(row.name)).toBe(row.valid);
    expect(new RegExp(schema.properties.providers.propertyNames.pattern, "u").test(row.name) && row.name.length > 0, JSON.stringify(row.name)).toBe(row.valid);
  }
});

test("a designation round-trips through the child environment and refuses relative paths", () => {
  const wire = writeScriptProviderMapV1({ "probe-port": join(tmpdir(), "provider.ts") });
  expect(readScriptProviderMapV1({ [SCRIPT_PROVIDERS_ENV]: wire }).providers["probe-port"]).toBe(join(tmpdir(), "provider.ts"));
  expect(() => writeScriptProviderMapV1({ "probe-port": "relative/provider.ts" })).toThrow("Complete script provider designation required");
  expect(() => readScriptProviderMapV1({})).toThrow("Script provider designation required");
  expect(() => readScriptProviderMapV1({ [SCRIPT_PROVIDERS_ENV]: "{" })).toThrow("Complete script provider designation required");
  expect(() => readScriptProviderMapV1({ [SCRIPT_PROVIDERS_ENV]: JSON.stringify({ version: 1, providers: { "probe-port": "relative.ts" } }) })).toThrow("Absolute script provider module paths required");
});

test("the designated module is loaded once, checked against the declared functions and refused otherwise", async () => {
  const directory = mkdtempSync(join(tmpdir(), "semio-providers-"));
  try {
    const good = join(directory, "good.ts"), missing = join(directory, "missing.ts"), absent = join(directory, "absent.ts");
    writeFileSync(good, "globalThis.__semioProviderLoads = (globalThis.__semioProviderLoads ?? 0) + 1;\nexport const provider = { ping: (value: number) => value + 1 };\n");
    writeFileSync(missing, "export const provider = { pong: () => 0 };\n");
    writeFileSync(absent, "export const other = {};\n");
    const environment = (path: string) => ({ [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1({ "probe-port": path }) });
    const first = await loadScriptProviderV1(probe, environment(good)), second = await loadScriptProviderV1(probe, environment(good));
    expect(first.ping(1)).toBe(2);
    expect(second).toBe(first);
    expect((globalThis as { __semioProviderLoads?: number }).__semioProviderLoads).toBe(1);
    await expect(loadScriptProviderV1(probe, environment(missing))).rejects.toThrow("lacks ping");
    await expect(loadScriptProviderV1(probe, environment(absent))).rejects.toThrow("must export provider");
    await expect(loadScriptProviderV1(probe, { [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1({ "other-port": good }) })).rejects.toThrow("probe-port is not designated");
    expect(pathToFileURL(good).href.startsWith("file://")).toBe(true);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test("the synchronous loader checks the declared functions, shares the module cache with the asynchronous loader and refuses otherwise", async () => {
  const directory = mkdtempSync(join(tmpdir(), "semio-providers-sync-"));
  try {
    const good = join(directory, "good.ts"), missing = join(directory, "missing.ts"), absent = join(directory, "absent.ts");
    writeFileSync(good, "globalThis.__semioSyncProviderLoads = (globalThis.__semioSyncProviderLoads ?? 0) + 1;\nexport const provider = { ping: (value: number) => value + 1 };\n");
    writeFileSync(missing, "export const provider = { pong: () => 0 };\n");
    writeFileSync(absent, "export const other = {};\n");
    const environment = (path: string) => ({ [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1({ "probe-port": path }) });
    const first = loadScriptProviderSyncV1(probe, environment(good)), second = loadScriptProviderSyncV1(probe, environment(good));
    expect(first.ping(1)).toBe(2);
    expect(second).toBe(first);
    expect(await loadScriptProviderV1(probe, environment(good))).toBe(first);
    expect((globalThis as { __semioSyncProviderLoads?: number }).__semioSyncProviderLoads).toBe(1);
    expect(() => loadScriptProviderSyncV1(probe, environment(missing))).toThrow("lacks ping");
    expect(() => loadScriptProviderSyncV1(probe, environment(absent))).toThrow("must export provider");
    expect(() => loadScriptProviderSyncV1(probe, { [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1({ "other-port": good }) })).toThrow("probe-port is not designated");
    expect(() => loadScriptProviderSyncV1(probe, {})).toThrow("Script provider designation required");
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
