/** 📐️ Floating panel fit at the Steam Deck screen (1280x800) and the compact 800x800 width. */
import { describe, expect, it } from "vitest";
import { PANEL_MIN_EDITOR_WIDTH_PX, panelFitV1 } from "../../📐️panel-fit/🟦️.ts";

describe("panel fit", () => {
  it("keeps the requested width at 1280x800 and clamps an oversized panel to the viewport fraction", () => {
    expect(panelFitV1(1280, 300)).toEqual({ widthPx: 300, collapsed: false });
    expect(panelFitV1(1280, 600)).toEqual({ widthPx: 576, collapsed: false });
  });

  it("clamps at 800x800 and still leaves a workable editor column", () => {
    const fit = panelFitV1(800, 600);
    expect(fit.widthPx).toBe(360);
    expect(800 - fit.widthPx).toBeGreaterThanOrEqual(PANEL_MIN_EDITOR_WIDTH_PX);
    expect(fit.collapsed).toBe(false);
  });

  it("folds when the editor column would fall below its minimum", () => {
    expect(panelFitV1(700, 300).collapsed).toBe(true);
    expect(panelFitV1(420, 200).collapsed).toBe(true);
  });
});
