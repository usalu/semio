import { describe, expect, test } from "vitest";
import lodash from "lodash";
import Ajv from "ajv/dist/2020";
import schema from "./silhouettes.schema.json";
import cases from "./silhouettes.cases.json";
import { borderStatePriority, boxSilhouetteSegments, type ResolvedBorderSegment, type SilhouetteBox } from "./🟦️.ts";

type Side = "top" | "right" | "bottom" | "left";

/** 🔬 Independent oracle of the closure invariant: every perimeter point of a stated box lies on a resolved span of its own or a stronger state, computed by interval union through lodash. */
function uncovered(box: SilhouetteBox, resolved: readonly ResolvedBorderSegment[]): [Side, number, number][] {
  const { left, top, right, bottom } = box.rect;
  const sides: [Side, boolean, number, number, number][] = [["top", true, top, left, right], ["bottom", true, bottom, left, right], ["left", false, left, top, bottom], ["right", false, right, top, bottom]];
  return sides.flatMap(([side, horizontal, coordinate, low, high]) => {
    const spans = lodash.sortBy(resolved.filter(span => (span.from.y === span.to.y) === horizontal && (horizontal ? span.from.y : span.from.x) === coordinate && borderStatePriority[span.state] >= borderStatePriority[box.state])
      .map(span => [Math.min(horizontal ? span.from.x : span.from.y, horizontal ? span.to.x : span.to.y), Math.max(horizontal ? span.from.x : span.from.y, horizontal ? span.to.x : span.to.y)]), span => span[0]);
    const gaps: [Side, number, number][] = [];
    let reach = low;
    for (const [from, to] of spans) {
      if (from > reach) gaps.push([side, reach, from]);
      reach = Math.max(reach, to);
    }
    if (reach < high) gaps.push([side, reach, high]);
    return gaps;
  });
}

describe("box silhouettes", () => {
  test("silhouettes schema validates the case file", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const fixture of cases) {
    test(fixture.name, () => {
      const boxes = fixture.boxes as SilhouetteBox[];
      const snapshot = JSON.stringify(boxes);
      const resolved = boxSilhouetteSegments(boxes);
      expect(resolved).toEqual(fixture.expected);
      expect(boxSilhouetteSegments([...boxes].reverse())).toEqual(fixture.expected);
      expect(JSON.stringify(boxes)).toBe(snapshot);
      for (const box of boxes.filter(box => box.state !== "normal")) expect(uncovered(box, resolved), `${box.id} perimeter`).toEqual([]);
    });
  }
  test("the closure invariant holds on a generated grid of cells with every cell stated in turn", () => {
    const size = 5, width = 10;
    const owner: SilhouetteBox = { id: "g", parent: null, owner: "g", rect: { left: 0, top: 0, right: size * width, bottom: width }, edges: ["top", "right", "bottom", "left"], state: "normal" };
    for (const stated of lodash.range(size)) for (const state of ["focus", "active", "hover"] as const) {
      const boxes = [owner, ...lodash.range(size).map((index): SilhouetteBox => ({ id: `c${index}`, parent: "g", owner: "g", rect: { left: index * width, top: 0, right: (index + 1) * width, bottom: width }, edges: ["top", "right", "bottom", "left"], state: index === stated ? state : "normal" }))];
      const resolved = boxSilhouetteSegments(boxes);
      expect(uncovered(boxes[stated + 1]!, resolved)).toEqual([]);
      expect(lodash.sumBy(resolved, span => Math.abs(span.to.x - span.from.x) + Math.abs(span.to.y - span.from.y))).toBe(2 * size * width + 2 * width + (size - 1) * width);
    }
  });
  test("a normal box suppresses sides flush with an undrawn parent edge and a stated box does not", () => {
    const parent: SilhouetteBox = { id: "p", parent: null, owner: "p", rect: { left: 0, top: 0, right: 10, bottom: 10 }, edges: ["bottom"], state: "normal" };
    const child: SilhouetteBox = { id: "c", parent: "p", owner: "c", rect: { left: 0, top: 0, right: 10, bottom: 10 }, edges: ["top", "right", "bottom", "left"], state: "normal" };
    expect(lodash.uniq(boxSilhouetteSegments([parent, child]).map(span => span.id.split(":")[1]))).toEqual(["bottom"]);
    expect(lodash.uniq(boxSilhouetteSegments([parent, { ...child, state: "focus" }]).map(span => span.id.split(":")[1])).sort()).toEqual(["bottom", "left", "right", "top"]);
  });
});
