/** 🔤️ Independent schema and byte expectations for the native BMP source contract. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

export function runBmpSourceHexFixtureChecks(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔤️native-source-hex/🔣️.json", import.meta.url), "utf8"));
  
  
  
  assert.equal(new Set(fixture.cases.map((row: { id: string }) => row.id)).size, fixture.cases.length);
  let checks = 2;
  for (const row of fixture.cases) {
    const compact = row.source.replace(/\s/gu, "");
    assert.equal(/^(?:[0-9a-fA-F]{2})+$/u.test(compact), row.valid, row.id);
    checks++;
    if (row.valid) {
      assert.deepEqual([...Buffer.from(compact, "hex")], fixture.bytes, row.id);
      checks++;
    }
  }
  console.log(`[DEBUG] BMP source-hex fixture: ${fixture.cases.length} cases, independent native hexadecimal bytes checked`);
  return checks;
}
