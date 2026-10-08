import { test, expect } from "bun:test";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

test("mounted original parent/children neutral atomic rows and independent inverse", () => {
  for (const row of fixture.cases) {
    const before = { parent: { label: "" }, children: Array.from({ length: row.children }, () => ({ count: 0, label: "" })) };
    const patch = [...(row.parentTouched ? [{ op: "replace" as const, path: "/parent/label", value: row.parentLabel }] : []), ...row.childCounts.flatMap((count, i) => [{ op: "replace" as const, path: `/children/${i}/count`, value: count }, { op: "replace" as const, path: `/children/${i}/label`, value: row.childLabel }])];
    const after = applyPatch(structuredClone(before), patch, true).newDocument;
    expect(after.children.map(child => child.count)).toEqual(row.childCounts);
    expect(after.parent.label).toBe(row.parentTouched ? row.parentLabel : "");
    const inverse = patch.toReversed().map(part => ({ op: "replace" as const, path: part.path, value: part.path.endsWith("count") ? 0 : "" }));
    expect(applyPatch(after, inverse, true).newDocument).toEqual(before);
    expect(patch.length).toBe(row.expectedOperations);
    expect(row.children + Number(row.parentTouched)).toBe(row.expectedMemberEdits);
    expect(Buffer.from(row.childLabel, "utf8").equals(Buffer.from(new TextEncoder().encode(row.childLabel)))).toBe(true);
  }
});

import { readFileSync } from "node:fs";
test("every original prepared operation supplies its exact borrowed semantic schema", () => {
  const parts = [["count", "set-count"], ["label", "set-label"]];
  expect(parts.map(([entity, kind]) => Buffer.concat([Buffer.from(entity!), Buffer.from("."), Buffer.from(kind!)]).toString("utf8"))).toEqual(fixture.expectedSchemas);
  const store = readFileSync(new URL("../../../../🏪️store/🦀️.rs", import.meta.url), "utf8");
  expect(store.includes("prepared_operation_schema_parts")).toBe(true);
});
