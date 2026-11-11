import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import {pluginFamilyText, pluginSubtreeText} from "../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";

test("direct original owned child dispatch uses the mounted publication authority", async () => {
  for (const row of fixture.cases) {
    const before = { parent: { label: "" }, child: { count: 0, label: "" } };
    const operations: any[] = [{ op: "replace", path: "/child/count", value: row.childCount }, { op: "replace", path: "/child/label", value: row.childLabel }];
    if (row.parentTouched) operations.push({ op: "replace", path: "/parent/label", value: row.parentLabel });
    expect(operations.length).toBe(row.operations);
    expect(Buffer.from(row.childLabel).toString("utf8")).toBe(row.childLabel);
    expect(applyPatch(structuredClone(before), operations, true).newDocument).toEqual({ parent: { label: row.parentLabel }, child: { count: row.childCount, label: row.childLabel } });
    expect(row.members).toBe(1 + Number(row.parentTouched));
  }
  const source = pluginSubtreeText("🧵️instance");
  expect(source.includes("dispatch.dispatch_original_emission(&mut self.original_emission.input,&mut output,cx).await?")).toBe(true);
  const owner = pluginSubtreeText("🧵️instance", "📨️dispatch");
  expect(owner.includes("pub async fn dispatch_original_emission")).toBe(true);
  expect(owner.includes("they land only through the stepped `dispatch_original_emission` mount")).toBe(true);
});

test("original completed output is retained before fallible mounted capture", async () => {
  for (const refused of [false, true]) {
    const original = { operations: ["count.set-count", "label.set-label"], label: "original 雪\0" };
    const expected = { retained: original, cancelled: refused, identity: 41 };
    const actual = applyPatch({ retained: structuredClone(original), cancelled: false, identity: 41 }, refused ? [{ op: "replace", path: "/cancelled", value: true }] : [], true).newDocument;
    expect(actual).toEqual(expected);
    expect(JSON.parse(JSON.stringify(actual)).retained).toEqual(original);
  }
  const owner = pluginSubtreeText("🧵️instance", "📨️dispatch");
  const bodyStart = owner.indexOf("pub async fn dispatch_original_emission"), body = owner.slice(bodyStart, bodyStart + 9000);
  const retained = body.indexOf("capture.as_mut().unwrap().receive(input,cx)");
  expect(retained).toBeGreaterThan(0);
  expect(body.indexOf("self.tools.app_registration(&input.verb)")).toBeGreaterThan(retained);
  expect(body.indexOf("capture_window_config")).toBeGreaterThan(retained);
  expect(body.indexOf("tool_operations.insert_admitted")).toBeGreaterThan(body.indexOf("capture_window_config"));
  expect(body.includes("original emission requires its exact registered native publication owner")).toBe(true);
});
