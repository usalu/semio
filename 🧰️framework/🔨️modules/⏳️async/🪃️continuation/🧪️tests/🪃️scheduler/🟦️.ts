import { describe, expect, it } from "vitest";
import { continuationCaseFaults, runContinuationCase, runContinuationCaseLive, createVirtualContinuationHost, createContinuationScheduler, type ContinuationSuite, type ContinuationCancel } from "../../🟦️.ts";

const { readFileSync } = await import("node:fs");
const { fileURLToPath } = await import("node:url");
const { dirname, join } = await import("node:path");
const suite = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🔣️.json"), "utf8")) as ContinuationSuite;

describe("🪃️ continuation scheduler", () => {
  it.each(suite.cases.map((testCase) => [testCase.name, testCase] as const))("holds the law on a virtual clock: %s", (_name, testCase) => {
    expect(continuationCaseFaults(testCase, runContinuationCase(testCase))).toEqual([]);
  });

  it.each(suite.cases.map((testCase) => [testCase.name, testCase] as const))("holds the same law on the real event loop: %s", async (_name, testCase) => {
    expect(continuationCaseFaults(testCase, await runContinuationCaseLive(testCase), 250)).toEqual([]);
  });

  it("never touches the host timer for a zero-delay request", () => {
    const host = createVirtualContinuationHost();
    const armed: number[] = [];
    const scheduler = createContinuationScheduler({ ...host.ports, setTimer: (run, delayMs) => { armed.push(delayMs); return host.ports.setTimer(run, delayMs); } });
    for (let hop = 0; hop < 64; hop += 1) scheduler.schedule(() => {}, 0);
    host.drain();
    expect(armed).toEqual([]);
  });

  it("keeps exactly ONE host timer armed for many deadlines", () => {
    const host = createVirtualContinuationHost();
    let live = 0;
    const scheduler = createContinuationScheduler({
      ...host.ports,
      setTimer: (run, delayMs) => { live += 1; return host.ports.setTimer(run, delayMs); },
      clearTimer: (handle) => { live -= 1; host.ports.clearTimer(handle); },
    });
    for (const delayMs of [90, 80, 70, 60, 50]) scheduler.schedule(() => {}, delayMs);
    expect(live).toBe(1);
    host.drain();
    expect(scheduler.pending()).toBe(0);
  });

  it("survives a chain that outlives its own cancellation", () => {
    const host = createVirtualContinuationHost();
    const scheduler = createContinuationScheduler(host.ports);
    const ran: number[] = [];
    let hop = 0;
    let cancel: ContinuationCancel = () => {};
    const step = (): void => {
      hop += 1;
      ran.push(hop);
      if (hop === 3) { cancel(); return; }
      cancel = scheduler.schedule(step, 0);
    };
    cancel = scheduler.schedule(step, 0);
    host.drain();
    expect(ran).toEqual([1, 2, 3]);
    expect(scheduler.pending()).toBe(0);
  });

  it("resolves yieldContinuation on an unthrottled macrotask, ahead of a pending deadline", async () => {
    const scheduler = createContinuationScheduler();
    const order: string[] = [];
    scheduler.schedule(() => order.push("deadline"), 200);
    await scheduler.yieldContinuation();
    order.push("yield");
    expect(order).toEqual(["yield"]);
    scheduler.dispose();
  });
});
