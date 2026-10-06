import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🌦️ Checks authored EPW mapping vectors against the neutral weather contract. */
export function runWeatherChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const weatherSchema = load("../../../../🔨️modules/⚡️simulation/⚙️engine/📍️site/🧬️schema/🔣️.json");
  const fixture = load("../../🧫️fixtures/🌦️weather/🔣️.json");
  const ajv = new Ajv({ strict: true });
  ajv.addSchema(weatherSchema);
  const validate = ajv.compile(weatherSchema);
  let checks = 1;
  for (const row of fixture.cases) {
    assert.equal(row.expected !== null, row.accepted);
    if (row.expected !== null) assert.equal(validate(row.expected), true, JSON.stringify(validate.errors));
    checks++;
  }
  for (const owner of ["📍️site", "🏛️bestest"]) {
    const source = readFileSync(new URL(`../../../../🔨️modules/⚡️simulation/⚙️engine/${owner}/🦀️.rs`, import.meta.url), "utf8");
    assert.doesNotMatch(source, /Epw|EPW|epw|semio_s_artifact_/u);
    checks++;
  }
  return checks;
}
