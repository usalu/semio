import { expect, test } from "bun:test";
import corpus from "./🧫️fixtures/🔣️.json";
import { composeSqliteSchema } from "../../🟦️.ts";

test("schema composition matches the language-neutral corpus and UTF-8 byte lengths", () => {
  for (const row of corpus.cases) {
    expect(composeSqliteSchema(row.segments)).toBe(row.schema);
    expect(new TextEncoder().encode(composeSqliteSchema(row.segments)).length).toBe(row.segments.reduce((sum, segment) => sum + Buffer.byteLength(segment, "utf8"), 0));
  }
});
