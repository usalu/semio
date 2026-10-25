/** 🔶️ The detects disposition ephemeral layout tests of the presentation interaction geometry suite. */
import type { DispositionPosition } from "@semio-tech/presentation-react";
import { expect, it } from "vitest";
import { clientToSectionFraction, dispositionHasEphemeralLayout, dispositionPlacementContainer, flowDispositionManipulationRect, flowDispositionOffsetStyle, flowPixelOffsetToSectionRect, interactiveDispositionChromeStyle, isFlowPixelOffsetTransform, marqueeSelectionRule, marqueeSelects, normalizeMarquee, pointerNearSlideResetHotspot, rectContains, rectsIntersect, resizeDispositionRect, revealDeckInOverview, slideHasEphemeralLayout, suppressRevealOverviewSlideNavigation, transformFrameStyle, translateDispositionRect } from "@semio-tech/presentation-react";

/** 🔷 Registers the detects disposition ephemeral layout tests of the presentation interaction geometry suite. */
export function registerDetectsDispositionEphemeralLayoutTests(): void {
  it("detects disposition ephemeral layout from anchor and transform", () => {
    const declared = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
    expect(dispositionHasEphemeralLayout(undefined, declared, undefined, false)).toBe(false);
    expect(dispositionHasEphemeralLayout(declared, declared, undefined, false)).toBe(false);
    expect(dispositionHasEphemeralLayout({ x: 0.3, y: 0.3, width: 0.4, height: 0.2 }, declared, undefined, false)).toBe(true);
  });
  it("detects slide ephemeral layout and reset hotspot proximity", () => {
    const declared = new Map<string, DispositionPosition | undefined>([["a", { x: 0.2, y: 0.3, width: 0.4, height: 0.2 }]]);
    expect(slideHasEphemeralLayout(new Map(), new Set(), declared)).toBe(false);
    expect(slideHasEphemeralLayout(new Map([["a", { x: 0.2, y: 0.3, width: 0.4, height: 0.2 }]]), new Set(), declared)).toBe(false);
    expect(slideHasEphemeralLayout(new Map([["a", { x: 0.3, y: 0.3, width: 0.4, height: 0.2 }]]), new Set(), declared)).toBe(true);
    expect(slideHasEphemeralLayout(new Map(), new Set(["a"]), declared)).toBe(true);
    const section = document.createElement("section");
    document.body.appendChild(section);
    section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
    expect(pointerNearSlideResetHotspot(section, 920, 20)).toBe(true);
    expect(pointerNearSlideResetHotspot(section, 100, 20)).toBe(false);
    document.body.removeChild(section);
  });
  it("detects intersection and containment", () => {
    const a = { x: 0.1, y: 0.1, width: 0.3, height: 0.3 };
    const b = { x: 0.25, y: 0.25, width: 0.3, height: 0.3 };
    const outer = { x: 0, y: 0, width: 1, height: 1 };
    expect(rectsIntersect(a, b)).toBe(true);
    expect(rectContains(outer, a)).toBe(true);
    expect(rectContains(a, b)).toBe(false);
  });
  it("applies crossing vs window marquee rules", () => {
    const inside = { x: 0.2, y: 0.2, width: 0.1, height: 0.1 };
    const partial = { x: 0.55, y: 0.55, width: 0.3, height: 0.3 };
    const crossingMarquee = normalizeMarquee({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 });
    const windowMarquee = normalizeMarquee({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 });
    expect(marqueeSelectionRule({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 })).toBe("crossing");
    expect(marqueeSelectionRule({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 })).toBe("window");
    expect(marqueeSelects(crossingMarquee, inside, "crossing")).toBe(true);
    expect(marqueeSelects(windowMarquee, inside, "window")).toBe(true);
    expect(marqueeSelects(windowMarquee, partial, "window")).toBe(false);
    expect(marqueeSelects(crossingMarquee, partial, "crossing")).toBe(true);
  });
  it("detects reveal overview from an element inside the deck", () => {
    const reveal = document.createElement("div");
    reveal.className = "reveal";
    const section = document.createElement("section");
    reveal.appendChild(section);
    expect(revealDeckInOverview(section)).toBe(false);
    reveal.classList.add("overview");
    expect(revealDeckInOverview(section)).toBe(true);
  });
  it("suppresses the reveal overview slide click after pointer gestures", () => {
    const reveal = document.createElement("div");
    reveal.className = "reveal";
    const slide = document.createElement("section");
    const inner = document.createElement("div");
    reveal.appendChild(slide);
    slide.appendChild(inner);
    let overviewClickCalls = 0;
    slide.addEventListener(
      "click",
      () => {
        overviewClickCalls += 1;
      },
      true,
    );
    suppressRevealOverviewSlideNavigation({ target: inner } as unknown as Event);
    const click = new MouseEvent("click", { bubbles: true, cancelable: true });
    inner.dispatchEvent(click);
    expect(overviewClickCalls).toBe(0);
    expect(click.defaultPrevented).toBe(true);
  });
  it("translates and resizes with minimum size", () => {
    const rect = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
    const moved = translateDispositionRect(rect, 0.1, -0.05);
    expect(moved.x).toBeCloseTo(0.3);
    expect(moved.y).toBeCloseTo(0.25);
    const narrow = { x: 0.35, y: 0.4, width: 0.3, height: 0.1 };
    const movedX = translateDispositionRect(narrow, 0.12, 0);
    expect(movedX.x).toBeCloseTo(0.47);
    expect(movedX.y).toBeCloseTo(0.4);
    const unbounded = translateDispositionRect({ x: 0.8, y: 0.5, width: 0.2, height: 0.1 }, 0.5, -0.3);
    expect(unbounded.x).toBeCloseTo(1.3);
    expect(unbounded.y).toBeCloseTo(0.2);
    const resized = resizeDispositionRect(rect, "se", 0.2, 0.1);
    expect(resized.width).toBeCloseTo(0.6);
    expect(resized.height).toBeCloseTo(0.3);
  });
  it("starts flow manipulation at zero offset with measured size", () => {
    const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
    expect(flowDispositionManipulationRect(measured, undefined)).toEqual({
      x: 0,
      y: 0,
      width: 0.3,
      height: 0.08,
    });
  });
  it("detects flow pixel-offset transforms and maps them to section frames", () => {
    const measured = { x: 0.2, y: 0.3, width: 0.25, height: 0.1 };
    const offset = { x: 96, y: 40, width: 0.25, height: 0.1 };
    expect(isFlowPixelOffsetTransform(offset, measured)).toBe(true);
    expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.25, height: 0.1 }, measured)).toBe(false);
    expect(isFlowPixelOffsetTransform({ x: 12, y: -4, width: 0.25, height: 0.1 }, measured)).toBe(true);
    expect(isFlowPixelOffsetTransform(offset, undefined)).toBe(true);
    expect(isFlowPixelOffsetTransform({ ...offset, width: 0.3 }, measured)).toBe(false);
    const section = document.createElement("section");
    section.style.width = "var(--layout-deck-width)";
    section.style.height = "var(--layout-deck-height)";
    document.body.appendChild(section);
    section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
    const sectionRect = flowPixelOffsetToSectionRect(measured, offset, section);
    document.body.removeChild(section);
    expect(sectionRect.x).toBeCloseTo(0.3);
    expect(sectionRect.y).toBeCloseTo(0.357, 2);
  });
  it("maps flow offsets to local translate pixels", () => {
    expect(flowDispositionOffsetStyle({ x: 96, y: 140, width: 0.3, height: 0.08 }).transform).toBe("translate3d(96px, 140px, 0)");
  });
  it("resolves arrangement canvas as placement container for positioned slides", () => {
    const section = document.createElement("section");
    const canvas = document.createElement("div");
    canvas.className = "presentation-arrangement-canvas";
    section.append(canvas);
    Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
    expect(dispositionPlacementContainer(section, false)).toBe(section);
    expect(dispositionPlacementContainer(section, true)).toBe(canvas);
  });
  it("maps pointer fractions via parent stack when nested reveal section has zero height", () => {
    const outer = document.createElement("section");
    const inner = document.createElement("section");
    outer.append(inner);
    Object.defineProperty(outer, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(outer, "offsetHeight", { value: 700, configurable: true });
    Object.defineProperty(inner, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(inner, "offsetHeight", { value: 0, configurable: true });
    outer.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
    inner.getBoundingClientRect = () => new DOMRect(0, 0, 960, 0);
    expect(dispositionPlacementContainer(inner, false)).toBe(outer);
    const fraction = clientToSectionFraction(inner, 480, 350, { clamp: false });
    expect(fraction.x).toBeCloseTo(0.5);
    expect(fraction.y).toBeCloseTo(0.5);
  });
  it("uses explicit chrome frame only for flow slides, not canvas-framed slides", () => {
    const rect = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
    expect(
      interactiveDispositionChromeStyle({
        selected: true,
        effectiveRect: rect,
        canvasFramed: false,
        enlarged: false,
      }),
    ).toEqual(transformFrameStyle(rect));
    expect(
      interactiveDispositionChromeStyle({
        selected: true,
        effectiveRect: rect,
        canvasFramed: true,
        enlarged: false,
      }),
    ).toBeUndefined();
  });
}
