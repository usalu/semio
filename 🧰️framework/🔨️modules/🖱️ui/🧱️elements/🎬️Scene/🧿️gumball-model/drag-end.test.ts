import { describe, expect, test } from "vitest";
import { GUMBALL_DRAG_END_EVENTS, gumballDragEndPointerState, gumballHighlightKind } from "./🟦️.ts";

describe("gumball axis guide after a drag", () => {
  test("a dragged handle's guide is drawn only while dragging, and nothing is highlighted once the drag ends", () => {
    expect(gumballHighlightKind("moveX", "moveX")).toBe("moveX");
    expect(gumballHighlightKind("scaleY", "moveX")).toBe("scaleY");
    expect(gumballHighlightKind(null, "moveX")).toBe("moveX");
    const settled = gumballDragEndPointerState();
    expect(gumballHighlightKind(settled.active, settled.hovered)).toBeNull();
  });

  test("a cancelled pointer or a blurred window ends the drag like a release", () => {
    expect([...GUMBALL_DRAG_END_EVENTS].sort()).toEqual(["blur", "pointercancel", "pointerup"]);
  });
});
