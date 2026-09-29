import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🚚️table-lanes/🔣️.json";
import schema from "../../🧬️schema/🚚️table-lanes/🔣️.json";
import { TABLE_SCENE_LANES, tableSceneFromLanes } from "../../🟦️.ts";

test("table carriers match their language-neutral schema", () => {
  expect(new Ajv().compile(schema)(fixture)).toBe(true);
  expect(TABLE_SCENE_LANES).toEqual<typeof fixture.lanes>(fixture.lanes);
});

test("large tables retain every row and Unicode cell through paged carriers", () => {
  const rows = Array.from({ length: fixture.rowCount }, (_, index) => ({ ...fixture.row, id: String(index) }));
  const spine = { columnsJson: "", rowsJson: "", selectionJson: '{"row":"4095"}' };
  const restored = tableSceneFromLanes(spine, new Map([[fixture.lanes[0]!.bodyKey, JSON.stringify(fixture.columns)], [fixture.lanes[1]!.bodyKey, JSON.stringify(rows)]]));
  expect(JSON.parse(restored.columnsJson)).toEqual(fixture.columns);
  expect(JSON.parse(restored.rowsJson)).toEqual(rows);
  expect(restored.selectionJson).toBe(spine.selectionJson);
  expect(spine.rowsJson).toBe("");
});
