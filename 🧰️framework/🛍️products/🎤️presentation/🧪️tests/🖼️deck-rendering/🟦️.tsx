/** 🖼️ The presentation react deck, rendering: mounting `PresentationDeck`, intro morph DOM, auto-animate sheets, figure crop, zoom and scroll math, pdf subsets and ghost anchors.
 *
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { buildResolutionScope, intro, remapSplitDispositions, split, splitFigureGrid } from "@semio-tech/presentation";
import type { AutoAnimateMatcherHost, PdfEmbodiment, Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { FIGURE_WHEEL_ZOOM_MAX, PRESENTATION_MANY_TO_ONE_MORPH_CLASS, clearRevealAutoAnimateInlineLayout, figureBackgroundSizeScrollAxis, figureBackgroundSizeZoomed, figureCoverOverflowAxis, figureCoverScrollContentSize, figureCoverScrollElementStyle, figureCropBackgroundPosition, figureCropBackgroundSize, figureCropBackgroundVars, figureCropScrollBackgroundSize, figureEmbodimentScrollEnabled, figureScrollOffsetForBackgroundPosition, figureScrollOverlayThumbMetrics, figureWheelZoomStep, finalizeRevealAutoAnimateRestState, isManyToOneMorphTransition, isRevealSlideAutoAnimating, isSectionAutoAnimating, mediaTeaserActive, mosaicWindowedCoverVars, mountPresentation, patchAutoAnimateUniformScale, patchPresentationAutoAnimateRunStyleSheet, patchPresentationAutoAnimateStyleSheet, pdfAdjacentPage, pdfCanGoToNextPage, pdfCanGoToPreviousPage, pdfCoverScale, pdfEmbodimentInitialPage, pdfPageNavEnabled, pdfScrollCoverScale, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher, readPresentationSlideIndicesFromUrl, relaxHiddenPreflight, resolvePresentationAssetUrl, resolvePresentationAutoAnimateRunSlides, resolveRevealArrangement, slidesShareAutoAnimateId, syncArrangementSettledState, syncManyToOneGhostMorphFramesFromDom, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { source, stylesheetRelativePath, globalsCssSource } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("PresentationDeck", () => {
  let container: HTMLDivElement;

  const testAffiliationSteps = {
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
  } as const;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
  });

  afterEach(() => {
    unmountPresentation();
    container.remove();
  });

  it("renders expanded intro slides", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false });
    });
    const sections = container.querySelectorAll(".slides > section > section");
    expect(sections.length).toBe(7);
    expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
    const revealEl = container.querySelector(".reveal");
    expect(revealEl?.getAttribute("style")).toContain("100vw");
    expect(container.querySelector('[data-id^="title"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="description"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="goal"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="authors--"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="institutions--"]')).toBeTruthy();
  });

  it("keeps intro flow slides centered while enabling interactive dispositions", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
      expect(slide.classList.contains("presentation-arrangement--interactive")).toBe(true);
      expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
      expect(slide.classList.contains("presentation-arrangement--positioned")).toBe(false);
      expect(slide.querySelector(".presentation-arrangement-canvas")).toBeNull();
      expect(slide.querySelectorAll("[data-disposition-id]").length).toBeGreaterThan(0);
    }
    expect(globalsCssSource).toMatch(/\.presentation-arrangement--interactive\s*\{[^}]*overflow\s*:\s*visible/s);
    expect(globalsCssSource).toMatch(/\.presentation-arrangement-canvas\s*>\s*\.presentation-interactive-disposition\[data-id\][\s\S]*:not\(\s*\.presentation-interactive-disposition--gesturing\s*\)[\s\S]*overflow\s*:\s*hidden/s);
    expect(globalsCssSource).not.toMatch(/\.presentation-arrangement--interactive\s*\{[^}]*position\s*:\s*relative/s);
  });

  it("applies muted opacity on layered description slide", () => {
    const deck = intro({ language: "de", title: { full: ["A"], short: "Short" }, description: { full: ["D"], short: "D short" }, goal: ["G"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const descriptionSlide = container.querySelector('.slides > section > section[title="description"]');
    expect(descriptionSlide?.querySelector(".opacity-20")).toBeTruthy();
  });

  it("matches eg-ice-25 intro morph DOM per arrangement", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A", "B", "C"], short: "Short" },
      description: { full: ["D1", "D2"], short: "D short" },
      goal: ["G1"],
      authors: {
        lines: [
          [
            { name: "Alice Example", marks: ["a", "1", "x"] },
            { name: "Bob Beta", marks: ["a", "1", "x"] },
          ],
          [{ name: "Carol Creator" }],
        ],
      },
      affiliations: testAffiliationSteps,
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
    expect(slide("title")?.querySelectorAll('h2[data-id^="title"]').length).toBe(3);
    expect(slide("description")?.querySelector('h2[data-id="title"]')).toBeTruthy();
    expect(slide("description")?.querySelector('div[data-id="title"].presentation-disposition-frame')).toBeNull();
    expect(slide("description")?.querySelectorAll('h2[data-id^="description"]').length).toBe(2);
    expect(slide("goal")?.querySelector('h2[data-id="description"]')?.textContent).toBe("D short");
    expect(slide("goal")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
    expect(slide("description")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
    expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
    const authorLines = slide("authors")?.querySelectorAll('h4[data-id^="authors--"]');
    expect(authorLines?.length).toBe(3);
    expect(slide("authors")?.getAttribute("data-auto-animate-id")).toMatch(/^einleitung--/);
    expect(slide("authors")?.querySelector(".presentation-intro-line")?.className).toContain("gap-x-");
    expect(slide("affiliations-1")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(1);
    expect(slide("affiliations-2")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
    expect(slide("affiliations-3")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
    expect(slide("affiliations-3")?.querySelectorAll('[data-id^="institutions--"]').length).toBe(3);
    expect(slide("affiliations-2")?.querySelector('h5[data-id="institutions"]')).toBeNull();
    expect(slide("affiliations-2")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("Uni");
    expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
    expect(slide("affiliations-3")?.textContent).toContain("Chair X");
    expect(slide("affiliations-3")?.querySelector(".presentation-affiliation-morph-source")).toBeNull();
    expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.classList.contains("presentation-affiliation-row")).toBe(true);
    expect(slide("affiliations-3")?.querySelector('[data-id="institutions--x"]')?.textContent).toContain("Chair X");
    expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"] [data-id="institutions--x"]')).toBeTruthy();
    expect(slide("affiliations-1")?.querySelector('h4[data-id="authors--Alice Example"] sup')?.textContent).toBe("a");
    const marked2 = slide("affiliations-2")?.querySelector('h4[data-id="authors--Alice Example"] sup');
    expect(marked2?.textContent).toBe("a,1");
    expect(marked2?.querySelector("span:not(.opacity-20)")?.textContent).toBe("1");
    const marked3 = slide("affiliations-3")?.querySelector('h4[data-id="authors--Alice Example"] sup');
    expect(marked3?.textContent).toBe("a,1,x");
    expect(marked3?.querySelector("span:not(.opacity-20)")?.textContent).toBe("x");
    expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("A. Example");
    expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeTruthy();
    expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeNull();
    expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("Alice Example");
    const aff2 = slide("affiliations-2");
    expect(aff2?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
    expect(aff2?.querySelector('h4[data-id="institutions--1"] .opacity-20')).toBeNull();
    const aff3 = slide("affiliations-3");
    expect(aff3?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
    expect(aff3?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
    expect(aff3?.querySelector('[data-id="institutions--x"] .opacity-20')).toBeNull();
  });

  it("applies title and secondary morph text sizes on intro slides", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A", "B", "C"], short: "Short" },
      description: { full: ["D1", "D2"], short: "D short" },
      goal: ["G1"],
      authors: {
        lines: [[{ name: "Alice", marks: ["1"] }, { name: "Bob" }], [{ name: "Carol" }]],
      },
      affiliations: testAffiliationSteps,
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const expectMorphClass = (selector: string, sizeClass: string) => {
      for (const node of container.querySelectorAll(selector)) {
        expect(node.classList.contains("presentation-morph-text")).toBe(true);
        expect(node.classList.contains(sizeClass)).toBe(true);
      }
    };
    expectMorphClass('h1[data-id^="title"], h2[data-id^="title"]', "presentation-morph-text--title");
    expectMorphClass('h2[data-id^="description"], p[data-id^="description"], h2[data-id^="goal"], p[data-id^="goal"]', "presentation-morph-text--secondary");
    expect(container.querySelector('h2[data-id^="title"].presentation-morph-text--secondary')).toBeNull();
    expect(container.querySelector('h2[data-id^="goal"].presentation-morph-text--title')).toBeNull();
  });

  it("does not use reveal fit-text on intro headings", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B"], short: "Short" }, description: { full: ["D1"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    expect(container.querySelector(".r-fit-text")).toBeNull();
  });

  it("enables reveal auto-animate on intro morph slides with unmatched layering", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A"], short: "Short" },
      description: { full: ["D"], short: "D short" },
      goal: ["G1", "G2"],
      authors: {
        lines: [
          [
            { name: "Alice", marks: ["a"] },
            { name: "Bob", marks: ["a"] },
          ],
          [{ name: "Carol", marks: ["a"] }],
        ],
      },
      affiliations: testAffiliationSteps,
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
      expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
      expect(slide.hasAttribute("data-auto-animate")).toBe(true);
      expect(slide.querySelector(".presentation-arrangement-surface")).toBeNull();
    }
    expect(globalsCssSource).toMatch(/\.presentation-arrangement--intro:not\(\.presentation-arrangement--positioned\)[\s\S]*:is\(h1,\s*h2,\s*h3,\s*h4,\s*p\)[\s\S]*margin:\s*0/s);
    expect(globalsCssSource).not.toMatch(/data-auto-animate-id\^="intro--"/);
  });

  it("enables reveal auto-animate and tags every morph arrangement with data-auto-animate", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const morphSections = container.querySelectorAll('.slides > section > section[data-auto-animate][data-auto-animate-id="einleitung--m0"]');
    expect(morphSections.length).toBe(7);
    const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
    expect(slide("title")?.querySelector('h2[data-id^="title"]')).toBeTruthy();
    expect(slide("title")?.querySelector('[data-id^="description"]')).toBeNull();
    expect(slide("description")?.querySelector('h2[data-id^="description"]')).toBeTruthy();
    expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
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
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), stylesheetRelativePath), "utf8");
    expect(css).toContain("presentation-figure-crop-morph-from-rest");
    expect(css).toContain("presentation-figure-crop-morph-grid-to-focus");
    expect(css.match(/presentation-morph-crop-from[\s\S]*?presentation-figure-crop-morph-from-rest/)).toBeTruthy();
    expect(css).toContain("@keyframes presentation-target-ghost-frame");
    expect(css).toContain('section.presentation-arrangement--many-to-one-morph[data-auto-animate="running"]');
    expect(css).toContain(".presentation-morph-crop-from");
    expect(css).toContain(".presentation-morph-crop-to");
  });

  it("rests morph-source ghosts with opacity only so reveal can measure FLIP targets", async () => {
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const cssPath = resolve(dirname(fileURLToPath(source.url)), stylesheetRelativePath);
    const css = readFileSync(cssPath, "utf8");
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

  it("resolvePresentationAssetUrl maps deck-relative paths through the Vite base", () => {
    expect(resolvePresentationAssetUrl("/🖼️bauteilbörse.png")).toBe("/🖼️bauteilbörse.png");
    expect(resolvePresentationAssetUrl("./🖼️bauteilbörse.png")).toBe("/🖼️bauteilbörse.png");
  });

  it("keeps figure selection chrome as a translucent overlay, not a solid fill", async () => {
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), stylesheetRelativePath), "utf8");
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
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), stylesheetRelativePath), "utf8");
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

  it("clearRevealAutoAnimateInlineLayout removes only FLIP transform from auto-animate targets", () => {
    const deckEl = document.createElement("div");
    const target = document.createElement("div");
    target.dataset.autoAnimateTarget = "0";
    target.style.left = "65%";
    target.style.transform = "translate(10px, 20px) scale(2)";
    deckEl.appendChild(target);
    clearRevealAutoAnimateInlineLayout(deckEl);
    expect(target.style.left).toBe("65%");
    expect(target.style.transform).toBe("");
  });

  it("finalizeRevealAutoAnimateRestState clears running so morph sources rest hidden", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const slide = document.createElement("section");
    slide.classList.add("present");
    slide.setAttribute("data-auto-animate", "running");
    slide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
    const morphSource = document.createElement("div");
    morphSource.className = "presentation-target-ghost";
    morphSource.dataset.autoAnimateTarget = "0";
    slide.appendChild(morphSource);
    deckEl.appendChild(slide);
    document.body.appendChild(deckEl);
    finalizeRevealAutoAnimateRestState(deckEl);
    expect(slide.getAttribute("data-auto-animate")).toBe("");
    expect(slide.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
    expect(morphSource.hasAttribute("data-auto-animate-target")).toBe(false);
    deckEl.remove();
  });

  it("matches auto-animate targets only by data-id", () => {
    const fromSlide = document.createElement("section");
    fromSlide.innerHTML = '<div data-id="catalogue-col1" class="presentation-morph-slot--figure"><h2>Rippenplatte</h2></div>';
    const toSlide = document.createElement("section");
    toSlide.innerHTML = '<div data-id="catalogue-col1" class="presentation-morph-slot--label"><h2>Rippenplatte</h2></div>';
    const host: AutoAnimateMatcherHost = {
      findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
        for (const element of fromScope.querySelectorAll<HTMLElement>(selector)) {
          const key = serializer(element);
          const toElement = toScope.querySelector<HTMLElement>(`${selector}[data-id="${element.getAttribute("data-id")}"]`);
          if (toElement) {
            pairs.push({ from: element, to: toElement });
          }
        }
      },
    };
    const pairs = presentationAutoAnimateMatcher.call(host, fromSlide, toSlide);
    expect(pairs).toHaveLength(1);
    expect(pairs[0]?.from.getAttribute("data-id")).toBe("catalogue-col1");
    expect(pairs[0]?.from.classList.contains("presentation-morph-slot--figure")).toBe(true);
  });

  it("detects reveal slide auto-animate pending or running on section or deck", () => {
    const section = document.createElement("section");
    const deck = document.createElement("div");
    deck.className = "reveal";
    deck.appendChild(section);
    expect(isRevealSlideAutoAnimating(section)).toBe(false);
    expect(isSectionAutoAnimating(section)).toBe(false);
    section.setAttribute("data-auto-animate", "pending");
    expect(isSectionAutoAnimating(section)).toBe(true);
    expect(isRevealSlideAutoAnimating(section)).toBe(true);
    section.setAttribute("data-auto-animate", "");
    const other = document.createElement("section");
    other.setAttribute("data-auto-animate", "running");
    deck.appendChild(other);
    expect(isSectionAutoAnimating(section)).toBe(false);
    expect(isRevealSlideAutoAnimating(section)).toBe(true);
  });

  it("detects consecutive slides in the same auto-animate run", () => {
    const focus = document.createElement("section");
    focus.setAttribute("data-auto-animate-id", "medien--m0");
    const labels = document.createElement("section");
    labels.setAttribute("data-auto-animate-id", "medien--m0");
    const overview = document.createElement("section");
    overview.setAttribute("data-auto-animate-id", "medien--m1");
    expect(slidesShareAutoAnimateId(focus, labels)).toBe(true);
    expect(slidesShareAutoAnimateId(focus, overview)).toBe(false);
  });

  it("clears settled state when arriving on a slide and prepares it before morph to listed targets", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const focus = document.createElement("section");
    focus.setAttribute("title", "catalogue-focus");
    focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
    focus.classList.add("presentation-arrangement--positioned");
    deckEl.appendChild(focus);
    focus.classList.add("presentation-arrangement--settled");
    syncArrangementSettledState(deckEl, focus, null);
    expect(focus.classList.contains("presentation-arrangement--settled")).toBe(false);
    const labels = document.createElement("section");
    labels.setAttribute("title", "catalogue-labels");
    deckEl.appendChild(labels);
    prepareArrangementBeforeAutoAnimate(focus, labels);
    expect(focus.classList.contains("presentation-arrangement--settled")).toBe(true);
    expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
    expect(labels.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
  });

  it("syncManyToOneGhostMorphFramesFromDom uses live source tile geometry on target ghosts", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const focus = document.createElement("section");
    focus.setAttribute("title", "catalogue-focus");
    focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
    focus.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
    const labels = document.createElement("section");
    labels.setAttribute("title", "catalogue-labels");
    labels.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
    const source = document.createElement("div");
    source.className = "presentation-interactive-disposition presentation-interactive-disposition--canvas-framed";
    source.setAttribute("data-id", "Stütze");
    Object.assign(source.style, {
      position: "absolute",
      left: "20%",
      top: "10%",
      width: "15%",
      height: "70%",
    });
    const ghost = document.createElement("div");
    ghost.className = "presentation-interactive-disposition presentation-target-ghost presentation-interactive-disposition--canvas-framed";
    ghost.setAttribute("data-id", "Stütze");
    Object.assign(ghost.style, {
      position: "absolute",
      left: "60%",
      top: "44%",
      width: "24%",
      height: "12%",
    });
    const slot = document.createElement("div");
    slot.className = "presentation-morph-slot presentation-morph-slot--figure";
    slot.dataset.presentationMorphCrop = JSON.stringify({ x: 0, y: 0, width: 0.5, height: 1 });
    ghost.append(slot);
    focus.append(source);
    labels.append(ghost);
    deckEl.append(focus, labels);
    document.body.append(deckEl);
    syncManyToOneGhostMorphFramesFromDom(focus, labels);
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-left")).toBe("20%");
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-top")).toBe("10%");
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-width")).toBe("15%");
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-height")).toBe("70%");
    expect(ghost.style.getPropertyValue("--presentation-frame-left")).toBe("60%");
    deckEl.remove();
  });

  it("does not mark many-to-one morph for catalogue to focus auto-animate", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const catalogue = document.createElement("section");
    catalogue.setAttribute("title", "catalogue");
    const focus = document.createElement("section");
    focus.setAttribute("title", "catalogue-focus");
    focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
    deckEl.append(catalogue, focus);
    expect(isManyToOneMorphTransition(catalogue, focus)).toBe(false);
    prepareArrangementBeforeAutoAnimate(catalogue, focus);
    expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
    expect(catalogue.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
  });

  it("renders expanded source ghosts for one-to-many morphTo", () => {
    const deck: Presentation = {
      id: "source-ghost",
      name: "Source ghost",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "split",
                  participants: [{ id: "whole" }, { id: "tile-a" }],
                  embodiments: [
                    { kind: "figure", id: "whole--figure", src: "/catalogue.png" },
                    {
                      kind: "figure",
                      id: "tile-a--figure",
                      src: "/catalogue.png",
                      crop: { x: 0, y: 0, width: 0.5, height: 1 },
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "whole",
                        dispositions: [
                          {
                            participantId: "whole",
                            embodimentId: "whole--figure",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.1, width: 0.8, height: 0.8 },
                            morphTo: [
                              {
                                participantId: "tile-a",
                                position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 },
                              },
                            ],
                          },
                        ],
                      },
                      transition: { kind: "morph" },
                    },
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: [
                          {
                            participantId: "tile-a",
                            embodimentId: "tile-a--figure",
                            emphasis: "active",
                            position: { x: 0.05, y: 0.2, width: 0.4, height: 0.6 },
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
    const wholeSlide = container.querySelector('section[title="whole"]') as HTMLElement;
    const sourceGhost = wholeSlide.querySelector('.presentation-interactive-disposition.presentation-source-ghost[data-id="tile-a"]');
    expect(sourceGhost).toBeTruthy();
    expect(wholeSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
  });

  it("renders positioned labels with data-id on the morph slot frame", () => {
    const deck: Presentation = {
      id: "label-slot",
      name: "Labels",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "labels",
                  participants: [{ id: "catalogue-col1" }],
                  embodiments: [
                    {
                      kind: "text",
                      id: "catalogue-col1--label",
                      lines: ["Rippenplatte"],
                      level: "heading",
                      morphRoot: "heading-line",
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "labels",
                        dispositions: [
                          {
                            participantId: "catalogue-col1",
                            embodimentId: "catalogue-col1--label",
                            emphasis: "active",
                            position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
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
    const slot = container.querySelector('[data-id="catalogue-col1"].presentation-morph-slot--label');
    expect(slot?.querySelector("h2")?.textContent).toBe("Rippenplatte");
    expect(slot?.querySelector("h2[data-id]")).toBeNull();
  });

  it("relaxes Tailwind preflight [hidden] so reveal's inline display drives slide visibility", () => {
    const style = document.createElement("style");
    style.textContent = '[hidden]:where(:not([hidden="until-found"])) { display: none; color: red; }';
    document.head.appendChild(style);
    const hiddenRule = (style.sheet as CSSStyleSheet).cssRules[0] as CSSStyleRule;
    expect(hiddenRule.style.getPropertyValue("display")).toBe("none");
    relaxHiddenPreflight();
    expect(hiddenRule.style.getPropertyValue("display")).toBe("");
    expect(hiddenRule.style.getPropertyValue("color")).toBe("red");
    style.remove();
  });

  it("does not navigate from bookmark query params", () => {
    expect(readPresentationSlideIndicesFromUrl("#/?sequence=main&thought=intro&slide=goal")).toEqual({
      h: 0,
      v: 0,
    });
    expect(readPresentationSlideIndicesFromUrl("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({
      h: 0,
      v: 2,
    });
    history.replaceState(null, "", "/presentation");
  });
});
