import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🚚️text-editor-lanes/🔣️.json";
import schema from "../../🧬️schema/🚚️text-editor-lanes/🔣️.json";
import { TEXT_EDITOR_SCENE_LANES, textEditorSceneFromLanes } from "../../🟦️.ts";

test("text buffer carriers match their language-neutral schema", () => {
  expect(new Ajv().compile(schema)(fixture)).toBe(true);
  expect(TEXT_EDITOR_SCENE_LANES).toEqual([{ lane: "buffer", field: fixture.field, bodyKey: fixture.laneKey, optional: false }]);
});

for (const row of fixture.cases) {
  test(`text buffers retain complete source: ${row.id}`, () => {
    const source = row.text.repeat(row.repeat);
    const spine = { buffer: "", language: "text", settingsJson: '{"readOnly":true}' };
    const restored = textEditorSceneFromLanes(spine, new Map([[fixture.laneKey, source]]));
    expect(restored).toEqual({ ...spine, buffer: source });
    expect(Buffer.from(restored.buffer).toString("utf8")).toBe(source);
    expect(spine.buffer).toBe("");
  });
}
