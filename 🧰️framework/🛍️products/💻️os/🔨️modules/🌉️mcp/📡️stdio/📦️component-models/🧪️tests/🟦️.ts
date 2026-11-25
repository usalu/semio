import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import schema from "../🔣️.json";
import { readMissingMcpModelV1 } from "../🕳️missing/🟦️.ts";
import corpus from "./🔣️.json";

const ajv = new Ajv({ strict: true });
const validate = ajv.compile({ ...schema.definitions.MissingModelDetails, $schema: undefined });

for (const row of corpus.reads) {
  test(`missing model refusal: ${row.name}`, () => {
    expect(readMissingMcpModelV1(row.frame as { result?: unknown; error?: unknown })).toEqual(row.read ?? undefined);
  });
}

test("missing model details agree with Ajv and independent SQLite JSON", () => {
  const db = new Database(":memory:");
  try {
    for (const row of corpus.missing) {
      expect(validate(row.payload.details)).toBe(true);
      const { id, crate, en } = db.query("select json_extract(?1, '$.details.missingModel.pluginId') id, json_extract(?1, '$.details.missingModel.cratePath') crate, json_extract(?1, '$.details.summary.en') en").get(JSON.stringify(row.payload)) as { id: string; crate: string; en: string };
      expect([id, crate]).toEqual([row.pluginId, row.cratePath]);
      expect(en).toBe(schema.definitions.MissingModel.const.summary.en.replace("{pluginId}", row.pluginId));
      expect(row.payload.message).toBe(en);
    }
    for (const row of corpus.reads) {
      const details = (row.frame as { result?: { structuredContent?: { details?: unknown } }; error?: { data?: { details?: unknown } } }).result?.structuredContent?.details ?? (row.frame as { error?: { data?: { details?: unknown } } }).error?.data?.details;
      if (row.read !== null) expect(validate(details)).toBe(true);
    }
  } finally {
    db.close();
  }
});
