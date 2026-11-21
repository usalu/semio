import { describe, expect, test } from "vitest";
import lodash from "lodash";
import Ajv from "ajv/dist/2020";
import schema from "./states.schema.json";
import cases from "./states.cases.json";
import { borderStatePriority, resolveBoxStates, type BorderState, type StateBox } from "./🟦️.ts";

/** 🔬 Independent oracle: ancestor chains and rank selection through lodash collections. */
function referenceStates(boxes: readonly StateBox[], pointer: string | null, focusVisible: string | null): Record<string, BorderState> {
  const byId = lodash.keyBy(boxes, "id");
  const chain = (id: string | null): StateBox[] => id === null ? [] : [byId[id]!, ...chain(byId[id]!.parent)];
  const pointerChain = chain(pointer);
  const hovered = lodash.find(pointerChain, box => box.interactive && !box.disabled)?.id;
  const enclosed = new Set(lodash.filter(pointerChain, box => box.hoverWithin && !box.disabled).map(box => box.id));
  const focused = lodash.find(chain(focusVisible), box => box.interactive)?.id;
  return lodash.mapValues(byId, box => lodash.maxBy<BorderState>([...box.marks, "normal", ...(box.id === hovered || enclosed.has(box.id) ? ["hover" as const] : []), ...(box.id === focused ? ["focus" as const] : [])], state => borderStatePriority[state])!);
}

describe("box state ownership", () => {
  test("states schema validates the case file", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const fixture of cases) {
    test(fixture.name, () => {
      const boxes = fixture.boxes as StateBox[];
      const snapshot = JSON.stringify(boxes);
      expect(resolveBoxStates(boxes, fixture.pointer, fixture.focusVisible)).toEqual(fixture.expected);
      expect(referenceStates(boxes, fixture.pointer, fixture.focusVisible)).toEqual(fixture.expected);
      expect(resolveBoxStates([...boxes].reverse(), fixture.pointer, fixture.focusVisible)).toEqual(fixture.expected);
      expect(JSON.stringify(boxes)).toBe(snapshot);
    });
  }
  test("ordinary ancestors do not acquire hover or focus from a descendant", () => {
    const depth = 6;
    const boxes: StateBox[] = lodash.range(depth).map(index => ({ id: `n${index}`, parent: index ? `n${index - 1}` : null, interactive: index % 2 === 1, disabled: false, marks: [] }));
    for (const leaf of boxes) {
      const states = resolveBoxStates(boxes, leaf.id, leaf.id);
      expect(lodash.filter(Object.values(states), state => state !== "normal").length).toBe(leaf.id === "n0" ? 0 : 1);
      expect(states).toEqual(referenceStates(boxes, leaf.id, leaf.id));
    }
  });
  test("an unknown pointer or focus id states nothing", () => {
    const boxes: StateBox[] = [{ id: "a", parent: null, interactive: true, disabled: false, marks: [] }];
    expect(resolveBoxStates(boxes, "missing", "missing")).toEqual({ a: "normal" });
  });
});
