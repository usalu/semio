import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parseTerrainBrushAction } from "../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🖌️brush/🟦️.ts";
import schema from "../🔣️.json";
import fixture from "./🔣️.json";

test("the World3d brush descriptor agrees with independent schema validation", () => {
  const validate = new Ajv({ multipleOfPrecision: 7 }).compile(schema);
  for (const input of fixture.accepted) {
    expect(validate(input)).toBe(true);
    expect(parseTerrainBrushAction(input)).toEqual(input);
  }
  for (const input of fixture.refused) {
    expect(validate(input)).toBe(false);
    expect(parseTerrainBrushAction(input)).toBeUndefined();
  }
});
