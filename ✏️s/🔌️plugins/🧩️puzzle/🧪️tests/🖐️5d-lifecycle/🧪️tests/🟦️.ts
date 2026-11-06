import { expect, test } from "bun:test";
import plan from "../🔣️.json";
import { timeTravelScenarioV1 as scenario } from "../🟦️.ts";

test("5d lifecycle names its own surfaces and executable browser step", () => {
  expect(plan.schema).toBe("s.puzzle5d.lifecycle.v1");
  expect([plan.boardWindow, plan.worldWindow]).toEqual(["puzzle5d-2d", "puzzle5d-3d"]);
  expect(scenario.variant).toBe("puzzle5d");
  expect(scenario.bootWindowPrefix).toBe("puzzle5d-");
  expect(scenario.order).toEqual([1]);
  expect(typeof scenario.steps[1]).toBe("function");
});
