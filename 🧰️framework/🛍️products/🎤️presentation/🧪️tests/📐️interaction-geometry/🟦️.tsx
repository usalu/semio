/** 📐️ The presentation react deck, interaction geometry: ephemeral layout, marquee rules, flow offsets, measured bounds, resize and enlarge math.
 *
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { buildResolutionScope, expandThoughtSlides, intro, resolveArrangement, split } from "@semio-tech/presentation";
import type { DispositionPosition } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it } from "vitest";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME, buildInteractiveSlideLayout, clientToSectionFraction, dispositionHasEphemeralLayout, dispositionPlacementContainer, flowDispositionManipulationRect, flowDispositionOffsetStyle, flowPixelOffsetToSectionRect, flowPointerDeltaToLocal, groupBoundingRect, interactiveDispositionChromeStyle, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, isFlowPixelOffsetTransform, isNormalizedSlideFrame, isUsableMeasuredRect, marqueeSelectionRule, marqueeSelects, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, normalizeMarquee, pointerNearSlideResetHotspot, rectContains, rectsIntersect, resizeDispositionRect, resolveRevealArrangement, revealDeckInOverview, scaleRectWithinGroup, slideCoordinateRoot, slideHasEphemeralLayout, suppressRevealOverviewSlideNavigation, tightElementBoundsRect, toggleEnlargeRect, transformFrameStyle, translateDispositionRect } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { source, stylesheetRelativePath } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("presentation interaction geometry", () => {
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

  it("detects flow pixel-offset transforms without measured rect", () => {
    const measured = { x: 0.2, y: 0.3, width: 0.25, height: 0.1 };
    expect(isFlowPixelOffsetTransform({ x: 12, y: -4, width: 0.25, height: 0.1 }, undefined)).toBe(true);
    expect(isFlowPixelOffsetTransform({ x: 0, y: 0, width: 0.25, height: 0.1 }, undefined)).toBe(true);
    expect(isFlowPixelOffsetTransform({ x: 0.2, y: 0.3, width: 0.25, height: 0.1 }, undefined)).toBe(false);
    expect(isFlowPixelOffsetTransform({ x: 0, y: 0, width: 0.25, height: 0.1 }, measured)).toBe(true);
  });

  it("maps flow pointer delta through drag target visual scale", () => {
    const section = document.createElement("section");
    section.className = "presentation-arrangement--interactive";
    document.body.appendChild(section);
    section.getBoundingClientRect = () => new DOMRect(0, 0, 480, 350);
    Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
    const delta = flowPointerDeltaToLocal(section, 100, 200, 150, 260);
    document.body.removeChild(section);
    expect(delta.dx).toBeCloseTo(100);
    expect(delta.dy).toBeCloseTo(120);
  });

  it("intro morph title placements have no declared slide frame", () => {
    const deck = intro({
      language: "de",
      title: { full: ["Title"], short: "T" },
      description: { full: ["D"], short: "d" },
      goal: ["G"],
      authors: { lines: [[{ name: "A" }]] },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni", shortName: "U", suffix: { mark: "x", name: "Chair" } },
          ],
        ],
      },
    });
    const thought = deck.chapters[0]!.sequences[0]!.thoughts[0]!;
    const renderSlide = expandThoughtSlides(thought)[0]!;
    const scope = buildResolutionScope([thought]);
    const resolved = resolveRevealArrangement(scope, renderSlide.arrangement, {});
    const layout = buildInteractiveSlideLayout(renderSlide.id, resolved, true);
    expect(layout.placements.every((entry) => entry.sectionRect === undefined)).toBe(true);
  });

  it("maps flow drag 1:1 when reveal stack layout is tall but drag target is slide-sized", () => {
    const reveal = document.createElement("div");
    reveal.className = "reveal";
    reveal.style.setProperty("--presentation-slide-width", "960");
    reveal.style.setProperty("--presentation-slide-height", "700");
    const stack = document.createElement("section");
    const inner = document.createElement("section");
    inner.className = "presentation-arrangement--interactive";
    const content = document.createElement("div");
    content.className = "presentation-interactive-disposition__content";
    inner.append(content);
    stack.append(inner);
    reveal.append(stack);
    document.body.append(reveal);
    Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(stack, "offsetHeight", { value: 4900, configurable: true });
    Object.defineProperty(inner, "offsetWidth", { value: 0, configurable: true });
    Object.defineProperty(inner, "offsetHeight", { value: 0, configurable: true });
    Object.defineProperty(content, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(content, "offsetHeight", { value: 120, configurable: true });
    stack.getBoundingClientRect = () => new DOMRect(0, 0, 480, 350);
    content.getBoundingClientRect = () => new DOMRect(0, 280, 480, 60);
    expect(slideCoordinateRoot(inner)).toBe(inner);
    const delta = flowPointerDeltaToLocal(content, 100, 200, 120, 220);
    document.body.removeChild(reveal);
    expect(delta.dx).toBeCloseTo(40);
    expect(delta.dy).toBeCloseTo(40);
  });

  it("does not treat sub-unit flow drag offsets as normalized slide frames", () => {
    const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
    expect(isFlowPixelOffsetTransform({ x: 12, y: 4, width: 0.3, height: 0.08 }, measured)).toBe(true);
    expect(isNormalizedSlideFrame({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 })).toBe(true);
    expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, measured)).toBe(false);
    expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, undefined)).toBe(false);
  });

  it("rejects unusable measured fractions", () => {
    expect(isUsableMeasuredRect({ x: 0.2, y: 0.3, width: 0.1, height: 0.1 })).toBe(true);
    expect(isUsableMeasuredRect({ x: 0.2, y: 0.3, width: 0.005, height: 0.1 })).toBe(false);
  });

  it("measures flow disposition bounds from morph nodes not full-width wrapper", () => {
    const section = document.createElement("section");
    const root = document.createElement("div");
    const heading = document.createElement("h2");
    heading.textContent = "Entwerfen mit Bestand";
    root.appendChild(heading);
    section.appendChild(root);
    document.body.appendChild(section);
    section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
    root.getBoundingClientRect = () => new DOMRect(0, 280, 960, 120);
    heading.getBoundingClientRect = () => new DOMRect(0, 300, 960, 80);
    const measured = measureDispositionBoundsInSection(root, section);
    document.body.removeChild(section);
    expect(measured?.width).toBeLessThan(0.5);
    expect(measured?.x).toBeGreaterThan(0.1);
  });

  it("measures ink bounds relative to the disposition wrapper for selection chrome", () => {
    const root = document.createElement("div");
    const heading = document.createElement("h2");
    heading.textContent = "Title";
    root.appendChild(heading);
    document.body.appendChild(root);
    root.getBoundingClientRect = () => new DOMRect(0, 280, 960, 120);
    heading.getBoundingClientRect = () => new DOMRect(330, 300, 300, 80);
    const inWrapper = measureDispositionBoundsInContainer(root, root);
    document.body.removeChild(root);
    expect(inWrapper?.x).toBeCloseTo(330 / 960);
    expect(inWrapper?.width).toBeCloseTo(300 / 960);
    expect(inWrapper?.y).toBeCloseTo(20 / 120);
  });

  it("uses tight ink bounds for block headings", () => {
    const heading = document.createElement("h2");
    heading.textContent = "Goal line";
    document.body.appendChild(heading);
    heading.getBoundingClientRect = () => new DOMRect(0, 200, 960, 48);
    const tight = tightElementBoundsRect(heading);
    document.body.removeChild(heading);
    expect(tight).not.toBeNull();
    expect(tight!.width).toBeLessThan(960);
  });

  it("scales group members and toggles enlarge", () => {
    const a = { x: 0.1, y: 0.2, width: 0.2, height: 0.2 };
    const b = { x: 0.5, y: 0.2, width: 0.2, height: 0.2 };
    const group = groupBoundingRect([a, b]);
    expect(group?.width).toBeCloseTo(0.6);
    const grown = { x: 0, y: 0.1, width: 0.8, height: 0.3 };
    const scaledA = scaleRectWithinGroup(a, group!, grown);
    expect(scaledA.x).toBeCloseTo(0);
    const full = toggleEnlargeRect(a, undefined);
    expect(full.rect).toEqual(SLIDE_INTERACTIVE_ENLARGE_FRAME);
    expect(full.stash).toEqual(a);
    const restored = toggleEnlargeRect(full.rect, full.stash);
    expect(restored.rect).toEqual(a);
  });

  it("uses uniform min-axis scale for interactive resize content", () => {
    const baseline = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
    const grown = { x: 0.2, y: 0.3, width: 0.6, height: 0.2 };
    expect(interactiveDispositionContentScale(grown, baseline)).toBeCloseTo(1.5);
    const stretched = { x: 0.2, y: 0.3, width: 0.6, height: 0.35 };
    expect(interactiveDispositionContentScale(stretched, baseline)).toBeCloseTo(1.5);
    expect(interactiveDispositionContentScale(baseline, baseline)).toBeNull();
    expect(interactiveDispositionContentScaleStyle(1.25).transform).toBe("scale(1.25)");
  });

  it("builds one interactive placement per tile disposition", () => {
    const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
    const scope = buildResolutionScope([{ participants: grid.participants, embodiments: grid.embodiments }]);
    const resolved = resolveArrangement(scope, {
      id: "tiles",
      dispositions: grid.dispositions,
    });
    const layout = buildInteractiveSlideLayout("slide-1", resolved, true);
    expect(layout.placements).toHaveLength(4);
    expect(layout.rowBands).toHaveLength(0);
    expect(layout.placements.every((entry) => entry.sectionRect !== undefined)).toBe(true);
    expect(layout.placements.every((entry) => entry.revealMorphId !== undefined)).toBe(true);
  });

  it("keeps reveal morph data-id on canvas placements only", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "title" }, { id: "description" }],
        embodiments: [
          { kind: "text", id: "title--full", lines: ["A"], level: "heading", morphRoot: "heading-line" },
          {
            kind: "text",
            id: "description--full",
            lines: ["Long description"],
            level: "heading",
            morphRoot: "heading-block",
          },
        ],
      },
    ]);
    const flow = buildInteractiveSlideLayout(
      "goal",
      resolveArrangement(scope, {
        id: "goal",
        dispositions: [
          { participantId: "title", embodimentId: "title--full", emphasis: "muted" },
          { participantId: "description", embodimentId: "description--full", emphasis: "muted" },
        ],
      }),
      true,
    );
    expect(flow.placements.every((entry) => entry.revealMorphId === undefined)).toBe(true);
    const canvas = buildInteractiveSlideLayout(
      "tiles",
      resolveArrangement(scope, {
        id: "tiles",
        dispositions: [
          {
            participantId: "description",
            embodimentId: "description--full",
            emphasis: "active",
            position: { x: 0.1, y: 0.2, width: 0.8, height: 0.3 },
          },
        ],
      }),
      true,
    );
    expect(canvas.placements[0]?.revealMorphId).toBe("description");
  });

  it("styles source ghosts hidden at rest and visible during auto-animate", async () => {
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const cssPath = resolve(dirname(fileURLToPath(source.url)), stylesheetRelativePath);
    const css = readFileSync(cssPath, "utf8");
    expect(css).toContain(".reveal .presentation-source-ghost");
    expect(css).toContain('section[data-auto-animate="pending"] .presentation-source-ghost');
    expect(css).not.toContain('section[title="catalogue-focus"]');
  });

});
