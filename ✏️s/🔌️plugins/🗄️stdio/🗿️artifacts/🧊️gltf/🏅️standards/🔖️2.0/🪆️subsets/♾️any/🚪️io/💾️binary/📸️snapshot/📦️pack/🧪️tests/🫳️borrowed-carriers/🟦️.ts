import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

/** 🫳️ Checks portable GLTF metadata examples against the owned carrier laws. */
export function runBorrowedCarrierChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const fixture = load("🔣️.json");
  assert.equal(new Set(fixture.cases.map((row: { owner: string }) => row.owner)).size, 6);
  for (const row of fixture.cases) {
    assert.deepEqual(row.fields.map((field: { id: number }) => field.id), row.fields.map((_: unknown, index: number) => index));
    assert.equal(new Set(row.fields.map((field: { key: string }) => field.key)).size, row.fields.length);
  }
  return fixture.cases.length + 2;
}
