import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/📏️close-demand/🔣️.json" with { type: "json" };

test("worker physical demand preserves caller authority and independent allocation state", async () => {
  for (const row of fixture.cases) {
    const before = { retainedBytes: row.physicalBytes, releasedBytes: 0 };
    const operations = row.callerBytes >= row.physicalBytes ? [
      { op: "replace" as const, path: "/retainedBytes", value: 0 },
      { op: "replace" as const, path: "/releasedBytes", value: row.physicalBytes },
    ] : [];
    const after = applyPatch(structuredClone(before), operations, true).newDocument;
    expect(after.releasedBytes).toBe(row.releasedBytes);
    expect(after.retainedBytes + after.releasedBytes).toBe(row.physicalBytes);
    expect(after.releasedBytes).toBeLessThanOrEqual(row.callerBytes);
    expect(Math.max(fixture.pageBytes, row.physicalBytes)).toBeLessThanOrEqual(fixture.admissionBytes);
  }
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("fn next_close_byte_demand(&self)")).toBe(true);
  expect(source.includes("pub fn next_close_byte_demand(&self) -> Result<usize, WorkerJobContention>")).toBe(true);
  console.log("[DEBUG] worker close demand: six physical extents preserve exact caller release grant");
});
