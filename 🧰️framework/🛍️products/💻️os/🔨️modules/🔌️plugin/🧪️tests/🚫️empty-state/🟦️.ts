import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { parseSchemaRecord } from "../../../../../../🔨️modules/🧬️schema/🧾️record/🟦️.ts";

type EmptyStateFixture = Record<"json" | "text" | "pack", readonly { value: unknown; accepted: boolean }[]>;

export function testFrameworkEmptyStateContract(): void {
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true });
  assert(ajv.compile<EmptyStateFixture>(schema)(fixture));
  for (const lane of ["json", "text", "pack"] as const) {
    const validate = ajv.compile(schema.$defs[lane === "json" ? "record" : lane]);
    for (const row of fixture[lane]) {
      assert.equal(validate(row.value), row.accepted);
      if (lane === "json") {
        if (row.accepted) assert.deepEqual(parseSchemaRecord(row.value, []), {});
        else assert.throws(() => parseSchemaRecord(row.value, []));
      }
    }
  }
  console.log("[DEBUG] Framework empty-state admission: seven JSON, three text and three Pack vectors agree with independent Ajv");
}
