/** 🔍️ The presentation react deck, rendering of the figure scroll, cover and zoom math, pdf page subsets, crop sizing and mosaic windows. */
import { buildResolutionScope, splitFigureGrid } from "@semio-tech/presentation";
import type { PdfEmbodiment, Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { FIGURE_WHEEL_ZOOM_MAX, figureBackgroundSizeScrollAxis, figureBackgroundSizeZoomed, figureCoverOverflowAxis, figureCoverScrollContentSize, figureCoverScrollElementStyle, figureCropBackgroundPosition, figureCropBackgroundSize, figureCropBackgroundVars, figureCropScrollBackgroundSize, figureEmbodimentScrollEnabled, figureScrollOffsetForBackgroundPosition, figureScrollOverlayThumbMetrics, figureWheelZoomStep, mediaTeaserActive, mosaicWindowedCoverVars, mountPresentation, pdfAdjacentPage, pdfCanGoToNextPage, pdfCanGoToPreviousPage, pdfCoverScale, pdfEmbodimentInitialPage, pdfPageNavEnabled, pdfScrollCoverScale, resolvePresentationAssetUrl, resolveRevealArrangement, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("PresentationDeck", () => {
  let container: HTMLDivElement;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
  });

  afterEach(() => {
    unmountPresentation();
    container.remove();
  });

  it("resolvePresentationAssetUrl maps deck-relative paths through the Vite base", () => {
    expect(resolvePresentationAssetUrl("/🖼️bauteilbörse.png")).toBe("/🖼️bauteilbörse.png");
    expect(resolvePresentationAssetUrl("./🖼️bauteilbörse.png")).toBe("/🖼️bauteilbörse.png");
  });

  it("keeps figure selection chrome as a translucent overlay, not a solid fill", async () => {
    const css = globalsCssSource;
    expect(css).toContain(".presentation-interactive-disposition--kind-figure.presentation-interactive-disposition--selected");
    expect(css).toContain(".presentation-morph-slot--figure::after");
    expect(css).not.toMatch(/\.presentation-interactive-disposition--kind-figure\.presentation-interactive-disposition--selected[\s\S]*?\.presentation-morph-slot--figure\s*\{[^}]*background-color:\s*var\(--color-primary\)/);
  });

  it("computes pdf cover scale from container and page viewport", () => {
    expect(pdfCoverScale(768, 280, 595, 842)).toBeCloseTo(768 / 595);
    expect(pdfCoverScale(200, 400, 595, 842)).toBeCloseTo(400 / 842);
    expect(pdfCoverScale(0, 400, 595, 842)).toBeNull();
    expect(pdfScrollCoverScale(768, 280, 595, 842)).toBeCloseTo(768 / 595);
    expect(pdfScrollCoverScale(200, 400, 595, 842)).toBeCloseTo(400 / 842);
  });

  it("figureCoverScrollElementStyle fits one axis for wide video frames", () => {
    const style = figureCoverScrollElementStyle(565, 350, 1200 / 1080);
    expect(style.width).toBe("100%");
    expect(style.height).toBe("auto");
    expect(style.aspectRatio).toBe(String(1200 / 1080));
  });

  it("navigates only within a declared pdf page subset", () => {
    const thesis: PdfEmbodiment = {
      kind: "pdf",
      id: "thesis--doc",
      src: "/thesis.pdf",
      page: 25,
      pages: [1, 12, 25, 35, 42, 43, 51],
    };
    expect(pdfEmbodimentInitialPage(thesis)).toBe(25);
    expect(pdfEmbodimentInitialPage({ ...thesis, page: 99 })).toBe(1);
    expect(pdfPageNavEnabled(thesis, null)).toBe(true);
    expect(pdfPageNavEnabled({ ...thesis, pages: [1] }, null)).toBe(false);
    expect(pdfCanGoToPreviousPage(25, thesis, null)).toBe(true);
    expect(pdfCanGoToNextPage(51, thesis, null)).toBe(false);
    expect(pdfAdjacentPage(25, "next", thesis, null)).toBe(35);
    expect(pdfAdjacentPage(51, "next", thesis, null)).toBe(51);
    expect(pdfAdjacentPage(51, "prev", thesis, null)).toBe(43);
    expect(pdfAdjacentPage(1, "prev", thesis, null)).toBe(1);
  });

  it("figureCoverOverflowAxis picks the overflowing axis from frame and source aspect", () => {
    expect(figureCoverOverflowAxis(200, 100, 1)).toBe("y");
    expect(figureCoverOverflowAxis(100, 200, 1)).toBe("x");
    expect(figureCoverOverflowAxis(100, 100, 1)).toBeNull();
  });

  it("figureScrollOffsetForBackgroundPosition maps origin percents to scroll offsets", () => {
    expect(figureScrollOffsetForBackgroundPosition("y", 50, 1000, 400)).toBe(300);
    expect(figureScrollOffsetForBackgroundPosition("y", 0, 1000, 400)).toBe(0);
    expect(figureScrollOffsetForBackgroundPosition("x", 0, 800, 200)).toBe(0);
  });

  it("figureBackgroundSizeScrollAxis maps crop background-size to one scroll axis", () => {
    expect(figureBackgroundSizeScrollAxis("400% auto")).toBe("x");
    expect(figureBackgroundSizeScrollAxis("auto 1600%")).toBe("y");
    expect(figureBackgroundSizeScrollAxis("cover")).toBeNull();
    expect(figureCropScrollBackgroundSize("cover", "x")).toBe("100% auto");
    expect(figureCropScrollBackgroundSize("cover", "y")).toBe("auto 100%");
  });

  it("figureWheelZoomStep clamps ctrl+wheel zoom between cover baseline and max", () => {
    expect(figureWheelZoomStep(-100, 1)).toBeCloseTo(1.1);
    expect(figureWheelZoomStep(100, 1)).toBe(1);
    expect(figureWheelZoomStep(-100, FIGURE_WHEEL_ZOOM_MAX)).toBe(FIGURE_WHEEL_ZOOM_MAX);
    expect(figureWheelZoomStep(100, 2)).toBeCloseTo(2 / 1.1);
  });

  it("figureBackgroundSizeZoomed scales crop background-size strings", () => {
    expect(figureBackgroundSizeZoomed("400% auto", 2)).toBe("800% auto");
    expect(figureBackgroundSizeZoomed("auto 1600%", 1.5)).toBe("auto 2400%");
    expect(figureBackgroundSizeZoomed("cover", 2)).toBe("cover");
  });

  it("figureCoverScrollElementStyle grows with ctrl+wheel zoom", () => {
    const base = figureCoverScrollElementStyle(565, 350, 1200 / 1080);
    const zoomed = figureCoverScrollElementStyle(565, 350, 1200 / 1080, 2);
    expect(base.width).toBe("100%");
    expect(zoomed.width).toBe("200%");
  });

  it("figureCoverScrollContentSize enables both axes when zoomed past cover", () => {
    const square = figureCoverScrollContentSize(100, 100, 1, 2);
    expect(square.axis).toBe("both");
    const tall = figureCoverScrollContentSize(200, 100, 1, 2);
    expect(tall.axis).toBe("both");
  });

  it("mediaTeaserActive is true only when teaser is set", () => {
    expect(mediaTeaserActive(undefined)).toBe(false);
    expect(mediaTeaserActive({})).toBe(true);
    expect(mediaTeaserActive({ label: "Preview" })).toBe(true);
  });

  it("figureEmbodimentScrollEnabled defaults on and respects mosaic and scroll:false", () => {
    expect(figureEmbodimentScrollEnabled({ kind: "figure", id: "a", src: "/a.png" })).toBe(true);
    expect(
      figureEmbodimentScrollEnabled({
        kind: "figure",
        id: "a",
        src: "/a.png",
        scroll: false,
      }),
    ).toBe(false);
    expect(
      figureEmbodimentScrollEnabled({
        kind: "figure",
        id: "a",
        src: "/a.png",
        mosaic: { rows: 2, columns: 2 },
      }),
    ).toBe(false);
  });

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

  it("sets revealMorphFromMorphToFrame from the previous slide morphTo slots", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "tile" }],
        embodiments: [
          {
            kind: "figure",
            id: "tile-figure",
            src: "/a.png",
            crop: { x: 0, y: 0, width: 0.5, height: 1 },
          },
        ],
      },
    ]);
    const gridFrame = { x: 0.1, y: 0.1, width: 0.2, height: 0.3 };
    const focusFrame = { x: 0.4, y: 0.2, width: 0.15, height: 0.18 };
    const resolved = resolveRevealArrangement(
      scope,
      {
        id: "focus",
        dispositions: [
          {
            participantId: "tile",
            embodimentId: "tile-figure",
            emphasis: "active",
            position: focusFrame,
          },
        ],
      },
      {
        previousSlide: {
          arrangement: {
            id: "catalogue",
            dispositions: [
              {
                participantId: "catalogue",
                embodimentId: "catalogue--figure",
                emphasis: "active",
                morphTo: [{ participantId: "tile", embodimentId: "tile-figure", position: gridFrame }],
              },
            ],
          },
        },
      },
    );
    expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphFromMorphToFrame).toEqual(gridFrame);
  });

  it("assigns windowed mosaic cover per split tile", () => {
    const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
    const mosaic = { rows: 2, columns: 2, frame };
    const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
    const embodiment = {
      kind: "figure" as const,
      id: "",
      src: "/catalogue.png",
      mosaic,
    };
    const expected = tiles.map((tile, index) => {
      const column = index % 2;
      const row = Math.floor(index / 2);
      return mosaicWindowedCoverVars({ column, row }, mosaic, frame, 1);
    });
    const varsList = tiles.map((tile) => figureCropBackgroundVars(embodiment, tile.crop, tile.position));
    expect(varsList.map((vars) => vars["--presentation-figure-bg-size" as keyof typeof vars])).toEqual(expected.map((entry) => entry.size));
    expect(varsList.map((vars) => vars["--presentation-figure-bg-position" as keyof typeof vars])).toEqual(expected.map((entry) => `${entry.posX}% ${entry.posY}%`));
  });
});
