/** 🧩️ The browser twin of the hub's trusted plugin module contract (`🌎️hub-source/🧬️schema/🟦️.ts`), replayed from the
 * hub's own language-agnostic fixture `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧩️plugin-module/🔣️.json`
 * — the cases `🧩️plugin-module/🦀️.rs` answers — with Ajv as the independent oracle for every structural verdict. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import {
  decodeTrustedPluginModuleBundleV1,
  trustedPluginModuleBundleSha256V1,
  validateTrustedPluginModuleBundleV1,
  validateTrustedPluginModuleIndexV1,
  verifyTrustedPluginModuleFileV1,
  type TrustedPluginModuleSourceV1,
} from "../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts";

const root = "../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog";
const read = (path: string) => JSON.parse(readFileSync(fileURLToPath(new URL(`${root}/${path}`, import.meta.url)), "utf8"));
const fixture = read("🧫️fixtures/🧩️plugin-module/🔣️.json");
const schema = read("🧬️schema/🔣️.json");
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema({ ...schema, $ref: undefined, $id: "https://json.schemas.assets.semio-tech.com/hub/artifact-authority/trusted-catalog/schema.json" }, "trusted-catalog");
const oracle = (definition: string) => ajv.getSchema(`trusted-catalog#/$defs/${definition}`)!;
const source: TrustedPluginModuleSourceV1 = fixture.record;
const accepts = (run: () => unknown) => {
  try {
    run();
    return true;
  } catch {
    return false;
  }
};
const hexBytes = (hex: string) => Uint8Array.from(hex.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));

describe("🧩️ trusted plugin module bundle", () => {
  it("covers every refusal family the hub law covers", () => {
    expect(fixture.cases.length).toBeGreaterThanOrEqual(20);
    expect(fixture.index.length).toBeGreaterThanOrEqual(6);
  });
  for (const testCase of fixture.cases) {
    it(`manifest ${testCase.id}`, () => {
      expect(oracle("TrustedPluginModuleBundleV1")(testCase.manifest)).toBe(testCase.schemaValid);
      expect(accepts(() => validateTrustedPluginModuleBundleV1(testCase.manifest, source))).toBe(testCase.accepted);
      if (testCase.accepted) expect(testCase.schemaValid).toBe(true);
    });
  }
  for (const testCase of fixture.index) {
    it(`index ${testCase.id}`, () => {
      expect(oracle("TrustedPluginModuleIndexV1")(testCase.index)).toBe(testCase.schemaValid);
      expect(accepts(() => validateTrustedPluginModuleIndexV1(testCase.index))).toBe(testCase.accepted);
    });
  }
  it("decodes only the canonical bytes, addressed by the fixture's digest", async () => {
    const canonical = new TextEncoder().encode(fixture.canonical.bytesUtf8);
    expect(decodeTrustedPluginModuleBundleV1(canonical, source)).toEqual(fixture.cases[0].manifest);
    expect(await trustedPluginModuleBundleSha256V1(canonical)).toBe(fixture.canonical.sha256);
    expect(accepts(() => decodeTrustedPluginModuleBundleV1(new TextEncoder().encode(fixture.canonical.pretty), source))).toBe(false);
    expect(accepts(() => decodeTrustedPluginModuleBundleV1(canonical.slice(0, -1), source))).toBe(false);
  });
  it("verifies every listed file against its own bytes and refuses tampering", async () => {
    const manifest = validateTrustedPluginModuleBundleV1(fixture.cases[0].manifest, source);
    for (const file of manifest.files) expect(await verifyTrustedPluginModuleFileV1(file, hexBytes(fixture.contents[file.path]))).toBe(true);
    for (const testCase of fixture.fileCases) {
      const file = manifest.files.find((candidate) => candidate.path === testCase.path)!;
      expect(await verifyTrustedPluginModuleFileV1(file, hexBytes(testCase.contentHex)), testCase.id).toBe(testCase.accepted);
    }
  });
});
