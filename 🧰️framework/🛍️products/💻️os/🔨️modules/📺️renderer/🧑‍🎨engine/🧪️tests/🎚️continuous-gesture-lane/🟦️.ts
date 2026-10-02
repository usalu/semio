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
 *      live — the receiver needs it to commit its press as ONE transaction, which is what makes a
 *      drag ONE history entry instead of N;
 *   4. a refused round trip frees the lane instead of wedging the gesture;
 *   5. an open press can be cancelled (blur, capture lost, unmount): the cancel is sent after the round
 *      trip in flight and before anything offered later, and the cancelled press's release is ignored —
 *      the receiver drops the press with zero trace (scrub protocol, `📓️api-scrub-machine.md`).
 *
 * The Rust twin over the same table is `✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit`'s
 * `every_continuous_gesture_row_costs_one_edit_and_ends_on_the_released_value`.
 */
import { afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { continuousPressIdentity, createContinuousGestureLane, type ContinuousGestureAbortReason, type ContinuousGesturePhase } from "@semio-tech/framework";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { interpretUiNode, UiDocumentStore, type UiInterpreterContext } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

type Sent = { readonly value: number; readonly phase: ContinuousGesturePhase };

/** 🔌️ A receiver whose round trips are resolved by the test, one at a time, so "in flight" is a fact
 * the law controls rather than a timing accident. */
function manualReceiver() {
  const sent: Sent[] = [];
  const aborts: ContinuousGestureAbortReason[] = [];
  const pending: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  return {
    sent,
    aborts,
    pending,
    send(value: number, phase: ContinuousGesturePhase) {
      sent.push({ value, phase });
      return new Promise<void>((resolve, reject) => pending.push({ resolve, reject }));
    },
    abort(reason: ContinuousGestureAbortReason) {
      aborts.push(reason);
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

  it("settles at once for a sink that answers anything but a promise, never wedging the gesture", () => {
    const sent: Sent[] = [];
    const faults: unknown[] = [];
    const sink = (value: number, phase: ContinuousGesturePhase): unknown => sent.push({ value, phase });
    const lane = createContinuousGestureLane<number>({ send: sink as (value: number, phase: ContinuousGesturePhase) => void, onFault: (error) => faults.push(error) });
    lane.offer(1);
    lane.offer(2);
    lane.commit(3);
    expect(sent).toEqual([
      { value: 1, phase: "live" },
      { value: 2, phase: "live" },
      { value: 3, phase: "commit" },
    ]);
    expect([lane.inFlight(), faults]).toEqual([false, []]);
  });

  it("mints a distinct press identity for every press of one control, even within one millisecond", () => {
    const identities = Array.from({ length: 64 }, () => continuousPressIdentity("panel.width"));
    expect(new Set(identities).size).toBe(identities.length);
    for (const identity of identities) expect(identity).toMatch(/^panel\.width:\d+:\d+$/);
  });
});

describe("🎚️ continuous gesture lane: host cancel", () => {
  it("cancels an open press after the round trip in flight, drops the owed value and ignores the press's release", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver), abort: receiver.abort.bind(receiver) });
    lane.offer(1);
    lane.offer(2);
    lane.abort("blur");
    expect(receiver.aborts).toEqual([]);
    await receiver.settleOne();
    expect(receiver.aborts).toEqual(["blur"]);
    expect(receiver.sent).toEqual([{ value: 1, phase: "live" }]);
    await receiver.settleOne();
    lane.commit(2);
    expect(receiver.sent).toEqual([{ value: 1, phase: "live" }]);
    expect(lane.open()).toBe(false);
  });

  it("sends the cancel before a value offered after it", async () => {
    const receiver = manualReceiver();
    const lane = createContinuousGestureLane<number>({ send: receiver.send.bind(receiver), abort: receiver.abort.bind(receiver) });
    lane.offer(1);
    lane.abort("captureLost");
    lane.offer(5);
    await receiver.settleOne();
    expect(receiver.aborts).toEqual(["captureLost"]);
    await receiver.settleOne();
    expect(receiver.sent).toEqual([
      { value: 1, phase: "live" },
      { value: 5, phase: "live" },
    ]);
  });

  it("cancels nothing and releases nothing once the press is closed", () => {
    const sent: Sent[] = [];
    const aborts: ContinuousGestureAbortReason[] = [];
    const lane = createContinuousGestureLane<number>({ send: (value, phase) => void sent.push({ value, phase }), abort: (reason) => void aborts.push(reason) });
    lane.abort("retired");
    lane.commit();
    lane.offer(3);
    lane.commit();
    lane.abort("blur");
    lane.commit();
    expect(sent).toEqual([
      { value: 3, phase: "live" },
      { value: 3, phase: "commit" },
    ]);
    expect(aborts).toEqual([]);
  });
});

