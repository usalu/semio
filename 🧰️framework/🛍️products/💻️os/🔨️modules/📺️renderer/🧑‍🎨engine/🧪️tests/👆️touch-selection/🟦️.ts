import { describe, expect, it } from "vitest";
import { createTouchSelectionV1, TOUCH_DOUBLE_TAP_MS_V1, TOUCH_LONG_PRESS_MS_V1 } from "../../🧱️elements/✏️TextEditor/👆️touch-selection/🟦️.ts";

function rig() {
  let clock = 0;
  const timers: { at: number; callback: () => void; live: boolean }[] = [];
  const picks: [number, number][] = [];
  const recognizer = createTouchSelectionV1({
    now: () => clock,
    setTimer: (callback, ms) => { const timer = { at: clock + ms, callback, live: true }; timers.push(timer); return () => { timer.live = false; }; },
    selectWord: (x, y) => { picks.push([x, y]); },
  });
  const advance = (ms: number) => { clock += ms; for (const timer of timers) if (timer.live && timer.at <= clock) { timer.live = false; timer.callback(); } };
  return { recognizer, advance, picks };
}

describe("touch word selection", () => {
  it("selects the word under a long press", () => {
    const r = rig();
    r.recognizer.down(40, 20);
    r.advance(TOUCH_LONG_PRESS_MS_V1);
    expect(r.picks).toEqual([[40, 20]]);
    r.recognizer.up();
    expect(r.picks).toHaveLength(1);
  });

  it("does not select when the finger moves beyond the slop (a scroll) or the press is released early", () => {
    const r = rig();
    r.recognizer.down(40, 20);
    r.recognizer.move(40, 60);
    r.advance(TOUCH_LONG_PRESS_MS_V1 + 10);
    expect(r.picks).toEqual([]);
    r.recognizer.down(10, 10);
    r.advance(100);
    r.recognizer.up();
    r.advance(TOUCH_LONG_PRESS_MS_V1);
    expect(r.picks).toEqual([]);
  });

  it("selects on a second tap close in time and place, not after a pause or elsewhere", () => {
    const r = rig();
    r.recognizer.down(10, 10); r.advance(40); r.recognizer.up();
    r.advance(100);
    r.recognizer.down(14, 12); r.advance(40); r.recognizer.up();
    expect(r.picks).toEqual([[14, 12]]);
    r.advance(TOUCH_DOUBLE_TAP_MS_V1 + 50);
    r.recognizer.down(14, 12); r.advance(40); r.recognizer.up();
    r.advance(100);
    r.recognizer.down(200, 12); r.advance(40); r.recognizer.up();
    expect(r.picks).toHaveLength(1);
  });

  it("cancel disarms a pending long press", () => {
    const r = rig();
    r.recognizer.down(1, 1);
    r.recognizer.cancel();
    r.advance(TOUCH_LONG_PRESS_MS_V1 * 2);
    expect(r.picks).toEqual([]);
  });
});
