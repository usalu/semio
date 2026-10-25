/** ✂️ The presentation react deck, rendering of cropped figure dispositions and the auto-animate style sheet rules: uniform scale, ghost opacity, anisotropic scale and run slides. */
import type { Presentation } from "@semio-tech/presentation-react";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { mountPresentation, patchAutoAnimateUniformScale, patchPresentationAutoAnimateRunStyleSheet, patchPresentationAutoAnimateStyleSheet, resolvePresentationAutoAnimateRunSlides, unmountPresentation } from "@semio-tech/presentation-react";
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

  it("renders cropped figure dispositions with matching morph slot DOM", () => {
    const deck: Presentation = {
      id: "crop-figure",
      name: "Crop",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "crop",
                  participants: [{ id: "catalogue-col1" }],
                  embodiments: [
                    {
                      kind: "figure",
                      id: "catalogue-col1--crop",
                      src: "/catalogue.png",
                      crop: { x: 0, y: 0, width: 0.5, height: 1 },
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "focus",
                        dispositions: [
                          {
                            participantId: "catalogue-col1",
                            embodimentId: "catalogue-col1--crop",
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
    const slot = container.querySelector('[data-id="catalogue-col1"]') as HTMLElement | null;
    const cropSlot = slot?.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
    expect(cropSlot?.style.backgroundImage).toContain("/catalogue.png");
    expect(cropSlot?.querySelector("h2")).toBeNull();
    expect(slot?.style.left).toBe("35%");
    expect(cropSlot?.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
    expect(cropSlot?.style.getPropertyValue("--presentation-figure-bg-position")).toBe("0% 50%");
    expect(slot?.querySelector(".presentation-figure-scroll-viewport--axis-x")).not.toBeNull();
  });

  it("rewrites non-uniform auto-animate scale() to a uniform zoom", () => {
    expect(patchAutoAnimateUniformScale("transform: translate(1px, 2px) scale(1.5, 2) !important;")).toBe("transform: translate(1px, 2px) scale(2) !important;");
  });

  it("appends morph ghost opacity 1→0 and morph-into 0→1 rules to the auto-animate sheet", () => {
    const sheet = { innerHTML: "transform: scale(1, 2);" };
    patchPresentationAutoAnimateStyleSheet(sheet, 0.8);
    expect(sheet.innerHTML).toContain("scale(2)");
    expect(sheet.innerHTML).toContain('[data-auto-animate="running"] .presentation-target-ghost[data-auto-animate-target]');
    expect(sheet.innerHTML).toContain("opacity: 1 !important");
    expect(sheet.innerHTML).toContain("presentation-target-ghost-fade-out 0.8s ease forwards !important");
    expect(sheet.innerHTML).not.toContain("presentation-morph-target-fade-in");
  });

  it("preserves anisotropic scale() in the auto-animate sheet for intro flow morph", () => {
    const sheet = { innerHTML: "transform: scale(1, 2) !important;" };
    patchPresentationAutoAnimateStyleSheet(sheet, 0.8, { introFlowMorph: true });
    expect(sheet.innerHTML).toContain("scale(1, 2)");
    expect(sheet.innerHTML).not.toContain("scale(2)");
  });

  it("leaves intro flow auto-animate sheets identical to reveal.js output", () => {
    const fromSlide = document.createElement("section");
    fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const toSlide = document.createElement("section");
    toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const sheet = { innerHTML: "transform: translate(10px, 20px) scale(0.4, 1.2) !important;" };
    patchPresentationAutoAnimateRunStyleSheet(sheet, 0.8, fromSlide, toSlide);
    expect(sheet.innerHTML).toBe("transform: translate(10px, 20px) scale(0.4, 1.2) !important;");
  });

  it("uses pending slide pair when reveal autoanimate omits event slides", () => {
    const fromSlide = document.createElement("section");
    fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const toSlide = document.createElement("section");
    toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const resolved = resolvePresentationAutoAnimateRunSlides({}, { fromSlide, toSlide });
    const sheet = { innerHTML: "transform: translate(10px, 20px) scale(0.4, 1.2) !important;" };
    patchPresentationAutoAnimateRunStyleSheet(sheet, 0.8, resolved.fromSlide, resolved.toSlide);
    expect(resolved).toEqual({ fromSlide, toSlide });
    expect(sheet.innerHTML).toBe("transform: translate(10px, 20px) scale(0.4, 1.2) !important;");
  });

  it("preserves anisotropic scale() in the auto-animate sheet for many-to-one morph", () => {
    const sheet = { innerHTML: "transform: scale(0.4, 0.9) !important;" };
    patchPresentationAutoAnimateStyleSheet(sheet, 0.8, { manyToOneMorph: true });
    expect(sheet.innerHTML).toContain("scale(0.4, 0.9)");
    expect(sheet.innerHTML).not.toContain("scale(0.9)");
    expect(sheet.innerHTML).toContain("presentation-target-ghost-frame 0.8s ease forwards");
  });

  it("animates figure crop and target-ghost frames only during many-to-one auto-animate", async () => {
    const css = globalsCssSource;
    expect(css).toContain("presentation-figure-crop-morph-from-rest");
    expect(css).toContain("presentation-figure-crop-morph-grid-to-focus");
    expect(css.match(/presentation-morph-crop-from[\s\S]*?presentation-figure-crop-morph-from-rest/)).toBeTruthy();
    expect(css).toContain("@keyframes presentation-target-ghost-frame");
    expect(css).toContain('section.presentation-arrangement--many-to-one-morph[data-auto-animate="running"]');
    expect(css).toContain(".presentation-morph-crop-from");
    expect(css).toContain(".presentation-morph-crop-to");
  });

  it("rests morph-source ghosts with opacity only so reveal can measure FLIP targets", async () => {
    const css = globalsCssSource;
    const restRule = css.match(/\.reveal \.presentation-target-ghost \{[\s\S]*?\}/)?.[0] ?? "";
    expect(restRule).toContain("opacity: 0");
    expect(restRule).not.toContain("opacity: 0 !important");
    expect(restRule).not.toContain("visibility: hidden");
    expect(css).toMatch(/section\[data-auto-animate="running"\][\s\S]*?presentation-target-ghost-fade-out/);
    expect(restRule).toContain("pointer-events: none !important");
    expect(restRule).toContain("z-index: 0");
    expect(css).toContain("> .presentation-interactive-disposition:not(.presentation-target-ghost):not(.presentation-source-ghost)");
    expect(css).toMatch(/section\[data-auto-animate="pending"\][\s\S]*?\.presentation-target-ghost/);
  });
});
