import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };

test("member constructor admission retains neutral original owners before any unfunded birth", () => {
  for (const row of fixture.cases) {
    const fields = [row.artifactId, row.actor, row.dialect.artifact_kind, row.dialect.standard, row.dialect.subset];
    expect(fields.map(field => Buffer.from(field, "utf8").toString("utf8"))).toEqual(fields);
    for (const denied of fixture.denied) {
      const original = { request: fields, allocated: 0 };
      const result = applyPatch(structuredClone(original), [], true).newDocument;
      expect(result).toEqual(original);
      expect(denied.maximumItems === 0 || denied.belowBirth).toBe(true);
    }
  }
  const store = readFileSync(resolve(import.meta.dir, "../../../../../../🦀️.rs"), "utf8");
  for (const method of ["open_birth_demand", "member_frame_bytes", "terminal_drop_byte_demand"]) expect(store.includes(method)).toBe(true);
  const begin = store.slice(store.indexOf("fn begin_open(request: &mut Option<$crate"), store.indexOf("async fn create(id:", store.indexOf("fn begin_open(request: &mut Option<$crate")));
  expect(begin.includes("grant.maximum_items == 0")).toBe(true);
  expect(begin.includes("grant.maximum_depth < required.depth")).toBe(true);
  expect(begin.includes("grant.maximum_capacity_bytes < required.capacity_bytes")).toBe(true);
  expect(begin.indexOf("grant.maximum_capacity_bytes < required.capacity_bytes")).toBeLessThan(begin.indexOf("request.take()"));
  expect(begin.includes("dialect.clone()")).toBe(false);
  expect(begin.includes("*request = Some(rejected.request)")).toBe(true);
  console.log("[DEBUG] Node Buffer + JSON Patch original identities remain untouched on denied constructor work/capacity; exact frame extents remain native architecture dependent");
});
