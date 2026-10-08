import { expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

test("direct original owned child dispatch uses the mounted publication authority", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const before = { parent: { label: "" }, child: { count: 0, label: "" } };
    const operations: any[] = [{ op: "replace", path: "/child/count", value: row.childCount }, { op: "replace", path: "/child/label", value: row.childLabel }];
    if (row.parentTouched) operations.push({ op: "replace", path: "/parent/label", value: row.parentLabel });
    expect(operations.length).toBe(row.operations);
    expect(Buffer.from(row.childLabel).toString("utf8")).toBe(row.childLabel);
    expect(applyPatch(structuredClone(before), operations, true).newDocument).toEqual({ parent: { label: row.parentLabel }, child: { count: row.childCount, label: row.childLabel } });
    expect(row.members).toBe(1 + Number(row.parentTouched));
  }
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("self.mount_original_emit_publication(verb, emit, meta, None).await")).toBe(true);
  const owner = await Bun.file(new URL("../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs", import.meta.url)).text();
  expect(owner.includes("fn mount_original_emit_publication")).toBe(true);
  console.log("[DEBUG] Ajv/RFC6902/Node UTF8 original direct owned child two cases, unchanged one-item262144/100000 limits");
});

test("original completed output is retained before fallible mounted capture", async () => {
  for (const refused of [false, true]) {
    const original = { operations: ["count.set-count", "label.set-label"], label: "original 雪\0" };
    const expected = { retained: original, cancelled: refused, identity: 41 };
    const actual = applyPatch({ retained: structuredClone(original), cancelled: false, identity: 41 }, refused ? [{ op: "replace", path: "/cancelled", value: true }] : [], true).newDocument;
    expect(actual).toEqual(expected);
    expect(JSON.parse(JSON.stringify(actual)).retained).toEqual(original);
  }
  const owner = await Bun.file(new URL("../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs", import.meta.url)).text();
  const body = owner.slice(owner.indexOf("async fn mount_original_emit_publication"), owner.indexOf("fn advance_private_child_group"));
  const installed = body.indexOf("self.tool_operations.insert_admitted");
  expect(installed).toBeGreaterThan(0);
  expect(body.indexOf("self.qualified_tool_proof(verb)")).toBeGreaterThan(installed);
  expect(body.indexOf("self.window_config_store.capture")).toBeGreaterThan(installed);
  expect(body.includes("mounted.terminal_fault")).toBe(true);
  console.log("[DEBUG] RFC6902/Node original completed-output success/refusal traces retain both authored operations and exactidentity beforefalliblecapture");
});
