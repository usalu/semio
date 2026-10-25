/** 🔸️ The detects flow pixel offset tests of the presentation interaction geometry suite. */
import { buildResolutionScope, expandThoughtSlides, intro, resolveArrangement, split } from "@semio-tech/presentation";
import { expect, it } from "vitest";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME, buildInteractiveSlideLayout, flowPointerDeltaToLocal, groupBoundingRect, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, isFlowPixelOffsetTransform, isNormalizedSlideFrame, isUsableMeasuredRect, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, resolveRevealArrangement, scaleRectWithinGroup, slideCoordinateRoot, tightElementBoundsRect, toggleEnlargeRect } from "@semio-tech/presentation-react";

/** 🔹 Registers the detects flow pixel offset tests of the presentation interaction geometry suite. */
export function registerDetectsFlowPixelOffsetTests(): void {
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
}
