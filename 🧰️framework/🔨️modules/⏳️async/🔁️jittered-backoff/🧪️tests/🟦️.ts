/** 🧪️ Pending attempt cancellation matches Node's timer oracle and observes either late settlement. */
import { setTimeout as delay } from "node:timers/promises";
import { expect, it } from "vitest";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { retryWithJitteredBackoff } from "../🟦️.ts";

for (const settlement of fixture.cases) it(`cancels before late ${settlement}`, async () => {
  const controller = new AbortController();
  const reason = Error(fixture.abortReason);
  let fulfill: (() => void) | undefined;
  let reject: ((error: Error) => void) | undefined;
  const pending = new Promise<void>((accept, refuse) => { fulfill = accept; reject = refuse; });
  const operation = retryWithJitteredBackoff(() => pending, { minMs: 1, maxMs: 1, signal: controller.signal }).then(() => "completed", error => error.message);
  const oracle = delay(10_000, undefined, { signal: controller.signal }).catch(error => error.cause.message);
  const timer = setTimeout(() => controller.abort(reason), fixture.timeoutMs);
  try {
    const observed = await Promise.race([operation, delay(1_000, "pending")]);
    expect(observed).toBe(fixture.abortReason);
    expect(observed).toBe(await oracle);
  } finally { clearTimeout(timer); if (settlement === "reject") reject?.(Error("late failure")); else fulfill?.(); }
  await Promise.resolve();
});
