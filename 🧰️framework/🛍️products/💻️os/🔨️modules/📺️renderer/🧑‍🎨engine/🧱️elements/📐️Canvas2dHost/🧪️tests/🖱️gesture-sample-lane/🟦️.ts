/** 🖱️ `Canvas2dHost`'s pointer path through the gesture sample lane (ticket 26/09/16/INPUT-CAUSALITY-LEDGER,
 * design §1 L4 / §2 D), stated as laws over `JsonLayersCanvasSession` with a receiver whose round trips the
 * test settles by hand — "in flight" is a fact the law controls, never a timing accident.
 *
 * What this replaced: one `canvasPointerMove` guest round trip per DOM `pointermove` (a one-second marquee
 * at 60 Hz drained for 1–3 s after mouseup), and `pointerleave` forging a `canvasPointerUp`.
 *   1. N moves while the first dispatch is unresolved cost ONE `canvasPointerMove` carrying all N samples
 *      in order, with `x`/`y` the last sample;
 *   2. down → moves → up arrive as `canvasPointerDown`, `canvasPointerMove`*, `canvasPointerUp { cancelled:
 *      false }`, in that order, never interleaved;
 *   3. a cancel is `canvasPointerUp { cancelled: true }`, and outside a gesture it is a no-op;
 *   4. the lane is armed by the promise `dispatch` returns — a rejected round trip frees it and the owed
 *      items still go out.
 */
import { describe, expect, it } from "vitest";
import { JsonLayersCanvasSession, createCanvasPointerGestureLane, type CanvasPointerDispatch } from "../../🟦️.tsx";

type Sent = { readonly action: string; readonly args: Record<string, unknown> };

/** 🔌️ A receiver that records every dispatch and hands the test one deferred promise per send. */
function manualReceiver() {
  const sent: Sent[] = [];
  const pending: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  const dispatch: CanvasPointerDispatch = (action, args) => {
    sent.push({ action, args: args ?? {} });
    return new Promise<void>((resolve, reject) => pending.push({ resolve, reject }));
  };
  const flush = async (): Promise<void> => {
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
  };
  return {
    sent,
    pending,
    dispatch,
    async settleOne(): Promise<void> {
      const next = pending.shift();
      if (!next) throw new Error("nothing in flight");
      next.resolve();
      await flush();
    },
    async rejectOne(): Promise<void> {
      const next = pending.shift();
      if (!next) throw new Error("nothing in flight");
      next.reject(new Error("refused"));
      await flush();
    },
    async settleAll(): Promise<void> {
      while (pending.length > 0) await this.settleOne();
    },
  };
}

function session(dispatch: CanvasPointerDispatch): JsonLayersCanvasSession {
  const live = new JsonLayersCanvasSession(() => "[]", { x: 0, y: 0, zoom: 1 }, () => undefined, dispatch);
  live.setSize(640, 480, 1);
  return live;
}

