/** 🔶️ The resolvepresentationasseturl maps deck relative tests of the PresentationDeck suite. */
import type { PdfEmbodiment } from "@semio-tech/presentation-react";
import { expect, it, beforeEach, afterEach } from "vitest";
import { FIGURE_WHEEL_ZOOM_MAX, figureBackgroundSizeScrollAxis, figureBackgroundSizeZoomed, figureCoverOverflowAxis, figureCoverScrollContentSize, figureCoverScrollElementStyle, figureCropScrollBackgroundSize, figureEmbodimentScrollEnabled, figureScrollOffsetForBackgroundPosition, figureWheelZoomStep, mediaTeaserActive, pdfAdjacentPage, pdfCanGoToNextPage, pdfCanGoToPreviousPage, pdfCoverScale, pdfEmbodimentInitialPage, pdfPageNavEnabled, pdfScrollCoverScale, resolvePresentationAssetUrl, unmountPresentation } from "@semio-tech/presentation-react";
import { globalsCssSource } from "../../🧰️presentation-deck-helpers/🟦️.tsx";

/** 🔷 Registers the resolvepresentationasseturl maps deck relative tests of the PresentationDeck suite. */
export function registerResolvepresentationasseturlMapsDeckRelativeTests(): void {
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
}
