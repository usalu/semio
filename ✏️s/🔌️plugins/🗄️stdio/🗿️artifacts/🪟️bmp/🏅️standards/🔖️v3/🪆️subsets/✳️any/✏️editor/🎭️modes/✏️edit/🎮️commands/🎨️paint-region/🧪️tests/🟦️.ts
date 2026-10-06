import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** 🎨️ Validates the neutral BMP paint action corpus with an independent JSON Schema engine. */
export const runBmpPaintRegionChecks = (): number => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  let checks = 0;
  for (const row of fixture.cases) {
    if (!validate(row.payload)) throw new Error(`${row.name}: ${JSON.stringify(validate.errors)}`);
    checks += 1;
  }
  for (const row of fixture.invalid) {
    if (validate(row.payload)) throw new Error(`${row.name}: invalid paint payload was admitted`);
    checks += 1;
  }
  console.log(`[TRACE] bmp-paint-region-twin checks=${checks}`);
  return checks;
};
