import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";

type Case = { name: string; issued: number; cursor: number; returned: number[]; visits: (number | null)[] };

export function testSnapshotReadRetirement(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️snapshot-read-retirement/🔣️.json", import.meta.url), "utf8")) as { cases: Case[] };
  for (const row of fixture.cases) {
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
}
