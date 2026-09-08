import assert from "node:assert/strict";
import { createRequire } from "node:module";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import mapping from "../🔣️.json";

/** 🧪️ Compares preference forwarding with the schema fixture and independent lodash transforms. */
export async function testPlaygroundPreferences(): Promise<void> {
  const { frameworkOsLockedPrefsEnv } = await import("../🟦️.ts");
  const require = createRequire(import.meta.url), lodash = require("lodash");
  assert.ok(require("jsonschema").validate(fixture, schema).valid);
  const expected = lodash.mapKeys(lodash.pickBy(lodash.mapValues(lodash.pick(fixture.environment, Object.keys(mapping)), (value: string) => value.trim()), Boolean), (_value: string, key: keyof typeof mapping) => mapping[key]);
  assert.deepEqual(expected, fixture.expected);
  assert.deepEqual(frameworkOsLockedPrefsEnv(fixture.environment), expected);
  assert.deepEqual(frameworkOsLockedPrefsEnv({}), {});
  console.log("[DEBUG] Pure playground preference forwarding matches the schema fixture and lodash oracle PASS");
}