//#region 🎚️InterpretedControls
type Intent = { readonly input: Record<string, unknown> | null };

function interpreted(component: Record<string, unknown>, key: string) {
  const store = new UiDocumentStore("scrub-test");
  store.loadSnapshot({
    surface: "scrub-test",
    revision: 0,
    root: 0,
    layoutEpoch: 0n,
    nodes: [
      {
        id: 0,
        key,
        component,
        layout: { kind: "leaf", width: "hug", height: "hug" },
        style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
        activity: "idle",
        disabled: false,
        transition: null,
        accessibility: { label: "Opacity", description: null, live: "off", shortcut: null, hidden: false },
        bindings: [{ trigger: "change", action: { scope: "scrub-test", name: "setOpacity", version: 1 }, args: { id: "layer-1" }, capability: null }],
        menu: null,
        children: [],
      },
    ],
  } as never);
  const intents: Intent[] = [];
  const context = { store, onAction: () => {}, onIntent: (intent: Intent) => void intents.push(intent) } as unknown as UiInterpreterContext;
  const view = render(createElement("div", null, interpretUiNode(store, context)));
  return { view, intents };
}

describe("🎚️ interpreted continuous controls speak the scrub protocol", () => {
  afterEach(() => cleanup());

  it("a keyboard step on a slider is one press: a tick and its release share one gesture", () => {
    const { view, intents } = interpreted({ type: "slider", value: 5, min: 0, max: 10, step: 1 }, "inspector.opacity");
    const thumb = view.container.querySelector<HTMLElement>('[data-slot="slider-thumb"]')!;
    fireEvent.keyDown(thumb, { key: "ArrowRight" });
    fireEvent.keyUp(thumb, { key: "ArrowRight" });
    expect(intents.map((intent) => intent.input)).toEqual([
      { value: 6, gesture: intents[0]!.input!.gesture, commit: false },
      { value: 6, gesture: intents[0]!.input!.gesture, commit: true },
    ]);
    expect(String(intents[0]!.input!.gesture), "a press is named by its control, its start and a page-wide serial").toMatch(/^inspector\.opacity:\d+:\d+$/);
  });

  it("a slider unmounted mid-press cancels the press as retired, with no value", () => {
    const { view, intents } = interpreted({ type: "slider", value: 5, min: 0, max: 10, step: 1 }, "inspector.opacity");
    const thumb = view.container.querySelector<HTMLElement>('[data-slot="slider-thumb"]')!;
    fireEvent.keyDown(thumb, { key: "ArrowRight" });
    view.unmount();
    expect(intents.map((intent) => intent.input)).toEqual([
      { value: 6, gesture: intents[0]!.input!.gesture, commit: false },
      { gesture: intents[0]!.input!.gesture, abort: "retired" },
    ]);
  });

  it("a slider that loses focus mid-press cancels it as blur", () => {
    const { view, intents } = interpreted({ type: "slider", value: 5, min: 0, max: 10, step: 1 }, "inspector.opacity");
    const slider = view.container.querySelector<HTMLElement>('[data-slot="slider"]')!;
    fireEvent.pointerDown(slider, { pointerId: 1, clientX: 0 });
    expect(intents).toHaveLength(1);
    fireEvent.blur(slider);
    expect(intents.at(-1)!.input).toEqual({ gesture: intents[0]!.input!.gesture, abort: "blur" });
  });

  it("a stepper click is one press: the tick and the button's release", () => {
    const { view, intents } = interpreted({ type: "numberStepper", value: 3, step: 1, uniform: true, min: null, max: null }, "inspector.count");
    const plus = view.container.querySelector<HTMLElement>('[data-slot="stepper-plus"]')!;
    fireEvent.mouseDown(plus);
    fireEvent.mouseUp(plus);
    expect(intents.map((intent) => intent.input)).toEqual([
      { value: 4, gesture: intents[0]!.input!.gesture, commit: false },
      { value: 4, gesture: intents[0]!.input!.gesture, commit: true },
    ]);
  });
});
//#endregion 🎚️InterpretedControls
