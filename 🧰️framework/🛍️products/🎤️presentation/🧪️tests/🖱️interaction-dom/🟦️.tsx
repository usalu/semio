/** 🖱️ The presentation react deck, interaction DOM: selecting, dragging, resizing, enlarging and resetting dispositions on a mounted deck.
 *
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { collectPresentationSlides, intro, split } from "@semio-tech/presentation";
import Reveal from "reveal.js";
import type { PdfCanvasDocument, PdfCanvasPort, Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME, mountPresentation, pdfScrollCoverScale, presentationAutoAnimateMatcher, resetPdfCanvasPort, setPdfCanvasPort, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource, stockRevealAutoAnimateMatcherHost } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("presentation interaction dom", () => {
  let container: HTMLDivElement;

  const testPdfCanvasPort: PdfCanvasPort = {
    load() {
      const document: PdfCanvasDocument = {
        numPages: 3,
        getPage: async () => ({
          getViewport: ({ scale }) => ({ width: 595 * scale, height: 842 * scale }),
          render: () => ({ promise: Promise.resolve(), cancel: () => undefined }),
          cleanup: () => undefined,
        }),
        destroy: () => undefined,
      };
      return { promise: Promise.resolve(document), destroy: () => undefined };
    },
  };

  const positionedDeck: Presentation = {
    id: "interactive-dom",
    name: "Interactive DOM",
    chapters: [
      {
        id: "main",
        sequences: [
          {
            id: "main",
            thoughts: [
              {
                id: "placed",
                participants: [{ id: "box" }],
                embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
                slides: [
                  {
                    arrangement: {
                      id: "placed",
                      dispositions: [
                        {
                          participantId: "box",
                          embodimentId: "box--main",
                          emphasis: "active",
                          position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
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

  const twoSlideDeck: Presentation = {
    id: "interactive-two-slide",
    name: "Interactive Two Slide",
    chapters: [
      {
        id: "main",
        sequences: [
          {
            id: "main",
            thoughts: [
              {
                id: "pair",
                participants: [{ id: "box" }],
                embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
                slides: [
                  {
                    arrangement: {
                      id: "alpha",
                      dispositions: [
                        {
                          participantId: "box",
                          embodimentId: "box--main",
                          emphasis: "active",
                          position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
                        },
                      ],
                    },
                  },
                  {
                    arrangement: {
                      id: "beta",
                      dispositions: [
                        {
                          participantId: "box",
                          embodimentId: "box--main",
                          emphasis: "active",
                          position: { x: 0.55, y: 0.55, width: 0.3, height: 0.15 },
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

  const pointerClick = (target: Element, clientX = 20, clientY = 20): void => {
    target.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, cancelable: true, button: 0, clientX, clientY }));
    target.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, cancelable: true, button: 0, clientX, clientY }));
  };

  const mockClientRect = (element: Element, left: number, top: number, width: number, height: number): void => {
    const rect = {
      left,
      top,
      width,
      height,
      right: left + width,
      bottom: top + height,
      x: left,
      y: top,
      toJSON: () => ({}),
    };
    element.getBoundingClientRect = () => rect as DOMRect;
  };

  const waitForPdfCanvas = async (root: Element, predicate: (canvas: HTMLCanvasElement) => boolean): Promise<HTMLCanvasElement> => {
    for (let attempt = 0; attempt < 40; attempt += 1) {
      const canvas = root.querySelector<HTMLCanvasElement>(".presentation-media-pdf canvas");
      if (canvas && predicate(canvas)) {
        return canvas;
      }
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
    }
    throw new Error("PDF canvas did not reach the expected state.");
  };

  const pointerDrag = (target: Element, fromX: number, fromY: number, toX: number, toY: number, pointerId = 1): void => {
    target.dispatchEvent(
      new PointerEvent("pointerdown", {
        bubbles: true,
        cancelable: true,
        button: 0,
        clientX: fromX,
        clientY: fromY,
        pointerId,
      }),
    );
    window.dispatchEvent(
      new PointerEvent("pointermove", {
        bubbles: true,
        cancelable: true,
        clientX: toX,
        clientY: toY,
        pointerId,
      }),
    );
    window.dispatchEvent(
      new PointerEvent("pointerup", {
        bubbles: true,
        cancelable: true,
        clientX: toX,
        clientY: toY,
        pointerId,
      }),
    );
  };

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

  it("renders data-disposition-id on every disposition", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    expect(container.querySelectorAll("[data-disposition-id]").length).toBe(1);
  });

  it("renders one interactive wrapper per tile disposition", () => {
    const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
    const deck: Presentation = {
      id: "split-interactive",
      name: "Split Interactive",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "split",
                  participants: grid.participants,
                  embodiments: grid.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: grid.dispositions,
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
    expect(container.querySelectorAll("[data-disposition-id]").length).toBe(4);
    expect(container.querySelectorAll(".presentation-interactive-row-band").length).toBe(0);
  });

  it("selects on click and deselects on empty slide click", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
    act(() => {
      pointerClick(layer);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    act(() => {
      pointerClick(canvas, 8, 8);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    const reveal = container.querySelector(".reveal") as HTMLElement;
    const background = document.createElement("div");
    background.className = "slide-background present";
    reveal.appendChild(background);
    act(() => {
      pointerClick(background, 8, 8);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
  });

  it("keeps selection when empty slide click started in reveal overview", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const reveal = container.querySelector(".reveal") as HTMLElement;
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    reveal.classList.add("overview");
    act(() => {
      pointerClick(layer);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    reveal.classList.remove("overview");
    act(() => {
      pointerClick(layer);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
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

  it("drags positioned disposition", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    // 🔀️ The wrapper owns the reveal `data-id` morph anchor, so the ephemeral drag must move the
    // wrapper frame itself (not just translate inner content). Auto-animate then morphs from the
    // dragged frame, including the ephemeral modification. The declared frame is centered first
    // (single box shifts +0.1/+0.1), then the 80px/40px drag adds +8.333%/+5.714%.
    expect(parseFloat(disposition.style.left)).toBeCloseTo(38.333, 1);
    expect(parseFloat(disposition.style.top)).toBeCloseTo(45.714, 1);
    expect(content.style.transform).toBe("");
  });

  it("keeps other canvas dispositions on their declared frames while one is dragged", () => {
    const twoBoxDeck: Presentation = {
      id: "interactive-dom-two",
      name: "Interactive DOM Two",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "placed",
                  participants: [{ id: "left" }, { id: "right" }],
                  embodiments: [
                    { kind: "text", id: "left--main", lines: ["Left"], level: "body" },
                    { kind: "text", id: "right--main", lines: ["Right"], level: "body" },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "placed",
                        dispositions: [
                          {
                            participantId: "left",
                            embodimentId: "left--main",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
                          },
                          {
                            participantId: "right",
                            embodimentId: "right--main",
                            emphasis: "active",
                            position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
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
      mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
    expect(dispositions).toHaveLength(2);
    const section = dispositions[0]!.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(dispositions[0]!, 96, 210, 288, 140);
    mockClientRect(dispositions[1]!, 576, 210, 288, 140);
    const peerBefore = dispositions[1]!.style.left;
    act(() => {
      pointerDrag(dispositions[0]!, 240, 280, 400, 320);
    });
    // 🔀️ Dragged disposition's wrapper (its morph anchor) follows the ephemeral frame...
    expect(parseFloat(dispositions[0]!.style.left)).toBeCloseTo(26.667, 1);
    // ...while peers keep their declared frames.
    expect(dispositions[1]!.style.left).toBe(peerBefore);
    expect(dispositions[0]!.querySelector<HTMLElement>(".presentation-interactive-disposition__content")?.style.transform).toBe("");
  });

  it("click-drags only the newly targeted disposition when another stays selected", () => {
    const twoBoxDeck: Presentation = {
      id: "interactive-dom-click-drag",
      name: "Interactive DOM Click Drag",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "placed",
                  participants: [{ id: "left" }, { id: "right" }],
                  embodiments: [
                    { kind: "text", id: "left--main", lines: ["Left"], level: "body" },
                    { kind: "text", id: "right--main", lines: ["Right"], level: "body" },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "placed",
                        dispositions: [
                          {
                            participantId: "left",
                            embodimentId: "left--main",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
                          },
                          {
                            participantId: "right",
                            embodimentId: "right--main",
                            emphasis: "active",
                            position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
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
      mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
    const section = dispositions[0]!.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(dispositions[0]!, 96, 210, 288, 140);
    mockClientRect(dispositions[1]!, 576, 210, 288, 140);
    act(() => {
      pointerClick(dispositions[0]!, 240, 280);
    });
    const leftBefore = dispositions[0]!.style.left;
    const rightBefore = dispositions[1]!.style.left;
    act(() => {
      pointerDrag(dispositions[1]!, 720, 280, 880, 320);
    });
    expect(dispositions[0]!.style.left).toBe(leftBefore);
    expect(dispositions[1]!.style.left).not.toBe(rightBefore);
  });

  it("shows tile disposition drag preview outside the declared frame without clipping", () => {
    const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
    const splitDeck: Presentation = {
      id: "interactive-dom-split-tiles",
      name: "Interactive DOM Split Tiles",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "tiles",
                  participants: grid.participants,
                  embodiments: grid.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: grid.dispositions,
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
      mountPresentation(container, splitDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const tile = container.querySelector(".presentation-arrangement-canvas > .presentation-interactive-disposition[data-disposition-id]") as HTMLElement;
    expect(tile).toBeTruthy();
    const section = tile.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(tile, 96, 70, 384, 210);
    act(() => {
      pointerDrag(tile, 200, 140, 360, 260);
    });
    // 🔀️ The tile wrapper (morph anchor) moves to the ephemeral frame so the morph starts there.
    expect(parseFloat(tile.style.left)).toBeCloseTo(26.667, 1);
    expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
    expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    // Pinned keeps the dragged tile unclipped while it sits outside the declared frame.
    expect(getComputedStyle(tile).overflow).not.toBe("hidden");
    expect(tile.querySelector<HTMLElement>(".presentation-interactive-disposition__content")?.style.transform).toBe("");
  });

  it("resizes positioned disposition from se handle", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    act(() => {
      pointerClick(disposition);
    });
    const handle = disposition.querySelector(".presentation-interaction-handle--se") as HTMLElement;
    act(() => {
      pointerDrag(handle, 560, 340, 640, 400);
    });
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    // 🔀️ Resize grows the wrapper frame (the morph anchor) itself; the content fills it at 100%
    // rather than carrying a scaled inline size, so auto-animate morphs from the resized frame.
    expect(parseFloat(disposition.style.width)).toBeCloseTo(48.333, 1);
    expect(parseFloat(disposition.style.height)).toBeCloseTo(28.571, 1);
    expect(content.style.width).toBe("");
    expect(content.style.height).toBe("");
    const chrome = disposition.querySelector(".presentation-interactive-disposition__chrome") as HTMLElement;
    expect(chrome.isConnected).toBe(true);
    expect(chrome.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
  });

  it("aligns canvas-framed chrome with the disposition wrapper on the arrangement canvas", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
    mockClientRect(section, 0, 0, 1200, 900);
    mockClientRect(canvas, 120, 100, 960, 700);
    const frameBox = { left: 408, top: 380, width: 384, height: 140 };
    mockClientRect(disposition, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
    mockClientRect(frame, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
    const chrome = disposition.querySelector(".presentation-interactive-disposition__chrome") as HTMLElement;
    mockClientRect(chrome, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
    const chromeRect = chrome.getBoundingClientRect();
    const wrapperRect = disposition.getBoundingClientRect();
    expect(chromeRect.left).toBeCloseTo(wrapperRect.left, 0);
    expect(chromeRect.top).toBeCloseTo(wrapperRect.top, 0);
    expect(chromeRect.width).toBeCloseTo(wrapperRect.width, 0);
    expect(chromeRect.height).toBeCloseTo(wrapperRect.height, 0);
  });

  it("toggles enlarge on a canvas-framed disposition", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
    const enlargeOn = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeOn.getAttribute("aria-pressed")).toBe("true");
    act(() => {
      enlargeOn.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    const enlargeOff = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeOff.getAttribute("aria-pressed")).toBe("false");
  });

  it("restores pre-enlarge frame after exit enlarge following drag", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    act(() => {
      pointerClick(disposition);
    });
    const declaredLeft = parseFloat(disposition.style.left);
    act(() => {
      pointerDrag(disposition, 300, 280, 400, 320);
    });
    const draggedLeft = parseFloat(disposition.style.left);
    expect(draggedLeft).not.toBeCloseTo(declaredLeft, 1);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(parseFloat(disposition.style.left)).toBeCloseTo(draggedLeft, 1);
    expect(parseFloat(disposition.style.left)).not.toBeCloseTo(declaredLeft, 1);
  });

  it("exits enlarge through the corner control while slide reset host is active", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
  });

  it("enlarges when pointerdown lands on the svg icon inside the enlarge button", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    const icon = enlargeButton.querySelector("svg");
    expect(icon).toBeTruthy();
    act(() => {
      pointerClick(icon!);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
  });

  it("keeps enlarge when empty slide click clears selection", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    act(() => {
      pointerClick(canvas, 8, 8);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    const enlargeWhileDeselected = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeWhileDeselected).toBeTruthy();
    act(() => {
      enlargeWhileDeselected.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
  });

  it("clears drag and selection after navigating away and back", async () => {
    const alphaRef = collectPresentationSlides(twoSlideDeck)[0];
    const betaRef = collectPresentationSlides(twoSlideDeck)[1];
    expect(alphaRef?.slide).toBe("alpha");
    expect(betaRef?.slide).toBe("beta");
    let revealApi: Reveal.Api | undefined;
    act(() => {
      mountPresentation(container, twoSlideDeck, {
        hash: false,
        slideNumber: false,
        surfaceChrome: false,
        onRevealReady: (api) => {
          revealApi = api;
        },
      });
    });
    await new Promise<void>((resolve) => {
      const start = Date.now();
      const wait = (): void => {
        if (revealApi) {
          resolve();
          return;
        }
        if (Date.now() - start > 5000) {
          throw new Error("reveal.js did not become ready.");
        }
        setTimeout(wait, 50);
      };
      wait();
    });
    const alphaSlide = container.querySelector('section[title="alpha"]') as HTMLElement;
    const disposition = alphaSlide.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    const originLeft = disposition.style.left;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    const modifiedLeft = disposition.style.left;
    expect(modifiedLeft).not.toBe(originLeft);
    await revealApi!.slide(betaRef!.h, betaRef!.v);
    await new Promise((resolve) => setTimeout(resolve, 50));
    await revealApi!.slide(alphaRef!.h, alphaRef!.v);
    await new Promise((resolve) => setTimeout(resolve, 50));
    const alphaSlideAgain = container.querySelector('section[title="alpha"]') as HTMLElement;
    const dispositionAgain = alphaSlideAgain.querySelector("[data-disposition-id]") as HTMLElement;
    expect(dispositionAgain.style.left).toBe(originLeft);
    expect(dispositionAgain.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
  });

  it("resets the whole slide from the proximity control in the top-right corner", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    expect(globalsCssSource).toMatch(/\.presentation-interaction-slide-reset-host\s*\{[^}]*pointer-events:\s*none/s);
    expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition\s*>\s*\.presentation-interaction-actions\s*\{[^}]*z-index:\s*90/s);
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    const originLeft = disposition.style.left;
    expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    expect(disposition.style.left).not.toBe(originLeft);
    const slideResetHost = section.querySelector(".presentation-interaction-slide-reset-host") as HTMLElement;
    const slideReset = slideResetHost.querySelector(".presentation-interaction-slide-reset") as HTMLButtonElement;
    expect(slideResetHost).toBeTruthy();
    expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(false);
    act(() => {
      window.dispatchEvent(new PointerEvent("pointermove", { bubbles: true, clientX: 920, clientY: 20, pointerId: 2 }));
    });
    expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(true);
    act(() => {
      slideReset.click();
    });
    expect(disposition.style.left).toBe(originLeft);
    expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
  });

  it("resets a dragged canvas-framed disposition to its declared position", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    const originLeft = disposition.style.left;
    const originTop = disposition.style.top;
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    expect(disposition.style.left).not.toBe(originLeft);
    expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
    const reset = disposition.querySelector(".presentation-interaction-reset") as HTMLButtonElement;
    expect(reset).toBeTruthy();
    const actions = disposition.querySelector(".presentation-interaction-actions")!;
    const buttons = [...actions.querySelectorAll("button")];
    expect(buttons[0]?.classList.contains("presentation-interaction-reset")).toBe(true);
    expect(buttons[1]?.classList.contains("presentation-interaction-enlarge")).toBe(true);
    act(() => {
      reset.click();
    });
    expect(disposition.style.left).toBe(originLeft);
    expect(disposition.style.top).toBe(originTop);
    expect(disposition.querySelector(".presentation-interaction-reset")).toBeNull();
  });

  it("scales pdf pages to cover the disposition frame", async () => {
    const deck: Presentation = {
      id: "pdf-cover",
      name: "Pdf Cover",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(frame, 96, 385, 768, 280);
    const expected = pdfScrollCoverScale(768, 280, 595, 842);
    expect(expected).not.toBeNull();
    const page = await waitForPdfCanvas(disposition, (canvas) => Math.abs(Number(canvas.dataset.scale) - (expected ?? 0)) < 0.01);
    expect(Number(page.dataset.scale)).toBeCloseTo(expected ?? 0);
  });

  it("drops nested pdf frame and uses enlarged slide sizing when toggling enlarge", async () => {
    const deck: Presentation = {
      id: "pdf-enlarge",
      name: "Pdf Enlarge",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 96, 385, 768, 280);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.querySelector(".presentation-disposition-frame")).toBeTruthy();
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    mockClientRect(content, 48, 52, 864, 595);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
    expect(disposition.querySelector(".presentation-disposition-frame")).toBeNull();
    const pageCanvas = await waitForPdfCanvas(disposition, (canvas) => canvas.height > 400);
    expect(pageCanvas.height).toBeGreaterThan(400);
    expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition--kind-pdf\.presentation-interactive-disposition--enlarged[\s\S]*\.presentation-media-pdf-document[\s\S]*height\s*:\s*100%/s);
  });

  it("shows center-bottom pdf page nav when enlarged and switches pages", async () => {
    const deck: Presentation = {
      id: "pdf-page-nav",
      name: "Pdf Page Nav",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 96, 385, 768, 280);
    act(() => {
      pointerClick(disposition);
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(disposition.querySelector(".presentation-pdf-page-nav")).toBeTruthy();
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    mockClientRect(content, 48, 52, 864, 595);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    const nav = disposition.querySelector(".presentation-pdf-page-nav");
    expect(nav).toBeTruthy();
    const page = disposition.querySelector(".presentation-media-pdf canvas") as HTMLElement;
    expect(page.dataset.page).toBe("1");
    const nextButton = disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
    const prevButton = disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
    expect(prevButton.disabled).toBe(true);
    expect(nextButton.disabled).toBe(false);
    act(() => {
      nextButton.click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(disposition.querySelector(".presentation-media-pdf canvas")?.getAttribute("data-page")).toBe("2");
    expect((disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement).disabled).toBe(false);
  });

  it("navigates only within pdf pages declared on the embodiment", async () => {
    const deck: Presentation = {
      id: "pdf-page-subset",
      name: "Pdf Page Subset",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [
                    {
                      kind: "pdf",
                      id: "thesis--doc",
                      src: "/thesis.pdf",
                      page: 1,
                      pages: [1, 12, 25],
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(disposition, 96, 385, 768, 280);
    act(() => {
      pointerClick(disposition);
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    const nextButton = () => disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
    const prevButton = () => disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
    const pageNumber = () => disposition.querySelector(".presentation-media-pdf canvas")?.getAttribute("data-page");
    expect(pageNumber()).toBe("1");
    act(() => {
      const next = nextButton();
      pointerClick(next);
      next.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--gesturing")).toBe(false);
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(pageNumber()).toBe("12");
    act(() => {
      const next = nextButton();
      pointerClick(next);
      next.click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(pageNumber()).toBe("25");
    expect(nextButton().disabled).toBe(true);
    act(() => {
      prevButton().click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(pageNumber()).toBe("12");
  });

  it("toggles enlarge on a cropped figure tile disposition", () => {
    const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
    const deck: Presentation = {
      id: "split-enlarge",
      name: "Split Enlarge",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "split",
                  participants: grid.participants,
                  embodiments: grid.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: grid.dispositions,
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
    const tile = container.querySelector("[data-disposition-id]") as HTMLElement;
    const canvas = tile.closest(".presentation-arrangement-canvas") as HTMLElement;
    act(() => {
      pointerClick(tile);
    });
    const enlargeButton = tile.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
    expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
    expect(tile.style.position).toBe("absolute");
    expect(tile.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    expect(tile.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
    expect(canvas.contains(tile)).toBe(true);
    const cropSlot = tile.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
    expect(cropSlot).toBeTruthy();
    expect(tile.querySelector(".presentation-media-figure")).toBeNull();
    expect(cropSlot?.style.backgroundImage).toContain("catalogue.png");
    const bgSize = cropSlot?.style.getPropertyValue("--presentation-figure-bg-size");
    expect(bgSize).toBeTruthy();
    expect(bgSize).not.toBe("100% 100%");
    expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition--enlarged:not\(\.presentation-interactive-disposition--offset\)[\s\S]*\.presentation-figure-crop-fill[\s\S]*width\s*:\s*100%\s*!important/s);
    act(() => {
      enlargeButton.click();
    });
    expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
  });
});
