/** 🧮️ Law of the idle-budget reducers over the language-agnostic fixture `../../../🧫️fixtures/💤️idle-budget.json` (expected values
 * computed by an independent Python implementation): the trace reduction picks the busiest renderer main thread, counts its
 * frames and animation frames, the renderer compositor's and the viz compositor's draws (never a tile worker's), and merges
 * overlapping tasks for busy %; the soak verdict's least-squares slopes, growth ratios, frame maximum and violations match. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { judgeSoak, reduceIdleTrace, type IdleReading, type SoakSample } from "../🟦️.ts";

type Fixture = {
  readonly schema: "semio.os-dev.idle-budget-fixture/v1";
  readonly trace: { readonly seconds: number; readonly events: Parameters<typeof reduceIdleTrace>[0]; readonly expected: IdleReading };
  readonly soaks: readonly { readonly name: string; readonly samples: readonly SoakSample[]; readonly expected: { readonly heapSlopeMBPerMin: number; readonly workersSlopeMBPerMin: number; readonly domGrowthRatio: number; readonly listenerGrowthRatio: number; readonly maxFramesPerSec: number; readonly violationKinds: readonly string[] } }[];
};

const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/💤️idle-budget.json", import.meta.url), "utf8")) as Fixture;
const KIND_OF_VIOLATION: readonly [RegExp, string][] = [[/^JS heap/u, "heap"], [/^worker heaps/u, "workers"], [/^DOM nodes/u, "dom"], [/^event listeners/u, "listeners"], [/frames\/s while idle$/u, "frames"]];

describe("idle-budget reducers", () => {
  it("reduces the fixture trace exactly", () => {
    expect(fixture.schema).toBe("semio.os-dev.idle-budget-fixture/v1");
    expect(reduceIdleTrace(fixture.trace.events, fixture.trace.seconds)).toEqual(fixture.trace.expected);
  });

  it.each(fixture.soaks.map((soak) => [soak.name, soak] as const))("judges the soak %s like the independent oracle", (_name, soak) => {
    const verdict = judgeSoak(soak.samples);
    const kinds = verdict.violations.map((violation) => KIND_OF_VIOLATION.find(([pattern]) => pattern.test(violation))?.[1] ?? violation);
    expect({ heap: verdict.heapSlopeMBPerMin, workers: verdict.workersSlopeMBPerMin, dom: verdict.domGrowthRatio, listeners: verdict.listenerGrowthRatio, frames: verdict.maxFramesPerSec, kinds }).toEqual({
      heap: soak.expected.heapSlopeMBPerMin,
      workers: soak.expected.workersSlopeMBPerMin,
      dom: soak.expected.domGrowthRatio,
      listeners: soak.expected.listenerGrowthRatio,
      frames: soak.expected.maxFramesPerSec,
      kinds: soak.expected.violationKinds,
    });
  });
});
