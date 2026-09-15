/** 🎚️ The host half of a continuous gesture, as laws — the lane every dragged slider and held
 * spinner dispatches through (`🖱️ui/🎬️scene/🟦️.ts`'s `createContinuousGestureLane`).
 *
 * A 60 Hz gesture over a round trip that costs more than 16 ms has exactly two honest answers: queue
 * every value, or keep only the newest one and send it when the receiver is free. Sending every one
 * was measured on 6018 as 29 `patchFlowWidgets`, 24 `toolRunStart`s and 29 history entries for ONE
 * one-second drag of the Inspection panel's number field, with the mesh arriving 3.0 s behind the
 * value — the user watching a backlog of stale geometry drain after letting go
 * (`📓️slider-preview-update-2026-09-15.md`).
 *
 * The four properties below ARE "butter-smooth", stated so they can fail:
 *   1. at most ONE send in flight and ONE owed, so N rapid values cost at most 2 live round trips;
 *   2. the value that is sent after a round trip is the LATEST one, never an intermediate;
 *   3. the release is always sent, and always as `commit`, even when its value already went out
 *      live — the receiver needs it to close its coalesced edit, which is what makes a drag ONE
 *      history entry instead of N;
 *   4. a refused round trip frees the lane instead of wedging the gesture.
 *
 * The Rust twin over the same table is `✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit`'s
 * `every_continuous_gesture_row_costs_one_edit_and_ends_on_the_released_value`.
 */
import { describe, expect, it } from "vitest";
import { createContinuousGestureLane, type ContinuousGesturePhase } from "@semio-tech/framework";

type Sent = { readonly value: number; readonly phase: ContinuousGesturePhase };

/** 🔌️ A receiver whose round trips are resolved by the test, one at a time, so "in flight" is a fact
 * the law controls rather than a timing accident. */
function manualReceiver() {
  const sent: Sent[] = [];
  const pending: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  return {
    sent,
    pending,
    send(value: number, phase: ContinuousGesturePhase) {
      sent.push({ value, phase });
      return new Promise<void>((resolve, reject) => pending.push({ resolve, reject }));
    },
    async settleOne() {
      const next = pending.shift();
      next?.resolve();
      await Promise.resolve();
      await Promise.resolve();
    },
    async failOne(error: unknown) {
      const next = pending.shift();
      next?.reject(error);
      await Promise.resolve();
      await Promise.resolve();
    },
  };
}

describe("🎚️ continuous gesture lane", () => {
  it("costs at most two live round trips however many values a gesture produces", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver) });
    for (let value = 0; value < 60; value += 1) lane.offer(value);
    expect(receiver.sent).toEqual([{ value: 0, phase: "live" }]);
    expect(lane.inFlight()).toBe(true);
    expect(lane.owed()).toBe(59);
    expect(receiver.pending).toHaveLength(1);
  });

  it("sends the LATEST offered value once the round trip lands, never an intermediate one", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver) });
    lane.offer(1);
    lane.offer(2);
    lane.offer(3);
    await receiver.settleOne();
    expect(receiver.sent.map((entry) => entry.value)).toEqual([1, 3]);
    lane.offer(4);
    lane.offer(5);
    await receiver.settleOne();
    expect(receiver.sent.map((entry) => entry.value)).toEqual([1, 3, 5]);
  });

  it("always sends the release, as commit, even when that value already went out live", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver) });
    lane.offer(7);
    await receiver.settleOne();
    lane.commit(7);
    await receiver.settleOne();
    expect(receiver.sent).toEqual([
      { value: 7, phase: "live" },
      { value: 7, phase: "commit" },
    ]);
  });

  it("commits the owed value when the release names none", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver) });
    lane.offer(1);
    lane.offer(2);
    lane.commit();
    await receiver.settleOne();
    expect(receiver.sent).toEqual([
      { value: 1, phase: "live" },
      { value: 2, phase: "commit" },
    ]);
    expect(lane.owed()).toBeNull();
  });

  it("ends a whole gesture on the released value and spends one send per landed round trip", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver) });
    for (let value = 1; value <= 20; value += 1) {
      lane.offer(value);
      if (value % 5 === 0) await receiver.settleOne();
    }
    lane.commit();
    await receiver.settleOne();
    await receiver.settleOne();
    const last = receiver.sent.at(-1);
    expect(last).toEqual({ value: 20, phase: "commit" });
    expect(receiver.sent.length).toBeLessThanOrEqual(6);
    expect(receiver.sent.filter((entry) => entry.phase === "commit")).toHaveLength(1);
  });

  it("frees the lane when a round trip is refused, and still sends what is owed", async () => {
    const receiver = manualReceiver();
    const faults: unknown[] = [];
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver), onFault: (error) => faults.push(error) });
    lane.offer(1);
    lane.offer(2);
    await receiver.failOne(new Error("refused"));
    expect(faults).toHaveLength(1);
    expect(receiver.sent.map((entry) => entry.value)).toEqual([1, 2]);
    expect(lane.inFlight()).toBe(true);
  });

  it("degenerates to send-everything only for a sink that answers void", () => {
    const sent: Sent[] = [];
    const lane = createContinuousGestureLane<number>({
      send: (value, phase) => {
        sent.push({ value, phase });
      },
    });
    lane.offer(1);
    lane.offer(2);
    lane.commit(3);
    expect(sent.map((entry) => entry.value)).toEqual([1, 2, 3]);
    expect(lane.inFlight()).toBe(false);
  });
});
