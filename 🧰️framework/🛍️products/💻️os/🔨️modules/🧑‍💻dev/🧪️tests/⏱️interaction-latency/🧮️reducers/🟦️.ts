/** ⏱️ Checks the neutral latency scenarios and the actual nearest-rank quantile reducer. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { quantileV1, type LatencyScenarios } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/⏱️interaction-latency.json", import.meta.url), "utf8")) as LatencyScenarios;

describe("interaction-latency scenarios", () => {
  it("judge every scenario on its React commits per input", () => {
    expect(fixture.scenarios.filter((scenario) => scenario.maxCommitsPerInput === undefined).map((scenario) => scenario.id)).toEqual([]);
  });

  it("take the nearest-rank quantile", () => {
    expect(quantileV1([], 0.5)).toBeNull();
    expect(quantileV1([4, 1, 3, 2], 0.5)).toBe(3);
    expect(quantileV1([4, 1, 3, 2], 0.95)).toBe(4);
    expect(quantileV1([1.26], 1)).toBe(1.3);
  });
});
