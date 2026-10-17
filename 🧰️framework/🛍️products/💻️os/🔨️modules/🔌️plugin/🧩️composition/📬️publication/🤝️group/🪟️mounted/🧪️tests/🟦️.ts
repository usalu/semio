import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { pluginFamilyText } from "../../../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";

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
  const family = pluginFamilyText();
  const owner = await Bun.file(new URL("../../📦️owner/🦀️.rs", import.meta.url)).text();
  expect(family.includes("pub private_child_groups: ArtifactFixedRegistry<Box<MountedPrivateChildGroup<M>>>")).toBe(true);
  expect(owner.includes("pub fn advance_openings(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)")).toBe(true);
  expect(family.includes("return group.advance_openings(cx,grant);")).toBe(true);
  expect(family.includes("fn advance_private_child_group(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>")).toBe(true);
  expect(family.includes("fn close_private_child_group_step<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>")).toBe(true);
  expect(family.includes("fn publish_mounted_owned_child_operation_unit(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>")).toBe(true);
});
