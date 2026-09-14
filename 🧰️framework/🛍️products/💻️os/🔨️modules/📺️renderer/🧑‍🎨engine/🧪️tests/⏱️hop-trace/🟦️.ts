/** ⏱️ The renderer hop tracer's laws: the stage vocabulary the probe and the host both read is the
 * declared one, a span is published whether its body returned or threw, closing twice publishes once,
 * a full ring drops its OLDEST spans, and the inter-hop gap — the term no single stage holds — is
 * measured between consecutive `invoke` spans.
 *
 * 🧫️ The vocabulary is read from `🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json`, so a second
 * implementation of the breakdown (the probe's table, a native tracer) is pinned against the same
 * list rather than a hand-kept copy.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../../../../../../🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json" with { type: "json" };
import {
  createHopTracer,
  HOP_TRACE_MEASURE_PREFIX,
  HOP_TRACE_RING_CAPACITY,
  HOP_TRACE_STAGES,
  hopTraceEpochNowMs,
  hopTraceEpochToTimeline,
  hopTraceInterHopGapsMs,
  hopTraceTotalsByStage,
  type HopTraceDetail,
  type HopTracePorts,
} from "../../../../../../../🔨️modules/⏱️trace/🟦️.ts";

/** 🕰️ A virtual clock plus a User Timing sink, so a law drives the real tracer with no browser. */
const virtualPorts = (): HopTracePorts & { readonly tick: (ms: number) => void; readonly measured: { name: string; startMs: number; endMs: number; detail: HopTraceDetail }[] } => {
  let nowMs = 0;
  const measured: { name: string; startMs: number; endMs: number; detail: HopTraceDetail }[] = [];
  return {
    now: () => nowMs,
    measure: (name, startMs, endMs, detail) => measured.push({ name, startMs, endMs, detail }),
    tick: (ms: number) => {
      nowMs += ms;
    },
    measured,
  };
};

