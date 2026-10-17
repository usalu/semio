/** 🎥️ The presentation react deck, rendering of media and tiles: video and pdf embodiments, teaser veils, absolute positions, split figure tiles, catalogue morph anchors and listed dispositions. */
import { remapSplitDispositions, split } from "@semio-tech/presentation";
import type { AutoAnimateMatcherHost, Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { mountPresentation, presentationAutoAnimateMatcher, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";

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

  it("renders video and pdf embodiments with data-id for auto-animate", () => {
    const deck: Presentation = {
      id: "media-test",
      name: "Media",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "media",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "clip" }, { id: "doc" }],
                  embodiments: [
                    { kind: "video", id: "clip--video", src: "/demo.mp4" },
                    { kind: "pdf", id: "doc--pdf", src: "/paper.pdf", page: 1 },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "slide",
                        dispositions: [
                          { participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
                          { participantId: "doc", embodimentId: "doc--pdf", emphasis: "active" },
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
    expect(container.querySelector('[data-id="clip"] video[src="/demo.mp4"]')).toBeTruthy();
    expect(container.querySelector('[data-id="doc"] .presentation-media-pdf-document')).toBeTruthy();
  });

  it("renders glassy teaser veils on media embodiments with optional labels", () => {
    const deck: Presentation = {
      id: "teaser-test",
      name: "Teaser",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "teaser",
              thoughts: [
                {
                  id: "teaser",
                  participants: [{ id: "fig" }, { id: "clip" }, { id: "embed" }],
                  embodiments: [
                    { kind: "figure", id: "fig--figure", src: "/a.png", teaser: {} },
                    {
                      kind: "video",
                      id: "clip--video",
                      src: "/demo.mp4",
                      teaser: { label: "Coming soon" },
                    },
                    {
                      kind: "iframe",
                      id: "embed--iframe",
                      src: "/demo.html",
                      teaser: { label: "Interactive demo" },
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "slide",
                        dispositions: [
                          { participantId: "fig", embodimentId: "fig--figure", emphasis: "active" },
                          { participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
                          { participantId: "embed", embodimentId: "embed--iframe", emphasis: "active" },
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
    const figureVeil = container.querySelector('[data-id="fig"] .presentation-media-teaser__veil');
    const videoLabel = container.querySelector('[data-id="clip"] .presentation-media-teaser__label');
    const iframeVeil = container.querySelector('[data-id="embed"] .presentation-media-teaser__veil');
    expect(figureVeil).toBeTruthy();
    expect(videoLabel?.textContent).toBe("Coming soon");
    expect(iframeVeil?.querySelector(".presentation-media-teaser__label")?.textContent).toBe("Interactive demo");
    expect(container.querySelector('[data-id="embed"] iframe.presentation-media-iframe')).toBeTruthy();
  });

  it("applies absolute positioning for dispositions with position", () => {
    const deck: Presentation = {
      id: "position-test",
      name: "Position",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "pos",
              thoughts: [
                {
                  id: "pos",
                  participants: [{ id: "box" }],
                  embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
                  slides: [
                    {
                      arrangement: {
                        id: "placed",
                        dispositions: [
                          {
                            participantId: "box",
                            embodimentId: "box--main",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.2, width: 0.5, height: 0.3 },
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
    const frame = container.querySelector(".presentation-disposition-frame") as HTMLElement | null;
    expect(frame?.style.position).toBe("absolute");
    expect(frame?.style.left).toBe("25%");
    expect(frame?.style.width).toBe("50%");
  });

  it("renders split figure tiles with per-participant data-id and background crops", () => {
    const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
    const artifacts = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
    const deck: Presentation = {
      id: "split-figure",
      name: "Split",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "split",
                  participants: artifacts.participants,
                  embodiments: artifacts.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: artifacts.dispositions,
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
    const wrappers = container.querySelectorAll("[data-disposition-id]");
    expect(wrappers.length).toBe(4);
    const first = wrappers[0] as HTMLElement;
    expect(first.classList.contains("presentation-interactive-disposition")).toBe(true);
    expect(first.style.position).toBe("absolute");
    const tileFrame = first.querySelector('.presentation-morph-slot--figure[data-id^="tile-r"]') as HTMLElement;
    expect(tileFrame).toBeTruthy();
    expect(tileFrame.style.backgroundImage).toContain("/catalogue.png");
    expect(tileFrame.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
    const tiles = [...container.querySelectorAll(".presentation-morph-slot--figure")] as HTMLElement[];
    for (const node of tiles) {
      expect(node.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
    }
    const positions = tiles.map((node) => node.style.getPropertyValue("--presentation-figure-bg-position"));
    expect(new Set(positions).size).toBe(4);
    expect(first.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
  });

  it("pairs catalogue tile morph anchors on interactive wrappers across slides", () => {
    const frameA = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
    const gridA = split({ source: "/catalogue.png", rows: 2, columns: 2, frame: frameA });
    const frameB = { x: 0.1, y: 0.15, width: 0.35, height: 0.3 };
    const positionsB = Object.fromEntries(
      gridA.dispositions.map((disposition, index) => [
        disposition.participantId,
        {
          x: frameB.x + (index % 2) * (frameB.width / 2),
          y: frameB.y + Math.floor(index / 2) * (frameB.height / 2),
          width: frameB.width / 2,
          height: frameB.height / 2,
        },
      ]),
    );
    const gridB = {
      ...gridA,
      dispositions: remapSplitDispositions(gridA.dispositions, positionsB),
    };
    const deck: Presentation = {
      id: "tile-morph",
      name: "Tile morph",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "media",
              thoughts: [
                {
                  id: "media",
                  participants: gridA.participants,
                  embodiments: gridA.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "catalogue",
                        dispositions: gridA.dispositions,
                      },
                      transition: { kind: "morph" },
                    },
                    {
                      arrangement: {
                        id: "catalogue-focus",
                        dispositions: gridB.dispositions,
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
    const sections = [...container.querySelectorAll("section[data-auto-animate-id]")] as HTMLElement[];
    expect(sections.length).toBeGreaterThanOrEqual(2);
    const fromSlide = sections.find((section) => section.title === "catalogue");
    const toSlide = sections.find((section) => section.title === "catalogue-focus");
    expect(fromSlide).toBeDefined();
    expect(toSlide).toBeDefined();
    const pairs = presentationAutoAnimateMatcher.call({ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost, fromSlide!, toSlide!);
    expect(pairs).toHaveLength(4);
    expect(pairs.every((pair) => pair.from.getAttribute("data-id") === pair.to.getAttribute("data-id"))).toBe(true);
    expect(pairs[0]?.from.classList.contains("presentation-interactive-disposition")).toBe(true);
    expect(pairs[0]?.to.classList.contains("presentation-interactive-disposition")).toBe(true);
  });

  it("renders only the dispositions listed on a slide", () => {
    const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
    const deck: Presentation = {
      id: "partial-split",
      name: "Partial",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "partial",
                  participants: grid.participants,
                  embodiments: grid.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "focus",
                        dispositions: grid.dispositions.slice(0, 2),
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
    expect(container.querySelectorAll('[data-id^="tile-r"]').length).toBe(2);
  });
});
