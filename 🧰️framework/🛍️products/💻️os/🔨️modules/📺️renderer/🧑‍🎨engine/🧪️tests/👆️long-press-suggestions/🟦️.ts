import { describe, expect, it } from "vitest";
import { createLongPressV1 } from "../../🧱️elements/👆️long-press/🟦️.ts";
import { WORLD3D_SUGGESTIONS_LABELS_V1, world3dSuggestionsGestureTarget, world3dSuggestionsKeyChord } from "../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";

function rig() {
  const timers: { live: boolean; callback: () => void }[] = [];
  const fired: [number, number][] = [];
  const press = createLongPressV1({ holdMs: 500, slopPx: 10, setTimer: (callback) => { const timer = { live: true, callback }; timers.push(timer); return () => { timer.live = false; }; }, onLongPress: (x, y) => { fired.push([x, y]); } });
  return { press, fired, elapse: () => { for (const timer of timers) if (timer.live) { timer.live = false; timer.callback(); } } };
}

describe("long press", () => {
  it("fires once at the pressed point when held", () => {
    const r = rig();
    r.press.down(5, 6);
    r.elapse();
    r.elapse();
    expect(r.fired).toEqual([[5, 6]]);
  });
  it("is disarmed by movement, release or cancel", () => {
    for (const disarm of [(p: ReturnType<typeof rig>["press"]) => p.move(5, 40), (p: ReturnType<typeof rig>["press"]) => p.up(), (p: ReturnType<typeof rig>["press"]) => p.cancel()]) {
      const r = rig();
      r.press.down(5, 5);
      disarm(r.press);
      r.elapse();
      expect(r.fired).toEqual([]);
    }
  });
});

describe("mesh suggestions alternatives", () => {
  const key = (k: string, extra: Partial<{ shiftKey: boolean; ctrlKey: boolean; metaKey: boolean; altKey: boolean }> = {}) => ({ key: k, shiftKey: false, ctrlKey: false, metaKey: false, altKey: false, ...extra });
  it("recognises the context-menu key and Shift+F10 only", () => {
    expect(world3dSuggestionsKeyChord(key("ContextMenu"))).toBe(true);
    expect(world3dSuggestionsKeyChord(key("F10", { shiftKey: true }))).toBe(true);
    expect(world3dSuggestionsKeyChord(key("F10"))).toBe(false);
    expect(world3dSuggestionsKeyChord(key("ContextMenu", { ctrlKey: true }))).toBe(false);
  });
  it("targets the focused marker and nothing otherwise", () => {
    expect(world3dSuggestionsGestureTarget("v:1")).toBe("v:1");
    expect(world3dSuggestionsGestureTarget(null)).toBeNull();
    expect(world3dSuggestionsGestureTarget("")).toBeNull();
  });
  it("labels both routes in en and de", () => {
    for (const row of Object.values(WORLD3D_SUGGESTIONS_LABELS_V1)) expect(row.en).not.toBe(row.de);
  });
});
