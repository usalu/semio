import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🌦️ Checks artifact-owned MIME policy against an independent schema validator. */
export function runDefinitionChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true }).compile(load("../../🧬️schema/🔣️.json"));
  let checks = 0;
  for (const row of load("../../🧫️fixtures/📜️definition/🔣️.json").cases) {
    const source = load("../../📜️artifact-definition.json");
    source.representations[0].mimes = row.mimes;
    assert.equal(validate(source), row.accepted);
    checks++;
  }
  return checks;
}
