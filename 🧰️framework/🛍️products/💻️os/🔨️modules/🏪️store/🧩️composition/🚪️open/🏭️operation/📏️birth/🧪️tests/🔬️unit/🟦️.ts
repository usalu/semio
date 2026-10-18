import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { resolve } from "node:path";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

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
  const store = readModuleTreeSource(resolve(import.meta.dir, "../../../../../../🦀️.rs"));
  for (const method of ["open_birth_demand", "member_frame_bytes", "terminal_drop_byte_demand"]) expect(store.includes(method)).toBe(true);
  const begin = store.slice(store.indexOf("fn begin_open(admission:&mut $crate::os_store::MemberOpenSourceAdmission"), store.indexOf("async fn create(id:", store.indexOf("fn begin_open(admission:&mut $crate::os_store::MemberOpenSourceAdmission")));
  expect(begin.includes("grant.maximum_items==0")).toBe(true);
  expect(begin.includes("grant.maximum_depth<required.depth")).toBe(true);
  expect(begin.includes("grant.maximum_capacity_bytes<required.capacity_bytes")).toBe(true);
  expect(begin.indexOf("grant.maximum_capacity_bytes<required.capacity_bytes")).toBeLessThan(begin.indexOf("InitialMemberStoreOpen::begin("));
  expect(begin.includes("dialect.clone()")).toBe(false);
  expect(begin.includes("admission.request()")).toBe(true);
  expect(begin.includes("request.take()")).toBe(false);
});
