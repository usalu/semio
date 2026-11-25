import { describe, expect, test } from "bun:test";
import { world3dLeftPressOrbits, world3dLeftReleaseIsDrag } from "../../🔗️instance-selection/🟦️.ts";

const plain = { shiftKey: false, ctrlKey: false, metaKey: false };

describe("world left press", () => {
  test("a plain press orbits with no utility and with the transform gumball armed", () => {
    expect(world3dLeftPressOrbits(plain, {})).toBe(true);
    expect(world3dLeftPressOrbits(plain, { activeUtility: null })).toBe(true);
    expect(world3dLeftPressOrbits(plain, { activeUtility: "transform" })).toBe(true);
  });

  test("a merge modifier or a utility that owns the left button starts a marquee instead", () => {
    for (const modifier of ["shiftKey", "ctrlKey", "metaKey"] as const) expect(world3dLeftPressOrbits({ ...plain, [modifier]: true }, {})).toBe(false);
    for (const activeUtility of ["select", "fill", "worldRelocate", "brush", "volumeBrush", "surfaceBrush"]) expect(world3dLeftPressOrbits(plain, { activeUtility })).toBe(false);
    expect(world3dLeftPressOrbits(plain, { paintMode: true })).toBe(false);
    expect(world3dLeftPressOrbits(plain, { engagementSessionActive: true })).toBe(false);
  });

  test("a release more than the drag threshold away is an orbit drag, not a pick", () => {
    expect(world3dLeftReleaseIsDrag(10, 10, 12, 11)).toBe(false);
    expect(world3dLeftReleaseIsDrag(10, 10, 20, 10)).toBe(true);
  });
});
