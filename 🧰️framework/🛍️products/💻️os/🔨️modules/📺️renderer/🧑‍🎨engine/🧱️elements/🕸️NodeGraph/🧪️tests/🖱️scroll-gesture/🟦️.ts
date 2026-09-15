/**
 * 🖱️ The Flow node-graph SCROLL contract: the camera is board state, so a wheel or drag gesture costs
 * the plugin nothing while it runs and exactly one publication when it settles, and every repaint the
 * gesture asks for is coalesced onto the demand scheduler instead of issued per input event.
 *
 * The defect this pins: `onWheel` read `viewport()` and dispatched `nodeGraphViewport` on every notch,
 * and `onPointerMove` resolved a hover target on every move — so 30 wheel ticks cost 5–9 guest hops
 * and a 1 s drag-pan cost 16–18, each one a `performInvocation` → `refreshUi` → React commit. The
 * board painted ONCE in the whole gesture and the median tick-to-paint was 653–1 110 ms, which is
 * what the user reported as "scrolling in the flow takes seconds to render"
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️flow-scroll-render-perf-2026-09-15.md` §2).
 *
 * Every rule here is driven through the REAL exported units over a virtual clock — no browser, no
 * wasm — so a regression fails here rather than only in the probe.
 *
 * @see `🐍️flow-scroll-render-perf-probe.mjs` — the live twin, which measures the same rules on :6022
 */
import { describe, expect, it } from "vitest";
import { FLOW_CAMERA_GESTURE_SETTLE_MS, createFlowCameraGesture, flowGestureIsCameraPan, type FlowCameraGesturePorts } from "../../🟦️.tsx";

//#region 🕰️VirtualClock
type ScheduledRun = { readonly at: number; readonly run: () => void; cancelled: boolean };

/** 🕰️ A clock the law advances by hand, plus the ledger of everything the gesture asked its host to
 * do — so "no guest hop during the gesture" is a count, not a reading of the source. */
function gestureHarness() {
  const ledger = { begins: [] as string[], ends: [] as string[], invalidations: 0, publications: 0 };
  const scheduled: ScheduledRun[] = [];
  let nowMs = 0;
  const ports: FlowCameraGesturePorts = {
    begin: (reason) => ledger.begins.push(reason),
    end: (reason) => ledger.ends.push(reason),
    invalidate: () => { ledger.invalidations += 1; },
    publish: () => { ledger.publications += 1; },
    schedule: (run, delayMs) => {
      const entry: ScheduledRun = { at: nowMs + delayMs, run, cancelled: false };
      scheduled.push(entry);
      return entry;
    },
    cancel: (handle) => { (handle as ScheduledRun).cancelled = true; },
  };
  const advance = (byMs: number) => {
    nowMs += byMs;
    for (const entry of [...scheduled]) {
      if (entry.cancelled || entry.at > nowMs) continue;
      entry.cancelled = true;
      entry.run();
    }
  };
  return { ledger, ports, advance, gesture: createFlowCameraGesture(ports) };
}
//#endregion 🕰️VirtualClock

describe("flow node-graph scroll gesture", () => {
  it("costs the plugin nothing while the wheel is turning", () => {
    const harness = gestureHarness();
    for (let tick = 0; tick < 30; tick += 1) {
      harness.gesture.tick();
      harness.advance(16);
    }
    expect(harness.ledger.publications).toBe(0);
  });

  it("publishes the settled camera exactly once, after the ticks stop", () => {
    const harness = gestureHarness();
    for (let tick = 0; tick < 30; tick += 1) {
      harness.gesture.tick();
      harness.advance(16);
    }
    harness.advance(FLOW_CAMERA_GESTURE_SETTLE_MS + 1);
    expect(harness.ledger.publications).toBe(1);
  });

  it("opens ONE gesture for a whole scroll and closes it once", () => {
    const harness = gestureHarness();
    for (let tick = 0; tick < 30; tick += 1) {
      harness.gesture.tick();
      harness.advance(16);
    }
    expect(harness.ledger.begins).toEqual(["wheel"]);
    harness.advance(FLOW_CAMERA_GESTURE_SETTLE_MS + 1);
    expect(harness.ledger.ends).toEqual(["wheel"]);
  });

  it("asks the demand scheduler for a frame per tick and never paints from the event itself", () => {
    const harness = gestureHarness();
    for (let tick = 0; tick < 30; tick += 1) {
      harness.gesture.tick();
      harness.advance(16);
    }
    expect(harness.ledger.invalidations).toBe(30);
  });

  it("counts two scrolls separated by a pause as two gestures with two publications", () => {
    const harness = gestureHarness();
    for (let tick = 0; tick < 5; tick += 1) {
      harness.gesture.tick();
      harness.advance(16);
    }
    harness.advance(FLOW_CAMERA_GESTURE_SETTLE_MS + 1);
    for (let tick = 0; tick < 5; tick += 1) {
      harness.gesture.tick();
      harness.advance(16);
    }
    harness.advance(FLOW_CAMERA_GESTURE_SETTLE_MS + 1);
    expect([harness.ledger.begins.length, harness.ledger.ends.length, harness.ledger.publications]).toEqual([2, 2, 2]);
  });

  it("is active for the whole scroll and settled after it", () => {
    const harness = gestureHarness();
    harness.gesture.tick();
    harness.advance(FLOW_CAMERA_GESTURE_SETTLE_MS - 1);
    expect(harness.gesture.active()).toBe(true);
    harness.advance(2);
    expect(harness.gesture.active()).toBe(false);
  });

  it("publishes nothing when a gesture is disposed mid-scroll", () => {
    const harness = gestureHarness();
    harness.gesture.tick();
    harness.gesture.dispose();
    harness.advance(FLOW_CAMERA_GESTURE_SETTLE_MS * 4);
    expect(harness.ledger.publications).toBe(0);
  });

  it("reads a middle-button press as a camera pan and a left-button press as a content gesture", () => {
    expect([flowGestureIsCameraPan(1, 4), flowGestureIsCameraPan(-1, 4), flowGestureIsCameraPan(0, 1), flowGestureIsCameraPan(2, 2)]).toEqual([true, true, false, false]);
  });
});
