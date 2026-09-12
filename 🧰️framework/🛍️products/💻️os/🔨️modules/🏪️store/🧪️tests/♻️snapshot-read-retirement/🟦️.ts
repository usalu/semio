import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";

type Case = { name: string; issued: number; cursor: number; returned: number[]; visits: (number | null)[] };

export function testSnapshotReadRetirement(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️snapshot-read-retirement/🔣️.json", import.meta.url), "utf8")) as { cases: Case[] };
  const validate = new Ajv().compile({ type: "object", required: ["name", "issued", "cursor", "returned", "visits"], additionalProperties: false, properties: {
    name: { type: "string" }, issued: { type: "integer", minimum: 1, maximum: 1024 }, cursor: { type: "integer", minimum: 0, maximum: 1023 },
    returned: { type: "array", uniqueItems: true, items: { type: "integer", minimum: 0, maximum: 1023 } },
    visits: { type: "array", items: { anyOf: [{ type: "null" }, { type: "integer", minimum: 0, maximum: 1023 }] } },
  } });
  for (const row of fixture.cases) {
    assert.ok(validate(row), JSON.stringify(validate.errors));
    let occupied: Record<string, boolean> = Object.fromEntries(Array.from({ length: row.issued }, (_, index) => [String(index), row.returned.includes(index)]));
    let cursor = row.cursor;
    const actual = row.visits.map(() => {
      const ordered = Object.keys(occupied).map(Number).sort((left, right) => (left - cursor + 1024) % 1024 - (right - cursor + 1024) % 1024);
      const index = ordered[0];
      assert.notEqual(index, undefined);
      cursor = (index + 1) % 1024;
      if (!occupied[index]) return null;
      occupied = applyPatch(occupied, [{ op: "remove", path: "/" + index }], true, false).newDocument;
      return index;
    });
    assert.deepEqual(actual, row.visits, row.name);
  }
  console.log("[DEBUG] sparse read retirement matches Ajv and an independent JSON Patch occupied-slot model");
}
