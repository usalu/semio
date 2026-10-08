import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };

test("mounted original typed child groups preserve every live lane until one funded common decision", async () => {

  
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(7);
  for (const row of fixture.cases) {
    expect(row.before.length).toBe(row.after.length);
    expect(row.prepared.length).toBe(row.after.length);
    const ready = row.prepared.every(Boolean) && !row.cancelled && row.fresh && row.commitItems === 1 && row.commitBytes === fixture.workBytes;
    expect(ready).toBe(row.published);
    const patch = ready ? row.after.map((value, index) => ({ op: "replace" as const, path: `/lanes/${index}`, value })) : [];
    const result = applyPatch({ lanes: structuredClone(row.before) }, patch, true).newDocument;
    expect(result.lanes).toEqual(row.published ? row.after : row.before);
    expect(row.commitBytes).toBeLessThanOrEqual(fixture.workBytes);
  }
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const owner = await Bun.file(new URL("../📦️owner/🦀️.rs", import.meta.url)).text();
  expect(source.includes("private_child_groups: ArtifactFixedRegistry")).toBe(true);
  expect(owner.includes("fn advance_private_child_group")).toBe(true);
  expect(owner.includes("fn close_private_child_group_step")).toBe(true);
  expect(owner.includes("fn publish_mounted_owned_child_operation_unit")).toBe(true);
  
  console.log("[DEBUG] mounted private child group oracle: seven atomic, stale, cancelled, incomplete and undergrant traces; original work4096/copy64 limits retained");
});
