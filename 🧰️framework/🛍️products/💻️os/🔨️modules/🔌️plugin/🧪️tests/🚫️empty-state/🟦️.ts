import { test } from "vitest";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { parseSchemaRecord } from "../../../../../../🔨️modules/🧬️schema/🧾️record/🟦️.ts";

type EmptyStateFixture = Record<"json" | "text" | "pack", readonly { value: unknown; accepted: boolean }[]>;

export function testFrameworkEmptyStateContract(): void {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🫙️empty-state/🔣️.json", import.meta.url), "utf8"));
  const fixture: EmptyStateFixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🚫️empty-state/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true });
  for (const lane of ["json", "text", "pack"] as const) {
    const validate = ajv.compile(schema.$defs[lane === "json" ? "EmptyStateRecordV1" : lane === "text" ? "EmptyStateTextV1" : "EmptyStatePackV1"]);
    for (const row of fixture[lane]) {
      assert.equal(validate(row.value), row.accepted);
      if (lane === "json") {
        if (row.accepted) assert.deepEqual(parseSchemaRecord(row.value, []), {});
        else assert.throws(() => parseSchemaRecord(row.value, []));
      }
    }
  }
}

test("actual empty state values agree with canonical domain schemas and independent Ajv", testFrameworkEmptyStateContract);