describe("⏱️ renderer hop trace", () => {
  it("publishes exactly the declared stage vocabulary under the declared prefix", () => {
    expect(HOP_TRACE_MEASURE_PREFIX).toBe(fixture.measurePrefix);
    expect(HOP_TRACE_RING_CAPACITY).toBe(fixture.ringCapacity);
    expect([...HOP_TRACE_STAGES]).toEqual(fixture.stages.map((stage) => stage.id));
  });

  it("times a synchronous body and publishes one measure per span", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    const value = tracer.time("encode", { actionId: "flowEvalTick" }, () => {
      ports.tick(7);
      return 42;
    });
    expect(value).toBe(42);
    expect(tracer.spans()).toEqual([{ stage: "encode", startMs: 0, durationMs: 7, detail: { actionId: "flowEvalTick" } }]);
    expect(ports.measured).toEqual([{ name: "semio.hop.encode", startMs: 0, endMs: 7, detail: { actionId: "flowEvalTick" } }]);
  });

  it("publishes a span whose body threw, and re-throws", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    expect(() =>
      tracer.time("channel", undefined, () => {
        ports.tick(11);
        throw new Error("guest trapped");
      }),
    ).toThrow("guest trapped");
    expect(tracer.spans().map((span) => [span.stage, span.durationMs])).toEqual([["channel", 11]]);
  });

  it("publishes a span whose awaited body rejected", async () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    await expect(
      tracer.timeAsync("channel", undefined, async () => {
        ports.tick(5);
        throw new Error("shard lost");
      }),
    ).rejects.toThrow("shard lost");
    expect(tracer.spans().map((span) => [span.stage, span.durationMs])).toEqual([["channel", 5]]);
  });

  it("closes once however many times close is called, and merges the closing detail", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    const close = tracer.open("invoke", { actionId: "flowEvalTick" });
    ports.tick(3);
    close({ frames: 2 });
    ports.tick(3);
    close({ frames: 99 });
    expect(tracer.spans()).toEqual([{ stage: "invoke", startMs: 0, durationMs: 3, detail: { actionId: "flowEvalTick", frames: 2 } }]);
    expect(fixture.invariants.closeIsIdempotent).toBe(true);
  });

  it("drops the oldest spans when the ring is full", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    for (let index = 0; index < HOP_TRACE_RING_CAPACITY + 5; index += 1) tracer.time("commit", { index }, () => ports.tick(1));
    const spans = tracer.spans();
    expect(spans.length).toBe(HOP_TRACE_RING_CAPACITY);
    expect(spans[0]!.detail.index).toBe(5);
    expect(fixture.invariants.oldestDropsWhenFull).toBe(true);
  });

  it("sums per stage and measures the inter-hop gap between consecutive invoke spans", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    tracer.time("invoke", undefined, () => ports.tick(300));
    ports.tick(400);
    tracer.time("invoke", undefined, () => ports.tick(200));
    ports.tick(100);
    tracer.time("invoke", undefined, () => ports.tick(250));
    tracer.time("commit", undefined, () => ports.tick(16));
    const totals = hopTraceTotalsByStage(tracer.spans());
    expect(totals.invoke).toEqual({ count: 3, totalMs: 750 });
    expect(totals.commit).toEqual({ count: 1, totalMs: 16 });
    expect(totals.encode).toBeUndefined();
    expect(hopTraceInterHopGapsMs(tracer.spans())).toEqual([400, 100]);
    expect(fixture.invariants.interHopGapIsMeasuredBetweenInvokeSpans).toBe(true);
  });

  it("never throws when the host has no User Timing sink, and still keeps the ring", () => {
    const tracer = createHopTracer({
      now: () => 0,
      measure: () => {
        throw new Error("no user timing");
      },
    });
    expect(() => tracer.time("decode", undefined, () => "decoded")).not.toThrow();
    expect(tracer.spans().map((span) => span.stage)).toEqual(["decode"]);
    expect(fixture.invariants.tracerNeverThrows).toBe(true);
  });

  it("records a span measured in another realm's clock, so the shard worker's own stages land on this timeline", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    ports.tick(1_000);
    tracer.record("worker.guest", 120, 6.25, { actorId: "a", patches: 0 });
    tracer.record("worker.reply", 126.25, 3.5, { actorId: "a" });
    // 🧭️ A recorded span is placed where it HAPPENED, not where the clock is now — otherwise the
    // worker's stages would pile up at the reply instant and the breakdown would not add up.
    expect(tracer.spans().map((span) => [span.stage, span.startMs, span.durationMs])).toEqual([
      ["worker.guest", 120, 6.25],
      ["worker.reply", 126.25, 3.5],
    ]);
    expect(ports.measured.map((measure) => measure.name)).toEqual([`${HOP_TRACE_MEASURE_PREFIX}worker.guest`, `${HOP_TRACE_MEASURE_PREFIX}worker.reply`]);
    expect(hopTraceTotalsByStage(tracer.spans())["worker.guest"]).toEqual({ count: 1, totalMs: 6.25 });
  });

  it("clamps a negative duration rather than publishing a span that ends before it starts", () => {
    const tracer = createHopTracer(virtualPorts());
    tracer.record("worker.receive", 50, -4);
    expect(tracer.spans()[0]!.durationMs).toBe(0);
  });

  it("declares every worker stage the shard worker measures, and the two invariants that make them readable", () => {
    for (const stage of ["worker.turn", "worker.receive", "worker.decode", "worker.guest", "worker.reply"] as const) expect(HOP_TRACE_STAGES).toContain(stage);
    expect(fixture.invariants.workerStagesShareTheEpochClockNotTheTimeline).toBe(true);
    expect(fixture.invariants.workerTurnEqualsDecodePlusGuest).toBe(true);
  });

  it("places a shared-epoch instant on this realm's own timeline and never before its origin", () => {
    const origin = typeof performance === "object" && typeof performance.timeOrigin === "number" ? performance.timeOrigin : 0;
    expect(hopTraceEpochToTimeline(origin + 1_250)).toBeCloseTo(1_250, 6);
    expect(hopTraceEpochToTimeline(origin - 10)).toBe(0);
    expect(hopTraceEpochNowMs()).toBeGreaterThanOrEqual(origin);
  });

  it("empties the ring on reset", () => {
    const ports = virtualPorts();
    const tracer = createHopTracer(ports);
    tracer.time("arm", undefined, () => ports.tick(2));
    tracer.reset();
    expect(tracer.spans()).toEqual([]);
  });
});