describe("🖱️ Canvas2dHost gesture sample lane", () => {
  it("batches 40 moves issued while the press is unresolved into ONE canvasPointerMove carrying every sample in order", async () => {
    const receiver = manualReceiver();
    const live = session(receiver.dispatch);
    live.pointerDown(10, 10, 0, false, { shift: false, ctrl: false, meta: false, alt: false });
    expect(receiver.sent.map((entry) => entry.action)).toEqual(["canvasPointerDown"]);
    for (let index = 1; index <= 40; index += 1) live.pointerMove(10 + index, 10 + 2 * index);
    // 🔒️ The press is still out: nothing else may have been sent.
    expect(receiver.sent.length).toBe(1);
    await receiver.settleOne();
    expect(receiver.sent.length).toBe(2);
    const move = receiver.sent[1]!;
    expect(move.action).toBe("canvasPointerMove");
    const samples = move.args.samples as readonly (readonly [number, number])[];
    expect(samples.length).toBe(40);
    expect(samples.map(([x]) => x)).toEqual(Array.from({ length: 40 }, (_, index) => 11 + index));
    expect(samples[39]).toEqual([50, 90]);
    expect(move.args.x).toBe(50);
    expect(move.args.y).toBe(90);
    expect(move.args.width).toBe(640);
    expect(move.args.height).toBe(480);
    await receiver.settleAll();
    expect(receiver.sent.length).toBe(2);
  });

  it("delivers down, moves, up in issue order and never interleaves the release with owed samples", async () => {
    const receiver = manualReceiver();
    const live = session(receiver.dispatch);
    live.pointerDown(0, 0, 0, false, { shift: true, ctrl: false, meta: false, alt: false });
    live.pointerMove(1, 1);
    live.pointerMove(2, 2);
    live.pointerUp(3, 3, { shift: true, ctrl: false, meta: false, alt: false });
    // 🖱️ Moves offered AFTER the release belong to the next hover batch, behind the release.
    live.pointerMove(4, 4);
    await receiver.settleAll();
    expect(receiver.sent.map((entry) => entry.action)).toEqual(["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasPointerMove"]);
    expect(receiver.sent[0]!.args).toMatchObject({ x: 0, y: 0, button: 0, shift: true, ctrl: false, meta: false, alt: false, width: 640, height: 480 });
    expect(receiver.sent[1]!.args.samples).toEqual([
      [1, 1],
      [2, 2],
    ]);
    expect(receiver.sent[2]!.args).toMatchObject({ x: 3, y: 3, shift: true, cancelled: false, width: 640, height: 480 });
    expect(receiver.sent[3]!.args).toMatchObject({ x: 4, y: 4, samples: [[4, 4]] });
  });

  it("turns a cancel mid-gesture into canvasPointerUp { cancelled: true } at the last known sample, and ignores one outside a gesture", async () => {
    const receiver = manualReceiver();
    const live = session(receiver.dispatch);
    // 🚫️ pointerleave with no gesture open (the old forged release): nothing.
    live.pointerMove(5, 5);
    live.pointerCancel();
    await receiver.settleAll();
    expect(receiver.sent.map((entry) => entry.action)).toEqual(["canvasPointerMove"]);
    live.pointerDown(20, 20, 0, false, { shift: false, ctrl: false, meta: false, alt: false });
    live.pointerMove(30, 40);
    live.pointerCancel();
    await receiver.settleAll();
    expect(receiver.sent.map((entry) => entry.action)).toEqual(["canvasPointerMove", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp"]);
    expect(receiver.sent[3]!.args).toMatchObject({ x: 30, y: 40, cancelled: true, shift: false, ctrl: false, meta: false, alt: false, width: 640, height: 480 });
    // 🔁️ A second cancel after the gesture closed is a no-op, and a fresh press opens a new gesture.
    live.pointerCancel();
    live.pointerDown(1, 1, 0, false, { shift: false, ctrl: false, meta: false, alt: false });
    await receiver.settleAll();
    expect(receiver.sent.length).toBe(5);
    expect(receiver.sent[4]!.action).toBe("canvasPointerDown");
  });

  it("frees the lane on a refused round trip and still sends what was owed", async () => {
    const receiver = manualReceiver();
    const warned: string[] = [];
    const original = console.warn;
    console.warn = (message: unknown) => {
      warned.push(String(message));
    };
    try {
      const lane = createCanvasPointerGestureLane(receiver.dispatch, () => ({ width: 640, height: 480 }));
      lane.begin(1, [0, 0], { button: 0, shift: false, ctrl: false, meta: false, alt: false, width: 640, height: 480 });
      lane.offer([1, 1]);
      lane.end([2, 2], { shift: false, ctrl: false, meta: false, alt: false, width: 640, height: 480 });
      expect(lane.inFlight()).toBe(true);
      await receiver.rejectOne();
      expect(warned.length).toBe(1);
      expect(warned[0]).toContain("canvas2d gesture lane");
      await receiver.settleAll();
      expect(receiver.sent.map((entry) => entry.action)).toEqual(["canvasPointerDown", "canvasPointerMove", "canvasPointerUp"]);
      expect(lane.inFlight()).toBe(false);
      expect(lane.owed()).toBe(0);
    } finally {
      console.warn = original;
    }
  });

  it("keeps the pan branch off the lane: a middle-button drag moves the camera locally and dispatches nothing", async () => {
    const receiver = manualReceiver();
    const cameras: { x: number; y: number; zoom: number }[] = [];
    const live = new JsonLayersCanvasSession(() => "[]", { x: 0, y: 0, zoom: 1 }, (camera) => cameras.push(camera), receiver.dispatch);
    live.setSize(640, 480, 1);
    live.pointerDown(100, 100, 1, false, { shift: false, ctrl: false, meta: false, alt: false });
    live.pointerMove(110, 120);
    live.pointerCancel();
    live.pointerMove(200, 200);
    await receiver.settleAll();
    expect(cameras.length).toBe(1);
    expect(cameras[0]).toMatchObject({ x: -10, y: -20 });
    expect(receiver.sent.map((entry) => entry.action)).toEqual(["canvasPointerMove"]);
    expect(receiver.sent[0]!.args).toMatchObject({ x: 200, y: 200 });
  });
});
