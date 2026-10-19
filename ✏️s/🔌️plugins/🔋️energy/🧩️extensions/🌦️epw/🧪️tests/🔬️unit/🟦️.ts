import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { unit } from "mathjs";

/** 🌦️ Checks authored EPW mapping vectors against the neutral weather contract. */
export function runWeatherChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const weatherSchema = load("../../../../🔨️modules/⚡️simulation/⚙️engine/📍️site/🧬️schema/🔣️.json");
  const fixture = load("../🧫️fixtures/🌦️weather/🔣️.json");
  const ajv = new Ajv({ strict: true });
  ajv.addSchema(weatherSchema);
  const validate = ajv.compile(weatherSchema);
  const validateWireHour = ajv.compile(load("../../🧬️schema/🔣️.json"));
  let checks = 1;
  for (const row of fixture.cases) {
    assert.equal(row.expected !== null, row.accepted);
    if (row.accepted || row.id === "hour-zero-refuses" || row.id === "hour-twenty-five-refuses") {
      const hours = row.input.trimEnd().split("\n").slice(8).map((line: string) => Number(line.split(",")[3]));
      assert.equal(hours.every((hour: number) => validateWireHour(hour)), row.accepted, row.id);
    }
    if (row.expected !== null) {
      assert.equal(validate(row.expected), true, JSON.stringify(validate.errors));
      const wire = row.input.trimEnd().split("\n").slice(8).map((line: string) => line.split(","));
      row.expected.records.forEach((record: { snow_depth_mm: number; precipitation_mm: number }, index: number) => {
        assert.equal(record.snow_depth_mm, unit(Number(wire[index]![30]), "cm").toNumber("mm"));
        assert.equal(record.precipitation_mm, unit(Number(wire[index]![33]), "mm").toNumber("mm"));
      });
    }
    checks++;
  }
  for (const owner of ["📍️site", "🏛️bestest"]) {
    const source = readFileSync(new URL(`../../../../🔨️modules/⚡️simulation/⚙️engine/${owner}/🦀️.rs`, import.meta.url), "utf8");
    assert.doesNotMatch(source, /Epw|EPW|epw|semio_s_artifact_/u);
    checks++;
  }
  console.log(`[DEBUG] Energy EPW weather neutral ${fixture.cases.length} vectors Ajv/mathjs cm-to-mm oracle passed`);
  return checks;
}
