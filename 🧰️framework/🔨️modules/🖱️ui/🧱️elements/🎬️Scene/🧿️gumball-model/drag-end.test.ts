import { describe, expect, test } from "vitest";
import { gumballDragEndPointerState, gumballHighlightKind } from "./🟦️.ts";

describe("gumball axis guide after a drag", () => {
  test("a dragged handle's guide is drawn only while dragging, and nothing is highlighted once the drag ends", () => {
    expect(gumballHighlightKind("moveX", "moveX")).toBe("moveX");
    expect(gumballHighlightKind("scaleY", "moveX")).toBe("scaleY");
    expect(gumballHighlightKind(null, "moveX")).toBe("moveX");
    const settled = gumballDragEndPointerState();
    expect(gumballHighlightKind(settled.active, settled.hovered)).toBeNull();
  });
});
