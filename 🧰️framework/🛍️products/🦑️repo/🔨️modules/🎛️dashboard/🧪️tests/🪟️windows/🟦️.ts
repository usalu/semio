/** 🪟️ The docking command wire contract and its independent Ajv oracle. @see ./🥒️.feature */
import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import maxBy from "lodash/maxBy.js";
import { readFileSync } from "node:fs";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const schema = read("../../🧬️schema/🪟️windows/🔣️.json");
const vectors = read("../../🧫️fixtures/🪟️windows/🔣️.json");
const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);

test("docking command vectors conform to the schema and reject ambiguous placements", () => {
  for (const command of [...vectors.commands, ...vectors.invalid]) expect(validate(command), JSON.stringify(validate.errors)).toBe(true);
  for (const placement of schema.properties.placement.enum) expect(validate({ source: "a", target: "b", placement })).toBe(true);
  for (const command of [
    { source: "a", target: "b", placement: "diagonal" },
    { source: "", target: "b", placement: "left" },
    { source: "a", placement: "tabs" },
    { source: "a", target: "b", placement: "tabs", tabIndex: -1 },
    { source: "a", target: "b", placement: "tabs", tabIndex: 1.5 },
    { source: "a", target: "b", placement: "tabs", mutateDuringPreview: true },
  ]) expect(validate(command)).toBe(false);
});

test("four corner placements have distinct commands and non-overlapping tab-slot previews", () => {
  for (const vector of vectors.corners) {
    expect(validate({ source: "a", target: "a", placement: vector.placement })).toBe(true);
    const [x, y, width, height] = vectors.area;
    const expected = [vector.corner.endsWith("Right") ? x + width - 24 : x, vector.corner.startsWith("bottom") ? y + height - 2 : y, 24, 2];
    expect(vector.rect).toEqual(expected);
  }
  expect(new Set(vectors.corners.map((row: any) => row.corner)).size).toBe(4);
});

test("body zones follow the regular UI dominant axis under an independent lodash selection", () => {
  const [x, y, width, height] = vectors.area;
  for (const vector of vectors.bodyZones) {
    const dx = vector.point[0] - x - width / 2;
    const dy = vector.point[1] - y - height / 2;
    const selected = maxBy([{ distance: Math.abs(dx), placement: dx < 0 ? "left" : "right" }, { distance: Math.abs(dy), placement: dy < 0 ? "top" : "bottom" }], "distance");
    expect(selected?.placement).toBe(vector.placement);
  }
});

test("shared preview vectors partition the available canvas with exactly one divider", () => {
  const [x, y, width, height] = vectors.area;
  const byPlacement = new Map(vectors.previews.map((vector: any) => [vector.placement, vector.rect]));
  const left = byPlacement.get("left") as number[];
  const right = byPlacement.get("right") as number[];
  const top = byPlacement.get("top") as number[];
  const bottom = byPlacement.get("bottom") as number[];
  expect(left[2]! + right[2]! + 1).toBe(width);
  expect(top[3]! + bottom[3]! + 1).toBe(height);
  expect(right[0]).toBe(x + left[2]! + 1);
  expect(bottom[1]).toBe(y + top[3]! + 1);
  expect(byPlacement.get("tabs")).toEqual(vectors.area);
});
