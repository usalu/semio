/** 🌊️ The presentation react deck, interaction DOM of intro flow dispositions: pixel-offset drag, auto-animate morph pairing, flow resize and flow enlarge. */
import { intro } from "@semio-tech/presentation";
import type { Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME, mountPresentation, presentationAutoAnimateMatcher, resetPdfCanvasPort, setPdfCanvasPort, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource, stockRevealAutoAnimateMatcherHost } from "../🧰️presentation-deck-helpers/🟦️.tsx";
import { mockClientRect, pointerClick, pointerDrag, testPdfCanvasPort } from "../🧱️interaction-dom-kit/🟦️.ts";

describe("presentation interaction dom", () => {
  let container: HTMLDivElement;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
    setPdfCanvasPort(testPdfCanvasPort);
  });

  afterEach(() => {
    unmountPresentation();
    resetPdfCanvasPort();
    container.remove();
  });

  it("allows intro flow disposition drag with pixel offsets", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A", "B", "C"], short: "Short" },
      description: { full: ["D1"], short: "D short" },
      goal: ["G1"],
      authors: { lines: [[{ name: "Alice" }]] },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
          ],
        ],
      },
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const section = container.querySelector('.slides > section > section[title="title"]') as HTMLElement;
    section.classList.add("present");
    const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
    const titleLine = section.querySelector('h2[data-id^="title"]') as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(disposition, 330, 120, 300, 80);
    mockClientRect(titleLine, 330, 120, 300, 80);
    act(() => {
      pointerClick(disposition);
      pointerDrag(disposition, 480, 320, 560, 360);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--offset")).toBe(true);
    expect(disposition.style.transform).toContain("translate3d(");
  });

  it("pairs intro description morph on leaf text for reveal auto-animate", () => {
    unmountPresentation();
    const deck = intro({
      language: "de",
      title: { full: ["Entwerfen mit Bestand"], short: "Entwerfen mit Bestand" },
      description: {
        full: ["Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten"],
        short: "Plattform zum Entwerfen mit wiederverwendete Bauteilen",
      },
      goal: ["Mehr Zeit zum manuellen Entwerfen", "dank Automatisierung!"],
      authors: { lines: [[{ name: "Alice" }]] },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
          ],
        ],
      },
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const descriptionSlide = container.querySelector('section[title="description"]') as HTMLElement;
    const goalSlide = container.querySelector('section[title="goal"]') as HTMLElement;
    expect(descriptionSlide.hasAttribute("data-auto-animate")).toBe(true);
    expect(descriptionSlide.getAttribute("data-auto-animate-id")).toBe(goalSlide.getAttribute("data-auto-animate-id"));
    const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), descriptionSlide, goalSlide);
    expect(pairs.some((pair) => pair.from.getAttribute("data-id") === "description")).toBe(true);
    expect(pairs.every((pair) => pair.from.matches("h1, h2, h3, h4, h5, h6, p"))).toBe(true);
    const descriptionPair = pairs.find((pair) => pair.from.getAttribute("data-id") === "description");
    expect(descriptionPair?.to.getAttribute("data-id")).toBe("description");
    expect(descriptionPair?.to.textContent).toContain("Plattform zum Entwerfen");
    expect(descriptionPair?.options).toBeUndefined();
    expect(goalSlide.querySelector('h2[data-id="description"]')).toBeTruthy();
    const authorsSlide = container.querySelector('section[title="authors"]') as HTMLElement;
    const goalToAuthors = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), goalSlide, authorsSlide);
    expect(goalToAuthors.some((pair) => pair.from.getAttribute("data-id") === "description" && pair.to.getAttribute("data-id") === "description")).toBe(true);
    const authorsPair = goalToAuthors.find((pair) => pair.from.getAttribute("data-id")?.startsWith("authors--"));
    expect(authorsPair).toBeTruthy();
    expect(authorsPair?.from.matches("h4")).toBe(true);
    expect(authorsPair?.options).toBeUndefined();
    expect(authorsSlide.querySelector('h2[data-id="description"].opacity-20')).toBeTruthy();
  });

  it("pairs authors across affiliation steps with non-zero morph measure delta", () => {
    unmountPresentation();
    const deck = intro({
      language: "de",
      title: { full: ["A"], short: "A" },
      description: { full: ["D"], short: "D" },
      goal: ["G"],
      authors: {
        lines: [[{ name: "Alice Example" }, { name: "Bob Beta" }], [{ name: "Carol Creator" }]],
      },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair" } },
          ],
        ],
      },
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const authorsSlide = container.querySelector('section[title="authors"]') as HTMLElement;
    const aff1Slide = container.querySelector('section[title="affiliations-1"]') as HTMLElement;
    const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), authorsSlide, aff1Slide);
    const authorsPair = pairs.find((pair) => pair.from.getAttribute("data-id")?.startsWith("authors--"));
    expect(authorsPair).toBeTruthy();
    expect(authorsPair?.options).toBeUndefined();
    expect(authorsPair?.from.textContent).toContain("Alice Example");
    expect(authorsPair?.to.textContent).toContain("A.");
  });

  it("resizes flow disposition from se handle when nested reveal section has zero height", () => {
    const deck: Presentation = {
      id: "flow-resize",
      name: "Flow Resize",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "flow",
                  participants: [{ id: "label" }],
                  embodiments: [{ kind: "text", id: "label--body", lines: ["Flow label"], level: "body" }],
                  slides: [
                    {
                      arrangement: {
                        id: "flow",
                        dispositions: [
                          {
                            participantId: "label",
                            embodimentId: "label--body",
                            emphasis: "active",
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
    const section = container.querySelector(".slides > section > section.presentation-arrangement--interactive:not(.presentation-arrangement--intro)") as HTMLElement;
    const stack = section.parentElement as HTMLElement;
    section.classList.add("present");
    stack.classList.add("present");
    Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(stack, "offsetHeight", { value: 700, configurable: true });
    Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(section, "offsetHeight", { value: 0, configurable: true });
    const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
    const label = disposition.querySelector("p") as HTMLElement;
    mockClientRect(stack, 0, 0, 960, 700);
    mockClientRect(section, 0, 0, 960, 0);
    mockClientRect(disposition, 330, 300, 300, 80);
    mockClientRect(label, 330, 300, 300, 80);
    act(() => {
      pointerClick(disposition);
    });
    const handle = disposition.querySelector(".presentation-interaction-handle--se") as HTMLElement;
    act(() => {
      pointerDrag(handle, 620, 370, 720, 420);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    expect(parseFloat(disposition.style.width)).toBeGreaterThan(15);
    expect(parseFloat(disposition.style.height)).toBeGreaterThan(5);
    expect(disposition.querySelector<HTMLElement>(".presentation-interactive-disposition__content")?.style.transform).toContain("scale(");
  });

  it("toggles enlarge on an intro flow disposition", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A", "B", "C"], short: "Short" },
      description: { full: ["D1"], short: "D short" },
      goal: ["G1"],
      authors: { lines: [[{ name: "Alice" }]] },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
          ],
        ],
      },
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const section = container.querySelector('.slides > section > section[title="title"]') as HTMLElement;
    const stack = section.parentElement as HTMLElement;
    section.classList.add("present");
    stack.classList.add("present");
    Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(stack, "offsetHeight", { value: 700, configurable: true });
    Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
    Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
    mockClientRect(stack, 0, 0, 960, 700);
    mockClientRect(section, 0, 0, 960, 700);
    const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
    const heading = disposition.querySelector("h2") as HTMLElement;
    mockClientRect(disposition, 330, 300, 300, 80);
    mockClientRect(heading, 330, 300, 300, 80);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeButton).toBeTruthy();
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(parseFloat(disposition.style.height)).toBeCloseTo(SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100);
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    expect(content.style.transform).toMatch(/^scale\(/);
    expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
    const resetButton = disposition.querySelector(".presentation-interaction-reset") as HTMLButtonElement;
    expect(resetButton).toBeTruthy();
    act(() => {
      resetButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(content.style.transform).toBe("");
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(globalsCssSource).toMatch(/\.presentation-arrangement-surface\s*>\s*\.presentation-interactive-disposition--enlarged/s);
  });
});
