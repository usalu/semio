import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import { build } from "esbuild";
import { resolve } from "node:path";
import schema from "../../🧬️schema/🔣️.json";
import extensionSchema from "../../../../../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json";
import corpus from "../../🧫️fixtures/📇️inventory/🔣️.json";
import { moduleDirectoryName, moduleIdForDirectoryName, parseModuleDirectories } from "../../🟦️.ts";
import { emitTypeScript } from "../../../📽️projection/🟦️.ts";
import { parseDeployedRegistryEntryV1 } from "../../../🔎️discovery/🧬️schema/🟦️.ts";
import type { DeployedRegistryEntryV1 } from "../../../🔎️discovery/🟦️.ts";

const ajv = new Ajv({ strict: true }).addKeyword("x-semio-formats").addSchema(extensionSchema).addSchema(schema);
const shape = ajv.getSchema(`${schema.$id}#/$defs/DeploymentCatalogV1`)!;
expect(ajv.getSchema(`${schema.$id}#/$defs/DeploymentInventoryCasesV1`)!(corpus)).toBe(true);

describe("explicit deployment inventory", () => {
  for (const row of corpus.cases) it(row.id, () => {
    const validShape = shape(row.input);
    const modules = row.input.modules;
    const ids = new Set(modules.map((entry) => entry.pluginId));
    const emojis = modules.map((entry) => [...(entry.directoryName ?? "").matchAll(emojiRegex())].map((match) => match[0].replaceAll("\uFE0F", "")).join(""));
    const oracle = validShape && ids.size === modules.length && new Set(emojis).size === modules.length;
    expect(oracle).toBe(row.accepted);
    if (!row.accepted) {
      expect(() => parseModuleDirectories(row.input)).toThrow();
      return;
    }
    const inventory = parseModuleDirectories(row.input);
    expect(inventory).toEqual(row.input.modules);
    if (row.expected === null) expect(() => moduleDirectoryName(row.query, inventory)).toThrow();
    else {
      expect(moduleDirectoryName(row.query, inventory)).toBe(row.expected);
      expect(moduleIdForDirectoryName(row.expected, inventory)).toBe(row.query);
    }
    expect(moduleIdForDirectoryName("missing-directory", inventory)).toBeUndefined();
  });

  it("executes generated empty, replacement and deleted-owner catalogs in Bun and independent Node", async () => {
    const resolveDir = resolve(import.meta.dirname, "../../../🤖️generated/🧩️plugins");
    const runtimePath = resolve(import.meta.dirname, "../../🟦️.ts");
    const accepted = corpus.cases.filter((entry) => entry.accepted);
    expect(accepted.some((entry) => entry.input.modules.length === 0)).toBe(true);
    for (const row of accepted) {
      const entries: DeployedRegistryEntryV1[] = row.input.modules.map(({ pluginId, directoryName }) => parseDeployedRegistryEntryV1({
        pluginId, directoryName, packageId: `semio:${pluginId}`, packageName: `future-${pluginId}`,
        cratePath: `future/${pluginId}/📦️packages/🦀️rust`, wasmOut: `future_${pluginId}.wasm`, role: "plugin",
        capabilities: [], contributes: [], consumes: [], dependsOn: [], activationEvents: [], extensionPoints: [], executionMode: "isolated", hashes: {"wasmSha256": "0000000000000000000000000000000000000000000000000000000000000000", "coreWasmSha256": "0000000000000000000000000000000000000000000000000000000000000000", "descriptorSha256": "0000000000000000000000000000000000000000000000000000000000000000"},
      }));
      const source = emitTypeScript(entries);
      const assertion = `\nif(JSON.stringify(COMPONENT_MODULE_DIRECTORIES)!==${JSON.stringify(JSON.stringify(row.input.modules))})throw Error("Inventory drift");\nlet actual=null;try{actual=pluginModuleUrl(${JSON.stringify(row.query)})}catch{}\nif(actual!==${JSON.stringify(row.expected === null ? null : `/🔌️plugin-modules/${row.expected}/🌉️bridge.js`)})throw Error("Owner route drift");\nconsole.log("[DEBUG] Independent generated inventory ${row.id}");`;
      const executable = source.replace('"../../📦️deployment/🟦️.ts"', JSON.stringify(runtimePath)).replace("\"../../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts\"", JSON.stringify(resolve(resolveDir, "../../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts"))) + assertion;
      const actual = Bun.spawnSync(["bun", "run", "-"], { stdin: Buffer.from(executable), stdout: "pipe", stderr: "pipe" });
      expect(actual.exitCode, Buffer.from(actual.stderr).toString()).toBe(0);
      expect(Buffer.from(actual.stdout).toString()).toContain(`[DEBUG] Independent generated inventory ${row.id}`);
      const independent = await build({ stdin: { contents: source + assertion, resolveDir, loader: "ts" }, bundle: true, platform: "node", format: "esm", write: false });
      const result = Bun.spawnSync(["node", "--input-type=module", "-e", independent.outputFiles![0]!.text], { stdout: "pipe", stderr: "pipe" });
      expect(result.exitCode, Buffer.from(result.stderr).toString()).toBe(0);
      expect(Buffer.from(result.stdout).toString()).toContain(`[DEBUG] Independent generated inventory ${row.id}`);
    }
  });
});
