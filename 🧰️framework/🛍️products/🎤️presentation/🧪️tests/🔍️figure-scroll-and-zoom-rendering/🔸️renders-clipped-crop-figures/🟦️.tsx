/** 🔸️ The renders clipped crop figures tests of the PresentationDeck suite. */
import type { Presentation } from "../../../🎯️targets/⚛️react/🟦️.tsx";
import { expect, it } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { figureCropBackgroundPosition, figureCropBackgroundSize, figureCropBackgroundVars, figureScrollOverlayThumbMetrics, mosaicWindowedCoverVars, mountPresentation } from "../../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource } from "../../🧰️presentation-deck-helpers/🟦️.tsx";

/** 🔹 Registers the renders clipped crop figures tests of the PresentationDeck suite. */
export function registerRendersClippedCropFiguresTests(): void {
  it("renders clipped crop figures when scroll is disabled", () => {
    const container = document.createElement("div");
    const deck: Presentation = {
      id: "scroll-off",
      name: "scroll-off",
      chapters: [
        {
          id: "c",
          sequences: [
            {
              id: "s",
              thoughts: [
                {
                  id: "t",
                  participants: [{ id: "figure" }],
                  embodiments: [
                    {
                      kind: "figure",
                      id: "figure--img",
                      src: "/catalogue.png",
                      crop: { x: 0, y: 0, width: 0.5, height: 1 },
                      scroll: false,
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "a",
                        dispositions: [
                          {
                            participantId: "figure",
                            embodimentId: "figure--img",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
                          },
                        ],
                      },
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    };
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const slot = container.querySelector('[data-id="figure"].presentation-morph-slot--figure') as HTMLElement | null;
    expect(slot).not.toBeNull();
    expect(container.querySelector(".presentation-figure-scroll-viewport")).toBeNull();
  });
  it("figureScrollOverlayThumbMetrics sizes overlay thumbs from scroll metrics", () => {
    expect(figureScrollOverlayThumbMetrics(280, 1086, 0).visible).toBe(true);
    expect(figureScrollOverlayThumbMetrics(280, 1086, 0).thumbSize).toBeGreaterThan(0);
    expect(figureScrollOverlayThumbMetrics(280, 280, 0).visible).toBe(false);
  });
  it("styles figure scroll viewports for one-axis overflow", async () => {
    const css = globalsCssSource;
    expect(css).toContain(".presentation-figure-scroll-viewport--axis-x");
    expect(css).toContain(".presentation-figure-scroll-viewport--axis-y");
    expect(css).toContain(".presentation-figure-scroll-scroller--axis-x");
    expect(css).toContain(".presentation-figure-scroll-scroller--axis-y");
    expect(css).toContain(".presentation-figure-scroll-scroller--axis-both");
    expect(css).toContain(".presentation-figure-scroll-viewport--overlay");
    expect(css).toContain(".presentation-figure-scroll-bar-thumb");
    expect(css).toContain(".presentation-figure-scroll-media");
    expect(css).toContain(".presentation-figure-scroll-content");
    expect(css).toContain("--scrollbar-thumb-active");
    expect(css).toContain(".presentation-figure-scroll-bar:hover .presentation-figure-scroll-bar-thumb");
    expect(css).toContain(".presentation-figure-scroll-bar--dragging .presentation-figure-scroll-bar-thumb");
    expect(css).toContain(".presentation-figure-scroll-bar-thumb:hover");
    expect(css).toContain(".presentation-figure-scroll-bar-thumb:active");
    expect(css).not.toContain("scrollbar-gutter: stable");
    expect(css).toMatch(/\.presentation-morph-slot--figure:not\(\.presentation-figure-scroll-content\)[\s\S]*width:\s*100%\s*!important/);
  });
  it("uses cover for full image and uniform crop zoom for partial crops", () => {
    const crop = { x: 0, y: 0, width: 0.5, height: 1 };
    const square = figureCropBackgroundVars({ kind: "figure", id: "", src: "/catalogue.png", crop }, crop, { x: 0, y: 0, width: 0.5, height: 0.5 });
    const wide = figureCropBackgroundVars({ kind: "figure", id: "", src: "/catalogue.png", crop }, crop, { x: 0, y: 0, width: 1, height: 0.25 });
    expect(square["--presentation-figure-bg-size" as keyof typeof square]).toBe("400% auto");
    expect(wide["--presentation-figure-bg-size" as keyof typeof wide]).toBe("1600% auto");
    expect(wide["--presentation-figure-bg-position" as keyof typeof wide]).toBe("0% 50%");
    expect(square["--presentation-figure-bg-position" as keyof typeof square]).toBe("0% 50%");
  });
  it("uses CSS cover for full catalogue in the catalogue frame", () => {
    const sourceAspect = 1222 / 896;
    const crop = { x: 0, y: 0, width: 1, height: 1 };
    const frame = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
    const vars = figureCropBackgroundVars({ kind: "figure", id: "", src: "/🖼️bauteilbörse.png", crop, sourceAspect }, crop, frame);
    expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe("cover");
    expect(vars["--presentation-figure-bg-position" as keyof typeof vars]).toBe("50% 50%");
  });
  it("uses windowed mosaic for rest and grid and centered crop for morph on catalogue-to-focus", () => {
    const sourceAspect = 1222 / 896;
    const catalogueFrame = { x: 0.127, y: 0.2, width: 0.746, height: 0.75 };
    const mosaic = { rows: 3, columns: 5, frame: catalogueFrame };
    const crop = {
      x: catalogueFrame.x + catalogueFrame.width / 5,
      y: catalogueFrame.y + catalogueFrame.height / 3,
      width: catalogueFrame.width / 5,
      height: catalogueFrame.height / 3,
    };
    const gridFrame = { x: 0.127, y: 0.2, width: 0.1492, height: 0.1823 };
    const focusFrame = { x: 0.1, y: 0.2, width: 0.15, height: 0.18 };
    const labelFrame = { x: 0.6, y: 0.44, width: 0.24, height: 0.12 };
    const windowed = mosaicWindowedCoverVars({ column: 1, row: 1 }, mosaic, catalogueFrame, sourceAspect);
    const morphPos = figureCropBackgroundPosition(crop);
    const morphSize = figureCropBackgroundSize(crop, labelFrame, sourceAspect);
    const vars = figureCropBackgroundVars(
      {
        kind: "figure",
        id: "tile-figure",
        src: "/🖼️bauteilbörse.png",
        crop,
        sourceAspect,
        mosaic,
      },
      crop,
      focusFrame,
      labelFrame,
      gridFrame,
    );
    expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe(windowed.size);
    expect(vars["--presentation-figure-bg-position" as keyof typeof vars]).toBe(`${windowed.posX}% ${windowed.posY}%`);
    expect(vars["--presentation-figure-bg-grid-size" as keyof typeof vars]).toBe(windowed.size);
    expect(vars["--presentation-figure-bg-grid-position" as keyof typeof vars]).toBe(`${windowed.posX}% ${windowed.posY}%`);
    expect(vars["--presentation-figure-bg-size-morph" as keyof typeof vars]).toBe(morphSize);
    expect(vars["--presentation-figure-bg-position-morph" as keyof typeof vars]).toBe(`${morphPos.posX}% ${morphPos.posY}%`);
  });
  it("mosaic windowed cover uses columns×100% width for catalogue 3×5 on a wide slide", () => {
    const sourceAspect = 1222 / 896;
    const frame = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
    const slideAspect = 960 / 700;
    const vars = mosaicWindowedCoverVars({ column: 0, row: 1 }, { rows: 3, columns: 5 }, frame, sourceAspect, slideAspect);
    expect(vars.size).toBe("500% auto");
    expect(vars.posX).toBe(0);
  });
}
