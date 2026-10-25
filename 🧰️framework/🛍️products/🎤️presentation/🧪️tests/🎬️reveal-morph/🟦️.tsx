/** 🎬️ The presentation react deck, reveal morph: resolving the reveal arrangement with its morph companions, and the reveal text morph pairing.
 *
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { buildResolutionScope } from "@semio-tech/presentation";
import type { Slide } from "@semio-tech/presentation-react";
import { describe, expect, it } from "vitest";
import { figureCropBackgroundVars, figureCropBackgroundVarsTargetGhost, isRevealAutoAnimatePairSource, morphFrameCssVars, presentationAutoAnimateMatcher, resolveRevealArrangement, revealInkMeasureForAutoAnimate, revealTextAutoAnimatePairOptions } from "@semio-tech/presentation-react";
import { stockRevealAutoAnimateMatcherHost } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("resolveRevealArrangement", () => {
  it("appends target companions for morphFrom on the labels slide", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "col1" }, { id: "tile" }],
        embodiments: [
          { kind: "text", id: "label", lines: ["A"], level: "heading" },
          { kind: "figure", id: "tile-figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
        ],
      },
    ]);
    const previousSlide: Slide = {
      arrangement: {
        id: "focus",
        dispositions: [
          {
            participantId: "tile",
            embodimentId: "tile-figure",
            emphasis: "active",
            position: { x: 0.5, y: 0.5, width: 0.2, height: 0.2 },
          },
        ],
      },
    };
    const resolved = resolveRevealArrangement(
      scope,
      {
        id: "labels",
        dispositions: [
          {
            participantId: "col1",
            embodimentId: "label",
            emphasis: "active",
            morphFrom: [
              {
                participantId: "tile",
                embodimentId: "tile-figure",
                position: { x: 0.2, y: 0.3, width: 0.2, height: 0.1 },
              },
            ],
          },
        ],
      },
      { previousSlide },
    );
    expect(resolved).toHaveLength(2);
    const companion = resolved.find((entry) => entry.revealMorphCompanion === "target");
    expect(companion?.morphId).toBe("tile");
    expect(companion?.position).toEqual({ x: 0.2, y: 0.3, width: 0.2, height: 0.1 });
    expect(companion?.revealMorphFromFrame).toEqual({ x: 0.5, y: 0.5, width: 0.2, height: 0.2 });
  });

  it("sets revealMorphToFrame on focus tiles when the next slide morphFrom references them", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "tile" }, { id: "col1" }],
        embodiments: [
          { kind: "figure", id: "tile-figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
          { kind: "text", id: "label", lines: ["A"], level: "heading" },
        ],
      },
    ]);
    const focusPosition = { x: 0.5, y: 0.5, width: 0.2, height: 0.2 };
    const labelPosition = { x: 0.2, y: 0.3, width: 0.2, height: 0.1 };
    const resolved = resolveRevealArrangement(
      scope,
      {
        id: "focus",
        dispositions: [
          {
            participantId: "tile",
            embodimentId: "tile-figure",
            emphasis: "active",
            position: focusPosition,
          },
        ],
      },
      {
        nextSlide: {
          arrangement: {
            id: "labels",
            dispositions: [
              {
                participantId: "col1",
                embodimentId: "label",
                emphasis: "active",
                morphFrom: [
                  {
                    participantId: "tile",
                    embodimentId: "tile-figure",
                    position: labelPosition,
                  },
                ],
              },
            ],
          },
        },
      },
    );
    expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphToFrame).toEqual(labelPosition);
  });

  it("morphFrameCssVars maps normalized frames to presentation custom properties", () => {
    expect(morphFrameCssVars({ x: 0.1, y: 0.2, width: 0.3, height: 0.4 }, { x: 0.5, y: 0.6, width: 0.2, height: 0.1 })).toEqual({
      "--presentation-morph-frame-left": "10%",
      "--presentation-morph-frame-top": "20%",
      "--presentation-morph-frame-width": "30%",
      "--presentation-morph-frame-height": "40%",
      "--presentation-frame-left": "50%",
      "--presentation-frame-top": "60%",
      "--presentation-frame-width": "20%",
      "--presentation-frame-height": "10%",
    });
  });

  it("keeps target-ghost rest crop on the source tile frame and morph crop on the label slot", () => {
    const crop = { x: 0.8, y: 0.7, width: 0.15, height: 0.2 };
    const embodiment = { kind: "figure" as const, id: "", src: "/catalogue.png", crop };
    const sourceFrame = { x: 0.77, y: 0.11, width: 0.2, height: 0.78 };
    const labelFrame = { x: 0.653333, y: 0.44, width: 0.246667, height: 0.12 };
    const vars = figureCropBackgroundVarsTargetGhost(embodiment, crop, sourceFrame, labelFrame);
    const sourceOnly = figureCropBackgroundVars(embodiment, crop, sourceFrame);
    const labelOnly = figureCropBackgroundVars(embodiment, crop, labelFrame);
    expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe(sourceOnly["--presentation-figure-bg-size" as keyof typeof sourceOnly]);
    expect(vars["--presentation-figure-bg-size-morph" as keyof typeof vars]).toBe(labelOnly["--presentation-figure-bg-size" as keyof typeof labelOnly]);
  });

  it("appends source companions for morphTo on the whole slide", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "whole" }, { id: "tile-a" }],
        embodiments: [
          { kind: "figure", id: "whole--figure", src: "/a.png" },
          { kind: "figure", id: "tile-a--figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
        ],
      },
    ]);
    const nextSlide: Slide = {
      arrangement: {
        id: "tiles",
        dispositions: [{ participantId: "tile-a", embodimentId: "tile-a--figure", emphasis: "active" }],
      },
    };
    const resolved = resolveRevealArrangement(
      scope,
      {
        id: "whole",
        dispositions: [
          {
            participantId: "whole",
            embodimentId: "whole--figure",
            emphasis: "active",
            morphTo: [{ participantId: "tile-a", position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 } }],
          },
        ],
      },
      { nextSlide },
    );
    expect(resolved).toHaveLength(2);
    const companion = resolved.find((entry) => entry.revealMorphCompanion === "source");
    expect(companion?.morphId).toBe("tile-a");
  });
});

describe("reveal text morph", () => {
  it("pairs intro leaf text nodes, not disposition wrappers", () => {
    const fromSlide = document.createElement("section");
    fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const toSlide = document.createElement("section");
    toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const fromWrap = document.createElement("div");
    fromWrap.className = "presentation-interactive-disposition";
    const fromHeading = document.createElement("h2");
    fromHeading.setAttribute("data-id", "description");
    fromHeading.textContent = "Long description line";
    fromWrap.append(fromHeading);
    fromSlide.append(fromWrap);
    const toWrap = document.createElement("div");
    toWrap.className = "presentation-interactive-disposition";
    const toHeading = document.createElement("h2");
    toHeading.setAttribute("data-id", "description");
    toHeading.textContent = "Short";
    toWrap.append(toHeading);
    toSlide.append(toWrap);
    expect(isRevealAutoAnimatePairSource(fromWrap)).toBe(false);
    expect(isRevealAutoAnimatePairSource(fromHeading)).toBe(true);
    const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), fromSlide, toSlide);
    expect(pairs).toEqual([
      {
        from: fromHeading,
        to: toHeading,
      },
    ]);
  });

  it("pairs short and full heading-block lines via a shared --0 suffix", () => {
    const fromSlide = document.createElement("section");
    fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const toSlide = document.createElement("section");
    toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const fromHeading = document.createElement("h2");
    fromHeading.setAttribute("data-id", "description--0");
    fromHeading.textContent = "Short";
    fromSlide.append(fromHeading);
    const toLine0 = document.createElement("h2");
    toLine0.setAttribute("data-id", "description--0");
    toLine0.textContent = "Long line one";
    const toLine1 = document.createElement("h2");
    toLine1.setAttribute("data-id", "description--1");
    toLine1.textContent = "Long line two";
    toSlide.append(toLine0, toLine1);
    const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), fromSlide, toSlide);
    expect(pairs).toEqual([
      {
        from: fromHeading,
        to: toLine0,
      },
    ]);
  });

  it("uses stock reveal auto-animate options on intro text pairs", () => {
    const fromSlide = document.createElement("section");
    fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const toSlide = document.createElement("section");
    toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
    const fromHeading = document.createElement("h2");
    fromHeading.setAttribute("data-id", "description--0");
    fromSlide.append(fromHeading);
    const toHeading = document.createElement("h2");
    toHeading.setAttribute("data-id", "description--0");
    toSlide.append(toHeading);
    const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), fromSlide, toSlide);
    expect(pairs[0]?.options).toBeUndefined();
  });

  it("uses slide-local measure options on non-intro text pairs", () => {
    const fromSlide = document.createElement("section");
    fromSlide.className = "presentation-arrangement--interactive";
    const toSlide = document.createElement("section");
    toSlide.className = "presentation-arrangement--interactive";
    const heading = document.createElement("h2");
    heading.setAttribute("data-id", "label");
    expect(revealTextAutoAnimatePairOptions(heading, fromSlide, toSlide)).toEqual({
      scale: false,
      measure: revealInkMeasureForAutoAnimate,
    });
  });

  it("measures morph text relative to the slide box, not the viewport", () => {
    const section = document.createElement("section");
    section.className = "presentation-arrangement--interactive";
    const heading = document.createElement("h2");
    heading.textContent = "Plattform";
    section.append(heading);
    document.body.append(section);
    section.getBoundingClientRect = () => new DOMRect(500, 0, 960, 700);
    heading.getBoundingClientRect = () => new DOMRect(780, 300, 400, 48);
    const viewport = heading.getBoundingClientRect();
    const measured = revealInkMeasureForAutoAnimate(heading);
    document.body.removeChild(section);
    expect(measured.x).not.toBeCloseTo(viewport.left);
    expect(measured.x).toBeGreaterThanOrEqual(0);
    expect(measured.x + measured.width).toBeLessThanOrEqual(960);
  });
});
