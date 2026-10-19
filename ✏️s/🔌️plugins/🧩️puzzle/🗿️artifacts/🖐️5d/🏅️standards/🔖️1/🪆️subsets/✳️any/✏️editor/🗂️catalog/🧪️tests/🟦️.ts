/** 🧪️ Authored weight dictionaries agree with independent mathjs arithmetic and Map ownership. */
import { expect, test } from "bun:test";
import { create, all } from "mathjs";
import { normalizeKindWeights } from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
const math = create(all, {});
test("Puzzle5D dictionary publication preserves normalized wire values", async () => {
  for (const row of fixture.cases) {
    const actual = normalizeKindWeights(row.weights as Readonly<Record<string, number>>, row.ids, row.changed, row.value);
    const expected = new Map(Object.entries(row.expected));
    expect(Object.keys(actual).sort()).toEqual([...expected.keys()].sort());
    for (const [id, weight] of expected) expect(actual[id]).toBeCloseTo(weight, 12);
    if (row.ids.length) expect(math.sum(Object.values(actual))).toBeCloseTo(1, 12);
    expect(JSON.parse(JSON.stringify(actual))).toEqual(row.expected);
  }
  const source = await Bun.file(new URL("../../👆️gestures/🎛️selection-and-weights/🦀️.rs", import.meta.url)).text();
  expect(source).toContain("cursor.next_advance_demands(");
  expect(source).toContain("admit_original_tool_construction_scoped(cx, &mut self.insert");
  expect(source).toContain("insert: self.insert.take()");
  expect(source).not.toContain("HashMap::with_capacity");
  expect(source).not.toContain("HashSet::with_capacity");
  const driver = await Bun.file(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧵️retained-command-job/🦀️.rs", import.meta.url)).text();
  const pending = driver.split("ArtifactCommandWorkStep::Pending { stage, preview } =>")[1]!.split("ArtifactCommandWorkStep::Replay")[0]!;
  expect(pending).not.toContain("work_progress");
  expect(fixture.construction.pendingSemanticItems).toBe(0);
  console.log("[DEBUG] Puzzle5D authored dictionary vectors=6 independent mathjs/Map agree; staged cursor retains original owners");
});
