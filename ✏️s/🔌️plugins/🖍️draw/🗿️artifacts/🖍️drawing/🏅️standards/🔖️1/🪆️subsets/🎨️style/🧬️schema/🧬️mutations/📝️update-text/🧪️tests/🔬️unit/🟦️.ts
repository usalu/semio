import { describe, expect, it } from "bun:test";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { applyTextEdit } from "../../🦠️mutation/🟦️.ts";

describe("semantic text editing", () => {
  const valid = semioSchemaAjvV1({allErrors: true}).compile(schema);
  for (const edit of fixture.edits) it(`roundtrips ${JSON.stringify(edit.content)}`, () => {
    const before = {layers: [{kind: "group", id: "group", children: [{kind: "text", id: "text", ...fixture.before}]}]};
    const mutation = {layerId: "text", ...edit};
    expect(valid({mutation: "updateText", ...mutation})).toBe(true);
    expect(valid(mutation)).toBe(false);
    const oracle = applyPatch(structuredClone(before), [
      {op: "replace", path: "/layers/0/children/0/content", value: edit.content},
      {op: "replace", path: "/layers/0/children/0/size", value: edit.size},
    ]).newDocument;
    const edited = applyTextEdit(before, mutation);
    expect(edited).toEqual(oracle);
    expect(applyTextEdit(edited, {layerId: "text", ...fixture.before})).toEqual(before);
    expect(before.layers[0].children[0]).toMatchObject(fixture.before);
  });
  it("rejects invalid sizes, missing targets and non-text targets without changes", () => {
    const before = {layers: [{kind: "text", id: "text", ...fixture.before}]};
    for (const size of [...fixture.invalidSizes, Infinity, NaN]) {
      const mutation = {layerId: "text", content: "changed", size};
      expect(valid({mutation: "updateText", ...mutation})).toBe(false);
      expect(() => applyTextEdit(before, mutation as never)).toThrow();
    }
    expect(() => applyTextEdit(before, {layerId: "missing", ...fixture.before})).toThrow();
    expect(() => applyTextEdit({layers: [{kind: "shape", id: "shape"}]}, {layerId: "shape", ...fixture.before})).toThrow();
    expect(before.layers[0]).toMatchObject(fixture.before);
  });

});

import before from "../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/⬅️before/🔣️.json";
import after from "../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/➡️after/🔣️.json";
import mutation from "../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/🦠️mutation/🔣️.json";
it("matches the canonical caption fixture and independent JSON Patch result", () => {
  expect(applyTextEdit(before, mutation)).toEqual(after);
  expect(applyPatch(structuredClone(before), [
    {op: "replace", path: "/layers/0/content", value: mutation.content},
    {op: "replace", path: "/layers/0/size", value: mutation.size},
  ]).newDocument).toEqual(after);
});
