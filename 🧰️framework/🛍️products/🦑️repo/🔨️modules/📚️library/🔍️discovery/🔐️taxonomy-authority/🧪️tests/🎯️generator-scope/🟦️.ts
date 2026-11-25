import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { validateGeneratorContractsAgainstWorkspace } from "../../🟦️.ts";
import fixture from "./🔣️.json";

test("scoped generator checks retain complete owner authority", () => {
  const root = process.cwd();
  const taxonomy = JSON.parse(readFileSync(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
  expect(new Ajv().validate({ type: "object", required: ["assets-build"], properties: { "assets-build": { type: "object" } } }, taxonomy.generatorContracts)).toBe(true);
  const changed = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, [fixture.id]: fixture.contract } };
  for (const row of fixture.scopes) expect(validateGeneratorContractsAgainstWorkspace(root, changed, new Set(row.ids))).toHaveLength(row.problems);
});
