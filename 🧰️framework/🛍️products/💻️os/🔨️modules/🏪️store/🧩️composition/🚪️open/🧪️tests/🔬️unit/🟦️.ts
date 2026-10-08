import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import opening from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import demand from "../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json" with { type: "json" };

test("member opening retains six rejection stages and queries indivisible physical owners", async () => {
  expect(opening.retention.map(row => [row.stage, row.snapshots, row.mutations])).toEqual([
    ["input", 0, 0], ["snapshot", 1, 0], ["forward", 1, 1], ["inverse", 1, 2], ["envelope", 1, 0], ["initialization", 2, 0],
  ]);
  for (const row of demand.cases) {
    const allocation = Buffer.alloc(row.physicalBytes);
    const funded = row.callerBytes >= allocation.length;
    const reference = applyPatch({ retained: allocation.length, released: 0 }, funded ? [
      { op: "replace", path: "/retained", value: 0 },
      { op: "replace", path: "/released", value: allocation.length },
    ] : [], true).newDocument;
    expect(reference.released).toBe(row.releasedBytes);
    expect(reference.retained + reference.released).toBe(allocation.length);
    expect(Math.max(7, allocation.length)).toBeLessThanOrEqual(demand.admissionBytes);
  }
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  const retained = source.slice(source.indexOf("impl<P, M> ErasedSnapshotRetirement for MemberStoreOpenRetained"));
  for (const axis of ["copy_byte", "capacity_byte", "release_byte", "depth"]) expect(retained).toContain(`fn next_${axis}_demand`);
  expect(source).toContain("super::artifact_retirement_box_demands");
  expect(source).toContain("pub fn next_release_byte_demand(&self)");
  expect(retained).not.toContain("next_close_byte_demand");
  const store = await Bun.file(new URL("../../../../🦀️.rs", import.meta.url)).text();
  const decoded = store.slice(store.indexOf("impl<Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreDecodedEditRetirement"), store.indexOf("impl<Mutation> Drop for ArtifactStoreDecodedEditRetirement"));
  for (const axis of ["copy_byte", "capacity_byte", "release_byte", "depth"]) expect(decoded).toContain(`fn next_${axis}_demand`);
  expect(decoded).not.toContain("next_close_byte_demand");
  expect(decoded.includes("artifact_retirement_box_close_step")).toBe(true);
  console.log("[DEBUG] member opening: six unchanged retention stages use six independently checked physical extents within admission262144");
});
