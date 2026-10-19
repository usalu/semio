import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import contract from "../../🛂️descriptor-verification/🧬️schema/🔣️.json" with { type: "json" };
import { verifySemanticDescriptorPairBytesV1 } from "../../✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { DESCRIPTOR_JSON_REL_PATH, findPluginCargoFiles, parseComponentSourceOwnerV1 } from "../../🔎️discovery/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

test("committed semantic source pairs agree with current schema and independent Ajv", () => {
  const root = findWorkspaceRoot(import.meta.dir), validate = new Ajv({ strict: false, allErrors: true }).compile(contract), manifests = findPluginCargoFiles(root);
  expect(manifests.length).toBeGreaterThan(0);
  for (const manifest of manifests) {
    const source = parseComponentSourceOwnerV1(manifest, root), path = resolve(root, source.cratePath, ...DESCRIPTOR_JSON_REL_PATH), json = readFileSync(path), value = JSON.parse(json.toString("utf8"));
    expect(validate(value), `${source.pluginId}: ${JSON.stringify(validate.errors)}`).toBe(true);
    const pair = verifySemanticDescriptorPairBytesV1(json, readFileSync(join(dirname(path), "🛂️.descriptor.semio")));
    expect(pair.pluginId).toBe(source.pluginId);
    expect(pair.packageId).toBe(source.packageId);
    expect(pair.role).toBe(source.role);
  }
}, 60_000);
