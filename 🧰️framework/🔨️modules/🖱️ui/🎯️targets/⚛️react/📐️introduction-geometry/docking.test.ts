/** 🧲️ The introduction info box docks onto the border lines beneath it. */
import { describe, expect, it } from "vitest";
import { clearIntroductionInfoBoxNearMisses, dockIntroductionInfoBox } from "./🟦️.ts";

describe("dockIntroductionInfoBox", () => {
  const box = { width: 100, height: 50 };
  const lines = { xs: [0, 360, 720], ys: [29, 424.5, 871] };

  it("moves the nearest box edge onto a line within reach, per axis, keeping the size when no second line is near", () => {
    expect(dockIntroductionInfoBox({ top: 380, left: 340 }, box, lines, 40)).toEqual({ top: 374.5, left: 360, width: 100, height: 50 });
  });

  it("docks the far edge when it is the nearer one", () => {
    expect(dockIntroductionInfoBox({ top: 380, left: 270 }, box, lines, 40)).toEqual({ top: 374.5, left: 260, width: 100, height: 50 });
  });

  it("stretches the opposite edge onto a second line within reach so both seams are shared", () => {
    expect(dockIntroductionInfoBox({ top: 40, left: 20 }, { width: 340, height: 60 }, lines, 40)).toEqual({ top: 29, left: 0, width: 360, height: 60 });
  });

  it("prefers an inner line over a nearer outer one when shrinking onto the second seam", () => {
    expect(dockIntroductionInfoBox({ top: 200, left: 28 }, { width: 343, height: 50 }, { xs: [28, 347, 375], ys: [] }, 40)).toEqual({ top: 200, left: 28, width: 319, height: 50 });
  });

  it("leaves an axis alone when no line is within reach", () => {
    expect(dockIntroductionInfoBox({ top: 200, left: 200 }, box, lines, 10)).toEqual({ top: 200, left: 200, width: 100, height: 50 });
  });
});

describe("clearIntroductionInfoBoxNearMisses", () => {
  it("slides the box until no own line is a near miss of a line beneath, keeping coincident lines", () => {
    const moved = clearIntroductionInfoBoxNearMisses({ top: 0, left: 700 }, { xs: [0, 19, 38], ys: [] }, { xs: [720], ys: [] }, 40, 120);
    expect(moved.left).toBe(642);
    expect(moved.top).toBe(0);
  });

  it("does not move a box whose lines already coincide with or clear the lines beneath", () => {
    expect(clearIntroductionInfoBoxNearMisses({ top: 5, left: 720 }, { xs: [0, 100], ys: [0] }, { xs: [720], ys: [5] }, 40, 120)).toEqual({ top: 5, left: 720 });
  });
});
